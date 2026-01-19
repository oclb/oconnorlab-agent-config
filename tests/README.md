# Claude Code Configuration Tests

Testing framework for validating Claude Code configuration: skills, hooks, behavioral instructions, and tool usage patterns.

## Quick Start

```bash
# Install dependencies
pip install -r requirements.txt

# Run all tests
python framework/runner.py

# Run meta-tests (validates the framework itself)
python framework/meta_runner.py

# Run specific test category
python framework/runner.py --pattern "cases/skills/*.yaml"

# Run single test
python framework/runner.py --filter "perform-analysis"
```

## Structure

```
tests/
├── framework/           # Test framework implementation
│   ├── runner.py       # Main entry point
│   ├── executor.py     # Test execution with instrumentation
│   ├── evaluator.py    # LLM-based assertions
│   ├── reporter.py     # Result reporting
│   └── meta_runner.py  # Framework self-tests
├── cases/              # Test case definitions (YAML)
│   ├── meta/          # Framework validation tests
│   ├── skills/        # Skill detection tests
│   ├── hooks/         # Hook trigger tests
│   └── behavior/      # CLAUDE.md instruction tests
├── fixtures/          # Test data files
├── results/           # Test run outputs (gitignored)
├── SPEC.md           # Full specification
└── README.md         # This file
```

## Writing Tests

Tests are YAML files with this structure:

```yaml
name: my-test-name
description: What this test validates

config:
  replicates: 3          # Run 3 times
  pass_threshold: 0.67   # 2/3 must pass
  max_turns: 10          # Limit agent iterations
  allowed_tools:         # Auto-approve these tools
    - Read
    - Bash

prompt: |
  The prompt to send to the agent.

assertions:
  - type: output_contains
    pattern: "expected text"

  - type: tool_used
    tool: Read
    min_calls: 1

  - type: evaluator
    prompt: "Did it work? {{output}}"
    pass_if: "yes"
```

## Assertion Types

| Type | Description |
|------|-------------|
| `output_contains` | Check output matches regex |
| `output_not_contains` | Check output does NOT match |
| `tool_used` | Verify tool was called |
| `tool_not_used` | Verify tool was NOT called |
| `hook_fired` | Check hook triggered |
| `skill_invoked` | Check skill was activated |
| `evaluator` | LLM judges complex criteria |
| `file_created` | Verify file exists |

## Meta-Tests

The `cases/meta/` directory contains tests that validate the framework:

| Test | Expected | Validates |
|------|----------|-----------|
| `trivial-pass` | PASS | Basic assertion works |
| `trivial-fail` | FAIL | Failure detection works |
| `tool-usage-pass` | PASS | Tool usage captured |
| `tool-not-used-pass` | PASS | Tool non-usage detected |
| `evaluator-pass` | PASS | LLM evaluator works |
| `evaluator-fail` | FAIL | LLM evaluator rejects |
| `replicates-majority` | PASS | Threshold aggregation |
| `hook-fired-pass` | PASS | Hook detection works |

Run with: `python framework/meta_runner.py`

## Results

Results are saved to `results/YYYY-MM-DD-HHMMSS/`:
- `summary.json` - Overall pass/fail counts
- `details.jsonl` - Full results per test
- `failures.md` - Human-readable failure report
