---
name: remind-resume
description: Resume the current conversation after a break. Invoke manually with /remind-resume to get a summary of work done in this session, git state, and what to pick up next. User-invoked only - never auto-trigger.
disable-model-invocation: true
---

# Remind / Resume

The user is returning after a break and may not remember the details of this conversation. They know broadly what they were doing but need the specifics surfaced: what changed, what was decided, what state things are in.

## Step 1: Gather Context (parallel)

**1a. Notebook entries for inter-session awareness**

Read `notebook/INDEX.md` and identify entries dated after this conversation started. Read those entries for your own context — the user doesn't need a list unless directly relevant to the current task.

**1b. Git and worktree state**

Spawn an Explore subagent to assess git state: branch, working directory status, unpushed commits, recent commits, open PRs, stashes, and active worktrees.

## Step 2: Report and Next Step

Present a concise report. Omit sections with nothing to report. End with a specific suggested next action and ask: **"Does this match where you left off, or would you like to adjust direction?"**

```
## Session Resume

### Current Task
[What the user is working on, derived from conversation history.]

### Work Done This Session
[Key accomplishments, decisions, files changed. Whether a notebook entry was created.]

### Git State
[Branch, uncommitted changes, unpushed commits, open PRs, worktrees, stashes.]

### Related Work From Other Sessions
[Only if other sessions' notebook entries touch this work. Otherwise omit.]
```
