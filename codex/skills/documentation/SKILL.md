---
name: documentation
description: Create or update developer-facing and agent-facing project documentation. Invoke manually with $documentation for codebase maps, module documentation, and README.md or AGENTS.md work.
recommended_scope: global
disable-model-invocation: true
---

# Documentation

Invoke this skill manually with `$documentation`. Do not rely on automatic trigger behavior.

Choose the appropriate internal subskill for the documentation task, then read only that subskill's `SKILL.md` and supporting files:

- `subskills/map-modules/` when the task is to create or update a high-level codebase overview.
- `subskills/document-module/` when the task is to create or update focused documentation of a module or file.
- `subskills/maintain-project/` when the task is to update the project notebook and AGENTS.md file.

Documentation should maximize signal-to-noise ratio for the current developer. Reference the notebook to discern context for the user's request; for example, if a recent entry details a debugging session, then the user might want to better understand what logic gave rise to the bug or what safeguards and tests now prevent similar bugs.
