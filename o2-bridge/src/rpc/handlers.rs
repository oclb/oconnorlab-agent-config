use crate::commands::{self, PathValidator};
use crate::config::PermissionConfig;
use crate::rpc::types::*;
use crate::ssh::SshConnection;
use std::sync::Arc;
use std::time::Instant;

/// Shared state for RPC handlers
pub struct RpcState {
    pub ssh: Arc<SshConnection>,
    pub config: Arc<PermissionConfig>,
    pub validator: Arc<PathValidator>,
}

impl RpcState {
    pub fn new(ssh: SshConnection, config: PermissionConfig) -> Self {
        let validator = PathValidator::new(config.clone());
        Self {
            ssh: Arc::new(ssh),
            config: Arc::new(config),
            validator: Arc::new(validator),
        }
    }

    /// Get connection status
    pub async fn connection_status(&self) -> ConnectionStatus {
        let connected = self.ssh.check_connection().await;

        ConnectionStatus {
            connected,
            user: self.ssh.user().to_string(),
            host: self.ssh.host().to_string(),
            socket_path: self.ssh.socket_path().display().to_string(),
            instructions: if connected {
                None
            } else {
                Some(self.ssh.connection_instructions())
            },
        }
    }

    /// Execute ls command
    pub async fn ls(
        &self,
        request: commands::LsRequest,
    ) -> Result<commands::LsResponse, RpcError> {
        let start = Instant::now();

        // Validate path
        let validated = self
            .validator
            .validate_read_path(&request.path)
            .map_err(|e| RpcError {
                code: ERR_PERMISSION_DENIED,
                message: e.to_string(),
                data: None,
            })?;

        // Build command
        let mut args = vec![];
        for flag in &request.flags {
            args.push(flag.to_arg());
        }
        args.push(validated.as_str());

        // Execute
        let output = self
            .ssh
            .execute_with_args("ls", &args.iter().map(|s| s.as_str()).collect::<Vec<_>>(), 30)
            .await
            .map_err(|e| RpcError {
                code: ERR_COMMAND_FAILED,
                message: e.to_string(),
                data: None,
            })?;

        // Parse output
        let entries = commands::parse_ls_output(&output.stdout, request.flags.contains(&commands::LsFlag::Long));

        Ok(commands::LsResponse {
            entries,
            path: validated.to_string(),
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }

    /// Execute cat command
    pub async fn cat(
        &self,
        request: commands::CatRequest,
    ) -> Result<commands::CatResponse, RpcError> {
        let start = Instant::now();

        // Validate path
        let validated = self
            .validator
            .validate_read_path(&request.path)
            .map_err(|e| RpcError {
                code: ERR_PERMISSION_DENIED,
                message: e.to_string(),
                data: None,
            })?;

        // Build command based on head/tail options
        let command = if let Some(head) = request.head {
            format!("head -n {} '{}'", head, validated.as_str())
        } else if let Some(tail) = request.tail {
            format!("tail -n {} '{}'", tail, validated.as_str())
        } else if let Some(offset) = request.offset {
            if let Some(limit) = request.limit {
                format!("sed -n '{},{}p' '{}'", offset, offset + limit - 1, validated.as_str())
            } else {
                format!("tail -n +{} '{}'", offset, validated.as_str())
            }
        } else {
            format!("cat '{}'", validated.as_str())
        };

        // Execute
        let output = self
            .ssh
            .execute(&command, 60)
            .await
            .map_err(|e| RpcError {
                code: ERR_COMMAND_FAILED,
                message: e.to_string(),
                data: None,
            })?;

        let lines: Vec<&str> = output.stdout.lines().collect();

        Ok(commands::CatResponse {
            content: output.stdout,
            path: validated.to_string(),
            total_lines: lines.len(),
            truncated: false,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }

    /// Execute grep command
    pub async fn grep(
        &self,
        request: commands::GrepRequest,
    ) -> Result<commands::GrepResponse, RpcError> {
        let start = Instant::now();

        // Validate regex pattern first
        regex::Regex::new(&request.pattern).map_err(|e| RpcError {
            code: ERR_INVALID_REGEX,
            message: format!("Invalid regex: {}", e),
            data: None,
        })?;

        // Validate all paths
        let validated_paths: Vec<String> = request
            .paths
            .iter()
            .map(|p| {
                self.validator.validate_read_path(p).map(|v| v.to_string())
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| RpcError {
                code: ERR_PERMISSION_DENIED,
                message: e.to_string(),
                data: None,
            })?;

        // Build grep command with flags
        let mut args = vec!["-E".to_string()]; // Extended regex
        for flag in &request.flags {
            args.push(flag.to_arg().to_string());
        }

        // Escape the pattern for shell
        let escaped_pattern = request.pattern.replace("'", "'\"'\"'");
        args.push(format!("'{}'", escaped_pattern));

        // Add paths
        for path in &validated_paths {
            args.push(format!("'{}'", path));
        }

        let command = format!("grep {}", args.join(" "));

        // Execute
        let output = self
            .ssh
            .execute(&command, 120)
            .await
            .map_err(|e| RpcError {
                code: ERR_COMMAND_FAILED,
                message: e.to_string(),
                data: None,
            })?;

        // Parse grep output
        let matches = commands::parse_grep_output(
            &output.stdout,
            request.flags.contains(&commands::GrepFlag::LineNumbers),
        );

        Ok(commands::GrepResponse {
            matches,
            total_matches: matches.len(),
            files_searched: validated_paths.len(),
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }
}
