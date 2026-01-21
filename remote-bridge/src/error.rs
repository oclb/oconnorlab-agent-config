#![allow(dead_code)]

use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BridgeError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("SSH error: {0}")]
    Ssh(#[from] SshError),

    #[error("Permission denied: {0}")]
    Permission(#[from] ValidationError),

    #[error("SLURM error: {0}")]
    Slurm(String),

    #[error("RPC error: {0}")]
    Rpc(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Config file not found: {0}")]
    NotFound(PathBuf),

    #[error("Failed to parse config: {0}")]
    Parse(String),

    #[error("Config validation failed: {0}")]
    Validation(String),

    #[error("Config integrity check failed: file was modified")]
    IntegrityViolation,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum SshError {
    #[error(
        "SSH connection not established. Run: ssh -M -S <socket> -o ControlPersist=yes -fN {0}@{1}"
    )]
    NotConnected(String, String),

    #[error("SSH connection check failed: {0}")]
    ConnectionCheckFailed(String),

    #[error("Command execution failed: {0}")]
    CommandFailed(String),

    #[error("Command timed out after {0} seconds")]
    Timeout(u64),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Path not allowed: {path}. Allowed paths: {allowed:?}")]
    PathNotAllowed { path: String, allowed: Vec<String> },

    #[error("Path traversal detected in: {0}")]
    PathTraversal(String),

    #[error("Environment variables not allowed in paths: {0}")]
    EnvironmentVariableNotAllowed(String),

    #[error("Path does not exist: {0}")]
    PathNotFound(String),

    #[error("Invalid regex pattern: {0}")]
    InvalidRegex(String),

    #[error("Resource limit exceeded: {0}")]
    ResourceLimitExceeded(String),
}
