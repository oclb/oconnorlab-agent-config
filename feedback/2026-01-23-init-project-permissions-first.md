# Feedback: init-project must configure permissions first

**Date:** 2026-01-23

## Issue

The `/init-project` skill attempts to run commands like `git init` and `ls -la` before configuring `.claude/settings.json` permissions. This causes the user to be prompted for approval on basic operations that should be pre-authorized.

## Expected Behavior

The skill should:
1. First create `.claude/settings.json` with all necessary permissions for the init process
2. Then proceed with git init, directory listing, and other setup operations

## Fix Required

Update the `/init-project` skill to add a "Phase 0: Configure Permissions" step that creates `.claude/settings.json` before any other operations.
