# Permit Local Git Commands by Default

**Date:** 2026-01-20

Permissions should allow all git commands that don't alter the remote repository. This includes:
- `git diff`
- `git status`
- `git log`
- `git add`
- `git commit`
- `git branch`
- `git checkout`
- `git stash`

Only commands that affect the remote (e.g., `git push`, `git push --force`) should require explicit permission.
