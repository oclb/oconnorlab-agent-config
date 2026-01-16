# /init-project Skill

Initialize a project for use with Claude Code and the notebook system.

## Overview

This skill sets up a project with:
- CLAUDE.md for project context
- Notebook system (as a separate git repo for clean separation)
- Project permissions for notebook access

## Execution Flow

### Phase 1: Check Prerequisites

**1.1 Check if in a git repository**

```bash
git rev-parse --git-dir 2>/dev/null
```

If NOT a git repo:
- Run `git init`
- Continue to 1.2

**1.2 Check if main repo has a remote**

```bash
git remote -v
```

If NO remote configured:
- Check if `gh` CLI is available: `which gh`
- If `gh` not available, display GitHub CLI setup instructions (see Appendix A)
- If `gh` available but not authenticated: `gh auth status`
  - If not authenticated, run `gh auth login` and guide user through it
- Once `gh` is ready, ask user:

> Your project doesn't have a GitHub remote yet. To continue, I'll help you create one.
>
> **What GitHub account should I use?** (Enter username or org name)

Then create the repo:
```bash
gh repo create <account>/<repo-name> --private --source=. --push
```

(Use current directory name as repo name by default)

**1.3 Verify `gh` CLI for notebook remote**

We'll need `gh` later for the notebook remote. If not available/authenticated, handle it now (same as 1.2).

### Phase 2: Inform User of Plan

Display this message (do NOT ask for confirmation - just inform):

> **Setting up project for Claude Code**
>
> I'll now:
> 1. Create `notebook/` directory structure (as a separate git repo)
> 2. Add `notebook/` to `.gitignore`
> 3. Create `.claude/settings.json` with notebook permissions
> 4. Create or update `CLAUDE.md` with project template
>
> The notebook will be a separate git repository, keeping your main repo clean while preserving full history of analyses.

### Phase 3: Ask About Notebook Remote

> **Notebook backup (recommended)**
>
> The notebook repo can have its own GitHub remote for backup and sync across machines.
> This is a private repo that only stores your analysis logs, not your code.
>
> Create a GitHub remote for the notebook? (Recommended)

Options:
- **Yes, create remote** (recommended) - Will create `<repo-name>-notebook` on your GitHub
- **No, local only** - Notebook stays local, no backup

### Phase 4: Execute Setup

**4.1 Create notebook structure**

```bash
mkdir -p notebook/{analyses,methods,feedback}
```

**4.2 Initialize notebook git repo**

```bash
cd notebook
git init
```

**4.3 Create notebook template files**

Create `notebook/INDEX.md`:
```markdown
# Notebook Index

## Analyses
| ID | Summary | Date | Tags |
|----|---------|------|------|

## Methods
| Date | Type | Summary |
|------|------|---------|
```

Create `notebook/TODO.md`:
```markdown
# To-Do

```

Create `notebook/DONE.md`:
```markdown
# Completed

```

**4.4 Initial notebook commit**

```bash
cd notebook
git add -A
git commit -m "Initialize notebook"
```

**4.5 Create notebook remote (if user chose yes)**

```bash
cd notebook
gh repo create <account>/<main-repo-name>-notebook --private --source=. --push
```

Use the same account as the main repo. Derive repo name from main repo name.

**4.6 Add notebook to main .gitignore**

