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

#[cfg(test)]
mod tests {
    use super::*;

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
