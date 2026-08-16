# Spell run record

## Durable bundle

For each run create `notebook/results/spell-runs/<date>-<slug>-<run-id>/` in the
project's notebook repository. When the code worktree does not contain `notebook/`,
locate the notebook from the primary checkout. The host may write telemetry there;
the Spell assignment must still prohibit agent-directed writes outside its code
worktree.

Use these names:

1. `assignment.spl` — exact self-contained prompt
2. `run.md` — run ID, timestamps, mode, process directory, branch, base commit,
   command, model override reason, exit status, and supervisor-observed failures
3. `trace/` — CLI trace directory
4. `verbose.log` — raw model log
5. `feedback.edn` — append-only agent feedback
6. `result.md` — returned report or implementation summary
7. `review.md` — Spell self-review, Codex assessment, tests, commit/PR, and outcome

Create or update a notebook entry for significant work and link this bundle.

## CLI invocation

Run from the target code worktree. Quote resolved paths.

Implementation uses the default CLI profile:

```bash
SPELL_FEEDBACK_PATH="$RUN/feedback.edn" bin/spell \
  --trace-dir "$RUN/trace" --log "$RUN/verbose.log" \
  "$RUN/assignment.spl"
```

Research uses the read-only profile:

```bash
SPELL_FEEDBACK_PATH="$RUN/feedback.edn" bin/spell \
  --trace-dir "$RUN/trace" --log "$RUN/verbose.log" \
  -a config/agent-profiles/explore.agent.edn "$RUN/assignment.spl"
```

Omit `-m` by default. If `bin/spell` fails because `clj` requires `rlwrap`, retry
the same arguments with `clojure -M:run` and record the wrapper failure. Do not
silently fall back from a model/provider failure.

## Required implementation prompt clauses

Include all of these in the initial assignment:

1. Work only inside the current worktree. Do not change anything outside it.
2. Do not create/switch branches, stage/commit files, push, or open a PR. Codex
   owns Git state. Read-only Git inspection is allowed.
3. Implement the stated acceptance criteria and run relevant tests.
4. Self-improve while doing the main task: use `feedback/log` for concrete bugs,
   friction, ideas, or documentation gaps encountered; do not hunt for issues.
5. After implementation and tests, spawn a fresh-context read-only reviewer to
   inspect the full cumulative diff against the assignment. Address actionable
   findings, rerun affected tests, and report findings plus resolutions.

For a later run on the same PR, append the cumulative diff state, accepted prior
decisions, unresolved review findings, and the new bounded assignment.

## Required research prompt clauses

State that the agent is read-only, must return its report in the response, and may
use feedback for concrete dogfood observations. Specify the exact questions,
evidence standard, desired citations or file references, and relevant boundaries.
