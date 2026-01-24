# Memory Reminder Hook Ignored

**Date:** 2026-01-20

## Issue

Claude received the `<reminder>Consider creating memory agent.</reminder>` hook notification multiple times but did not act on it. The reminder appeared in at least two consecutive messages before the user asked about it.

## Context

Work completed: Centralized feedback to `$CONFIG_REPO/feedback/` (todo #25). This was significant work that warranted a memory entry:
- Updated global/CLAUDE.md feedback instructions
- Updated project CLAUDE.md
- Moved 3 existing feedback files
- Committed changes to both main repo and notebook

## Expected Behavior

When the memory reminder hook fires, Claude should:
1. Evaluate whether the recent work warrants a memory entry
2. If yes, spawn the background memory agent immediately
3. If no, have a clear reason why (e.g., trivial single-line fix)

## Possible Causes

- Hook reminder is easy to overlook among other system-reminder tags
- No clear trigger in Claude's workflow to check for and act on hook reminders
- Memory creation felt "optional" rather than expected

## Suggested Improvements

1. Make hook reminders more prominent in instructions
2. Add explicit instruction: "When you see `<reminder>Consider creating memory agent.</reminder>`, immediately evaluate and act"
3. Consider whether the hook should be more assertive (e.g., "You MUST evaluate memory creation now")
