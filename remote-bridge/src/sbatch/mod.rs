//! Sandboxed sbatch script generation
//!
//! This module generates SLURM sbatch scripts that run commands inside
//! Singularity containers with restricted bind mounts for security.

use crate::commands::{BindMount, SandboxedSbatchRequest};
use crate::config::{PermissionConfig, SingularityConfig};
use std::fmt::Write;
use std::path::Path;

/// Error type for script generation
#[derive(Debug, thiserror::Error)]
pub enum ScriptError {
    #[error("No container image specified and no default configured")]
    NoImage,

    #[error("Singularity scripts_dir not configured")]
    NoScriptsDir,

    #[error("Input path not allowed: {0}")]
    InputPathNotAllowed(String),

    #[error("Output path not allowed: {0}")]
    OutputPathNotAllowed(String),

    #[error("Resource limit exceeded: {0}")]
    ResourceLimitExceeded(String),

    #[error("Invalid array specification: {0}")]
    InvalidArraySpec(String),
}

/// Result of script generation
pub struct GeneratedScript {
    /// The script content
    pub content: String,
    /// Suggested filename (based on job name and timestamp)
    pub filename: String,
    /// The container image that will be used
    pub image: String,
    /// All bind mounts that were configured
    pub bind_mounts: Vec<BindMount>,
}

/// Generate a sandboxed sbatch script
pub fn generate_script(
    request: &SandboxedSbatchRequest,
    config: &PermissionConfig,
) -> Result<GeneratedScript, ScriptError> {
    let singularity = &config.singularity;

    // Determine container image
    let image = request
        .image
        .clone()
        .or_else(|| singularity.default_image.clone())
        .ok_or(ScriptError::NoImage)?;

    // Validate and collect bind mounts
    let bind_mounts = collect_bind_mounts(request, config, singularity)?;

    // Validate resource limits
    validate_resources(request, config)?;

    // Validate array spec if provided
    if let Some(array) = &request.array {
        validate_array_spec(array, config)?;
    }

    // Generate the script content
    let content = generate_script_content(request, &image, &bind_mounts, singularity);

    // Generate filename
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let safe_name = sanitize_job_name(&request.job_name);
    let filename = format!("{}_{}.sh", safe_name, timestamp);

    Ok(GeneratedScript {
        content,
        filename,
        image,
        bind_mounts,
    })
}

/// Collect and validate all bind mounts
fn collect_bind_mounts(
    request: &SandboxedSbatchRequest,
    config: &PermissionConfig,
    singularity: &SingularityConfig,
) -> Result<Vec<BindMount>, ScriptError> {
    let mut mounts = Vec::new();

    // Add input paths as read-only
    for path in &request.input_paths {
        if !config.is_read_allowed(Path::new(path.as_str())) {
            return Err(ScriptError::InputPathNotAllowed(path.clone()));
        }
        mounts.push(BindMount {
            host: path.clone(),
            container: path.clone(),
            mode: "ro".to_string(),
        });
    }

    // Add output paths as read-write
    for path in &request.output_paths {
        if !config.is_write_allowed(Path::new(path.as_str())) {
            return Err(ScriptError::OutputPathNotAllowed(path.clone()));
        }
        mounts.push(BindMount {
            host: path.clone(),
            container: path.clone(),
            mode: "rw".to_string(),
        });
    }

    // Add working directory if specified (must be writable)
    if let Some(wd) = &request.working_dir {
        if !config.is_write_allowed(Path::new(wd.as_str())) {
            return Err(ScriptError::OutputPathNotAllowed(wd.clone()));
        }
        // Only add if not already in output_paths
        if !request.output_paths.contains(wd) {
            mounts.push(BindMount {
                host: wd.clone(),
                container: wd.clone(),
                mode: "rw".to_string(),
            });
        }
    }

    // Add extra binds from config
    for bind_spec in &singularity.extra_binds {
        if let Some(mount) = parse_bind_spec(bind_spec) {
            mounts.push(mount);
        }
    }

    Ok(mounts)
}

/// Parse a bind spec string like "path:mode" or "host:container:mode"
fn parse_bind_spec(spec: &str) -> Option<BindMount> {
    let parts: Vec<&str> = spec.split(':').collect();
    match parts.len() {
        2 => {
            // "path:mode" format
            Some(BindMount {
                host: parts[0].to_string(),
                container: parts[0].to_string(),
                mode: parts[1].to_string(),
            })
        }
        3 => {
            // "host:container:mode" format
            Some(BindMount {
                host: parts[0].to_string(),
                container: parts[1].to_string(),
                mode: parts[2].to_string(),
            })
        }
        _ => None,
    }
}

