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
#   4. Sets up notification system (ntfy)
#   5. Creates ~/.claude directory and behavior.conf
#   6. Symlinks CLAUDE.md, skills, hooks, and settings.json to ~/.claude/

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

# Step 4: Set up notification system
echo ""
echo "Step 4: Setting up notification system..."

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

echo ""
echo "  To receive notifications, subscribe on your device(s):"
echo "    - Phone: Install ntfy app, subscribe to: $(whoami)_o2_notifications"
echo "    - Desktop: Visit https://ntfy.sh/$(whoami)_o2_notifications"
echo "    - Test with: source ~/.bashrc && test_notify"

# Step 5: Create .claude directory and behavior.conf
echo ""
echo "Step 5: Setting up Claude configuration directory..."
mkdir -p "$CLAUDE_DIR"
echo "  Created: $CLAUDE_DIR"

echo ""
echo "Step 5b: Recording config repo location and environment..."
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

# Helper function to create symlink with backup
setup_symlink() {
    local target="$1"
    local link="$2"
    local name="$3"

    if [ -e "$link" ] && [ ! -L "$link" ]; then
        echo "  Backing up existing $name"
        mv "$link" "${link}.backup"
    fi

    if [ -L "$link" ]; then
        rm "$link"
    fi

    echo "  Creating symlink: $link -> $target"
    ln -s "$target" "$link"
}

# Step 6: Set up symlinks
echo ""
echo "Step 6: Setting up symlinks..."

setup_symlink "$REPO_DIR/global/CLAUDE.md" "$CLAUDE_DIR/CLAUDE.md" "CLAUDE.md"
setup_symlink "$REPO_DIR/global/settings.json" "$CLAUDE_DIR/settings.json" "settings.json"
setup_symlink "$REPO_DIR/skills" "$CLAUDE_DIR/skills" "skills"
setup_symlink "$REPO_DIR/hooks" "$CLAUDE_DIR/hooks" "hooks"

echo ""
echo "======================================"
echo "Setup complete!"
echo "======================================"
echo ""
echo "IMPORTANT: Run 'source ~/.bashrc' or start a new terminal session"
echo "before using Claude Code."
echo ""
echo "Optional: Create user-specific configuration"
echo "  To add personal instructions that won't be committed:"
echo "    cp $REPO_DIR/global/CLAUDE.user.md.example $REPO_DIR/global/CLAUDE.user.md"
echo "    vim $REPO_DIR/global/CLAUDE.user.md"
echo ""
echo "  See $REPO_DIR/USER-CONFIG.md for details."
echo ""
echo "To verify setup:"
echo "  1. Run: echo \$TMPDIR"
echo "     Should show: $SCRATCH_BASE"
echo "  2. Run: ls -la ~/.claude/"
echo "     Should show symlinks to this repo"
echo ""
