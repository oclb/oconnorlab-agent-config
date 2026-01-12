#!/bin/bash

# Claude Code Configuration Setup Script for O2 Cluster
# This script sets up Claude Code on the Harvard O2 HPC cluster
#
# Prerequisites:
#   - Must be run from a login node (not a compute node)
#   - Claude Code must be installed
#
# What it does:
#   1. Creates scratch directory for TMPDIR
#   2. Configures TMPDIR in .bashrc
#   3. Installs sandbox dependencies (socat) via conda
#   4. Creates ~/.claude directory
#   5. Symlinks skills directory to ~/.claude/
#   6. Generates settings.json from template
#   7. Symlinks settings.json to ~/.claude/

set -e

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLAUDE_DIR="$HOME/.claude"
SCRATCH_BASE="/n/scratch/users/${USER:0:1}/$USER"

echo "======================================"
echo "Claude Code O2 Cluster Setup"
echo "======================================"
echo ""
echo "Repository directory: $REPO_DIR"
echo "Claude directory: $CLAUDE_DIR"
echo "Scratch directory: $SCRATCH_BASE"
echo ""

# Step 1: Create scratch directory for TMPDIR
echo "Step 1: Setting up scratch directory..."
if [ -d "$SCRATCH_BASE" ]; then
    echo "  Scratch directory already exists: $SCRATCH_BASE"
else
    echo "  Creating scratch directory..."
    if [ -x /n/cluster/bin/scratch_create_directory.sh ]; then
        /n/cluster/bin/scratch_create_directory.sh
        echo "  Scratch directory created: $SCRATCH_BASE"
    else
        echo "  ERROR: Cannot find scratch_create_directory.sh"
        echo "  Please run this from an O2 login node."
        exit 1
    fi
fi

# Step 2: Add TMPDIR to .bashrc if not already present
echo ""
echo "Step 2: Configuring TMPDIR in .bashrc..."
if grep -q "export TMPDIR=/n/scratch" "$HOME/.bashrc" 2>/dev/null; then
    echo "  TMPDIR already configured in .bashrc"
else
    echo "" >> "$HOME/.bashrc"
    echo "# Claude Code: Set TMPDIR to scratch space (required for O2)" >> "$HOME/.bashrc"
    echo "export TMPDIR=$SCRATCH_BASE" >> "$HOME/.bashrc"
    echo "  Added TMPDIR to .bashrc"
fi

# Export for current session
export TMPDIR="$SCRATCH_BASE"
echo "  TMPDIR set to: $TMPDIR"

# Step 3: Install sandbox dependencies (socat) via conda
echo ""
echo "Step 3: Installing sandbox dependencies..."
CONDA_ENV="$HOME/.conda/envs/claude-sandbox"

if [ -f "$CONDA_ENV/bin/socat" ]; then
    echo "  Sandbox dependencies already installed: $CONDA_ENV"
else
    echo "  Loading conda module..."
    module load conda/miniforge3/24.11.3-0

    echo "  Creating conda environment with socat..."
    conda create -y -p "$CONDA_ENV" -c conda-forge socat > /dev/null 2>&1
    echo "  Installed socat to: $CONDA_ENV"
fi

# Add conda env to PATH in .bashrc if not already present
if grep -q "claude-sandbox/bin" "$HOME/.bashrc" 2>/dev/null; then
    echo "  Sandbox PATH already configured in .bashrc"
else
    echo "" >> "$HOME/.bashrc"
    echo "# Claude Code: Add sandbox tools (socat) to PATH" >> "$HOME/.bashrc"
    echo 'export PATH="$HOME/.conda/envs/claude-sandbox/bin:$PATH"' >> "$HOME/.bashrc"
    echo "  Added sandbox tools to PATH in .bashrc"
fi

# Export for current session
export PATH="$CONDA_ENV/bin:$PATH"
echo "  Sandbox tools available: $(which socat 2>/dev/null || echo 'not found')"

# Step 3b: Set up notification system
echo ""
echo "Step 3b: Setting up notification system..."

# Check and add NTFY_TOPIC
if grep -q "export NTFY_TOPIC=" "$HOME/.bashrc" 2>/dev/null; then
    echo "  NTFY_TOPIC already configured in .bashrc"
else
    echo "" >> "$HOME/.bashrc"
    echo "# Claude Code: Notification topic for O2 job notifications" >> "$HOME/.bashrc"
    echo "export NTFY_TOPIC=\"$(whoami)_o2_notifications\"" >> "$HOME/.bashrc"
    echo "  Added NTFY_TOPIC to .bashrc"
fi

# Check and add source line
if grep -q "source.*o2-notify.sh" "$HOME/.bashrc" 2>/dev/null; then
    echo "  Notification script already sourced in .bashrc"
else
    echo "" >> "$HOME/.bashrc"
    echo "# Claude Code: Notification system for O2 jobs" >> "$HOME/.bashrc"
    echo "source $REPO_DIR/o2-notify.sh" >> "$HOME/.bashrc"
    echo "  Added notification system to .bashrc"
fi

