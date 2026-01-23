# Feedback: Prompt user to restart Claude after creating permissions

**Date:** 2026-01-23

## Issue

After `/init-project` creates `.claude/settings.json` with permissions, the permissions don't take effect until Claude is restarted. The user should be prompted to restart Claude after permissions are configured.

## Expected Behavior

After creating `.claude/settings.json`, the skill should:
1. Inform the user that permissions have been configured
2. Prompt them to restart Claude Code for the permissions to take effect
3. Instruct them to run `/init-project` again after restarting

## Suggested Message

> **Permissions configured!**
>
> Please restart Claude Code for the new permissions to take effect:
> 1. Exit Claude Code (`/exit` or Ctrl+C)
> 2. Restart Claude Code
> 3. Run `/init-project` again to continue setup
