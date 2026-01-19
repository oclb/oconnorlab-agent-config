#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// Job state filter for squeue/sacct
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobState {
    Pending,
    Running,
    Suspended,
    Completed,
    Cancelled,
    Failed,
    Timeout,
    All,
}

impl JobState {
    pub fn to_slurm_filter(&self) -> &'static str {
        match self {
            JobState::Pending => "PD",
            JobState::Running => "R",
            JobState::Suspended => "S",
            JobState::Completed => "CD",
            JobState::Cancelled => "CA",
            JobState::Failed => "F",
            JobState::Timeout => "TO",
            JobState::All => "all",
        }
    }
}

/// Request for squeue command
#[derive(Debug, Serialize, Deserialize)]
pub struct SqueueRequest {
    /// Filter by user (default: current user)
    pub user: Option<String>,
    /// Filter by specific job IDs
    #[serde(default)]
    pub job_ids: Vec<String>,
    /// Filter by partition
    pub partition: Option<String>,
    /// Filter by state
    pub state: Option<JobState>,
}

/// Response from squeue command
#[derive(Debug, Serialize, Deserialize)]
pub struct SqueueResponse {
    pub jobs: Vec<QueuedJob>,
    pub duration_ms: u64,
}

/// A job in the queue
#[derive(Debug, Serialize, Deserialize)]
pub struct QueuedJob {
    pub job_id: String,
    pub name: String,
    pub user: String,
    pub partition: String,
    pub state: String,
    pub time: String,
    pub nodes: String,
    pub reason: Option<String>,
}

/// Request for sacct command
#[derive(Debug, Serialize, Deserialize)]
pub struct SacctRequest {
    /// Filter by specific job IDs
    #[serde(default)]
    pub job_ids: Vec<String>,
    /// Filter by user
    pub user: Option<String>,
    /// Start time (e.g., "now-1day", "2024-01-01")
    pub start_time: Option<String>,
    /// End time
    pub end_time: Option<String>,
    /// Filter by state
    pub state: Option<JobState>,
}

/// Response from sacct command
#[derive(Debug, Serialize, Deserialize)]
pub struct SacctResponse {
    pub jobs: Vec<JobAccounting>,
    pub duration_ms: u64,
}

/// Job accounting information
#[derive(Debug, Serialize, Deserialize)]
pub struct JobAccounting {
    pub job_id: String,
    pub job_name: String,
    pub partition: String,
    pub state: String,
    pub exit_code: String,
    pub elapsed: String,
    pub max_rss: Option<String>,
    pub max_vmem: Option<String>,
    pub n_cpus: u32,
    pub n_tasks: u32,
}

/// Partition selection - typed enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Partition {
    Priority,
    Short,
    Medium,
    Long,
    Interactive,
    Highmem,
    Gpu,
    Mpi,
}

impl Partition {
    pub fn as_str(&self) -> &'static str {
        match self {
            Partition::Priority => "priority",
            Partition::Short => "short",
            Partition::Medium => "medium",
            Partition::Long => "long",
            Partition::Interactive => "interactive",
            Partition::Highmem => "highmem",
            Partition::Gpu => "gpu",
            Partition::Mpi => "mpi",
        }
    }

    pub fn max_time_hours(&self) -> u32 {
        match self {
            Partition::Priority => 120,    // 5 days
            Partition::Short => 12,
            Partition::Medium => 120,      // 5 days
            Partition::Long => 720,        // 30 days
            Partition::Interactive => 12,
            Partition::Highmem => 120,
            Partition::Gpu => 120,
            Partition::Mpi => 120,
        }
    }
}

/// Time specification for SLURM jobs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSpec {
    pub days: u32,
    pub hours: u32,
    pub minutes: u32,
}

impl TimeSpec {
    pub fn to_slurm_format(&self) -> String {
        format!("{}-{:02}:{:02}", self.days, self.hours, self.minutes)
    }

    pub fn total_hours(&self) -> u32 {
        self.days * 24 + self.hours
    }
}

/// Memory specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySpec {
    pub amount: u32,
    pub unit: MemoryUnit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MemoryUnit {
    MB,
    GB,
    TB,
}

impl MemorySpec {
    pub fn to_slurm_format(&self) -> String {
        let suffix = match self.unit {
            MemoryUnit::MB => "M",
            MemoryUnit::GB => "G",
            MemoryUnit::TB => "T",
        };
        format!("{}{}", self.amount, suffix)
    }

    pub fn to_gb(&self) -> u32 {
        match self.unit {
            MemoryUnit::MB => self.amount / 1024,
            MemoryUnit::GB => self.amount,
            MemoryUnit::TB => self.amount * 1024,
        }
    }
}

/// GPU specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuSpec {
    pub count: u32,
    pub gpu_type: Option<GpuType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GpuType {
    Tesla,
    V100,
    A100,
}

