---
name: defer
description: Ergonomic notebook-backed TODO capture for deferred work. Invoke manually with /defer when the user asks to defer, save, park, or add a TODO for later work while preserving the current conversation context.
recommended_scope: global
disable-model-invocation: true
---

# Defer

Invoke this skill manually with `/defer`. Do not rely on automatic trigger behavior.

Turn the user's deferred work into a notebook TODO plus a context entry that a future agent can use without reconstructing the conversation.

## Workflow

1. Identify the deferred task and summarize it as a one-line user-facing TODO title.
2. Spawn a subagent with the current conversation context forked. Give it the one-line task anchor and ask it to use `/notebook-entry` for the context entry.
3. Wait for the subagent, then briefly report the TODO id and entry path.

## Subagent Task

Ask the subagent to edit only the project notebook:

```text
Create a deferred task in the project notebook.

Task: <one-line TODO title>

Use /notebook-entry to create or update the project notebook entry for this deferred task. Use the forked conversation history as the source of truth and ask for no additional context.

Requirements:
- Assume `notebook/` is a subdirectory of the active project root.
- After the entry exists, add a TODO item to `notebook/TODO.md` using the local format. The TODO line should be one-line and user-facing, with a `Context:` link to the entry.
- Read `$CONFIG_REPO/templates/todo-reference.md` if you need the local TODO/DONE format. If `CONFIG_REPO` is unset, derive it with `CONFIG_REPO="$("${CLAUDE_HOME:-$HOME/.claude}/bin/config-agent-tool" repo-dir)"`.
- Commit only the notebook changes.
- Do not implement the deferred task.
- Return the TODO id and entry path.
```

If no notebook exists, say that `init-project` should create one before deferring durable project tasks.
