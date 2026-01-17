use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod config;
mod ssh;
mod rpc;
mod commands;
mod error;

#[derive(Parser)]
#[command(name = "o2-bridge")]
#[command(about = "Secure bridge for Claude Code to access O2 cluster")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the O2 bridge server
    Start {
        /// Path to permissions config file
        #[arg(short, long, default_value = "~/.config/o2-bridge/permissions.toml")]
        config: PathBuf,

        /// Path to Unix socket
        #[arg(short, long, default_value = "~/.claude/o2-bridge.sock")]
        socket: PathBuf,

        /// O2 username (defaults to current user)
        #[arg(short, long)]
        user: Option<String>,

        /// O2 hostname
        #[arg(long, default_value = "o2.hms.harvard.edu")]
        host: String,
    },

    /// Check connection and server status
    Status {
        /// Path to Unix socket
        #[arg(short, long, default_value = "~/.claude/o2-bridge.sock")]
        socket: PathBuf,
    },

    /// Stop the running bridge server
    Stop {
        /// Path to Unix socket
        #[arg(short, long, default_value = "~/.claude/o2-bridge.sock")]
        socket: PathBuf,
    },

    /// Verify config file integrity
    VerifyConfig {
        /// Path to permissions config file
        #[arg(short, long, default_value = "~/.config/o2-bridge/permissions.toml")]
        config: PathBuf,
    },

    /// Update config checksum (after intentional changes)
    UpdateChecksum {
        /// Path to permissions config file
        #[arg(short, long, default_value = "~/.config/o2-bridge/permissions.toml")]
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

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Start { config, socket, user, host } => {
            let config_path = expand_tilde(&config);
            let socket_path = expand_tilde(&socket);

            info!("Starting O2 bridge...");
            info!("Config: {}", config_path.display());
            info!("Socket: {}", socket_path.display());

            // Load and verify config
            let permissions = config::load_and_verify(&config_path).await?;

            // Get username
            let username = user.unwrap_or_else(|| {
                std::env::var("USER").unwrap_or_else(|_| "unknown".to_string())
            });

            // Create SSH connection manager
            let ssh = ssh::SshConnection::new(username, host, socket_path.clone());

            // Start RPC server
            rpc::start_server(socket_path, permissions, ssh).await?;
        }

        Commands::Status { socket } => {
            let socket_path = expand_tilde(&socket);
            println!("Checking status at {}...", socket_path.display());

            // TODO: Connect to socket and query status
            if socket_path.exists() {
                println!("Socket exists. Server may be running.");
            } else {
                println!("Socket not found. Server is not running.");
            }
        }

        Commands::Stop { socket } => {
            let socket_path = expand_tilde(&socket);
            println!("Stopping server at {}...", socket_path.display());

            // TODO: Send shutdown signal via RPC
            if socket_path.exists() {
                std::fs::remove_file(&socket_path)?;
                println!("Socket removed.");
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
                            println!("  o2-bridge update-checksum -c {}", config_path.display());
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
