mod permissions;
mod integrity;

pub use permissions::*;
pub use integrity::*;

use crate::error::ConfigError;
use std::path::Path;
use tracing::info;

/// Load config and verify integrity
pub async fn load_and_verify(path: &Path) -> Result<PermissionConfig, ConfigError> {
    // First verify integrity
    let status = verify_integrity(path).await?;

    match status {
        IntegrityStatus::Valid => {
            info!("Config integrity verified");
        }
        IntegrityStatus::Modified { stored, current } => {
            return Err(ConfigError::IntegrityViolation);
        }
        IntegrityStatus::NoBaseline => {
            info!("No checksum baseline - creating one");
            update_checksum(path).await?;
        }
    }

    // Load the config
    let config = PermissionConfig::load(path)?;
    config.validate()?;

    Ok(config)
}
