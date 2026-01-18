# Global Claude Configuration

## Configuration Repository

Your settings and skills are managed in a Git repository, symlinked to their standard locations by setup scripts. Everything should "just work" - no special paths needed.

The repo location is stored in `~/.claude/behavior.conf` as `CONFIG_REPO`. If you need to understand the configuration setup, modify settings at the source, or read documentation about available skills, check `$CONFIG_REPO/README.md`.

## User-Specific Configuration

**IMPORTANT:** If the file `$CONFIG_REPO/global/CLAUDE.user.md` exists, read it AFTER reading this file. It contains user-specific instructions and preferences that override or extend these global settings. The user file is gitignored and won't cause conflicts when pulling from the repo.

## Behavior Flags

At the start of each session, read `~/.claude/behavior.conf` to check the current flag values. These flags modify how you should behave.

### Flag Definitions

| Flag | Default | Behavior |
|------|---------|----------|
| `AFK` | `false` | When `true`: Be more independent. Make reasonable decisions without asking. Proceed with likely interpretations rather than clarifying ambiguities. Complete multi-step tasks autonomously. Only pause for critical decisions that would be difficult to reverse. |
| `Environment` | `local` | Always `local`. For O2 cluster access, use the `/remote-o2` skill which connects via SSH. |

### Auto-Detection Keywords

Watch for these keywords in user prompts and automatically update the behavior.conf file:

| Keyword | Action |
|---------|--------|
| `(afk)` | Set `AFK=true` in behavior.conf |
| `(back)` | Set `AFK=false` in behavior.conf |

When you detect these keywords:
1. Use sed or similar to update the flag in `~/.claude/behavior.conf`
2. Confirm the change briefly (e.g., "AFK mode enabled")
3. Apply the new behavior immediately

### Flag File Format

The file uses simple `KEY=value` format:
- Lines starting with `#` are comments
- Blank lines are ignored
- Flag names are case-sensitive
- Boolean flags use `true` or `false`
- String flags use their defined values (e.g., `Environment=local`)

### How to Apply Flags

