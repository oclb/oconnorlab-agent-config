#!/bin/bash
# Notebook Entry Reminder Hook
#
# A Stop hook that enforces memory decisions after substantive work.
#
# Triggers when response includes Edit, Write, or Bash tool calls, OR text >200 words.
# When triggered, requires the response to end with a JSON blob: {"memory_created": true/false}
# Blocks stopping if the blob is missing.

# Check for memory decision file FIRST (before any other processing)
# This breaks the loop where signaling the decision triggers another check
DECISION_FILE="/tmp/claude/memory-decision"
if [ -f "$DECISION_FILE" ]; then
    rm -f "$DECISION_FILE"
    exit 0
fi

# Read hook input from stdin
HOOK_INPUT=$(cat)

# Check if jq is available
if ! command -v jq &> /dev/null; then
    exit 0  # Can't parse without jq, skip silently
fi

# Extract transcript_path from input (Stop hook provides path, not inline transcript)
TRANSCRIPT_PATH=$(echo "$HOOK_INPUT" | jq -r '.transcript_path // empty' 2>/dev/null)

if [ -z "$TRANSCRIPT_PATH" ] || [ ! -f "$TRANSCRIPT_PATH" ]; then
    exit 0
fi

# Read recent assistant messages from the transcript file (JSONL format)
# Note: Claude's responses are split across multiple assistant entries (thinking, tool_use, text)
# so we need to check several recent entries, not just the last one
RECENT_ASSISTANTS=$(tail -50 "$TRANSCRIPT_PATH" | jq -s '[.[] | select(.type == "assistant")]' 2>/dev/null)

if [ -z "$RECENT_ASSISTANTS" ] || [ "$RECENT_ASSISTANTS" = "null" ] || [ "$RECENT_ASSISTANTS" = "[]" ]; then
    exit 0
fi

# Check for Edit, Write, or Bash tool calls in any recent assistant message
# Use printf to preserve JSON (echo mangles escape sequences)
TOOL_TRIGGER=$(printf '%s' "$RECENT_ASSISTANTS" | jq -r '
    .[]
    | .message.content[]?
    | select(.type == "tool_use")
    | .name
    | select(. == "Edit" or . == "Write" or . == "Bash")
' 2>/dev/null | head -1)

# Count words in text content from all recent assistant messages
WORD_COUNT=$(printf '%s' "$RECENT_ASSISTANTS" | jq -r '
    [.[] | .message.content[]? | select(.type == "text") | .text]
    | join(" ")
' 2>/dev/null | wc -w | tr -d ' ')

# Only trigger on substantive work
if [ -z "$TOOL_TRIGGER" ] && [ "${WORD_COUNT:-0}" -le 200 ]; then
    exit 0
fi

# Missing decision - block and instruct via stderr + exit code 2
cat >&2 << 'EOF'
After substantive work, decide whether this session produced knowledge worth preserving.
Signal your decision by running: echo "true" > /tmp/claude/memory-decision (if entry created/updated)
                              or: echo "false" > /tmp/claude/memory-decision (if no entry needed)
EOF

exit 2
