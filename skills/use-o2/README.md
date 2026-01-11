# Use O2 Cluster Plugin

A Claude Code skill for working with the O2 high-performance computing cluster at Harvard Medical School, using the SLURM workload manager.

## What It Does

When you ask Claude to run computationally intensive analyses, this skill automatically:
- Determines if O2 is appropriate for the task
- Chooses the right partition (short, medium, long, gpu, highmem, etc.)
- Estimates resource requirements (memory, cores, time)
- Creates properly formatted SLURM submission scripts
- Submits jobs and provides monitoring instructions
- Helps troubleshoot common issues

## When to Use O2

The skill activates when:
- You explicitly mention "O2", "cluster", or "SLURM"
- Analysis requires substantial resources:
  - **Memory**: >16GB RAM
  - **Runtime**: >4 hours
  - **GPUs**: Any GPU computation
  - **Parallelization**: Many cores needed

## O2 Cluster Overview

**O2** is Harvard Medical School's shared HPC cluster using **SLURM** (Simple Linux Utility for Resource Management).

**Main Partitions:**
- `short`: Jobs <12 hours (default)
- `medium`: Jobs 12 hours - 5 days
- `long`: Jobs >5 days (requires access)
- `interactive`: Interactive sessions (up to 12 hours)
- `highmem`: Memory >200GB
- `gpu`: GPU computation
- `mpi`: Multi-node MPI jobs

## Running Claude Code on O2

When you SSH to O2, you land on a login node (login01-login05). Login nodes are fine for most tasks, but resource-intensive work should use compute nodes to avoid your session being killed.

### Starting an Interactive Session (Optional)

For compute-intensive analyses, use an interactive session on a compute node:

```bash
# Start interactive session
srun -p interactive -t 0-04:00 -c 4 --mem=16G --pty /bin/bash

# Verify you're on a compute node
hostname
# Should show: compute-X-YY-ZZ (not login0X)
```

See the **Best Practices** section below for guidelines on when to use compute nodes.

## Quick Start

### Submit a Simple Job

```bash
sbatch -p short -t 0-02:00 -c 4 --mem=16G --wrap="python analysis.py"
```

### Submit with Script

```bash
# create_job.sh
#!/bin/bash
#SBATCH -p short
#SBATCH -t 0-04:00
#SBATCH -c 8
#SBATCH --mem=32G
#SBATCH -o %j.out
#SBATCH -e %j.err
#SBATCH -J my_analysis

python analysis.py
```

```bash
sbatch create_job.sh
```

### Interactive Session

```bash
srun -p interactive -t 0-04:00 -c 4 --mem=16G --pty /bin/bash
```

## Essential Commands

| Command | Purpose |
|---------|---------|
| `sbatch script.sh` | Submit batch job |
| `squeue -u $USER` | Check your jobs |
| `scancel 12345678` | Cancel job |
| `seff 12345678` | Check job efficiency |
| `O2squeue` | O2-specific queue view |

## Common Workflows

### Workflow 1: Single Analysis

```bash
# 1. Create submission script
cat > run_gwas.sh << 'EOF'
#!/bin/bash
#SBATCH -p short
#SBATCH -t 0-06:00
#SBATCH -c 8
#SBATCH --mem=64G
#SBATCH -o gwas_%j.out
#SBATCH -e gwas_%j.err

module load plink/1.90
cd /n/data1/hms/dbmi/username/project
plink --bfile genotypes --assoc --out results
EOF

# 2. Submit
sbatch run_gwas.sh

# 3. Monitor
squeue -u $USER

# 4. Check efficiency when done
seff <job_id>
```

### Workflow 2: Job Array (Parameter Sweep)

```bash
#!/bin/bash
#SBATCH -p short
#SBATCH -t 0-02:00
#SBATCH -c 1
#SBATCH --mem=8G
#SBATCH --array=1-100
#SBATCH -o logs/sim_%A_%a.out
#SBATCH -J simulations

# Run simulation with different random seed
python simulate.py --seed ${SLURM_ARRAY_TASK_ID}
```

### Workflow 3: Pipeline with Dependencies

```bash
# Submit jobs in sequence
JOB1=$(sbatch --parsable preprocess.sh)
JOB2=$(sbatch --parsable --dependency=afterok:$JOB1 analyze.sh)
JOB3=$(sbatch --parsable --dependency=afterok:$JOB2 summarize.sh)
```

## Resource Guidelines

### Memory Estimation

