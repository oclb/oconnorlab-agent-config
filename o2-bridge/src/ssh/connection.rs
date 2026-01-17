use crate::error::SshError;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use std::time::Duration;
use tokio::process::Command;
use tokio::sync::RwLock;
use tracing::debug;

/// SSH connection manager using ControlMaster
pub struct SshConnection {
    user: String,
    host: String,
    socket_path: PathBuf,
    state: Arc<RwLock<ConnectionState>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    Unknown,
    Connected,
    Disconnected,
}

/// Output from a remote command
#[derive(Debug)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

impl SshConnection {
    pub fn new(user: String, host: String, _bridge_socket: PathBuf) -> Self {
        // Use a dedicated socket for SSH ControlMaster
        let socket_path = PathBuf::from(format!("/tmp/o2-bridge-ssh-{}", user));

        Self {
            user,
            host,
            socket_path,
            state: Arc::new(RwLock::new(ConnectionState::Unknown)),
        }
    }

    /// Get the SSH socket path
    pub fn socket_path(&self) -> &PathBuf {
        &self.socket_path
    }

    /// Check if SSH connection is alive
    pub async fn check_connection(&self) -> bool {
        let output = Command::new("ssh")
            .args([
                "-S",
                self.socket_path.to_str().unwrap_or(""),
                "-O",
                "check",
                &format!("{}@{}", self.user, self.host),
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output()
            .await;

        let connected = match output {
            Ok(o) => o.status.success(),
            Err(_) => false,
        };

        let mut state = self.state.write().await;
        *state = if connected {
            ConnectionState::Connected
        } else {
            ConnectionState::Disconnected
        };

        connected
    }

    /// Get instructions for establishing connection
    pub fn connection_instructions(&self) -> String {
        format!(
            "SSH connection required. Please run:\n\n  \
             ssh -M -S {} -o ControlPersist=yes -fN {}@{}\n\n\
             This will prompt for Duo authentication (one-time per session).",
            self.socket_path.display(),
            self.user,
            self.host
        )
    }

    /// Execute a command over SSH
    pub async fn execute(
        &self,
        command: &str,
        timeout_secs: u64,
    ) -> Result<CommandOutput, SshError> {
        // First check connection
        if !self.check_connection().await {
            return Err(SshError::NotConnected(
                self.user.clone(),
                self.host.clone(),
            ));
        }

        debug!("Executing remote command: {}", command);

        let result = tokio::time::timeout(
            Duration::from_secs(timeout_secs),
            Command::new("ssh")
                .args([
                    "-S",
                    self.socket_path.to_str().unwrap_or(""),
                    &format!("{}@{}", self.user, self.host),
                    command,
                ])
                .output(),
        )
        .await;

        match result {
            Ok(Ok(output)) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let exit_code = output.status.code().unwrap_or(-1);

                if !stderr.is_empty() && exit_code != 0 {
                    debug!("Command stderr: {}", stderr);
                }

                Ok(CommandOutput {
                    stdout,
                    stderr,
                    exit_code,
                })
            }
            Ok(Err(e)) => Err(SshError::Io(e)),
            Err(_) => Err(SshError::Timeout(timeout_secs)),
        }
    }

    /// Execute a command with proper argument escaping
    pub async fn execute_with_args(
        &self,
        program: &str,
        args: &[&str],
        timeout_secs: u64,
    ) -> Result<CommandOutput, SshError> {
        // Build command with proper escaping
        // Each argument is single-quoted to prevent shell interpretation
        let escaped_args: Vec<String> = args
            .iter()
            .map(|arg| {
                // Escape single quotes within the argument
                let escaped = arg.replace("'", "'\"'\"'");
                format!("'{}'", escaped)
            })
            .collect();

        let command = format!("{} {}", program, escaped_args.join(" "));
        self.execute(&command, timeout_secs).await
    }

    /// Get current connection state
    pub async fn state(&self) -> ConnectionState {
        self.state.read().await.clone()
    }

    /// Get user
    pub fn user(&self) -> &str {
        &self.user
    }

    /// Get host
    pub fn host(&self) -> &str {
        &self.host
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_instructions() {
        let conn = SshConnection::new(
            "testuser".to_string(),
            "o2.hms.harvard.edu".to_string(),
            PathBuf::from("/tmp/test.sock"),
        );

        let instructions = conn.connection_instructions();
        assert!(instructions.contains("testuser@o2.hms.harvard.edu"));
        assert!(instructions.contains("-M -S"));
        assert!(instructions.contains("ControlPersist"));
    }
}
