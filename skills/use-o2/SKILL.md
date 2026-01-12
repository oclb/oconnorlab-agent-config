---
name: use-o2
description: This skill should be used when the user asks to "submit to O2", "run on O2", "use the cluster", "submit a SLURM job", mentions O2 or compute cluster job submission, or when an analysis requires substantial computational resources (>16GB RAM, >4 hours runtime, or GPUs).
version: 1.0.0
---

# Use O2 Cluster Skill

This skill helps you work with the O2 compute cluster at Harvard Medical School, using the SLURM workload manager for job submission and resource management.

## Quick Reference: Node-Specific Behavior

**When Claude Code starts on O2, first run `hostname` to detect node type.**

| Node Type | Hostname Pattern | Compute OK? | Git Remote OK? |
|-----------|------------------|-------------|----------------|
| Login     | `login01`, `login02`, etc. | NO (use compute node) | YES |
| Compute   | `compute-a-16-28`, etc. | YES | NO (no internet) |

**Summary of Claude's behavior on O2:**

- **On login nodes**: Can do git push/pull/fetch, but should NOT run compute-intensive operations. Suggest interactive session for heavy work.
- **On compute nodes**: Can run analyses freely, but CANNOT do git push/pull/fetch. Local git operations (commit, add, status) work fine. Prompt user to run remote git commands from a login node.

## Understanding O2 Login vs Compute Nodes

When operating on O2, be aware of whether you're on a login node or compute node.

### Detecting Node Type

**IMPORTANT: When Claude Code starts on O2, immediately run `hostname` to detect the node type.** This determines what operations are safe to perform.

```bash
hostname
```

- **Login nodes**: Hostname starts with `login` (e.g., login01, login02, login03, login04, login05)
- **Compute nodes**: Hostname starts with `compute` (e.g., compute-a-16-28, compute-e-16-155)

You can also use this quick check:
```bash
# Returns "login" or "compute"
hostname | cut -d'-' -f1
```

### Best Practice: Use Compute Nodes for Resource-Intensive Work

**Login nodes** are shared by all users for lightweight tasks like editing scripts, submitting jobs, and checking status. Running resource-intensive processes on login nodes may result in your session being killed if you use too many resources.

**For compute-intensive work**, use an interactive session on a compute node:
```bash
srun -p interactive -t 0-4:00 -c 4 --mem=16G --pty /bin/bash
```

Once you get the interactive session, your hostname will change from `login0X` to `compute-X-YY-ZZ`.

### Guidelines for Login Nodes

If you're on a login node, you can still use Claude Code for most tasks. Just be mindful of resource usage.

**IMPORTANT: On login nodes, Claude should AVOID running compute-intensive operations.** Login nodes are shared by all users and your processes may be killed if they consume too many resources.

**Lightweight operations (fine on login nodes):**
- Creating/editing SLURM submission scripts
- Checking job status (`squeue`, `seff`, `sacct`)
- Navigating directories, viewing files
- Small data explorations, quick tests
- Installing packages, setting up environments
- **Git operations including push/pull/fetch** (login nodes have internet access)

**Resource-intensive operations (DO NOT run on login nodes - use compute nodes instead):**
- Large data processing or analysis
- Long-running computations (>5 minutes)
- Heavy compilation
- Memory-intensive operations (>4GB)
- Running analyses, models, or pipelines

**If user requests compute-intensive work on a login node:**
1. Suggest starting an interactive session: `srun -p interactive -t 0-4:00 -c 4 --mem=16G --pty /bin/bash`
2. Or submit as a batch job if it doesn't need interactivity

**Recommendation:** If unsure, check your current node with `hostname`. For any substantial computational work, consider starting an interactive session to avoid potential interruptions.

### Network Restrictions on Compute Nodes

**CRITICAL: Compute nodes have no direct internet access.** This means certain operations will fail on compute nodes.

**Operations that DO NOT work on compute nodes:**
- `git push` - Cannot reach remote repositories
- `git pull` - Cannot reach remote repositories
- `git fetch` - Cannot reach remote repositories
- `git clone <remote-url>` - Cannot reach remote repositories
- `pip install` from PyPI - No internet access
- `conda install` from remote channels - No internet access
- Any operation requiring external network access

