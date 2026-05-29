#!/bin/bash
# Silently pull config repo updates on session start
# Runs via SessionStart hook - no output to avoid cluttering context

CONFIG_REPO="$(dirname "$(dirname "$(readlink -f ~/.claude/CLAUDE.md)")")"

# Quick pull with timeout (don't block startup if offline)
timeout 3 git -C "$CONFIG_REPO" pull --quiet 2>/dev/null

exit 0