# Show subscription instructions
echo ""
echo "  To receive notifications, subscribe on your device(s):"
echo "    • Phone: Install ntfy app, subscribe to: $(whoami)_o2_notifications"
echo "    • Desktop: Visit https://ntfy.sh/$(whoami)_o2_notifications"
echo "    • Test with: source ~/.bashrc && test_notify"

# Step 4: Create .claude directory
echo ""
echo "Step 4: Setting up Claude configuration directory..."
mkdir -p "$CLAUDE_DIR"
echo "  Created: $CLAUDE_DIR"

# Step 4b: Update CONFIG_REPO and Environment in behavior.conf
echo ""
echo "Step 4b: Recording config repo location and environment..."
if [ -f "$CLAUDE_DIR/behavior.conf" ]; then
    if grep -q "^CONFIG_REPO=" "$CLAUDE_DIR/behavior.conf"; then
        sed -i "s|^CONFIG_REPO=.*|CONFIG_REPO=$REPO_DIR|" "$CLAUDE_DIR/behavior.conf"
    else
        echo "CONFIG_REPO=$REPO_DIR" >> "$CLAUDE_DIR/behavior.conf"
    fi
    if grep -q "^Environment=" "$CLAUDE_DIR/behavior.conf"; then
        sed -i "s|^Environment=.*|Environment=O2|" "$CLAUDE_DIR/behavior.conf"
    else
        echo "Environment=O2" >> "$CLAUDE_DIR/behavior.conf"
    fi
    echo "  Set CONFIG_REPO=$REPO_DIR in behavior.conf"
    echo "  Set Environment=O2 in behavior.conf"
else
    cat > "$CLAUDE_DIR/behavior.conf" <<EOF
CONFIG_REPO=$REPO_DIR
Environment=O2
EOF
    echo "  Created behavior.conf with CONFIG_REPO=$REPO_DIR"
    echo "  Set Environment=O2"
fi

# Step 5: Set up skills symlink
echo ""
echo "Step 5: Setting up skills symlink..."
if [ -d "$CLAUDE_DIR/skills" ] && [ ! -L "$CLAUDE_DIR/skills" ]; then
    echo "  Backing up existing skills directory to skills.backup"
    mv "$CLAUDE_DIR/skills" "$CLAUDE_DIR/skills.backup"
fi

if [ -L "$CLAUDE_DIR/skills" ]; then
    echo "  Removing existing skills symlink"
    rm "$CLAUDE_DIR/skills"
fi

echo "  Creating symlink: $CLAUDE_DIR/skills -> $REPO_DIR/skills"
ln -s "$REPO_DIR/skills" "$CLAUDE_DIR/skills"

# Step 6: Generate settings.json from template
echo ""
echo "Step 6: Generating settings.json..."

if [ -f "$REPO_DIR/settings.template.json" ]; then
    # Use template - substitute __REPO_DIR__ with actual path
    sed "s|__REPO_DIR__|$REPO_DIR|g" "$REPO_DIR/settings.template.json" > "$REPO_DIR/settings.local.json"
    SETTINGS_FILE="$REPO_DIR/settings.local.json"
    echo "  Generated from template: $SETTINGS_FILE"
else
    # Fall back to existing settings.json
    SETTINGS_FILE="$REPO_DIR/settings.json"
    echo "  Using existing: $SETTINGS_FILE"
    echo "  WARNING: settings.json may contain hardcoded paths"
fi

# Step 7: Backup and symlink settings.json
echo ""
echo "Step 7: Linking settings.json..."
if [ -f "$CLAUDE_DIR/settings.json" ] && [ ! -L "$CLAUDE_DIR/settings.json" ]; then
    echo "  Backing up existing settings.json to settings.json.backup"
    mv "$CLAUDE_DIR/settings.json" "$CLAUDE_DIR/settings.json.backup"
fi

if [ -L "$CLAUDE_DIR/settings.json" ]; then
    echo "  Removing existing symlink"
    rm "$CLAUDE_DIR/settings.json"
fi

echo "  Creating symlink: $CLAUDE_DIR/settings.json -> $SETTINGS_FILE"
ln -s "$SETTINGS_FILE" "$CLAUDE_DIR/settings.json"

echo ""
echo "======================================"
echo "Setup complete!"
echo "======================================"
echo ""
echo "IMPORTANT: Run 'source ~/.bashrc' or start a new terminal session"
echo "before using Claude Code."
echo ""
echo "To verify setup:"
echo "  1. Run: echo \$TMPDIR"
echo "     Should show: $SCRATCH_BASE"
echo "  2. Run: ls -la ~/.claude/settings.json"
echo "     Should show symlink to this repo"
echo "  3. Run: ls -la ~/.claude/skills"
echo "     Should show symlink to this repo"
echo "  4. Start Claude Code and try: /learn-tool"
echo ""
echo "To enable notifications:"
echo "  1. Add to ~/.bashrc:"
echo "       export NTFY_TOPIC=\"$(whoami)_o2_notifications\""
echo "  2. Subscribe on your device:"
echo "       Phone: Install ntfy app from App/Play Store"
echo "       Desktop: Visit https://ntfy.sh/$(whoami)_o2_notifications"
echo "  3. Test with: test_notify"
echo "  4. Use in jobs: notify 'Job done!'"
echo ""
