use anyhow::Result;
use clap::{Parser, Subcommand};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod config;
mod ssh;
mod rpc;
mod commands;
mod error;
mod sbatch;
#[cfg(test)]
mod testing;

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

    /// Send an RPC command to a running bridge
    Rpc {
        /// Connection name
        name: String,

        /// RPC method name (e.g., connection_status, ls, cat, grep, git_pull, squeue, sbatch)
        method: String,

        /// JSON parameters (optional, e.g., '{"path":"/n/data1/..."}')
        params: Option<String>,
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

            // Create SSH connection (persistent session with piped stdin/stdout)
            let ssh = Arc::new(ssh::SshConnection::new(
                username.clone(),
                host.clone(),
            ));

            // Establish the persistent SSH session (user sees login/Duo prompt here)
            println!("Connecting to {}@{}...", username, host);
            println!("(You may need to approve Duo authentication)\n");
            ssh.connect().await?;

            // Verify the connection actually works by running a test command
            print!("Verifying connection... ");
            std::io::stdout().flush()?;
            match ssh.execute("whoami", 60).await {
                Ok(output) if output.exit_code == 0 => {
                    let remote_user = output.stdout.trim();
                    println!("OK (logged in as {})", remote_user);
                    info!("SSH session verified: {}", remote_user);
                }
                Ok(output) => {
                    println!("FAILED");
                    anyhow::bail!(
                        "Connection test failed with exit code {}: {}",
                        output.exit_code,
                        output.stdout
                    );
                }
                Err(e) => {
                    println!("FAILED");
                    anyhow::bail!("Connection test failed: {}. Did you approve Duo authentication?", e);
                }
            }

            // Set up cleanup on Ctrl+C
            let rpc_socket_clone = rpc_socket.clone();
            ctrlc::set_handler(move || {
                eprintln!("\nShutting down...");
                // SSH session is cleaned up when process exits
                let _ = std::fs::remove_file(&rpc_socket_clone);
                std::process::exit(0);
            }).expect("Error setting Ctrl-C handler");

            // Start RPC server
            info!("RPC socket: {}", rpc_socket.display());
            info!("Bridge '{}' ready. Press Ctrl+C to stop.", name);

            let result = rpc::start_server(rpc_socket.clone(), permissions, ssh).await;

            // Cleanup on exit
            let _ = std::fs::remove_file(&rpc_socket);

            result?;
        }

        Commands::Status { name } => {
            let rpc_socket = socket_path(&name);

            println!("Bridge: {}", name);
            println!("RPC socket: {}", rpc_socket.display());

            let rpc_running = rpc_socket.exists();
            // Note: SSH status is tied to the bridge process now
            // If RPC socket exists, the bridge (and its SSH session) should be running

            println!("Status: {}", if rpc_running { "running" } else { "stopped" });
        }

        Commands::Stop { name } => {
            let rpc_socket = socket_path(&name);

            println!("Stopping bridge '{}'...", name);

            // The SSH session is owned by the bridge process
            // Removing the socket will cause connection failures, but to fully stop
            // you need to kill the bridge process (Ctrl+C or kill)
            if rpc_socket.exists() {
                std::fs::remove_file(&rpc_socket)?;
                println!("RPC socket removed");
                println!("Note: The bridge process may still be running. Use Ctrl+C or kill to stop it.");
            } else {
                println!("Bridge '{}' is not running.", name);
            }
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

        Commands::Rpc { name, method, params } => {
            let rpc_socket = socket_path(&name);

            if !rpc_socket.exists() {
                eprintln!("Bridge '{}' is not running.", name);
                eprintln!("Start it with: remote-bridge start {} --user YOUR_USERNAME", name);
                std::process::exit(1);
            }

            // Build JSON-RPC request
            let request = if let Some(params_str) = params {
                // Parse params to validate JSON
                let params_value: serde_json::Value = serde_json::from_str(&params_str)
                    .map_err(|e| anyhow::anyhow!("Invalid JSON params: {}", e))?;
                serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": method,
                    "params": params_value,
                    "id": 1
                })
            } else {
                serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": method,
                    "id": 1
                })
            };

            // Connect to socket
            let mut stream = UnixStream::connect(&rpc_socket)
                .map_err(|e| anyhow::anyhow!("Failed to connect to bridge: {}", e))?;

            // Send request
            let request_str = request.to_string() + "\n";
            stream.write_all(request_str.as_bytes())?;
            stream.flush()?;

            // Read response
            let mut reader = BufReader::new(stream);
            let mut response = String::new();
            reader.read_line(&mut response)?;

            // Pretty-print JSON response
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&response) {
                println!("{}", serde_json::to_string_pretty(&json)?);
            } else {
                print!("{}", response);
            }
        }
    }

    Ok(())
}
