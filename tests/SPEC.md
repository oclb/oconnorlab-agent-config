# Claude Code Configuration Testing Framework

## Overview

A testing framework for validating Claude Code configuration: skills, hooks, behavioral instructions, and tool usage patterns. Uses the Claude Agent SDK for full introspection of agent behavior.

## Design Goals

1. **Observable** - Capture every tool call, skill activation, and hook trigger
2. **Evaluable** - Use a separate agent to judge if behavior meets criteria
3. **Statistical** - Run replicates and enforce pass thresholds
4. **Declarative** - Define tests in YAML, not code
5. **Fast feedback** - Parallel execution where possible

## Architecture

```
tests/
├── SPEC.md                    # This document
├── framework/
│   ├── runner.py              # Main test runner
│   ├── executor.py            # Runs individual test cases
│   ├── evaluator.py           # LLM-based output evaluation
│   ├── hooks.py               # Instrumentation hooks
│   └── reporter.py            # Generates test reports
├── cases/
│   ├── skills/                # Skill detection tests
│   │   ├── perform-analysis.yaml
│   │   ├── new-software.yaml
│   │   └── teaching-mode.yaml
│   ├── hooks/                 # Hook trigger tests
│   │   └── stop-notify.yaml
│   └── behavior/              # CLAUDE.md instruction tests
│       └── afk-mode.yaml
├── fixtures/                  # Test data files
│   └── sample-data.csv
└── results/                   # Test run outputs
    └── YYYY-MM-DD-HHMMSS/
        ├── summary.json
        ├── details.jsonl
        └── failures.md
```

## Test Case Format

Tests are defined in YAML with the following structure:

```yaml
# cases/skills/perform-analysis.yaml
name: skill-detection-perform-analysis
description: Verifies perform-analysis skill auto-triggers for data analysis requests

# Test configuration
config:
  replicates: 3                    # Number of times to run
  pass_threshold: 0.67             # Fraction that must pass (2/3)
  timeout_seconds: 120             # Per-replicate timeout
  max_turns: 10                    # Limit agent iterations
  allowed_tools:                   # Tools to auto-approve
    - Read
    - Glob
    - Grep
    - Bash
    - Edit
    - Write

# The prompt to send
prompt: |
  Analyze the gene expression data in fixtures/sample-data.csv.
  Identify any batch effects and suggest corrections.

# Optional setup/teardown
setup:
  - command: "mkdir -p /tmp/claude/test-workspace"
  - command: "cp fixtures/sample-data.csv /tmp/claude/test-workspace/"

teardown:
  - command: "rm -rf /tmp/claude/test-workspace"

# Assertions to check
assertions:
  # Check for specific tools being used
  - type: tool_used
    tool: Read
    min_calls: 1
    description: "Agent should read the data file"

  - type: tool_used
    tool: Edit
    target_pattern: "notebook/entries/.*\\.md"
    description: "Agent should create a notebook entry"

  # Check output contains expected content
  - type: output_contains
    pattern: "batch effect|batch correction"
    case_insensitive: true
    description: "Should mention batch effects"

  # Check a skill was invoked (detected via system prompt or behavior)
  - type: skill_invoked
    skill: perform-analysis
    detection_method: behavior  # or "explicit" if using /skill-name

  # Check a hook fired
  - type: hook_fired
    hook_type: Stop
    description: "Stop hook should fire on completion"

  # LLM-based evaluation for complex criteria
  - type: evaluator
    prompt: |
      Evaluate if this analysis followed an 8-step framework:
      1. Understand motivation
      2. Set expectations
      3. Verify resources
      4. Make a plan
      5. Perform analysis
      6. Display results
      7. Document choices
      8. Finalize

      The agent's output:
      ---
      {{output}}
      ---

      Did the agent follow most of these steps (at least 5/8)?
    pass_if: "yes"
    description: "Should follow 8-step analysis framework"

  # Check specific behavior flags were respected
  - type: behavior_flag
    flag: AFK
    expected_value: true
    check: |
      Agent should not ask clarifying questions.
      Look for: "would you like", "should I", "do you want"
      If these appear, the AFK flag was not respected.
```

## Assertion Types

### `tool_used`
Verifies a specific tool was called.

```yaml
- type: tool_used
  tool: Read                        # Tool name
  min_calls: 1                      # Minimum invocations (default: 1)
  max_calls: 10                     # Maximum invocations (optional)
  target_pattern: ".*\\.py"         # Regex for tool input (optional)
  description: "Description"
```

