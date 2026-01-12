#!/bin/bash
# Test hook to see what JSON input Claude Code provides
HOOK_INPUT=$(cat)
echo "=== HOOK INPUT ===" >> /tmp/claude/hook-test.log
echo "$HOOK_INPUT" | jq '.' >> /tmp/claude/hook-test.log 2>&1 || echo "$HOOK_INPUT" >> /tmp/claude/hook-test.log
echo "" >> /tmp/claude/hook-test.log