**Operations that WORK FINE on compute nodes:**
- `git commit` - Local operation
- `git add`, `git status`, `git diff`, `git log` - All local
- `git branch`, `git checkout`, `git merge` - All local
- `git stash`, `git reset` - All local
- Running Python/R scripts with local packages
- All file operations

**What to do when git remote operations are needed:**

If you need to push, pull, or sync with a remote repository while on a compute node:

1. **Prompt the user**: Tell them they need to run the git remote command themselves from a login node:
   ```
   "I'm on a compute node which doesn't have internet access. To push your changes,
   please run this command from a login node: git push origin <branch>"
   ```

2. **Alternative**: Suggest they exit back to a login node or open a new terminal on a login node for git operations.

3. **For Claude Code sessions**: If the user needs to do git remote operations frequently, suggest running Claude Code from a login node instead of a compute node.

**Workaround for package installation:**
If packages need to be installed, do this on a login node BEFORE starting the compute session, or suggest the user run the install command from a login node.

### Configuring Git with SSH on O2

GitHub can be accessed via HTTPS or SSH. **SSH is recommended** because it avoids repeated password/token prompts.

**Option 1: Clone a new repo using SSH**
```bash
git clone git@github.com:username/repo.git
```

**Option 2: Change existing repo from HTTPS to SSH**
```bash
# Check current remote
git remote -v

# If it shows https://github.com/..., change to SSH:
git remote set-url origin git@github.com:username/repo.git
```

**Setting up SSH keys for GitHub:**

Claude can help create the SSH key. The user must add it to their GitHub account.

1. **Create SSH key** (Claude can run this):
   ```bash
   # Check if key already exists
   ls -la ~/.ssh/id_ed25519.pub 2>/dev/null || ls -la ~/.ssh/id_rsa.pub 2>/dev/null

   # If no key exists, create one:
   ssh-keygen -t ed25519 -C "your_email@example.com" -f ~/.ssh/id_ed25519 -N ""
   ```

2. **Display the public key** (Claude can run this):
   ```bash
   cat ~/.ssh/id_ed25519.pub
   ```

3. **User must add key to GitHub** (provide these instructions):
   ```
   To add your SSH key to GitHub:
   1. Go to https://github.com/settings/keys
   2. Click "New SSH key"
   3. Give it a title like "O2 cluster"
   4. Paste the public key (the output from the cat command above)
   5. Click "Add SSH key"
   ```

4. **Test the connection** (Claude can run this):
   ```bash
   ssh -T git@github.com
   ```
   Success message: "Hi username! You've successfully authenticated..."

**Note:** SSH keys created on O2 are stored in your home directory, which is shared across login and compute nodes. However, SSH to GitHub still requires internet access, so git remote operations still only work from login nodes.

## When This Skill Applies

Use this skill when:
- User explicitly mentions "O2", "cluster", or "SLURM"
- Analysis requires substantial resources:
  - Memory: >16GB RAM
  - Runtime: >4 hours
  - GPUs needed
  - Parallel processing across many cores
- Submitting batch jobs
- Monitoring or managing cluster jobs
- User asks "should I use O2?" or "run this on the cluster?"

## O2 Cluster Overview

**O2** is Harvard Medical School's high-performance computing cluster using **SLURM** (Simple Linux Utility for Resource Management) for job scheduling and resource allocation.

**Key features:**
- Shared resource among hundreds of users
- Job scheduler ensures fair resource distribution
- Multiple partitions for different job types
- Both interactive and batch job submission

## Workflow for Using O2

### Step 1: Determine If O2 is Appropriate

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

### Step 2: Choose the Appropriate Partition

O2 has several partitions with different purposes and limits:

**Main partitions:**