| Task Type | Typical Memory | Notes |
|-----------|----------------|-------|
| GWAS (small) | 8-16GB | <10K samples |
| GWAS (large) | 32-128GB | >100K samples |
| RNA-seq align | 32GB | Per sample |
| Deep learning | 32-64GB | +GPU memory |
| Large matrices | 128-512GB | Use highmem |

### Time Estimation

Start conservative (1.5-2x expected time), then optimize based on `seff` output.

### Core Usage

Only request multiple cores if your code is parallelized:
- Python: `multiprocessing`, `joblib`, `dask`
- R: `parallel`, `foreach`, `future`
- Software: Check if supports `-j` or `--threads`

## Choosing the Right Partition

```
Is it interactive? → interactive (max 12h, 20 cores)
    ↓ No
Runtime <12 hours? → short
    ↓ No
Runtime <5 days? → medium
    ↓ No
Runtime >5 days? → long (need RC approval)

Special cases:
- Memory >200GB? → highmem
- Need GPU? → gpu
- Multi-node MPI? → mpi
```

## Job Monitoring

### Check Status

```bash
# Your jobs
squeue -u $USER

# Detailed info
scontrol show job 12345678

# O2-specific view (more details)
O2squeue
```

### While Running

```bash
# Follow output in real-time
tail -f 12345678.out
tail -f 12345678.err
```

### After Completion

```bash
# Check efficiency
seff 12345678

# Detailed accounting
sacct -j 12345678 --format=JobID,JobName,Elapsed,CPUTime,MaxRSS,State
```

## Interpreting seff Output

```
Job ID: 12345678
Cores: 8
CPU Utilized: 06:30:15
CPU Efficiency: 81.25%
Job Wall-clock time: 01:00:20
Memory Utilized: 28.5 GB
Memory Efficiency: 89.06% of 32.0 GB
```

**Good efficiency:**
- CPU: >80% (using cores well)
- Memory: 60-90% (good estimate)

**Needs optimization:**
- CPU: <50% (code not parallel, or requested too many cores)
- Memory: <30% (over-requested) or >95% (may have hit limit)

## Common Issues

### Job Stays Pending

**Reason**: `(Resources)`
- Cluster busy, wait for resources
- Check partition load: `squeue -p short`

**Reason**: `(Priority)`
- Other jobs have higher priority
- Wait or use priority partition if urgent

**Reason**: Request too large
- Reduce memory/cores or use different partition
- Check node limits: `sinfo`

### Job Fails Immediately

**Check error file:**
```bash
cat 12345678.err
```

**Common causes:**
- Module not loaded → Add `module load` command
- File not found → Use absolute paths
- Permission denied → Check file permissions
- Out of memory → Increase `--mem`

### Low CPU Efficiency

**Problem**: Requested 8 cores, efficiency <50%

**Causes:**
- Code not parallelized
- I/O bottleneck (disk reads/writes)
- Unbalanced workload

**Solutions:**
- Verify code uses multiple cores
- Reduce core request to what's actually used
- Copy data to `/tmp` (local disk) for processing

### Out of Memory

**Error**: `oom-kill event`

**Solution:**
1. Check actual usage: `sacct -j 12345678 --format=MaxRSS`
2. Increase memory by 20-30%
3. Consider:
   - Process in chunks
   - Memory-efficient algorithms
   - `highmem` partition if very large

## Best Practices

### 0. **Know Your Node: Login vs Compute**

When you log in to O2, you land on a login node (login01-login05). You can use Claude Code on login nodes for most tasks, but be aware of resource usage.

**Check which node you're on:**
```bash
hostname
```
- Starts with `login`: You're on a login node
- Starts with `compute`: You're on a compute node

**Login nodes are fine for:**
- Editing scripts and files
- Submitting jobs (`sbatch`)
- Checking job status (`squeue`, `seff`)
- Navigating directories
- Small data exploration and testing
- Installing packages

**Use compute nodes (interactive session) for resource-intensive work:**
- Large data processing or analysis
- Heavy compilation
- Long-running computations
- Memory-intensive operations (>4GB)

**Starting an interactive session:**
```bash
# Request interactive session (4 hours, 4 cores, 16GB RAM)
srun -p interactive -t 0-04:00 -c 4 --mem=16G --pty /bin/bash
```

Your hostname will change from `login0X` to `compute-X-YY-ZZ` once the session starts.

**Note:** Running resource-intensive processes on login nodes may result in your session being killed if you exceed resource limits.

