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
| `NewUser` | `true` | When `true`: Be proactive about explaining Claude Code features and capabilities. Consider invoking the `/support` skill when the user might benefit. Offer brief explanations of relevant skills as they come up. When `false`: Assume the user is familiar with the system and focus on efficient task execution. |

### NewUser Onboarding Behavior

When `NewUser=true`, guide users through the system naturally as you work:

1. **Mention relevant skills** - When a task matches a skill, briefly note it exists (e.g., "I'll analyze this data. By the way, `/perform-analysis` provides a structured 8-step framework for this kind of work.")

2. **Offer context after tasks** - Occasionally ask "Would you like me to explain what I did?" or suggest "Type `/support` to see all available skills."

3. **Suggest support when appropriate** - If the user seems unsure or asks open-ended questions, consider invoking the support skill to orient them.

4. **Introduce AFK mode** - When you need to ask multiple questions, mention that `(afk)` mode exists for autonomous work.

5. **Don't overwhelm** - Limit explanations to once per skill/feature per session. After mentioning something, don't repeat it.

**Toggling NewUser mode**: Unlike AFK which uses keywords, NewUser mode changes only when the user explicitly asks (e.g., "I'm comfortable now, turn off onboarding" or "Enable NewUser mode again"). Use sed to update the flag in behavior.conf.

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

### Detecting Notebook Setup

Before any notebook operation, check if the notebook is a separate repo:

```bash
# Check if notebook exists and is a git repo
if [ -d "notebook/.git" ]; then
    # Notebook is separate repo - use git -C notebook
    git -C notebook add ...
    git -C notebook commit -m "..."
    # Push if remote exists
    if git -C notebook remote | grep -q origin; then
        git -C notebook push
    fi
else
    # Notebook not set up - suggest /init-project
    # Or fall back to main repo commits for backward compatibility
fi
```

### Notebook Commit Pattern

For all notebook operations, use this pattern:

```bash
git -C notebook add <files>
git -C notebook commit -m "<message>"
# Push if remote configured
git -C notebook remote | grep -q origin && git -C notebook push
```

## Notebook Index

Maintain `notebook/INDEX.md` as a quick-reference summary of all project memories. This enables faster retrieval than scanning individual files.

### Index Format

```markdown
# Notebook Index

## Analyses
| ID | Summary | Date | Tags |
|----|---------|------|------|

## Methods
| Date | Type | Summary |
|------|------|---------|
```

### When to Update

Update the index whenever you create or modify a notebook entry:
- After completing an analysis → add row to Analyses
- After creating a methods memory → add row to Methods

**Commit with the entry:** Include index updates in the same commit as the notebook entry.

### Using the Index for Retrieval

When starting work that might relate to past memories:
1. Read `notebook/INDEX.md` first (fast scan)
2. Identify relevant entries by ID, tags, or summary
3. Read full entries only for those that seem relevant

This is more efficient than reading every README.md.

## Methods Memory

The `notebook/methods/` directory is the unified memory system for everything except formal analyses. It captures knowledge about the codebase, tools, data, and methodological decisions.

### When to Create a Methods Memory

**Proactively create a memory when you:**

1. **Implement a new feature** - Document what was built and why
2. **Fix a software issue** - Document the bug, root cause, and fix
3. **Touch a new dataset** - Document location, source, characteristics, issues
4. **Discuss a methodological idea** - If a conversation clarifies how something should work, or decides an approach, capture it
5. **Set up or learn a new tool** - Document installation, usage, and any gotchas

**The goal is to not lose knowledge.** If you learned something that would be useful for future work, write it down.

### Log Format

Create `notebook/methods/YYYY-MM-DD-<brief-description>.md`:

```markdown
# <Brief Description>

**Date:** YYYY-MM-DD
**Type:** [feature | bugfix | data | tool | decision]
**Commit:** <commit hash if available>

## Summary
[One paragraph explaining what this is about]

## Details
[Additional context - what was done, why, how]

## Notes
[Any gotchas, limitations, or things to remember]
```

**Type field values:**
- `feature` - New functionality implemented
- `bugfix` - Bug identified and fixed
- `data` - Dataset documented (location, source, characteristics, issues)
- `tool` - External tool or library set up
- `decision` - Methodological decision or approach clarified

**Commit the entry (including index update):**
```bash
mkdir -p notebook/methods
# Add row to notebook/INDEX.md Methods table
git -C notebook add methods/ INDEX.md
git -C notebook commit -m "methods: <brief description>"
git -C notebook remote | grep -q origin && git -C notebook push
```

### Updating Project CLAUDE.md

After creating a methods memory, consider whether the project's CLAUDE.md needs updating.

**Add to CLAUDE.md if:**
- New capability that affects how the project works
- Breaking change that affects existing workflows
- Important finding about the method's behavior
- Key dataset that analyses depend on

**Don't add for:**
- Small bug fixes
- Routine tool installations
- Incremental improvements
- Exploratory decisions that might change

When in doubt, lean toward not adding - the notebook has the record.

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
  - Context: `notebook/analyses/related-analysis` (if applicable)
  - Added: YYYY-MM-DD

- [ ] #2 **Another task** - Description
  - Added: YYYY-MM-DD
```

**notebook/DONE.md** (completed tasks):
```markdown
# Completed

- [x] #0 **Example task** - Original description preserved
  - Context: `notebook/analyses/related-analysis` (if it had one)
  - Added: YYYY-MM-DD
  - Completed: YYYY-MM-DD
  - Result: `notebook/analyses/resulting-analysis`
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

Most completed todos should result in a persistent record elsewhere:
- Analysis task → `notebook/analyses/` entry via `/perform-analysis`
- Everything else → `notebook/methods/` entry (features, bugfixes, data, tools, decisions)

The `Result:` link in DONE.md connects the task to its outcome for traceability.

## Feedback Logging

Log feedback about Claude's behavior to `notebook/feedback/` so skills and configuration can improve over time.

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
