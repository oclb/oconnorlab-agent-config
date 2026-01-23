# Feedback: Add safe gh commands to global permissions

**Date:** 2026-01-23

## Issue

Common safe commands like `which gh` and read-only `gh` commands (e.g., `gh auth status`, `gh repo list`) are not globally permitted. This causes unnecessary permission prompts during initialization.

## Expected Behavior

The global CLAUDE.md or a global settings file should pre-authorize:
- `which *` - checking for command availability
- `gh auth status` - checking authentication state
- `gh auth switch *` - switching between authenticated accounts
- `gh repo list *` - listing repos (read-only)
- Other read-only gh commands

## Suggested Addition to Global Permissions

```json
{
  "permissions": {
    "allow": [
      "Bash(which *)",
      "Bash(gh auth status)",
      "Bash(gh auth switch *)",
      "Bash(gh repo list *)"
    ]
  }
}
```