### 1. Test First
- Run interactively to test code
- Use small data subset
- Then scale up with batch job

### 2. Estimate Conservatively
- Time: 1.5-2x expected
- Memory: Add 20-30% buffer
- Optimize after checking `seff`

### 3. Organize Output
```bash
mkdir -p logs
#SBATCH -o logs/%j.out
#SBATCH -e logs/%j.err
```

### 4. Use Descriptive Names
```bash
#SBATCH -J gwas_chr1_ukb
```

### 5. Script Everything
Don't rely on command history - save all submission scripts.

### 6. Monitor Efficiency
Always run `seff` after jobs complete to optimize future jobs.

### 7. Enable Desktop/Phone Notifications
Get notified when jobs complete, even when away from your desk:

**Setup (one time):**
```bash
# Add to ~/.bashrc
export NTFY_TOPIC="$(whoami)_o2_notifications"

# Subscribe on your devices
# Phone: Install ntfy app, subscribe to your topic
# Desktop: Visit https://ntfy.sh/your_topic
```

**Usage:**
```bash
# Simple notification
notify "Job completed!"

# In SLURM script
#SBATCH --wrap="python analysis.py && notify 'Analysis done'"

# With timing
notifyme long_running_command
```

See the **Notifications** section below for detailed setup and usage.

## Notifications

