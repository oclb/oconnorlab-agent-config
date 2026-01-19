#!/usr/bin/env python3
"""
Main test runner for Claude Code configuration testing.

Usage:
    python runner.py                          # Run all tests
    python runner.py --pattern "cases/skills/*.yaml"  # Run skill tests only
    python runner.py --filter "perform-analysis"       # Run matching tests
    python runner.py --verbose                         # Detailed output
"""

import asyncio
import argparse
import sys
from pathlib import Path

# Add parent to path for imports
sys.path.insert(0, str(Path(__file__).parent))

from executor import TestExecutor
from reporter import Reporter


async def main():
    parser = argparse.ArgumentParser(
        description="Run Claude Code configuration tests"
    )
    parser.add_argument(
        "--pattern",
        default="cases/**/*.yaml",
        help="Glob pattern for test case files (default: cases/**/*.yaml)"
    )
    parser.add_argument(
        "--filter",
        help="Only run tests with names matching this string"
    )
    parser.add_argument(
        "--output",
        default="results",
        help="Output directory for results (default: results)"
    )
    parser.add_argument(
        "--parallel",
        type=int,
        default=1,
        help="Number of parallel test executions (default: 1)"
    )
    parser.add_argument(
        "--verbose",
        action="store_true",
        help="Print verbose output"
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="List tests without running them"
    )

    args = parser.parse_args()

    # Find test directory
    test_dir = Path(__file__).parent.parent
    cases_pattern = test_dir / args.pattern

    # Discover test cases
    cases = list(test_dir.glob(args.pattern))

    if args.filter:
        cases = [c for c in cases if args.filter in str(c)]

    if not cases:
        print(f"No test cases found matching: {args.pattern}")
        if args.filter:
            print(f"  with filter: {args.filter}")
        sys.exit(1)

    print(f"Found {len(cases)} test case(s)")

    if args.dry_run:
        for case in cases:
            print(f"  - {case.relative_to(test_dir)}")
        sys.exit(0)

    if args.verbose:
        for case in cases:
            print(f"  - {case.relative_to(test_dir)}")
        print()

    # Execute tests
    executor = TestExecutor(base_dir=test_dir)
    results = await executor.run_all(cases, parallel=args.parallel)

    # Generate report
    output_dir = test_dir / args.output
    reporter = Reporter(output_dir)
    report_dir = reporter.generate(results)

    # Exit with appropriate code
    all_passed = all(r.get("passed", False) for r in results.values())
    sys.exit(0 if all_passed else 1)


if __name__ == "__main__":
    asyncio.run(main())
