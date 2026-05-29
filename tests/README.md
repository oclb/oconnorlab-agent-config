# Configuration Tests

This directory contains two kinds of tests for the merged lab agent configuration.

1. `test_config_agent_tool.py` validates the unified installer and skill-link manager for both `--agent claude` and `--agent codex`.
2. `framework/` and the remaining `cases/` directories are the legacy Claude behavior-test harness, pruned to remove cases for skills that were intentionally dropped during the merge.

## Quick Start

```bash
PYTHONDONTWRITEBYTECODE=1 python3 -m unittest tests/test_config_agent_tool.py
python framework/meta_runner.py
python framework/runner.py
```

Behavior-test results are written to `tests/results/`, which is gitignored.
