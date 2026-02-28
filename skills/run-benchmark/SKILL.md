---
name: run-benchmark
description: "Run and analyze Spell benchmarks. Use when the user asks to: run a benchmark, test for regressions, compare Spell vs Claude Code, re-run after changes, analyze benchmark results, investigate failures, or any task involving GSM8K, MATH, AIME, BABILong, Omni-MATH, Exercism, SWE-bench, or the orchestration benchmark. Also triggers for 'check if X broke anything', 'how does Spell compare to CC', or 'run the exercism suite'."
---

# Run Benchmark

Run Spell benchmarks, compare against baselines, and investigate results. Four harnesses exist — see [references/harness-inventory.md](references/harness-inventory.md) for exact CLI flags and dataset options.

## Workflow

### 1. Plan the run

Clarify with the user (or infer from context):
- **Which benchmark?** Math (GSM8K, MATH, AIME, Omni-MATH), long-context (BABILong), coding (Exercism, SWE-bench), or orchestration
- **Why?** Regression test after changes, new benchmark, comparison run, or re-investigation
- **Scope?** Pilot (small N) or full run. Default to pilot first.
- **Comparison?** Spell-only, or Spell vs Claude Code (vs baseline)

For regression tests after language changes, run the benchmark suite that exercises the changed feature. Common pairings:
- Prompt/system prompt changes → GSM8K + MATH (fast, sensitive to prompt)
- Parser/eval changes → BABILong + Exercism (exercise code generation paths)
- New builtins/macros → re-run the benchmark where the gap was identified
- Broad changes → GSM8K + MATH Easy + BABILong 16k (the "smoke test" trio)

### 2. Run pilot first

Always start with a small subset (5-10 items) unless the user specifies otherwise. This catches configuration issues early before spending budget.

```bash
# Example: math pilot
cd benchmarking && uv run run_benchmark.py --dataset gsm8k --condition spell --n 10

# Example: exercism pilot
clj -M:dev -m exercism-bench run --difficulty 4-5 --limit 8
```

### 3. Live triage during long runs

For runs longer than ~5 minutes, don't wait for completion. Use sub-agents to inspect results as they come in:

- **Spawn a background triage agent** that tails the output directory for new result files
- The triage agent reads verbose logs / trace files for completed items and flags issues immediately
- Meanwhile, the main agent continues monitoring the run

This enables early detection of systematic failures (e.g., a broken builtin causing every item to error) so the run can be stopped and fixed rather than burning budget.

### 4. Investigate results by priority

After the run completes (or as results stream in), investigate in this order:

| Priority | Condition | Why |
|----------|-----------|-----|
| **P1** | Spell errors (crashes, timeouts, parse failures) | Bugs in the language — always fixable |
| **P2** | Spell wrong, Claude Code right | Spell's approach failed where iterative tool-use succeeded — reveals capability gaps |
| **P3** | Spell right, Claude Code wrong | Spell's approach worked where CC didn't — evidence of Spell's value |
| **P4** | Both wrong | Hard problems — useful for understanding difficulty frontier |
| **P5** | Both right | Lowest priority — confirms things work, check cost/latency differences |

For each investigated item:
- Read the verbose/trace log to understand what Spell actually did
- For errors: identify root cause category (parse error, unbound symbol, timeout, API failure, semantic)
- For wrong answers: was the approach sound but execution failed, or was the approach itself wrong?
- For Spell-vs-CC differences: what did CC's iterative tool-use do that Spell's code-generation didn't (or vice versa)?

### 5. Report results

Use the standard format. Denominator is always total items — errors count as wrong:

```
X% (correct/total) — N errors, M wrong
```

For comparison runs, use a table:

```markdown
| Runner | Accuracy | Errors | Wrong | Cost | Median Latency |
|--------|----------|--------|-------|------|----------------|
| Spell  | 90% (27/30) | 1 | 2 | $20.14 | 31s |
| CC     | 100% (30/30) | 0 | 0 | $7.44 | 37s |
```

Include error categorization when there are failures:

```markdown
### Error Breakdown
- 2 parse errors (large string in continuation)
- 1 timeout (exceeded 120s budget)
- 1 unbound symbol (effect guard scoping)
```

### 6. Fix and re-run (if needed)

When P1/P2 investigation reveals fixable issues:
1. Fix the root cause (prompt, builtin, parser, etc.)
2. Re-run **only the failed items** to verify the fix (use `--items` or `--slug` flags)
3. Then re-run the full suite to confirm no regressions

### 7. Document in notebook

Create a notebook entry for benchmark runs that produce meaningful results. Include:
- Benchmark name, dataset, model, sample size
- Results table with accuracy, cost, latency
- Error categorization
- Comparison table (if applicable)
- What changed since last run (if regression test)
- Findings and any fixes applied

Entry naming convention: `YYYY-MM-DD-{benchmark}-{context}`, e.g. `2026-02-12-math-benchmark-regression-v2`, `2026-02-13-exercism-benchmark-pilot`.

## Harness Quick Reference

| Benchmark | Harness | Invoke | Datasets |
|-----------|---------|--------|----------|
| GSM8K, MATH, AIME, Omni-MATH | Python | `cd benchmarking && uv run run_benchmark.py` | `gsm8k`, `math_easy`, `math_hard`, `aime_2025`, `omni_math` |
| BABILong | Python | `cd benchmarking && uv run run_benchmark.py` | `babilong` (auto-selects io agent) |
| SWE-bench | Python | `cd benchmarking && uv run run_swebench.py` | `mini` (50), `lite` (300), `verified` (500) |
| Exercism | Clojure | `clj -M:dev -m exercism-bench run` | Exercism Python (129 exercises, d1-d9) |
| Orchestration | Clojure | `clj -M:dev -m benchmark run` | 9 orchestration prompts |

See [references/harness-inventory.md](references/harness-inventory.md) for full flag reference and example invocations.

## Best Practices

- **Pilot first.** 5-10 items catches most issues. Don't burn $20 on a full run that fails systematically.
- **Use `--dry-run`** (Python harness) to verify config before committing to a run.
- **Save verbose logs.** All harnesses save per-item logs. These are essential for post-hoc investigation.
- **Track cost.** Report total cost and cost-per-item. Budget flags prevent runaway spending.
- **Compare apples to apples.** Same model, same items, same retry policy. Use `--items` to re-run exact subsets.
- **Errors are wrong answers.** The denominator is always total items attempted, never "items that ran without errors."
- **Use `uv` for all Python work.** Always use `uv run` (not `python3`) to invoke Python scripts and `uv pip` for package management. The Python harnesses are invoked via `uv run run_benchmark.py`, etc.
