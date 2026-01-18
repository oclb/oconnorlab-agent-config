#!/bin/bash
# Notebook Entry Reminder Hook
#
# A Stop hook that reminds Claude to consider creating/updating notebook entries
# after substantive work. Triggers when:
#   - Response includes Edit, Write, or Bash tool calls, OR
#   - Response text is >200 words
#
# Output is injected as a system reminder to Claude.

# Read hook input from stdin
HOOK_INPUT=$(cat)

# Check if jq is available
if ! command -v jq &> /dev/null; then
    exit 0  # Can't parse without jq, skip silently
fi

# Extract the last assistant message from transcript
LAST_ASSISTANT=$(echo "$HOOK_INPUT" | jq -r '
    .transcript
    | map(select(.role == "assistant"))
    | last
    // empty
' 2>/dev/null)

if [ -z "$LAST_ASSISTANT" ] || [ "$LAST_ASSISTANT" = "null" ]; then
    exit 0
fi

# Check for Edit, Write, or Bash tool calls
TOOL_TRIGGER=$(echo "$LAST_ASSISTANT" | jq -r '
    .content[]?
    | select(.type == "tool_use")
    | .name
    | select(. == "Edit" or . == "Write" or . == "Bash")
' 2>/dev/null | head -1)

# Count words in text content
WORD_COUNT=$(echo "$LAST_ASSISTANT" | jq -r '
    [.content[]? | select(.type == "text") | .text]
    | join(" ")
' 2>/dev/null | wc -w | tr -d ' ')

# Trigger if tools used OR >200 words
if [ -n "$TOOL_TRIGGER" ] || [ "${WORD_COUNT:-0}" -gt 200 ]; then
    cat << 'EOF'
If this session has produced knowledge worth recalling (analysis, debugging, feature work, research), ensure a notebook entry exists or is updated.
EOF
fi

exit 0
