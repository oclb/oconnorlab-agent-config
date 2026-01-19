"""
LLM-based evaluator for complex test assertions.
"""

from pathlib import Path
from claude_agent_sdk import query, ClaudeAgentOptions, SandboxSettings


class Evaluator:
    """Uses a fast model to evaluate complex criteria."""

    def __init__(self, model: str = "haiku", sandbox_path: Path | None = None):
        """
        Args:
            model: Model to use for evaluation (haiku, sonnet)
            sandbox_path: If provided, sandbox the evaluator to this path
        """
        self.model = model
        self.sandbox_path = sandbox_path

    async def evaluate(self, prompt: str, pass_if: str) -> dict:
        """
        Evaluate a prompt and check if response matches pass_if.

        The evaluator is instructed to respond with just "yes" or "no"
        to make pass/fail determination reliable.

        Args:
            prompt: The evaluation prompt with criteria
            pass_if: Expected response for pass ("yes" or "no")

        Returns:
            dict with keys: passed, response, reason
        """
        eval_prompt = f"""You are a test evaluator. Your job is to determine if certain criteria are met.

Answer with ONLY "yes" or "no" - nothing else.

{prompt}

Answer "yes" if the criteria are met, "no" if not. Just one word."""

        response = ""
        try:
            # Build options
            options = ClaudeAgentOptions(
                allowed_tools=[],  # No tools needed for evaluation
                max_turns=1,
                permission_mode="bypassPermissions",
            )

            # Set model if specified
            if self.model in ("haiku", "sonnet"):
                options.model = f"claude-3-5-{self.model}-latest"

            # Add sandbox if path provided
            if self.sandbox_path:
                options.cwd = str(self.sandbox_path)
                options.sandbox = SandboxSettings(
                    write_allow_only=[str(self.sandbox_path), "/tmp/claude/"]
                )

            async for message in query(prompt=eval_prompt, options=options):
                if hasattr(message, "result") and message.result:
                    response = message.result.strip().lower()

        except Exception as e:
            return {
                "passed": False,
                "response": "",
                "reason": f"Evaluator error: {e}"
            }

        # Normalize response
        response_normalized = response.split()[0] if response else ""
        pass_if_normalized = pass_if.lower().strip()

        passed = response_normalized.startswith(pass_if_normalized)

        return {
            "passed": passed,
            "response": response,
            "reason": None if passed else f"Expected '{pass_if}', got '{response}'"
        }


class SimpleEvaluator:
    """Non-LLM evaluator for deterministic checks."""

    @staticmethod
    def contains(text: str, pattern: str, case_insensitive: bool = False) -> bool:
        """Check if text contains pattern."""
        import re
        flags = re.IGNORECASE if case_insensitive else 0
        return bool(re.search(pattern, text, flags))

    @staticmethod
    def matches(text: str, pattern: str) -> bool:
        """Check if text matches pattern exactly."""
        import re
        return bool(re.fullmatch(pattern, text.strip()))

    @staticmethod
    def is_numeric(text: str) -> bool:
        """Check if text is a number."""
        try:
            float(text.strip())
            return True
        except ValueError:
            return False
