#!/bin/bash

# Claude Code Configuration Setup Script
# This script creates symlinks from ~/.claude/ to your synced config files

set -e

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLAUDE_DIR="$HOME/.claude"

echo "Setting up Claude Code configuration..."
echo "Repository directory: $REPO_DIR"
echo "Claude directory: $CLAUDE_DIR"

# Create .claude directory if it doesn't exist
mkdir -p "$CLAUDE_DIR"

# Backup existing settings if they exist and aren't symlinks
if [ -f "$CLAUDE_DIR/settings.json" ] && [ ! -L "$CLAUDE_DIR/settings.json" ]; then
    echo "Backing up existing settings.json to settings.json.backup"
    mv "$CLAUDE_DIR/settings.json" "$CLAUDE_DIR/settings.json.backup"
fi

# Create symlink for settings.json
if [ -L "$CLAUDE_DIR/settings.json" ]; then
    echo "Removing existing symlink for settings.json"
    rm "$CLAUDE_DIR/settings.json"
fi

echo "Creating symlink: $CLAUDE_DIR/settings.json -> $REPO_DIR/settings.json"
ln -s "$REPO_DIR/settings.json" "$CLAUDE_DIR/settings.json"

echo ""
echo "Setup complete! Your Claude Code settings are now synced."
echo ""
echo "To verify, run: ls -la ~/.claude/settings.json"
