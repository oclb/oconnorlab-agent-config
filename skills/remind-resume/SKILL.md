---
name: remind-resume
description: Resume the current conversation after a break. Invoke manually with /remind-resume to get a summary of work done in this session, git state, and what to pick up next. User-invoked only - never auto-trigger.
disable-model-invocation: true
---

# Remind / Resume

The user is returning to this conversation after a break (switching machines, taking a meeting, etc.). They may have multiple active work streams and don't fully remember where this one stands. Your job is to re-orient them: summarize what's been accomplished in this session, where things are, and what the logical next step is.

Do not assume the user remembers the details of this conversation. They aren't amnesiac — they know broadly what they were doing — but the specifics (which files were changed, what decisions were made, whether things were committed) need to be surfaced.

## Step 1: Gather Context (parallel)

Run both of the following in parallel:

**1a. Notebook entries for inter-session awareness**

Determine the timestamp of the user's first message in this conversation. Read `notebook/INDEX.md` and identify any entries whose date falls after the conversation started — these represent work done in *other* sessions that may interact with the current work. Read those entries. This information is for your own context; the user does not need a list of these entries unless they are directly relevant to the current task.

**1b. Git and worktree state**

Spawn an Explore subagent to assess the full git state of the project. The subagent should determine:

- Current branch and working directory status (clean, modified, untracked files)
- Whether there are unpushed commits or an upstream tracking branch
- Recent commits on the current branch
- Any open pull requests (use `gh pr list` if available)
- Any stashes
- Active worktrees (these indicate interrupted work streams)

The subagent knows git — don't over-specify commands. It should return a concise summary.

## Step 2: Compile Report

Present a concise report using this structure. Omit any section that has nothing to report.

```
## Session Resume

### Current Task
[What is the user currently working on in this session? Derive this from
the conversation history — it may or may not correspond to a TODO item.
State it concretely: "Implementing X", "Debugging Y", "Planning Z".]

### Work Done This Session
[Summarize what has been accomplished in this conversation so far.
Include key decisions made, files changed, problems solved.
Note whether a notebook entry has been created for this session's work.]

### Git State
[Concise summary from the subagent. Highlight anything that needs
attention: uncommitted changes, unpushed commits, open PRs,
active worktrees, stashes.]

### Related Work From Other Sessions
[Only include this section if notebook entries from other sessions
(created after this conversation started) touch on the same area of
work. Briefly note what was done and how it relates. If nothing
relevant, omit this section entirely.]
```

## Step 3: Suggest Next Step

After the report, suggest the logical next action based on the session state. Be specific:

- If there's an in-progress task, suggest continuing it.
- If work is done but uncommitted, suggest committing.
- If committed but no PR, suggest creating one.
- If blocked, say so and suggest what would unblock it.

Then ask: **"Does this match where you left off, or would you like to adjust direction?"**

## Notes

- This skill is purely informational — it reads state but changes nothing.
- Keep the report scannable. The user's attention is split; respect their time.
- Focus on *this session's* work. Other sessions are background context, not the main event.
- If there are active worktrees or stashes, flag them — they likely represent the user's other interrupted work streams.
