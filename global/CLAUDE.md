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

## Notebook System

The notebook is a **separate git repository** inside the project directory, gitignored from the main repo. This keeps the main repo clean while preserving full notebook history.

### Structure

```
notebook/
├── .git/                 # Separate repo
├── INDEX.md              # Entry index for retrieval (Explore agents start here)
├── TODO.md               # Active tasks
├── DONE.md               # Completed tasks
└── entries/              # All memories (analyses, features, research, discussions, etc.)
    └── YYYY-MM-DD-<slug>.md
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
**Author:** Claude
**User:** <github-username>

## Summary
[One paragraph: what was done and why]

## Details
[The work itself - can be updated as work progresses]

## References
- `<entry-name>`: <why it was useful>
```

**Author** is always "Claude" (the agent creating the entry). **User** is the GitHub username from `git config user.name` - this identifies who was working with Claude when the entry was created.

The **References** section records which previous entries informed this work and why. Examples:
- `ssh-socket-validation`: debugging pattern for stale sockets
- `o2-job-template`: SLURM script structure used as starting point
- `variant-filtering-v2`: statistical approach for handling missingness

### Example Entries

**Analysis entry:**
```markdown
# Gene Expression Batch Effect Analysis

**Date:** 2026-01-15
**Author:** Claude
**User:** jsmith

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
**Author:** Claude
**User:** jsmith

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
**Author:** Claude
**User:** jsmith

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

First, get the current user's GitHub username:
  git config user.name

Based on the conversation above, create/update the entry with:
1. A descriptive title and slug (if new)
2. Metadata: Date, Author (always "Claude"), User (the GitHub username from above)
3. Summary: one paragraph of what was done and why
4. Details: the substantive work - decisions made, code written, issues resolved, findings
5. References: link to any related entries that were consulted

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

### Cross-User Entry Attribution

When referencing notebook entries, check the **User** field. If the entry was created by a different user than the current one (compare with `git config user.name`), mention this explicitly:

> "Based on the analysis in `batch-effect-analysis` (created with user jsmith)..."

This alerts the current user that they may not be familiar with this context, since the work was done in a session they weren't part of. This is especially important for:
- Decisions or architectural choices the current user didn't participate in
- Findings that affect current work but originated elsewhere
- References to code or artifacts the user may not have seen

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

Log feedback about Claude's behavior to the config repository's `feedback/` directory. This ensures feedback is collected in one place for contribution back to the project.

To find the config repo: `readlink ~/.claude/CLAUDE.md | xargs dirname | xargs dirname` (resolves symlink to find repo root).

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

1. Find the config repo: `CONFIG_REPO=$(readlink ~/.claude/CLAUDE.md | xargs dirname | xargs dirname)`
2. Get the current user's GitHub username: `git config user.name`
3. Create `$CONFIG_REPO/feedback/YYYY-MM-DD-brief-description.md` with the user attribution at the top:
   ```markdown
   **User:** <github-username>

   <freeform feedback content>
   ```
4. Commit:
   ```bash
   git -C "$CONFIG_REPO" add feedback/ && git -C "$CONFIG_REPO" commit -m "feedback: <brief description>"
   git -C "$CONFIG_REPO" remote | grep -q origin && git -C "$CONFIG_REPO" push
   ```

No index file needed - just capture the feedback with user attribution and commit.