/// Validate resource limits against config (if limits are configured)
fn validate_resources(
    request: &SandboxedSbatchRequest,
    config: &PermissionConfig,
) -> Result<(), ScriptError> {
    // If no resource limits configured, let O2 handle enforcement
    let limits = match &config.resources {
        Some(limits) => limits,
        None => return Ok(()),
    };

    if request.cpus > limits.max_cpus {
        return Err(ScriptError::ResourceLimitExceeded(format!(
            "CPUs {} exceeds max {}",
            request.cpus, limits.max_cpus
        )));
    }

    if let Some(mem) = &request.memory {
        let mem_gb = mem.to_gb();
        if mem_gb > limits.max_memory_gb {
            return Err(ScriptError::ResourceLimitExceeded(format!(
                "Memory {}GB exceeds max {}GB",
                mem_gb,
                limits.max_memory_gb
            )));
        }
    }

    if let Some(time) = &request.time {
        let total_hours = time.total_hours();
        if total_hours > limits.max_time_hours {
            return Err(ScriptError::ResourceLimitExceeded(format!(
                "Time {}h exceeds max {}h",
                total_hours,
                limits.max_time_hours
            )));
        }
    }

    if let Some(gpu) = &request.gpu {
        if gpu.count > limits.max_gpus {
            return Err(ScriptError::ResourceLimitExceeded(format!(
                "GPUs {} exceeds max {}",
                gpu.count, limits.max_gpus
            )));
        }
    }

    Ok(())
}

/// Validate array job specification
fn validate_array_spec(spec: &str, config: &PermissionConfig) -> Result<(), ScriptError> {
    // Parse array spec like "1-100" or "1-100%10"
    let spec_part = spec.split('%').next().unwrap_or(spec);

    // Extract the range
    if let Some((start, end)) = spec_part.split_once('-') {
        let start: u32 = start
            .parse()
            .map_err(|_| ScriptError::InvalidArraySpec(spec.to_string()))?;
        let end: u32 = end
            .parse()
            .map_err(|_| ScriptError::InvalidArraySpec(spec.to_string()))?;

        // Only check array size limit if resources are configured
        if let Some(limits) = &config.resources {
            let array_size = end - start + 1;
            if array_size > limits.max_array_size {
                return Err(ScriptError::ResourceLimitExceeded(format!(
                    "Array size {} exceeds max {}",
                    array_size, limits.max_array_size
                )));
            }
        }
    }

    Ok(())
}

/// Generate the actual script content
fn generate_script_content(
    request: &SandboxedSbatchRequest,
    image: &str,
    bind_mounts: &[BindMount],
    singularity: &SingularityConfig,
) -> String {
    let mut script = String::new();

    // Shebang
    writeln!(script, "#!/bin/bash").unwrap();
    writeln!(script).unwrap();

    // SLURM directives
    writeln!(script, "#SBATCH --job-name={}", request.job_name).unwrap();

    if let Some(partition) = &request.partition {
        writeln!(script, "#SBATCH --partition={}", partition.as_str()).unwrap();
    }

    writeln!(script, "#SBATCH --cpus-per-task={}", request.cpus).unwrap();

    if let Some(mem) = &request.memory {
        writeln!(script, "#SBATCH --mem={}", mem.to_slurm_format()).unwrap();
    }

    if let Some(time) = &request.time {
        writeln!(script, "#SBATCH --time={}", time.to_slurm_format()).unwrap();
    }

    if let Some(gpu) = &request.gpu {
        writeln!(script, "#SBATCH --gres={}", gpu.to_slurm_gres()).unwrap();
    }

    if let Some(array) = &request.array {
        writeln!(script, "#SBATCH --array={}", array).unwrap();
    }

    if let Some(output) = &request.output {
        writeln!(script, "#SBATCH --output={}", output).unwrap();
    }

    if let Some(error) = &request.error {
        writeln!(script, "#SBATCH --error={}", error).unwrap();
    }

    // Extra directives
    for (key, value) in &request.extra_directives {
        writeln!(script, "#SBATCH --{}={}", key, value).unwrap();
    }

    writeln!(script).unwrap();

    // Header comment
    writeln!(script, "# Generated by remote-bridge sandboxed_sbatch").unwrap();
    writeln!(script, "# Container: {}", image).unwrap();
    writeln!(script, "# Bind mounts:").unwrap();
    for mount in bind_mounts {
        writeln!(
            script,
            "#   {}:{} ({})",
            mount.host, mount.container, mount.mode
        )
        .unwrap();
    }
    writeln!(script).unwrap();

    // Load singularity module
    writeln!(script, "module load singularity").unwrap();
    writeln!(script).unwrap();

    // Set singularity cache directory if configured
    if let Some(cache_dir) = &singularity.cache_dir {
        writeln!(
            script,
            "export SINGULARITY_CACHEDIR={}",
            cache_dir.display()
        )
        .unwrap();
        writeln!(script).unwrap();
    }

    // Export environment variables
    for (key, value) in &request.environment {
        // Escape value for shell
        let escaped = value.replace('\'', "'\"'\"'");
        writeln!(script, "export {}='{}'", key, escaped).unwrap();
    }
    if !request.environment.is_empty() {
        writeln!(script).unwrap();
    }

    // Build singularity exec command
    writeln!(script, "singularity exec \\").unwrap();

    // Add bind mounts
    for mount in bind_mounts {
        writeln!(
            script,
            "    --bind {}:{}:{} \\",
            mount.host, mount.container, mount.mode
        )
        .unwrap();
    }

    // Set working directory if specified
    if let Some(wd) = &request.working_dir {
        writeln!(script, "    --pwd {} \\", wd).unwrap();
    }

    // Container image
    writeln!(script, "    {} \\", image).unwrap();

    // Command to execute
    writeln!(script, "    {}", request.command).unwrap();

    script
}

