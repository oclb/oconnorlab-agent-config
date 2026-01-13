#!/bin/bash

# Claude Code Configuration Setup Script
# This script sets up Claude Code configuration for any machine
#
# What it does:
#   1. Creates ~/.claude directory and behavior.conf
#   2. Symlinks CLAUDE.md, skills, hooks, and settings.json to ~/.claude/
#   3. Sets up ntfy.sh notifications in shell config

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
    # Only add NewUser if it doesn't exist (preserve user's choice)
    if ! grep -q "^NewUser=" "$CLAUDE_DIR/behavior.conf"; then
        echo "NewUser=true" >> "$CLAUDE_DIR/behavior.conf"
        echo "  Set NewUser=true in behavior.conf (first-time setup)"
    fi
    echo "  Set CONFIG_REPO=$REPO_DIR in behavior.conf"
    echo "  Set Environment=local in behavior.conf"
else
    cat > "$CLAUDE_DIR/behavior.conf" <<EOF
CONFIG_REPO=$REPO_DIR
Environment=local
NewUser=true
EOF
    echo "  Created behavior.conf with CONFIG_REPO=$REPO_DIR"
    echo "  Set Environment=local"
    echo "  Set NewUser=true (first-time setup)"
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

# Step 3: Set up notifications
echo ""
echo "Step 3: Setting up notifications..."

# Add NTFY_TOPIC if not already present
if grep -q "export NTFY_TOPIC=" "$SHELL_CONFIG" 2>/dev/null; then
    echo "  NTFY_TOPIC already configured in $SHELL_CONFIG"
else
    echo "" >> "$SHELL_CONFIG"
    echo "# Claude Code: Notification topic for ntfy.sh" >> "$SHELL_CONFIG"
    echo "export NTFY_TOPIC=\"$(whoami)_claude_notifications\"" >> "$SHELL_CONFIG"
    echo "  Added NTFY_TOPIC to $SHELL_CONFIG"
fi

# Add source line for notification helpers if not already present
if grep -q "source.*notify-helpers.sh" "$SHELL_CONFIG" 2>/dev/null; then
    echo "  Notification helpers already sourced in $SHELL_CONFIG"
else
    echo "" >> "$SHELL_CONFIG"
    echo "# Claude Code: Notification helper functions" >> "$SHELL_CONFIG"
    echo "source \"$REPO_DIR/notify-helpers.sh\"" >> "$SHELL_CONFIG"
    echo "  Added notification helpers to $SHELL_CONFIG"
fi

echo ""
echo "======================================"
echo "Setup complete!"
echo "======================================"
echo ""
echo "IMPORTANT: Run 'source $SHELL_CONFIG' or start a new terminal session"
echo ""
echo "To receive notifications:"
echo "  1. Subscribe on your device:"
echo "     - Phone: Install ntfy app, subscribe to: $(whoami)_claude_notifications"
echo "     - Desktop: Visit https://ntfy.sh/$(whoami)_claude_notifications"
echo "  2. Test with: source $SHELL_CONFIG && test_notify"
echo ""
echo "To verify setup:"
echo "  ls -la ~/.claude/"
echo ""
