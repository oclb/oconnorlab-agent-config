# Global Claude Configuration

## Configuration Repository

Your settings and skills are managed in a Git repository. The user's `~/.claude/CLAUDE.md` imports this file using Claude Code's native `@import` syntax, so updates to the repo are automatically picked up.

To find the config repo location, look for the `@` import line in `~/.claude/CLAUDE.md`.

## AFK Mode (Per-Turn)

When a user includes `(afk)` in their message, apply **AFK mode for that turn only**:
- Be more independent - proceed if 80%+ confident in interpretation
- Document assumptions in notebook entries
- Only pause for: irreversible actions (destructive git operations, file deletions), external API calls with side effects, or decisions where wrong choice would require significant rework
- When pausing, state the specific decision point

AFK mode does not persist across turns. Each message starts fresh unless it contains `(afk)`.

## Tool Installation and Commands

Prefer to run commands directly rather than asking the user to run them. This includes:
- Installing tools and packages (e.g., `brew install`, `pip install`, `cargo install`)
- Running build commands
- Executing scripts

Only ask the user to run commands when:
- The command requires authentication the user must provide interactively (e.g., `gh auth login`)
- The command requires elevated privileges you don't have
- The command has significant side effects the user should explicitly approve

## Notebook System

The notebook is a **separate git repository** inside the project directory, gitignored from the main repo. This keeps the main repo clean while preserving full notebook history.

### Structure

```
notebook/
├── .git/                 # Separate repo
├── INDEX.md              # Entry index for retrieval (Explore agents start here)
├── ARCHIVE.md            # Archived entries (removed from INDEX.md, still searchable)
├── TODO.md               # Active tasks
├── DONE.md               # Completed tasks
└── entries/              # All memories
    └── YYYY-MM-DD-<slug>.md
```

If notebook is not set up as a separate repo, suggest `/init-project`.

### Notebook Commit Pattern

```bash
git -C notebook add <files>
git -C notebook commit -m "<message>"
git -C notebook remote | grep -q origin && git -C notebook push
```

## Notebook Entries

Entries record **substantial work** that a future session would need to know about. **The bar is high.** Most conversations do NOT need an entry.

Create an entry when:
- You completed a multi-step analysis or implementation
- You made architectural or methodological decisions that affect future work
- You discovered something non-obvious (a bug root cause, an unexpected finding)
- You set up tools or environments with gotchas worth recording

Do NOT create entries for: quick answers, minor fixes, work fully captured by git commits, routine operations, or planning without action.

### Entry Format

```markdown
# <Descriptive Title>

**Date:** YYYY-MM-DD
**Author:** Claude
**User:** <github-username from git config user.name>

## Summary
[One paragraph]

## Details
[The work itself]

## References
- `<entry-name>`: <why it was useful>
```

When you create an entry, state: "Created notebook entry: `<entry-name>`"

## Memory Creation via Sub-Agent

When you determine an entry is needed, spawn a background memory agent:

1. Find the config repo: `CONFIG_REPO=$(readlink ~/.claude/CLAUDE.md | xargs dirname | xargs dirname)`
2. Read `$CONFIG_REPO/templates/memory-agent-prompt.md`
3. Spawn the agent with that file's contents as the prompt:
   - subagent_type: "general-purpose"
   - run_in_background: true
   - model: "sonnet"

**After spawning:**
- Do NOT call TaskOutput to check progress
- Do NOT mention the agent's status — just continue with your substantive work
- When the agent completes, do NOT comment on it — produce no output
- Subsequent memory agents in the same session will see the existing entry and append to it

If memory creation is critical, spawn the agent early in your response rather than at the end.

## Index and Retrieval

### Session Start

1. Read `notebook/INDEX.md` immediately
2. If user's first message references past work ("that", "the", "our"), retrieve relevant entries
3. If planning a task similar to an indexed entry, read that entry first

### Retrieval Triggers

Use **Explore subagent** when ANY of these patterns appear:
- Demonstrative references: "the analysis", "that script", "what we did", "last time"
- User references a date, filename, or method in INDEX.md
- User asks to continue, extend, or fix previous work
- You're about to start work that might duplicate an existing entry

Direct Explore to start at `notebook/INDEX.md`, then read full entries as needed.

### Cross-User Attribution

When referencing entries created by a different user (check the **User** field vs. `git config user.name`), mention this explicitly: "Based on `entry-name` (created with user X)..."

### Archiving

Old entries can be moved from INDEX.md to `notebook/ARCHIVE.md` (same format). Entry files stay in `notebook/entries/`. Use `/maintain-project` for guided archiving.

## Project CLAUDE.md Integration

Reference notebook entries from the project's `CLAUDE.md` for active context (e.g., "Authentication uses OAuth2 (see `oauth2-implementation`)"). Remove references when superseded — CLAUDE.md is for current context only.

## Persistent To-Do List

Track tasks in `notebook/TODO.md` (active) and `notebook/DONE.md` (completed).

**When to use:** User explicitly asks to add, show, or work on todos.

**Key rules:**
- TODO.md has a `Next ID:` counter at the top — always use it and increment after adding
- Link todos to notebook entries via `Context:` when relevant
- **Todo completion is handled by the memory agent.** When you finish work on a todo, just create a notebook entry as usual — the memory agent will match it to the todo and move it to DONE.md automatically. You do NOT need to manually update TODO.md/DONE.md after completing work.
- For manual todo management (adding, editing, deleting), commit with descriptive messages: `git -C notebook add TODO.md && git -C notebook commit -m "todo: add #N - <name>"`

For detailed formats and all operations, read `<config-repo>/templates/todo-reference.md`.

## Feedback Logging

Log feedback about Claude's behavior to the config repo's `feedback/` directory.

**Log automatically when:** user corrects your interpretation, user manually invokes a skill you should have auto-detected, a command fails due to anticipatable issues, or user points to an entry you should have retrieved.

**Ask first when:** user expresses skepticism, your first approach fails, or user mentions behavior could be improved.

**How to log:**
```bash
CONFIG_REPO=$(readlink ~/.claude/CLAUDE.md | xargs dirname | xargs dirname)
# Create $CONFIG_REPO/feedback/YYYY-MM-DD-brief-description.md with **User:** <git config user.name> header
git -C "$CONFIG_REPO" add feedback/ && git -C "$CONFIG_REPO" commit -m "feedback: <brief description>"
git -C "$CONFIG_REPO" remote | grep -q origin && git -C "$CONFIG_REPO" push
```
