---
name: remote-o2
description: This skill should be used when the user asks to "submit to O2", "run on O2", "use the cluster", "submit a SLURM job", mentions O2 or compute cluster job submission, or when an analysis requires substantial computational resources (>16GB RAM, >4 hours runtime, or GPUs).
user_invocable: true
version: 2.4.0
---

# Remote O2 Access Skill

## Role

Remote O2 cluster access specialist, managing secure job submission and monitoring through the remote-bridge application.

## Goal

Enable local-machine users to submit, monitor, and manage SLURM jobs on O2 without direct SSH sessions, while maintaining security through containerized execution and path validation.

## Key Principles

1. **Job submission uses sandboxed_sbatch** - Jobs run in Singularity containers with restricted filesystem access
2. **Single Duo authentication** - The bridge maintains a persistent SSH session; authenticate once, run unlimited commands
3. **Permission-based access** - All paths validated against user config before execution

## When This Skill Applies

- User mentions "O2", "cluster", or "SLURM"
- Analysis requires substantial resources (>16GB RAM, >4 hours, GPUs)
- User explicitly invokes `/remote-o2`

## SLURM Reference

For SLURM knowledge (partitions, resource estimation, job scripts, monitoring), refer to the **use-o2** skill.

## Step 1: Check Bridge Status

```bash
remote-bridge rpc o2 connection_status
```

