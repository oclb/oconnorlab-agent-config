---
name: remote-o2
description: This skill should be used when the user asks to "submit to O2", "run on O2", "use the cluster", "submit a SLURM job", mentions O2 or compute cluster job submission, or when an analysis requires substantial computational resources (>16GB RAM, >4 hours runtime, or GPUs).
user_invocable: true
version: 1.0.0
---

# Remote O2 Access Skill

This skill enables Claude Code to access the O2 cluster remotely from a local machine via SSH multiplexing and tmux. It handles setup, connection management, and command execution.

## When This Skill Applies

**Auto-trigger when:**
- User mentions "O2", "cluster", or "SLURM" while `Environment=local`
- Analysis requires substantial resources (>16GB RAM, >4 hours, GPUs)
- User explicitly invokes `/remote-o2`

**Check environment first:**
```bash
grep "^Environment=" ~/.claude/behavior.conf
```
- If `Environment=O2`: You're already on O2. Use `/use-o2` skill instead.
- If `Environment=local`: Continue with this skill.

## Step 1: Check Setup Status

Read behavior.conf and check `O2_REMOTE_SETUP`:

```bash
grep "^O2_" ~/.claude/behavior.conf
```

**If `O2_REMOTE_SETUP=false` or missing:** Go to [First-Time Setup](#first-time-setup)
**If `O2_REMOTE_SETUP=true`:** Go to [Connection Management](#connection-management)

## First-Time Setup

### 1.1 Collect User Information

Ask the user for:

1. **O2 username** (required)
   - Ask: "What is your O2 username?"

2. **Lab directory** (required)
   - Ask: "What should Claude use as the top-level directory in which to store permanent files? (For example, `/n/data1/hms/dbmi/oconnor/lab/your_name/`)"
   - This is where project code and results live

3. **Scratch directory** (optional, has default)
   - Temporary storage for large files
   - Default: `/n/scratch/users/{first_letter_of_username}/{username}` (derived from answer to question 1)
   - Example: if username is `lukeo`, default is `/n/scratch/users/l/lukeo`

### 1.2 Get CONFIG_REPO Path

```bash
grep "^CONFIG_REPO=" ~/.claude/behavior.conf | cut -d'=' -f2
```

### 1.3 Generate Setup Scripts

Create directory for scripts:
```bash
mkdir -p "$CONFIG_REPO/o2-scripts"
```

**Generate `o2-setup.sh`** (to run ON O2):

```bash
cat > "$CONFIG_REPO/o2-scripts/o2-setup.sh" << 'EOF'
#!/bin/bash
# O2 Setup Script for Claude Code Remote Access
# Run this ON O2 after SSH'ing in

set -e

# Configuration (filled in by Claude)
LAB_DIR="__LAB_DIR__"
SCRATCH_DIR="__SCRATCH_DIR__"

echo "Setting up Claude Code remote access on O2..."

# Create directories
mkdir -p "$LAB_DIR/claude-projects"
mkdir -p "$SCRATCH_DIR/claude-tmp"
mkdir -p ~/bin

# Create tmux session starter
cat > ~/bin/start-claude-session.sh << 'SCRIPT'
#!/bin/bash
SESSION="claude"
WORKSPACE="__LAB_DIR__/claude-projects"

# Check if session exists
if tmux has-session -t $SESSION 2>/dev/null; then
    echo "Session $SESSION already exists, reattaching workspace"
else
    tmux new-session -d -s $SESSION -c "$WORKSPACE"
    echo "Created new session $SESSION in $WORKSPACE"
fi
SCRIPT

# Fix the workspace path in the script
sed -i "s|__LAB_DIR__|$LAB_DIR|g" ~/bin/start-claude-session.sh
chmod +x ~/bin/start-claude-session.sh

echo ""
echo "Setup complete!"
echo "  Lab workspace: $LAB_DIR/claude-projects"
echo "  Scratch space: $SCRATCH_DIR/claude-tmp"
echo "  Tmux starter:  ~/bin/start-claude-session.sh"
EOF
```

Replace `__LAB_DIR__` and `__SCRATCH_DIR__` with actual values:
```bash
sed -i '' "s|__LAB_DIR__|$LAB_DIR|g" "$CONFIG_REPO/o2-scripts/o2-setup.sh"
sed -i '' "s|__SCRATCH_DIR__|$SCRATCH_DIR|g" "$CONFIG_REPO/o2-scripts/o2-setup.sh"
chmod +x "$CONFIG_REPO/o2-scripts/o2-setup.sh"
```

**Generate `connect-o2.sh`** (to run locally):

```bash
cat > "$CONFIG_REPO/o2-scripts/connect-o2.sh" << 'EOF'
#!/bin/bash
# Connect to O2 for Claude Code
# Run this locally to establish SSH master connection

set -e

SOCKET="__SOCKET__"
USER="__USER__"
HOST="o2.hms.harvard.edu"

echo "Connecting to O2..."

# Clean up dead socket if present
if [ -e "$SOCKET" ]; then
    if ! ssh -S "$SOCKET" -O check $HOST 2>/dev/null; then
        echo "Removing stale socket..."
        rm -f "$SOCKET"
    else
        echo "Connection already active!"
        ssh -S "$SOCKET" $HOST "~/bin/start-claude-session.sh"
        exit 0
    fi
fi

# Establish master connection (will prompt for Duo)
echo "Establishing SSH master connection (Duo authentication required)..."
ssh -M -S "$SOCKET" -o ControlPersist=yes -fN ${USER}@${HOST}

# Start tmux session
echo "Starting tmux session..."
ssh -S "$SOCKET" $HOST "~/bin/start-claude-session.sh"

echo ""
echo "O2 connection ready for Claude Code!"
EOF
```

Replace placeholders:
```bash
sed -i '' "s|__SOCKET__|$O2_SOCKET|g" "$CONFIG_REPO/o2-scripts/connect-o2.sh"
sed -i '' "s|__USER__|$O2_USER|g" "$CONFIG_REPO/o2-scripts/connect-o2.sh"
chmod +x "$CONFIG_REPO/o2-scripts/connect-o2.sh"
```

### 1.4 Instruct User to Run Setup

Tell the user:

```
I've created the setup scripts. Please run these two commands:

1. First, run the O2 setup (requires Duo authentication):
   ssh <username>@o2.hms.harvard.edu 'bash -s' < <config_repo>/o2-scripts/o2-setup.sh

2. After that completes, establish the connection:
   <config_repo>/o2-scripts/connect-o2.sh

Let me know when both are done.
```

### 1.5 Update behavior.conf

After user confirms setup is complete:

```bash
# Add O2 remote config to behavior.conf
cat >> ~/.claude/behavior.conf << EOF

# O2 Remote Access (managed by /remote-o2 skill)
O2_USER=$O2_USER
O2_LAB_DIR=$O2_LAB_DIR
O2_SCRATCH_DIR=$O2_SCRATCH_DIR
O2_SOCKET=/tmp/o2-socket
O2_TMUX_SESSION=claude
O2_REMOTE_SETUP=true
EOF
```

## Connection Management

### Check Connection Status

```bash
O2_SOCKET=$(grep "^O2_SOCKET=" ~/.claude/behavior.conf | cut -d'=' -f2)
ssh -S "$O2_SOCKET" -O check o2.hms.harvard.edu 2>/dev/null
```

**If exit code 0:** Connection is alive. Ready to execute commands.

**If exit code non-zero:** Connection is dead. Tell user:
```
O2 connection expired. Please reconnect:
  <config_repo>/o2-scripts/connect-o2.sh

This requires Duo authentication. Let me know when done.
```

### Verify Tmux Session

After connection confirmed:
```bash
ssh -S "$O2_SOCKET" o2.hms.harvard.edu "tmux has-session -t claude 2>/dev/null && echo 'Session exists' || ~/bin/start-claude-session.sh"
```

## Command Execution

### Basic Command Pattern

Use sentinel-based completion detection:

```bash
# Read config
O2_SOCKET=$(grep "^O2_SOCKET=" ~/.claude/behavior.conf | cut -d'=' -f2)
O2_LAB_DIR=$(grep "^O2_LAB_DIR=" ~/.claude/behavior.conf | cut -d'=' -f2)

# Generate unique sentinel
SENTINEL="__CLAUDE_DONE_${RANDOM}_$(date +%s)__"

# Send command (cd to lab dir first)
ssh -S "$O2_SOCKET" o2.hms.harvard.edu \
    "tmux send-keys -t claude 'cd $O2_LAB_DIR/claude-projects && YOUR_COMMAND_HERE; echo $SENTINEL' Enter"

# Poll for completion (with timeout)
MAX_WAIT=300  # 5 minutes default
ELAPSED=0
while [ $ELAPSED -lt $MAX_WAIT ]; do
    sleep 2
    ELAPSED=$((ELAPSED + 2))
    OUTPUT=$(ssh -S "$O2_SOCKET" o2.hms.harvard.edu "tmux capture-pane -t claude -p -S -100")
    if echo "$OUTPUT" | grep -q "$SENTINEL"; then
        break
    fi
done

# Display output (excluding sentinel line)
echo "$OUTPUT" | grep -v "$SENTINEL"
```

### Large Output Pattern

For commands with substantial output, redirect to scratch:

```bash
O2_SCRATCH_DIR=$(grep "^O2_SCRATCH_DIR=" ~/.claude/behavior.conf | cut -d'=' -f2)
OUTPUT_FILE="$O2_SCRATCH_DIR/claude-tmp/output_$(date +%s).txt"

# Send command with output redirection
ssh -S "$O2_SOCKET" o2.hms.harvard.edu \
    "tmux send-keys -t claude 'YOUR_COMMAND_HERE > $OUTPUT_FILE 2>&1; echo $SENTINEL' Enter"

# After sentinel detected, read the file
ssh -S "$O2_SOCKET" o2.hms.harvard.edu "cat $OUTPUT_FILE"

# Clean up
ssh -S "$O2_SOCKET" o2.hms.harvard.edu "rm -f $OUTPUT_FILE"
```

### File Transfer Pattern

**Upload file to O2:**
```bash
scp -o "ControlPath=$O2_SOCKET" local_file.py ${O2_USER}@o2.hms.harvard.edu:$O2_LAB_DIR/claude-projects/
```

**Download file from O2:**
```bash
scp -o "ControlPath=$O2_SOCKET" ${O2_USER}@o2.hms.harvard.edu:$O2_LAB_DIR/claude-projects/results.csv ./
```

### Multi-line Script Pattern

For complex scripts, write to a temp file and execute:

```bash
# Write script locally
cat > /tmp/claude/o2_script.sh << 'SCRIPT'
#!/bin/bash
cd /path/to/project
python analysis.py --input data.csv
SCRIPT

# Upload and execute
scp -o "ControlPath=$O2_SOCKET" /tmp/claude/o2_script.sh ${O2_USER}@o2.hms.harvard.edu:$O2_SCRATCH_DIR/claude-tmp/
ssh -S "$O2_SOCKET" o2.hms.harvard.edu \
    "tmux send-keys -t claude 'bash $O2_SCRATCH_DIR/claude-tmp/o2_script.sh; echo $SENTINEL' Enter"
```

## Compute Resource Decisions

When a command needs significant resources, decide between:

### Option 1: Interactive Node (for iterative work)

```bash
# Request interactive node
ssh -S "$O2_SOCKET" o2.hms.harvard.edu \
    "tmux send-keys -t claude 'srun --pty -p interactive -t 0-4:00 -c 4 --mem=16G /bin/bash' Enter"

# Wait for allocation (poll for changed hostname)
while true; do
    sleep 5
    HOSTNAME=$(ssh -S "$O2_SOCKET" o2.hms.harvard.edu "tmux capture-pane -t claude -p -S -5" | grep -o 'compute-[^ ]*' | tail -1)
    if [ -n "$HOSTNAME" ]; then
        echo "Got interactive node: $HOSTNAME"
        break
    fi
done
```

### Option 2: Batch Job (for long-running work)

Use `/use-o2` skill patterns but execute through the remote connection:

```bash
# Create and submit job script
ssh -S "$O2_SOCKET" o2.hms.harvard.edu "tmux send-keys -t claude 'sbatch job.sh; echo $SENTINEL' Enter"
```

**Decision guide:**
- Quick exploration/testing (<30 min): Run directly on login node
- Interactive development (30 min - 4 hours): Request interactive node
- Long-running analysis (>4 hours): Submit as batch job

## Integration with /use-o2

When connected remotely:
1. This skill handles connection management
2. `/use-o2` patterns handle SLURM job creation and monitoring
3. Execute `/use-o2` commands through the remote connection

Example workflow:
```
1. /remote-o2 ensures connection
2. User requests resource-intensive analysis
3. Claude creates SLURM script using /use-o2 patterns
4. Claude submits via: ssh -S $socket o2 "tmux send-keys -t claude 'sbatch script.sh' Enter"
5. Claude monitors via: ssh -S $socket o2 "tmux send-keys -t claude 'squeue -u $USER' Enter"
```

## Troubleshooting

### Connection Issues

**"Connection refused" or socket errors:**
```bash
# Remove stale socket
rm -f /tmp/o2-socket

# User re-runs connect script
./o2-scripts/connect-o2.sh
```

**Tmux session not found:**
```bash
ssh -S "$O2_SOCKET" o2.hms.harvard.edu "~/bin/start-claude-session.sh"
```

### Command Timeout

If sentinel not detected within timeout:
1. Capture more output: `-S -500` instead of `-S -100`
2. Check if command is still running
3. For very long commands, increase timeout or use batch job

### Output Truncation

If output is truncated by capture-pane:
1. Use the large output pattern (redirect to file)
2. Or request specific portions: `head -100`, `tail -100`

## Quick Reference

| Action | Command |
|--------|---------|
| Check connection | `ssh -S $socket -O check o2` |
| Send command | `ssh -S $socket o2 "tmux send-keys -t claude 'cmd' Enter"` |
| Capture output | `ssh -S $socket o2 "tmux capture-pane -t claude -p -S -100"` |
| Upload file | `scp -o "ControlPath=$socket" file user@o2:path/` |
| Download file | `scp -o "ControlPath=$socket" user@o2:path/file ./` |
| Kill connection | `ssh -S $socket -O exit o2` |
