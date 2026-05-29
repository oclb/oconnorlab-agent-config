#![allow(dead_code)]

use crate::error::SshError;
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::{Read, Write};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info};
use uuid::Uuid;

/// SSH connection with persistent session via PTY
pub struct SshConnection {
    user: String,
    host: String,
    session: Arc<Mutex<Option<PtySession>>>,
}

/// A persistent SSH session using a pseudo-terminal
struct PtySession {
    master: Box<dyn portable_pty::MasterPty + Send>,
    reader: Box<dyn Read + Send>,
    writer: Box<dyn Write + Send>,
}

/// Output from a remote command
#[derive(Debug)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

impl SshConnection {
    pub fn new(user: String, host: String) -> Self {
        Self {
            user,
            host,
            session: Arc::new(Mutex::new(None)),
        }
    }

    /// Start the persistent SSH session with interactive authentication via PTY
    pub async fn connect(&self) -> Result<(), SshError> {
        let session_guard = self.session.lock().await;

        if session_guard.is_some() {
            return Ok(()); // Already connected
        }

        info!(
            "Starting persistent SSH session to {}@{}",
            self.user, self.host
        );

        // Create a PTY - this gives SSH a real terminal
        let pty_system = native_pty_system();
        let pair = pty_system
            .openpty(PtySize {
                rows: 24,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| SshError::CommandFailed(format!("Failed to open PTY: {}", e)))?;

        // Build SSH command
        let mut cmd = CommandBuilder::new("ssh");
        cmd.arg("-tt");
        cmd.arg("-o");
        cmd.arg("StrictHostKeyChecking=accept-new");
        cmd.arg("-o");
        cmd.arg("ServerAliveInterval=60");
        cmd.arg("-o");
        cmd.arg("ServerAliveCountMax=3");
        cmd.arg(format!("{}@{}", self.user, self.host));

        // Pass through SSH agent socket for key-based auth
        if let Ok(auth_sock) = std::env::var("SSH_AUTH_SOCK") {
            cmd.env("SSH_AUTH_SOCK", auth_sock);
        }

        // Spawn SSH in the PTY
        let _child = pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| SshError::CommandFailed(format!("Failed to spawn SSH: {}", e)))?;

        // Get reader/writer for the master side
        let reader = pair
            .master
            .try_clone_reader()
            .map_err(|e| SshError::CommandFailed(format!("Failed to get PTY reader: {}", e)))?;
        let writer = pair
            .master
            .take_writer()
            .map_err(|e| SshError::CommandFailed(format!("Failed to get PTY writer: {}", e)))?;

        // Drop lock during interactive auth
        drop(session_guard);

        // Handle Duo authentication and wait for shell readiness
        let (reader, writer) = self.interactive_auth(reader, writer).await?;

        // Re-acquire lock and store session
        let mut session_guard = self.session.lock().await;
        *session_guard = Some(PtySession {
            master: pair.master,
            reader,
            writer,
        });

        info!("SSH session established");
        Ok(())
    }

