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
| `AFK` | `false` | When `true`: Be more independent. Proceed if 80%+ confident in interpretation. Document assumptions in notebook entries. Only pause for: irreversible actions (destructive git operations, file deletions), external API calls with side effects, or decisions where wrong choice would require significant rework. When pausing, state the specific decision point. |
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

### Example Entries

**Analysis entry:**
```markdown
# Gene Expression Batch Effect Analysis

**Date:** 2026-01-15

## Summary
Investigated unexpected clustering in PCA of RNA-seq data. Identified sequencing batch as primary driver of PC1 (explaining 23% of variance). Recommended ComBat correction before downstream analysis.

## Details
Initial PCA showed samples clustering by an unknown factor rather than treatment group. Systematically tested metadata variables:

| Variable | PC1 correlation | PC2 correlation |
|----------|-----------------|-----------------|
| Batch | 0.89 | 0.12 |
| Treatment | 0.15 | 0.67 |
| RIN score | 0.34 | 0.08 |

Batch correction with ComBat reduced batch-PC1 correlation to 0.11 while preserving treatment signal. QC plots saved to `figures/batch_correction_qc.png`.

**Decision:** Use ComBat-corrected counts for all downstream analyses. Raw counts preserved in `data/raw/` for reproducibility.

## References
- `rnaseq-pipeline-setup`: sample metadata location and format
```

**Feature implementation entry:**
```markdown
# User Authentication with OAuth2

**Date:** 2026-01-10

## Summary
Implemented OAuth2 authentication flow with Google provider. Users can now sign in via Google account, with session persistence via HTTP-only cookies.

## Details
Architecture decisions:
- **Provider:** Google OAuth2 (most users have accounts, good documentation)
- **Session storage:** HTTP-only cookies with 7-day expiry (balances security and UX)
- **Token refresh:** Silent refresh 5 minutes before expiry

Key files modified:
- `src/auth/oauth.ts` - OAuth flow implementation
- `src/middleware/session.ts` - Session validation middleware
- `src/routes/auth.ts` - `/auth/login`, `/auth/callback`, `/auth/logout` endpoints

Edge cases handled:
- Token refresh failure → redirect to login with flash message
- Revoked Google permissions → clear session, prompt re-auth
- Concurrent sessions → allowed (no single-session enforcement)

## References
- `api-route-structure`: followed existing route patterns for consistency
```

**Debugging/research entry:**
```markdown
# SSH Socket Validation Findings

**Date:** 2026-01-17

## Summary
Discovered that ControlMaster `-O check` returns success even for stale sockets. Implemented inode-based validation as reliable alternative.

## Details
Initial approach used `ssh -O check` but this only verifies socket file exists, not connection validity. After testing, found that comparing socket inode before/after connection attempt reliably detects stale sockets.

Validation function:
```bash
validate_socket() {
    local socket="$1"
    local inode_before=$(stat -f %i "$socket" 2>/dev/null)
    ssh -O check -S "$socket" user@host 2>/dev/null
    local inode_after=$(stat -f %i "$socket" 2>/dev/null)
    [[ "$inode_before" == "$inode_after" ]]
}
```

**Root cause:** ControlMaster checks socket file descriptor, not actual TCP connection state. When connection drops, socket file persists until explicitly removed or new connection attempted.

## References
- `o2-connection-setup`: original socket implementation this extends
```

### Writing Entries As You Go

Write entries incrementally during work, not just as a summary at the end. Start the entry when beginning a task, add details as you progress, and finalize when complete.

**Announce entry creation:** When you create an entry, state: "Created notebook entry: `<entry-name>`"

### New Session vs. Continuing Work

- **New session → new entry** (most of the time)
- **Continue existing entry** only when user says "continue", "finish", or similar

Git tracks entry history, so updating is fine when appropriate.

## Memory Creation via Sub-Agent

For every response, consider whether you, or the user, have made a tangible contribution to the project; if so, log it. Tangible contributions include:
- Expressing an idea or explaining a concept or approach
- Giving a new name to something
- Articulating a new goal
- Designing or completing an analysis 
- Making a decision about high-level approach to a problem
- Implementing a new feature
- Setting up new tools or environments
- Discovering or fixing a problem
- Completing an item from the TODO list

**When you determine memory creation is needed**, spawn a background memory agent:

```
Task tool call:
- subagent_type: "general-purpose"
- run_in_background: true
- model: "sonnet"
- prompt: (see template below)
```

**Memory agent prompt template:**

```
You are creating a notebook entry for the work done in this conversation.

Entry target: notebook/entries/YYYY-MM-DD-<slug>.md
- If this file already exists, APPEND to the Details section (the conversation may have continued work on the same topic)
- If this is a new file, create it with the standard format

Based on the conversation above, create/update the entry with:
1. A descriptive title and slug (if new)
2. Summary: one paragraph of what was done and why
3. Details: the substantive work - decisions made, code written, issues resolved, findings
4. References: link to any related entries that were consulted

After writing the entry:
1. Update notebook/INDEX.md (add row if new entry, or update summary if existing)
2. Commit: git -C notebook add entries/ INDEX.md && git -C notebook commit -m "entry: <slug>" && git -C notebook remote | grep -q origin && git -C notebook push

Return the entry path when done: "Created/Updated notebook entry: `<entry-name>`"
```

**After spawning the memory agent:**
- The agent runs in background and returns the entry path when complete
- Echo the result in your response so future turns see what entry was created
- If continuing work in the same session, subsequent memory agents will see the existing entry and can append to it

**What counts as significant work:**
- ✓ Multi-file code changes
- ✓ Debugging sessions that found root causes
- ✓ Analysis with results
- ✓ Tool setup with gotchas discovered
- ✓ Architectural decisions with tradeoffs
- ✗ Simple Q&A
- ✗ Single-line fixes
- ✗ Reading files without action
- ✗ Failed attempts with no learning

**Note:** If memory creation is critical, spawn the agent early in your response rather than at the end, giving it more time to complete before the user might exit.

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

1. Read `notebook/INDEX.md` immediately
2. If user's first message contains pronouns referencing past work ("that", "the", "our"), retrieve relevant entries
3. If planning a task similar to an indexed entry, read that entry first

### Retrieval Triggers

Use **Explore subagent** when ANY of these patterns appear in user messages:
- Demonstrative references: "the analysis", "that script", "what we did", "last time"
- User references a date, filename, or method name that appears in INDEX.md
- User asks to continue, extend, or fix previous work
- You're about to start work that might duplicate an existing entry

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

### Feedback Triggers

**Log feedback automatically (without asking) when:**
- User corrects your interpretation ("no, I meant...", "that's not what I asked")
- User manually invokes a skill you should have auto-detected
- A command fails due to environment or permission issues Claude could have anticipated
- User points to a notebook entry Claude should have retrieved but didn't

**Ask "Would you like to log feedback for future improvement?" when:**
- User expresses hesitation or skepticism ("hmm", "are you sure?", "that doesn't seem right")
- Your first approach to an O2/remote task fails
- User explicitly mentions Claude's behavior could be improved

### How to Log Feedback

1. Create `notebook/feedback/YYYY-MM-DD-brief-description.md`
2. Content is freeform - whatever the user wants to capture
3. Commit:
   ```bash
   git -C notebook add feedback/ && git -C notebook commit -m "feedback: <brief description>"
   git -C notebook remote | grep -q origin && git -C notebook push
   ```

No template, no index - just capture the feedback and commit.
