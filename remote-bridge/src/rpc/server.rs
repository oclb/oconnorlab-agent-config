#![allow(dead_code)]

use crate::config::PermissionConfig;
use crate::rpc::handlers::RpcState;
use crate::ssh::RemoteExecutor;
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
    ssh: Arc<dyn RemoteExecutor>,
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

    info!("RPC methods available: connection_status, ls, cat, grep, head, wc, find, download, git_pull, squeue, sacct, sbatch, sandboxed_sbatch, scancel, job_wait, shutdown");

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

/// Format parameters for logging (abbreviated for readability)
fn format_params_for_log(method: &str, params: &Option<serde_json::Value>) -> String {
    match params {
        None => String::new(),
        Some(p) => {
            // Extract key identifying info based on method
            match method {
                "ls" | "cat" | "head" | "wc" | "find" | "download" | "git_pull" => {
                    p.get("path")
                        .and_then(|v| v.as_str())
                        .map(|s| truncate_path(s))
                        .unwrap_or_default()
                }
                "grep" => {
                    let pattern = p.get("pattern").and_then(|v| v.as_str()).unwrap_or("");
                    let path = p.get("path").and_then(|v| v.as_str()).map(truncate_path).unwrap_or_default();
                    if pattern.len() > 20 {
                        format!("'{}...' {}", &pattern[..20], path)
                    } else {
                        format!("'{}' {}", pattern, path)
                    }
                }
                "sbatch" | "sandboxed_sbatch" => {
                    p.get("script_path")
                        .and_then(|v| v.as_str())
                        .map(|s| truncate_path(s))
                        .or_else(|| p.get("name").and_then(|v| v.as_str()).map(String::from))
                        .unwrap_or_else(|| "job".to_string())
                }
                "scancel" | "job_wait" => {
                    p.get("job_id")
                        .and_then(|v| v.as_str())
                        .map(String::from)
                        .unwrap_or_default()
                }
                _ => String::new(),
            }
        }
    }
}

/// Truncate a path to show just the filename or last component
fn truncate_path(path: &str) -> String {
    std::path::Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .map(String::from)
        .unwrap_or_else(|| {
            if path.len() > 30 {
                format!("...{}", &path[path.len()-27..])
            } else {
                path.to_string()
            }
        })
}

/// Dispatch a method call to the appropriate handler
async fn dispatch_method(state: &RpcState, request: JsonRpcRequest) -> JsonRpcResponse {
    let method = request.method.clone();
    let start = std::time::Instant::now();

    // Log the request (skip connection_status as it's called frequently for health checks)
    let params_str = format_params_for_log(&method, &request.params);
    if method != "connection_status" {
        if params_str.is_empty() {
            info!("--> {}", method);
        } else {
            info!("--> {} {}", method, params_str);
        }
    }

    let response = dispatch_method_inner(state, request).await;

    // Log completion (skip connection_status)
    if method != "connection_status" {
        let duration = start.elapsed();
        let status = if response.error.is_some() { "ERR" } else { "OK" };
        info!("<-- {} {} ({:.1?})", method, status, duration);
    }

    response
}

/// Inner dispatch logic
async fn dispatch_method_inner(state: &RpcState, request: JsonRpcRequest) -> JsonRpcResponse {
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

        "head" => {
            let params = match request.params {
                Some(p) => p,
                None => {
                    return JsonRpcResponse::error(id, -32602, "Missing params".to_string());
                }
            };

            match serde_json::from_value::<commands::HeadRequest>(params) {
                Ok(req) => match state.head(req).await {
                    Ok(result) => {
                        JsonRpcResponse::success(id, serde_json::to_value(result).unwrap())
                    }
                    Err(e) => JsonRpcResponse::error(id, e.code, e.message),
                },
                Err(e) => JsonRpcResponse::error(id, -32602, format!("Invalid params: {}", e)),
            }
        }

        "wc" => {
            let params = match request.params {
                Some(p) => p,
                None => {
                    return JsonRpcResponse::error(id, -32602, "Missing params".to_string());
                }
            };

            match serde_json::from_value::<commands::WcRequest>(params) {
                Ok(req) => match state.wc(req).await {
                    Ok(result) => {
                        JsonRpcResponse::success(id, serde_json::to_value(result).unwrap())
                    }
                    Err(e) => JsonRpcResponse::error(id, e.code, e.message),
                },
                Err(e) => JsonRpcResponse::error(id, -32602, format!("Invalid params: {}", e)),
            }
        }

        "find" => {
            let params = match request.params {
                Some(p) => p,
                None => {
                    return JsonRpcResponse::error(id, -32602, "Missing params".to_string());
                }
            };

            match serde_json::from_value::<commands::FindRequest>(params) {
                Ok(req) => match state.find(req).await {
                    Ok(result) => {
                        JsonRpcResponse::success(id, serde_json::to_value(result).unwrap())
                    }
                    Err(e) => JsonRpcResponse::error(id, e.code, e.message),
                },
                Err(e) => JsonRpcResponse::error(id, -32602, format!("Invalid params: {}", e)),
            }
        }

        "download" => {
            let params = match request.params {
                Some(p) => p,
                None => {
                    return JsonRpcResponse::error(id, -32602, "Missing params".to_string());
                }
            };

            match serde_json::from_value::<commands::DownloadRequest>(params) {
                Ok(req) => match state.download(req).await {
                    Ok(result) => {
                        JsonRpcResponse::success(id, serde_json::to_value(result).unwrap())
                    }
                    Err(e) => {
                        // Include data for file too large errors
                        if let Some(data) = e.data {
                            let mut response = JsonRpcResponse::error(id, e.code, e.message);
                            if let Some(ref mut err) = response.error {
                                err.data = Some(data);
                            }
                            response
                        } else {
                            JsonRpcResponse::error(id, e.code, e.message)
                        }
                    }
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

        "sandboxed_sbatch" => {
            let params = match request.params {
                Some(p) => p,
                None => {
                    return JsonRpcResponse::error(id, -32602, "Missing params".to_string());
                }
            };

            match serde_json::from_value::<commands::SandboxedSbatchRequest>(params) {
                Ok(req) => match state.sandboxed_sbatch(req).await {
                    Ok(result) => {
                        JsonRpcResponse::success(id, serde_json::to_value(result).unwrap())
                    }
                    Err(e) => JsonRpcResponse::error(id, e.code, e.message),
                },
                Err(e) => JsonRpcResponse::error(id, -32602, format!("Invalid params: {}", e)),
            }
        }

        "job_wait" => {
            let params = match request.params {
                Some(p) => p,
                None => {
                    return JsonRpcResponse::error(id, -32602, "Missing params".to_string());
                }
            };

            match serde_json::from_value::<commands::JobWaitRequest>(params) {
                Ok(req) => match state.job_wait(req).await {
                    Ok(result) => {
                        JsonRpcResponse::success(id, serde_json::to_value(result).unwrap())
                    }
                    Err(e) => JsonRpcResponse::error(id, e.code, e.message),
                },
                Err(e) => JsonRpcResponse::error(id, -32602, format!("Invalid params: {}", e)),
            }
        }

        "scancel" => {
            let params = match request.params {
                Some(p) => p,
                None => {
                    return JsonRpcResponse::error(id, -32602, "Missing params".to_string());
                }
            };

            match serde_json::from_value::<commands::ScancelRequest>(params) {
                Ok(req) => match state.scancel(req).await {
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