| Partition | Use Case | Time Limit | Notes |
|-----------|----------|------------|-------|
| `short` | Jobs <12 hours | 12 hours | Default for most jobs |
| `medium` | Jobs 12 hours - 5 days | 5 days | Longer running jobs |
| `long` | Jobs >5 days | 30 days | Request access from RC |
| `interactive` | Interactive work | 12 hours | 1-2 jobs at a time, max 20 cores |
| `highmem` | Memory >200GB | Varies | For memory-intensive jobs |
| `mpi` | MPI/distributed jobs | Varies | Multi-node parallel jobs |
| `gpu` | GPU computation | Varies | CUDA/GPU workloads |
| `priority` | Urgent high-priority | Varies | 1-2 jobs, limited use |

**Decision guide:**
```
Runtime <12 hours? → short
Runtime 12h-5 days? → medium
Runtime >5 days? → long (need access)
Memory >200GB? → highmem
Need GPU? → gpu
Need MPI? → mpi
Interactive session? → interactive
```

### Step 3: Estimate Resource Requirements

**Critical parameters to specify:**

**1. Time (`-t` or `--time`)**
- Format: `D-HH:MM` (days-hours:minutes) or `HH:MM:SS`
- Examples: `0-03:00` (3 hours), `2-00:00` (2 days), `30:00` (30 minutes)
- **Be conservative**: Overestimate by ~20%
- Job killed if exceeds time limit

**2. Memory (`--mem` or `--mem-per-cpu`)**
- Total memory: `--mem=8G` (8 gigabytes)
- Per CPU: `--mem-per-cpu=4G` (4GB per core)
- Units: K (kilobyte), M (megabyte), G (gigabyte), T (terabyte)
- **Best practice**: Use `--mem-per-cpu` for parallel jobs
- Default if not specified: 4GB total

**3. CPUs/Cores (`-c` or `--cpus-per-task`)**
- Number of cores: `-c 4` (4 cores)
- Default: 1 core
- Max on interactive: 20 cores
- **Ensure your code uses multiple cores** (e.g., parallel R, multiprocessing Python)

**4. GPUs (`--gres`)**
- Request GPU: `--gres=gpu:1` (1 GPU)
- Specific GPU type: `--gres=gpu:tesla:1`
- Only on GPU partitions

**Example estimations:**

| Task | Time | Memory | Cores | Partition |
|------|------|--------|-------|-----------|
| GWAS (10K samples) | 2 hours | 16GB | 4 | short |
| RNA-seq alignment | 6 hours | 32GB | 8 | short |
| Deep learning | 24 hours | 64GB | 4 | gpu |
| Large matrix ops | 3 days | 256GB | 16 | highmem |
| Simulation (1000 reps) | 8 hours | 8GB | 20 | short |

### Step 4: Write a SLURM Submission Script

**Basic template:**

```bash
#!/bin/bash
#SBATCH -p short                # Partition
#SBATCH -t 0-03:00              # Time (D-HH:MM)
#SBATCH -c 4                    # Number of cores
#SBATCH --mem=16G               # Total memory
#SBATCH -o %j.out               # Output file (%j = job ID)
#SBATCH -e %j.err               # Error file
#SBATCH -J my_job_name          # Job name
#SBATCH --mail-type=END         # Email when done
#SBATCH --mail-user=user@hms.harvard.edu

# Load required modules
module load gcc/9.2.0
module load python/3.9.14

# Move to working directory
cd /n/data1/hms/dbmi/username/project

# Run the analysis
python analysis.py --input data.csv --output results.txt

# Optionally: Report completion
echo "Job completed at $(date)"
```

**Advanced template (job array):**

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

**GPU job template:**

```bash
#!/bin/bash
#SBATCH -p gpu
#SBATCH -t 1-00:00
#SBATCH -c 4
#SBATCH --mem=32G
#SBATCH --gres=gpu:1
#SBATCH -o %j.out
#SBATCH -e %j.err

# Load CUDA
module load cuda/11.7

# Run GPU job
python train_model.py
```

**Job with notifications:**

```bash
#!/bin/bash
#SBATCH -p short
#SBATCH -t 0-04:00
#SBATCH -c 8
#SBATCH --mem=32G
#SBATCH -o %j.out
#SBATCH -e %j.err

# Run analysis
python analysis.py

# Send notification with job status
notify_job_complete $?
```

