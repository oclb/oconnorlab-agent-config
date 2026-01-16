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
| `NewUser` | `true` | When `true`: Be proactive about explaining Claude Code features and capabilities. Consider invoking the `/help` skill when the user might benefit. Offer brief explanations of relevant skills as they come up. When `false`: Assume the user is familiar with the system and focus on efficient task execution. |

### NewUser Onboarding Behavior

When `NewUser=true`, guide users through the system naturally as you work:

1. **Mention relevant skills** - When a task matches a skill, briefly note it exists (e.g., "I'll analyze this data. By the way, `/perform-analysis` provides a structured 8-step framework for this kind of work.")

2. **Offer context after tasks** - Occasionally ask "Would you like me to explain what I did?" or suggest "Type `/help` to see all available skills."

3. **Suggest help when appropriate** - If the user seems unsure or asks open-ended questions, consider invoking the help skill to orient them.

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

## Logging Methodological Changes

When you make changes to the codebase (not analyses, but the actual software/methods being developed), log them to `notebook/methods/`.

### What to Log

Log all meaningful changes to the codebase:
- Feature additions (major and minor)
- Algorithm or methodology changes
- Bug fixes
- API changes or interface modifications
- Performance improvements
- Refactoring with rationale

Even minor changes get logged - the notebook is the complete record. The filtering happens when deciding whether to update CLAUDE.md, not here.

### Log Format

Create `notebook/methods/YYYY-MM-DD-<brief-description>.md`:

```markdown
# <Brief Description>

**Date:** YYYY-MM-DD
**Commit:** <commit hash if available>

## What Changed
[Description - more detail than a commit message]

## Why
[Motivation for the change]

## Impact
[What this affects - analyses, outputs, compatibility]
```

**Commit the entry:**
```bash
mkdir -p notebook/methods
git add notebook/methods/
git commit -m "methods: <brief description>"
```

### Updating Project CLAUDE.md

After logging a method change, consider whether the project's CLAUDE.md needs updating.

**Ask the user** if the change seems significant:
- New capability that affects how the project works
- Breaking change that affects existing workflows
- Important finding about the method's behavior

**Don't ask** for clearly minor changes:
- Small bug fixes
- Trivial features
- Incremental improvements
- Routine maintenance

When in doubt about importance, lean toward not asking - the notebook entry provides the record.

## Persistent To-Do List

Track tasks across sessions in `notebook/TODO.md`. This complements the session-scoped TodoWrite tool with persistent storage.

### When to Use

- User explicitly asks to "add a todo", "remember to do X", "log this task"
- User asks to "show todos", "what's on my list", "what needs doing"
- User asks to "work on todo #N" or "do the X task"

### File Format

```markdown
# To-Do List

## Active

- [ ] #1 **Task name** - Brief description
  - Context: `notebook/analyses/related-analysis` (if applicable)
  - Added: YYYY-MM-DD

- [ ] #2 **Another task** - Description
  - Added: YYYY-MM-DD

## Completed

- [x] #0 **Example task** - What was done
  - Completed: YYYY-MM-DD
  - Result: `notebook/analyses/resulting-analysis` (if applicable)
```

### Behaviors

**Adding a todo:**
1. Assign the next available number (incrementing, never reuse)
2. If the todo arises from a conversation about an analysis or other notebook entry, add a `Context:` line linking to it
3. Commit: `git add notebook/TODO.md && git commit -m "todo: add #N - <task name>"`

**Working on a todo:**
1. Read `notebook/TODO.md` to find the item
2. If it has a `Context:` link, read that notebook entry for background
3. Make a plan for the task
4. Execute the work
5. When complete, move the item to the Completed section with:
   - `Completed:` date
   - `Result:` link if the work created a notebook entry (analysis, method, etc.)
6. Commit: `git add notebook/TODO.md && git commit -m "todo: complete #N - <task name>"`

**Editing a todo:**
1. Update the description or context as needed
2. Commit: `git add notebook/TODO.md && git commit -m "todo: update #N - <brief change>"`

**Deleting a todo (without completing):**
1. Remove the item entirely (don't move to Completed)
2. Commit: `git add notebook/TODO.md && git commit -m "todo: remove #N - <reason>"`

### Integration with Notebook

Most completed todos should result in a persistent record:
- Analysis task → `notebook/analyses/` entry via `/perform-analysis`
- Method change → `notebook/methods/` entry
- Software setup → `notebook/software/` entry
- Data exploration → `notebook/data/` entry

The `Result:` link in completed todos connects the task to its outcome, creating a traceable history.
