---
name: notebook-entry
description: Create or update durable project notebook entries. Use whenever Codex creates notebook memory, especially from a forked subagent.
recommended_scope: global
---

# Notebook Entry

Use this skill to create durable project memory in the active project's `notebook/` repo. If invoked in a subagent, use the forked conversation history as the source of truth and ask for no additional context. Work autonomously; if blocked by missing permissions or unavailable files, report the blocker instead of waiting for the user.

Assume `notebook/` is a subdirectory of the active project root. If `notebook/.git` is missing, stop with: `No notebook: run /init-project first`.

Get the user identity for entry metadata:

```bash
git config user.name
```

## Choose The Entry

Entry target: `notebook/entries/YYYY-MM-DD-<slug>.md`.

- Create a new entry for a new session in most cases.
- Continue an existing entry only when the user said "continue", "finish", or similar, or when the current work is plainly the same ongoing task.
- If the entry exists, append new information to `## Details` instead of replacing earlier notes.
- If creating a new entry, use a short descriptive slug in lowercase hyphen-case.

## Entry Format

```markdown
# <Descriptive Title>

**Date:** YYYY-MM-DD
**Author:** Codex
**User:** <git config user.name>

## Summary
<One paragraph: what was done and why>

## Details
<The substantive work: decisions made, code written, issues resolved, findings, commands/tests worth remembering, and next-step context>

## References
- `<entry-name>`: <why it was useful>
```

Use `Codex` for `Author`. Use the `git config user.name` value for `User`.

## Content Guidelines

- Write enough detail for a future session to resume without reconstructing the whole conversation.
- Include all user-assigned tasks covered by the session, including tasks that were completed, partially completed, explicitly deferred, or left open.
- Preserve user-provided context that shaped the work: constraints, preferences, examples, hypotheses, file paths, commands, outputs, and domain details.
- Record decisions and why they were made, especially tradeoffs, rejected alternatives, and changes from the user's original plan.
- Link relevant artifacts: changed files, plans, PRs, issues, generated outputs, benchmark results, reports, figures, and related notebook entries.
- For implementation changes through the top-level `$work-cycle` skill, include the notebook plan path or link and the PR number or commit hash.
- For long sessions, organize `## Details` chronologically so a future agent can follow how the work evolved.
- Prefer concrete decisions, changed files, test results, gotchas, and unresolved follow-ups over broad narrative.
- Keep a high signal-to-noise ratio: omit routine tool chatter, obvious restatements, transient mistakes with no lasting consequence, and conversational filler.
- Include references only for previous notebook entries that were actually consulted or materially informed the work.
- Keep public-facing documents, private local paths, secrets, credentials, and irrelevant chat texture out of the entry.
- If updating during ongoing work, write incrementally; git history can preserve how the entry evolved.

For style examples, find the config repo from the installed global instructions and read `templates/entry-examples.md` only if needed:

```bash
CONFIG_REPO="$("${CODEX_HOME:-$HOME/.codex}/bin/config-agent-tool" repo-dir)"
```

## Complete Matching Todos

Only do this when the conversation shows the user asked to work on a specific todo, such as "work on todo #3" or "do the X task".

1. Read `notebook/TODO.md` and find the matching item.
2. Read `notebook/DONE.md`.
3. Move the item from TODO to DONE, preserving original fields and adding:
   - `Completed: YYYY-MM-DD`
   - `Result: notebook/entries/<slug>`
   - Change the checkbox from `- [ ]` to `- [x]`

Skip this section entirely when no specific todo was being worked.

## Update Index And Commit

1. Update `notebook/INDEX.md`: add a row for a new entry or update the summary for an existing entry.
2. Commit notebook changes with separate commands, not chained shell commands:

```bash
git -C notebook add entries/ INDEX.md TODO.md DONE.md
git -C notebook commit -m "entry: <slug>"
git -C notebook push
```

Including unchanged `TODO.md` or `DONE.md` is fine. If `git push` fails because no remote exists, report that without treating the entry as failed.

Return exactly one short status line, using the final entry path:

```text
Created/Updated notebook entry: `notebook/entries/YYYY-MM-DD-<slug>.md`
```
