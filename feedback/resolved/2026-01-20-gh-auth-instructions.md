# GitHub CLI Authorization Instructions

**Date:** 2026-01-20

After installing `gh`, Claude should:

1. Source `.zshrc` if needed to make `gh` available in the current session
2. State clearly: "Follow instructions below to authorize the GitHub CLI:"
3. Run: `gh auth login --web --git-protocol https && gh auth setup-git`

This ensures the user knows they need to interact with the auth flow and sets up git integration in one step.
