---
name: setup-o2
description: Internal init-project subskill for first-time O2/SLURM remote-bridge setup, including bridge installation, permissions, SSH keys, and first connection.
version: 2.4.0
---

# Setup O2

Set up remote access to Harvard O2 through `remote-bridge`. This subskill is for initial setup only. After the bridge is installed, configured, and connected, use the top-level `use-o2` skill for job submission, bridge commands, SLURM usage, monitoring, containers, and day-to-day troubleshooting.

## When This Applies

- User asks to set up O2, cluster access, SLURM access, or `remote-bridge`.
- A project is being initialized and expects compute-heavy O2 work.
- O2 access exists but the local bridge, permissions, SSH key, or first connection has not been configured.

## Setup Boundary

This subskill owns:

1. Installing or locating the `remote-bridge` CLI.
2. Creating `~/.config/remote-bridge/permissions.toml`.
3. Configuring allowed O2 read/write paths and Singularity defaults.
4. Creating or validating SSH key access to O2.
5. Starting the bridge and confirming `connection_status`.

Use `use-o2` for:

1. Listing, reading, searching, or downloading O2 files through the bridge.
2. Pulling git repos on O2.
3. Submitting jobs with `sandboxed_sbatch`.
4. Checking `squeue`, `sacct`, `job_wait`, `scancel`, or job efficiency.
5. Choosing partitions, resources, containers, or SLURM script templates.

## Step 1: Check Existing Bridge

```bash
which remote-bridge
remote-bridge --version
remote-bridge rpc o2 connection_status
```

If `connection_status` shows `"connected": true`, setup is complete. Switch to `use-o2` for actual work.

If the binary is missing, continue to installation. If the binary exists but the bridge is not running, continue to permissions and SSH validation before first connection.

## Step 2: Install Bridge

`remote-bridge` code and binaries are not bundled with this configuration. Before installing, identify the source of truth for the bridge in the local environment.

If a checked-out bridge repo with prebuilt binaries is available, install the matching binary:

```bash
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)
echo "Platform: $OS-$ARCH"
```

```bash
case "$OS-$ARCH" in
    darwin-arm64)  BINARY="remote-bridge-darwin-arm64" ;;
    darwin-x86_64) BINARY="remote-bridge-darwin-x86_64" ;;
    linux-x86_64)  BINARY="remote-bridge-linux-x86_64" ;;
    *) BINARY="" ;;
esac
```

If `$BINARY` is set and the binary source directory is known:

```bash
mkdir -p ~/.local/bin
cp "<bridge-source>/bin/$BINARY" ~/.local/bin/remote-bridge
chmod +x ~/.local/bin/remote-bridge
```

If no prebuilt binary is available, build from source only after confirming the source repo:

```bash
which cargo
cd "<bridge-source>/remote-bridge"
cargo build --release
mkdir -p ~/.local/bin
cp target/release/remote-bridge ~/.local/bin/remote-bridge
```

Ensure `~/.local/bin` is on `PATH` for the user's shell if needed.

## Step 3: Configure Permissions

Create the config directory and start from the bridge's example permissions file when available:

```bash
mkdir -p ~/.config/remote-bridge
cp "<bridge-source>/config/permissions.example.toml" ~/.config/remote-bridge/permissions.toml
```

Ask the user for their O2 paths:

1. Lab/project directory, usually under `/n/data1/...`.
2. Scratch directory, usually under `/n/scratch/users/...`.
3. Optional shared container directory or preferred `.sif` image path.

Edit `~/.config/remote-bridge/permissions.toml` so read/write paths match the project. At minimum, configure:

```toml
[paths]
read = [
  "/n/data1/hms/dbmi/.../project-or-lab",
  "/n/scratch/users/.../project"
]
write = [
  "/n/scratch/users/.../project"
]
```

If the project will use `sandboxed_sbatch`, also configure Singularity defaults:

```toml
[singularity]
default_image = "/n/app/containers/users/USERNAME/python-science.sif"
scripts_dir = "/n/scratch/users/u/USERNAME/agent-scripts/"
cache_dir = "/n/scratch/users/u/USERNAME/.singularity/cache"
extra_binds = [
  "/n/app:ro",
]
```

After editing permissions, update the checksum if the installed bridge requires it:

```bash
remote-bridge update-checksum
```

## Step 4: Configure SSH Key

SSH key authentication is required. The bridge reads SSH configuration from `~/.ssh/config`.

Ask whether the user already has an O2 SSH key.

If yes, identify the private key path and make sure it is referenced:

```sshconfig
Host o2.hms.harvard.edu
    IdentityFile ~/.ssh/o2_ed25519
```

If no, create a dedicated key:

```bash
ssh-keygen -t ed25519 -f ~/.ssh/o2_ed25519 -N "" -C "o2-cluster-access"
```

Add it to `~/.ssh/config`:

```sshconfig
Host o2.hms.harvard.edu
    IdentityFile ~/.ssh/o2_ed25519
```

Copy the key to O2:

```bash
ssh-copy-id -i ~/.ssh/o2_ed25519 YOUR_USERNAME@o2.hms.harvard.edu
```

Test key-based access:

```bash
ssh -o BatchMode=yes -o ConnectTimeout=10 YOUR_USERNAME@o2.hms.harvard.edu echo "SSH key working"
```

If the test prompts for a password or fails, retry key installation before starting the bridge.

## Step 5: First Connection

Ask the user for their O2 username, then start the bridge in a separate terminal:

```bash
remote-bridge start o2 --user YOUR_USERNAME
```

The user should complete Duo authentication and keep that terminal open.

Verify from the Claude Code terminal:

```bash
remote-bridge rpc o2 connection_status
```

If connected, setup is complete. Record useful project-specific paths in the project `CLAUDE.md`, for example:

```markdown
## O2 Paths

- O2 repo:
- O2 scratch:
- Default container:
```

Then use `use-o2` for bridge operations and job submission.

## Setup Troubleshooting

### `remote-bridge` Not Found

Confirm installation location and `PATH`:

```bash
ls -l ~/.local/bin/remote-bridge
echo "$PATH"
```

Add `~/.local/bin` to the user's shell startup file if needed.

### Bridge Not Running

Start it in a separate terminal:

```bash
remote-bridge start o2 --user YOUR_USERNAME
```

### SSH Key Fails

Check `~/.ssh/config`, key permissions, and O2 key installation:

```bash
ls -l ~/.ssh/o2_ed25519 ~/.ssh/o2_ed25519.pub
ssh -v YOUR_USERNAME@o2.hms.harvard.edu
```

### Permission Denied By Bridge

Edit `~/.config/remote-bridge/permissions.toml` to include the needed path, then run:

```bash
remote-bridge update-checksum
```
