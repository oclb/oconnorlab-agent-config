from __future__ import annotations

import contextlib
import importlib.machinery
import importlib.util
import io
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock


REPO_ROOT = Path(__file__).resolve().parents[1]
TOOL_PATH = REPO_ROOT / "bin" / "config-agent-tool"
CommandResult = int | str | subprocess.CalledProcessError | None


def load_tool():
    loader = importlib.machinery.SourceFileLoader(
        "config_agent_tool_under_test", str(TOOL_PATH)
    )
    spec = importlib.util.spec_from_loader(loader.name, loader)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"Could not load {TOOL_PATH}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


@contextlib.contextmanager
def chdir(path: Path):
    previous = Path.cwd()
    os.chdir(path)
    try:
        yield
    finally:
        os.chdir(previous)


class ConfigAgentToolTests(unittest.TestCase):
    def setUp(self) -> None:
        self.tempdir = tempfile.TemporaryDirectory()
        self.root = Path(self.tempdir.name)
        self.codex_home = self.root / "codex-home"
        self.claude_home = self.root / "claude-home"
        self.env = mock.patch.dict(
            os.environ,
            {
                "CODEX_HOME": str(self.codex_home),
                "CLAUDE_HOME": str(self.claude_home),
                "PATH": os.pathsep.join(["/usr/bin", "/bin"]),
            },
            clear=False,
        )
        self.env.start()
        self.tool = load_tool()

    def tearDown(self) -> None:
        self.env.stop()
        self.tempdir.cleanup()

    def invoke(self, *args: str) -> tuple[CommandResult, str, str]:
        stdout = io.StringIO()
        stderr = io.StringIO()
        with (
            mock.patch.object(sys, "argv", ["config-agent-tool", *args]),
            contextlib.redirect_stdout(stdout),
            contextlib.redirect_stderr(stderr),
        ):
            try:
                code = self.tool.main()
            except SystemExit as exc:
                code = exc.code
            except subprocess.CalledProcessError as exc:
                code = exc
        return code, stdout.getvalue(), stderr.getvalue()

    def assert_symlink_target(self, link: Path, target: Path) -> None:
        self.assertTrue(link.is_symlink(), f"{link} is not a symlink")
        self.assertEqual(link.resolve(strict=False), target.resolve(strict=False))

    def test_agent_is_required_for_install(self) -> None:
        code, stdout, stderr = self.invoke("install")

        self.assertEqual(code, 2)
        self.assertEqual(stdout, "")
        self.assertIn("--agent", stderr)

    def test_codex_install_renders_override_and_links_tool(self) -> None:
        code, stdout, stderr = self.invoke("install", "--agent", "codex")

        self.assertEqual(code, 0)
        self.assertEqual(stderr, "")
        self.assert_symlink_target(
            self.codex_home / "bin" / "config-agent-tool",
            REPO_ROOT / "bin" / "config-agent-tool",
        )
        override = (self.codex_home / "AGENTS.override.md").read_text(encoding="utf-8")
        content_agents = (REPO_ROOT / "codex" / "global" / "AGENTS.md").read_text(encoding="utf-8")
        self.assertIn(self.tool.GENERATED_MARKER, override)
        self.assertIn("# Shared Codex Instructions\n\n" + content_agents, override)
        self.assertIn("No skills were installed automatically", stdout)

    def test_claude_install_adds_import_and_links_settings_hooks_and_tool(self) -> None:
        code, stdout, stderr = self.invoke("install", "--agent", "claude")

        self.assertEqual(code, 0)
        self.assertEqual(stderr, "")
        claude_md = (self.claude_home / "CLAUDE.md").read_text(encoding="utf-8")
        self.assertIn(f"@{REPO_ROOT / 'claude' / 'global' / 'CLAUDE.md'}", claude_md)
        self.assert_symlink_target(
            self.claude_home / "settings.json",
            REPO_ROOT / "claude" / "global" / "settings.json",
        )
        self.assert_symlink_target(self.claude_home / "hooks", REPO_ROOT / "claude" / "hooks")
        self.assert_symlink_target(
            self.claude_home / "bin" / "config-agent-tool",
            REPO_ROOT / "bin" / "config-agent-tool",
        )
        self.assertIn("No skills were installed automatically", stdout)

    def test_link_skills_uses_agent_specific_tree(self) -> None:
        code, stdout, stderr = self.invoke(
            "link-skills", "--agent", "claude", "--global", "--add", "work-cycle"
        )

        self.assertEqual(code, 0)
        self.assertEqual(stderr, "")
        self.assert_symlink_target(
            self.claude_home / "skills" / "work-cycle",
            REPO_ROOT / "claude" / "skills" / "work-cycle",
        )
        self.assertIn("Creating symlink:", stdout)

    def test_project_list_skills_hides_globally_installed_skills(self) -> None:
        (self.codex_home / "skills").mkdir(parents=True)
        (self.codex_home / "skills" / "work-cycle").symlink_to(
            REPO_ROOT / "codex" / "skills" / "work-cycle"
        )
        project = self.root / "project"
        project.mkdir()

        with chdir(project):
            code, stdout, stderr = self.invoke("list-skills", "--agent", "codex")

        self.assertEqual(code, 0)
        self.assertEqual(stderr, "")
        listed_skill_names = {
            line.split()[0]
            for line in stdout.splitlines()
            if line and not line.startswith(("Agent:", "Target:", "skill", "-"))
        }
        self.assertNotIn("work-cycle", listed_skill_names)

    def test_update_stops_when_git_pull_fails(self) -> None:
        calls = []

        def fail_pull(command, *args, **kwargs):
            calls.append(command)
            if command[:4] == ["git", "-C", str(REPO_ROOT), "pull"]:
                raise subprocess.CalledProcessError(returncode=1, cmd=command)
            return subprocess.CompletedProcess(command, 0, stdout="", stderr="")

        with mock.patch.object(self.tool.subprocess, "run", side_effect=fail_pull):
            code, stdout, stderr = self.invoke("update", "--agent", "codex")

        self.assertIsInstance(code, subprocess.CalledProcessError)
        self.assertEqual(stdout, "")
        self.assertEqual(stderr, "")
        self.assertEqual(calls, [["git", "-C", str(REPO_ROOT), "pull", "--ff-only"]])
        self.assertFalse((self.codex_home / "AGENTS.override.md").exists())


if __name__ == "__main__":
    unittest.main()
