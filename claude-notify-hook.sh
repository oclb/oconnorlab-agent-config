#!/bin/bash
# Claude Code Notification Hook
# Works both locally (macOS) and on O2 cluster
#
# Usage: claude-notify-hook.sh "Title" "Message" [priority]
#
# This script is called by Claude Code hooks (Notification, Stop, etc.)
# It detects the environment and sends notifications appropriately:
#   - macOS: Uses terminal-notifier or osascript
#   - O2/Remote: Uses ntfy.sh
#   - Fallback: Prints to stderr

# Read hook input from stdin (JSON from Claude Code)
HOOK_INPUT=$(cat)

# Extract context from hook input
if command -v jq &> /dev/null; then
    CWD=$(echo "$HOOK_INPUT" | jq -r '.cwd // empty' 2>/dev/null)
else
    # Fallback without jq: simple grep/sed parsing
    CWD=$(echo "$HOOK_INPUT" | grep -o '"cwd":"[^"]*"' | cut -d'"' -f4)
fi

# Get conversation name from current working directory
if [ -n "$CWD" ]; then
    CONVERSATION_NAME=$(basename "$CWD")
else
    CONVERSATION_NAME=$(basename "$PWD")
fi

# Build title with conversation name
BASE_TITLE="${1:-Claude Code}"
if [ "$CONVERSATION_NAME" != "/" ] && [ -n "$CONVERSATION_NAME" ]; then
    TITLE="$BASE_TITLE: $CONVERSATION_NAME"
else
    TITLE="$BASE_TITLE"
fi

MESSAGE="${2:-Notification}"
PRIORITY="${3:-default}"

# Detect environment
is_o2() {
    [[ "$(hostname)" =~ ^(login|compute) ]] && [[ -d /n/data1 ]]
}

is_macos() {
    [[ "$OSTYPE" == "darwin"* ]]
}

# Function: Send notification via ntfy.sh (O2 or remote)
notify_ntfy() {
    if [ -z "$NTFY_TOPIC" ]; then
        return 1
    fi

    local ntfy_priority="$PRIORITY"
    local tags="desktop_computer,bell"

    # Map priority to ntfy levels
    case "$PRIORITY" in
        high|urgent) ntfy_priority="high"; tags="desktop_computer,warning" ;;
        low) ntfy_priority="low" ;;
        *) ntfy_priority="default" ;;
    esac

    curl -s \
        -H "Title: $TITLE" \
        -H "Priority: $ntfy_priority" \
        -H "Tags: $tags" \
        -d "$MESSAGE" \
        "${NTFY_SERVER:-https://ntfy.sh}/$NTFY_TOPIC" > /dev/null 2>&1

    return $?
}

# Function: Send notification via terminal-notifier (macOS)
notify_terminal_notifier() {
    if ! command -v terminal-notifier &> /dev/null; then
        return 1
    fi

    terminal-notifier \
        -title "$TITLE" \
        -message "$MESSAGE" \
        -sender com.anthropic.claudefordesktop \
        -sound Glass \
        > /dev/null 2>&1

    return $?
}

# Function: Send notification via osascript (macOS fallback)
notify_osascript() {
    osascript -e "display notification \"$MESSAGE\" with title \"$TITLE\" sound name \"Glass\"" \
        > /dev/null 2>&1

    return $?
}

# Function: Fallback notification (stderr)
notify_fallback() {
    echo "[$(date '+%H:%M:%S')] $TITLE: $MESSAGE" >&2
    return 0
}

# Main notification logic
send_notification() {
    # Try O2/remote ntfy.sh first if on O2 or NTFY_TOPIC is set
    if is_o2 || [ -n "$NTFY_TOPIC" ]; then
        if notify_ntfy; then
            return 0
        fi
    fi

    # Try macOS notifications
    if is_macos; then
        if notify_terminal_notifier; then
            return 0
        elif notify_osascript; then
            return 0
        fi
    fi

    # Fallback to stderr
    notify_fallback
    return 0
}

# Execute
send_notification

# Play sound on macOS (if available)
if is_macos; then
    case "$TITLE" in
        *"completed"*|*"finished"*|*"done"*|*"Stop"*)
            afplay /System/Library/Sounds/Hero.aiff 2>/dev/null &
            ;;
        *"input"*|*"Notification"*|*"waiting"*)
            afplay /System/Library/Sounds/Glass.aiff 2>/dev/null &
            ;;
    esac
fi

exit 0
