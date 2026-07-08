---
name: documentation
description: Create or update developer-facing and agent-facing project documentation, or run module-scoped codebase audits. Invoke manually with /documentation for codebase maps, module documentation, codebase review/audit workflows, and README.md or CLAUDE.md work.
recommended_scope: global
disable-model-invocation: true
---

# Documentation

Invoke this skill manually with `/documentation`. Do not rely on automatic trigger behavior.

Choose the appropriate internal subskill for the documentation task, then read only that subskill's `SKILL.md` and supporting files:

- `subskills/map-modules/` when the task is to create or update a high-level codebase overview.
- `subskills/document-module/` when the task is to create or update focused documentation of a module or file.
- `subskills/module-codebase-review/` when the task is a codebase audit, codebase review, release check, paper-freeze check, or module-scoped multi-agent review.
- `subskills/maintain-project/` when the task is project hygiene: updating the project notebook or CLAUDE.md, checking TODOs, checking stale branches/PRs/stashes, or reviewing overall project state.

Documentation should maximize signal-to-noise ratio for the current developer. Reference the notebook to discern context for the user's request; for example, if a recent entry details a debugging session, then the user might want to better understand what logic gave rise to the bug or what safeguards and tests now prevent similar bugs.
