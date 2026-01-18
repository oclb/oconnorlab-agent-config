//! Remote Bridge Library
//!
//! This library provides the core functionality for the remote-bridge CLI tool.
//! It exposes the configuration types and RPC handlers for testing.

pub mod config;
pub mod commands;
pub mod ssh;
pub mod rpc;
pub mod sbatch;
pub mod error;

#[cfg(test)]
pub mod testing;