### `tool_not_used`
Verifies a tool was NOT called.

```yaml
- type: tool_not_used
  tool: WebSearch
  description: "Should not search web for local file task"
```

### `output_contains`
Checks final output for patterns.

```yaml
- type: output_contains
  pattern: "error|failed"           # Regex pattern
  case_insensitive: true            # Default: false
  invert: true                      # Pass if NOT found (default: false)
  description: "Should not contain errors"
```

### `skill_invoked`
Detects if a skill was activated.

```yaml
- type: skill_invoked
  skill: perform-analysis
  detection_method: behavior        # "behavior" (heuristic) or "explicit" (/command)
```

Detection methods:
- `explicit`: Checks if `/skill-name` was invoked via Skill tool
- `behavior`: Uses evaluator to infer skill activation from output patterns

### `hook_fired`
Verifies a hook was triggered.

```yaml
- type: hook_fired
  hook_type: Stop                   # Stop, PreToolUse, PostToolUse, etc.
  tool_name: Edit                   # For tool-specific hooks (optional)
```

### `evaluator`
Uses a separate LLM call to evaluate complex criteria.

```yaml
- type: evaluator
  prompt: |
    {{output}}         # Interpolated with actual output
    {{tool_calls}}     # JSON of all tool calls
    {{hooks_fired}}    # List of hooks that fired

    [Your evaluation criteria here]
  pass_if: "yes"                    # Expected evaluator response
  model: haiku                      # Model for evaluation (default: haiku)
```

### `behavior_flag`
Checks if a behavior flag was respected.

```yaml
- type: behavior_flag
  flag: AFK
  expected_value: true
  check: |
    Instructions for evaluator to check flag compliance.
```

### `file_created`
Verifies a file was created.

```yaml
- type: file_created
  path: "notebook/entries/*.md"     # Glob pattern
  content_pattern: "## Summary"     # Optional content check
```

### `file_not_modified`
Verifies a file was NOT modified.

```yaml
- type: file_not_modified
  path: "src/main.py"
  description: "Read-only task should not modify source"
```

## Framework Components

### Runner (`runner.py`)

Main entry point. Discovers and executes test cases.

```python
import asyncio
import argparse
from pathlib import Path
from executor import TestExecutor
from reporter import Reporter

async def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--pattern", default="cases/**/*.yaml")
    parser.add_argument("--parallel", type=int, default=1)
    parser.add_argument("--output", default="results")
    parser.add_argument("--filter", help="Run only tests matching pattern")
    args = parser.parse_args()

    # Discover test cases
    cases = list(Path(".").glob(args.pattern))
    if args.filter:
        cases = [c for c in cases if args.filter in str(c)]

    # Execute tests
    executor = TestExecutor()
    results = await executor.run_all(cases, parallel=args.parallel)

    # Generate report
    reporter = Reporter(args.output)
    reporter.generate(results)

if __name__ == "__main__":
    asyncio.run(main())
```

### Executor (`executor.py`)

Runs individual test cases with full instrumentation.

