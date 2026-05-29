---
name: agents-md
description: Customize CLAUDE.md file(s) for either the user or a project, including guidance on project, subdirectory, and user-level instruction scope.
---

# CLAUDE.md

Use this subskill when changing durable agent instructions in `CLAUDE.md` files.

## Project CLAUDE.md

Claude Code can use one `CLAUDE.md` per directory, including the project root. Agents working inside subdirectories automatically load and concatenate the relevant `CLAUDE.md` files along that path.

Use project `CLAUDE.md` files for stable context:

- The project's goal.
- Modules and their responsibilities.
- Key files, including code and artifacts like a paper draft.
- Project infrastructure: key dependencies, name of a CLI command, how to run tests.
- Project specific gotchas and user preferences.

Subdirectory `CLAUDE.md` files should contain context specifically relevant to understanding or modifying that part of the codebase. For typical scientific programming projects, extensive directory-specific prompt structure is usually unnecessary.

Do not use `CLAUDE.md` as technical documentation; either the documentation becomes stale when the code changes, or every code change incurs the overhead cost of updating the documentation file.

## User CLAUDE.md

Claude Code loads the user-owned `~/.claude/CLAUDE.md`. The unified installer preserves that file and adds an `@<config-repo>/claude/global/CLAUDE.md` import line.

Use this command for first-run setup:

```bash
${CLAUDE_HOME:-$HOME/.claude}/bin/config-agent-tool install --agent claude
```

Changes to the shared global file are picked up by new Claude Code sessions through the import. Changes to the user's own `~/.claude/CLAUDE.md` should remain short and personal.
