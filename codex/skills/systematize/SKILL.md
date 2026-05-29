---
name: systematize
description: Use when the user wants to systematize a workflow, encode desired or undesired agent behavior, create or edit a skill, or get support with their Codex setup.
recommended_scope: global
---

# Systematize

Select the appropriate subskill based on context:

- `skill-creator`: create a new skill or update an existing skill.
- `postmortem`: analyze something that did not go as expected and identify a system prompt or skill change to prevent recurrence; may be used with `skill-creator`.
- `agents-md`: customize the AGENTS.md file(s) for either the user or the project.
- `support`: answer or diagnose a technical issue or question related to this configuration or the user's local Codex setup.
- For Codex support specifically, use the Codex-provided `openai-docs` skill.

Available levers:

- Project-specific `AGENTS.md`, including subdirectory-specific `AGENTS.md` files in the project repo.
- User-specific `AGENTS.md`.
- This configuration repo's content `AGENTS.md`; modify it only for changes that should propagate to other users of this repo.
- Skills at any of those three scopes, either by editing existing skills or creating new ones.
