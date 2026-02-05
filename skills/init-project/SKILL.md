# /init-project Skill

Initialize a project for use with Claude Code and the notebook system.

## Overview

This skill sets up a project with:
- CLAUDE.md for project context
- Notebook system (as a separate git repo for clean separation)
- Project permissions for notebook access

## Execution Flow

### Phase 1: Configure Project Settings (FIRST)

**Check if settings already exist:**

```bash
test -f .claude/settings.json && echo "exists"
```

If `.claude/settings.json` already exists, **skip to Phase 2** (user has already restarted after initial setup).

**1.1 Create `.claude/settings.json`**

```bash
mkdir -p .claude
```

Create `.claude/settings.json`:
```json
{
  "permissions": {
    "allow": [
      "Read(/**)",
      "Edit(/notebook/**)",
      "Write(/notebook/**)",
      "Bash(git *)",
      "Bash(gh *)"
    ]
  }
}
```

This pre-approves:
- `Read(/**)` - Project-wide search (enables Glob/Grep tools)
- `Edit`/`Write` for notebook
- `Bash(git *)` - All git operations (init, remote, commit, push, etc.)
- `Bash(gh *)` - All GitHub CLI operations (repo create, auth, etc.)

Note: After setup is complete, we'll tighten permissions by replacing `Bash(git *)` with `Bash(git -C notebook *)`.

**1.2 Prompt user to restart**

Settings only take effect when Claude Code starts. After creating the file, display:

> **Restart required**
>
> I've created `.claude/settings.json` with setup permissions. For these to take effect, please:
>
> 1. Exit this conversation (Ctrl+C)
> 2. Run: `claude -c continue`
>
> This will continue setup with the new permissions active.

**Stop here.** Do not continue to Phase 2 until the user has restarted.

### Phase 2: Check Prerequisites

**2.1 Check if in a git repository**

```bash
git rev-parse --git-dir 2>/dev/null
```

If NOT a git repo:
- Run `git init`
- Continue to 2.2

**2.2 Check if main repo has a remote**

```bash
git remote -v
```

If NO remote configured:
- Check if `gh` CLI is available: `which gh`
- If `gh` not available:
  - If `brew` is available: install directly with `brew install gh`, then `source ~/.zshrc` (or `~/.bashrc`) to make it available in the current session
  - If `brew` not available: display GitHub CLI setup instructions (see Appendix A)
- If `gh` available but not authenticated: `gh auth status`
  - If not authenticated, tell the user: "Follow the instructions below to authorize the GitHub CLI:"
  - Run: `gh auth login --web --git-protocol https && gh auth setup-git`
  - The user must interact with the browser-based auth flow
- Once `gh` is ready, ask user:

> Your project doesn't have a GitHub remote yet. To continue, I'll help you create one.
>
> **What GitHub account should I use?** (Enter username or org name)

Then create the repo:
```bash
gh repo create <account>/<repo-name> --private --source=. --push
```

(Use current directory name as repo name by default)

**2.3 Verify `gh` CLI for notebook remote**

We'll need `gh` later for the notebook remote. If not available/authenticated, handle it now (same as 2.2).

**2.4 Check notification dependencies (macOS only)**

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

### Phase 3: Inform User of Plan

Display this message (do NOT ask for confirmation - just inform):

> **Setting up project for Claude Code**
>
> I'll now:
> 1. Create `notebook/` directory structure (as a separate git repo)
> 2. Add `notebook/` to `.gitignore`
> 3. Create or update `CLAUDE.md` with project template
>
> The notebook will be a separate git repository, keeping your main repo clean while preserving full history of analyses.

### Phase 4: Ask About Notebook Remote

> **Notebook backup (recommended)**
>
> The notebook repo can have its own GitHub remote for backup and sync across machines.
> This is a private repo that only stores your analysis logs, not your code.
>
> Create a GitHub remote for the notebook? (Recommended)

Options:
- **Yes, create remote** (recommended) - Will create `<repo-name>-notebook` on your GitHub
- **No, local only** - Notebook stays local, no backup

### Phase 5: Execute Setup

**5.1 Create notebook structure**

```bash
mkdir -p notebook/{entries,feedback}
```

**5.2 Initialize notebook git repo**

