---
name: agents-md
description: Customize AGENTS.md file(s) for either the user or a project, including guidance on project, subdirectory, and user-level instruction scope.
---

# AGENTS.md

Use this subskill when changing durable agent instructions in `AGENTS.md` files.

## Project AGENTS.md

Codex can use one `AGENTS.md` per directory, including the project root. Agents working inside subdirectories automatically load and concatenate the relevant `AGENTS.md` files along that path.

Use project `AGENTS.md` files for stable context:

- The project's goal.
- Modules and their responsibilities.
- Key files, including code and artifacts like a paper draft.
- Project infrastructure: key dependencies, name of a CLI command, how to run tests.
- Project specific gotchas and user preferences.

Subdirectory `AGENTS.md` files should contain context specifically relevant to understanding or modifying that part of the codebase. For typical scientific programming projects, extensive directory-specific prompt structure is usually unnecessary.

Do not use `AGENTS.md` as technical documentation; either the documentation becomes stale when the code changes, or every code change incurs the overhead cost of updating the documentation file.

## User AGENTS.md

Global Codex instructions come from two source files:

- `~/.codex/user/AGENTS.md`: user-editable personal instructions.
- `<config-repo>/global/AGENTS.md`: shared instructions from this config repo.

These are rendered into `~/.codex/AGENTS.override.md`, which is the active generated file and should not be edited directly:

```bash
${CODEX_HOME:-$HOME/.codex}/bin/config-agent-tool render-override
```

During first-run setup, `${CODEX_HOME:-$HOME/.codex}/bin/config-agent-tool install --global` invokes the renderer with `--first-run`, which refuses to overwrite an existing unmanaged `AGENTS.override.md`.

Changes to the user's `~/.codex/user/AGENTS.md` are picked up by Codex only after rerunning the generation command and starting a new session.

The user-level AGENTS.md file should contain user preferences or context about the user that ought to be applied across all of their projects. Keep it short. It is prepended to every Codex session across all projects, so unnecessary content has a cumulative cost.
