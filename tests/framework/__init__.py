"""
Claude Code Configuration Testing Framework

A framework for testing Claude Code configuration: skills, hooks,
behavioral instructions, and tool usage patterns.
"""

from .executor import TestExecutor, TestResult, TestContext
from .evaluator import Evaluator, SimpleEvaluator
from .reporter import Reporter, ConsoleReporter
from .sandbox import SandboxedWorkspace, sandboxed_workspace, WorkspacePool

__all__ = [
    "TestExecutor",
    "TestResult",
    "TestContext",
    "Evaluator",
    "SimpleEvaluator",
    "Reporter",
    "ConsoleReporter",
    "SandboxedWorkspace",
    "sandboxed_workspace",
    "WorkspacePool",
]
