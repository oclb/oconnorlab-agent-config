# /init-project

Initialize a project for use with Claude Code and the notebook system.

## When to Use

- First time using Claude Code with a project
- Replaces the built-in `/init` command with a more comprehensive setup

## What It Does

1. **Ensures main repo has a GitHub remote** - Guides user through setup if needed
2. **Creates notebook structure** - `notebook/{analyses,methods,feedback}/`
3. **Initializes notebook as separate git repo** - Keeps main repo clean
4. **Offers notebook remote** - Recommended for backup/sync
5. **Configures permissions** - Creates `.claude/settings.json`
6. **Creates CLAUDE.md** - Project context template

## Why Separate Notebook Repo?

The notebook grows indefinitely as you run analyses. Keeping it in a separate repo:
- Keeps main repo git log clean (only code changes)
- Allows different backup/sharing rules for notebook vs code
- Makes it easy to share code without sharing exploratory work
- Prevents repo bloat from analysis outputs

## Prerequisites

- **GitHub CLI (`gh`)** - Used to create repos easily
  - Install: `brew install gh` (macOS) or `sudo apt install gh` (Linux)
  - Auth: `gh auth login`

## Example Session

```
> /init-project

Your project doesn't have a GitHub remote yet. To continue, I'll help you create one.

What GitHub account should I use? myusername

Creating github.com/myusername/my-project...

Setting up project for Claude Code

I'll now:
1. Create notebook/ directory structure (as a separate git repo)
2. Add notebook/ to .gitignore
3. Create .claude/settings.json with notebook permissions
4. Create CLAUDE.md with project template

Notebook backup (recommended)

Create a GitHub remote for the notebook?
> Yes, create remote (recommended)

Creating github.com/myusername/my-project-notebook...

Project initialized!

- Notebook: notebook/ (separate git repo)
- Notebook remote: https://github.com/myusername/my-project-notebook
- Permissions: .claude/settings.json
- Project context: CLAUDE.md

Next steps:
- Edit CLAUDE.md to add project-specific context
- Use /perform-analysis to run and log analyses
```

## Idempotent

Safe to run multiple times. Will skip steps that are already complete and update what's needed.
