---
name: use-o2
description: SLURM and O2 cluster reference. This skill provides knowledge about partitions, resource estimation, job submission, and monitoring. Used by /remote-o2 when executing cluster commands.
version: 2.0.0
---

# O2 Cluster Reference

This skill provides reference material about the O2 cluster and SLURM. It is used by `/remote-o2` when executing cluster commands remotely.

**Note:** This skill is not invoked directly. Use `/remote-o2` to access O2 from your local machine.

## O2 Cluster Overview

**O2** is Harvard Medical School's high-performance computing cluster using **SLURM** (Simple Linux Utility for Resource Management) for job scheduling.

**Key features:**
- Shared resource among hundreds of users
- Job scheduler ensures fair resource distribution
- Multiple partitions for different job types
- Both interactive and batch job submission

## When to Use O2

**Use O2 when:**
- Memory needs exceed local machine (>16GB)
- Long-running jobs (>4 hours)
- Parallel processing across many cores
- GPU computation required
- Need to run many similar jobs (job arrays)
- Data is already on O2 cluster

**Don't use O2 when:**
- Quick test (<5 minutes)
- Interactive development requiring rapid iteration
- Job needs <8GB RAM and <1 hour
- Data transfer overhead exceeds computation time

## Partitions

O2 has several partitions with different purposes and limits:

| Partition | Use Case | Time Limit | Max Cores | Notes |
|-----------|----------|------------|-----------|-------|
| `priority` | Single urgent job | 5 days | 20 | **Use for single jobs** - dispatched first, max 2 concurrent |
| `short` | Jobs <12 hours | 12 hours | 20 | Default for most jobs |
| `medium` | Jobs 12h - 5 days | 5 days | 20 | Longer running jobs |
| `long` | Jobs >5 days | 30 days | 20 | Requires RC access |
| `interactive` | Interactive work | 12 hours | 20 | 1-2 concurrent, for debugging/testing |
| `highmem` | Memory >200GB | Varies | Varies | Memory-intensive jobs |
| `gpu` | GPU computation | Varies | Varies | CUDA/GPU workloads |
| `mpi` | Multi-node MPI | Varies | >20 | Distributed parallel jobs |

**Decision guide:**
```
Single important job? → priority (max 2 concurrent)
Runtime <12 hours? → short
Runtime 12h-5 days? → medium
Runtime >5 days? → long (need access)
Memory >200GB? → highmem
Need GPU? → gpu
Need >20 cores (MPI)? → mpi
Interactive debugging? → interactive
```

## Resource Estimation

### Time (`-t` or `--time`)
- Format: `D-HH:MM` (days-hours:minutes) or `HH:MM:SS`
- Examples: `0-03:00` (3 hours), `2-00:00` (2 days), `0-00:30` (30 minutes)
- **Strategy**: Request maximum time for the partition
  - If job will likely take <12h → use `short`, request `0-12:00`
  - If job will likely take 12h-5d → use `medium`, request `5-00:00`
- Job killed if exceeds time limit
- GPU jobs: estimate more carefully (GPU queue competition is higher)

### Memory (`--mem` or `--mem-per-cpu`)
- Total memory: `--mem=8G` (8 gigabytes)
- Per CPU: `--mem-per-cpu=4G` (4GB per core)
- Units: K, M, G, T
- Default if not specified: 4GB total
- **Rule of thumb**: 2-3x the size of data loaded into memory

### CPUs/Cores (`-c` or `--cpus-per-task`)
- Number of cores: `-c 4` (4 cores)
- Default: 1 core
- **Only request what you'll use** - check if code supports parallelism

### GPUs (`--gres`)
- Request GPU: `--gres=gpu:1`
- Specific GPU type: `--gres=gpu:tesla:1`
- Only on GPU partitions

## SLURM Script Templates

### Basic Job
```bash
#!/bin/bash
#SBATCH -p short                # Partition
#SBATCH -t 0-03:00              # Time (D-HH:MM)
#SBATCH -c 4                    # Number of cores
#SBATCH --mem=16G               # Total memory
#SBATCH -o %j.out               # Output file (%j = job ID)
#SBATCH -e %j.err               # Error file
#SBATCH -J my_job_name          # Job name

# Load required modules
module load gcc/9.2.0
module load python/3.9.14

# Move to working directory
cd /n/data1/hms/dbmi/username/project

# Run the analysis
python analysis.py --input data.csv --output results.txt
```

### Job Array (parallel jobs)
```bash
#!/bin/bash
#SBATCH -p short
#SBATCH -t 0-02:00
#SBATCH -c 1
#SBATCH --mem=4G
#SBATCH -o logs/%A_%a.out       # %A = array job ID, %a = task ID
#SBATCH -e logs/%A_%a.err
#SBATCH --array=1-100           # 100 parallel jobs
#SBATCH -J analysis_array

# Get input file for this array task
INPUT_FILE=$(sed -n "${SLURM_ARRAY_TASK_ID}p" file_list.txt)

# Run analysis on this file
python process_one_file.py --input ${INPUT_FILE}
```

### GPU Job
```bash
#!/bin/bash
#SBATCH -p gpu
#SBATCH -t 1-00:00
#SBATCH -c 4
#SBATCH --mem=32G
#SBATCH --gres=gpu:1
#SBATCH -o %j.out
#SBATCH -e %j.err

module load cuda/11.7
python train_model.py
```

