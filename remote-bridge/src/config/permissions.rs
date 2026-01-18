#![allow(dead_code)]

use crate::error::ConfigError;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Permission configuration loaded from TOML
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PermissionConfig {
    pub paths: PathPermissions,
    #[serde(default)]
    pub resources: Option<ResourceLimits>,
    #[serde(default)]
    pub containers: ContainerConfig,
    #[serde(default)]
    pub modules: ModuleConfig,
    #[serde(default)]
    pub singularity: SingularityConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PathPermissions {
    /// Directories Claude can read from
    pub read: Vec<PathBuf>,
    /// Directories Claude can write to
    pub write: Vec<PathBuf>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResourceLimits {
    pub max_cpus: u32,
    pub max_memory_gb: u32,
    pub max_time_hours: u32,
    #[serde(default)]
    pub max_gpus: u32,
    #[serde(default = "default_max_array_size")]
    pub max_array_size: u32,
}

fn default_max_array_size() -> u32 {
    1000
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContainerConfig {
    #[serde(default)]
    pub allowed: Vec<ContainerSpec>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContainerSpec {
    pub registry: String,
    pub name: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModuleConfig {
    #[serde(default)]
    pub allowed: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SingularityConfig {
    /// Default container image (path to .sif or docker:// URI)
    pub default_image: Option<String>,
    /// Directory where generated sbatch scripts are written
    pub scripts_dir: Option<PathBuf>,
    /// Singularity cache directory
    pub cache_dir: Option<PathBuf>,
    /// Additional bind mounts (format: "path:mode" or "host:container:mode")
    #[serde(default)]
    pub extra_binds: Vec<String>,
}

impl PermissionConfig {
    /// Load config from TOML file
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        if !path.exists() {
            return Err(ConfigError::NotFound(path.to_path_buf()));
        }

        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)
            .map_err(|e| ConfigError::Parse(e.to_string()))?;

        Ok(config)
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Ensure write paths are subsets of read paths
        for write_path in &self.paths.write {
            let is_readable = self.paths.read.iter().any(|read_path| {
                write_path.starts_with(read_path)
            });
            if !is_readable {
                return Err(ConfigError::Validation(format!(
                    "Write path {} is not under any read path",
                    write_path.display()
                )));
            }
        }

        // Validate resource limits if specified
        if let Some(ref resources) = self.resources {
            if resources.max_cpus == 0 {
                return Err(ConfigError::Validation("max_cpus must be > 0".to_string()));
            }
            if resources.max_memory_gb == 0 {
                return Err(ConfigError::Validation("max_memory_gb must be > 0".to_string()));
            }
            if resources.max_time_hours == 0 {
                return Err(ConfigError::Validation("max_time_hours must be > 0".to_string()));
            }
        }

        Ok(())
    }

    /// Check if a path is allowed for reading
    pub fn is_read_allowed(&self, path: &Path) -> bool {
        self.paths.read.iter().any(|allowed| path.starts_with(allowed))
    }

    /// Check if a path is allowed for writing
    pub fn is_write_allowed(&self, path: &Path) -> bool {
        self.paths.write.iter().any(|allowed| path.starts_with(allowed))
    }

    /// Check if a container image is allowed
    pub fn is_container_allowed(&self, registry: &str, name: &str, tag: &str) -> bool {
        self.containers.allowed.iter().any(|spec| {
            spec.registry == registry
                && spec.name == name
                && spec.tags.iter().any(|t| t == tag || t == "*")
        })
    }

    /// Check if a module is allowed (empty list = allow all)
    pub fn is_module_allowed(&self, module: &str) -> bool {
        // Empty allowed list means all modules are allowed
        if self.modules.allowed.is_empty() {
            return true;
        }
        self.modules.allowed.iter().any(|m| m == module)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let toml = r#"
[paths]
read = ["/data/lab/", "/scratch/"]
write = ["/data/lab/projects/"]

[resources]
max_cpus = 32
max_memory_gb = 128
max_time_hours = 120
max_gpus = 2

[[containers.allowed]]
registry = "docker://ghcr.io"
name = "python"
tags = ["3.9", "3.10"]

[modules]
allowed = ["gcc/9.2.0", "python/3.9.14"]
"#;

        let config: PermissionConfig = toml::from_str(toml).unwrap();
        assert_eq!(config.paths.read.len(), 2);
        assert_eq!(config.resources.as_ref().unwrap().max_cpus, 32);
        assert_eq!(config.containers.allowed.len(), 1);
        assert_eq!(config.modules.allowed.len(), 2);
    }

    #[test]
    fn test_path_permissions() {
        let config = PermissionConfig {
            paths: PathPermissions {
                read: vec![PathBuf::from("/data/lab/")],
                write: vec![PathBuf::from("/data/lab/projects/")],
            },
            resources: None,  // No resource limits
            containers: ContainerConfig::default(),
            modules: ModuleConfig::default(),
            singularity: SingularityConfig::default(),
        };

        assert!(config.is_read_allowed(Path::new("/data/lab/file.txt")));
        assert!(config.is_read_allowed(Path::new("/data/lab/projects/file.txt")));
        assert!(!config.is_read_allowed(Path::new("/other/file.txt")));

        assert!(config.is_write_allowed(Path::new("/data/lab/projects/file.txt")));
        assert!(!config.is_write_allowed(Path::new("/data/lab/file.txt")));
    }
}
