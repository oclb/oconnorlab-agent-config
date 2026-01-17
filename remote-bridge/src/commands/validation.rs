#![allow(dead_code)]

use crate::config::PermissionConfig;
use crate::error::ValidationError;
use std::path::{Component, Path, PathBuf};

/// A validated path that has been checked against permissions
#[derive(Debug, Clone)]
pub struct ValidatedPath {
    path: PathBuf,
    original: String,
}

impl ValidatedPath {
    fn new(path: PathBuf, original: String) -> Self {
        Self { path, original }
    }

    pub fn as_path(&self) -> &Path {
        &self.path
    }

    pub fn as_str(&self) -> &str {
        self.path.to_str().unwrap_or("")
    }
}

impl std::fmt::Display for ValidatedPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}

/// Path validator that checks against permission config
pub struct PathValidator {
    config: PermissionConfig,
    home_dir: Option<PathBuf>,
}

impl PathValidator {
    pub fn new(config: PermissionConfig) -> Self {
        // Note: home_dir on the remote system should be fetched dynamically
        // For now, we'll handle ~ expansion on the remote side
        Self {
            config,
            home_dir: None,
        }
    }

    /// Set the remote home directory (fetched from SSH)
    pub fn set_home_dir(&mut self, home: PathBuf) {
        self.home_dir = Some(home);
    }

    /// Validate a path for reading
    pub fn validate_read_path(&self, path: &str) -> Result<ValidatedPath, ValidationError> {
        let canonical = self.canonicalize(path)?;

        if !self.config.is_read_allowed(&canonical) {
            return Err(ValidationError::PathNotAllowed {
                path: path.to_string(),
                allowed: self
                    .config
                    .paths
                    .read
                    .iter()
                    .map(|p| p.display().to_string())
                    .collect(),
            });
        }

        Ok(ValidatedPath::new(canonical, path.to_string()))
    }

    /// Validate a path for writing
    pub fn validate_write_path(&self, path: &str) -> Result<ValidatedPath, ValidationError> {
        let canonical = self.canonicalize(path)?;

        if !self.config.is_write_allowed(&canonical) {
            return Err(ValidationError::PathNotAllowed {
                path: path.to_string(),
                allowed: self
                    .config
                    .paths
                    .write
                    .iter()
                    .map(|p| p.display().to_string())
                    .collect(),
            });
        }

        Ok(ValidatedPath::new(canonical, path.to_string()))
    }

    /// Canonicalize a path (lexical, without filesystem access)
    fn canonicalize(&self, path: &str) -> Result<PathBuf, ValidationError> {
        // Reject environment variables
        if path.contains('$') {
            return Err(ValidationError::EnvironmentVariableNotAllowed(
                path.to_string(),
            ));
        }

        // Expand ~ if we have home dir
        let expanded = if path.starts_with("~/") {
            if let Some(ref home) = self.home_dir {
                home.join(&path[2..])
            } else {
                // Keep the ~ for now; will be expanded on remote
                PathBuf::from(path)
            }
        } else if path == "~" {
            if let Some(ref home) = self.home_dir {
                home.clone()
            } else {
                PathBuf::from(path)
            }
        } else {
            PathBuf::from(path)
        };

        // Lexical canonicalization to catch path traversal
        let mut result = PathBuf::new();
        for component in expanded.components() {
            match component {
                Component::ParentDir => {
                    // Check if we're trying to go above root or allowed paths
                    if !result.pop() {
                        return Err(ValidationError::PathTraversal(path.to_string()));
                    }
                }
                Component::Normal(c) => {
                    result.push(c);
                }
                Component::RootDir => {
                    result.push("/");
                }
                Component::CurDir => {
                    // Skip "."
                }
                Component::Prefix(_) => {
                    // Windows-only, ignore
                }
            }
        }

        // Ensure the path is absolute
        if !result.is_absolute() {
            return Err(ValidationError::PathNotAllowed {
                path: path.to_string(),
                allowed: vec!["Paths must be absolute".to_string()],
            });
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{PathPermissions, PermissionConfig, ResourceLimits};

    fn test_config() -> PermissionConfig {
        PermissionConfig {
            paths: PathPermissions {
                read: vec![PathBuf::from("/data/lab/"), PathBuf::from("/scratch/")],
                write: vec![PathBuf::from("/data/lab/projects/")],
            },
            resources: ResourceLimits {
                max_cpus: 8,
                max_memory_gb: 32,
                max_time_hours: 24,
                max_gpus: 0,
                max_array_size: 100,
            },
            containers: Default::default(),
            modules: Default::default(),
        }
    }

    #[test]
    fn test_valid_read_path() {
        let validator = PathValidator::new(test_config());

        let result = validator.validate_read_path("/data/lab/file.txt");
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_read_path() {
        let validator = PathValidator::new(test_config());

        let result = validator.validate_read_path("/etc/passwd");
        assert!(result.is_err());
    }

    #[test]
    fn test_path_traversal_blocked() {
        let validator = PathValidator::new(test_config());

        // Try to escape via ..
        let result = validator.validate_read_path("/data/lab/../../../etc/passwd");
        assert!(result.is_err());
    }

    #[test]
    fn test_env_var_blocked() {
        let validator = PathValidator::new(test_config());

        let result = validator.validate_read_path("$HOME/file.txt");
        assert!(matches!(
            result,
            Err(ValidationError::EnvironmentVariableNotAllowed(_))
        ));
    }

    #[test]
    fn test_write_path_validation() {
        let validator = PathValidator::new(test_config());

        // Valid write path
        let result = validator.validate_write_path("/data/lab/projects/output.txt");
        assert!(result.is_ok());

        // Read-only path
        let result = validator.validate_write_path("/data/lab/readonly.txt");
        assert!(result.is_err());
    }
}