/// Sanitize job name for use in filename
fn sanitize_job_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::{MemorySpec, MemoryUnit, Partition, TimeSpec};
    use crate::config::{
        ContainerConfig, ModuleConfig, PathPermissions, PermissionConfig, ResourceLimits,
        SingularityConfig,
    };
    use std::path::PathBuf;

    fn test_config() -> PermissionConfig {
        PermissionConfig {
            paths: PathPermissions {
                read: vec![
                    PathBuf::from("/data/input/"),
                    PathBuf::from("/data/output/"),
                ],
                write: vec![PathBuf::from("/data/output/")],
            },
            resources: Some(ResourceLimits {
                max_cpus: 32,
                max_memory_gb: 128,
                max_time_hours: 120,
                max_gpus: 2,
                max_array_size: 1000,
            }),
            containers: ContainerConfig::default(),
            modules: ModuleConfig::default(),
            singularity: SingularityConfig {
                default_image: Some("/containers/python.sif".to_string()),
                scripts_dir: Some(PathBuf::from("/scratch/scripts/")),
                cache_dir: Some(PathBuf::from("/scratch/.singularity")),
                extra_binds: vec!["/n/app:ro".to_string()],
            },
        }
    }

    #[test]
    fn test_generate_basic_script() {
        let config = test_config();
        let request = SandboxedSbatchRequest {
            job_name: "test-job".to_string(),
            command: "python script.py".to_string(),
            image: None, // Use default
            partition: Some(Partition::Short),
            cpus: 4,
            memory: Some(MemorySpec {
                amount: 16,
                unit: MemoryUnit::GB,
            }),
            time: Some(TimeSpec {
                days: 0,
                hours: 2,
                minutes: 0,
            }),
            gpu: None,
            array: None,
            working_dir: Some("/data/output/project".to_string()),
            output: Some("/data/output/logs/%j.out".to_string()),
            error: Some("/data/output/logs/%j.err".to_string()),
            input_paths: vec!["/data/input/dataset".to_string()],
            output_paths: vec!["/data/output/results".to_string()],
            environment: [("PYTHONPATH".to_string(), "/app".to_string())]
                .into_iter()
                .collect(),
            extra_directives: Default::default(),
            return_script: false,
        };

        let result = generate_script(&request, &config).unwrap();

        assert!(result.content.contains("#SBATCH --job-name=test-job"));
        assert!(result.content.contains("#SBATCH --partition=short"));
        assert!(result.content.contains("#SBATCH --cpus-per-task=4"));
        assert!(result.content.contains("#SBATCH --mem=16G"));
        assert!(result.content.contains("module load singularity"));
        assert!(result.content.contains("singularity exec"));
        assert!(result.content.contains("/data/input/dataset:/data/input/dataset:ro"));
        assert!(result.content.contains("/data/output/results:/data/output/results:rw"));
        assert!(result.content.contains("python script.py"));
        assert_eq!(result.image, "/containers/python.sif");
    }

    #[test]
    fn test_input_path_validation() {
        let config = test_config();
        let request = SandboxedSbatchRequest {
            job_name: "test".to_string(),
            command: "echo hello".to_string(),
            image: Some("/test.sif".to_string()),
            partition: None,
            cpus: 1,
            memory: None,
            time: None,
            gpu: None,
            array: None,
            working_dir: None,
            output: None,
            error: None,
            input_paths: vec!["/unauthorized/path".to_string()],
            output_paths: vec![],
            environment: Default::default(),
            extra_directives: Default::default(),
            return_script: false,
        };

        let result = generate_script(&request, &config);
        assert!(matches!(result, Err(ScriptError::InputPathNotAllowed(_))));
    }

    #[test]
    fn test_resource_limit_validation() {
        let config = test_config();
        let request = SandboxedSbatchRequest {
            job_name: "test".to_string(),
            command: "echo hello".to_string(),
            image: Some("/test.sif".to_string()),
            partition: None,
            cpus: 64, // Exceeds max of 32
            memory: None,
            time: None,
            gpu: None,
            array: None,
            working_dir: None,
            output: None,
            error: None,
            input_paths: vec![],
            output_paths: vec![],
            environment: Default::default(),
            extra_directives: Default::default(),
            return_script: false,
        };

        let result = generate_script(&request, &config);
        assert!(matches!(result, Err(ScriptError::ResourceLimitExceeded(_))));
    }

    #[test]
    fn test_parse_bind_spec() {
        // Short form
        let mount = parse_bind_spec("/path:ro").unwrap();
        assert_eq!(mount.host, "/path");
        assert_eq!(mount.container, "/path");
        assert_eq!(mount.mode, "ro");

        // Long form
        let mount = parse_bind_spec("/host:/container:rw").unwrap();
        assert_eq!(mount.host, "/host");
        assert_eq!(mount.container, "/container");
        assert_eq!(mount.mode, "rw");
    }
}
