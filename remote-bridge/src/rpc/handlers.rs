#![allow(dead_code)]

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use crate::commands::{self, PathValidator};
use crate::config::PermissionConfig;
use crate::rpc::types::{ConnectionStatus, RpcError, ERR_COMMAND_FAILED, ERR_FILE_TOO_LARGE, ERR_INVALID_REGEX, ERR_PERMISSION_DENIED, ERR_CONFIG_ERROR};
use crate::sbatch;
use crate::ssh::RemoteExecutor;
use std::sync::Arc;
use std::time::Instant;

/// Shared state for RPC handlers
pub struct RpcState {
    pub ssh: Arc<dyn RemoteExecutor>,
    pub config: Arc<PermissionConfig>,
    pub validator: Arc<PathValidator>,
}

impl RpcState {
    pub fn new(ssh: Arc<dyn RemoteExecutor>, config: PermissionConfig) -> Self {
        let validator = PathValidator::new(config.clone());
        Self {
            ssh,
            config: Arc::new(config),
            validator: Arc::new(validator),
        }
    }

    /// Get connection status
    pub async fn connection_status(&self) -> ConnectionStatus {
        let connected = self.ssh.is_connected().await;

        ConnectionStatus {
            connected,
            user: self.ssh.user().to_string(),
            host: self.ssh.host().to_string(),
            socket_path: "N/A (persistent session)".to_string(),
            instructions: if connected {
                None
            } else {
                Some("SSH session not connected. Restart the bridge.".to_string())
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
        let mut args: Vec<&str> = vec!["--color=never"];
        for flag in &request.flags {
            args.push(flag.to_arg());
        }
        let path_str = validated.as_str();
        args.push(path_str);

        // Execute
        let output = self
            .ssh
            .execute_with_args("ls", &args, 30)
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

        let total_lines = output.stdout.lines().count();

        Ok(commands::CatResponse {
            content: output.stdout,
            path: validated.to_string(),
            total_lines,
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
        let total_matches = matches.len();
        let files_searched = validated_paths.len();

        Ok(commands::GrepResponse {
            matches,
            total_matches,
            files_searched,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }

    /// Execute git pull command
    pub async fn git_pull(
        &self,
        request: commands::GitPullRequest,
    ) -> Result<commands::GitPullResponse, RpcError> {
        let start = Instant::now();

        // Validate path (must be readable)
        let validated = self
            .validator
            .validate_read_path(&request.path)
            .map_err(|e| RpcError {
                code: ERR_PERMISSION_DENIED,
                message: e.to_string(),
                data: None,
            })?;

        // Build git pull command
        let branch_arg = request.branch.as_deref().unwrap_or("");
        let command = if branch_arg.is_empty() {
            format!("cd '{}' && git pull '{}'", validated.as_str(), request.remote)
        } else {
            format!(
                "cd '{}' && git pull '{}' '{}'",
                validated.as_str(),
                request.remote,
                branch_arg
            )
        };

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

        let (already_up_to_date, files_changed) =
            commands::parse_git_pull_output(&output.stdout);

        Ok(commands::GitPullResponse {
            path: validated.to_string(),
            output: output.stdout,
            already_up_to_date,
            files_changed,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }

    /// Execute squeue command
    pub async fn squeue(
        &self,
        request: commands::SqueueRequest,
    ) -> Result<commands::SqueueResponse, RpcError> {
        let start = Instant::now();

        // Build squeue command
        let mut args = vec!["squeue".to_string()];

        if let Some(ref user) = request.user {
            args.push(format!("-u {}", user));
        }

        if !request.job_ids.is_empty() {
            args.push(format!("-j {}", request.job_ids.join(",")));
        }

        if let Some(ref partition) = request.partition {
            args.push(format!("-p {}", partition));
        }

        if let Some(ref state) = request.state {
            args.push(format!("-t {}", state.to_slurm_filter()));
        }

        let command = args.join(" ");

        // Execute
        let output = self
            .ssh
            .execute(&command, 30)
            .await
            .map_err(|e| RpcError {
                code: ERR_COMMAND_FAILED,
                message: e.to_string(),
                data: None,
            })?;

        let jobs = commands::parse_squeue_output(&output.stdout);

        Ok(commands::SqueueResponse {
            jobs,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }

    /// Execute sacct command
    pub async fn sacct(
        &self,
        request: commands::SacctRequest,
    ) -> Result<commands::SacctResponse, RpcError> {
        let start = Instant::now();

        // Build sacct command with parseable output
        let mut args = vec![
            "sacct".to_string(),
            "--parsable2".to_string(),
            "--format=JobID,JobName,Partition,State,ExitCode,Elapsed,MaxRSS,MaxVMSize,NCPUs,NTasks".to_string(),
        ];

        if !request.job_ids.is_empty() {
            args.push(format!("-j {}", request.job_ids.join(",")));
        }

        if let Some(ref user) = request.user {
            args.push(format!("-u {}", user));
        }

        if let Some(ref start_time) = request.start_time {
            args.push(format!("-S {}", start_time));
        }

        if let Some(ref end_time) = request.end_time {
            args.push(format!("-E {}", end_time));
        }

        if let Some(ref state) = request.state {
            args.push(format!("-s {}", state.to_slurm_filter()));
        }

        let command = args.join(" ");

        // Execute
        let output = self
            .ssh
            .execute(&command, 30)
            .await
            .map_err(|e| RpcError {
                code: ERR_COMMAND_FAILED,
                message: e.to_string(),
                data: None,
            })?;

        let jobs = commands::parse_sacct_output(&output.stdout);

        Ok(commands::SacctResponse {
            jobs,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }

    /// Execute sbatch command
    pub async fn sbatch(
        &self,
        request: commands::SbatchRequest,
    ) -> Result<commands::SbatchResponse, RpcError> {
        let start = Instant::now();

        // Validate script path (must be readable)
        let validated_script = self
            .validator
            .validate_read_path(&request.script_path)
            .map_err(|e| RpcError {
                code: ERR_PERMISSION_DENIED,
                message: e.to_string(),
                data: None,
            })?;

        // Build sbatch command
        let command = if let Some(ref working_dir) = request.working_dir {
            // Validate working dir if provided
            let validated_wd = self
                .validator
                .validate_read_path(working_dir)
                .map_err(|e| RpcError {
                    code: ERR_PERMISSION_DENIED,
                    message: e.to_string(),
                    data: None,
                })?;
            format!(
                "cd '{}' && sbatch '{}'",
                validated_wd.as_str(),
                validated_script.as_str()
            )
        } else {
            format!("sbatch '{}'", validated_script.as_str())
        };

        // Execute
        let output = self
            .ssh
            .execute(&command, 30)
            .await
            .map_err(|e| RpcError {
                code: ERR_COMMAND_FAILED,
                message: e.to_string(),
                data: None,
            })?;

        // Parse job ID from output
        let job_id = commands::parse_sbatch_output(&output.stdout).ok_or_else(|| RpcError {
            code: ERR_COMMAND_FAILED,
            message: format!("Failed to parse sbatch output: {}", output.stdout),
            data: None,
        })?;

        Ok(commands::SbatchResponse {
            job_id,
            script_path: validated_script.to_string(),
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }

    /// Execute download command - fetches file content with size limit
    pub async fn download(
        &self,
        request: commands::DownloadRequest,
    ) -> Result<commands::DownloadResponse, RpcError> {
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

        // Check file size first
        let size_cmd = format!("stat -c%s '{}' 2>/dev/null || stat -f%z '{}'",
            validated.as_str(), validated.as_str());
        let size_output = self
            .ssh
            .execute(&size_cmd, 10)
            .await
            .map_err(|e| RpcError {
                code: ERR_COMMAND_FAILED,
                message: e.to_string(),
                data: None,
            })?;

        let size_bytes: u64 = size_output
            .stdout
            .trim()
            .parse()
            .map_err(|_| RpcError {
                code: ERR_COMMAND_FAILED,
                message: format!("Could not determine file size: {}", size_output.stdout),
                data: None,
            })?;

        // Check size limit
        if size_bytes > commands::MAX_DOWNLOAD_SIZE {
            let scp_cmd = format!(
                "scp {}@transfer.rc.hms.harvard.edu:{} /local/destination/",
                self.ssh.user(),
                validated.as_str()
            );
            return Err(RpcError {
                code: ERR_FILE_TOO_LARGE,
                message: format!(
                    "File too large for download ({} bytes, max {} bytes). Use transfer node instead.",
                    size_bytes, commands::MAX_DOWNLOAD_SIZE
                ),
                data: Some(serde_json::to_value(commands::DownloadTooLargeError {
                    path: validated.to_string(),
                    size_bytes,
                    max_bytes: commands::MAX_DOWNLOAD_SIZE,
                    scp_command: scp_cmd,
                }).unwrap()),
            });
        }

        // Read file and base64 encode on remote (more efficient)
        let cmd = format!("base64 '{}'", validated.as_str());
        let output = self
            .ssh
            .execute(&cmd, 60)
            .await
            .map_err(|e| RpcError {
                code: ERR_COMMAND_FAILED,
                message: e.to_string(),
                data: None,
            })?;

        // Verify the base64 decodes properly (sanity check)
        let content = output.stdout.replace('\n', "").replace('\r', "");
        if BASE64.decode(&content).is_err() {
            return Err(RpcError {
                code: ERR_COMMAND_FAILED,
                message: "Failed to encode file content".to_string(),
                data: None,
            });
        }

        Ok(commands::DownloadResponse {
            path: validated.to_string(),
            content,
            size_bytes,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }

    /// Execute find command
    pub async fn find(
        &self,
        request: commands::FindRequest,
    ) -> Result<commands::FindResponse, RpcError> {
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

        // Build find command
        let mut cmd = format!("find '{}'", validated.as_str());

        if let Some(depth) = request.max_depth {
            cmd.push_str(&format!(" -maxdepth {}", depth));
        }

        if let Some(ref file_type) = request.file_type {
            cmd.push_str(&format!(" -type {}", file_type.to_arg()));
        }

        if let Some(ref name) = request.name {
            // Escape the name pattern for shell
            let escaped = name.replace("'", "'\"'\"'");
            cmd.push_str(&format!(" -name '{}'", escaped));
        }

        // Limit results
        cmd.push_str(&format!(" 2>/dev/null | head -n {}", request.limit + 1));

        // Execute
        let output = self
            .ssh
            .execute(&cmd, 120)
            .await
            .map_err(|e| RpcError {
                code: ERR_COMMAND_FAILED,
                message: e.to_string(),
                data: None,
            })?;

        let mut files: Vec<String> = output
            .stdout
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.to_string())
            .collect();

        let truncated = files.len() > request.limit;
        if truncated {
            files.truncate(request.limit);
        }

        let total_found = files.len();

        Ok(commands::FindResponse {
            files,
            total_found,
            truncated,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }

    /// Execute wc command
    pub async fn wc(
        &self,
        request: commands::WcRequest,
    ) -> Result<commands::WcResponse, RpcError> {
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

        // Build wc command
        let flag = if request.lines_only {
            "-l"
        } else if request.words_only {
            "-w"
        } else if request.bytes_only {
            "-c"
        } else {
            "" // Return all three
        };

        let cmd = format!("wc {} '{}'", flag, validated.as_str());

        // Execute
        let output = self
            .ssh
            .execute(&cmd, 60)
            .await
            .map_err(|e| RpcError {
                code: ERR_COMMAND_FAILED,
                message: e.to_string(),
                data: None,
            })?;

        // Parse wc output
        let parts: Vec<&str> = output.stdout.trim().split_whitespace().collect();

        let (lines, words, bytes) = if request.lines_only {
            (parts.first().and_then(|s| s.parse().ok()), None, None)
        } else if request.words_only {
            (None, parts.first().and_then(|s| s.parse().ok()), None)
        } else if request.bytes_only {
            (None, None, parts.first().and_then(|s| s.parse().ok()))
        } else {
            // Full output: lines words bytes filename
            (
                parts.first().and_then(|s| s.parse().ok()),
                parts.get(1).and_then(|s| s.parse().ok()),
                parts.get(2).and_then(|s| s.parse().ok()),
            )
        };

        Ok(commands::WcResponse {
            path: validated.to_string(),
            lines,
            words,
            bytes,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }

    /// Execute head command
    pub async fn head(
        &self,
        request: commands::HeadRequest,
    ) -> Result<commands::HeadResponse, RpcError> {
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

        let cmd = format!("head -n {} '{}'", request.lines, validated.as_str());

        // Execute
        let output = self
            .ssh
            .execute(&cmd, 30)
            .await
            .map_err(|e| RpcError {
                code: ERR_COMMAND_FAILED,
                message: e.to_string(),
                data: None,
            })?;

        let lines_returned = output.stdout.lines().count();

        Ok(commands::HeadResponse {
            path: validated.to_string(),
            content: output.stdout,
            lines_returned,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }

    /// Execute scancel command
    pub async fn scancel(
        &self,
        request: commands::ScancelRequest,
    ) -> Result<commands::ScancelResponse, RpcError> {
        let start = Instant::now();

        if request.job_ids.is_empty() {
            return Err(RpcError {
                code: -32602,
                message: "No job IDs provided".to_string(),
                data: None,
            });
        }

        // Build scancel command
        let cmd = format!("scancel {}", request.job_ids.join(" "));

        // Execute
        let output = self
            .ssh
            .execute(&cmd, 30)
            .await
            .map_err(|e| RpcError {
                code: ERR_COMMAND_FAILED,
                message: e.to_string(),
                data: None,
            })?;

        Ok(commands::ScancelResponse {
            cancelled_jobs: request.job_ids.clone(),
            output: output.stdout,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }

    /// Wait for a job to complete, polling with increasing intervals
    pub async fn job_wait(
        &self,
        request: commands::JobWaitRequest,
    ) -> Result<commands::JobWaitResponse, RpcError> {
        use commands::{ArrayWaitMode, CompletedJob};

        let start = Instant::now();
        let mut completed_jobs: Vec<CompletedJob> = Vec::new();
        let mut poll_count = 0u32;

        // Extract base job ID (without array index)
        let base_job_id = request.job_id.split('_').next().unwrap_or(&request.job_id);

        loop {
            // Check timeout
            let elapsed_secs = start.elapsed().as_secs();
            if elapsed_secs > request.max_wait_secs {
                return Ok(commands::JobWaitResponse {
                    job_id: request.job_id.clone(),
                    completed_jobs,
                    all_completed: false,
                    wait_time_secs: elapsed_secs,
                });
            }

            // Calculate sleep time: 0, 5, 10, 15, ... capped at 60s
            let sleep_secs = std::cmp::min(poll_count * 5, 60) as u64;
            if poll_count > 0 {
                tokio::time::sleep(std::time::Duration::from_secs(sleep_secs)).await;
            }
            poll_count += 1;

            // Query sacct for job status
            let command = format!(
                "sacct -j {} --parsable2 --noheader --format=JobID,State,ExitCode,Elapsed",
                base_job_id
            );

            let output = self
                .ssh
                .execute(&command, 30)
                .await
                .map_err(|e| RpcError {
                    code: ERR_COMMAND_FAILED,
                    message: e.to_string(),
                    data: None,
                })?;

            // Parse sacct output
            // Format: JobID|State|ExitCode|Elapsed
            let mut found_jobs: Vec<CompletedJob> = Vec::new();
            let mut pending_count = 0usize;

            for line in output.stdout.lines() {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() >= 4 {
                    let job_id = parts[0].trim();
                    let state = parts[1].trim();
                    let exit_code = parts[2].trim();
                    let elapsed = parts[3].trim();

                    // Skip .batch and .extern entries
                    if job_id.contains(".batch") || job_id.contains(".extern") {
                        continue;
                    }

                    if commands::is_terminal_state(state) {
                        found_jobs.push(CompletedJob {
                            job_id: job_id.to_string(),
                            state: state.to_string(),
                            exit_code: exit_code.to_string(),
                            elapsed: elapsed.to_string(),
                        });
                    } else {
                        pending_count += 1;
                    }
                }
            }

            // Update completed jobs
            for job in &found_jobs {
                if !completed_jobs.iter().any(|j| j.job_id == job.job_id) {
                    completed_jobs.push(job.clone());
                }
            }

            // Check completion based on mode
            match &request.array_mode {
                ArrayWaitMode::Any => {
                    if !completed_jobs.is_empty() {
                        return Ok(commands::JobWaitResponse {
                            job_id: request.job_id.clone(),
                            completed_jobs,
                            all_completed: pending_count == 0,
                            wait_time_secs: start.elapsed().as_secs(),
                        });
                    }
                }
                ArrayWaitMode::All => {
                    if pending_count == 0 && !completed_jobs.is_empty() {
                        return Ok(commands::JobWaitResponse {
                            job_id: request.job_id.clone(),
                            completed_jobs,
                            all_completed: true,
                            wait_time_secs: start.elapsed().as_secs(),
                        });
                    }
                }
                ArrayWaitMode::Index(idx) => {
                    let target_id = format!("{}_{}", base_job_id, idx);
                    if let Some(job) = completed_jobs.iter().find(|j| j.job_id == target_id) {
                        return Ok(commands::JobWaitResponse {
                            job_id: request.job_id.clone(),
                            completed_jobs: vec![job.clone()],
                            all_completed: true,
                            wait_time_secs: start.elapsed().as_secs(),
                        });
                    }
                }
            }
        }
    }

    /// Execute sandboxed sbatch command
    ///
    /// This generates an sbatch script that runs the command inside a Singularity
    /// container with restricted bind mounts, writes it to the remote, and submits it.
    pub async fn sandboxed_sbatch(
        &self,
        request: commands::SandboxedSbatchRequest,
    ) -> Result<commands::SandboxedSbatchResponse, RpcError> {
        let start = Instant::now();

        // Generate the script
        let generated = sbatch::generate_script(&request, &self.config).map_err(|e| RpcError {
            code: ERR_CONFIG_ERROR,
            message: e.to_string(),
            data: None,
        })?;

        // Get scripts directory from config
        let scripts_dir = self
            .config
            .singularity
            .scripts_dir
            .as_ref()
            .ok_or_else(|| RpcError {
                code: ERR_CONFIG_ERROR,
                message: "Singularity scripts_dir not configured".to_string(),
                data: None,
            })?;

        // Validate scripts_dir is writable
        if !self.config.is_write_allowed(scripts_dir) {
            return Err(RpcError {
                code: ERR_PERMISSION_DENIED,
                message: format!(
                    "Scripts directory {} is not in allowed write paths",
                    scripts_dir.display()
                ),
                data: None,
            });
        }

        let script_path = scripts_dir.join(&generated.filename);
        let script_path_str = script_path.to_string_lossy();

        // Ensure scripts directory exists on remote
        let mkdir_cmd = format!("mkdir -p '{}'", scripts_dir.display());
        self.ssh
            .execute(&mkdir_cmd, 10)
            .await
            .map_err(|e| RpcError {
                code: ERR_COMMAND_FAILED,
                message: format!("Failed to create scripts directory: {}", e),
                data: None,
            })?;

        // Write script to remote using base64 (more reliable than heredoc over PTY)
        let encoded = BASE64.encode(generated.content.as_bytes());
        let write_cmd = format!(
            "echo '{}' | base64 -d > '{}'",
            encoded, script_path_str
        );

        self.ssh
            .execute(&write_cmd, 30)
            .await
            .map_err(|e| RpcError {
                code: ERR_COMMAND_FAILED,
                message: format!("Failed to write script: {}", e),
                data: None,
            })?;

        // Make script executable
        let chmod_cmd = format!("chmod +x '{}'", script_path_str);
        self.ssh
            .execute(&chmod_cmd, 10)
            .await
            .map_err(|e| RpcError {
                code: ERR_COMMAND_FAILED,
                message: format!("Failed to chmod script: {}", e),
                data: None,
            })?;

        // For dry_run, skip submission and return the generated script
        let job_id = if request.dry_run {
            "DRY_RUN".to_string()
        } else {
            // Submit the job
            let sbatch_cmd = format!("sbatch '{}'", script_path_str);
            let output = self
                .ssh
                .execute(&sbatch_cmd, 30)
                .await
                .map_err(|e| RpcError {
                    code: ERR_COMMAND_FAILED,
                    message: format!("sbatch failed: {}", e),
                    data: None,
                })?;

            // Parse job ID
            commands::parse_sbatch_output(&output.stdout).ok_or_else(|| RpcError {
                code: ERR_COMMAND_FAILED,
                message: format!("Failed to parse sbatch output: {}", output.stdout),
                data: None,
            })?
        };

        // Substitute %j with actual job ID in log paths
        let stdout_path = generated.stdout_path.replace("%j", &job_id);
        let stderr_path = generated.stderr_path.replace("%j", &job_id);

        Ok(commands::SandboxedSbatchResponse {
            job_id,
            script_path: script_path_str.to_string(),
            script_content: if request.return_script || request.dry_run {
                Some(generated.content)
            } else {
                None
            },
            image_used: generated.image,
            bind_mounts: generated.bind_mounts,
            stdout_path,
            stderr_path,
            duration_ms: start.elapsed().as_millis() as u64,
        })
    }
}
