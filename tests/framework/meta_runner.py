#!/usr/bin/env python3
"""
Meta-test runner: validates the test framework itself.

Runs meta-tests and checks that they pass/fail as expected.
Tests with "pass" in the name should pass.
Tests with "fail" in the name should fail.

Usage:
    python meta_runner.py
"""

import asyncio
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))

from executor import TestExecutor


# Define expected outcomes for meta-tests
EXPECTED_OUTCOMES = {
    "meta-trivial-pass": True,       # Should pass
    "meta-trivial-fail": False,      # Should fail (impossible pattern)
    "meta-tool-usage-pass": True,    # Should pass (Read tool used)
    "meta-tool-not-used-pass": True, # Should pass (no tools needed)
    "meta-evaluator-pass": True,     # Should pass (Python is a language)
    "meta-evaluator-fail": False,    # Should fail (2+2 != 5)
    "meta-replicates-majority": True, # Should pass (deterministic)
    "meta-hook-fired-pass": True,    # Should pass (Stop hook fires)
}


async def run_meta_tests():
    """Run all meta-tests and validate expected outcomes."""
    test_dir = Path(__file__).parent.parent
    meta_cases = list((test_dir / "cases" / "meta").glob("*.yaml"))

    if not meta_cases:
        print("No meta-test cases found!")
        return False

    print(f"Running {len(meta_cases)} meta-tests...")
    print()

    # Meta-tests don't need sandboxing - they test the framework itself
    executor = TestExecutor(base_dir=test_dir, use_sandbox=False)
    results = await executor.run_all(meta_cases)

    # Check against expected outcomes
    all_correct = True
    correct_count = 0

    for case_path, result in results.items():
        test_name = result.get("name", Path(case_path).stem)
        actual_passed = result.get("passed", False)
        expected_passed = EXPECTED_OUTCOMES.get(test_name)

        if expected_passed is None:
            print(f"? {test_name}: No expected outcome defined")
            continue

        if actual_passed == expected_passed:
            status = "\u2713"
            outcome = "correctly passed" if actual_passed else "correctly failed"
            correct_count += 1
        else:
            status = "\u2717"
            outcome = f"WRONG: expected {'pass' if expected_passed else 'fail'}, got {'pass' if actual_passed else 'fail'}"
            all_correct = False

        print(f"{status} {test_name}: {outcome}")

        # Show details for unexpected outcomes
        if actual_passed != expected_passed:
            for rep in result.get("replicates", []):
                for ar in rep.get("assertion_results", []):
                    if not ar.get("passed", True):
                        reason = ar.get("reason", "Unknown")
                        print(f"    - {reason}")

    print()
    print("=" * 60)
    print(f"Meta-test results: {correct_count}/{len(results)} behaved as expected")
    print("=" * 60)

    return all_correct


async def run_single_meta_test(test_name: str):
    """Run a single meta-test for debugging."""
    test_dir = Path(__file__).parent.parent
    case_path = test_dir / "cases" / "meta" / f"{test_name}.yaml"

    if not case_path.exists():
        print(f"Test not found: {case_path}")
        return

    print(f"Running: {test_name}")
    print()

    executor = TestExecutor(base_dir=test_dir, use_sandbox=False)
    results = await executor.run_case(case_path)

    for result in results:
        print(f"Passed: {result.passed}")
        print(f"Output: {result.output[:200]}...")
        print(f"Tool calls: {len(result.tool_calls)}")
        for tc in result.tool_calls:
            print(f"  - {tc['tool']}: {str(tc.get('input', ''))[:50]}")
        print(f"Hooks fired: {result.hooks_fired}")
        print(f"Duration: {result.duration_seconds:.2f}s")

        if result.assertion_results:
            print("Assertions:")
            for ar in result.assertion_results:
                status = "\u2713" if ar["passed"] else "\u2717"
                desc = ar["assertion"].get("description", ar["assertion"].get("type"))
                print(f"  {status} {desc}")
                if not ar["passed"] and ar.get("reason"):
                    print(f"      Reason: {ar['reason']}")


if __name__ == "__main__":
    if len(sys.argv) > 1:
        # Run specific test for debugging
        asyncio.run(run_single_meta_test(sys.argv[1]))
    else:
        # Run all meta-tests
        success = asyncio.run(run_meta_tests())
        sys.exit(0 if success else 1)
