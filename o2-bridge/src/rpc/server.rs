use crate::commands;
use crate::config::PermissionConfig;
use crate::rpc::handlers::RpcState;
use crate::ssh::SshConnection;
use anyhow::Result;
use jsonrpsee::server::{RpcModule, Server};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::UnixListener;
use tracing::{error, info};

/// Start the JSON-RPC server on a Unix socket
pub async fn start_server(
    socket_path: PathBuf,
    config: PermissionConfig,
    ssh: SshConnection,
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

    // Build RPC module
    let mut module = RpcModule::new(state.clone());

    // Register connection_status method
    module.register_async_method("connection_status", |_, ctx, _| async move {
        Ok(ctx.connection_status().await)
    })?;

    // Register ls method
    module.register_async_method("ls", |params, ctx, _| async move {
        let request: commands::LsRequest = params.parse()?;
        ctx.ls(request).await.map_err(|e| {
            jsonrpsee::types::ErrorObjectOwned::owned(e.code, e.message, e.data)
        })
    })?;

    // Register cat method
    module.register_async_method("cat", |params, ctx, _| async move {
        let request: commands::CatRequest = params.parse()?;
        ctx.cat(request).await.map_err(|e| {
            jsonrpsee::types::ErrorObjectOwned::owned(e.code, e.message, e.data)
        })
    })?;

    // Register grep method
    module.register_async_method("grep", |params, ctx, _| async move {
        let request: commands::GrepRequest = params.parse()?;
        ctx.grep(request).await.map_err(|e| {
            jsonrpsee::types::ErrorObjectOwned::owned(e.code, e.message, e.data)
        })
    })?;

    // Register shutdown method
    module.register_async_method("shutdown", |_, _, _| async move {
        info!("Shutdown requested via RPC");
        // This will be handled by the main loop
        Ok::<_, jsonrpsee::types::ErrorObjectOwned>("shutting down")
    })?;

    info!("RPC methods registered: connection_status, ls, cat, grep, shutdown");

    // Check initial SSH connection status
    let status = state.connection_status().await;
    if status.connected {
        info!("SSH connection active");
    } else {
        info!("SSH not connected. Clients will receive connection instructions.");
    }

    // Accept connections
    // Note: jsonrpsee doesn't directly support Unix sockets, so we use a custom approach
    // For now, we'll use a simple line-based JSON-RPC over the socket
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let module = module.clone();
                let state = state.clone();

                tokio::spawn(async move {
                    if let Err(e) = handle_connection(stream, module, state).await {
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
    module: RpcModule<Arc<RpcState>>,
    _state: Arc<RpcState>,
) -> Result<()> {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

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
        let response = match serde_json::from_str::<jsonrpsee::types::Request<'_>>(line) {
            Ok(request) => {
                // Execute the method
                let method = request.method.as_ref();
                let id = request.id.clone();

                let result = module.call(method, request.params).await;

                match result {
                    Ok(response) => {
                        serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "result": serde_json::from_str::<serde_json::Value>(&response.result).unwrap_or(serde_json::Value::Null)
                        })
                    }
                    Err(e) => {
                        serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": id,
                            "error": {
                                "code": -32603,
                                "message": e.to_string()
                            }
                        })
                    }
                }
            }
            Err(e) => {
                serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": null,
                    "error": {
                        "code": -32700,
                        "message": format!("Parse error: {}", e)
                    }
                })
            }
        };

        // Send response
        let response_str = serde_json::to_string(&response)? + "\n";
        writer.write_all(response_str.as_bytes()).await?;
        writer.flush().await?;
    }

    Ok(())
}
