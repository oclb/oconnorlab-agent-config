import unittest
from pathlib import Path

import yaml


ROOT = Path(__file__).resolve().parents[1]
SKILL_DIR = ROOT / "codex" / "skills" / "delegator"


class DelegatorSkillTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.skill = (SKILL_DIR / "SKILL.md").read_text(encoding="utf-8")
        cls.metadata = yaml.safe_load(
            (SKILL_DIR / "agents" / "openai.yaml").read_text(encoding="utf-8")
        )

    def test_is_user_scoped_and_manual_only(self) -> None:
        self.assertIn("recommended_scope: global", self.skill)
        self.assertIn("disable-model-invocation: true", self.skill)
        self.assertFalse(self.metadata["policy"]["allow_implicit_invocation"])
        self.assertIn("only through explicit `$delegator`", self.skill)

    def test_lists_without_dispatching(self) -> None:
        self.assertIn('todo\" list --json', self.skill)
        self.assertIn("every record in `active`", self.skill)
        self.assertIn("Preserve `#<id>`, title, description", self.skill)
        self.assertIn("listing a todo never authorizes doing or dispatching it", self.skill)

    def test_dispatches_user_owned_tasks_with_required_context(self) -> None:
        for required in (
            "`list_projects`",
            "`create_thread`",
            "`set_thread_title`",
            "`set_thread_pinned`",
            "`wait_threads`",
            "exact todo record",
            "full linked context entry",
            "project instructions",
            "worktrees, branches, PRs",
            "requested finish line",
            "environment.type: worktree",
        ):
            self.assertIn(required, self.skill)
        self.assertIn("never use a subagent", self.skill)
        self.assertIn("TODO #<id>: <todo title>", self.skill)

    def test_prevents_duplicate_or_conflicting_writers(self) -> None:
        self.assertIn("Never create a duplicate", self.skill)
        self.assertIn("`read_thread`", self.skill)
        self.assertIn("`send_message_to_thread`", self.skill)
        self.assertIn("instead of creating a conflicting writer", self.skill)

    def test_autonomous_finish_requires_review_with_boundaries(self) -> None:
        self.assertIn("separate fresh-context reviewer", self.skill)
        self.assertIn("no implementation history", self.skill)
        self.assertIn("iteration on its actionable findings", self.skill)
        self.assertIn("final revalidation", self.skill)
        self.assertIn("when Luke's finish line authorizes it", self.skill)

    def test_queued_creation_stays_pending_until_titled_and_pinned(self) -> None:
        self.assertIn("A queued `clientThreadId` is not a ready task id", self.skill)
        self.assertIn("Do not pass it to ready-task tools", self.skill)
        self.assertIn("keep the coordinator turn open", self.skill)
        self.assertIn("until the resulting `threadId` appears", self.skill)
        self.assertIn("Only after every created task is titled and pinned", self.skill)


if __name__ == "__main__":
    unittest.main()