```python
import asyncio
import yaml
import re
from dataclasses import dataclass, field
from typing import Any
from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher

@dataclass
class TestResult:
    name: str
    replicate: int
    passed: bool
    output: str
    tool_calls: list[dict]
    hooks_fired: list[dict]
    assertion_results: list[dict]
    duration_seconds: float
    error: str | None = None

@dataclass
class TestContext:
    """Captures all observable behavior during a test run."""
    tool_calls: list[dict] = field(default_factory=list)
    hooks_fired: list[dict] = field(default_factory=list)
    output: str = ""

class TestExecutor:
    async def run_case(self, case_path: str) -> list[TestResult]:
        with open(case_path) as f:
            case = yaml.safe_load(f)

        config = case.get("config", {})
        replicates = config.get("replicates", 1)
        results = []

        for rep in range(replicates):
            result = await self._run_single(case, rep)
            results.append(result)

        return results

    async def _run_single(self, case: dict, replicate: int) -> TestResult:
        config = case.get("config", {})
        ctx = TestContext()

        # Build hooks for instrumentation
        hooks = self._build_instrumentation_hooks(ctx)

        # Run setup commands
        await self._run_commands(case.get("setup", []))

        try:
            # Execute the agent
            async for message in query(
                prompt=case["prompt"],
                options=ClaudeAgentOptions(
                    allowed_tools=config.get("allowed_tools", []),
                    max_turns=config.get("max_turns", 10),
                    hooks=hooks
                )
            ):
                if hasattr(message, "result"):
                    ctx.output = message.result

            # Check assertions
            assertion_results = await self._check_assertions(
                case.get("assertions", []),
                ctx
            )

            passed = all(r["passed"] for r in assertion_results)

            return TestResult(
                name=case["name"],
                replicate=replicate,
                passed=passed,
                output=ctx.output,
                tool_calls=ctx.tool_calls,
                hooks_fired=ctx.hooks_fired,
                assertion_results=assertion_results,
                duration_seconds=0,  # TODO: track timing
            )

        finally:
            await self._run_commands(case.get("teardown", []))

    def _build_instrumentation_hooks(self, ctx: TestContext) -> dict:
        """Create hooks that capture all tool usage."""

        async def capture_pre_tool(input_data, tool_use_id, context):
            ctx.tool_calls.append({
                "tool": input_data.get("tool_name"),
                "input": input_data.get("tool_input"),
                "phase": "pre"
            })
            return {}

        async def capture_post_tool(input_data, tool_use_id, context):
            # Update the last tool call with result
            if ctx.tool_calls:
                ctx.tool_calls[-1]["result"] = input_data.get("tool_result")
                ctx.tool_calls[-1]["phase"] = "complete"
            return {}

        async def capture_stop(input_data, tool_use_id, context):
            ctx.hooks_fired.append({"type": "Stop"})
            return {}

        return {
            "PreToolUse": [HookMatcher(matcher=".*", hooks=[capture_pre_tool])],
            "PostToolUse": [HookMatcher(matcher=".*", hooks=[capture_post_tool])],
            "Stop": [HookMatcher(matcher=".*", hooks=[capture_stop])],
        }

    async def _check_assertions(
        self,
        assertions: list[dict],
        ctx: TestContext
    ) -> list[dict]:
        results = []
        for assertion in assertions:
            result = await self._check_single_assertion(assertion, ctx)
            results.append(result)
        return results

    async def _check_single_assertion(
        self,
        assertion: dict,
        ctx: TestContext
    ) -> dict:
        atype = assertion["type"]

        if atype == "tool_used":
            return self._check_tool_used(assertion, ctx)
        elif atype == "tool_not_used":
            return self._check_tool_not_used(assertion, ctx)
        elif atype == "output_contains":
            return self._check_output_contains(assertion, ctx)
        elif atype == "hook_fired":
            return self._check_hook_fired(assertion, ctx)
        elif atype == "evaluator":
            return await self._check_evaluator(assertion, ctx)
        elif atype == "file_created":
            return self._check_file_created(assertion)
        else:
            return {"passed": False, "reason": f"Unknown assertion type: {atype}"}

    def _check_tool_used(self, assertion: dict, ctx: TestContext) -> dict:
        tool = assertion["tool"]
        min_calls = assertion.get("min_calls", 1)
        max_calls = assertion.get("max_calls")
        pattern = assertion.get("target_pattern")

        matching_calls = [
            c for c in ctx.tool_calls
            if c["tool"] == tool
        ]

        if pattern:
            matching_calls = [
                c for c in matching_calls
                if re.search(pattern, str(c.get("input", "")))
            ]

        count = len(matching_calls)

        if count < min_calls:
            return {
                "passed": False,
                "assertion": assertion,
                "reason": f"{tool} called {count} times, expected >= {min_calls}"
            }

        if max_calls and count > max_calls:
            return {
                "passed": False,
                "assertion": assertion,
                "reason": f"{tool} called {count} times, expected <= {max_calls}"
            }

        return {"passed": True, "assertion": assertion}

    def _check_tool_not_used(self, assertion: dict, ctx: TestContext) -> dict:
        tool = assertion["tool"]
        calls = [c for c in ctx.tool_calls if c["tool"] == tool]

        if calls:
            return {
                "passed": False,
                "assertion": assertion,
                "reason": f"{tool} was called {len(calls)} times"
            }

        return {"passed": True, "assertion": assertion}

    def _check_output_contains(self, assertion: dict, ctx: TestContext) -> dict:
        pattern = assertion["pattern"]
        flags = re.IGNORECASE if assertion.get("case_insensitive") else 0
        invert = assertion.get("invert", False)

        match = re.search(pattern, ctx.output, flags)
        found = match is not None

        if invert:
            passed = not found
            reason = f"Pattern '{pattern}' was found" if found else None
        else:
            passed = found
            reason = f"Pattern '{pattern}' not found" if not found else None

        return {"passed": passed, "assertion": assertion, "reason": reason}

    def _check_hook_fired(self, assertion: dict, ctx: TestContext) -> dict:
        hook_type = assertion["hook_type"]
        matching = [h for h in ctx.hooks_fired if h["type"] == hook_type]

        if not matching:
            return {
                "passed": False,
                "assertion": assertion,
                "reason": f"Hook {hook_type} did not fire"
            }

        return {"passed": True, "assertion": assertion}

    async def _check_evaluator(self, assertion: dict, ctx: TestContext) -> dict:
        from evaluator import Evaluator

        evaluator = Evaluator(model=assertion.get("model", "haiku"))

        # Interpolate variables into prompt
        prompt = assertion["prompt"]
        prompt = prompt.replace("{{output}}", ctx.output)
        prompt = prompt.replace("{{tool_calls}}", str(ctx.tool_calls))
        prompt = prompt.replace("{{hooks_fired}}", str(ctx.hooks_fired))

        result = await evaluator.evaluate(prompt, assertion["pass_if"])

        return {
            "passed": result["passed"],
            "assertion": assertion,
            "evaluator_response": result["response"],
            "reason": result.get("reason")
        }

    def _check_file_created(self, assertion: dict) -> dict:
        from pathlib import Path

        pattern = assertion["path"]
        matches = list(Path(".").glob(pattern))

        if not matches:
            return {
                "passed": False,
                "assertion": assertion,
                "reason": f"No files matching {pattern}"
            }

        content_pattern = assertion.get("content_pattern")
        if content_pattern:
            for match in matches:
                if re.search(content_pattern, match.read_text()):
                    return {"passed": True, "assertion": assertion}
            return {
                "passed": False,
                "assertion": assertion,
                "reason": f"No file contains pattern: {content_pattern}"
            }

        return {"passed": True, "assertion": assertion}

    async def _run_commands(self, commands: list[dict]):
        import subprocess
        for cmd in commands:
            subprocess.run(cmd["command"], shell=True, check=True)

    async def run_all(
        self,
        case_paths: list[str],
        parallel: int = 1
    ) -> dict:
        all_results = {}

        for case_path in case_paths:
            results = await self.run_case(str(case_path))
            all_results[str(case_path)] = self._aggregate_results(results)

        return all_results

    def _aggregate_results(self, results: list[TestResult]) -> dict:
        """Aggregate replicate results and check pass threshold."""
        if not results:
            return {"passed": False, "reason": "No results"}

        # Get config from first result's case
        passed_count = sum(1 for r in results if r.passed)
        total = len(results)
        pass_rate = passed_count / total

        # TODO: Get threshold from case config
        threshold = 0.67

        return {
            "name": results[0].name,
            "passed": pass_rate >= threshold,
            "pass_rate": pass_rate,
            "passed_count": passed_count,
            "total_replicates": total,
            "threshold": threshold,
            "replicates": [
                {
                    "replicate": r.replicate,
                    "passed": r.passed,
                    "assertion_results": r.assertion_results,
                    "tool_calls": r.tool_calls,
                    "hooks_fired": r.hooks_fired,
                }
                for r in results
            ]
        }
```