1. Read `~/.claude/behavior.conf` at session start
2. For each flag defined above, check if it exists in behavior.conf
3. If a flag is missing from behavior.conf (or the file doesn't exist), use the Default from the table above
4. Adjust your behavior according to the flag definitions

## Notebook System

The notebook is a **separate git repository** inside the project directory, gitignored from the main repo. This keeps the main repo clean while preserving full notebook history.

### Structure

```
notebook/
├── .git/                 # Separate repo
├── INDEX.md              # Entry index for retrieval (Explore agents start here)
├── TODO.md               # Active tasks
├── DONE.md               # Completed tasks
├── entries/              # All memories (analyses, features, research, discussions, etc.)
│   └── YYYY-MM-DD-<slug>.md
└── feedback/             # Feedback about Claude's behavior (separate, not indexed)
    └── YYYY-MM-DD-<description>.md
```

### Notebook Commit Pattern

For all notebook operations:

```bash
git -C notebook add <files>
git -C notebook commit -m "<message>"
git -C notebook remote | grep -q origin && git -C notebook push
```

If notebook is not set up as a separate repo, suggest `/init-project`.

## Notebook Entries

All memories go in `notebook/entries/`. Any task that produces knowledge worth recalling should create an entry: analyses, features, bug fixes, research, discussions, tool setup, presentations, paper drafts, etc.

**The goal is to not lose knowledge.** If you learned something useful, write it down.

### When to Create an Entry

Create an entry when:
- You learned something that would be useful to recall later
- The task took more than a few minutes of substantive work
- You made decisions that future sessions should know about
- You produced artifacts (scripts, figures, documents)

### Entry Format

Create `notebook/entries/YYYY-MM-DD-<slug>.md`:

```markdown
# <Descriptive Title>

**Date:** YYYY-MM-DD

## Summary
[One paragraph: what was done and why]

## Details
[The work itself - can be updated as work progresses]

## References
- `<entry-name>`: <why it was useful>
```

The **References** section records which previous entries informed this work and why. Examples:
- `ssh-socket-validation`: debugging pattern for stale sockets
- `o2-job-template`: SLURM script structure used as starting point
- `variant-filtering-v2`: statistical approach for handling missingness

### Writing Entries As You Go

Write entries incrementally during work, not just as a summary at the end. Start the entry when beginning a task, add details as you progress, and finalize when complete.

**Announce entry creation:** When you create an entry, state: "Created notebook entry: `<entry-name>`"

### New Session vs. Continuing Work

- **New session → new entry** (most of the time)
- **Continue existing entry** only when user says "continue", "finish", or similar

Git tracks entry history, so updating is fine when appropriate.

## Index and Retrieval

### INDEX.md Format

`notebook/INDEX.md` is a minimal quick-reference for retrieval:

```markdown
# Notebook Index

| Date | Name | Summary |
|------|------|---------|
| 2026-01-18 | memory-system-design | Consolidated memory with tagged references |
| 2026-01-17 | ssh-socket-validation | ControlMaster -O check insufficient for socket validation |
```

**Update INDEX.md** whenever you create or modify an entry. Commit together.

### Session Start

At the start of every conversation, read `notebook/INDEX.md` to know what memories exist.

### Retrieval via Explore Agent

Use the **Explore subagent** to retrieve relevant memories when:
- Unsure what user means or references
- User mentions past work ("that analysis", "the script we wrote", etc.)
- Starting any task that requires multi-step planning

Direct the Explore agent to start at `notebook/INDEX.md`, identify potentially relevant entries, then read full entries as needed. The References section in entries provides hints about whether linked entries are worth exploring.

## Project CLAUDE.md Integration

The project's `CLAUDE.md` should reference notebook entries for active context:

**Add references to CLAUDE.md when:**
- New capability that affects how the project works
- Important finding that shapes ongoing work
- Key dataset or method that analyses depend on

**Format in CLAUDE.md:**
```markdown
## Current State
- Authentication uses OAuth2 (see `oauth2-implementation`)
- Variant filtering excludes singletons (see `variant-filtering-v2`)
```

**Remove references when stale:** Prune CLAUDE.md when entries are superseded or no longer relevant. The notebook preserves history; CLAUDE.md is for current context only.

## Persistent To-Do List

Track tasks across sessions using two files:
- `notebook/TODO.md` - Active tasks (kept small)
- `notebook/DONE.md` - Completed tasks (archival record)

### When to Use

- User explicitly asks to "add a todo", "remember to do X", "log this task"
- User asks to "show todos", "what's on my list", "what needs doing"
- User asks to "work on todo #N" or "do the X task"

### File Formats

**notebook/TODO.md** (active tasks):
```markdown
# To-Do

- [ ] #1 **Task name** - Brief description
  - Context: `notebook/entries/related-entry` (if applicable)
  - Added: YYYY-MM-DD

- [ ] #2 **Another task** - Description
  - Added: YYYY-MM-DD
```

**notebook/DONE.md** (completed tasks):
```markdown
# Completed

- [x] #0 **Example task** - Original description preserved
  - Context: `notebook/entries/related-entry` (if it had one)
  - Added: YYYY-MM-DD
  - Completed: YYYY-MM-DD
  - Result: `notebook/entries/resulting-entry`
```

### Behaviors

**Adding a todo:**
1. Assign the next available number (incrementing, never reuse - check both TODO.md and DONE.md)
2. If the todo arises from a conversation about an analysis or other notebook entry, add a `Context:` line linking to it
3. Commit:
   ```bash
   git -C notebook add TODO.md && git -C notebook commit -m "todo: add #N - <task name>"
   git -C notebook remote | grep -q origin && git -C notebook push
   ```

**Working on a todo:**
1. Read `notebook/TODO.md` to find the item
2. If it has a `Context:` link, read that notebook entry for background
3. Make a plan for the task
4. Execute the work
5. When complete, move the entire item to DONE.md with:
   - All original fields preserved
   - `Completed:` date added
   - `Result:` link if the work created a notebook entry
6. Commit:
   ```bash
   git -C notebook add TODO.md DONE.md && git -C notebook commit -m "todo: complete #N - <task name>"
   git -C notebook remote | grep -q origin && git -C notebook push
   ```

**Editing a todo:**
1. Update the description or context as needed
2. Commit:
   ```bash
   git -C notebook add TODO.md && git -C notebook commit -m "todo: update #N - <brief change>"
   git -C notebook remote | grep -q origin && git -C notebook push
   ```

**Deleting a todo (without completing):**
1. Remove the item entirely (don't move to DONE.md)
2. Commit:
   ```bash
   git -C notebook add TODO.md && git -C notebook commit -m "todo: remove #N - <reason>"
   git -C notebook remote | grep -q origin && git -C notebook push
   ```

### Integration with Notebook

Most completed todos should result in a notebook entry. The `Result:` link in DONE.md connects the task to its entry for traceability.

## Feedback Logging

Log feedback about Claude's behavior to `notebook/feedback/`. This feedback is for propagating improvements back to the claude-config repository (skills, settings, CLAUDE.md instructions). It is **not indexed** in INDEX.md - it's separate from project memories.

### When to Suggest Logging Feedback

Proactively offer to log feedback when:
- You didn't detect that the user wanted a skill (they had to invoke it manually or mention it)
- Your first approach to an O2/remote task failed
- User signals skepticism ("hmm", hesitation, correction)
- You failed to retrieve a relevant memory and the user had to point to it
- You failed to create a memory and the user had to ask explicitly

Phrasing: "Would you like to log feedback about this for future improvement?"

### How to Log Feedback

1. Create `notebook/feedback/YYYY-MM-DD-brief-description.md`
2. Content is freeform - whatever the user wants to capture
3. Commit:
   ```bash
   git -C notebook add feedback/ && git -C notebook commit -m "feedback: <brief description>"
   git -C notebook remote | grep -q origin && git -C notebook push
   ```

No template, no index - just capture the feedback and commit.
