#!/bin/bash
# Claude Code Notification Hook (macOS)
#
# Sends local notifications via terminal-notifier or osascript.
# Requires: brew install terminal-notifier
#
# Called by Claude Code hooks (Notification, Stop, etc.)

# Read hook input from stdin (JSON from Claude Code)
HOOK_INPUT=$(cat)

# Extract context from hook input
if command -v jq &> /dev/null; then
    CWD=$(echo "$HOOK_INPUT" | jq -r '.cwd // empty' 2>/dev/null)
else
    CWD=$(echo "$HOOK_INPUT" | grep -o '"cwd":"[^"]*"' | cut -d'"' -f4)
fi

# Get project name from working directory
if [ -n "$CWD" ]; then
    PROJECT_NAME=$(basename "$CWD")
else
    PROJECT_NAME=$(basename "$PWD")
fi

# Build title with project name
BASE_TITLE="${1:-Claude Code}"
if [ "$PROJECT_NAME" != "/" ] && [ -n "$PROJECT_NAME" ]; then
    TITLE="$BASE_TITLE: $PROJECT_NAME"
else
    TITLE="$BASE_TITLE"
fi

MESSAGE="${2:-Notification}"

# Send notification via terminal-notifier (preferred)
if command -v terminal-notifier &> /dev/null; then
    terminal-notifier \
        -title "$TITLE" \
        -message "$MESSAGE" \
        -sender com.anthropic.claudefordesktop \
        -sound Glass \
        > /dev/null 2>&1
# Fallback to osascript
elif [[ "$OSTYPE" == "darwin"* ]]; then
    osascript -e "display notification \"$MESSAGE\" with title \"$TITLE\" sound name \"Glass\"" \
        > /dev/null 2>&1
fi

exit 0
