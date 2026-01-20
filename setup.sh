#!/bin/bash

# Claude Code Configuration Setup Script
# This script sets up Claude Code configuration for any machine
#
# What it does:
#   1. Creates ~/.claude directory
#   2. Symlinks CLAUDE.md, skills, hooks, and settings.json to ~/.claude/
#   3. Sets up local notification dependencies

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

# Detect shell and config file
detect_shell_config() {
    # Check what shell is set as default
    local user_shell=$(basename "$SHELL")

    case "$user_shell" in
        zsh)
            if [ -f "$HOME/.zshrc" ]; then
                echo "$HOME/.zshrc"
            else
                # Create .zshrc if it doesn't exist
                touch "$HOME/.zshrc"
                echo "$HOME/.zshrc"
            fi
            ;;
        bash)
            if [ -f "$HOME/.bashrc" ]; then
                echo "$HOME/.bashrc"
            elif [ -f "$HOME/.bash_profile" ]; then
                echo "$HOME/.bash_profile"
            else
                touch "$HOME/.bashrc"
                echo "$HOME/.bashrc"
            fi
            ;;
        *)
            # Default to .bashrc for unknown shells
            if [ -f "$HOME/.bashrc" ]; then
                echo "$HOME/.bashrc"
            else
                echo "$HOME/.profile"
            fi
            ;;
    esac
}

SHELL_CONFIG=$(detect_shell_config)
echo "Detected shell config: $SHELL_CONFIG"
echo ""

# Step 1: Create .claude directory if it doesn't exist
echo "Step 1: Creating Claude configuration directory..."
mkdir -p "$CLAUDE_DIR"
echo "  Created: $CLAUDE_DIR"

# Clean up old behavior.conf if it exists (no longer used)
if [ -f "$CLAUDE_DIR/behavior.conf" ]; then
    echo "  Removing deprecated behavior.conf (no longer used)"
    rm -f "$CLAUDE_DIR/behavior.conf"
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

# Step 3: Check for terminal-notifier (notifications)
echo ""
echo "Step 3: Checking notification dependencies..."

if command -v terminal-notifier &> /dev/null; then
    echo "  terminal-notifier is installed"
else
    echo "  WARNING: terminal-notifier not found"
    echo "  Install with: brew install terminal-notifier"
fi

# Step 4: Create settings.local.json with user-specific permissions
echo ""
echo "Step 4: Creating local settings with user-specific permissions..."

LOCAL_SETTINGS="$CLAUDE_DIR/settings.local.json"

# Create settings.local.json with user-specific permissions
cat > "$LOCAL_SETTINGS" <<EOF
{
  "permissions": {
    "allow": [
      "Bash($REPO_DIR/o2-scripts/o2-run.sh:*)",
      "Bash($REPO_DIR/o2-scripts/connect-o2.sh:*)",
      "Bash($REPO_DIR/o2-scripts/o2-setup.sh:*)",
      "Read(//$REPO_DIR/feedback/**)",
      "Write(//$REPO_DIR/feedback/**)"
    ]
  }
}
EOF
echo "  Created $LOCAL_SETTINGS with O2 script and feedback permissions"

echo ""
echo "======================================"
echo "Setup complete!"
echo "======================================"
echo ""
echo "To verify setup:"
echo "  ls -la ~/.claude/"
echo ""
