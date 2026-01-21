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

**1.4 Check notification dependencies (macOS only)**

```bash
uname -s
```

If macOS (`Darwin`), check for terminal-notifier:

```bash
which terminal-notifier
```

If not installed:
- Check if Homebrew is available: `which brew`
- If brew available, install: `brew install terminal-notifier`
- If brew not available, inform user:

> **Optional: Desktop notifications**
>
> For desktop notifications when Claude needs input or completes tasks, install terminal-notifier:
> ```bash
> brew install terminal-notifier
> ```
>
> (Requires Homebrew: https://brew.sh)

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
mkdir -p notebook/{entries,feedback}
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

| Date | Name | Summary |
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
      "Read(/**)",
      "Read(/notebook/**)",
      "Edit(/notebook/**)",
      "Write(/notebook/**)",
      "Bash(git -C notebook *)"
    ]
  }
}
```

Note: `Read(/**)` enables Glob and Grep tools across the project. Notebook permissions allow full read/write access plus git operations.

Commit:
```bash
git add .claude/settings.json
git commit -m "chore: add Claude Code project settings"
```

**4.8 Create or update CLAUDE.md**

If CLAUDE.md doesn't exist, ask the user:

> **Generate CLAUDE.md**
>
> Claude Code's `/init` command can scan your codebase and auto-generate a CLAUDE.md with:
> - Detected tech stack and frameworks
> - Project structure overview
> - Build/test/lint commands
> - Coding conventions
>
> How would you like to create CLAUDE.md?

Options:
- **Auto-generate with /init** (recommended) - Intelligent analysis of your codebase
- **Use basic template** - Simple template you fill in manually

If user chooses **Auto-generate with /init**:

1. Run the `/init` command (this is Claude Code's built-in command)
2. Wait for it to complete
3. Continue to check for notebook reference (below)

If user chooses **Use basic template**, create CLAUDE.md:

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

**After CLAUDE.md exists (either way)**, check if it has a notebook reference. If not, append:

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

### Phase 6: O2 Cluster Setup (Optional)

Ask the user:

> **O2 Cluster Access (Recommended)**
>
> Set up this project on O2 for running compute-intensive analyses?
> This enables the workflow: local edit → push → O2 pull → sbatch
>
> - **Yes, set up O2** (recommended for projects with heavy compute)
> - **No, skip** (can set up later)

If yes, proceed with O2 setup:

**6.1 Check O2 access configuration**

Check if remote-bridge is configured by looking for `~/.config/remote-bridge/permissions.toml`. If not set up, invoke `/remote-o2` skill first to establish O2 connection.

**6.2 Check GitHub SSH key on O2**

Via the O2 tmux session, check if SSH key exists:

```bash
# On O2
ls ~/.ssh/id_ed25519.pub 2>/dev/null
```

If key doesn't exist, set it up:

**6.2.1 Get user's GitHub email**

> What email address is associated with your GitHub account?

**6.2.2 Generate SSH key on O2**

```bash
# On O2
ssh-keygen -t ed25519 -C "<email>" -f ~/.ssh/id_ed25519 -N ""
```

**6.2.3 Display public key and guide user**

```bash
# On O2
cat ~/.ssh/id_ed25519.pub
```

Display to user:

> **Add this SSH key to GitHub**
>
> 1. Copy the key above (starts with `ssh-ed25519`)
> 2. Go to: https://github.com/settings/keys
> 3. Click "New SSH key"
> 4. Title: "O2 Cluster"
> 5. Paste the key and click "Add SSH key"
>
> Let me know when done.

**6.2.4 Test GitHub connection**

```bash
# On O2
ssh -T git@github.com
```

Should see: "Hi <username>! You've successfully authenticated..."

If it fails, troubleshoot (may need to accept GitHub's host key first).

**6.3 Clone project on O2**

**6.3.1 Determine O2 project location**

Ask the user:

> Where should I clone this project on O2?
> (Common locations: `/n/data1/.../<username>/projects/<repo-name>` or `/n/scratch/users/.../projects/<repo-name>`)

Get user confirmation or custom path.

**6.3.2 Get SSH URLs**

```bash
# Local - get SSH URLs for both repos
git remote get-url origin | sed 's|https://github.com/|git@github.com:|'
git -C notebook remote get-url origin | sed 's|https://github.com/|git@github.com:|'
```

**6.3.3 Clone repos on O2**

```bash
# On O2
mkdir -p <project-path>
cd <project-path>
git clone git@github.com:<user>/<repo>.git .
git clone git@github.com:<user>/<repo>-notebook.git notebook
```

**6.3.4 Store O2 project path**

Add a note to the project's CLAUDE.md about the O2 location:

```markdown
## O2 Cluster
- Project location: `<path-on-o2>`
```

This enables future work to know where the project lives on O2.

**6.4 Verify setup**

```bash
# On O2
cd <project-path>
git status
git -C notebook status
```

Both should show clean working directories.

### Phase 7: Summary

Display completion message:

> **Project initialized!**
>
> - Notebook: `notebook/` (separate git repo)
> - Notebook remote: `https://github.com/<account>/<repo>-notebook` (or "local only")
> - Permissions: `.claude/settings.json`
> - Project context: `CLAUDE.md`
> - O2 clone: `<path-on-o2>` (or "not configured")
>
> **Next steps:**
> - Edit `CLAUDE.md` to add project-specific context
> - Use `/perform-analysis` to run and log analyses
> - Use `/new-data` when working with new datasets
>
> **O2 workflow** (if configured):
> ```
> # Push changes locally
> git push && git -C notebook push
>
> # Then use /remote-o2 to pull and run on O2
> ```

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