### Evaluator (`evaluator.py`)

Uses a fast model to evaluate complex criteria.

```python
from claude_agent_sdk import query, ClaudeAgentOptions

class Evaluator:
    def __init__(self, model: str = "haiku"):
        self.model = model

    async def evaluate(self, prompt: str, pass_if: str) -> dict:
        """
        Evaluate a prompt and check if response matches pass_if.

        The evaluator is instructed to respond with just "yes" or "no"
        to make pass/fail determination reliable.
        """
        eval_prompt = f"""You are a test evaluator. Answer with ONLY "yes" or "no".

{prompt}

Answer "yes" if the criteria are met, "no" if not."""

        response = ""
        async for message in query(
            prompt=eval_prompt,
            options=ClaudeAgentOptions(
                allowed_tools=[],  # No tools needed for evaluation
                max_turns=1,
                model=self.model
            )
        ):
            if hasattr(message, "result"):
                response = message.result.strip().lower()

        passed = response.startswith(pass_if.lower())

        return {
            "passed": passed,
            "response": response,
            "reason": None if passed else f"Expected '{pass_if}', got '{response}'"
        }
```

### Reporter (`reporter.py`)

Generates human-readable and machine-parseable reports.

```python
import json
from pathlib import Path
from datetime import datetime

class Reporter:
    def __init__(self, output_dir: str):
        self.output_dir = Path(output_dir)
        self.timestamp = datetime.now().strftime("%Y-%m-%d-%H%M%S")
        self.run_dir = self.output_dir / self.timestamp
        self.run_dir.mkdir(parents=True, exist_ok=True)

    def generate(self, results: dict):
        self._write_summary(results)
        self._write_details(results)
        self._write_failures(results)
        self._print_console(results)

    def _write_summary(self, results: dict):
        summary = {
            "timestamp": self.timestamp,
            "total_cases": len(results),
            "passed_cases": sum(1 for r in results.values() if r["passed"]),
            "failed_cases": sum(1 for r in results.values() if not r["passed"]),
            "cases": {
                name: {
                    "passed": r["passed"],
                    "pass_rate": r["pass_rate"],
                    "threshold": r["threshold"]
                }
                for name, r in results.items()
            }
        }

        with open(self.run_dir / "summary.json", "w") as f:
            json.dump(summary, f, indent=2)

    def _write_details(self, results: dict):
        with open(self.run_dir / "details.jsonl", "w") as f:
            for name, result in results.items():
                f.write(json.dumps({"case": name, **result}) + "\n")

    def _write_failures(self, results: dict):
        failures = []
        for name, result in results.items():
            if not result["passed"]:
                failures.append(f"## {result['name']}\n")
                failures.append(f"Pass rate: {result['pass_rate']:.1%} ")
                failures.append(f"(threshold: {result['threshold']:.1%})\n\n")

                for rep in result["replicates"]:
                    if not rep["passed"]:
                        failures.append(f"### Replicate {rep['replicate']}\n")
                        for ar in rep["assertion_results"]:
                            if not ar["passed"]:
                                failures.append(f"- FAIL: {ar.get('reason', 'Unknown')}\n")
                        failures.append("\n")

        with open(self.run_dir / "failures.md", "w") as f:
            f.write("# Test Failures\n\n")
            f.write("".join(failures) if failures else "All tests passed!\n")

    def _print_console(self, results: dict):
        total = len(results)
        passed = sum(1 for r in results.values() if r["passed"])

        print(f"\n{'='*60}")
        print(f"Test Results: {passed}/{total} cases passed")
        print(f"{'='*60}\n")

        for name, result in results.items():
            status = "✓" if result["passed"] else "✗"
            rate = f"{result['pass_rate']:.0%}"
            print(f"{status} {result['name']}: {rate} pass rate")

        print(f"\nDetailed results: {self.run_dir}")
```

