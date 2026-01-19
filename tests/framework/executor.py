"""
Test executor: runs individual test cases with full instrumentation.
"""

import asyncio
import re
import time
import yaml
import subprocess
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

from claude_agent_sdk import (
    query,
    ClaudeAgentOptions,
    ToolUseBlock,
    ToolResultBlock,
    ResultMessage,
    SandboxSettings,
)

try:
    from .sandbox import SandboxedWorkspace
except ImportError:
    from sandbox import SandboxedWorkspace


@dataclass
class TestResult:
    """Result of a single test replicate."""
    name: str
    replicate: int
    passed: bool
    output: str
    tool_calls: list[dict]
    hooks_fired: list[dict]
    assertion_results: list[dict]
    duration_seconds: float
    workspace_path: str | None = None
    error: str | None = None


@dataclass
class TestContext:
    """Captures all observable behavior during a test run."""
    tool_calls: list[dict] = field(default_factory=list)
    hooks_fired: list[dict] = field(default_factory=list)
    skills_invoked: list[str] = field(default_factory=list)
    output: str = ""
    workspace: Path | None = None


class TestExecutor:
    """Executes test cases and captures results."""

    def __init__(
        self,
        base_dir: Path | None = None,
        template_path: Path | None = None,
        use_sandbox: bool = True,
    ):
        """
        Args:
            base_dir: Base directory for test discovery
            template_path: Path to project template for sandboxed tests
            use_sandbox: Whether to run tests in isolated sandboxes
        """
        self.base_dir = base_dir or Path.cwd()
        self.use_sandbox = use_sandbox

        # Default template path
        if template_path is None:
            self.template_path = self.base_dir / "fixtures" / "project"
        else:
            self.template_path = template_path

    async def run_case(self, case_path: str | Path) -> list[TestResult]:
        """Run all replicates of a test case."""
        case_path = Path(case_path)

        with open(case_path) as f:
            case = yaml.safe_load(f)

        config = case.get("config", {})
        replicates = config.get("replicates", 1)
        results = []

        for rep in range(replicates):
            result = await self._run_single(case, rep, case_path)
            results.append(result)

        return results

    async def _run_single(
        self,
        case: dict,
        replicate: int,
        case_path: Path
    ) -> TestResult:
        """Run a single replicate of a test case in an isolated sandbox."""
        config = case.get("config", {})
        ctx = TestContext()
        start_time = time.time()
        error = None

        # Determine if this test needs sandboxing
        use_sandbox = config.get("sandbox", self.use_sandbox)
        isolation = config.get("isolation", {})

        if use_sandbox and self.template_path.exists():
            # Run in sandboxed workspace
            with SandboxedWorkspace(self.template_path) as workspace:
                ctx.workspace = workspace
                result = await self._execute_in_workspace(
                    case, replicate, ctx, workspace, start_time
                )
                return result
        else:
            # Run without sandbox (for meta-tests or when disabled)
            cwd = self.base_dir.parent if self.base_dir.name == "tests" else self.base_dir
            return await self._execute_in_workspace(
                case, replicate, ctx, cwd, start_time
            )

    async def _execute_in_workspace(
        self,
        case: dict,
        replicate: int,
        ctx: TestContext,
        workspace: Path,
        start_time: float,
    ) -> TestResult:
        """Execute test within a workspace (sandboxed or not)."""
        config = case.get("config", {})
        error = None

        # Run setup commands in workspace
        try:
            await self._run_commands(case.get("setup", []), cwd=workspace)
        except Exception as e:
            return TestResult(
                name=case["name"],
                replicate=replicate,
                passed=False,
                output="",
                tool_calls=[],
                hooks_fired=[],
                assertion_results=[],
                duration_seconds=time.time() - start_time,
                workspace_path=str(workspace),
                error=f"Setup failed: {e}"
            )

        try:
            # Build sandbox settings to restrict writes to workspace
            sandbox_settings = SandboxSettings(
                write_allow_only=[str(workspace), "/tmp/claude/"]
            ) if ctx.workspace else None

            # Execute the agent
            async for message in query(
                prompt=case["prompt"],
                options=ClaudeAgentOptions(
                    allowed_tools=config.get("allowed_tools", []),
                    max_turns=config.get("max_turns", 10),
                    permission_mode="bypassPermissions",
                    cwd=str(workspace),
                    sandbox=sandbox_settings,
                    setting_sources=["user", "project", "local"],
                )
            ):
                # Extract tool calls from AssistantMessage
                if hasattr(message, "content"):
                    for block in message.content:
                        if isinstance(block, ToolUseBlock):
                            ctx.tool_calls.append({
                                "tool": block.name,
                                "input": block.input,
                                "id": block.id,
                            })
                            # Track skill invocations
                            if block.name == "Skill":
                                skill = block.input.get("skill", "unknown")
                                ctx.skills_invoked.append(skill)

                # Capture final result
                if isinstance(message, ResultMessage):
                    ctx.hooks_fired.append({"type": "Stop"})

                if hasattr(message, "result"):
                    ctx.output = message.result or ""

            # Check assertions (pass workspace for file checks)
            assertion_results = await self._check_assertions(
                case.get("assertions", []),
                ctx,
                workspace
            )

            passed = all(r["passed"] for r in assertion_results)

        except Exception as e:
            error = str(e)
            passed = False
            assertion_results = []

        finally:
            # Run teardown commands
            try:
                await self._run_commands(case.get("teardown", []), cwd=workspace)
            except Exception as e:
                if error is None:
                    error = f"Teardown failed: {e}"

        return TestResult(
            name=case["name"],
            replicate=replicate,
            passed=passed,
            output=ctx.output,
            tool_calls=ctx.tool_calls,
            hooks_fired=ctx.hooks_fired,
            assertion_results=assertion_results,
            duration_seconds=time.time() - start_time,
            workspace_path=str(workspace) if ctx.workspace else None,
            error=error
        )

    async def _check_assertions(
        self,
        assertions: list[dict],
        ctx: TestContext,
        workspace: Path,
    ) -> list[dict]:
        """Check all assertions against the test context."""
        results = []
        for assertion in assertions:
            result = await self._check_single_assertion(assertion, ctx, workspace)
            results.append(result)
        return results

    async def _check_single_assertion(
        self,
        assertion: dict,
        ctx: TestContext,
        workspace: Path,
    ) -> dict:
        """Check a single assertion."""
        atype = assertion["type"]

        # Assertions that need workspace path
        if atype == "file_created":
            return self._check_file_created(assertion, workspace)
        elif atype == "file_not_modified":
            return self._check_file_not_modified(assertion, workspace)
        elif atype == "evaluator":
            return await self._check_evaluator(assertion, ctx, workspace)

        # Assertions that only need context
        checkers = {
            "tool_used": self._check_tool_used,
            "tool_not_used": self._check_tool_not_used,
            "output_contains": self._check_output_contains,
            "output_not_contains": self._check_output_not_contains,
            "hook_fired": self._check_hook_fired,
            "skill_invoked": self._check_skill_invoked,
        }

        checker = checkers.get(atype)
        if checker:
            return checker(assertion, ctx)

        return {
            "passed": False,
            "assertion": assertion,
            "reason": f"Unknown assertion type: {atype}"
        }

    def _check_tool_used(self, assertion: dict, ctx: TestContext) -> dict:
        """Check that a tool was used."""
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
        desc = assertion.get("description", f"{tool} usage")

        if count < min_calls:
            return {
                "passed": False,
                "assertion": assertion,
                "reason": f"{desc}: called {count} times, expected >= {min_calls}"
            }

        if max_calls and count > max_calls:
            return {
                "passed": False,
                "assertion": assertion,
                "reason": f"{desc}: called {count} times, expected <= {max_calls}"
            }

        return {"passed": True, "assertion": assertion}

    def _check_tool_not_used(self, assertion: dict, ctx: TestContext) -> dict:
        """Check that a tool was NOT used."""
        tool = assertion["tool"]
        calls = [c for c in ctx.tool_calls if c["tool"] == tool]
        desc = assertion.get("description", f"{tool} not used")

        if calls:
            return {
                "passed": False,
                "assertion": assertion,
                "reason": f"{desc}: was called {len(calls)} times"
            }

        return {"passed": True, "assertion": assertion}

    def _check_output_contains(self, assertion: dict, ctx: TestContext) -> dict:
        """Check that output contains a pattern."""
        pattern = assertion["pattern"]
        flags = re.IGNORECASE if assertion.get("case_insensitive") else 0
        desc = assertion.get("description", f"output contains '{pattern}'")

        match = re.search(pattern, ctx.output, flags)

        if not match:
            return {
                "passed": False,
                "assertion": assertion,
                "reason": f"{desc}: pattern not found"
            }

        return {"passed": True, "assertion": assertion}

    def _check_output_not_contains(self, assertion: dict, ctx: TestContext) -> dict:
        """Check that output does NOT contain a pattern."""
        pattern = assertion["pattern"]
        flags = re.IGNORECASE if assertion.get("case_insensitive") else 0
        desc = assertion.get("description", f"output does not contain '{pattern}'")

        match = re.search(pattern, ctx.output, flags)

        if match:
            return {
                "passed": False,
                "assertion": assertion,
                "reason": f"{desc}: pattern was found"
            }

        return {"passed": True, "assertion": assertion}

    def _check_hook_fired(self, assertion: dict, ctx: TestContext) -> dict:
        """Check that a hook fired."""
        hook_type = assertion["hook_type"]
        desc = assertion.get("description", f"{hook_type} hook fired")

        matching = [h for h in ctx.hooks_fired if h["type"] == hook_type]

        if not matching:
            return {
                "passed": False,
                "assertion": assertion,
                "reason": f"{desc}: hook did not fire"
            }

        return {"passed": True, "assertion": assertion}

    def _check_skill_invoked(self, assertion: dict, ctx: TestContext) -> dict:
        """Check that a skill was invoked."""
        skill = assertion["skill"]
        desc = assertion.get("description", f"skill '{skill}' invoked")

        if skill not in ctx.skills_invoked:
            return {
                "passed": False,
                "assertion": assertion,
                "reason": f"{desc}: skill not invoked (invoked: {ctx.skills_invoked})"
            }

        return {"passed": True, "assertion": assertion}

    def _check_file_created(self, assertion: dict, workspace: Path) -> dict:
        """Check that a file was created in workspace."""
        pattern = assertion["path"]
        content_pattern = assertion.get("content_pattern")
        desc = assertion.get("description", f"file '{pattern}' created")

        # Search within workspace
        matches = list(workspace.glob(pattern))

        if not matches:
            return {
                "passed": False,
                "assertion": assertion,
                "reason": f"{desc}: no files matching pattern in workspace"
            }

        if content_pattern:
            for match in matches:
                try:
                    if re.search(content_pattern, match.read_text()):
                        return {"passed": True, "assertion": assertion}
                except Exception:
                    continue
            return {
                "passed": False,
                "assertion": assertion,
                "reason": f"{desc}: no file contains pattern '{content_pattern}'"
            }

        return {"passed": True, "assertion": assertion}

    def _check_file_not_modified(self, assertion: dict, workspace: Path) -> dict:
        """Check that a file was NOT modified."""
        path = assertion["path"]
        desc = assertion.get("description", f"file '{path}' not modified")

        full_path = workspace / path
        if not full_path.exists():
            return {
                "passed": False,
                "assertion": assertion,
                "reason": f"{desc}: file does not exist"
            }

        return {"passed": True, "assertion": assertion}

    async def _check_evaluator(
        self,
        assertion: dict,
        ctx: TestContext,
        workspace: Path
    ) -> dict:
        """Use LLM to evaluate complex criteria (sandboxed)."""
        try:
            from .evaluator import Evaluator
        except ImportError:
            from evaluator import Evaluator

        evaluator = Evaluator(
            model=assertion.get("model", "haiku"),
            sandbox_path=workspace if ctx.workspace else None
        )

        # Interpolate variables into prompt
        prompt = assertion["prompt"]
        prompt = prompt.replace("{{output}}", ctx.output)
        prompt = prompt.replace("{{tool_calls}}", str(ctx.tool_calls))
        prompt = prompt.replace("{{hooks_fired}}", str(ctx.hooks_fired))
        prompt = prompt.replace("{{skills_invoked}}", str(ctx.skills_invoked))

        result = await evaluator.evaluate(prompt, assertion["pass_if"])
        desc = assertion.get("description", "evaluator check")

        return {
            "passed": result["passed"],
            "assertion": assertion,
            "evaluator_response": result["response"],
            "reason": f"{desc}: {result.get('reason')}" if not result["passed"] else None
        }

    async def _run_commands(self, commands: list[dict], cwd: Path | None = None):
        """Run setup/teardown commands."""
        for cmd in commands:
            command = cmd.get("command", cmd) if isinstance(cmd, dict) else cmd
            result = subprocess.run(
                command,
                shell=True,
                capture_output=True,
                text=True,
                cwd=str(cwd) if cwd else None
            )
            if result.returncode != 0 and not (isinstance(cmd, dict) and cmd.get("ignore_errors", False)):
                raise RuntimeError(
                    f"Command failed: {command}\n"
                    f"stdout: {result.stdout}\n"
                    f"stderr: {result.stderr}"
                )

    async def run_all(
        self,
        case_paths: list[Path],
        parallel: int = 1
    ) -> dict[str, dict]:
        """
        Run all test cases and aggregate results.

        Args:
            case_paths: List of test case file paths
            parallel: Number of concurrent tests (1 = sequential)

        Returns:
            Dictionary mapping case paths to aggregated results
        """
        if parallel <= 1:
            return await self._run_sequential(case_paths)
        else:
            return await self._run_parallel(case_paths, parallel)

    async def _run_sequential(self, case_paths: list[Path]) -> dict[str, dict]:
        """Run tests sequentially."""
        all_results = {}

        for case_path in case_paths:
            try:
                results = await self.run_case(case_path)
                all_results[str(case_path)] = self._aggregate_results(results)
            except Exception as e:
                all_results[str(case_path)] = {
                    "name": str(case_path),
                    "passed": False,
                    "error": str(e),
                    "pass_rate": 0.0,
                    "passed_count": 0,
                    "total_replicates": 0,
                    "threshold": 0.0,
                    "replicates": []
                }

        return all_results

    async def _run_parallel(
        self,
        case_paths: list[Path],
        max_concurrent: int
    ) -> dict[str, dict]:
        """Run tests in parallel with concurrency limit."""
        semaphore = asyncio.Semaphore(max_concurrent)
        all_results = {}

        async def run_with_semaphore(case_path: Path) -> tuple[str, dict]:
            async with semaphore:
                try:
                    results = await self.run_case(case_path)
                    return str(case_path), self._aggregate_results(results)
                except Exception as e:
                    return str(case_path), {
                        "name": str(case_path),
                        "passed": False,
                        "error": str(e),
                        "pass_rate": 0.0,
                        "passed_count": 0,
                        "total_replicates": 0,
                        "threshold": 0.0,
                        "replicates": []
                    }

        # Run all tests concurrently (limited by semaphore)
        tasks = [run_with_semaphore(path) for path in case_paths]
        results = await asyncio.gather(*tasks)

        for path, result in results:
            all_results[path] = result

        return all_results

    def _aggregate_results(self, results: list[TestResult]) -> dict:
        """Aggregate replicate results and check pass threshold."""
        if not results:
            return {
                "name": "unknown",
                "passed": False,
                "reason": "No results",
                "pass_rate": 0.0,
                "passed_count": 0,
                "total_replicates": 0,
                "threshold": 0.0,
                "replicates": []
            }

        passed_count = sum(1 for r in results if r.passed)
        total = len(results)
        pass_rate = passed_count / total

        # Default threshold
        threshold = 0.5

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
                    "output": r.output[:500] + "..." if len(r.output) > 500 else r.output,
                    "duration_seconds": r.duration_seconds,
                    "workspace_path": r.workspace_path,
                    "error": r.error
                }
                for r in results
            ]
        }