    /// Handle Duo authentication and wait for shell readiness.
    /// SSH key auth is required - no password input is supported.
    async fn interactive_auth(
        &self,
        reader: Box<dyn Read + Send>,
        writer: Box<dyn Write + Send>,
    ) -> Result<(Box<dyn Read + Send>, Box<dyn Write + Send>), SshError> {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::mpsc;

        // Sentinel for detecting shell readiness
        let sentinel = format!("__READY_{}__", Uuid::new_v4().to_string().replace("-", ""));

        // Shared state
        let done = Arc::new(AtomicBool::new(false));
        let sentinel_sent = Arc::new(AtomicBool::new(false));
        let duo_requested = Arc::new(AtomicBool::new(false));

        // Channel to signal Duo prompt detected (reader -> writer)
        let (duo_tx, duo_rx) = mpsc::channel::<()>();

        let sentinel_clone = sentinel.clone();
        let done_clone = done.clone();
        let sentinel_sent_clone = sentinel_sent.clone();
        let duo_requested_clone = duo_requested.clone();

        // Thread 1: Read from PTY and display, detect Duo prompt and sentinel
        let reader_handle = std::thread::spawn(move || {
            let mut reader = reader;
            let mut buf = [0u8; 1024];
            let mut accumulated = String::new();
            let mut term_stdout = std::io::stdout();

            loop {
                if done_clone.load(Ordering::Relaxed) {
                    break;
                }

                match reader.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        let output = &buf[..n];

                        // Print to terminal so user sees Duo prompts
                        term_stdout.write_all(output).ok();
                        term_stdout.flush().ok();

                        // Accumulate for detection
                        if let Ok(s) = std::str::from_utf8(output) {
                            accumulated.push_str(s);
                            if accumulated.len() > 4000 {
                                accumulated = accumulated[accumulated.len() - 2000..].to_string();
                            }

                            // Detect Duo prompt and signal writer to send "1" for push
                            if !duo_requested_clone.load(Ordering::Relaxed)
                                && accumulated.contains("Passcode or option")
                            {
                                duo_tx.send(()).ok();
                                duo_requested_clone.store(true, Ordering::Relaxed);
                            }

                            // Check for sentinel followed by digits (proves shell expansion, not local echo)
                            // Local echo shows literal "$$", but shell expands to PID
                            if sentinel_sent_clone.load(Ordering::Relaxed) {
                                let pattern = format!(r"{}\d+", regex::escape(&sentinel_clone));
                                if let Ok(re) = regex::Regex::new(&pattern) {
                                    if re.is_match(&accumulated) {
                                        done_clone.store(true, Ordering::Relaxed);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
            reader
        });

        // Thread 2: Write Duo response and shell probes to PTY
        let done_clone2 = done.clone();
        let sentinel_sent_clone2 = sentinel_sent.clone();
        let sentinel_clone2 = sentinel.clone();

        let writer_handle = std::thread::spawn(move || {
            let mut writer = writer;
            let mut duo_sent_time: Option<std::time::Instant> = None;
            let mut last_probe = std::time::Instant::now();
            let start = std::time::Instant::now();

            loop {
                if done_clone2.load(Ordering::Relaxed) {
                    break;
                }

                std::thread::sleep(std::time::Duration::from_millis(50));

                // Check for Duo prompt signal - send "1" to request push
                if duo_rx.try_recv().is_ok() && duo_sent_time.is_none() {
                    writer.write_all(b"1\n").ok();
                    writer.flush().ok();
                    duo_sent_time = Some(std::time::Instant::now());
                }

                // Determine when to start probing:
                // - After Duo: wait 5s for user to approve push
                // - No Duo (on-campus): start probing after 3s
                let should_probe = match duo_sent_time {
                    Some(duo_time) => duo_time.elapsed() > std::time::Duration::from_secs(5),
                    None => start.elapsed() > std::time::Duration::from_secs(3),
                };

                if should_probe && last_probe.elapsed() > std::time::Duration::from_secs(3) {
                    let probe = format!("echo {}$$\n", sentinel_clone2);
                    writer.write_all(probe.as_bytes()).ok();
                    writer.flush().ok();
                    sentinel_sent_clone2.store(true, Ordering::Relaxed);
                    last_probe = std::time::Instant::now();
                }
            }
            writer
        });

        // Main thread: wait for shell readiness (no keyboard input needed with SSH key auth)
        let timeout = std::time::Duration::from_secs(120);
        let start = std::time::Instant::now();

        loop {
            if done.load(Ordering::Relaxed) {
                break;
            }

            if start.elapsed() > timeout {
                done.store(true, Ordering::Relaxed);
                return Err(SshError::Timeout(120));
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        // Wait for threads and get reader/writer back
        let reader = reader_handle
            .join()
            .map_err(|_| SshError::CommandFailed("Reader thread panicked".to_string()))?;
        let writer = writer_handle
            .join()
            .map_err(|_| SshError::CommandFailed("Writer thread panicked".to_string()))?;

        info!("Shell ready (sentinel received)");
        Ok((reader, writer))
    }

    /// Check if session is alive
    pub async fn is_connected(&self) -> bool {
        let session = self.session.lock().await;
        session.is_some()
    }

    /// Execute a command in the persistent session
    pub async fn execute(
        &self,
        command: &str,
        timeout_secs: u64,
    ) -> Result<CommandOutput, SshError> {
        let mut session_guard = self.session.lock().await;

        let session = session_guard
            .as_mut()
            .ok_or_else(|| SshError::NotConnected(self.user.clone(), self.host.clone()))?;

        // Generate unique sentinel markers
        let sentinel_start = format!("__START_{}__", Uuid::new_v4().to_string().replace("-", ""));
        let sentinel_end = format!("__END_{}__", Uuid::new_v4().to_string().replace("-", ""));

        // Construct command with sentinels
        let wrapped_command = format!(
            "echo '{}'; {} 2>&1; echo '{}'{}\n",
            sentinel_start, command, sentinel_end, "$?"
        );

        debug!("Sending command: {}", command);

        // Send command
        session
            .writer
            .write_all(wrapped_command.as_bytes())
            .map_err(SshError::Io)?;
        session.writer.flush().map_err(SshError::Io)?;

        // Read output until we see end sentinel
        let mut output_lines = Vec::new();
        let mut capturing = false;
        let mut exit_code = 0;
        let mut buf = [0u8; 4096];
        let mut accumulated = String::new();

        let timeout = std::time::Duration::from_secs(timeout_secs);
        let start = std::time::Instant::now();

        loop {
            if start.elapsed() > timeout {
                return Err(SshError::Timeout(timeout_secs));
            }

            match session.reader.read(&mut buf) {
                Ok(0) => {
                    return Err(SshError::CommandFailed("Connection closed".to_string()));
                }
                Ok(n) => {
                    if let Ok(s) = std::str::from_utf8(&buf[..n]) {
                        accumulated.push_str(s);
                    }

                    // Process complete lines
                    while let Some(newline_pos) = accumulated.find('\n') {
                        let line = accumulated[..newline_pos]
                            .trim_end_matches('\r')
                            .to_string();
                        accumulated = accumulated[newline_pos + 1..].to_string();

                        debug!("Read line: {}", line);

                        if line.contains(&sentinel_start) {
                            capturing = true;
                            continue;
                        }

                        if line.contains(&sentinel_end) {
                            // Extract exit code
                            if let Some(code_str) = line.strip_prefix(&sentinel_end) {
                                exit_code = code_str.trim().parse().unwrap_or(0);
                            }
                            let stdout = output_lines.join("\n");
                            return Ok(CommandOutput {
                                stdout,
                                stderr: String::new(),
                                exit_code,
                            });
                        }

                        if capturing {
                            output_lines.push(line);
                        }
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
                Err(e) => {
                    return Err(SshError::Io(e));
                }
            }
        }
    }

    /// Execute with argument escaping
    pub async fn execute_with_args(
        &self,
        program: &str,
        args: &[&str],
        timeout_secs: u64,
    ) -> Result<CommandOutput, SshError> {
        let escaped_args: Vec<String> = args
            .iter()
            .map(|arg| {
                let escaped = arg.replace("'", "'\"'\"'");
                format!("'{}'", escaped)
            })
            .collect();

        let command = format!("{} {}", program, escaped_args.join(" "));
        self.execute(&command, timeout_secs).await
    }

    /// Close the session
    pub async fn close(&self) {
        let mut session = self.session.lock().await;
        if let Some(mut sess) = session.take() {
            let _ = sess.writer.write_all(b"exit\n");
            let _ = sess.writer.flush();
            std::thread::sleep(std::time::Duration::from_millis(500));
            info!("SSH session closed");
        }
    }

    pub fn user(&self) -> &str {
        &self.user
    }

    pub fn host(&self) -> &str {
        &self.host
    }
}
