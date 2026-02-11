#!/bin/bash

# Claude Code Configuration Setup Script
# This script sets up Claude Code configuration for any machine
#
# What it does:
#   1. Creates ~/.claude directory
#   2. Adds @import to user's CLAUDE.md (preserves user content)
#   3. Symlinks settings.json, skills, hooks (preserves existing settings)
#   4. Adds O2 permissions to settings.local.json
#   5. Sets up local notification dependencies

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
        echo "  Backing up existing $name to ${name}.backup"
        mv "$link" "${link}.backup"
    fi

    if [ -L "$link" ]; then
        rm "$link"
    fi

    echo "  Creating symlink: $link -> $target"
    ln -s "$target" "$link"
}

# Step 2: Set up CLAUDE.md with @import
echo ""
echo "Step 2: Setting up CLAUDE.md..."

CLAUDE_MD="$CLAUDE_DIR/CLAUDE.md"
IMPORT_LINE="@$REPO_DIR/global/CLAUDE.md"

# Check if import line already exists
if [ -f "$CLAUDE_MD" ] && grep -qF "$IMPORT_LINE" "$CLAUDE_MD" 2>/dev/null; then
    echo "  Import line already present in CLAUDE.md"
else
    # If CLAUDE.md exists and is a symlink to our repo, remove it (migration from old setup)
    if [ -L "$CLAUDE_MD" ]; then
        LINK_TARGET=$(readlink "$CLAUDE_MD")
        if [[ "$LINK_TARGET" == *"claude-config"* ]]; then
            echo "  Removing old symlink (migrating to @import)"
            rm "$CLAUDE_MD"
        fi
    fi

    # Create or append to CLAUDE.md
    if [ ! -f "$CLAUDE_MD" ]; then
        echo "  Creating CLAUDE.md with import"
        echo "# User Claude Configuration" > "$CLAUDE_MD"
        echo "" >> "$CLAUDE_MD"
        echo "# Import shared configuration from claude-config repo" >> "$CLAUDE_MD"
        echo "$IMPORT_LINE" >> "$CLAUDE_MD"
    else
        echo "  Adding import line to existing CLAUDE.md"
        echo "" >> "$CLAUDE_MD"
        echo "# Import shared configuration from claude-config repo" >> "$CLAUDE_MD"
        echo "$IMPORT_LINE" >> "$CLAUDE_MD"
    fi
fi

# Step 3: Set up settings.json (symlink, preserving existing as settings.local.json)
echo ""
echo "Step 3: Setting up settings.json..."

SETTINGS="$CLAUDE_DIR/settings.json"
LOCAL_SETTINGS="$CLAUDE_DIR/settings.local.json"

# If settings.json exists and is NOT a symlink, preserve it as settings.local.json
if [ -e "$SETTINGS" ] && [ ! -L "$SETTINGS" ]; then
    if [ -e "$LOCAL_SETTINGS" ]; then
        echo "  WARNING: Both settings.json and settings.local.json exist"
        echo "  Backing up settings.json to settings.json.backup"
        mv "$SETTINGS" "${SETTINGS}.backup"
    else
        echo "  Preserving existing settings.json as settings.local.json"
        mv "$SETTINGS" "$LOCAL_SETTINGS"
    fi
fi

# Now symlink settings.json to repo
setup_symlink "$REPO_DIR/global/settings.json" "$SETTINGS" "settings.json"

# Step 4: Set up skills and hooks symlinks
echo ""
echo "Step 4: Setting up skills and hooks..."

setup_symlink "$REPO_DIR/skills" "$CLAUDE_DIR/skills" "skills"
setup_symlink "$REPO_DIR/hooks" "$CLAUDE_DIR/hooks" "hooks"

# Step 5: Add O2 permissions to settings.local.json
echo ""
echo "Step 5: Adding O2 permissions to settings.local.json..."

# Define the O2 permissions we need
O2_PERMISSIONS=(
    "Bash($REPO_DIR/o2-scripts/o2-run.sh:*)"
    "Bash($REPO_DIR/o2-scripts/connect-o2.sh:*)"
    "Bash($REPO_DIR/o2-scripts/o2-setup.sh:*)"
    "Read(//$REPO_DIR/feedback/**)"
    "Write(//$REPO_DIR/feedback/**)"
    "Edit(//$REPO_DIR/feedback/**)"
    "Bash(git -C $REPO_DIR add feedback/*)"
    "Bash(git -C $REPO_DIR commit *)"
    "Bash(git -C $REPO_DIR push)"
    "Bash(git -C $REPO_DIR mv feedback/*)"
)

if command -v jq &> /dev/null; then
    # Use jq for proper JSON handling
    if [ -f "$LOCAL_SETTINGS" ]; then
        # Read existing permissions and merge
        EXISTING=$(cat "$LOCAL_SETTINGS")

        # Build the new permissions array, avoiding duplicates
        NEW_PERMS="$EXISTING"
        for perm in "${O2_PERMISSIONS[@]}"; do
            # Check if permission already exists
            if ! echo "$NEW_PERMS" | jq -e ".permissions.allow | index(\"$perm\")" > /dev/null 2>&1; then
                NEW_PERMS=$(echo "$NEW_PERMS" | jq ".permissions.allow += [\"$perm\"]")
            fi
        done
        echo "$NEW_PERMS" | jq '.' > "$LOCAL_SETTINGS"
        echo "  Merged O2 permissions into existing settings.local.json"
    else
        # Create new settings.local.json
        jq -n --argjson perms "$(printf '%s\n' "${O2_PERMISSIONS[@]}" | jq -R . | jq -s .)" \
            '{permissions: {allow: $perms}}' > "$LOCAL_SETTINGS"
        echo "  Created settings.local.json with O2 permissions"
    fi
else
    # Fallback: simple file creation (no merge capability without jq)
    if [ -f "$LOCAL_SETTINGS" ]; then
        echo "  WARNING: jq not installed, cannot merge permissions"
        echo "  Please manually add O2 permissions to $LOCAL_SETTINGS"
        echo "  Or install jq: brew install jq"
    else
        # Create basic settings.local.json without jq
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
        echo "  Created settings.local.json with O2 permissions"
    fi
fi

# Step 6: Check for terminal-notifier (notifications)
echo ""
echo "Step 6: Checking notification dependencies..."

if command -v terminal-notifier &> /dev/null; then
    echo "  terminal-notifier is installed"
else
    echo "  WARNING: terminal-notifier not found"
    echo "  Install with: brew install terminal-notifier"
fi

echo ""
echo "======================================"
echo "Setup complete!"
echo "======================================"
echo ""
echo "Configuration:"
echo "  CLAUDE.md:           User-owned with @import to repo"
echo "  settings.json:       Symlink to repo (auto-updates)"
echo "  settings.local.json: User-owned (your personal settings + O2 permissions)"
echo "  skills/:             Symlink to repo"
echo "  hooks/:              Symlink to repo"
echo ""
echo "Customization:"
echo "  - Edit ~/.claude/CLAUDE.md to add personal instructions (before or after the @import line)"
echo "  - Edit ~/.claude/settings.local.json for personal permissions and preferences"
echo "  - settings.local.json takes precedence over the shared settings.json"
echo ""
echo "To verify: ls -la ~/.claude/"
echo ""