Append to `.gitignore` (create if doesn't exist):
```
# Claude Code notebook (separate repo)
notebook/
```

Commit to main repo:
```bash
git add .gitignore
git commit -m "chore: gitignore notebook (separate repo)"
```

**4.7 Create project permissions**

Create `.claude/settings.json`:
```json
{
  "permissions": {
    "allow": [
      "Read(/notebook/**)",
      "Edit(/notebook/**)",
      "Write(/notebook/**)",
      "Bash(mkdir -p notebook:*)",
      "Bash(git -C notebook *)",
      "Bash(ls notebook:*)",
      "Bash(cp * notebook/*)",
      "Bash(mv * notebook/*)"
    ]
  }
}
```

Note: `cp` and `mv` patterns require ` notebook/` (with space) in the command, ensuring notebook is the destination, not just the source.

Commit:
```bash
git add .claude/settings.json
git commit -m "chore: add Claude Code project settings"
```

**4.8 Create or update CLAUDE.md**

If CLAUDE.md doesn't exist, create it with template:

```markdown
# Project Name

## Overview

[Brief description of the project]

## Current Status

[What's the current focus?]

## Key Files

| Path | Description |
|------|-------------|

## Data

| Dataset | Location | Description |
|---------|----------|-------------|

## Notes

[Important context for Claude]
```

If CLAUDE.md exists, check if it has a notebook reference. If not, append:

```markdown

## Notebook

This project uses a separate notebook repository for analysis logs. See `notebook/INDEX.md` for a summary of past work.
```

Commit:
```bash
git add CLAUDE.md
git commit -m "docs: initialize/update CLAUDE.md"
```

### Phase 5: Offer Update Notebook

If the project has existing git history (more than just the commits we made):

> This project has existing git history. Would you like me to run `/update-notebook` to capture any past work?

If yes, invoke the `/update-notebook` skill.

### Phase 6: Summary

Display completion message:

> **Project initialized!**
>
> - Notebook: `notebook/` (separate git repo)
> - Notebook remote: `https://github.com/<account>/<repo>-notebook` (or "local only")
> - Permissions: `.claude/settings.json`
> - Project context: `CLAUDE.md`
>
> **Next steps:**
> - Edit `CLAUDE.md` to add project-specific context
> - Use `/perform-analysis` to run and log analyses
> - Use `/new-data` when working with new datasets

---

## Appendix A: GitHub CLI Setup Instructions

When `gh` CLI is not available, display:

> **GitHub CLI Required**
>
> This setup uses the GitHub CLI (`gh`) to create repositories. It's the easiest way to manage GitHub from the command line.
>
> **Install GitHub CLI:**
>
> macOS:
> ```bash
> brew install gh
> ```
>
> Linux (Debian/Ubuntu):
> ```bash
> sudo apt install gh
> ```
>
> Other: See https://cli.github.com/
>
> **After installing, authenticate:**
>
> Run this command (it will run in the background):
> ```bash
> gh auth login --web --git-protocol https
> ```
>
> The command will display:
> ```
> ! First copy your one-time code: XXXX-XXXX
> Open this URL to continue in your web browser: https://github.com/login/device
> ```
>
> **Steps to complete:**
> 1. Copy the one-time code shown
> 2. Open https://github.com/login/device in your browser
> 3. Enter the code and authorize
>
> Once you see "✓ Logged in as <username>", run one more command to configure git:
> ```bash
> gh auth setup-git
> ```
> This enables git to use your GitHub credentials automatically.
>
> Once complete, run `/init-project` again.

---

## Appendix B: Handling Existing Partial Setup

If running `/init-project` on a project with partial setup:

| Existing State | Action |
|----------------|--------|
| `notebook/` exists but not a git repo | Init as git repo, continue |
| `notebook/.git` exists | Skip notebook creation, check for remote |
| `.claude/settings.json` exists | Merge permissions (add notebook permissions if missing) |
| `CLAUDE.md` exists | Update rather than replace |
| `notebook/` in `.gitignore` | Skip adding to .gitignore |

The skill should be idempotent - safe to run multiple times.

---

## Appendix C: Updating Other Skills

Skills that create notebook entries must commit to the notebook repo, not the main repo:

**Pattern for notebook commits:**
```bash
git -C notebook add <files>
git -C notebook commit -m "<message>"
git -C notebook push  # if remote exists
```

**Check if notebook has remote:**
```bash
git -C notebook remote -v
```

Skills to update:
- `/perform-analysis`
- Global CLAUDE.md: Methods memory, TODO management, Feedback logging
