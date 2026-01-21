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

        info!("Starting persistent SSH session to {}@{}", self.user, self.host);

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

        // Do interactive authentication (user sees prompts, can type password/Duo)
        let (reader, writer) = self
            .interactive_auth(reader, writer)
            .await?;

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

    /// Handle interactive authentication - proxy I/O until shell is ready
    async fn interactive_auth(
        &self,
        reader: Box<dyn Read + Send>,
        writer: Box<dyn Write + Send>,
    ) -> Result<(Box<dyn Read + Send>, Box<dyn Write + Send>), SshError> {
        use std::sync::mpsc;
        use std::sync::atomic::{AtomicBool, Ordering};

        // Sentinel for detecting shell readiness
        let sentinel = format!("__READY_{}__", Uuid::new_v4().to_string().replace("-", ""));

        // Shared state
        let done = Arc::new(AtomicBool::new(false));
        let sentinel_sent = Arc::new(AtomicBool::new(false));
        let duo_requested = Arc::new(AtomicBool::new(false)); // True after we've sent "1" for Duo push

        // Channel to send keyboard input to writer thread
        let (key_tx, key_rx) = mpsc::channel::<Vec<u8>>();
        // Channel to signal Duo prompt detected (reader -> writer)
        let (duo_tx, duo_rx) = mpsc::channel::<()>();

        // Use raw mode for stdin to get individual keystrokes
        crossterm::terminal::enable_raw_mode()
            .map_err(|e| SshError::CommandFailed(format!("Failed to enable raw mode: {}", e)))?;

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
                    Ok(0) => {
                        // EOF
                        break;
                    }
                    Ok(n) => {
                        let output = &buf[..n];

                        // Print to terminal
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
                    Err(_) => {
                        break;
                    }
                }
            }
            reader
        });

        // Thread 2: Write keyboard input, Duo response, and probes to PTY
        let done_clone2 = done.clone();
        let sentinel_sent_clone2 = sentinel_sent.clone();
        let sentinel_clone2 = sentinel.clone();

        let writer_handle = std::thread::spawn(move || {
            let mut writer = writer;
            let mut duo_sent_time: Option<std::time::Instant> = None;
            let mut last_probe = std::time::Instant::now();

            loop {
                if done_clone2.load(Ordering::Relaxed) {
                    break;
                }

                // Check for keyboard input from channel (non-blocking)
                match key_rx.recv_timeout(std::time::Duration::from_millis(50)) {
                    Ok(bytes) => {
                        writer.write_all(&bytes).ok();
                        writer.flush().ok();
                    }
                    Err(mpsc::RecvTimeoutError::Timeout) => {}
                    Err(mpsc::RecvTimeoutError::Disconnected) => break,
                }

                // Check for Duo prompt signal - send "1" to request push
                if duo_rx.try_recv().is_ok() && duo_sent_time.is_none() {
                    writer.write_all(b"1\n").ok();
                    writer.flush().ok();
                    duo_sent_time = Some(std::time::Instant::now());
                }

                // Only send shell probes after Duo auth started and 5 seconds passed
                // (gives time for user to approve push)
                if let Some(duo_time) = duo_sent_time {
                    if duo_time.elapsed() > std::time::Duration::from_secs(5)
                        && last_probe.elapsed() > std::time::Duration::from_secs(3)
                    {
                        let probe = format!("echo {}$$\n", sentinel_clone2);
                        writer.write_all(probe.as_bytes()).ok();
                        writer.flush().ok();
                        sentinel_sent_clone2.store(true, Ordering::Relaxed);
                        last_probe = std::time::Instant::now();
                    }
                }
            }
            writer
        });

        // Main thread: Read keyboard and send to writer thread
        let timeout = std::time::Duration::from_secs(120);
        let start = std::time::Instant::now();

        loop {
            if done.load(Ordering::Relaxed) {
                break;
            }

            if start.elapsed() > timeout {
                done.store(true, Ordering::Relaxed);
                crossterm::terminal::disable_raw_mode().ok();
                return Err(SshError::Timeout(120));
            }

            // Poll for keyboard input
            if crossterm::event::poll(std::time::Duration::from_millis(100)).unwrap_or(false) {
                if let Ok(crossterm::event::Event::Key(key)) = crossterm::event::read() {
                    let bytes = key_to_bytes(key);
                    if !bytes.is_empty() {
                        key_tx.send(bytes).ok();
                    }
                }
            }
        }

        // Restore terminal mode
        crossterm::terminal::disable_raw_mode().ok();

        // Wait for threads and get reader/writer back
        let reader = reader_handle.join().map_err(|_| {
            SshError::CommandFailed("Reader thread panicked".to_string())
        })?;
        let writer = writer_handle.join().map_err(|_| {
            SshError::CommandFailed("Writer thread panicked".to_string())
        })?;

        info!("Shell ready (sentinel received)");
        Ok((reader, writer))
    }

    /// Check if session is alive
    pub async fn is_connected(&self) -> bool {
        let session = self.session.lock().await;
        session.is_some()
    }

    /// Execute a command in the persistent session
    pub async fn execute(&self, command: &str, timeout_secs: u64) -> Result<CommandOutput, SshError> {
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
                        let line = accumulated[..newline_pos].trim_end_matches('\r').to_string();
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

/// Convert crossterm key event to bytes to send to PTY
fn key_to_bytes(key: crossterm::event::KeyEvent) -> Vec<u8> {
    use crossterm::event::{KeyCode, KeyModifiers};

    match key.code {
        KeyCode::Char(c) => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                // Ctrl+C = 0x03, Ctrl+D = 0x04, etc.
                vec![(c as u8) & 0x1f]
            } else {
                c.to_string().into_bytes()
            }
        }
        KeyCode::Enter => vec![b'\r'],
        KeyCode::Backspace => vec![0x7f],
        KeyCode::Tab => vec![b'\t'],
        KeyCode::Esc => vec![0x1b],
        KeyCode::Up => vec![0x1b, b'[', b'A'],
        KeyCode::Down => vec![0x1b, b'[', b'B'],
        KeyCode::Right => vec![0x1b, b'[', b'C'],
        KeyCode::Left => vec![0x1b, b'[', b'D'],
        _ => vec![],
    }
}
