---
name: init-project
description: Initialize a project workspace for Claude Code with project instructions, repository checks, notebook scaffolding, optional remotes, and optional O2/SLURM setup. Invoke manually with /init-project for first-time project setup, notebook bootstrap, project remotes, or O2/SLURM setup.
recommended_scope: global
disable-model-invocation: true
---

# Init Project

Invoke this skill manually with `/init-project`. Do not rely on automatic trigger behavior.

Initialize a project for Claude Code-assisted research work.

Use this skill for normal project setup. Do not use it to install Claude Code's user-level configuration.

## What This Sets Up

- `CLAUDE.md` for project-local Claude Code context.
- `notebook/` as a separate git repo for durable research memory.
- `.gitignore` entries so notebook history stays out of the main repo.
- Optional project-scope skills under `.claude/skills/`.
- Optional GitHub remotes for the main project and notebook.
- Optional O2/SLURM setup guidance via `subskills/setup-o2/`.

Do not create `.claude/` files or `CLAUDE.md` unless the user explicitly asks for Claude compatibility.

## Preflight

Inspect the project before editing:

```bash
pwd
git rev-parse --show-toplevel 2>/dev/null || true
git status --short --branch 2>/dev/null || true
git remote -v 2>/dev/null || true
test -f CLAUDE.md && sed -n '1,220p' CLAUDE.md
test -f README.md && sed -n '1,180p' README.md
test -f .gitignore && sed -n '1,220p' .gitignore
```

If the current directory is not a git repository, ask whether to run `git init` unless the user already made clear that this directory is the project root.

## Main Repository Remote

If the main repository has no remote:

1. Check `gh`:

   ```bash
   which gh
   gh auth status
   ```

2. If `gh` is missing, install it when the local package manager is obvious and the environment allows it; otherwise give concise install instructions.
3. If `gh` is not authenticated, run `gh auth login --web --git-protocol https` and let the user complete the browser flow.
4. Ask for the GitHub owner and visibility before creating a remote.
5. Create the repo with the current directory name by default:

   ```bash
   gh repo create <owner>/<repo-name> --private --source=. --remote=origin
   ```

Do not push unless the user explicitly asks or has already requested remote backup.

## Notebook Setup

Create the notebook structure if missing:

```bash
mkdir -p notebook/{entries,feedback,plans}
test -d notebook/.git || git -C notebook init
```

Create missing template files:

```markdown
# Notebook Index

| Date | Name | Summary |
|------|------|---------|
```

```markdown
# To-Do
```

```markdown
# Completed
```

Commit notebook initialization when there are notebook changes:

```bash
git -C notebook add -A
git -C notebook commit -m "Initialize notebook"
```

If the user wants notebook backup, create a private notebook remote. Use the same owner as the main repo unless the user chooses otherwise:

```bash
gh repo create <owner>/<repo-name>-notebook --private --source=notebook --remote=origin
```

Push only when the user approves backup/sync.

## Main `.gitignore`

Ensure the main repo ignores the notebook:

```gitignore
# Claude Code project notebook (separate git repo)
notebook/
```

Do not automatically ignore `CLAUDE.md`; project instructions are usually intended to be shared with collaborators. If the user wants private project instructions, recommend a private overlay or notebook entry instead.

Commit `.gitignore` only if the user wants setup changes committed.

## Project `CLAUDE.md`

If `CLAUDE.md` exists, update it conservatively. If it does not exist, create a compact starter:

```markdown
# Project Instructions

## Project Overview

Describe the project goal, key datasets, and expected outputs here.

## Commands

- Test:
- Lint:
- Run:

## Module Boundaries

No authoritative module boundaries have been defined yet. If Claude Code later infers boundaries from the code, review them and replace this placeholder with the accepted decomposition.

## Notebook

This project uses `notebook/` as a separate git repository for plans, analysis logs, and durable research memory. Start with `notebook/INDEX.md` when resuming prior work.
```

Preserve any existing project-specific instructions. Never overwrite a non-empty `CLAUDE.md` without showing the proposed change.

## Optional Project Skills

Project-scope skills are optional. Run `${CLAUDE_HOME:-$HOME/.claude}/bin/config-agent-tool list-skills --agent claude` from the project root. It prints a compact table of repo skills that are not already installed globally, including each skill's recommended scope, current project install state, and frontmatter description.

Ask the user which skills, if any, they want installed for this project. For standard project setup, recommend the skills whose `recommended_scope` is `project`. If the user chooses skills, install them:

```bash
${CLAUDE_HOME:-$HOME/.claude}/bin/config-agent-tool link-skills --agent claude --add <chosen-skill-names>
```

If the user wants to remove project-local skills, use:

```bash
${CLAUDE_HOME:-$HOME/.claude}/bin/config-agent-tool link-skills --agent claude --remove <skill-names>
```

## Optional O2 Setup

If the user expects compute-heavy work, asks to configure O2, or mentions O2/SLURM setup, read `subskills/setup-o2/SKILL.md` after local project setup and follow that workflow. Do not make O2 setup a default requirement.

## Completion Message

End with:

- what was created or updated
- whether notebook backup is local-only or remote-backed
- whether the main repo has a remote
- which project-scope skills were installed, if any
- whether a Claude Code restart is needed
- two next useful prompts based on the project

Keep the tour short. A new user should leave knowing they can ask for analysis, data inspection, project maintenance, or resume help.