### Pipeline with Dependencies
```bash
# Submit first job
JOB1=$(sbatch --parsable -p short -t 0-02:00 --mem=16G --wrap="python preprocess.py")

# Submit second job that depends on first
JOB2=$(sbatch --parsable --dependency=afterok:${JOB1} \
              -p short -t 0-04:00 --mem=32G --wrap="python analyze.py")

# Submit third job that depends on second
JOB3=$(sbatch --parsable --dependency=afterok:${JOB2} \
              -p short -t 0-01:00 --mem=8G --wrap="python summarize.py")

echo "Submitted pipeline: $JOB1 -> $JOB2 -> $JOB3"
```

## Job Submission

**Script file:**
```bash
sbatch my_job.sh
```

**Inline submission:**
```bash
sbatch -p short -t 0-1:00 --mem=8G --wrap="python analysis.py"
```

**Interactive session:**
```bash
srun -p interactive -t 0-04:00 -c 4 --mem=16G --pty /bin/bash
```

## Job Monitoring

### Check Status
```bash
# Your jobs
squeue -u $USER -o "%.18i %.9P %.30j %.8T %.10M %.6D %R"

# Check output
tail -20 <jobid>.out

# Recent completions
sacct -u $USER --starttime=now-1hour --format=JobID,JobName,State,ExitCode,Elapsed
```

### Job States
- `PD`: Pending (waiting for resources)
- `R`: Running
- `CG`: Completing
- `CD`: Completed
- `F`: Failed
- `CA`: Cancelled

### Job Efficiency (after completion)
```bash
seff 12345678
```

Shows CPU and memory efficiency. Target:
- CPU efficiency >80%: Using cores effectively
- Memory efficiency 60-90%: Good estimate

### Job Management
```bash
scancel 12345678         # Cancel specific job
scancel -u $USER         # Cancel all your jobs
scontrol hold 12345678   # Hold job
scontrol release 12345678 # Release held job
tail -f 12345678.out     # Follow output
```

## Common Issues

### Job Pending
- Cluster busy → wait or try different partition
- Request too large → reduce memory/cores
- Time too long for partition → use medium/long

### Job Fails Immediately
- Module not loaded → add `module load`
- File not found → use absolute paths
- Permission denied → check file permissions
- Out of memory → increase `--mem`

### Job Killed
**Time limit:**
```
CANCELLED AT ... DUE TO TIME LIMIT
```
→ Increase time or optimize code

**Memory:**
```
Detected 1 oom-kill event(s)
```
→ Increase `--mem` or process in chunks

### Low CPU Efficiency
- Code not parallelized → use parallel libraries or reduce cores
- I/O bottleneck → copy data to /tmp first

## Best Practices

1. **Start conservative**: First job with generous resources
2. **Check efficiency**: Use `seff` after completion
3. **Document**: Record actual resource usage for future reference
4. **Use job arrays**: For multiple similar jobs
5. **Script everything**: Don't rely on command history
6. **Use descriptive names**: `-J gwas_chr1` not `-J job1`

## Quick Reference

### Essential Commands
| Command | Purpose |
|---------|---------|
| `sbatch script.sh` | Submit batch job |
| `srun -p interactive --pty /bin/bash` | Interactive session |
| `squeue -u $USER` | Check your jobs |
| `scancel 12345678` | Cancel job |
| `seff 12345678` | Job efficiency |

### Essential SBATCH Directives
| Directive | Purpose | Example |
|-----------|---------|---------|
| `-p` | Partition | `-p short` |
| `-t` | Time limit | `-t 0-04:00` |
| `-c` | CPU cores | `-c 8` |
| `--mem` | Total memory | `--mem=32G` |
| `-o` | Output file | `-o %j.out` |
| `-e` | Error file | `-e %j.err` |
| `-J` | Job name | `-J analysis` |
| `--array` | Job array | `--array=1-100` |
| `--gres` | GPU | `--gres=gpu:1` |

## Git-Based Workflow for Remote Submission

When working remotely via the bridge, use this workflow to create and submit SLURM jobs:

### Setup (One-Time)

1. User clones the project repo on O2:
   ```bash
   ssh o2
   cd /n/data1/hms/dbmi/.../lab/username/
   git clone <repo-url> project-name
   ```

2. Record the O2 path in project `CLAUDE.md`:
   ```markdown
   ## O2 Paths
   - O2 repo: /n/data1/hms/dbmi/.../lab/username/project-name
   ```

### Workflow

1. **Create sbatch script locally** in the project directory
2. **Commit and push** to git
3. **Pull on O2** via bridge:
   ```json
   {"jsonrpc":"2.0","method":"git_pull","params":{"path":"/n/data1/.../project-name"},"id":1}
   ```
4. **Submit job** via bridge:
   ```json
   {"jsonrpc":"2.0","method":"sbatch","params":{"script_path":"/n/data1/.../project-name/jobs/my_job.sh"},"id":2}
   ```
5. **Monitor** via bridge:
   ```json
   {"jsonrpc":"2.0","method":"squeue","params":{"user":"username"},"id":3}
   ```

### Bridge Commands for SLURM

| Method | Purpose | Example params |
|--------|---------|----------------|
| `git_pull` | Pull latest changes | `{"path":"/n/data1/.../repo"}` |
| `sbatch` | Submit job | `{"script_path":"/path/to/job.sh"}` |
| `squeue` | Check queue | `{"user":"ljo8"}` or `{"job_ids":["123"]}` |
| `sacct` | Job accounting | `{"job_ids":["123"],"start_time":"now-1day"}` |

## Resources

- [O2 Wiki](https://harvardmed.atlassian.net/wiki/spaces/O2/overview)
- [SLURM Command Reference](https://harvardmed.atlassian.net/wiki/spaces/O2/pages/1594261585/O2+Command+CheatSheet)
- [Partition Selection Guide](https://harvardmed.atlassian.net/wiki/spaces/O2/pages/1586793641)
