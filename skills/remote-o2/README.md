# Remote O2 Access Skill

Access the Harvard O2 cluster from your local machine through Claude Code.

## Overview

This skill enables Claude Code to execute commands on O2 without you having to SSH manually. It uses SSH multiplexing to maintain a persistent connection and tmux for session persistence.

**Key features:**
- One-time setup, then automatic connection management
- Persistent sessions survive network hiccups
- Integrates with `/use-o2` for SLURM job submission
- Auto-triggers when O2 resources are needed

## How It Works

```
Local Machine                          O2 Cluster
┌─────────────┐     SSH Master        ┌─────────────┐
│ Claude Code │ ──────────────────────│ tmux session│
│             │     (Duo once)        │   "claude"  │
│ /remote-o2  │ ←────────────────────→│             │
└─────────────┘     Commands/Output   └─────────────┘
```

1. **SSH master connection** - Authenticated once with Duo, stays open
2. **tmux session** - Persistent shell on O2 that survives disconnects
3. **Command execution** - Claude sends commands via `tmux send-keys`, captures output via `tmux capture-pane`

## Setup

### First Time

When you first invoke `/remote-o2`, Claude will:

1. Ask for your O2 username and directories
2. Generate two scripts in `<config-repo>/o2-scripts/`:
   - `o2-setup.sh` - One-time setup to run on O2
   - `connect-o2.sh` - Local script to establish connection

3. Guide you through running them

### Each Session

At the start of a work session, run the connect script:

```bash
./path/to/claude-config/o2-scripts/connect-o2.sh
```

This:
- Establishes the SSH master connection (Duo prompt)
- Starts/reattaches the tmux session on O2
- Connection persists until you close it or your laptop sleeps

## Usage

### Explicit Invocation

```
/remote-o2
```

Claude will check connection status and reconnect if needed.

### Auto-Triggering

The skill auto-triggers when:
- You mention "O2", "cluster", or "SLURM"
- An analysis requires >16GB RAM or >4 hours runtime
- You ask Claude to run something on the cluster

### Example Workflows

**Run a quick command on O2:**
```
"Check what's in my O2 home directory"
→ Claude connects (if needed) and runs ls
```

**Submit a SLURM job:**
```
"Run this analysis on O2, it needs 64GB RAM"
→ Claude connects, creates SLURM script, submits, monitors
```

**Transfer files:**
```
"Upload analysis.py to O2 and run it"
→ Claude uses scp through the master connection
```

## Configuration

Settings are stored in `~/.claude/behavior.conf`:

```bash
# O2 Remote Access
O2_USER=lukeo
O2_LAB_DIR=/n/data1/hms/dbmi/xxx/lukeo
O2_SCRATCH_DIR=/n/scratch/users/l/lukeo
O2_SOCKET=/tmp/o2-socket
O2_TMUX_SESSION=claude
O2_REMOTE_SETUP=true
```

| Variable | Purpose |
|----------|---------|
| `O2_USER` | Your O2 username |
| `O2_LAB_DIR` | Permanent storage (code, results) |
| `O2_SCRATCH_DIR` | Temporary storage (large outputs) |
| `O2_SOCKET` | SSH master socket path |
| `O2_TMUX_SESSION` | tmux session name |
| `O2_REMOTE_SETUP` | Whether setup is complete |

## Directory Structure on O2

After setup:

```
$O2_LAB_DIR/
└── claude-projects/     # Your project files (permanent)

$O2_SCRATCH_DIR/
└── claude-tmp/          # Temporary files, large outputs (cleaned periodically)
```

## Troubleshooting

### "Connection expired" message

The SSH master connection dies when:
- Your laptop sleeps or disconnects from network
- The connection times out (rare with ControlPersist=yes)
- O2 restarts

**Solution:** Run the connect script again:
```bash
./o2-scripts/connect-o2.sh
```

### Commands hang or timeout

**Possible causes:**
- Command is still running (check tmux)
- O2 is under heavy load
- Network issues

**Manual check:**
```bash
# Check tmux directly
ssh o2.hms.harvard.edu
tmux attach -t claude
```

### "Socket not found" errors

The socket file may have been cleaned up:
```bash
rm -f /tmp/o2-socket  # Clean up stale socket
./o2-scripts/connect-o2.sh  # Reconnect
```

### Need to reset everything

```bash
# Kill existing connection
ssh -S /tmp/o2-socket -O exit o2.hms.harvard.edu 2>/dev/null
rm -f /tmp/o2-socket

# On O2, kill tmux session
ssh o2.hms.harvard.edu "tmux kill-session -t claude"

# Reconnect fresh
./o2-scripts/connect-o2.sh
```

## Integration with Other Skills

### /use-o2

When you're connected remotely, `/use-o2` patterns work through the connection:
- SLURM job creation and submission
- Job monitoring with progressive intervals
- Resource estimation and tracking

### /perform-analysis

If an analysis requires O2 resources:
1. `/perform-analysis` detects resource needs
2. Auto-invokes `/remote-o2` to ensure connection
3. Executes analysis on O2

## Limitations

- **Duo authentication** - Required once per session (unavoidable)
- **Output capture** - Limited by tmux scrollback; very large outputs redirect to file
- **Interactive programs** - Programs requiring TTY interaction may not work well
- **Network dependency** - Connection dies if network drops (but tmux preserves work)

## Security Notes

- The SSH socket is stored in `/tmp/` and is only accessible by your user
- Connection uses your existing SSH keys and O2 credentials
- No passwords or tokens are stored by this skill
