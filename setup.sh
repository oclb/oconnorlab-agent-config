#!/bin/bash

# Claude Code Configuration Setup Script
# This script sets up Claude Code configuration for any machine
#
# What it does:
#   1. Creates ~/.claude directory and behavior.conf
#   2. Symlinks CLAUDE.md, skills, hooks, and settings.json to ~/.claude/

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

# Step 1b: Update CONFIG_REPO and Environment in behavior.conf
echo ""
echo "Step 1b: Recording config repo location and environment..."
if [ -f "$CLAUDE_DIR/behavior.conf" ]; then
    if grep -q "^CONFIG_REPO=" "$CLAUDE_DIR/behavior.conf"; then
        sed -i.bak "s|^CONFIG_REPO=.*|CONFIG_REPO=$REPO_DIR|" "$CLAUDE_DIR/behavior.conf"
        rm -f "$CLAUDE_DIR/behavior.conf.bak"
    else
        echo "CONFIG_REPO=$REPO_DIR" >> "$CLAUDE_DIR/behavior.conf"
    fi
    if grep -q "^Environment=" "$CLAUDE_DIR/behavior.conf"; then
        sed -i.bak "s|^Environment=.*|Environment=local|" "$CLAUDE_DIR/behavior.conf"
        rm -f "$CLAUDE_DIR/behavior.conf.bak"
    else
        echo "Environment=local" >> "$CLAUDE_DIR/behavior.conf"
    fi
    echo "  Set CONFIG_REPO=$REPO_DIR in behavior.conf"
    echo "  Set Environment=local in behavior.conf"
else
    cat > "$CLAUDE_DIR/behavior.conf" <<EOF
CONFIG_REPO=$REPO_DIR
Environment=local
EOF
    echo "  Created behavior.conf with CONFIG_REPO=$REPO_DIR"
    echo "  Set Environment=local"
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

# Step 2: Set up symlinks
echo ""
echo "Step 2: Setting up symlinks..."

setup_symlink "$REPO_DIR/global/CLAUDE.md" "$CLAUDE_DIR/CLAUDE.md" "CLAUDE.md"
setup_symlink "$REPO_DIR/global/settings.json" "$CLAUDE_DIR/settings.json" "settings.json"
setup_symlink "$REPO_DIR/skills" "$CLAUDE_DIR/skills" "skills"
setup_symlink "$REPO_DIR/hooks" "$CLAUDE_DIR/hooks" "hooks"

echo ""
echo "======================================"
echo "Setup complete!"
echo "======================================"
echo ""
echo "To verify, run:"
echo "  ls -la ~/.claude/"
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
