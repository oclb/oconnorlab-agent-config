# Remote Bridge

A secure Rust application for Claude Code to access remote hosts via SSH with minimal authentication overhead.

## Problem

When accessing O2 (or similar clusters) from Claude Code, each SSH command triggers a Duo push. This makes interactive work painful.

## Solution

Remote Bridge establishes a **single persistent SSH session** with proper terminal emulation (PTY), allowing:
- One-time Duo authentication during connection
- Unlimited commands through that session
- Structured JSON-RPC API (not arbitrary shell commands)
- Path-based permission enforcement

## Architecture

```
┌─────────────┐     JSON-RPC      ┌───────────────┐     PTY/SSH     ┌─────────┐
│ Claude Code │ ◄───────────────► │ remote-bridge │ ◄─────────────► │  O2     │
└─────────────┘   Unix Socket     └───────────────┘  Persistent     └─────────┘
                                        │             Session
                                        ▼
                              permissions.toml (protected)
```

## Quick Start

### 1. Build

```bash
cd remote-bridge
cargo build --release
```

Binary will be at `target/release/remote-bridge`.

### 2. Configure Permissions

```bash
mkdir -p ~/.config/remote-bridge
cp config/permissions.example.toml ~/.config/remote-bridge/permissions.toml
# Edit to match your paths
$EDITOR ~/.config/remote-bridge/permissions.toml
```

### 3. Start Bridge

```bash
remote-bridge start o2 --user YOUR_USERNAME
```

This will:
1. Prompt for password (if not using SSH keys)
2. Trigger Duo authentication
3. Establish persistent SSH session
4. Start JSON-RPC server on Unix socket

### 4. Send Commands

From another terminal (or Claude Code):

```bash
# Connection status
echo '{"jsonrpc":"2.0","method":"connection_status","id":1}' | nc -U ~/.claude/remote-bridge-o2.sock

# List directory
echo '{"jsonrpc":"2.0","method":"ls","params":{"path":"/n/data1/...","flags":[]},"id":2}' | nc -U ~/.claude/remote-bridge-o2.sock

# Read file
echo '{"jsonrpc":"2.0","method":"cat","params":{"path":"/n/data1/.../file.txt"},"id":3}' | nc -U ~/.claude/remote-bridge-o2.sock
```

### 5. Stop Bridge

Press Ctrl+C in the terminal running the bridge, or:

```bash
remote-bridge stop o2
```

## CLI Commands

| Command | Purpose |
|---------|---------|
| `start <name>` | Start bridge with given name |
| `status <name>` | Check if bridge is running |
| `stop <name>` | Stop bridge (removes socket) |
| `verify-config` | Check config integrity |
| `update-checksum` | Update checksum after config changes |

### Start Options

```
remote-bridge start o2 \
  --user ljo8 \
  --host o2.hms.harvard.edu \
  --config ~/.config/remote-bridge/permissions.toml
```

## JSON-RPC Methods

### connection_status

Check if SSH session is alive.

```json
{"jsonrpc":"2.0","method":"connection_status","id":1}
```

Response:
```json
{
  "jsonrpc": "2.0",
  "result": {
    "connected": true,
    "user": "ljo8",
    "host": "o2.hms.harvard.edu"
  },
  "id": 1
}
```

### ls

List directory contents.

```json
{
  "jsonrpc": "2.0",
  "method": "ls",
  "params": {
    "path": "/n/data1/hms/dbmi/oconnor/lab/luke/",
    "flags": ["Long", "All", "Human"]
  },
  "id": 2
}
```

Flags: `Long`, `All`, `Human`, `Recursive`, `SortByTime`, `SortBySize`

### cat

Read file contents.

```json
{
  "jsonrpc": "2.0",
  "method": "cat",
  "params": {
    "path": "/n/data1/.../file.txt",
    "head": 100
  },
  "id": 3
}
```

Options: `head`, `tail`, `offset`, `limit`

### grep

Search files for pattern.

```json
{
  "jsonrpc": "2.0",
  "method": "grep",
  "params": {
    "pattern": "def main",
    "paths": ["/n/data1/.../src/"],
    "flags": ["Recursive", "LineNumbers"]
  },
  "id": 4
}
```

Flags: `IgnoreCase`, `Recursive`, `LineNumbers`, `InvertMatch`, `WordMatch`, `CountOnly`, `FilesWithMatches`

### git_pull

Pull latest changes in a git repository.

```json
{
  "jsonrpc": "2.0",
  "method": "git_pull",
  "params": {
    "path": "/n/data1/.../project",
    "remote": "origin"
  },
  "id": 5
}
```

Options: `remote` (default: "origin"), `branch` (optional)

