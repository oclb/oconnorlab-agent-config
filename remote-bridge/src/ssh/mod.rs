mod connection;
#[cfg(test)]
pub mod mock;

use crate::error::SshError;
use async_trait::async_trait;

pub use connection::*;

/// Trait for remote command execution
///
/// This trait abstracts the SSH connection, allowing for mock implementations
/// in tests while using the real SSH connection in production.
#[async_trait]
pub trait RemoteExecutor: Send + Sync {
    /// Execute a command on the remote host
    async fn execute(&self, command: &str, timeout_secs: u64) -> Result<CommandOutput, SshError>;

    /// Execute a command with properly escaped arguments
    async fn execute_with_args(
        &self,
        program: &str,
        args: &[&str],
        timeout_secs: u64,
    ) -> Result<CommandOutput, SshError> {
        // Default implementation: build command string with escaped args
        let escaped_args: Vec<String> = args
            .iter()
            .map(|arg| {
                let escaped = arg.replace('\'', "'\"'\"'");
                format!("'{}'", escaped)
            })
            .collect();

        let command = format!("{} {}", program, escaped_args.join(" "));
        self.execute(&command, timeout_secs).await
    }

    /// Check if the connection is active
    async fn is_connected(&self) -> bool;

    /// Get the username for the connection
    fn user(&self) -> &str;

    /// Get the hostname for the connection
    fn host(&self) -> &str;
}

#[async_trait]
impl RemoteExecutor for SshConnection {
    async fn execute(&self, command: &str, timeout_secs: u64) -> Result<CommandOutput, SshError> {
        SshConnection::execute(self, command, timeout_secs).await
    }

    async fn execute_with_args(
        &self,
        program: &str,
        args: &[&str],
        timeout_secs: u64,
    ) -> Result<CommandOutput, SshError> {
        SshConnection::execute_with_args(self, program, args, timeout_secs).await
    }

    async fn is_connected(&self) -> bool {
        SshConnection::is_connected(self).await
    }

    fn user(&self) -> &str {
        SshConnection::user(self)
    }

    fn host(&self) -> &str {
        SshConnection::host(self)
    }
}
