//! Mock implementation of RemoteExecutor for testing
//!
//! This module provides a mock SSH connection that can be configured
//! with expected commands and their responses.

use super::{CommandOutput, RemoteExecutor};
use crate::error::SshError;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Mutex;

/// Mock response for a command
#[derive(Debug, Clone)]
pub enum MockResponse {
    /// Successful command execution
    Success {
        stdout: String,
        stderr: String,
        exit_code: i32,
    },
    /// Command execution failure
    Failure { error: String },
    /// Command timeout
    Timeout,
}

impl MockResponse {
    /// Create a successful response with stdout only
    pub fn ok(stdout: impl Into<String>) -> Self {
        MockResponse::Success {
            stdout: stdout.into(),
            stderr: String::new(),
            exit_code: 0,
        }
    }

    /// Create a successful response with non-zero exit code
    pub fn exit_code(stdout: impl Into<String>, code: i32) -> Self {
        MockResponse::Success {
            stdout: stdout.into(),
            stderr: String::new(),
            exit_code: code,
        }
    }

    /// Create a failure response
    pub fn fail(error: impl Into<String>) -> Self {
        MockResponse::Failure {
            error: error.into(),
        }
    }

    /// Create a timeout response
    pub fn timeout() -> Self {
        MockResponse::Timeout
    }
}

/// Mock executor for testing
///
/// Configure expected commands and their responses using the builder pattern.
pub struct MockExecutor {
    user: String,
    host: String,
    connected: bool,
    /// Map from exact command string to response
    expectations: HashMap<String, MockResponse>,
    /// Record of all commands that were executed
    call_history: Mutex<Vec<String>>,
    /// Default response for unmatched commands
    default_response: Option<MockResponse>,
}

impl MockExecutor {
    /// Create a new mock executor
    pub fn new() -> Self {
        Self {
            user: "testuser".to_string(),
            host: "testhost".to_string(),
            connected: true,
            expectations: HashMap::new(),
            call_history: Mutex::new(Vec::new()),
            default_response: None,
        }
    }

    /// Set the username
    pub fn with_user(mut self, user: impl Into<String>) -> Self {
        self.user = user.into();
        self
    }

    /// Set the hostname
    pub fn with_host(mut self, host: impl Into<String>) -> Self {
        self.host = host.into();
        self
    }

    /// Set connection status
    pub fn with_connected(mut self, connected: bool) -> Self {
        self.connected = connected;
        self
    }

    /// Add an expected command and its response
    pub fn expect(mut self, command: impl Into<String>, response: MockResponse) -> Self {
        self.expectations.insert(command.into(), response);
        self
    }

    /// Set a default response for unmatched commands
    pub fn with_default(mut self, response: MockResponse) -> Self {
        self.default_response = Some(response);
        self
    }

    /// Get the history of executed commands
    pub fn call_history(&self) -> Vec<String> {
        self.call_history.lock().unwrap().clone()
    }

    /// Check if a specific command was called
    pub fn was_called(&self, command: &str) -> bool {
        self.call_history
            .lock()
            .unwrap()
            .iter()
            .any(|c| c == command)
    }

    /// Check if a command containing the given substring was called
    pub fn was_called_with(&self, substring: &str) -> bool {
        self.call_history
            .lock()
            .unwrap()
            .iter()
            .any(|c| c.contains(substring))
    }

    /// Get the number of times any command was called
    pub fn call_count(&self) -> usize {
        self.call_history.lock().unwrap().len()
    }

    /// Clear call history
    pub fn clear_history(&self) {
        self.call_history.lock().unwrap().clear();
    }
}

