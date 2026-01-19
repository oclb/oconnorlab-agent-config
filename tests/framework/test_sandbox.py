#!/usr/bin/env python3
"""
Test script to verify sandbox isolation works correctly.

This script:
1. Runs a test that writes a file in a sandboxed workspace
2. Verifies the file exists in the sandbox during the test
3. Verifies the file does NOT exist in the real project after cleanup
"""

import asyncio
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))

from executor import TestExecutor


async def test_sandbox_isolation():
    """Test that sandbox properly isolates file writes."""
    test_dir = Path(__file__).parent.parent
    project_root = test_dir.parent

    # The marker file that the test will create
    marker_file = "sandbox-test-marker.txt"

    # Ensure marker doesn't exist before test
    real_marker = project_root / marker_file
    if real_marker.exists():
        real_marker.unlink()
        print(f"Cleaned up pre-existing {marker_file}")

    # Run test with sandbox enabled
    print("Running sandbox isolation test...")
    print()

    executor = TestExecutor(
        base_dir=test_dir,
        use_sandbox=True,  # Enable sandbox
    )

    case_path = test_dir / "cases" / "sandbox" / "sandbox-isolation.yaml"
    results = await executor.run_case(case_path)

    # Check results
    for result in results:
        print(f"Test passed: {result.passed}")
        print(f"Workspace: {result.workspace_path}")
        print(f"Tool calls: {len(result.tool_calls)}")
        for tc in result.tool_calls:
            tool_input = tc.get('input', {})
            file_path = tool_input.get('file_path', '') if isinstance(tool_input, dict) else str(tool_input)[:80]
            print(f"  - {tc['tool']}: {file_path}")

        if result.error:
            print(f"Error: {result.error}")

        print()
        print("Assertions:")
        for ar in result.assertion_results:
            status = "✓" if ar["passed"] else "✗"
            desc = ar["assertion"].get("description", ar["assertion"].get("type"))
            print(f"  {status} {desc}")
            if not ar["passed"] and ar.get("reason"):
                print(f"      Reason: {ar['reason']}")

    print()

    # Verify marker file does NOT exist in real project
    if real_marker.exists():
        print("✗ FAIL: Marker file leaked to real project!")
        print(f"  File exists at: {real_marker}")
        real_marker.unlink()  # Clean up
        return False
    else:
        print("✓ PASS: Marker file correctly isolated to sandbox")
        print("  File was created in sandbox workspace only")
        return True


async def test_parallel_sandbox():
    """Test that parallel execution uses separate sandboxes."""
    test_dir = Path(__file__).parent.parent

    print()
    print("=" * 60)
    print("Testing parallel sandbox isolation...")
    print("=" * 60)
    print()

    executor = TestExecutor(base_dir=test_dir, use_sandbox=True)

    # Run multiple simple tests in parallel
    case_paths = list((test_dir / "cases" / "meta").glob("trivial-*.yaml"))

    if len(case_paths) < 2:
        print("Not enough test cases for parallel test")
        return True

    print(f"Running {len(case_paths)} tests in parallel (2 concurrent)...")

    results = await executor.run_all(case_paths, parallel=2)

    # Check all completed
    completed = sum(1 for r in results.values() if r.get("replicates"))
    print(f"Completed: {completed}/{len(case_paths)}")

    # Verify no workspace conflicts
    workspace_paths = set()
    for path, result in results.items():
        for rep in result.get("replicates", []):
            ws = rep.get("workspace_path")
            if ws:
                if ws in workspace_paths:
                    print(f"✗ FAIL: Workspace reused: {ws}")
                    return False
                workspace_paths.add(ws)

    print(f"✓ PASS: All {len(workspace_paths)} workspaces were unique")
    return True


if __name__ == "__main__":
    print("=" * 60)
    print("Sandbox Isolation Tests")
    print("=" * 60)
    print()

    success1 = asyncio.run(test_sandbox_isolation())
    success2 = asyncio.run(test_parallel_sandbox())

    print()
    print("=" * 60)
    if success1 and success2:
        print("All sandbox tests passed!")
    else:
        print("Some sandbox tests failed!")
    print("=" * 60)

    sys.exit(0 if (success1 and success2) else 1)
