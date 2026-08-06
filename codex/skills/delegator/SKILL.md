---
name: delegator
description: Manually invoked project notebook-todo dispatcher. Use only when Luke explicitly invokes $delegator to list project notebook todos or dispatch selected todos into separate user-owned Codex tasks; never use for Todoist, generic planning, or automatic task delegation.
recommended_scope: global
disable-model-invocation: true
---

# Delegator

Invoke only through explicit `$delegator`. Stay the coordinator: listing a todo never authorizes doing or dispatching it.

## List Todos Immediately

1. Resolve the current project root and its nested `notebook/`. If it is absent, stop with `No notebook: run $init-project first`.
2. From the project, run `"${CODEX_HOME:-$HOME/.codex}/bin/todo" list --json`. Read every record in `active`; these are notebook todos, never Todoist tasks. Read each existing `context` entry.
3. Compactly present the complete active list. Preserve `#<id>`, title, description, and the useful part of the context entry. Then wait for Luke to choose; do not implement anything.

## Reconcile Before Dispatch

Refresh the todo JSON, then use `list_threads` to reconcile active, completed, and already-dispatched work. Match by todo id first, including pinned titles beginning `TODO #<id>:`; inspect plausible matches with `read_thread`. Keep this dispatch ledger in the coordinator task and report existing ownership concisely. Never create a duplicate for an owned or completed todo.

Identify artifacts each selected todo may write. If an existing task owns the same document, artifact, branch, or lock, use `send_message_to_thread` to add the requirement to that owner instead of creating a conflicting writer. Tell Luke which requirement was routed where.

## Create One User-Owned Task Per Selected Todo

For every unowned selection:

1. Use `list_projects`, then `create_thread`; never use a subagent. For Git repositories, default to `environment.type: worktree`. Use the saved project directly only when Luke requests it or isolation is inapplicable.
2. Make the prompt self-contained: include the exact todo record; full linked context entry; applicable project instructions; project, notebook, and relevant artifact paths; known worktrees, branches, PRs, and ownership; Luke's requested finish line; and explicit scope boundaries. Ask repository tasks to validate their work. Do not invent requirements.
3. If Luke requested autonomous completion, require implementation and validation, then review by a separate fresh-context reviewer with no implementation history. Require iteration on its actionable findings and final revalidation. Only merge, publish, send, or take another consequential final action when Luke's finish line authorizes it.
4. As soon as `create_thread` returns a ready `threadId`, call `set_thread_title` with `TODO #<id>: <todo title>` and `set_thread_pinned` with `pinned: true`; verify both through `list_threads`, then use `wait_threads` for a compact initial progress snapshot.
5. A queued `clientThreadId` is not a ready task id. Record it as pending and keep the coordinator turn open. Do not pass it to ready-task tools. Reconcile through `list_threads` until the resulting `threadId` appears, then title and pin it. If setup takes more than 60 seconds, give Luke a concise progress update while continuing.
6. Only after every created task is titled and pinned, return its `::created-thread{threadId="..."}` directive (or the original `clientThreadId` directive when required for queued-setup UI), plus a compact mapping from todo id to created or owning task.

Continue coordinating follow-up requirements through the owning tasks. Re-run reconciliation before every later dispatch request.