## CLI Usage

```bash
# Run all tests
python tests/framework/runner.py

# Run specific test category
python tests/framework/runner.py --pattern "cases/skills/*.yaml"

# Run single test
python tests/framework/runner.py --filter "perform-analysis"

# Run with parallelism (careful: may cause resource contention)
python tests/framework/runner.py --parallel 2

# Custom output directory
python tests/framework/runner.py --output ./test-results
```

## Example Test Cases

### Skill Detection Test

```yaml
# cases/skills/new-software.yaml
name: skill-detection-new-software
description: Verifies new-software skill triggers for tool learning requests

config:
  replicates: 3
  pass_threshold: 0.67
  max_turns: 5
  allowed_tools:
    - Read
    - Bash
    - WebSearch
    - WebFetch

prompt: "Help me learn jq"

assertions:
  - type: tool_used
    tool: WebSearch
    description: "Should search for jq documentation"

  - type: tool_used
    tool: Bash
    target_pattern: "brew install|apt install|which jq"
    description: "Should attempt installation or check if installed"

  - type: output_contains
    pattern: "json|JSON"
    case_insensitive: true
    description: "Should explain jq is for JSON processing"

  - type: evaluator
    prompt: |
      Did this response follow a learn-tool workflow?
      Expected elements:
      - Search for documentation
      - Install or verify installation
      - Show usage examples
      - Provide links to docs

      Response:
      {{output}}

      Tools used:
      {{tool_calls}}
    pass_if: "yes"
```

