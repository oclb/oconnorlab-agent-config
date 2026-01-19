"""
Test reporter: generates human-readable and machine-parseable reports.
"""

import json
from pathlib import Path
from datetime import datetime


class Reporter:
    """Generates test reports in multiple formats."""

    def __init__(self, output_dir: str | Path):
        self.output_dir = Path(output_dir)
        self.timestamp = datetime.now().strftime("%Y-%m-%d-%H%M%S")
        self.run_dir = self.output_dir / self.timestamp
        self.run_dir.mkdir(parents=True, exist_ok=True)

    def generate(self, results: dict) -> Path:
        """Generate all report formats."""
        self._write_summary(results)
        self._write_details(results)
        self._write_failures(results)
        self._print_console(results)
        return self.run_dir

    def _write_summary(self, results: dict):
        """Write JSON summary."""
        total = len(results)
        passed = sum(1 for r in results.values() if r.get("passed", False))

        summary = {
            "timestamp": self.timestamp,
            "total_cases": total,
            "passed_cases": passed,
            "failed_cases": total - passed,
            "pass_rate": passed / total if total > 0 else 0,
            "cases": {
                name: {
                    "passed": r.get("passed", False),
                    "pass_rate": r.get("pass_rate", 0),
                    "threshold": r.get("threshold", 0),
                    "replicates": r.get("total_replicates", 0),
                    "error": r.get("error")
                }
                for name, r in results.items()
            }
        }

        with open(self.run_dir / "summary.json", "w") as f:
            json.dump(summary, f, indent=2)

    def _write_details(self, results: dict):
        """Write detailed JSONL results."""
        with open(self.run_dir / "details.jsonl", "w") as f:
            for name, result in results.items():
                f.write(json.dumps({"case": name, **result}) + "\n")

    def _write_failures(self, results: dict):
        """Write markdown failure report."""
        failures = []

        for name, result in results.items():
            if not result.get("passed", False):
                failures.append(f"## {result.get('name', name)}\n\n")

                if result.get("error"):
                    failures.append(f"**Error:** {result['error']}\n\n")
                else:
                    pass_rate = result.get("pass_rate", 0)
                    threshold = result.get("threshold", 0)
                    failures.append(f"**Pass rate:** {pass_rate:.1%} ")
                    failures.append(f"(threshold: {threshold:.1%})\n\n")

                    for rep in result.get("replicates", []):
                        if not rep.get("passed", False):
                            failures.append(f"### Replicate {rep.get('replicate', '?')}\n\n")

                            if rep.get("error"):
                                failures.append(f"**Error:** {rep['error']}\n\n")

                            for ar in rep.get("assertion_results", []):
                                if not ar.get("passed", False):
                                    reason = ar.get("reason", "Unknown reason")
                                    failures.append(f"- **FAIL:** {reason}\n")

                            failures.append("\n")

        with open(self.run_dir / "failures.md", "w") as f:
            f.write("# Test Failures\n\n")
            if failures:
                f.write("".join(failures))
            else:
                f.write("All tests passed!\n")

    def _print_console(self, results: dict):
        """Print results to console."""
        total = len(results)
        passed = sum(1 for r in results.values() if r.get("passed", False))

        print()
        print("=" * 60)
        print(f"Test Results: {passed}/{total} cases passed")
        print("=" * 60)
        print()

        for name, result in results.items():
            status = "\u2713" if result.get("passed", False) else "\u2717"
            test_name = result.get("name", name)
            pass_rate = result.get("pass_rate", 0)
            error = result.get("error")

            if error:
                print(f"{status} {test_name}: ERROR - {error[:50]}...")
            else:
                print(f"{status} {test_name}: {pass_rate:.0%} pass rate")

        print()
        print(f"Detailed results: {self.run_dir}")
        print()


class ConsoleReporter:
    """Lightweight reporter for quick console output only."""

    @staticmethod
    def print_result(name: str, passed: bool, details: str = ""):
        """Print a single test result."""
        status = "\u2713" if passed else "\u2717"
        print(f"{status} {name}" + (f": {details}" if details else ""))

    @staticmethod
    def print_summary(passed: int, total: int):
        """Print summary line."""
        print()
        print(f"Results: {passed}/{total} passed")