Or use a one-liner with notification:
```bash
sbatch -p short -t 0-02:00 --mem=16G --wrap="python analysis.py && notify 'Analysis complete!'"
```

### Step 5: Submit the Job

**Method 1: Submit script file**
```bash
sbatch my_job.sh
```

**Method 2: Inline submission (simple jobs)**
```bash
sbatch -p short -t 0-1:00 --mem=8G --wrap="python analysis.py"
```

**Method 3: Interactive session**
```bash
srun -p interactive -t 0-04:00 -c 4 --mem=16G --pty /bin/bash
```

**After submission:**
- Note the job ID: `Submitted batch job 12345678`
- Jobs start when resources available (may queue)

### Step 6: Monitor Jobs

**Check job status:**
```bash
# Your jobs
squeue -u $USER

# Specific job
squeue -j 12345678

# All jobs (summary)
O2squeue  # O2-specific, shows more detail

# Detailed job info
scontrol show job 12345678
```

**Check job efficiency (after completion):**
```bash
seff 12345678
```

This shows:
- CPU efficiency (are you using all cores?)
- Memory efficiency (did you request too much?)
- Helps optimize future jobs

**Common squeue output columns:**
- `JOBID`: Job identifier
- `PARTITION`: Which partition
- `NAME`: Job name
- `USER`: Your username
- `ST`: State (PD=pending, R=running, CG=completing)
- `TIME`: Time running
- `NODES`: Number of nodes
- `NODELIST(REASON)`: Where running or why waiting

### Step 7: Manage Running Jobs

**Cancel a job:**
```bash
scancel 12345678         # Cancel specific job
scancel -u $USER         # Cancel all your jobs
scancel -n my_job_name   # Cancel by job name
```

**Hold a job (prevent from running):**
```bash
scontrol hold 12345678
```

**Release a held job:**
```bash
scontrol release 12345678
```

**View output while job runs:**
```bash
tail -f 12345678.out     # Follow output file
tail -f 12345678.err     # Follow error file
```

### Step 8: After Job Completes

**Check results:**
```bash
# View output
cat 12345678.out
cat 12345678.err

# Check efficiency
seff 12345678

# Job accounting info
sacct -j 12345678 --format=JobID,JobName,Partition,State,Elapsed,CPUTime,MaxRSS
```

**Interpret seff output:**
```
Job ID: 12345678
Cluster: o2
User/Group: username/groupname
State: COMPLETED
Cores: 4
CPU Utilized: 03:45:20
CPU Efficiency: 93.67%
Job Wall-clock time: 01:00:15
Memory Utilized: 14.2 GB
Memory Efficiency: 88.75% of 16.0 GB
```

**Good efficiency:**
- CPU efficiency >80%: Using cores effectively
- Memory efficiency 60-90%: Good estimate, not wasteful

**Poor efficiency:**
- CPU efficiency <50%: Not using all cores (code not parallel?)
- Memory efficiency <30%: Over-requested memory
- Memory efficiency >95%: Might have hit limit, could have failed

## Common Workflows

### Workflow 1: Single Analysis Job

```bash
# 1. Create submission script
cat > run_analysis.sh << 'EOF'
#!/bin/bash
#SBATCH -p short
#SBATCH -t 0-04:00
#SBATCH -c 8
#SBATCH --mem=32G
#SBATCH -o analysis_%j.out
#SBATCH -e analysis_%j.err
#SBATCH -J gwas_analysis

module load plink/1.90

cd /n/data1/hms/dbmi/username/gwas_project

plink --bfile genotypes \
      --pheno phenotypes.txt \
      --assoc \
      --out results
EOF

# 2. Submit
sbatch run_analysis.sh

# 3. Monitor
squeue -u $USER

# 4. Check results when complete
cat analysis_*.out
seff <job_id>
```

### Workflow 2: Parameter Sweep (Job Array)

