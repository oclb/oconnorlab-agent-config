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
#   3. Generates settings.json from template
#   4. Symlinks settings.json to ~/.claude/

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

# Step 3: Create .claude directory
echo ""
echo "Step 3: Setting up Claude configuration directory..."
mkdir -p "$CLAUDE_DIR"
echo "  Created: $CLAUDE_DIR"

# Step 4: Generate settings.json from template
echo ""
echo "Step 4: Generating settings.json..."

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

# Step 5: Backup and symlink settings.json
echo ""
echo "Step 5: Linking settings.json..."
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
echo "  3. Start Claude Code and try: /learn-tool"
echo ""
