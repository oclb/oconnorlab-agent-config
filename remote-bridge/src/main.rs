use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;

mod config;
mod ssh;
mod rpc;
mod commands;
mod error;

#[derive(Parser)]
#[command(name = "remote-bridge")]
#[command(about = "Secure bridge for Claude Code to access remote hosts via SSH")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a bridge connection
    Start {
        /// Connection name (used for socket naming)
        name: String,

        /// Remote username
        #[arg(short, long)]
        user: Option<String>,

        /// Remote hostname
        #[arg(long, default_value = "o2.hms.harvard.edu")]
        host: String,

        /// Path to permissions config file
        #[arg(short, long, default_value = "~/.config/remote-bridge/permissions.toml")]
        config: PathBuf,
    },

    /// Check connection status
    Status {
        /// Connection name
        name: String,
    },

    /// Stop a bridge connection
    Stop {
        /// Connection name
        name: String,
    },

    /// Verify config file integrity
    VerifyConfig {
        /// Path to permissions config file
        #[arg(short, long, default_value = "~/.config/remote-bridge/permissions.toml")]
        config: PathBuf,
    },

    /// Update config checksum (after intentional changes)
    UpdateChecksum {
        /// Path to permissions config file
        #[arg(short, long, default_value = "~/.config/remote-bridge/permissions.toml")]
        config: PathBuf,
    },
}

fn expand_tilde(path: &PathBuf) -> PathBuf {
    if let Some(path_str) = path.to_str() {
        if path_str.starts_with("~/") {
            if let Some(home) = dirs::home_dir() {
                return home.join(&path_str[2..]);
            }
        }
    }
    path.clone()
}

fn socket_path(name: &str) -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".claude")
        .join(format!("remote-bridge-{}.sock", name))
}

fn ssh_socket_path(name: &str) -> PathBuf {
    PathBuf::from(format!("/tmp/remote-bridge-ssh-{}", name))
}

/// Establish SSH connection (spawns ssh, user sees Duo prompt)
fn establish_ssh_connection(user: &str, host: &str, socket: &PathBuf) -> Result<()> {
    // Clean up stale socket if exists
    if socket.exists() {
        let _ = std::fs::remove_file(socket);
    }

    println!("Connecting to {}@{}...", user, host);
    println!("(You may need to approve Duo authentication)\n");

    // Spawn SSH with ControlMaster - this will prompt for Duo
    // -M: ControlMaster mode
    // -S: Socket path
    // -o ControlPersist=12h: Keep connection alive for 12 hours
    // -f: Background after authentication
    // -N: No remote command
    let status = Command::new("ssh")
        .args([
            "-M",
            "-S", socket.to_str().unwrap(),
            "-o", "ControlPersist=12h",
            "-o", "ServerAliveInterval=60",
            "-o", "ServerAliveCountMax=3",
            "-f",
            "-N",
            &format!("{}@{}", user, host),
        ])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Failed to spawn SSH")?;

    if !status.success() {
        anyhow::bail!("SSH connection failed with exit code: {:?}", status.code());
    }

    // Verify connection is actually up
    let check = Command::new("ssh")
        .args([
            "-S", socket.to_str().unwrap(),
            "-O", "check",
            &format!("{}@{}", user, host),
        ])
        .output()?;

    if !check.status.success() {
        anyhow::bail!("SSH connection check failed after connect");
    }

    println!("SSH connection established!");
    Ok(())
}

