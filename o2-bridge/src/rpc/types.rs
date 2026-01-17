use serde::{Deserialize, Serialize};

/// Connection status response
#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionStatus {
    pub connected: bool,
    pub user: String,
    pub host: String,
    pub socket_path: String,
    pub instructions: Option<String>,
}

/// Generic command result
#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResult {
    pub success: bool,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
}

/// Error response for RPC
#[derive(Debug, Serialize, Deserialize)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

// Error codes
pub const ERR_NOT_CONNECTED: i32 = -32001;
pub const ERR_PERMISSION_DENIED: i32 = -32002;
pub const ERR_INVALID_PATH: i32 = -32003;
pub const ERR_INVALID_REGEX: i32 = -32004;
pub const ERR_COMMAND_FAILED: i32 = -32005;
pub const ERR_TIMEOUT: i32 = -32006;
pub const ERR_VALIDATION: i32 = -32007;
