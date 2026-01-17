---
name: remote-o2
description: This skill should be used when the user asks to "submit to O2", "run on O2", "use the cluster", "submit a SLURM job", mentions O2 or compute cluster job submission, or when an analysis requires substantial computational resources (>16GB RAM, >4 hours runtime, or GPUs).
user_invocable: true
version: 2.1.0
---

# Remote O2 Access Skill

This skill enables Claude Code to access the O2 cluster remotely from a local machine via the `remote-bridge` application.

## Key Benefit: Single Duo Push

The bridge establishes a persistent SSH session with proper terminal emulation. You authenticate once (Duo push), then run unlimited commands through that session.

## When This Skill Applies

**Auto-trigger when:**
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

### First-Time Setup

#### Step 1: Check if bridge binary exists

```bash
which remote-bridge
```

**If found:** Skip to [Step 4: Create permissions config](#step-4-create-permissions-config)
**If not found:** Continue to Step 2

#### Step 2: Check for Rust/Cargo

```bash
which cargo
```

**If cargo exists:** Skip to [Step 3: Build the bridge](#step-3-build-the-bridge)

**If cargo doesn't exist:** Ask the user to install Rust:

```
Rust is required to build the remote-bridge. Please install it:

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Follow the prompts (default installation is fine), then restart your terminal
or run: source ~/.cargo/env

Let me know when Rust is installed.
```

#### Step 3: Build the bridge

```bash
cd $CONFIG_REPO/remote-bridge && cargo build --release
```

Then add to PATH. First detect the user's shell:

```bash
echo $SHELL
```

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

Note: Replace `/path/to/claude-config` with the actual CONFIG_REPO path.

#### Step 4: Create permissions config

```bash
mkdir -p ~/.config/remote-bridge
cp $CONFIG_REPO/remote-bridge/config/permissions.example.toml ~/.config/remote-bridge/permissions.toml
```

#### Step 5: Edit permissions

Ask the user for their O2 paths:
- Lab directory (e.g., `/n/data1/hms/dbmi/oconnor/lab/username/`)
- Scratch directory (e.g., `/n/scratch/users/u/username/`)

Then edit the config:
```bash
$EDITOR ~/.config/remote-bridge/permissions.toml
```

Update the `[paths]` section with their actual directories.

### Start the Bridge

Tell the user to run in a separate terminal:

```bash
remote-bridge start o2 --user YOUR_USERNAME
```

The user will see:
1. Password prompt (if not using SSH keys)
2. Duo authentication prompt
3. Confirmation that bridge is ready

**Important:** The bridge runs in the foreground. The user should keep that terminal open.

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

### Git Pull

```bash
remote-bridge rpc o2 git_pull '{"path":"/n/data1/.../project"}'
```

Optional params: `remote` (default: "origin"), `branch` (default: current)

### Submit Job (sbatch)

```bash
remote-bridge rpc o2 sbatch '{"script_path":"/n/data1/.../job.sh"}'
```

Optional: `working_dir` - directory to run sbatch from

Response includes `job_id` of submitted job.

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

## Git-Based Job Submission Workflow

This workflow enables Claude to create and submit SLURM jobs:

### One-Time Setup

1. User clones project repo on O2 (manually via SSH)
2. User records O2 path in project `CLAUDE.md`:
   ```markdown
   ## O2 Paths
   - O2 repo: /n/data1/hms/dbmi/.../project
   ```

### Workflow

1. **Create sbatch script** locally in project
2. **Commit and push** to git
3. **Pull on O2**: `remote-bridge rpc o2 git_pull '{"path":"/n/data1/.../project"}'`
4. **Submit job**: `remote-bridge rpc o2 sbatch '{"script_path":"/n/data1/.../job.sh"}'`
5. **Monitor**: `remote-bridge rpc o2 squeue '{"user":"..."}'`

See `/use-o2` skill for SLURM script templates and partition selection.

## Permission Enforcement

The bridge validates all paths against `~/.config/remote-bridge/permissions.toml`:

- **Read paths**: Only directories listed in `paths.read` are accessible
- **Write paths**: Only directories listed in `paths.write` can be modified
- **No shell access**: Claude cannot run arbitrary commands

If a request is denied, you'll receive an error response with "Path not allowed".

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
| Search files | `remote-bridge rpc o2 grep '{"pattern":"...","paths":["..."]}'` |
| Git pull | `remote-bridge rpc o2 git_pull '{"path":"..."}'` |
| Submit job | `remote-bridge rpc o2 sbatch '{"script_path":"..."}'` |
| Check queue | `remote-bridge rpc o2 squeue '{"user":"..."}'` |
| Wait for job | `remote-bridge rpc o2 job_wait '{"job_id":"..."}'` |