### squeue

Check SLURM job queue.

```json
{
  "jsonrpc": "2.0",
  "method": "squeue",
  "params": {
    "user": "ljo8"
  },
  "id": 6
}
```

Options: `user`, `job_ids`, `partition`, `state`

States: `pending`, `running`, `suspended`, `completed`, `cancelled`, `failed`, `timeout`, `all`

### sacct

Get job accounting information.

```json
{
  "jsonrpc": "2.0",
  "method": "sacct",
  "params": {
    "job_ids": ["12345678"],
    "start_time": "now-1day"
  },
  "id": 7
}
```

Options: `job_ids`, `user`, `start_time`, `end_time`, `state`

### sbatch

Submit a SLURM batch job.

```json
{
  "jsonrpc": "2.0",
  "method": "sbatch",
  "params": {
    "script_path": "/n/data1/.../job.sh",
    "working_dir": "/n/data1/.../project"
  },
  "id": 8
}
```

Returns `job_id` of submitted job.

### job_wait

Wait for a job to complete, polling with increasing intervals (0s, 5s, 10s, 15s, ... up to 60s).

```json
{
  "jsonrpc": "2.0",
  "method": "job_wait",
  "params": {
    "job_id": "12345678",
    "array_mode": "any",
    "max_wait_secs": 86400
  },
  "id": 9
}
```

Options:
- `job_id`: Job ID to wait for
- `array_mode`: For job arrays - `"any"` (default, first to complete), `"all"` (wait for all), or `{"index": N}` (specific task)
- `max_wait_secs`: Timeout in seconds (default: 86400 = 24 hours)

Response includes exit status:
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

### shutdown

Request graceful shutdown.

```json
{"jsonrpc":"2.0","method":"shutdown","id":99}
```

## Permission Config

The bridge enforces path-based permissions via `~/.config/remote-bridge/permissions.toml`:

```toml
[paths]
# Directories Claude can read from
read = [
    "/n/data1/hms/dbmi/oconnor/lab/luke/",
    "/n/scratch/users/l/ljo8/",
]

# Directories Claude can write to (must be under read paths)
write = [
    "/n/data1/hms/dbmi/oconnor/lab/luke/claude-projects/",
]

[resources]
max_cpus = 32
max_memory_gb = 128
max_time_hours = 120
max_gpus = 2

[modules]
allowed = ["python/3.9.14", "gcc/9.2.0"]
```

### Config Integrity

The bridge verifies config hasn't been tampered with:

1. First run creates a checksum baseline
2. Subsequent runs verify the checksum matches
3. If modified, bridge refuses to start

After intentional changes:
```bash
remote-bridge update-checksum
```

## Security Model

1. **No shell access**: Claude cannot run arbitrary commands - only predefined operations
2. **Path enforcement**: All file operations validated against allowed paths
3. **Type-safe commands**: Flags are enums, not arbitrary strings
4. **Config protection**: Checksum prevents unauthorized permission changes
5. **Local only**: Unix socket limits access to local processes

## How It Works

### PTY-Based SSH

The bridge uses a pseudo-terminal (PTY) to spawn SSH, providing:
- Proper terminal emulation (SSH thinks it's interactive)
- Duo authentication works correctly during connect
- Single persistent session for all commands

### Threaded I/O During Auth

During connection, three threads handle I/O:
1. **Reader thread**: PTY output → user's terminal
2. **Writer thread**: Keyboard input + probes → PTY
3. **Main thread**: Polls keyboard with crossterm

A sentinel (`echo __READY_xxx__`) detects when shell is ready.

### Command Execution

Commands use unique sentinels to capture output:
```bash
echo '__START_uuid__'; command 2>&1; echo '__END_uuid__'$?
```

## Troubleshooting

### "Bridge appears to be running"

```bash
remote-bridge stop o2
# or
rm ~/.claude/remote-bridge-o2.sock
```

### No Duo during connect

If you see password prompt but no Duo push, the PTY initialization may have failed. Check logs and try reconnecting.

### Commands timing out

Default timeout is 30-120 seconds depending on command. For long operations, consider implementing job submission (Phase 3/4).

## Development

```bash
# Build debug
cargo build

# Run with logging
RUST_LOG=debug cargo run -- start o2 --user ljo8

# Run tests
cargo test
```

## Roadmap

- [x] Core infrastructure (CLI, config, RPC server)
- [x] SSH with PTY for proper Duo auth
- [x] Filesystem commands (ls, cat, grep)
- [x] Git commands (git_pull)
- [x] SLURM commands (squeue, sacct, sbatch)
- [ ] Job script templating with Singularity support
- [ ] Audit logging