```bash
# 1. Create parameter file
cat > params.txt << 'EOF'
0.01
0.05
0.1
0.5
1.0
EOF

# 2. Create submission script
cat > sweep.sh << 'EOF'
#!/bin/bash
#SBATCH -p short
#SBATCH -t 0-02:00
#SBATCH -c 1
#SBATCH --mem=8G
#SBATCH --array=1-5
#SBATCH -o logs/param_sweep_%A_%a.out
#SBATCH -e logs/param_sweep_%A_%a.err
#SBATCH -J param_sweep

# Get parameter for this task
PARAM=$(sed -n "${SLURM_ARRAY_TASK_ID}p" params.txt)

# Run with this parameter
python run_simulation.py --param $PARAM --seed ${SLURM_ARRAY_TASK_ID}
EOF

# 3. Create logs directory
mkdir -p logs

# 4. Submit array
sbatch sweep.sh

# 5. Monitor all tasks
squeue -u $USER

# 6. Check individual task results
cat logs/param_sweep_*_1.out  # First task
cat logs/param_sweep_*_*.out  # All tasks
```

### Workflow 3: Interactive Session

```bash
# 1. Request interactive session
srun -p interactive -t 0-04:00 -c 4 --mem=16G --pty /bin/bash

# Now on compute node - can run commands interactively
hostname  # Shows compute node name

# 2. Load modules and run
module load R/4.2.1
R

# 3. When done, exit
exit
```

### Workflow 4: Pipeline with Dependencies

```bash
# 1. Submit first job
JOB1=$(sbatch --parsable -p short -t 0-02:00 --mem=16G --wrap="python preprocess.py")

# 2. Submit second job that depends on first
JOB2=$(sbatch --parsable --dependency=afterok:${JOB1} \
              -p short -t 0-04:00 --mem=32G --wrap="python analyze.py")

# 3. Submit third job that depends on second
JOB3=$(sbatch --parsable --dependency=afterok:${JOB2} \
              -p short -t 0-01:00 --mem=8G --wrap="python summarize.py")

echo "Submitted pipeline: $JOB1 -> $JOB2 -> $JOB3"

# 4. Monitor pipeline
squeue -u $USER
```

## Common Issues and Solutions

### Issue 1: Job Pending (Not Starting)

**Symptoms:**
```
JOBID    PARTITION  NAME  USER  ST  TIME  NODES  NODELIST(REASON)
12345678 short      job   user  PD  0:00  1      (Resources)
```

**Possible reasons and solutions:**

**Resources not available:**
- Cluster is busy, wait for resources to free up
- Check: `squeue -p short` to see partition load
- Solution: Wait, or try different partition

**Priority too low:**
- Other users' jobs have higher priority
- Solution: Wait, or use priority partition if urgent

**Resource request too large:**
- Requesting more than available on any node
- Check: `sinfo` to see node configurations
- Solution: Reduce memory/core request

**Time limit too long for partition:**
- Requesting 24 hours on `short` partition (max 12h)
- Solution: Use `medium` partition or reduce time

### Issue 2: Job Fails Immediately

**Check error file:**
```bash
cat 12345678.err
```

**Common causes:**

**Module not loaded:**
```
python: command not found
```
Solution: Add `module load python/3.9.14`

**File not found:**
```
FileNotFoundError: [Errno 2] No such file or directory: 'data.csv'
```
Solution: Use absolute paths or `cd` to correct directory

**Permission denied:**
```
Permission denied: /n/data1/hms/dbmi/otheruser/
```
Solution: Check file permissions, use your own directories

**Out of memory:**
```
slurmstepd: error: Detected 1 oom-kill event(s)
```
Solution: Increase `--mem` request

### Issue 3: Job Runs But Produces No Output

**Check job is actually running:**
```bash
squeue -j 12345678
```

**Check output files exist:**
```bash
ls -lh 12345678.out 12345678.err
```

**Possible issues:**

**Still queued:**
- ST = PD (pending), not running yet
- Solution: Wait

**Redirected output elsewhere:**
- Check script for custom output paths
- Solution: Look in specified output directory

