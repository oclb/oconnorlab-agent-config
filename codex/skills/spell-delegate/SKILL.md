---
name: spell-delegate
description: Delegate a bounded implementation task to a local Spell CLI agent. Use when implementing features within Spell itself.
---

# Spell Delegate

Use Spell as a one-shot worker under Codex supervision. Later runs do not remember
earlier ones or the user conversation. Omit `-m` for the default model unless the
user requests otherwise or it fails; record why an override was necessary.

Codex owns user alignment and planning, Git state, the notebook, validation,
acceptance, PRs, and merge approval. Spell owns only its explicit assignment.
Before implementation, align with the user on scope and the big-picture approach.
If the user asks for a plan, make it detailed enough to name the files to change
and pass the approved plan verbatim to Spell.

## Prepare the run

1. Inspect the repository and notebook. Preserve the primary checkout and create
   an isolated branch and worktree from the requested base.
2. Create `notebook/results/spell-runs/<date>-<slug>-<run-id>/`. When the code
   worktree lacks `notebook/`, locate it from the primary checkout.
3. Create this run bundle:
   - `assignment.spl`: exact self-contained prompt.
   - `run.md`: run ID, timestamps, process directory, branch, base commit, exact
     command, model override reason, exit status, and supervisor-observed failures.
   - `trace/`: CLI trace.
   - `verbose.log`: raw model log.
   - `feedback.edn`: append-only Spell feedback.
   - `result.md`: returned implementation report.
   - `review.md`: Spell self-review, Codex assessment, validation, commit or PR,
     and outcome.
4. Run Spell from the code worktree. Host telemetry may go to the notebook bundle;
   the agent writes only in its worktree.

Example, with all standard delegation features enabled:

```bash
SPELL_FEEDBACK_PATH=/absolute/notebook/results/spell-runs/2026-08-16-example-001/feedback.edn \
bin/spell --dogfood --agents-md \
  --budget 20 \
  --trace-dir /absolute/notebook/results/spell-runs/2026-08-16-example-001/trace \
  --log /absolute/notebook/results/spell-runs/2026-08-16-example-001/verbose.log \
  /absolute/notebook/results/spell-runs/2026-08-16-example-001/assignment.spl
```

`--dogfood` activates self-improvement feedback for the main agent and workers.
`--agents-md` puts the worktree-root `AGENTS.md` into the prompt, capped at 32 KiB.
Explicit paths make trace, log, and feedback durable. Use `--budget 20` normally and
`--budget 100` for complex work such as broad cross-module changes or long test
and review cycles. These are ceilings, not spending targets.

## Write the assignment

Include the user's motivation, preferences, decisions, exact scope, acceptance
criteria, and approved plan. Always include these instructions:

1. Work only inside the current worktree. Do not change anything outside it.
2. Do not create or switch branches, stage or commit files, push, or open a PR.
   Codex owns Git state; read-only Git inspection is allowed.
3. Implement the acceptance criteria and run relevant tests.
4. While doing the task, use `feedback/log` for concrete Spell bugs, friction,
   ideas, or documentation gaps encountered; do not divert into issue hunting.
5. After implementation and tests, delegate a fresh-context reviewer with the
   original assignment. Have it inspect the cumulative diff, address actionable
   findings, and rerun affected tests.
6. Return a summary of changes; tests; reviewer findings and their patches or
   explanations for non-patches; feedback items; and deviations from the
   assignment. After a clean re-review, return this report immediately instead of
   repeating already-passing test suites. If review does not pass, say so plainly.

Multiple one-shot Spell runs may contribute to one PR. Every later assignment
must restate the original requirements, cumulative state, accepted decisions,
prior findings, and remaining task. If patches are needed after a run, Codex or a
Codex subagent normally makes them unless the user asks for another Spell run.

## Supervise the process

Default to synchronous acceptance, not a permanently blocked interface:

1. Start Spell with an initial yield of about 30 seconds. If it exits, start and
   result cost one server-to-computer round-trip.
2. If it is still running, the tool returns a session ID while the process keeps
   running on the computer. Each later poll is another round-trip. Poll about
   once a minute for straightforward work. For complex work, inspect progress
   roughly every five minutes using short polls; never make one five-minute
   blocking wait. Report progress after ten minutes.
3. Between polls, Codex can respond to the user or do independent work while
   Spell continues. A wait already in flight must return before Codex can act in
   this task.
4. Use an explicitly asynchronous start—a very short initial yield—when parallel
   work is immediately useful. It is the same retained process and still requires
   later polling, review, and acceptance.

Spell's CLI depth default is unlimited, so omit `--depth` unless a lower safety
cap is justified. A tool yield or poll interval is not a process timeout. Do not
set an overall timeout by default. If the execution environment requires one,
use judgment and scale it with the task: allow at least one hour for a standard
$20 run and four hours for a complex $100 run, extending it when trace evidence
shows useful progress.

A one-shot run cannot receive mid-run instructions. Put additive context in the
next run. If new direction invalidates the current task, terminate it or ignore
its result rather than treating stale work as accepted.

## Review and preserve provenance

Inspect Git status and the full diff, run proportionate validation, and decide
whether to accept, revise, or discard the work. A confident Spell report is not
completion evidence. If the report is absent, confused, implausibly confident, or
otherwise suspicious, inspect the trace and logs; delegate that audit to a Codex
subagent when useful.

Every implementation has Spell's fresh-context review. If changes advance to a
PR, run a separate fresh-context Codex review against the actual PR, reconcile
findings, revalidate, and merge only with explicit user approval.

For accepted commits, add `Spell-Run: <run-id>` for each contributing run and
follow the global `Notebook-Entry` convention when applicable. Put Codex review
provenance in the PR description or equivalent record.

Create or update the durable notebook entry with the assignment, trace, feedback,
result, worktree, base and resulting commits, reviews, and validation. Use
`:agent-handle` and `:trace-node-id` to locate the exact response behind each
observation. Record earlier failures as supervisor-observed. Classify findings as
model/agent behavior, Spell robustness, CLI friction, or documentation; Codex
triages them.

Relay the result to the user with a one-to-three-sentence TL;DR and, if incomplete,
a one-to-two-sentence recommended next step. If the user says “continue,” follow
that recommendation.
