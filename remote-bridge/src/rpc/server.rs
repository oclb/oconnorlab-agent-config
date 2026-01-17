#![allow(dead_code)]

use crate::config::PermissionConfig;
use crate::rpc::handlers::RpcState;
use crate::ssh::SshConnection;
use crate::commands;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixListener;
use tracing::{error, info};

/// JSON-RPC 2.0 request
#[derive(Debug, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Option<serde_json::Value>,
    id: serde_json::Value,
}

/// JSON-RPC 2.0 response
#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
    id: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
}

impl JsonRpcResponse {
    fn success(id: serde_json::Value, result: serde_json::Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: Some(result),
            error: None,
            id,
        }
    }

    fn error(id: serde_json::Value, code: i32, message: String) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JsonRpcError {
                code,
                message,
                data: None,
            }),
            id,
        }
    }
}

/// Start the JSON-RPC server on a Unix socket
pub async fn start_server(
    socket_path: PathBuf,
    config: PermissionConfig,
    ssh: Arc<SshConnection>,
) -> Result<()> {
    // Remove stale socket if it exists
    if socket_path.exists() {
        std::fs::remove_file(&socket_path)?;
    }

    // Create parent directory if needed
    if let Some(parent) = socket_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Create Unix listener
    let listener = UnixListener::bind(&socket_path)?;
    info!("Listening on Unix socket: {}", socket_path.display());

    // Create shared state
    let state = Arc::new(RpcState::new(ssh, config));

    // Check initial SSH connection status
    let status = state.connection_status().await;
    if status.connected {
        info!("SSH connection active");
    } else {
        info!("SSH not connected. Clients will receive connection instructions.");
    }

    info!("RPC methods available: connection_status, ls, cat, grep, git_pull, squeue, sacct, sbatch, shutdown");

    // Accept connections
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let state = state.clone();

                tokio::spawn(async move {
                    if let Err(e) = handle_connection(stream, state).await {
                        error!("Connection error: {}", e);
                    }
                });
            }
            Err(e) => {
                error!("Accept error: {}", e);
            }
        }
    }
}

/// Handle a single client connection
async fn handle_connection(
    stream: tokio::net::UnixStream,
    state: Arc<RpcState>,
) -> Result<()> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).await?;

        if bytes_read == 0 {
            // Connection closed
            break;
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Parse as JSON-RPC request
        let response = match serde_json::from_str::<JsonRpcRequest>(line) {
            Ok(request) => dispatch_method(&state, request).await,
            Err(e) => JsonRpcResponse::error(
                serde_json::Value::Null,
                -32700,
                format!("Parse error: {}", e),
            ),
        };

        // Send response
        let response_str = serde_json::to_string(&response)? + "\n";
        writer.write_all(response_str.as_bytes()).await?;
        writer.flush().await?;
    }

    Ok(())
}

/// Dispatch a method call to the appropriate handler
async fn dispatch_method(state: &RpcState, request: JsonRpcRequest) -> JsonRpcResponse {
    let id = request.id.clone();

    match request.method.as_str() {
        "connection_status" => {
            let result = state.connection_status().await;
            JsonRpcResponse::success(id, serde_json::to_value(result).unwrap())
        }

        "ls" => {
            let params = match request.params {
                Some(p) => p,
                None => {
                    return JsonRpcResponse::error(id, -32602, "Missing params".to_string());
                }
            };

            match serde_json::from_value::<commands::LsRequest>(params) {
                Ok(req) => match state.ls(req).await {
                    Ok(result) => {
                        JsonRpcResponse::success(id, serde_json::to_value(result).unwrap())
                    }
                    Err(e) => JsonRpcResponse::error(id, e.code, e.message),
                },
                Err(e) => JsonRpcResponse::error(id, -32602, format!("Invalid params: {}", e)),
            }
        }

        "cat" => {
            let params = match request.params {
                Some(p) => p,
                None => {
                    return JsonRpcResponse::error(id, -32602, "Missing params".to_string());
                }
            };

            match serde_json::from_value::<commands::CatRequest>(params) {
                Ok(req) => match state.cat(req).await {
                    Ok(result) => {
                        JsonRpcResponse::success(id, serde_json::to_value(result).unwrap())
                    }
                    Err(e) => JsonRpcResponse::error(id, e.code, e.message),
                },
                Err(e) => JsonRpcResponse::error(id, -32602, format!("Invalid params: {}", e)),
            }
        }

        "grep" => {
            let params = match request.params {
                Some(p) => p,
                None => {
                    return JsonRpcResponse::error(id, -32602, "Missing params".to_string());
                }
            };

            match serde_json::from_value::<commands::GrepRequest>(params) {
                Ok(req) => match state.grep(req).await {
                    Ok(result) => {
                        JsonRpcResponse::success(id, serde_json::to_value(result).unwrap())
                    }
                    Err(e) => JsonRpcResponse::error(id, e.code, e.message),
                },
                Err(e) => JsonRpcResponse::error(id, -32602, format!("Invalid params: {}", e)),
            }
        }

        "git_pull" => {
            let params = match request.params {
                Some(p) => p,
                None => {
                    return JsonRpcResponse::error(id, -32602, "Missing params".to_string());
                }
            };

            match serde_json::from_value::<commands::GitPullRequest>(params) {
                Ok(req) => match state.git_pull(req).await {
                    Ok(result) => {
                        JsonRpcResponse::success(id, serde_json::to_value(result).unwrap())
                    }
                    Err(e) => JsonRpcResponse::error(id, e.code, e.message),
                },
                Err(e) => JsonRpcResponse::error(id, -32602, format!("Invalid params: {}", e)),
            }
        }

        "squeue" => {
            let params = request.params.unwrap_or(serde_json::json!({}));

            match serde_json::from_value::<commands::SqueueRequest>(params) {
                Ok(req) => match state.squeue(req).await {
                    Ok(result) => {
                        JsonRpcResponse::success(id, serde_json::to_value(result).unwrap())
                    }
                    Err(e) => JsonRpcResponse::error(id, e.code, e.message),
                },
                Err(e) => JsonRpcResponse::error(id, -32602, format!("Invalid params: {}", e)),
            }
        }

        "sacct" => {
            let params = request.params.unwrap_or(serde_json::json!({}));

            match serde_json::from_value::<commands::SacctRequest>(params) {
                Ok(req) => match state.sacct(req).await {
                    Ok(result) => {
                        JsonRpcResponse::success(id, serde_json::to_value(result).unwrap())
                    }
                    Err(e) => JsonRpcResponse::error(id, e.code, e.message),
                },
                Err(e) => JsonRpcResponse::error(id, -32602, format!("Invalid params: {}", e)),
            }
        }

        "sbatch" => {
            let params = match request.params {
                Some(p) => p,
                None => {
                    return JsonRpcResponse::error(id, -32602, "Missing params".to_string());
                }
            };

            match serde_json::from_value::<commands::SbatchRequest>(params) {
                Ok(req) => match state.sbatch(req).await {
                    Ok(result) => {
                        JsonRpcResponse::success(id, serde_json::to_value(result).unwrap())
                    }
                    Err(e) => JsonRpcResponse::error(id, e.code, e.message),
                },
                Err(e) => JsonRpcResponse::error(id, -32602, format!("Invalid params: {}", e)),
            }
        }

        "shutdown" => {
            info!("Shutdown requested via RPC");
            JsonRpcResponse::success(id, serde_json::json!("shutting down"))
        }

        _ => JsonRpcResponse::error(id, -32601, format!("Method not found: {}", request.method)),
    }
}
