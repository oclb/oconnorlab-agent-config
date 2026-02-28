# Harness Inventory

Detailed CLI reference for each benchmark harness. Read this when you need exact flags or invocation syntax.

## Table of Contents

- [Python Benchmark Suite](#python-benchmark-suite) (GSM8K, MATH, AIME, BABILong, Omni-MATH)
- [SWE-bench](#swe-bench)
- [Exercism Python](#exercism-python)
- [Orchestration Benchmark](#orchestration-benchmark)

---

## Python Benchmark Suite

**Location:** `benchmarking/`

```bash
cd benchmarking && uv run run_benchmark.py [FLAGS]
```

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--dataset` | string | `longbench_short` | Dataset name or `all` |
| `--condition` | string | `baseline` | `baseline`, `spell`, `claude_code`, or `all` |
| `--model` | string | `anthropic:claude-sonnet-4-20250514` | provider:model spec |
| `--n` | int | all | Number of items |
| `--budget` | float | 1.0 | USD budget per item (Spell only) |
| `--output` | file | `results/{timestamp}.json` | Output path |
| `--dry-run` | flag | — | Print config and exit |
| `--parallel` | int | 5 | Parallel workers |
| `--agent` | file | auto | Agent definition file |
| `--items` | string | — | Comma-separated item IDs |

Auto-agent: datasets starting with `babilong` or `longbench` use `agents/with-io.agent.edn`.

**Results:** `results/{timestamp}.json` with config, metrics, per-item results.

**Examples:**
```bash
# GSM8K pilot
uv run run_benchmark.py --dataset gsm8k --condition spell --n 10 --dry-run
uv run run_benchmark.py --dataset gsm8k --condition spell --n 10

# MATH comparison
uv run run_benchmark.py --dataset math_hard --condition all --n 30

# BABILong regression check
uv run run_benchmark.py --dataset babilong --condition spell --n 8

# Re-run specific failed items
uv run run_benchmark.py --dataset aime_2025 --condition spell --items aime_2025_9,aime_2025_14
```

---

## SWE-bench

**Location:** `benchmarking/`

```bash
cd benchmarking && uv run run_swebench.py [FLAGS]
```

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--dataset` | string | `mini` | `mini` (50), `lite` (300), `verified` (500) |
| `--condition` | string | `both` | `spell`, `claude_code`, or `both` |
| `--model` | string | `anthropic:claude-opus-4-5-20251101` | Spell model |
| `--claude-model` | string | `opus` | CC model alias |
| `--n` | int | all | Number of items |
| `--budget` | float | 10.0 | USD per instance |
| `--timeout` | int | 600 | Seconds per instance |
| `--workspace-root` | dir | `~/.cache/swebench-spell-workspaces` | Repo clone root |
| `--keep-workspaces` | flag | — | Keep repos after run |
| `--output-dir` | dir | `results/swebench` | Output directory |
| `--dry-run` | flag | — | Print config and exit |

**Results:** JSONL (SWE-bench format) + full JSON with metadata in `results/swebench/`.

**Official evaluation:**
```bash
pip install swebench
python -m swebench.harness.run_evaluation \
    --dataset_name princeton-nlp/SWE-bench_Verified \
    --predictions_path results/swebench/spell_verified_*.jsonl \
    --max_workers 8
```

---

## Exercism Python

**Location:** project root

```bash
clj -M:dev -m exercism-bench run [FLAGS]
```

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--model` | string | `sonnet` | Model alias or provider:model |
| `--runner` | string | `spell` | `spell` or `claude-code` |
| `--agent` | file | `agents/with-io-minimal.agent.edn` | Agent definition |
| `--difficulty` | range | all | e.g., `4-5` |
| `--slug` | string | — | Specific exercise(s), repeatable |
| `--limit` | int | all | Max exercises |
| `--depth` | int | 8 | Max LLM depth (Spell) |
| `--budget` | float | 2.0 | USD per exercise (Spell) |
| `--no-retry` | flag | — | Skip retry on failure |
| `--keep` | flag | — | Keep temp directories |

**Model aliases:** `haiku`, `sonnet`, `opus`

**Prerequisites:** First run auto-clones `exercism/python` to `data/exercism-python/`. Uses `uv tool run pytest` internally.

**Results:** `data/exercism-results/{timestamp}/` with `results.edn`, `summary.md`, per-exercise verbose and test logs.

**Examples:**
```bash
# Quick pilot
clj -M:dev -m exercism-bench run --difficulty 1-3 --limit 5

# Spell vs CC comparison on medium exercises
clj -M:dev -m exercism-bench run --difficulty 4-5 --limit 30 --runner spell
clj -M:dev -m exercism-bench run --difficulty 4-5 --limit 30 --runner claude-code

# Re-run specific exercise
clj -M:dev -m exercism-bench run --slug bowling --model opus

# Different model
clj -M:dev -m exercism-bench run --model openai:gpt-5.2 --difficulty 4-5 --limit 10
```

---

## Orchestration Benchmark

**Location:** project root

```bash
clj -M:dev -m benchmark [MODE] [OPTIONS]
```

**Modes:**

### `run` (default)
```bash
clj -M:dev -m benchmark run [--agent FILE] [--no-recover] [prompt-names...]
```

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--agent` | file | default | Agent definition |
| `--no-recover` | flag | — | Disable error recovery |
| prompt names | varargs | all | Specific prompts to run |

### `reanalyze`
```bash
clj -M:dev -m benchmark reanalyze
```
Re-runs pattern detection and AI judge on saved outputs (no new API calls except judge).

**Prompts:** 9 prompts in `notebook/entries/orchestration-benchmark-pilot/prompts/`:
`iterative-refinement`, `adversarial-self-check`, `decomposition`, `conditional-branching`, `multi-source-synthesis`, `blind-evaluation`, `tool-computation`, `independent-analysts`, `number-guessing`

**Config:** 3 replicates × 3 models (opus, sonnet, gpt5.2), max depth 8.

**Results:** `notebook/entries/orchestration-benchmark-pilot/outputs/` with `results.edn`, `summary.md`, per-run verbose logs.

**Examples:**
```bash
# Run all
clj -M:dev -m benchmark run

# Run subset with custom agent
clj -M:dev -m benchmark run --agent agents/custom.agent.edn blind-evaluation tool-computation

# Re-judge existing results
clj -M:dev -m benchmark reanalyze
```