### Hook Trigger Test

```yaml
# cases/hooks/stop-notify.yaml
name: hook-trigger-stop-notify
description: Verifies Stop hook fires and triggers notification

config:
  replicates: 2
  pass_threshold: 1.0  # Must always work
  max_turns: 3
  allowed_tools:
    - Read

prompt: "What is 2 + 2?"

assertions:
  - type: hook_fired
    hook_type: Stop
    description: "Stop hook should fire on task completion"

  - type: output_contains
    pattern: "4"
    description: "Should answer the question correctly"
```

### AFK Mode Test

```yaml
# cases/behavior/afk-mode.yaml
name: behavior-flag-afk-mode
description: Verifies AFK mode prevents unnecessary questions

config:
  replicates: 3
  pass_threshold: 0.67
  max_turns: 5
  allowed_tools:
    - Read
    - Edit

prompt: |
  (afk) Update the README.md to mention the new testing framework.

assertions:
  - type: output_contains
    pattern: "would you like|should I|do you want|which option"
    case_insensitive: true
    invert: true  # Should NOT contain these
    description: "Should not ask clarifying questions in AFK mode"

  - type: tool_used
    tool: Edit
    target_pattern: "README.md"
    description: "Should proceed to edit without asking"

  - type: evaluator
    prompt: |
      In AFK mode, the agent should proceed autonomously without asking
      unnecessary clarifying questions.

      Did this agent work autonomously, or did it ask questions?

      Output:
      {{output}}
    pass_if: "yes"
```

### Notebook Integration Test

```yaml
# cases/skills/perform-analysis-notebook.yaml
name: skill-perform-analysis-notebook-integration
description: Verifies perform-analysis creates notebook entry

config:
  replicates: 2
  pass_threshold: 1.0
  max_turns: 15
  allowed_tools:
    - Read
    - Glob
    - Grep
    - Edit
    - Write
    - Bash

setup:
  - command: "mkdir -p fixtures && echo 'sample,value\na,1\nb,2' > fixtures/test.csv"

teardown:
  - command: "rm -f fixtures/test.csv"

prompt: |
  Analyze the data in fixtures/test.csv.
  This is a simple sanity check of the data.

assertions:
  - type: file_created
    path: "notebook/entries/*.md"
    content_pattern: "## Summary"
    description: "Should create notebook entry with Summary section"

  - type: tool_used
    tool: Read
    target_pattern: "test.csv"
    description: "Should read the data file"

  - type: tool_used
    tool: Edit
    target_pattern: "INDEX.md"
    description: "Should update the notebook index"
```

## Extending the Framework

### Adding Custom Assertion Types

Add new assertion types in `executor.py`:

```python
def _check_custom_assertion(self, assertion: dict, ctx: TestContext) -> dict:
    # Your custom logic here
    return {"passed": True/False, "assertion": assertion, "reason": "..."}
```

### Adding Custom Hooks for Instrumentation

Extend `_build_instrumentation_hooks` to capture additional events:

```python
async def capture_skill_invocation(input_data, tool_use_id, context):
    if input_data.get("tool_name") == "Skill":
        ctx.skills_invoked.append(input_data.get("tool_input", {}).get("skill"))
    return {}
```

### Integrating with CI/CD

```yaml
# .github/workflows/test-config.yml
name: Test Claude Config

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Claude Code
        run: curl -fsSL https://claude.ai/install.sh | bash

      - name: Install dependencies
        run: pip install claude-agent-sdk pyyaml

      - name: Run tests
        env:
          ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}
        run: python tests/framework/runner.py

      - name: Upload results
        uses: actions/upload-artifact@v4
        with:
          name: test-results
          path: tests/results/
```

## Cost Considerations

Each test replicate makes API calls. Estimate costs:

- **Prompt processing**: ~1K-5K tokens per test
- **Response generation**: ~500-2K tokens per test
- **Evaluator calls**: ~200 tokens each

For a test suite with 10 cases × 3 replicates:
- ~30 main API calls
- ~30-90 evaluator calls (fast/cheap with Haiku)

Use `max_turns` to limit runaway tests.

## Future Enhancements

1. **Snapshot testing** - Compare outputs to golden files
2. **Regression detection** - Track pass rates over time
3. **Flakiness detection** - Flag tests with high variance
4. **Coverage reporting** - Which skills/hooks are tested
5. **Interactive debugging** - Replay failed tests step-by-step
