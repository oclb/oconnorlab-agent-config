# Add Permissions for Config Repo Feedback Folder

**Date:** 2026-01-21
**Component:** global/settings.json
**Type:** Enhancement

## Issue

When asked to log feedback to the claude config repo (`$CONFIG_REPO/feedback/`), Claude has to request permission for each file operation. This adds friction to the feedback workflow.

## Current State

Global settings.json only has permissions for:
- `~/.claude/behavior.conf`
- `remote-bridge` commands
- A few WebFetch domains

## Suggested Permissions

Add to global/settings.json:
```json
"Read($CONFIG_REPO/feedback/**)",
"Write($CONFIG_REPO/feedback/**)",
"Edit($CONFIG_REPO/feedback/**)",
"Bash(git -C $CONFIG_REPO add feedback/*)",
"Bash(git -C $CONFIG_REPO commit *)"
```

Or more broadly for the whole config repo:
```json
"Read($CONFIG_REPO/**)",
"Edit($CONFIG_REPO/**)",
"Write($CONFIG_REPO/**)",
"Bash(git -C $CONFIG_REPO *)"
```

## Note

The `$CONFIG_REPO` variable would need to be expanded to the actual path (`/Users/loconnor/Documents/claude`) since Claude Code settings may not support variable expansion.
