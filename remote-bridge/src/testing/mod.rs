//! Test fixtures and utilities
//!
//! This module provides common test configurations and utilities
//! for testing the remote-bridge components.

use crate::config::{
    ContainerConfig, ModuleConfig, PathPermissions, PermissionConfig, ResourceLimits,
    SingularityConfig,
};
use std::path::PathBuf;

/// Create a standard test permission config
///
/// This config allows:
/// - Read: /data/input/, /data/output/
/// - Write: /data/output/, /scratch/
/// - Resources: 32 CPUs, 128GB RAM, 120 hours, 2 GPUs, 1000 array tasks
/// - Singularity: default image at /containers/python.sif
pub fn test_config() -> PermissionConfig {
    PermissionConfig {
        paths: PathPermissions {
            read: vec![
                PathBuf::from("/data/input/"),
                PathBuf::from("/data/output/"),
                PathBuf::from("/scratch/"),
            ],
            write: vec![
                PathBuf::from("/data/output/"),
                PathBuf::from("/scratch/"),
            ],
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
            logs_dir: Some(PathBuf::from("/scratch/.agent/logs/")),
            cache_dir: Some(PathBuf::from("/scratch/.singularity")),
            extra_binds: vec!["/n/app:ro".to_string()],
            module_name: String::new(),  // Default: no module load
        },
    }
}

/// Create a minimal test config (no singularity, no resource limits)
pub fn minimal_config() -> PermissionConfig {
    PermissionConfig {
        paths: PathPermissions {
            read: vec![PathBuf::from("/data/")],
            write: vec![PathBuf::from("/data/output/")],
        },
        resources: None,  // Let O2 handle resource limits
        containers: ContainerConfig::default(),
        modules: ModuleConfig::default(),
        singularity: SingularityConfig::default(),
    }
}

/// Create a config with restricted singularity (no default image)
pub fn config_no_default_image() -> PermissionConfig {
    let mut config = test_config();
    config.singularity.default_image = None;
    config
}

/// Create a config with no scripts_dir
pub fn config_no_scripts_dir() -> PermissionConfig {
    let mut config = test_config();
    config.singularity.scripts_dir = None;
    config
}

/// Common mock responses for SSH commands
pub mod responses {
    /// Successful mkdir response
    pub const MKDIR_SUCCESS: &str = "";

    /// Successful chmod response
    pub const CHMOD_SUCCESS: &str = "";

    /// Successful sbatch response
    pub fn sbatch_success(job_id: &str) -> String {
        format!("Submitted batch job {}", job_id)
    }

    /// Successful ls -la response
    pub const LS_RESPONSE: &str = "total 16
drwxr-xr-x  4 user group 4096 Jan 17 10:00 .
drwxr-xr-x  3 user group 4096 Jan 17 09:00 ..
-rw-r--r--  1 user group 1234 Jan 17 10:00 file1.txt
-rw-r--r--  1 user group 5678 Jan 17 09:30 file2.py";

    /// Successful cat response
    pub const CAT_RESPONSE: &str = "line1\nline2\nline3";

    /// Successful squeue response (empty queue)
    pub const SQUEUE_EMPTY: &str = "JOBID PARTITION NAME USER ST TIME NODES NODELIST(REASON)";

    /// Successful squeue response with jobs
    pub const SQUEUE_WITH_JOBS: &str = "JOBID PARTITION NAME USER ST TIME NODES NODELIST(REASON)
12345678 short test-job testuser R 0:05:00 1 compute-a-001
12345679 medium analysis testuser PD 0:00:00 1 (Resources)";

    /// Successful sacct response
    pub const SACCT_RESPONSE: &str = "JobID|JobName|Partition|State|ExitCode|Elapsed|MaxRSS|MaxVMSize|NCPUS|NTasks
12345678|test-job|short|COMPLETED|0:0|00:05:32|1024K|2048K|4|1";

    /// Successful git pull (up to date)
    pub const GIT_PULL_UP_TO_DATE: &str = "Already up to date.";

    /// Successful git pull (with changes)
    pub const GIT_PULL_WITH_CHANGES: &str = "Updating abc1234..def5678
Fast-forward
 file1.py | 10 ++++++++++
 file2.py |  5 ++---
 2 files changed, 12 insertions(+), 3 deletions(-)";

    /// wc output
    pub const WC_RESPONSE: &str = "      10      50     500 /path/to/file.txt";

    /// head output
    pub const HEAD_RESPONSE: &str = "line1\nline2\nline3\nline4\nline5";

    /// grep output
    pub const GREP_RESPONSE: &str = "/path/file.py:10:def main():
/path/file.py:25:    main()";

    /// find output
    pub const FIND_RESPONSE: &str = "/data/input/file1.py
/data/input/subdir/file2.py
/data/input/subdir/file3.py";
}

/// Helper to build a cat > heredoc command pattern
pub fn cat_heredoc_pattern(path: &str) -> String {
    format!("cat > '{}'", path)
}

/// Helper to build a mkdir command
pub fn mkdir_pattern(path: &str) -> String {
    format!("mkdir -p '{}'", path)
}

/// Helper to build a chmod command
pub fn chmod_pattern(path: &str) -> String {
    format!("chmod +x '{}'", path)
}

/// Helper to build an sbatch command
pub fn sbatch_pattern(path: &str) -> String {
    format!("sbatch '{}'", path)
}