impl Default for MockExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RemoteExecutor for MockExecutor {
    async fn execute(&self, command: &str, _timeout_secs: u64) -> Result<CommandOutput, SshError> {
        // Record the call
        self.call_history.lock().unwrap().push(command.to_string());

        // Check if not connected
        if !self.connected {
            return Err(SshError::NotConnected(self.user.clone(), self.host.clone()));
        }

        // Look up exact match first
        if let Some(response) = self.expectations.get(command) {
            return match response {
                MockResponse::Success {
                    stdout,
                    stderr,
                    exit_code,
                } => Ok(CommandOutput {
                    stdout: stdout.clone(),
                    stderr: stderr.clone(),
                    exit_code: *exit_code,
                }),
                MockResponse::Failure { error } => Err(SshError::CommandFailed(error.clone())),
                MockResponse::Timeout => Err(SshError::Timeout(0)),
            };
        }

        // Try prefix matching for commands with dynamic parts
        // This helps with commands like "cat > '/path/...'" where the path varies
        for (expected, response) in &self.expectations {
            if command.starts_with(expected) || expected.starts_with(command) {
                return match response {
                    MockResponse::Success {
                        stdout,
                        stderr,
                        exit_code,
                    } => Ok(CommandOutput {
                        stdout: stdout.clone(),
                        stderr: stderr.clone(),
                        exit_code: *exit_code,
                    }),
                    MockResponse::Failure { error } => Err(SshError::CommandFailed(error.clone())),
                    MockResponse::Timeout => Err(SshError::Timeout(0)),
                };
            }
        }

        // Use default response if set
        if let Some(response) = &self.default_response {
            return match response {
                MockResponse::Success {
                    stdout,
                    stderr,
                    exit_code,
                } => Ok(CommandOutput {
                    stdout: stdout.clone(),
                    stderr: stderr.clone(),
                    exit_code: *exit_code,
                }),
                MockResponse::Failure { error } => Err(SshError::CommandFailed(error.clone())),
                MockResponse::Timeout => Err(SshError::Timeout(0)),
            };
        }

        // No match found - return an error indicating unexpected command
        Err(SshError::CommandFailed(format!(
            "Unexpected command in mock: {}",
            command
        )))
    }

    async fn is_connected(&self) -> bool {
        self.connected
    }

    fn user(&self) -> &str {
        &self.user
    }

    fn host(&self) -> &str {
        &self.host
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_executor_success() {
        let mock = MockExecutor::new().expect("ls -la", MockResponse::ok("file1.txt\nfile2.txt"));

        let result = mock.execute("ls -la", 10).await.unwrap();
        assert_eq!(result.stdout, "file1.txt\nfile2.txt");
        assert_eq!(result.exit_code, 0);
        assert!(mock.was_called("ls -la"));
    }

    #[tokio::test]
    async fn test_mock_executor_failure() {
        let mock =
            MockExecutor::new().expect("bad_command", MockResponse::fail("command not found"));

        let result = mock.execute("bad_command", 10).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mock_executor_not_connected() {
        let mock = MockExecutor::new().with_connected(false);

        let result = mock.execute("any_command", 10).await;
        assert!(matches!(result, Err(SshError::NotConnected(_, _))));
    }

    #[tokio::test]
    async fn test_mock_executor_unexpected_command() {
        let mock = MockExecutor::new();

        let result = mock.execute("unexpected", 10).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_mock_executor_default_response() {
        let mock = MockExecutor::new().with_default(MockResponse::ok("default output"));

        let result = mock.execute("any_command", 10).await.unwrap();
        assert_eq!(result.stdout, "default output");
    }

    #[tokio::test]
    async fn test_mock_executor_call_history() {
        let mock = MockExecutor::new().with_default(MockResponse::ok(""));

        mock.execute("cmd1", 10).await.unwrap();
        mock.execute("cmd2", 10).await.unwrap();
        mock.execute("cmd1", 10).await.unwrap();

        assert_eq!(mock.call_count(), 3);
        assert!(mock.was_called("cmd1"));
        assert!(mock.was_called("cmd2"));
        assert!(!mock.was_called("cmd3"));
    }
}