**If response shows `"connected": true`:** Go to [Using the Bridge](#using-the-bridge)
**If error "Bridge 'o2' is not running":** Go to [Starting the Bridge](#starting-the-bridge)

## Starting the Bridge

### Phase 1: Binary Installation

Check if the bridge binary exists:

```bash
which remote-bridge
```

**If found:** Skip to [Phase 2: Permission Configuration](#phase-2-permission-configuration)

**If not found:** Build the bridge:

1. Check for Rust/Cargo:
   ```bash
   which cargo
   ```

2. If cargo doesn't exist, ask the user to install Rust:
   ```
   Rust is required to build the remote-bridge. Please install it:

       curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   Follow the prompts (default installation is fine), then restart your terminal
   or run: source ~/.cargo/env

   Let me know when Rust is installed.
   ```

3. Build the bridge:
   ```bash
   cd $CONFIG_REPO/remote-bridge && cargo build --release
   ```

4. Add to PATH (detect shell with `echo $SHELL`):

   **For zsh (~/.zshrc):**
   ```bash
   echo 'export PATH="$PATH:/path/to/claude-config/remote-bridge/target/release"' >> ~/.zshrc
   source ~/.zshrc
   ```

   **For bash (~/.bashrc):**
   ```bash
   echo 'export PATH="$PATH:/path/to/claude-config/remote-bridge/target/release"' >> ~/.bashrc
   source ~/.bashrc
   ```

   Replace `/path/to/claude-config` with the actual CONFIG_REPO path.

### Phase 2: Permission Configuration

1. Create the config directory:
   ```bash
   mkdir -p ~/.config/remote-bridge
   cp $CONFIG_REPO/remote-bridge/config/permissions.example.toml ~/.config/remote-bridge/permissions.toml
   ```

2. Ask the user for their O2 paths:
   - Lab directory (e.g., `/n/data1/hms/dbmi/oconnor/lab/username/`)
   - Scratch directory (e.g., `/n/scratch/users/u/username/`)

3. Edit the config:
   ```bash
   $EDITOR ~/.config/remote-bridge/permissions.toml
   ```
   Update the `[paths]` section with their actual directories.

### Phase 3: First Connection

Tell the user to run in a separate terminal:

```bash
remote-bridge start o2 --user YOUR_USERNAME
```

The user will see:
1. Password prompt (if not using SSH keys)
2. Duo authentication prompt
3. Confirmation that bridge is ready

The bridge runs in the foreground. The user should keep that terminal open.

Once the user confirms the bridge is running, proceed to [Using the Bridge](#using-the-bridge).

## Using the Bridge

All commands use `remote-bridge rpc o2 <method> [params_json]`.

### Check Connection Status

```bash
remote-bridge rpc o2 connection_status
```

### List Directory

```bash
remote-bridge rpc o2 ls '{"path":"/n/data1/...","flags":["Long","Human"]}'
```

Available flags: `Long`, `All`, `Human`, `Recursive`, `SortByTime`, `SortBySize`

### Read File

```bash
# Full file
remote-bridge rpc o2 cat '{"path":"/path/to/file.txt"}'

# First 100 lines
remote-bridge rpc o2 cat '{"path":"/path/to/file.txt","head":100}'

# Last 50 lines
remote-bridge rpc o2 cat '{"path":"/path/to/file.txt","tail":50}'

# Lines 100-200
remote-bridge rpc o2 cat '{"path":"/path/to/file.txt","offset":100,"limit":100}'
```

### Search Files (Grep)

```bash
remote-bridge rpc o2 grep '{"pattern":"def main","paths":["/path/to/search/"],"flags":["Recursive","LineNumbers"]}'
```

Available flags: `IgnoreCase`, `Recursive`, `LineNumbers`, `InvertMatch`, `WordMatch`, `CountOnly`, `FilesWithMatches`

### Head (First N Lines)

```bash
remote-bridge rpc o2 head '{"path":"/path/to/file.txt","lines":20}'
```

Default: 10 lines. Simpler than cat with head option.

### Word Count (wc)

```bash
# Get all counts (lines, words, bytes)
remote-bridge rpc o2 wc '{"path":"/path/to/file.txt"}'

# Lines only
remote-bridge rpc o2 wc '{"path":"/path/to/file.txt","lines_only":true}'

# Bytes only
remote-bridge rpc o2 wc '{"path":"/path/to/file.txt","bytes_only":true}'
```

### Find Files

```bash
# Find Python files
remote-bridge rpc o2 find '{"path":"/n/data1/.../project","name":"*.py"}'

# Find with depth limit
remote-bridge rpc o2 find '{"path":"/n/data1/...","name":"*.txt","max_depth":2}'

# Find directories only
remote-bridge rpc o2 find '{"path":"/n/data1/...","file_type":"directory"}'

# Limit results
remote-bridge rpc o2 find '{"path":"/n/data1/...","name":"*test*","limit":50}'
```

File types: `file`, `directory`, `symlink`

### Download Small Files

Download files up to 1MB. Content is base64-encoded.

```bash
remote-bridge rpc o2 download '{"path":"/n/data1/.../results.txt"}'
```

**For files >1MB:** The bridge will reject the request and provide an scp command to transfer via the transfer node. Tell the user:

```
This file is too large for download via the bridge (max 1MB).
Please transfer it manually using the transfer node:

scp USERNAME@transfer.rc.hms.harvard.edu:/n/data1/.../large_file.txt ~/Downloads/
```

**Why the limit?** Login nodes are not for large file transfers. The transfer node (`transfer.rc.hms.harvard.edu`) is designed for this purpose and won't affect other users.

### Git Pull

```bash
remote-bridge rpc o2 git_pull '{"path":"/n/data1/.../project"}'
```

Optional params: `remote` (default: "origin"), `branch` (default: current)

### Submit Job (sbatch)

For pre-existing scripts that must run exactly as written:

```bash
remote-bridge rpc o2 sbatch '{"script_path":"/n/data1/.../job.sh"}'
```

Optional: `working_dir` - directory to run sbatch from

Response includes `job_id` of submitted job.

### Submit Sandboxed Job (sandboxed_sbatch)

Generates an sbatch script that runs inside a Singularity container with restricted filesystem access.

```bash
remote-bridge rpc o2 sandboxed_sbatch '{
  "job_name": "my-analysis",
  "command": "python /n/data1/.../script.py",
  "input_paths": ["/n/data1/.../input_data/"],
  "output_paths": ["/n/scratch/users/u/username/results/"],
  "cpus": 4,
  "memory": {"amount": 16, "unit": "gb"},
  "time": {"days": 0, "hours": 2, "minutes": 0},
  "partition": "short"
}'
```

**GPU job example:**
```bash
remote-bridge rpc o2 sandboxed_sbatch '{
  "job_name": "train-model",
  "command": "python /n/scratch/.../train.py --epochs 100",
  "input_paths": ["/n/data1/.../training_data/"],
  "output_paths": ["/n/scratch/.../checkpoints/"],
  "cpus": 8,
  "memory": {"amount": 64, "unit": "gb"},
  "time": {"days": 2, "hours": 0, "minutes": 0},
  "partition": "gpu_quad",
  "gpu": {"count": 1, "gpu_type": "a100"}
}'
```

**Job array example (parallel processing):**
```bash
remote-bridge rpc o2 sandboxed_sbatch '{
  "job_name": "batch-process",
  "command": "python /n/scratch/.../process.py --task $SLURM_ARRAY_TASK_ID",
  "input_paths": ["/n/data1/.../samples/"],
  "output_paths": ["/n/scratch/.../results/"],
  "cpus": 1,
  "memory": {"amount": 8, "unit": "gb"},
  "time": {"hours": 1},
  "partition": "short",
  "array": "1-100%20"
}'
```

**Required parameters:**
- `job_name`: Name for the job
- `command`: Command to run inside the container

**Path parameters:**
- `input_paths`: Directories to mount read-only (validated against config)
- `output_paths`: Directories to mount read-write (validated against config)
- `working_dir`: Working directory inside the container (must be in output_paths)
- `output`: SLURM stdout file path (e.g., `/path/to/logs/%j.out`)
- `error`: SLURM stderr file path (e.g., `/path/to/logs/%j.err`)

**Resource parameters:**
- `cpus`: Number of CPUs (default: 1)
- `memory`: `{"amount": N, "unit": "mb"|"gb"|"tb"}`
- `time`: `{"days": D, "hours": H, "minutes": M}`
- `partition`: One of `priority`, `short`, `medium`, `long`, `interactive`, `highmem`, `gpu`, `mpi`
- `gpu`: `{"count": N, "gpu_type": "tesla"|"v100"|"a100"}`
- `array`: Array job spec (e.g., `"1-100"` or `"1-100%10"`)

**Container parameters:**
- `image`: Path to .sif file (optional if default configured in permissions.toml)

**Other parameters:**
- `environment`: `{"VAR": "value"}` - Environment variables to set
- `extra_directives`: `{"mail-type": "END"}` - Additional SLURM directives
- `return_script`: If true, response includes the generated script content

**Response:**
```json
{
  "job_id": "12345678",
  "script_path": "/n/scratch/.../scripts/my-analysis_20260117_143022.sh",
  "image_used": "/containers/python.sif",
  "bind_mounts": [
    {"host": "/n/data1/.../input_data/", "container": "/n/data1/.../input_data/", "mode": "ro"},
    {"host": "/n/scratch/.../results/", "container": "/n/scratch/.../results/", "mode": "rw"}
  ],
  "duration_ms": 1234
}
```

**Why use sandboxed_sbatch?**
1. **Security**: The job can only access explicitly listed directories
2. **Reproducibility**: Container ensures consistent environment
3. **Validation**: Paths and resources are validated before submission
4. **Audit trail**: Generated scripts are saved for review

### Cancel Jobs (scancel)

```bash
# Cancel one job
remote-bridge rpc o2 scancel '{"job_ids":["12345678"]}'

# Cancel multiple jobs
remote-bridge rpc o2 scancel '{"job_ids":["12345678","12345679"]}'
```

### Check Queue (squeue)

```bash
# All jobs for a user
remote-bridge rpc o2 squeue '{"user":"ljo8"}'

# Specific job IDs
remote-bridge rpc o2 squeue '{"job_ids":["12345678"]}'

# Filter by state
remote-bridge rpc o2 squeue '{"user":"ljo8","state":"running"}'
```

States: `pending`, `running`, `suspended`, `completed`, `cancelled`, `failed`, `timeout`, `all`

### Job Accounting (sacct)

```bash
# Recent jobs
remote-bridge rpc o2 sacct '{"user":"ljo8","start_time":"now-1day"}'

# Specific job
remote-bridge rpc o2 sacct '{"job_ids":["12345678"]}'
```

Response includes resource usage: `max_rss` (peak memory), `max_vmem` (virtual memory), `n_cpus`, `n_tasks`, `elapsed` time. Use this to check if jobs are using appropriate resources.

### Wait for Job Completion (job_wait)

Polls until job completes, with increasing intervals (0s, 5s, 10s, 15s, ... up to 60s).

```bash
# Wait for a single job
remote-bridge rpc o2 job_wait '{"job_id":"12345678"}'

# Job array: wait for ANY task to complete (default)
remote-bridge rpc o2 job_wait '{"job_id":"12345678","array_mode":"any"}'

# Job array: wait for ALL tasks to complete
remote-bridge rpc o2 job_wait '{"job_id":"12345678","array_mode":"all"}'

# Job array: wait for specific index (e.g., task 5)
remote-bridge rpc o2 job_wait '{"job_id":"12345678","array_mode":{"index":5}}'

# Custom timeout (default: 86400 = 24 hours)
remote-bridge rpc o2 job_wait '{"job_id":"12345678","max_wait_secs":3600}'
```

Response includes exit status for completed jobs:
```json
{
  "job_id": "12345678",
  "completed_jobs": [
    {"job_id": "12345678_1", "state": "COMPLETED", "exit_code": "0:0", "elapsed": "00:05:32"}
  ],
  "all_completed": false,
  "wait_time_secs": 45
}
```

**Use with background tasks:** Run `job_wait` as a background Bash command. Claude will be notified when the job completes.

## Job Submission Workflow

### sandboxed_sbatch Workflow

1. **Identify inputs/outputs**: What data does the job read? Where does it write?
2. **Submit directly**:
   ```bash
   remote-bridge rpc o2 sandboxed_sbatch '{
     "job_name": "analysis-v1",
     "command": "python /n/scratch/.../analyze.py --input /n/data1/.../data/",
     "input_paths": ["/n/data1/.../data/"],
     "output_paths": ["/n/scratch/.../results/"],
     "cpus": 4,
     "memory": {"amount": 32, "unit": "gb"},
     "time": {"hours": 4}
   }'
   ```
3. **Monitor**: `remote-bridge rpc o2 squeue '{"user":"..."}'`
4. **Wait if needed**: `remote-bridge rpc o2 job_wait '{"job_id":"..."}'`

### Git-Based Workflow (Alternative)

Use when scripts need to be version-controlled on O2:

1. User clones project repo on O2 (manually via SSH)
2. User records O2 path in project `CLAUDE.md`
3. Create/edit scripts locally, commit and push
4. Pull on O2: `remote-bridge rpc o2 git_pull '{"path":"/n/data1/.../project"}'`
5. Submit via `sandboxed_sbatch` with the script as the command

See `/use-o2` skill for SLURM script templates and partition selection.

## Permission Enforcement

The bridge validates all paths against `~/.config/remote-bridge/permissions.toml`:

- **Read paths**: Only directories listed in `paths.read` are accessible
- **Write paths**: Only directories listed in `paths.write` can be modified
- **No shell access**: Claude cannot run arbitrary commands

If a request is denied, you'll receive an error response with "Path not allowed".

## Singularity Configuration for sandboxed_sbatch

To use `sandboxed_sbatch`, add a `[singularity]` section to permissions.toml:

```toml
[singularity]
# Default container image (used if no image specified in request)
default_image = "/n/app/containers/users/USERNAME/python-science.sif"

# Directory where generated sbatch scripts are written
# Must be within paths.write
scripts_dir = "/n/scratch/users/u/USERNAME/claude-scripts/"

# Singularity cache directory
cache_dir = "/n/scratch/users/u/USERNAME/.singularity/cache"

# Extra bind mounts always included (beyond input_paths/output_paths)
extra_binds = [
    "/n/app:ro",  # O2 applications (read-only)
]
```

**Creating a container image:**

Option 1 - Pull from Docker Hub (simplest):
```bash
# On O2 or locally
module load singularity
singularity pull python_3.11.sif docker://python:3.11
```

Option 2 - Build with definition file (more control):
```bash
# Requires root or fakeroot - build locally, then transfer to O2
apptainer build python-science.sif my_environment.def
scp python-science.sif USERNAME@transfer.rc.hms.harvard.edu:/n/app/containers/users/USERNAME/
```

See O2 documentation for Singularity details.

## Troubleshooting

### "Bridge 'o2' is not running"

Ask user to start it in a separate terminal:
```bash
remote-bridge start o2 --user USERNAME
```

### Connection not active

The SSH session may have timed out. Ask user to:
1. Stop the bridge: Ctrl+C in the bridge terminal
2. Restart: `remote-bridge start o2 --user USERNAME`

### Permission denied errors

The requested path isn't in the user's permission config. Ask user to:
1. Add the path to `~/.config/remote-bridge/permissions.toml`
2. Run `remote-bridge update-checksum` after editing

## Quick Reference

| Action | Command |
|--------|---------|
| Check status | `remote-bridge rpc o2 connection_status` |
| List directory | `remote-bridge rpc o2 ls '{"path":"...","flags":["Long"]}'` |
| Read file | `remote-bridge rpc o2 cat '{"path":"..."}'` |
| First N lines | `remote-bridge rpc o2 head '{"path":"...","lines":20}'` |
| Line count | `remote-bridge rpc o2 wc '{"path":"...","lines_only":true}'` |
| Search files | `remote-bridge rpc o2 grep '{"pattern":"...","paths":["..."]}'` |
| Find files | `remote-bridge rpc o2 find '{"path":"...","name":"*.py"}'` |
| Download file | `remote-bridge rpc o2 download '{"path":"..."}'` |
| Git pull | `remote-bridge rpc o2 git_pull '{"path":"..."}'` |
| Submit job (raw) | `remote-bridge rpc o2 sbatch '{"script_path":"..."}'` |
| Submit job (sandboxed) | `remote-bridge rpc o2 sandboxed_sbatch '{"job_name":"...","command":"..."}'` |
| Cancel job | `remote-bridge rpc o2 scancel '{"job_ids":["..."]}'` |
| Check queue | `remote-bridge rpc o2 squeue '{"user":"..."}'` |
| Job accounting | `remote-bridge rpc o2 sacct '{"job_ids":["..."]}'` |
| Wait for job | `remote-bridge rpc o2 job_wait '{"job_id":"..."}'` |
