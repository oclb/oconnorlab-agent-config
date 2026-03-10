---
name: remind-resume
description: Resume a previous work session. Invoke manually with /remind-resume to get a summary of where you left off, including recent notebook entries, git state, and active tasks. User-invoked only - never auto-trigger.
disable-model-invocation: true
---

# Remind / Resume

Help the user pick up where they left off. Produce a concise status report covering: what was being worked on, what state changes are in, and what's happened since the last session.

## Process

Run the information-gathering steps below (parallelizing the independent ones), then compile into the summary report.

### Step 1: Gather Context (parallel)

Run all of the following in parallel:

**1a. Recent notebook entries**

Read `notebook/INDEX.md`. Identify the 3 most recent entries by date. Read those entry files to understand what was being worked on.

**1b. Active tasks**

Read `notebook/TODO.md` for open tasks.

**1c. Git status of main repo**

Run these commands:

```bash
# Current branch and working directory state
git branch --show-current
git status --short

# Unpushed commits (if tracking a remote)
git log @{u}..HEAD --oneline 2>/dev/null || echo "No upstream tracking branch"

# Recent commits (last 10)
git log --oneline -10

# Stashes
git stash list

# Open PRs from current branch
gh pr list --head "$(git branch --show-current)" --state open --json number,title,url 2>/dev/null || echo "gh not available or not authenticated"

# All open PRs in the repo
gh pr list --state open --json number,title,url,headRefName 2>/dev/null || echo ""
```

**1d. Worktree state**

```bash
# Check for active worktrees (indicates in-progress work)
git worktree list
```

**1e. Notebook repo state**

```bash
# Last notebook commit (proxy for last Claude session)
git -C notebook log -1 --format="Last notebook activity: %ai%n%s" 2>/dev/null || echo "No notebook repo"

# Notebook repo status (uncommitted notebook changes)
git -C notebook status --short 2>/dev/null
```

**1f. Work since last notebook session**

```bash
# Get the timestamp of the last notebook commit
LAST_SESSION=$(git -C notebook log -1 --format="%aI" 2>/dev/null)

if [ -n "$LAST_SESSION" ]; then
  # Main repo commits made after the last notebook commit
  echo "=== Main repo commits since last notebook activity ==="
  git log --oneline --after="$LAST_SESSION"

  # Notebook entries modified after the last notebook commit
  echo "=== Recently modified notebook entries ==="
  find notebook/entries -name "*.md" -newer notebook/.git/COMMIT_EDITMSG 2>/dev/null
fi
```

### Step 2: Compile Report

Present a concise report using this structure. Omit any section that has nothing to report.

```
## Session Resume

### Last Session
[Summary of the most recent notebook entry — what was being worked on, key findings/decisions]

### Current Task
[From TODO.md — what's actively being worked on, if anything]

### Git State
- **Branch:** [current branch]
- **Working directory:** [clean / N modified files / N untracked files]
- **Unpushed commits:** [none / list]
- **Open PRs:** [none / list with URLs]
- **Worktrees:** [none / list — these indicate interrupted work]
- **Stashes:** [none / list]

### Since Last Session
[Any main repo commits or notebook entries created since the last notebook commit.
If nothing: "No changes since last session."]

### Recent Notebook Entries
[Bulleted list of last 3 entries with one-line summaries, most recent first]
```

### Step 3: Ask What to Work On

After presenting the report, ask:

> What would you like to pick up? You can continue the previous task, start something new, or ask me to dig deeper into any of the above.

## Notes

- This skill is purely informational — it reads state but changes nothing.
- Keep the report scannable. Lead with the most important information (what was I doing? what state is it in?).
- If there are multiple worktrees, highlight them — they likely represent interrupted work streams.
- If there are uncommitted changes or stashes, flag them prominently — the user may have forgotten about them.
