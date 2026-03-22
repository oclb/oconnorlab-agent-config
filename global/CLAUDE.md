# Global Claude Configuration

## Configuration Repository

Your settings and skills are managed in a Git repository. The user's `~/.claude/CLAUDE.md` imports this file using Claude Code's native `@import` syntax, so updates to the repo are automatically picked up.

To find the config repo location, look for the `@` import line in `~/.claude/CLAUDE.md`.

## Notebook System

The notebook is a separate git repository inside the project directory, gitignored from the main repo.

```
notebook/
├── .git/
├── INDEX.md              # Entry index (Explore agents start here)
├── ARCHIVE.md            # Archived entries
├── TODO.md               # Active tasks
├── DONE.md               # Completed tasks
└── entries/
    └── YYYY-MM-DD-<slug>.md
```

If notebook is not set up as a separate repo, suggest `/init-project`.

### Notebook Entries

Entries record substantial work that a future session would need to know about. Most conversations do not need an entry.

Create an entry when embarking upon:
- A multi-step analysis or implementation
- Architectural or methodological decisions that affect future work
- A non-obvious discovery (a bug root cause, an unexpected finding)
- Tool or environment setup with gotchas worth recording

You can create an entry when you start a task and update it as you progress — entries are living documents, not just summaries.

Do not create entries for: quick answers, minor fixes, work fully captured by git commits, routine operations, or planning without action.

When you create an entry, state: "Created notebook entry: `<entry-name>`"

### Memory Creation via Sub-Agent

Spawn the memory agent early in your response — before your main work — so it has time to complete before the user might exit the session.

To spawn a memory agent:
1. Find the config repo: `CONFIG_REPO=$(readlink ~/.claude/CLAUDE.md | xargs dirname | xargs dirname)`
2. Read `$CONFIG_REPO/templates/memory-agent-prompt.md`
3. Spawn the agent with that file's contents as the prompt (subagent_type: "general-purpose", run_in_background: true, model: "sonnet")

After spawning, do not check on the agent's progress or mention its status. When it completes, produce no output about it. Subsequent memory agents in the same session will see the existing entry and append to it.

### Index and Retrieval

At session start, read `notebook/INDEX.md`. If the user's first message references past work ("that", "the", "our"), retrieve relevant entries. If planning a task similar to an indexed entry, read that entry first.

Use an Explore subagent when the user references past work, asks to continue/extend/fix previous work, or when you're about to start work that might duplicate an existing entry. Direct Explore to start at `notebook/INDEX.md`.

When referencing entries created by a different user (check the **User** field vs. `git config user.name`), mention this: "Based on `entry-name` (created with user X)..."

Old entries can be moved from INDEX.md to `notebook/ARCHIVE.md`. Entry files stay in `notebook/entries/`. Use `/maintain-project` for guided archiving.

### Project CLAUDE.md Integration

Reference notebook entries from the project's `CLAUDE.md` for active context (e.g., "Authentication uses OAuth2 (see `oauth2-implementation`)"). Remove references when superseded.

### Persistent To-Do List

Track tasks in `notebook/TODO.md` (active) and `notebook/DONE.md` (completed). Use when the user explicitly asks to add, show, or work on todos.

TODO.md has a `Next ID:` counter at the top — always use it and increment after adding. Link todos to notebook entries via `Context:` when relevant. Todo completion is handled by the memory agent — when you finish work on a todo, just create a notebook entry as usual and the memory agent will move the todo to DONE.md automatically.

For detailed formats, read `<config-repo>/templates/todo-reference.md`.

## Feedback Logging

When the user uses the word "feedback", log it to the config repo's `feedback/` directory:

```bash
CONFIG_REPO=$(readlink ~/.claude/CLAUDE.md | xargs dirname | xargs dirname)
# Create $CONFIG_REPO/feedback/YYYY-MM-DD-brief-description.md with **User:** <git config user.name> header
git -C "$CONFIG_REPO" add feedback/ && git -C "$CONFIG_REPO" commit -m "feedback: <brief description>"
git -C "$CONFIG_REPO" remote | grep -q origin && git -C "$CONFIG_REPO" push
```

## Tool Installation

Prefer to run commands directly (install, build, execute) rather than asking the user to run them. Only ask when the command requires interactive authentication, elevated privileges, or has significant side effects.

## Literature Search

When listing sources, always include clickable hyperlinks (DOI links, PubMed URLs, or publisher URLs).

## AFK Mode

When a user includes `(afk)` in their message, apply AFK mode for that turn only: be more independent, proceed if 80%+ confident, document assumptions, and only pause for irreversible actions or decisions where the wrong choice would require significant rework. AFK mode does not persist across turns.