**Buffered output:**
- Output buffered, not written yet
- Solution: Add `flush()` calls in code, or wait longer

### Issue 4: Low CPU Efficiency

**seff shows CPU efficiency <50%:**

**Causes:**

**Code not parallelized:**
- Requested 8 cores but code is single-threaded
- Solution: Use parallel libraries, reduce core request

**I/O bottleneck:**
- CPUs waiting for disk reads/writes
- Solution: Copy data to /tmp (local disk), process there

**Unbalanced workload:**
- Some cores finish early, wait for others
- Solution: Use dynamic scheduling, better load balancing

### Issue 5: Job Killed for Exceeding Memory

**Error message:**
```
slurmstepd: error: Detected 1 oom-kill event(s) in step 12345678.batch
```

**Solution:**
1. Check actual memory used: `sacct -j 12345678 --format=JobID,MaxRSS`
2. Increase memory request by 20-30%
3. If memory very high, consider:
   - Processing data in chunks
   - Using memory-efficient algorithms
   - Using `highmem` partition

### Issue 6: Job Killed for Exceeding Time

**Error in output:**
```
slurmstepd: error: *** JOB 12345678 ON compute-a-16-163 CANCELLED AT 2024-01-10T15:23:45 DUE TO TIME LIMIT ***
```

**Solution:**
1. Check actual time used: `sacct -j 12345678 --format=JobID,Elapsed`
2. Increase time limit by 20-30%
3. Consider optimization:
   - Profile code for bottlenecks
   - Use more cores (if parallelizable)
   - Use faster algorithms

### Issue 7: Network/Git Operations Fail on Compute Node

**Symptoms:**
```
fatal: unable to access 'https://github.com/...': Could not resolve host: github.com
```
or
```
ssh: Could not resolve hostname github.com: Name or service not known
```

**Cause:**
Compute nodes do not have internet access. This is by design for security reasons.

**Solution:**
1. **Check your node type**: `hostname` - if it starts with `compute`, you're on a compute node
2. **For git push/pull/fetch**: Run these commands from a login node instead
3. **For package installation**: Install packages on a login node before starting your compute session
4. **If running Claude Code**: Consider running Claude Code from a login node if you need frequent git remote operations

**Workaround for Claude Code users:**
- Claude Code can still make local git commits on compute nodes
- For pushing changes, exit to a login node or open a separate terminal on a login node
- Run `git push` manually from the login node

## Best Practices

### Resource Estimation

1. **Start conservative**: First job with generous resources
2. **Check efficiency**: Use `seff` after job completes
3. **Optimize**: Adjust future jobs based on actual usage
4. **Document**: Keep notes on resource needs for different analyses

### Job Organization

1. **Use descriptive job names**: `-J gwas_chr1` not `-J job1`
2. **Organize output**: Create `logs/` directory for output files
3. **Use job arrays**: For multiple similar jobs (parameter sweeps)
4. **Script everything**: Don't rely on command history

### Code Optimization

1. **Parallelize when possible**: Use all requested cores
2. **Use appropriate data structures**: Memory-efficient formats
3. **Profile before scaling up**: Test on small data first
4. **Clean up**: Remove intermediate files

### Partition Selection

1. **short for most jobs**: Default choice
2. **medium for overnight**: Jobs 12-120 hours
3. **interactive for development**: Testing, debugging
4. **highmem only when needed**: >200GB memory
5. **gpu for GPU work**: Don't use for CPU-only jobs

### Monitoring and Debugging

1. **Check jobs regularly**: `squeue -u $USER`
2. **Use notifications**: Desktop/phone notifications via `notify` or email via `--mail-type=END,FAIL`
3. **Keep output files**: Don't delete immediately
4. **Test interactively first**: Before batch submission

### Notifications

Get real-time notifications when jobs complete using ntfy.sh:

**Quick setup:**
```bash
# Add to ~/.bashrc on O2
export NTFY_TOPIC="$(whoami)_o2_notifications"

# Subscribe on phone (install ntfy app) or desktop (visit https://ntfy.sh/your_topic)
```

