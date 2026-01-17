---
name: remote-o2
description: This skill should be used when the user asks to "submit to O2", "run on O2", "use the cluster", "submit a SLURM job", mentions O2 or compute cluster job submission, or when an analysis requires substantial computational resources (>16GB RAM, >4 hours runtime, or GPUs).
user_invocable: true
version: 2.0.0
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

First, check if the bridge is running:

```bash
ls -la ~/.claude/remote-bridge-o2.sock 2>/dev/null
```

**If socket exists:** Go to [Using the Bridge](#using-the-bridge)
**If socket doesn't exist:** Go to [Starting the Bridge](#starting-the-bridge)

## Starting the Bridge

### First-Time Setup

Follow these steps to install and configure the bridge.

#### Step 1: Check if bridge binary exists

```bash
which remote-bridge || ls $CONFIG_REPO/remote-bridge/target/release/remote-bridge 2>/dev/null
```

**If binary exists:** Skip to [Step 4: Create permissions config](#step-4-create-permissions-config)
**If binary doesn't exist:** Continue to Step 2

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
echo 'export PATH="$PATH:$CONFIG_REPO/remote-bridge/target/release"' >> ~/.zshrc
source ~/.zshrc
```

**For bash (~/.bashrc):**
```bash
echo 'export PATH="$PATH:$CONFIG_REPO/remote-bridge/target/release"' >> ~/.bashrc
source ~/.bashrc
```

Note: Replace `$CONFIG_REPO` with the actual path in the export line (e.g., `~/Dropbox/GitHub/claude-config`).

Verify installation:
```bash
which remote-bridge
```

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

Tell the user to run:

```bash
remote-bridge start o2 --user YOUR_USERNAME
```

The user will see:
1. Password prompt (if not using SSH keys)
2. Duo authentication prompt
3. Confirmation that bridge is ready

**Important:** The bridge runs in the foreground. The user should keep this terminal open or run it in a background session.

Once the user confirms the bridge is running, proceed to [Using the Bridge](#using-the-bridge).

## Using the Bridge

### JSON-RPC Communication

The bridge exposes a Unix socket at `~/.claude/remote-bridge-o2.sock`. Send JSON-RPC 2.0 requests via netcat.

### Check Connection Status

```bash
echo '{"jsonrpc":"2.0","method":"connection_status","id":1}' | nc -U ~/.claude/remote-bridge-o2.sock
```

Expected response:
```json
{"jsonrpc":"2.0","result":{"connected":true,"user":"...","host":"o2.hms.harvard.edu"},"id":1}
```

If `connected: false`, ask the user to restart the bridge.

### List Directory

```bash
echo '{"jsonrpc":"2.0","method":"ls","params":{"path":"/n/data1/...","flags":["Long","Human"]},"id":1}' | nc -U ~/.claude/remote-bridge-o2.sock
```

Available flags: `Long`, `All`, `Human`, `Recursive`, `SortByTime`, `SortBySize`

### Read File

```bash
# Full file
echo '{"jsonrpc":"2.0","method":"cat","params":{"path":"/path/to/file.txt"},"id":1}' | nc -U ~/.claude/remote-bridge-o2.sock

# First 100 lines
echo '{"jsonrpc":"2.0","method":"cat","params":{"path":"/path/to/file.txt","head":100},"id":1}' | nc -U ~/.claude/remote-bridge-o2.sock

# Last 50 lines
echo '{"jsonrpc":"2.0","method":"cat","params":{"path":"/path/to/file.txt","tail":50},"id":1}' | nc -U ~/.claude/remote-bridge-o2.sock

# Lines 100-200
echo '{"jsonrpc":"2.0","method":"cat","params":{"path":"/path/to/file.txt","offset":100,"limit":100},"id":1}' | nc -U ~/.claude/remote-bridge-o2.sock
```

### Search Files (Grep)

```bash
echo '{"jsonrpc":"2.0","method":"grep","params":{"pattern":"def main","paths":["/path/to/search/"],"flags":["Recursive","LineNumbers"]},"id":1}' | nc -U ~/.claude/remote-bridge-o2.sock
```

Available flags: `IgnoreCase`, `Recursive`, `LineNumbers`, `InvertMatch`, `WordMatch`, `CountOnly`, `FilesWithMatches`

## Command Patterns

### Simple Wrapper Function

For cleaner commands, define a function:

```bash
o2_rpc() {
    echo "$1" | nc -U ~/.claude/remote-bridge-o2.sock
}

# Then use:
o2_rpc '{"jsonrpc":"2.0","method":"ls","params":{"path":"/n/data1/...","flags":[]},"id":1}'
```

### Exploring Directory Structure

```bash
# List top-level contents
o2_rpc '{"jsonrpc":"2.0","method":"ls","params":{"path":"/n/data1/hms/dbmi/oconnor/lab/luke/","flags":["Long","Human"]},"id":1}'

# Check subdirectory
o2_rpc '{"jsonrpc":"2.0","method":"ls","params":{"path":"/n/data1/.../project/","flags":["All"]},"id":1}'
```

### Finding Files

```bash
# Search for Python files containing a function
o2_rpc '{"jsonrpc":"2.0","method":"grep","params":{"pattern":"def process_data","paths":["/n/data1/.../project/"],"flags":["Recursive","LineNumbers"]},"id":1}'
```

### Reading Code

```bash
# Read a script
o2_rpc '{"jsonrpc":"2.0","method":"cat","params":{"path":"/n/data1/.../script.py"},"id":1}'

# Just the first 50 lines
o2_rpc '{"jsonrpc":"2.0","method":"cat","params":{"path":"/n/data1/.../script.py","head":50},"id":1}'
```

## Permission Enforcement

The bridge validates all paths against `~/.config/remote-bridge/permissions.toml`:

- **Read paths**: Only directories listed in `paths.read` are accessible
- **Write paths**: Only directories listed in `paths.write` can be modified
- **No shell access**: Claude cannot run arbitrary commands

If a request is denied, you'll receive an error response:
```json
{"jsonrpc":"2.0","error":{"code":403,"message":"Path not allowed: /unauthorized/path"},"id":1}
```

## SLURM Commands (Coming Soon)

SLURM commands (squeue, sacct, sbatch) are planned for future versions. For now, use the `/use-o2` skill for SLURM reference and ask the user to run SLURM commands manually.

## Troubleshooting

### Socket doesn't exist

The bridge isn't running. Ask user to start it:
```bash
remote-bridge start o2 --user USERNAME
```

### Connection refused / Connection not active

The SSH session may have timed out. Ask user to:
1. Stop the bridge: Ctrl+C in the bridge terminal
2. Restart: `remote-bridge start o2 --user USERNAME`

### Permission denied errors

The requested path isn't in the user's permission config. Ask user to either:
1. Add the path to `~/.config/remote-bridge/permissions.toml`
2. Run `remote-bridge update-checksum` after editing

### Command timeout

Default timeout is 30-120 seconds. For long operations, SLURM job submission is preferred (coming in future version).

## Quick Reference

| Action | Command |
|--------|---------|
| Check status | `echo '{"jsonrpc":"2.0","method":"connection_status","id":1}' \| nc -U ~/.claude/remote-bridge-o2.sock` |
| List directory | `ls` method with path and flags |
| Read file | `cat` method with path (optional: head/tail/offset/limit) |
| Search files | `grep` method with pattern, paths, flags |
| Stop bridge | User presses Ctrl+C in bridge terminal |
