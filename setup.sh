#!/bin/bash

# Claude Code Configuration Setup Script
# This script sets up Claude Code configuration for any machine
#
# What it does:
#   1. Creates ~/.claude directory
#   2. Generates settings.json from template with correct paths
#   3. Symlinks settings.json to ~/.claude/

set -e

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLAUDE_DIR="$HOME/.claude"

echo "======================================"
echo "Claude Code Setup"
echo "======================================"
echo ""
echo "Repository directory: $REPO_DIR"
echo "Claude directory: $CLAUDE_DIR"
echo ""

# Detect OS
OS="$(uname -s)"
echo "Detected OS: $OS"
echo ""

# Step 1: Create .claude directory if it doesn't exist
echo "Step 1: Creating Claude configuration directory..."
mkdir -p "$CLAUDE_DIR"
echo "  Created: $CLAUDE_DIR"

# Step 1b: Update CONFIG_REPO in behavior.conf
echo ""
echo "Step 1b: Recording config repo location..."
if [ -f "$CLAUDE_DIR/behavior.conf" ]; then
    if grep -q "^CONFIG_REPO=" "$CLAUDE_DIR/behavior.conf"; then
        sed -i.bak "s|^CONFIG_REPO=.*|CONFIG_REPO=$REPO_DIR|" "$CLAUDE_DIR/behavior.conf"
        rm -f "$CLAUDE_DIR/behavior.conf.bak"
    else
        echo "CONFIG_REPO=$REPO_DIR" >> "$CLAUDE_DIR/behavior.conf"
    fi
    echo "  Set CONFIG_REPO=$REPO_DIR in behavior.conf"
else
    echo "CONFIG_REPO=$REPO_DIR" > "$CLAUDE_DIR/behavior.conf"
    echo "  Created behavior.conf with CONFIG_REPO=$REPO_DIR"
fi

# Step 2: Generate settings.json from template
echo ""
echo "Step 2: Generating settings.json..."

if [ -f "$REPO_DIR/settings.template.json" ]; then
    # Use template if it exists
    sed "s|__REPO_DIR__|$REPO_DIR|g" "$REPO_DIR/settings.template.json" > "$REPO_DIR/settings.local.json"
    SETTINGS_FILE="$REPO_DIR/settings.local.json"
    echo "  Generated from template: $SETTINGS_FILE"
else
    # Fall back to existing settings.json
    SETTINGS_FILE="$REPO_DIR/settings.json"
    echo "  Using existing: $SETTINGS_FILE"
fi

# Step 3: Add OS-specific notification hook
if [ "$OS" = "Darwin" ] && command -v terminal-notifier &> /dev/null; then
    echo "  macOS detected with terminal-notifier - notifications enabled"
    # Could add notification hook here if using template
elif [ "$OS" = "Linux" ]; then
    echo "  Linux detected - notifications disabled (no terminal-notifier)"
fi

# Step 4: Set up skills symlink
echo ""
echo "Step 3: Setting up skills symlink..."
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

# Step 5: Backup existing settings if they exist and aren't symlinks
echo ""
echo "Step 4: Setting up settings symlink..."
if [ -f "$CLAUDE_DIR/settings.json" ] && [ ! -L "$CLAUDE_DIR/settings.json" ]; then
    echo "  Backing up existing settings.json to settings.json.backup"
    mv "$CLAUDE_DIR/settings.json" "$CLAUDE_DIR/settings.json.backup"
fi

# Remove existing symlink
if [ -L "$CLAUDE_DIR/settings.json" ]; then
    echo "  Removing existing symlink"
    rm "$CLAUDE_DIR/settings.json"
fi

# Create symlink
echo "  Creating symlink: $CLAUDE_DIR/settings.json -> $SETTINGS_FILE"
ln -s "$SETTINGS_FILE" "$CLAUDE_DIR/settings.json"

echo ""
echo "======================================"
echo "Setup complete!"
echo "======================================"
echo ""
echo "To verify, run:"
echo "  ls -la ~/.claude/settings.json"
echo "  ls -la ~/.claude/skills"
echo ""

# OS-specific notes
if [ "$OS" = "Darwin" ]; then
    echo "macOS Notes:"
    echo "  - Install terminal-notifier for notifications: brew install terminal-notifier"
    echo ""
elif [ "$OS" = "Linux" ]; then
    echo "Linux Notes:"
    echo "  - If on O2 cluster, run setup-o2.sh instead for TMPDIR configuration"
    echo ""
fi
