# /init-project

Initialize a project for use with Claude Code and the notebook system.

## When to Use

- First time using Claude Code with a project
- Complements the built-in `/init` command with notebook system setup

## What It Does

1. **Configures permissions first** - Creates `.claude/settings.json` with broad permissions to avoid prompts during setup
2. **Ensures main repo has a GitHub remote** - Guides user through setup if needed
3. **Creates notebook structure** - `notebook/{entries,feedback}/`
4. **Initializes notebook as separate git repo** - Keeps main repo clean
5. **Offers notebook remote** - Recommended for backup/sync
6. **Creates or enhances CLAUDE.md** - Offers to run `/init` for intelligent auto-generation, or uses a basic template
7. **Tightens permissions** - Removes broad setup permissions, keeping only what's needed for ongoing work

## Integration with `/init`

When no CLAUDE.md exists, `/init-project` offers two options:

| Option | What happens |
|--------|--------------|
| **Auto-generate with /init** (recommended) | Runs Claude Code's built-in `/init` to scan your codebase and generate an intelligent CLAUDE.md |
| **Use basic template** | Creates a simple template you fill in manually |

After CLAUDE.md exists (either way), `/init-project` appends a notebook reference section and continues with the notebook setup.

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

Creating .claude/settings.json with setup permissions...

**Restart required**

I've created .claude/settings.json with setup permissions. For these to take effect, please:

1. Exit this conversation (Ctrl+C)
2. Run: claude -c continue

> [user runs: claude -c continue]

Continuing setup with permissions active...

Your project doesn't have a GitHub remote yet. To continue, I'll help you create one.

What GitHub account should I use? myusername

Creating github.com/myusername/my-project...

Setting up project for Claude Code

I'll now:
1. Create notebook/ directory structure (as a separate git repo)
2. Add notebook/ to .gitignore
3. Create or update CLAUDE.md

Generate CLAUDE.md

Claude Code's /init command can scan your codebase and auto-generate a CLAUDE.md.
How would you like to create CLAUDE.md?
> Auto-generate with /init (recommended)

[/init runs and generates CLAUDE.md based on codebase analysis]

Notebook backup (recommended)

Create a GitHub remote for the notebook?
> Yes, create remote (recommended)

Creating github.com/myusername/my-project-notebook...

Project initialized!

- Notebook: notebook/ (separate git repo)
- Notebook remote: https://github.com/myusername/my-project-notebook
- Permissions: .claude/settings.json
- Project context: CLAUDE.md (auto-generated)

Next steps:
- Review CLAUDE.md and add any missing context
- Use /perform-analysis to run and log analyses
```

## Idempotent

Safe to run multiple times. Will skip steps that are already complete and update what's needed.