```bash
cd notebook
git init
```

**5.3 Create notebook template files**

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

**5.4 Initial notebook commit**

```bash
cd notebook
git add -A
git commit -m "Initialize notebook"
```

**5.5 Create notebook remote (if user chose yes)**

```bash
cd notebook
gh repo create <account>/<main-repo-name>-notebook --private --source=. --push
```

Use the same account as the main repo. Derive repo name from main repo name.

**5.6 Add notebook and CLAUDE.md to main .gitignore**

Append to `.gitignore` (create if doesn't exist):
```
# Claude Code (project-local context)
notebook/
CLAUDE.md
```

Commit to main repo:
```bash
git add .gitignore
git commit -m "chore: gitignore notebook and CLAUDE.md"
```

**5.7 Create or update CLAUDE.md**

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

Note: CLAUDE.md is gitignored and not committed to the main repo. It stays local as project context.

**5.8 Tighten project permissions**

Now that setup is complete, update `.claude/settings.json` to remove the broad `Bash(git *)` and `Bash(gh *)` permissions, keeping only what's needed for ongoing work:

```json
{
  "permissions": {
    "allow": [
      "Read(/**)",
      "Edit(/notebook/**)",
      "Write(/notebook/**)",
      "Bash(git -C notebook *)"
    ]
  }
}
```

Commit:
```bash
git add .claude/settings.json
git commit -m "chore: add Claude Code project settings"
```

### Phase 6: Offer Update Notebook

If the project has existing git history (more than just the commits we made):

> This project has existing git history. Would you like me to run `/update-notebook` to capture any past work?

If yes, invoke the `/update-notebook` skill.

### Phase 7: O2 Cluster Setup (Optional)

Ask the user:

> **O2 Cluster Access (Recommended)**
>
> Set up this project on O2 for running compute-intensive analyses?
> This enables the workflow: local edit → push → O2 pull → sbatch
>
> - **Yes, set up O2** (recommended for projects with heavy compute)
> - **No, skip** (can set up later)

If yes, proceed with O2 setup:

**7.1 Check O2 access configuration**

Check if remote-bridge is configured by looking for `~/.config/remote-bridge/permissions.toml`. If not set up, invoke `/remote-o2` skill first to establish O2 connection.

**7.2 Check GitHub SSH key on O2**

Via the O2 tmux session, check if SSH key exists:

```bash
# On O2
ls ~/.ssh/id_ed25519.pub 2>/dev/null
```

If key doesn't exist, set it up:

**7.2.1 Get user's GitHub email**

> What email address is associated with your GitHub account?

**7.2.2 Generate SSH key on O2**

```bash
# On O2
ssh-keygen -t ed25519 -C "<email>" -f ~/.ssh/id_ed25519 -N ""
```

**7.2.3 Display public key and guide user**

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

**7.2.4 Test GitHub connection**

```bash
# On O2
ssh -T git@github.com
```

Should see: "Hi <username>! You've successfully authenticated..."

If it fails, troubleshoot (may need to accept GitHub's host key first).

**7.3 Clone project on O2**

**7.3.1 Determine O2 project location**

Ask the user:

> Where should I clone this project on O2?
> (Common locations: `/n/data1/.../<username>/projects/<repo-name>` or `/n/scratch/users/.../projects/<repo-name>`)

Get user confirmation or custom path.

**7.3.2 Get SSH URLs**

```bash
# Local - get SSH URLs for both repos
git remote get-url origin | sed 's|https://github.com/|git@github.com:|'
git -C notebook remote get-url origin | sed 's|https://github.com/|git@github.com:|'
```

**7.3.3 Clone repos on O2**

```bash
# On O2
mkdir -p <project-path>
cd <project-path>
git clone git@github.com:<user>/<repo>.git .
git clone git@github.com:<user>/<repo>-notebook.git notebook
```

**7.3.4 Store O2 project path**

Add a note to the project's CLAUDE.md about the O2 location:

```markdown
## O2 Cluster
- Project location: `<path-on-o2>`
```

This enables future work to know where the project lives on O2.

**7.4 Verify setup**

```bash
# On O2
cd <project-path>
git status
git -C notebook status
```

Both should show clean working directories.

### Phase 8: Summary

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
