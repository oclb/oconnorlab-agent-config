use crate::error::ConfigError;
use sha2::{Digest, Sha256};
use std::path::Path;

/// Result of integrity check
#[derive(Debug)]
pub enum IntegrityStatus {
    /// Checksum matches
    Valid,
    /// Checksum does not match
    Modified { stored: String, current: String },
    /// No checksum file exists yet
    NoBaseline,
}

/// Get the checksum file path for a config file
fn checksum_path(config_path: &Path) -> std::path::PathBuf {
    config_path.with_extension("sha256")
}

/// Compute SHA-256 checksum of a file
pub fn compute_checksum(path: &Path) -> Result<String, ConfigError> {
    let content = std::fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&content);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}

/// Verify config file integrity
pub async fn verify_integrity(config_path: &Path) -> Result<IntegrityStatus, ConfigError> {
    if !config_path.exists() {
        return Err(ConfigError::NotFound(config_path.to_path_buf()));
    }

    let checksum_file = checksum_path(config_path);

    if !checksum_file.exists() {
        return Ok(IntegrityStatus::NoBaseline);
    }

    let stored = std::fs::read_to_string(&checksum_file)?.trim().to_string();
    let current = compute_checksum(config_path)?;

    if stored == current {
        Ok(IntegrityStatus::Valid)
    } else {
        Ok(IntegrityStatus::Modified { stored, current })
    }
}

/// Update the stored checksum for a config file
pub async fn update_checksum(config_path: &Path) -> Result<(), ConfigError> {
    if !config_path.exists() {
        return Err(ConfigError::NotFound(config_path.to_path_buf()));
    }

    let checksum = compute_checksum(config_path)?;
    let checksum_file = checksum_path(config_path);

    std::fs::write(&checksum_file, &checksum)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_checksum_workflow() {
        // Create a temp config file
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "[paths]\nread = [\"/data/\"]").unwrap();
        let path = file.path();

        // Initially no baseline
        let status = verify_integrity(path).await.unwrap();
        assert!(matches!(status, IntegrityStatus::NoBaseline));

        // Create baseline
        update_checksum(path).await.unwrap();

        // Now should be valid
        let status = verify_integrity(path).await.unwrap();
        assert!(matches!(status, IntegrityStatus::Valid));

        // Modify the file
        let mut file = std::fs::OpenOptions::new().append(true).open(path).unwrap();
        writeln!(file, "# modified").unwrap();

        // Should detect modification
        let status = verify_integrity(path).await.unwrap();
        assert!(matches!(status, IntegrityStatus::Modified { .. }));
    }
}