impl GpuSpec {
    pub fn to_slurm_gres(&self) -> String {
        match &self.gpu_type {
            Some(t) => {
                let type_str = match t {
                    GpuType::Tesla => "tesla",
                    GpuType::V100 => "v100",
                    GpuType::A100 => "a100",
                };
                format!("gpu:{}:{}", type_str, self.count)
            }
            None => format!("gpu:{}", self.count),
        }
    }
}

/// Parse squeue output
pub fn parse_squeue_output(output: &str) -> Vec<QueuedJob> {
    let mut jobs = Vec::new();

    for line in output.lines().skip(1) {
        // Skip header
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 8 {
            jobs.push(QueuedJob {
                job_id: parts[0].to_string(),
                partition: parts[1].to_string(),
                name: parts[2].to_string(),
                user: parts[3].to_string(),
                state: parts[4].to_string(),
                time: parts[5].to_string(),
                nodes: parts[6].to_string(),
                reason: if parts.len() > 7 {
                    Some(parts[7..].join(" "))
                } else {
                    None
                },
            });
        }
    }

    jobs
}

/// Request for sbatch command
#[derive(Debug, Serialize, Deserialize)]
pub struct SbatchRequest {
    /// Path to the sbatch script file (must exist on remote)
    pub script_path: String,
    /// Working directory for the job
    pub working_dir: Option<String>,
}

/// Response from sbatch command
#[derive(Debug, Serialize, Deserialize)]
pub struct SbatchResponse {
    pub job_id: String,
    pub script_path: String,
    pub duration_ms: u64,
}

/// How to wait for job arrays
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArrayWaitMode {
    /// Wait for any array task to complete (default)
    Any,
    /// Wait for all array tasks to complete
    All,
    /// Wait for a specific array index
    Index(u32),
}

impl Default for ArrayWaitMode {
    fn default() -> Self {
        ArrayWaitMode::Any
    }
}

/// Request for job_wait command
#[derive(Debug, Serialize, Deserialize)]
pub struct JobWaitRequest {
    /// Job ID to wait for (e.g., "12345678" or "12345678_5" for specific array task)
    pub job_id: String,
    /// For job arrays: wait for any (default), all, or specific index
    #[serde(default)]
    pub array_mode: ArrayWaitMode,
    /// Maximum time to wait in seconds (default: 86400 = 24 hours)
    #[serde(default = "default_max_wait")]
    pub max_wait_secs: u64,
}

fn default_max_wait() -> u64 {
    86400 // 24 hours
}

/// Response from job_wait command
#[derive(Debug, Serialize, Deserialize)]
pub struct JobWaitResponse {
    pub job_id: String,
    /// Jobs that completed (for arrays, may be multiple)
    pub completed_jobs: Vec<CompletedJob>,
    /// Whether all requested jobs completed (vs timeout)
    pub all_completed: bool,
    /// Total time spent waiting
    pub wait_time_secs: u64,
}

/// Information about a completed job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedJob {
    pub job_id: String,
    pub state: String,
    pub exit_code: String,
    pub elapsed: String,
}

/// Request for scancel command
#[derive(Debug, Serialize, Deserialize)]
pub struct ScancelRequest {
    /// Job IDs to cancel
    pub job_ids: Vec<String>,
}

/// Response from scancel command
#[derive(Debug, Serialize, Deserialize)]
pub struct ScancelResponse {
    pub cancelled_jobs: Vec<String>,
    pub output: String,
    pub duration_ms: u64,
}

/// Check if a job state indicates completion
pub fn is_terminal_state(state: &str) -> bool {
    matches!(
        state.to_uppercase().as_str(),
        "COMPLETED" | "FAILED" | "CANCELLED" | "TIMEOUT" | "OUT_OF_MEMORY"
        | "NODE_FAIL" | "PREEMPTED" | "BOOT_FAIL" | "DEADLINE"
    )
}

/// Parse sbatch output to extract job ID
/// Output format: "Submitted batch job 12345678"
pub fn parse_sbatch_output(output: &str) -> Option<String> {
    output
        .lines()
        .find(|line| line.starts_with("Submitted batch job"))
        .and_then(|line| line.split_whitespace().last())
        .map(|s| s.to_string())
}

/// Parse sacct output into structured data
pub fn parse_sacct_output(output: &str) -> Vec<JobAccounting> {
    let mut jobs = Vec::new();

    for line in output.lines().skip(2) {
        // Skip header lines
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() >= 10 {
            jobs.push(JobAccounting {
                job_id: parts[0].trim().to_string(),
                job_name: parts[1].trim().to_string(),
                partition: parts[2].trim().to_string(),
                state: parts[3].trim().to_string(),
                exit_code: parts[4].trim().to_string(),
                elapsed: parts[5].trim().to_string(),
                max_rss: if parts[6].trim().is_empty() {
                    None
                } else {
                    Some(parts[6].trim().to_string())
                },
                max_vmem: if parts[7].trim().is_empty() {
                    None
                } else {
                    Some(parts[7].trim().to_string())
                },
                n_cpus: parts[8].trim().parse().unwrap_or(0),
                n_tasks: parts[9].trim().parse().unwrap_or(0),
            });
        }
    }

    jobs
}

