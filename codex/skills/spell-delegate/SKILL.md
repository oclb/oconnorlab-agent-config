---
name: spell-delegate
description: Delegate bounded one-shot implementation, research, or analysis tasks to a local Spell CLI agent while Codex owns Git state, review, notebook provenance, and user communication. Use when independent Spell exploration or implementation would materially help, including dogfooding Spell itself.
---

# Spell Delegate

Use Spell as a one-shot worker and Codex as supervisor. Do not create a persistent
intercom or assume that a later Spell run remembers an earlier one.

Before invoking Spell, read [references/run-record.md](references/run-record.md)
for the run bundle, command forms, and required prompt clauses.

## Choose the delegation

1. Use an **implementation run** for bounded code or documentation changes.
2. Use a **research run** for substantial feature exploration, proposal analysis,
   or data gathering where an independent report will help.
3. Skip delegation when the needed information is narrow and direct inspection is
   faster. Codex may always inspect files or research independently to verify or
   extend Spell's work.

Use the CLI's default model by omitting `-m`. Override it only when the user asks,
the default fails, or the chosen transport is incompatible; record the reason.

## Codex owns the boundary

Codex owns user alignment, Git state, the notebook, validation, acceptance, PRs,
and merge approval. Spell owns only its explicit assignment.

For implementation, Codex must:

1. Inspect the repository and notebook, preserve the primary checkout, and create
   an isolated branch and worktree from the requested base.
2. Invoke Spell with that worktree as its process directory. Instruct it to edit
   only that worktree and never create/switch branches, add/commit files, push, or
   open a PR. Read-only Git inspection is allowed.
3. Give exact scope and acceptance criteria. Enable feedback and durable tracing.
4. Inspect the resulting diff and Git status, run proportionate checks, and decide
   whether to accept, revise, or discard the work. Spell's report is not evidence
   that the task is complete.

Multiple one-shot runs may contribute to one PR. Each new assignment must restate
the original requirements, current cumulative state, relevant prior findings, and
the remaining task; never rely on conversational memory.

## Require two review rounds

Every implementation assignment must tell the implementing Spell agent to run a
fresh-context Spell reviewer after making and testing the changes, then address
actionable findings and report both findings and resolutions. The reviewer should
use the read-only `explore` agent when available.

If the changes advance to a PR, Codex must start its own fresh-context reviewer
against the actual PR before merge. Reconcile actionable findings, revalidate the
final diff, and merge only with the user's explicit approval. This Codex-side PR
review is required even if several Spell implementation runs reviewed themselves.

## Research runs

Use `config/agent-profiles/explore.agent.edn`: it exposes `io-read`, web, and
feedback, but no file-write or process-execution functions. Ask for a report in
the response; Codex saves it in the run bundle. Do not ask the research agent to
create artifacts or modify the notebook.

Research delegation is especially useful for initial exploration of a significant
feature or proposal. Codex remains responsible for checking claims and deciding
what reaches the user.

## Run synchronously without becoming unavailable

Default to synchronous supervision: start one process, retain its session handle,
and do not accept the task until it exits and its artifacts are reviewed. Use a
bounded initial yield (normally 30 seconds); if the process is still running, poll
the same session in bounded intervals and provide concise progress updates.

"Synchronous" describes acceptance, not UI exclusivity. A yielded process can keep
running while Codex receives a user message or does other useful work. Use an
explicit asynchronous run when parallel work is valuable, then retain and poll its
session. A one-shot run cannot receive a mid-run message: queue additive context
for the next run, or terminate/ignore the result if new direction invalidates it.

## Preserve provenance

Codex creates and curates the run bundle and durable notebook entry. Link the
assignment, trace, feedback, result, worktree/base commit, resulting commit or PR,
Spell self-review, Codex review, and validation. Use feedback `:agent-handle` and
`:trace-node-id` to locate the exact response to a logged observation. If failure
occurs before the agent can log feedback, record it as a supervisor-observed issue.

Classify dogfood findings as model/agent behavior, Spell robustness, CLI friction,
or documentation. Do not turn raw feedback into an issue or code change without
Codex triage.