/// Close SSH connection
fn close_ssh_connection(socket: &PathBuf) {
    if socket.exists() {
        let _ = Command::new("ssh")
            .args([
                "-S", socket.to_str().unwrap(),
                "-O", "exit",
                "dummy", // hostname doesn't matter for exit
            ])
            .output();

        // Clean up socket file if still exists
        let _ = std::fs::remove_file(socket);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start { name, user, host, config } => {
            // Initialize logging after arg parsing
            FmtSubscriber::builder()
                .with_max_level(Level::INFO)
                .with_target(false)
                .init();

            let config_path = expand_tilde(&config);
            let rpc_socket = socket_path(&name);
            let ssh_socket = ssh_socket_path(&name);

            // Check if already running
            if rpc_socket.exists() {
                anyhow::bail!(
                    "Bridge '{}' appears to be running. Use 'remote-bridge stop {}' first.",
                    name, name
                );
            }

            info!("Starting remote-bridge '{}'", name);

            // Load and verify config
            let permissions = config::load_and_verify(&config_path).await?;
            info!("Config loaded: {}", config_path.display());

            // Get username
            let username = user.unwrap_or_else(|| {
                std::env::var("USER").unwrap_or_else(|_| "unknown".to_string())
            });

            // Establish SSH connection (user sees Duo prompt here)
            establish_ssh_connection(&username, &host, &ssh_socket)?;

            // Create SSH connection manager
            let ssh = ssh::SshConnection::new(
                username.clone(),
                host.clone(),
                ssh_socket.clone(),
            );

            // Set up cleanup on Ctrl+C
            let ssh_socket_clone = ssh_socket.clone();
            let rpc_socket_clone = rpc_socket.clone();
            ctrlc::set_handler(move || {
                eprintln!("\nShutting down...");
                close_ssh_connection(&ssh_socket_clone);
                let _ = std::fs::remove_file(&rpc_socket_clone);
                std::process::exit(0);
            }).expect("Error setting Ctrl-C handler");

            // Start RPC server
            info!("RPC socket: {}", rpc_socket.display());
            info!("Bridge '{}' ready. Press Ctrl+C to stop.", name);

            let result = rpc::start_server(rpc_socket.clone(), permissions, ssh).await;

            // Cleanup on exit
            close_ssh_connection(&ssh_socket);
            let _ = std::fs::remove_file(&rpc_socket);

            result?;
        }

        Commands::Status { name } => {
            let rpc_socket = socket_path(&name);
            let ssh_socket = ssh_socket_path(&name);

            println!("Bridge: {}", name);
            println!("RPC socket: {}", rpc_socket.display());
            println!("SSH socket: {}", ssh_socket.display());

            let rpc_running = rpc_socket.exists();
            let ssh_connected = ssh_socket.exists() && {
                Command::new("ssh")
                    .args(["-S", ssh_socket.to_str().unwrap(), "-O", "check", "dummy"])
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false)
            };

            println!("RPC server: {}", if rpc_running { "running" } else { "stopped" });
            println!("SSH: {}", if ssh_connected { "connected" } else { "disconnected" });
        }

        Commands::Stop { name } => {
            let rpc_socket = socket_path(&name);
            let ssh_socket = ssh_socket_path(&name);

            println!("Stopping bridge '{}'...", name);

            // Close SSH connection
            close_ssh_connection(&ssh_socket);
            println!("SSH connection closed");

            // Remove RPC socket
            if rpc_socket.exists() {
                std::fs::remove_file(&rpc_socket)?;
                println!("RPC socket removed");
            }

            println!("Stopped.");
        }

        Commands::VerifyConfig { config } => {
            let config_path = expand_tilde(&config);
            match config::verify_integrity(&config_path).await {
                Ok(status) => {
                    match status {
                        config::IntegrityStatus::Valid => {
                            println!("Config integrity verified.");
                        }
                        config::IntegrityStatus::Modified { stored, current } => {
                            println!("WARNING: Config has been modified!");
                            println!("Stored checksum:  {}", stored);
                            println!("Current checksum: {}", current);
                            println!("\nIf you made intentional changes, run:");
                            println!("  remote-bridge update-checksum -c {}", config_path.display());
                        }
                        config::IntegrityStatus::NoBaseline => {
                            println!("No checksum baseline found. Run update-checksum to create one.");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error verifying config: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::UpdateChecksum { config } => {
            let config_path = expand_tilde(&config);
            config::update_checksum(&config_path).await?;
            println!("Checksum updated for {}", config_path.display());
        }
    }

    Ok(())
}