// ============================================================================
// Sandboxed sbatch types (Singularity-based execution)
// ============================================================================

/// Request for sandboxed sbatch command
///
/// This generates and submits an sbatch script that runs the command inside
/// a Singularity container with restricted bind mounts for security.
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxedSbatchRequest {
    /// Job name (--job-name)
    pub job_name: String,

    /// The command to run inside the container
    /// Can be a script path or inline command
    pub command: String,

    /// Container image to use (path to .sif or docker:// URI)
    /// If not specified, uses the default from singularity config
    pub image: Option<String>,

    /// Partition (--partition)
    #[serde(default)]
    pub partition: Option<Partition>,

    /// Number of CPUs (--cpus-per-task)
    #[serde(default = "default_cpus")]
    pub cpus: u32,

    /// Memory specification (--mem)
    #[serde(default)]
    pub memory: Option<MemorySpec>,

    /// Time limit (--time)
    #[serde(default)]
    pub time: Option<TimeSpec>,

    /// GPU specification (--gres)
    #[serde(default)]
    pub gpu: Option<GpuSpec>,

    /// Array job specification (--array), e.g., "1-100" or "1-100%10"
    #[serde(default)]
    pub array: Option<String>,

    /// Working directory inside the container (--chdir)
    /// Must be within an allowed write path
    #[serde(default)]
    pub working_dir: Option<String>,

    /// Output file path (--output)
    /// Must be within an allowed write path
    #[serde(default)]
    pub output: Option<String>,

    /// Error file path (--error)
    /// Must be within an allowed write path
    #[serde(default)]
    pub error: Option<String>,

    /// Input paths that will be bind-mounted as read-only
    /// These are validated against paths.read in the config
    #[serde(default)]
    pub input_paths: Vec<String>,

    /// Output paths that will be bind-mounted as read-write
    /// These are validated against paths.write in the config
    #[serde(default)]
    pub output_paths: Vec<String>,

    /// Environment variables to pass to the job
    #[serde(default)]
    pub environment: std::collections::HashMap<String, String>,

    /// Additional SLURM directives (key-value pairs)
    /// e.g., {"mail-type": "END", "mail-user": "user@example.com"}
    #[serde(default)]
    pub extra_directives: std::collections::HashMap<String, String>,

    /// If true, also return the generated script content (for debugging)
    #[serde(default)]
    pub return_script: bool,
}

fn default_cpus() -> u32 {
    1
}

/// Response from sandboxed sbatch command
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxedSbatchResponse {
    /// SLURM job ID
    pub job_id: String,

    /// Path to the generated sbatch script on remote
    pub script_path: String,

    /// The generated script content (only if return_script was true)
    pub script_content: Option<String>,

    /// Container image that was used
    pub image_used: String,

    /// Bind mounts that were configured
    pub bind_mounts: Vec<BindMount>,

    /// Path where job stdout will be written (with job_id substituted for %j)
    pub stdout_path: String,

    /// Path where job stderr will be written (with job_id substituted for %j)
    pub stderr_path: String,

    /// Execution time in milliseconds
    pub duration_ms: u64,
}

/// A bind mount specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindMount {
    /// Host path
    pub host: String,
    /// Container path (usually same as host)
    pub container: String,
    /// Mount mode: "ro" (read-only) or "rw" (read-write)
    pub mode: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sbatch_output() {
        let output = "Submitted batch job 12345678\n";
        assert_eq!(parse_sbatch_output(output), Some("12345678".to_string()));
    }

    #[test]
    fn test_time_spec() {
        let time = TimeSpec {
            days: 1,
            hours: 12,
            minutes: 30,
        };
        assert_eq!(time.to_slurm_format(), "1-12:30");
        assert_eq!(time.total_hours(), 36);
    }

    #[test]
    fn test_memory_spec() {
        let mem = MemorySpec {
            amount: 32,
            unit: MemoryUnit::GB,
        };
        assert_eq!(mem.to_slurm_format(), "32G");
        assert_eq!(mem.to_gb(), 32);
    }

    #[test]
    fn test_gpu_spec() {
        let gpu = GpuSpec {
            count: 2,
            gpu_type: Some(GpuType::A100),
        };
        assert_eq!(gpu.to_slurm_gres(), "gpu:a100:2");

        let gpu_any = GpuSpec {
            count: 1,
            gpu_type: None,
        };
        assert_eq!(gpu_any.to_slurm_gres(), "gpu:1");
    }
}