O2 jobs can send real-time notifications to your phone or desktop when they complete. This uses [ntfy.sh](https://ntfy.sh/), a simple HTTP-based notification service.

### Setup

**1. Configure your notification topic:**

Add to your `~/.bashrc` on O2:
```bash
# Use a unique topic name (username + random string recommended)
export NTFY_TOPIC="$(whoami)_o2_notifications"
```

**2. Subscribe on your devices:**

**Phone (iOS/Android):**
- Install ntfy app from App Store or Play Store
- Tap "Subscribe to topic"
- Enter your topic name: `your_username_o2_notifications`

**Desktop:**
- Visit: `https://ntfy.sh/your_username_o2_notifications`
- Bookmark for easy access
- Or install [ntfy desktop app](https://github.com/Aetherinox/ntfy-desktop)

**macOS Terminal (optional):**
```bash
# Subscribe and show notifications
ntfy subscribe your_username_o2_notifications 'osascript -e "display notification \"$NTFY_MESSAGE\" with title \"$NTFY_TITLE\""'
```

**3. Test the setup:**
```bash
source ~/.bashrc
test_notify
```

You should receive a test notification on all subscribed devices!

### Usage Examples

**Simple notification:**
```bash
notify "Job completed successfully!"
```

**With title and priority:**
```bash
notify "Analysis finished" "GWAS Results" high
```

**Notify after command:**
```bash
python long_analysis.py && notify "Analysis done!"
```

**In SLURM job script:**
```bash
#!/bin/bash
#SBATCH -p short
#SBATCH -t 0-04:00
#SBATCH -c 8
#SBATCH --mem=32G

# Your analysis
python analysis.py

# Notify on completion (checks exit code)
notify_job_complete $?
```

**Wrap command with timing:**
```bash
# Automatically notifies with runtime
notifyme python long_script.py
```

**One-liner SLURM submission with notification:**
```bash
sbatch -p short -t 0-02:00 --mem=16G --wrap="python analysis.py && notify 'Analysis complete'"
```

### Available Functions

After running the setup script, these functions are available:

| Function | Description | Example |
|----------|-------------|---------|
| `notify` | Send simple notification | `notify "Done!"` |
| `notify_job_complete` | Notify SLURM job status | `notify_job_complete $?` |
| `notifyme` | Run command and notify with timing | `notifyme python script.py` |
| `test_notify` | Test notification setup | `test_notify` |

### Alternative: Email Notifications

SLURM has built-in email notifications:

```bash
#SBATCH --mail-type=END,FAIL
#SBATCH --mail-user=yourname@hms.harvard.edu
```

**Mail types:**
- `BEGIN`: Job started
- `END`: Job completed
- `FAIL`: Job failed
- `ALL`: All events

**Pros:** No setup needed, works everywhere
**Cons:** Email delays, no phone push notifications, less immediate

### Privacy & Security

**ntfy.sh public service:**
- Topics are public by default (anyone who knows your topic can see messages)
- Don't send sensitive data
- Use a random topic name for obscurity
- Topics auto-delete after ~12 hours of inactivity

**For sensitive data:**
- Use email notifications instead (`--mail-type`)
- Self-host ntfy ([docs](https://docs.ntfy.sh/install/))
- Use access control and authentication

### Troubleshooting

**Not receiving notifications:**
```bash
# Check topic is set
echo $NTFY_TOPIC

# Test manually
curl -d "Test message" https://ntfy.sh/$NTFY_TOPIC

# Check you're subscribed to the right topic
```

**Notifications from compute nodes:**
- Compute nodes can access internet via http/https
- No special configuration needed
- Works from interactive and batch jobs

**Too many notifications:**
- Use only for long jobs (>30 min)
- Set priority to `low` for routine jobs
- Only notify on failures: `|| notify "Failed!" "Error" high`

## Helpful Bash Aliases

Add to `~/.bashrc`:

```bash
# Quick job submission
alias qsub='sbatch -p short -t 0-04:00 -c 4 --mem=16G'

# Interactive session
alias interact='srun -p interactive -t 0-04:00 -c 4 --mem=16G --pty /bin/bash'

# Check my jobs
alias myjobs='squeue -u $USER'

# Job efficiency
alias myeff='seff $(squeue -u $USER -h -o %i | head -1)'
```

## Integration with Other Skills

### With perform-analysis

When perform-analysis detects high resource needs:

```
User: "Perform differential expression on 100K samples"

Claude: [Analyzes requirements]
- Memory needed: ~128GB
- Runtime: ~12 hours
- This exceeds local resources

[Invokes use-o2 skill]
[Creates SLURM script for highmem partition]
[Submits job with appropriate resources]
[Provides monitoring commands]
```

### With teaching-mode

```
User: "Teach me how to submit a job to O2"

Claude: [Provides step-by-step tutorial]
- Explains SLURM basics
- Shows resource estimation
- Creates example script with explanations
- Explains monitoring and efficiency checking
```

## Example: Complete Analysis Workflow

```bash
# 1. Log into O2
ssh user@o2.hms.harvard.edu

# 2. Navigate to project
cd /n/data1/hms/dbmi/username/gwas_project

# 3. Create submission script
cat > run_gwas.sh << 'EOF'
#!/bin/bash
#SBATCH -p short
#SBATCH -t 0-08:00
#SBATCH -c 16
#SBATCH --mem=128G
#SBATCH -o logs/gwas_%j.out
#SBATCH -e logs/gwas_%j.err
#SBATCH -J ukb_gwas
#SBATCH --mail-type=END
#SBATCH --mail-user=user@hms.harvard.edu

# Load modules
module load plink/1.90
module load R/4.2.1

# Run GWAS
echo "Starting GWAS at $(date)"
plink --bfile ukb_data \
      --pheno phenotypes.txt \
      --covar covariates.txt \
      --linear \
      --threads 16 \
      --out results/gwas_results

# Generate QQ plot
echo "Generating QQ plot at $(date)"
Rscript scripts/make_qq_plot.R results/gwas_results.assoc.linear

echo "Completed at $(date)"
EOF

# 4. Create logs directory
mkdir -p logs

# 5. Submit job
sbatch run_gwas.sh
# Note job ID: 12345678

# 6. Monitor
squeue -u $USER
tail -f logs/gwas_12345678.out

# 7. When complete, check efficiency
seff 12345678

# 8. Review results
ls -lh results/
cat logs/gwas_12345678.out
```

## Resources

- [O2 Wiki](https://harvardmed.atlassian.net/wiki/spaces/O2/overview)
- [SLURM Commands Cheat Sheet](https://harvardmed.atlassian.net/wiki/spaces/O2/pages/1594261585/O2+Command+CheatSheet)
- [Using SLURM Basic](https://harvardmed.atlassian.net/wiki/spaces/O2/pages/1586793632/Using+Slurm+Basic)
- [Partition Guide](https://harvardmed.atlassian.net/wiki/spaces/O2/pages/1586793641)
- [Troubleshooting](https://harvardmed.atlassian.net/wiki/spaces/O2/pages/1586793613)
- [HMS Research Computing](https://rc.hms.harvard.edu)
- [HBC Training Materials](https://hbctraining.github.io/Training-modules/Tips_and_Tricks_on_O2/)

## Installation

Already included if using claude-config repository with `pluginDirs`.

## Version

Current version: 1.0.0

## Customization

Edit `skills/use-o2/SKILL.md` to:
- Adjust default resource estimates for your typical workflows
- Add lab-specific partitions or allocations
- Include commonly used modules
- Add project-specific paths and conventions
