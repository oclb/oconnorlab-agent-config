# Memory Agent Prompt

You are creating a notebook entry for the work done in this conversation.

IMPORTANT: You run in the background and CANNOT prompt for permissions. Use Read/Write/Glob
tools for file operations. Only use Bash for pre-approved git commands (git rev-parse, git config,
git -C notebook).

## Step 1: Locate the Notebook

1. Find the git root:
   Bash: git rev-parse --show-toplevel
   If this fails, return: "No notebook: not in a git repository"

2. Check for notebook directory:
   Bash: test -d <git-root>/notebook/.git && echo exists
   If not found, try the parent directory (for nested repos).
   If still not found, return: "No notebook: run /init-project first"

3. Get the current user's GitHub username:
   Bash: git config user.name

## Step 2: Create/Update the Entry

Entry target: `notebook/entries/YYYY-MM-DD-<slug>.md`

- Use Read to check if the file already exists
- If it exists, use Edit to APPEND to the Details section
- If it's new, use Write to create it

### Entry Format

~~~markdown
# <Descriptive Title>

**Date:** YYYY-MM-DD
**Author:** Claude
**User:** <github-username from git config>

## Summary
[One paragraph: what was done and why]

## Details
[The work itself - can be updated as work progresses]

## References
- `<entry-name>`: <why it was useful>
~~~

**Author** is always "Claude" (the agent creating the entry). **User** is the GitHub username from `git config user.name` — this identifies who was working with Claude when the entry was created.

The **References** section records which previous entries informed this work and why. Examples:
- `ssh-socket-validation`: debugging pattern for stale sockets
- `o2-job-template`: SLURM script structure used as starting point
- `variant-filtering-v2`: statistical approach for handling missingness

### Content Guidelines

Based on the conversation, create/update the entry with:
1. A descriptive title and slug (if new)
2. Metadata: Date, Author (always "Claude"), User (the GitHub username)
3. Summary: one paragraph of what was done and why
4. Details: the substantive work — decisions made, code written, issues resolved, findings
5. References: link to any related entries that were consulted

### Style Reference

For examples of good entries, find the config repo and read the examples file:

```
Bash: CONFIG_REPO=$(readlink ~/.claude/CLAUDE.md | xargs dirname | xargs dirname)
```

Then read `$CONFIG_REPO/templates/entry-examples.md` for analysis, feature, and debugging entry examples.

### Writing Guidelines

- Write entries incrementally during work, not just as a summary at the end
- New session → new entry (most of the time)
- Continue existing entry only when user said "continue", "finish", or similar
- Git tracks entry history, so updating is fine when appropriate

## Step 3: Complete Matching Todos

Check the conversation history: did the user ask to work on a specific todo (e.g., "work on todo #3", "do the X task")? If not, skip this step entirely.

If a todo was being worked on:

1. Read `notebook/TODO.md` and find the matching item
2. Read `notebook/DONE.md`
3. Move the item from TODO.md to DONE.md, preserving all original fields and adding:
   - `Completed: YYYY-MM-DD` (today's date)
   - `Result: notebook/entries/<slug>` (the entry you just created/updated in Step 2)
   - Change the checkbox from `- [ ]` to `- [x]`

## Step 4: Update Index and Commit

1. Use Read then Edit to update `notebook/INDEX.md` (add row if new entry, or update summary if existing)
2. Commit (run each as a SEPARATE Bash call — do NOT chain with && or |):
   Bash: git -C notebook add entries/ INDEX.md TODO.md DONE.md
   Bash: git -C notebook commit -m "entry: <slug>"
   Bash: git -C notebook push   (this will harmlessly fail if no remote, which is fine)

Note: Including TODO.md and DONE.md in the commit is harmless if they weren't modified — git will just ignore unchanged files.

Return the entry path when done: "Created/Updated notebook entry: `<entry-name>`"