**Usage:**
```bash
# Simple notification
notify "Job done!"

# In SLURM script
#SBATCH --wrap="python analysis.py && notify 'Analysis complete'"

# With job status tracking
notify_job_complete $?

# Test setup
test_notify
```

**Alternative - Email notifications:**
```bash
#SBATCH --mail-type=END,FAIL
#SBATCH --mail-user=yourname@hms.harvard.edu
```

See the use-o2 README for detailed notification setup and privacy considerations.

## Helper Functions

Add these to your `~/.bashrc` for convenience:

```bash
# Quick job submission
qsub() {
    sbatch -p short -t 0-04:00 -c 4 --mem=16G \
           -o logs/%j.out -e logs/%j.err \
           --wrap="$@"
}

# Interactive session
interact() {
    srun -p interactive -t 0-04:00 -c ${1:-4} --mem=${2:-16G} --pty /bin/bash
}

# Check my jobs
myjobs() {
    squeue -u $USER -o "%.18i %.9P %.30j %.8T %.10M %.6D %R"
}

# Job efficiency summary
jobeff() {
    for job in $(squeue -u $USER -h -o %i); do
        seff $job
    done
}

# Cancel all my jobs
killall() {
    scancel -u $USER
}
```

Usage:
```bash
qsub "python analysis.py"           # Quick submit with defaults
interact 8 32G                      # Interactive with 8 cores, 32GB
myjobs                              # Check status
jobeff                              # Efficiency of running jobs
```

## Integration with perform-analysis Skill

When perform-analysis determines an analysis should run on O2:

```
[In perform-analysis Step 3: Verify Resources]

This analysis will require:
- Memory: ~64GB (dataset size + processing overhead)
- Runtime: ~8 hours (based on similar analyses)
- Cores: 16 (for parallel processing)

This exceeds local resources. Submitting to O2 cluster...

[Invokes use-o2 skill]

[Creates SLURM script with appropriate resources]
[Submits job]
[Provides monitoring instructions]
```

## Quick Reference

### Essential Commands

| Command | Purpose |
|---------|---------|
| `sbatch script.sh` | Submit batch job |
| `srun -p interactive --pty /bin/bash` | Interactive session |
| `squeue -u $USER` | Check your jobs |
| `scancel 12345678` | Cancel job |
| `seff 12345678` | Job efficiency |
| `scontrol show job 12345678` | Detailed job info |

### Essential SBATCH Directives

| Directive | Purpose | Example |
|-----------|---------|---------|
| `-p` | Partition | `-p short` |
| `-t` | Time limit | `-t 0-04:00` |
| `-c` | CPU cores | `-c 8` |
| `--mem` | Total memory | `--mem=32G` |
| `--mem-per-cpu` | Memory per core | `--mem-per-cpu=4G` |
| `-o` | Output file | `-o %j.out` |
| `-e` | Error file | `-e %j.err` |
| `-J` | Job name | `-J analysis` |
| `--array` | Job array | `--array=1-100` |
| `--gres` | GPU | `--gres=gpu:1` |

### Partition Time Limits

| Partition | Max Time | Use Case |
|-----------|----------|----------|
| short | 12 hours | Default |
| medium | 5 days | Longer jobs |
| long | 30 days | Very long (need access) |
| interactive | 12 hours | Interactive work |

## Resources

- [O2 Wiki](https://harvardmed.atlassian.net/wiki/spaces/O2/overview)
- [SLURM Command Reference](https://harvardmed.atlassian.net/wiki/spaces/O2/pages/1594261585/O2+Command+CheatSheet)
- [Using SLURM Basic](https://harvardmed.atlassian.net/wiki/spaces/O2/pages/1586793632/Using+Slurm+Basic)
- [Partition Selection Guide](https://harvardmed.atlassian.net/wiki/spaces/O2/pages/1586793641)
- [Job Troubleshooting](https://harvardmed.atlassian.net/wiki/spaces/O2/pages/1586793613)
- [Research Computing Support](https://rc.hms.harvard.edu)
