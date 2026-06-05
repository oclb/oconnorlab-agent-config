from __future__ import annotations

import contextlib
import importlib.machinery
import importlib.util
import io
import json
import os
import subprocess
import sys
import tempfile
import time
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
        self.assert_symlink_target(
            self.codex_home / "hooks.json",
            REPO_ROOT / "codex" / "global" / "hooks.json",
        )
        self.assert_symlink_target(self.codex_home / "hooks", REPO_ROOT / "codex" / "hooks")
        override = (self.codex_home / "AGENTS.override.md").read_text(encoding="utf-8")
        content_agents = (REPO_ROOT / "codex" / "global" / "AGENTS.md").read_text(encoding="utf-8")
        hooks_json = json.loads((self.codex_home / "hooks.json").read_text(encoding="utf-8"))
        command = hooks_json["hooks"]["SessionStart"][0]["hooks"][0]["command"]
        self.assertIn(self.tool.GENERATED_MARKER, override)
        self.assertIn("# Shared Codex Instructions\n\n" + content_agents, override)
        self.assertIn("CODEX_HOME", command)
        self.assertIn("No skills were installed automatically", stdout)

    def test_codex_install_refuses_unmanaged_hooks_json(self) -> None:
        self.codex_home.mkdir(parents=True)
        unmanaged = self.codex_home / "hooks.json"
        unmanaged.write_text('{"hooks": {}}\n', encoding="utf-8")

        code, stdout, stderr = self.invoke("install", "--agent", "codex")

        self.assertIsInstance(code, str)
        self.assertIn("Refusing to overwrite existing unmanaged hooks.json", code)
        self.assertIn("manually merge", code)
        self.assertIn("rerun install --agent codex", code)
        self.assertEqual(stderr, "")
        self.assertEqual(unmanaged.read_text(encoding="utf-8"), '{"hooks": {}}\n')
        self.assertTrue((self.codex_home / "AGENTS.override.md").exists())
        self.assert_symlink_target(
            self.codex_home / "bin" / "config-agent-tool",
            REPO_ROOT / "bin" / "config-agent-tool",
        )
        self.assert_symlink_target(self.codex_home / "hooks", REPO_ROOT / "codex" / "hooks")
        self.assertIn("Rendered", stdout)

    def test_codex_update_warns_and_keeps_unmanaged_hooks_directory(self) -> None:
        self.codex_home.mkdir(parents=True)
        (self.codex_home / "user").mkdir()
        (self.codex_home / "bin").mkdir()
        user_hooks = self.codex_home / "hooks"
        user_hooks.mkdir()
        (user_hooks / "custom.sh").write_text("#!/bin/sh\n", encoding="utf-8")
        (self.codex_home / "user" / "AGENTS.md").write_text(
            "Personal instructions.\n", encoding="utf-8"
        )
        (self.codex_home / "bin" / "config-agent-tool").symlink_to(
            REPO_ROOT / "bin" / "config-agent-tool"
        )

        with mock.patch.object(self.tool, "git_pull_ff_only") as pull:
            code, stdout, stderr = self.invoke("update", "--agent", "codex")

        self.assertEqual(code, 0)
        self.assertIn("Warning: leaving unmanaged Codex hooks directory in place", stderr)
        pull.assert_called_once_with()
        self.assertFalse(user_hooks.is_symlink())
        self.assertTrue((user_hooks / "custom.sh").exists())
        self.assert_symlink_target(
            self.codex_home / "hooks.json",
            REPO_ROOT / "codex" / "global" / "hooks.json",
        )
        self.assertTrue((self.codex_home / "AGENTS.override.md").exists())
        self.assertIn("Rendered", stdout)

    def test_codex_install_accepts_unmanaged_hooks_json_after_manual_merge(self) -> None:
        self.codex_home.mkdir(parents=True)
        unmanaged = self.codex_home / "hooks.json"
        unmanaged.write_text(
            json.dumps(
                {
                    "hooks": {
                        "SessionStart": [
                            {
                                "matcher": "startup",
                                "hooks": [
                                    {
                                        "type": "command",
                                        "command": self.tool.CODEX_UPDATE_HOOK_COMMAND,
                                    }
                                ],
                            }
                        ]
                    }
                }
            )
            + "\n",
            encoding="utf-8",
        )

        code, stdout, stderr = self.invoke("install", "--agent", "codex")

        self.assertEqual(code, 0)
        self.assertEqual(stderr, "")
        self.assertFalse(unmanaged.is_symlink())
        self.assert_symlink_target(self.codex_home / "hooks", REPO_ROOT / "codex" / "hooks")
        self.assertIn("Leaving unmanaged Codex hooks.json with startup update hook", stdout)

    def test_codex_install_refuses_unmanaged_hooks_directory(self) -> None:
        user_hooks = self.codex_home / "hooks"
        user_hooks.mkdir(parents=True)
        (user_hooks / "custom.sh").write_text("#!/bin/sh\n", encoding="utf-8")

        code, stdout, stderr = self.invoke("install", "--agent", "codex")

        self.assertIsInstance(code, str)
        self.assertIn("Refusing to overwrite existing unmanaged hooks directory", code)
        self.assertEqual(stderr, "")
        self.assertFalse(user_hooks.is_symlink())
        self.assertTrue((user_hooks / "custom.sh").exists())
        self.assert_symlink_target(
            self.codex_home / "hooks.json",
            REPO_ROOT / "codex" / "global" / "hooks.json",
        )
        self.assertIn("Rendered", stdout)

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

    def test_claude_install_replaces_stale_flat_import_without_duplicate(self) -> None:
        self.claude_home.mkdir(parents=True)
        stale_import = f"@{REPO_ROOT / 'global' / 'CLAUDE.md'}"
        current_import = f"@{REPO_ROOT / 'claude' / 'global' / 'CLAUDE.md'}"
        (self.claude_home / "CLAUDE.md").write_text(
            "# User Claude Configuration\n\n"
            "# Import shared lab agent configuration\n"
            f"{stale_import}\n"
            f"{current_import}\n",
            encoding="utf-8",
        )

        code, stdout, stderr = self.invoke("install", "--agent", "claude")

        self.assertEqual(code, 0)
        self.assertEqual(stderr, "")
        claude_md = (self.claude_home / "CLAUDE.md").read_text(encoding="utf-8")
        self.assertNotIn(stale_import, claude_md)
        self.assertEqual(claude_md.count(current_import), 1)
        self.assertIn("Repaired stale Claude import", stdout)

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

    def test_link_skills_add_all_links_every_known_skill(self) -> None:
        code, stdout, stderr = self.invoke(
            "link-skills", "--agent", "codex", "--global", "--add-all"
        )

        self.assertEqual(code, 0)
        self.assertEqual(stderr, "")
        expected_skills = {
            skill.name: skill for skill in self.tool.skills(self.tool.agent_config("codex"))
        }
        expected_names = set(expected_skills)
        actual_names = {path.name for path in (self.codex_home / "skills").iterdir()}
        self.assertEqual(actual_names, expected_names)
        for name, skill in expected_skills.items():
            self.assert_symlink_target(
                self.codex_home / "skills" / name,
                skill.path,
            )
        self.assertIn("Creating symlink:", stdout)

    def test_link_skills_remove_all_removes_only_managed_links(self) -> None:
        target = self.codex_home / "skills"
        target.mkdir(parents=True)
        (target / "work-cycle").symlink_to(REPO_ROOT / "codex" / "skills" / "work-cycle")
        (target / "stale").symlink_to(REPO_ROOT / "codex" / "skills" / "stale")
        unmanaged = target / "unmanaged"
        unmanaged.mkdir()

        code, stdout, stderr = self.invoke(
            "link-skills", "--agent", "codex", "--global", "--remove-all"
        )

        self.assertEqual(code, 0)
        self.assertEqual(stderr, "")
        self.assertFalse((target / "work-cycle").exists())
        self.assertFalse((target / "stale").is_symlink())
        self.assertTrue(unmanaged.exists())
        self.assertIn("Removing symlink: codex skill 'stale'", stdout)
        self.assertIn("Removing symlink: codex skill 'work-cycle'", stdout)

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
        pull_error = subprocess.CalledProcessError(
            returncode=1, cmd=["git", "-C", str(REPO_ROOT), "pull", "--ff-only"]
        )

        with mock.patch.object(self.tool, "git_pull_ff_only", side_effect=pull_error) as pull:
            code, stdout, stderr = self.invoke("update", "--agent", "codex")

        self.assertIsInstance(code, subprocess.CalledProcessError)
        self.assertEqual(stdout, "")
        self.assertEqual(stderr, "")
        pull.assert_called_once_with()
        self.assertFalse((self.codex_home / "AGENTS.override.md").exists())

    def test_codex_update_repairs_managed_surface_links_after_pull(self) -> None:
        code, stdout, stderr = self.invoke("install", "--agent", "codex")
        self.assertEqual(code, 0)
        self.assertEqual(stderr, "")
        (self.codex_home / "hooks.json").unlink()
        (self.codex_home / "hooks.json").symlink_to(REPO_ROOT / "global" / "hooks.json")
        (self.codex_home / "hooks").unlink()
        (self.codex_home / "hooks").symlink_to(REPO_ROOT / "hooks")
        (self.codex_home / "bin" / "config-agent-tool").unlink()

        with mock.patch.object(self.tool, "git_pull_ff_only") as pull:
            code, stdout, stderr = self.invoke("update", "--agent", "codex")

        self.assertEqual(code, 0)
        self.assertEqual(stderr, "")
        pull.assert_called_once_with()
        self.assert_symlink_target(
            self.codex_home / "hooks.json",
            REPO_ROOT / "codex" / "global" / "hooks.json",
        )
        self.assert_symlink_target(self.codex_home / "hooks", REPO_ROOT / "codex" / "hooks")
        self.assert_symlink_target(
            self.codex_home / "bin" / "config-agent-tool",
            REPO_ROOT / "bin" / "config-agent-tool",
        )
        self.assertIn("Replacing symlink: Codex hooks.json", stdout)
        self.assertIn("Replacing symlink: Codex hooks", stdout)
        self.assertIn("Creating symlink:", stdout)

    def test_codex_update_warns_and_keeps_unmanaged_hooks_json(self) -> None:
        self.codex_home.mkdir(parents=True)
        (self.codex_home / "user").mkdir()
        (self.codex_home / "bin").mkdir()
        unmanaged = self.codex_home / "hooks.json"
        unmanaged.write_text('{"hooks": {"Stop": []}}\n', encoding="utf-8")
        (self.codex_home / "user" / "AGENTS.md").write_text(
            "Personal instructions.\n", encoding="utf-8"
        )
        (self.codex_home / "bin" / "config-agent-tool").symlink_to(
            REPO_ROOT / "bin" / "config-agent-tool"
        )

        with mock.patch.object(self.tool, "git_pull_ff_only") as pull:
            code, stdout, stderr = self.invoke("update", "--agent", "codex")

        self.assertEqual(code, 0)
        self.assertIn("Warning: leaving unmanaged Codex hooks.json in place", stderr)
        pull.assert_called_once_with()
        self.assertEqual(unmanaged.read_text(encoding="utf-8"), '{"hooks": {"Stop": []}}\n')
        self.assert_symlink_target(self.codex_home / "hooks", REPO_ROOT / "codex" / "hooks")
        self.assertTrue((self.codex_home / "AGENTS.override.md").exists())
        self.assertIn("Rendered", stdout)

    def test_claude_update_repairs_legacy_flat_layout_paths(self) -> None:
        self.claude_home.mkdir(parents=True)
        (self.claude_home / "CLAUDE.md").write_text(
            "# User Claude Configuration\n\n"
            "# Import shared lab agent configuration\n"
            f"@{REPO_ROOT / 'global' / 'CLAUDE.md'}\n",
            encoding="utf-8",
        )
        (self.claude_home / "settings.json").symlink_to(
            REPO_ROOT / "global" / "settings.json"
        )
        (self.claude_home / "hooks").symlink_to(REPO_ROOT / "hooks")

        with mock.patch.object(self.tool, "git_pull_ff_only") as pull:
            code, stdout, stderr = self.invoke("update", "--agent", "claude")

        self.assertEqual(code, 0)
        self.assertEqual(stderr, "")
        pull.assert_called_once_with()
        stale_import = f"@{REPO_ROOT / 'global' / 'CLAUDE.md'}"
        current_import = f"@{REPO_ROOT / 'claude' / 'global' / 'CLAUDE.md'}"
        claude_md = (self.claude_home / "CLAUDE.md").read_text(encoding="utf-8")
        self.assertNotIn(stale_import, claude_md)
        self.assertEqual(claude_md.count(current_import), 1)
        self.assert_symlink_target(
            self.claude_home / "settings.json",
            REPO_ROOT / "claude" / "global" / "settings.json",
        )
        self.assert_symlink_target(self.claude_home / "hooks", REPO_ROOT / "claude" / "hooks")
        self.assert_symlink_target(
            self.claude_home / "bin" / "config-agent-tool",
            REPO_ROOT / "bin" / "config-agent-tool",
        )
        self.assertIn("Repaired stale Claude import", stdout)
        self.assertIn("Replacing symlink: Claude settings", stdout)
        self.assertIn("Replacing symlink: Claude hooks", stdout)

    def test_codex_set_me_up_onboarding_uses_merged_layout_commands(self) -> None:
        onboarding_files = [
            REPO_ROOT / ".agents" / "skills" / "set-me-up" / "SKILL.md",
            REPO_ROOT
            / ".agents"
            / "skills"
            / "set-me-up"
            / "references"
            / "onboarding-script.md",
        ]
        text = "\n".join(path.read_text(encoding="utf-8") for path in onboarding_files)

        self.assertIn("codex/skills", text)
        self.assertIn("codex/global/AGENTS.md", text)
        self.assertIn("Auto-trigger", text)
        self.assertIn("set up, initialize, install, onboard, configure", text)
        self.assertIn("install --agent codex", text)
        self.assertIn("~/.codex/hooks.json", text)
        self.assertIn("~/.codex/hooks/update-config.sh", text)
        self.assertIn("SessionStart", text)
        self.assertIn("/hooks", text)
        self.assertIn("does not edit `~/.codex/config.toml`", text)
        self.assertIn("list-skills --agent codex --global", text)
        self.assertIn("link-skills --agent codex --global", text)
        self.assertIn("$work-cycle", text)
        self.assertNotIn("test -d skills", text)
        self.assertNotIn("global/AGENTS.md", text.replace("codex/global/AGENTS.md", ""))
        self.assertNotIn("install --global", text)
        self.assertNotIn("list-skills --global", text)
        self.assertNotIn("link-skills --global", text)
        self.assertNotIn("$software", text)

    def test_codex_update_hook_invokes_update_silently_and_returns_quickly(self) -> None:
        fake_home = self.root / "fake-codex-home"
        fake_bin = fake_home / "bin"
        fake_bin.mkdir(parents=True)
        fake_tool = fake_bin / "config-agent-tool"
        fake_tool.write_text(
            "#!/bin/sh\n"
            "printf '%s\\n' \"$*\" > \"$CODEX_HOME/invocation.log\"\n"
            "echo visible stdout\n"
            "echo visible stderr >&2\n"
            "exit 7\n",
            encoding="utf-8",
        )
        fake_tool.chmod(0o755)
        env = {
            **os.environ,
            "CODEX_HOME": str(fake_home),
            "HOME": str(self.root / "home"),
        }

        start = time.monotonic()
        result = subprocess.run(
            ["sh", str(REPO_ROOT / "codex" / "hooks" / "update-config.sh")],
            check=False,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            env=env,
            timeout=3,
        )
        elapsed = time.monotonic() - start

        self.assertEqual(result.returncode, 0)
        self.assertEqual(result.stdout, "")
        self.assertEqual(result.stderr, "")
        self.assertLess(elapsed, 2)
        self.assertEqual(
            (fake_home / "invocation.log").read_text(encoding="utf-8").strip(),
            "update --agent codex",
        )

    def test_codex_hooks_json_command_runs_installed_hook(self) -> None:
        code, stdout, stderr = self.invoke("install", "--agent", "codex")
        self.assertEqual(code, 0)
        self.assertEqual(stderr, "")
        fake_tool = self.codex_home / "bin" / "config-agent-tool"
        fake_tool.unlink()
        fake_tool.write_text(
            "#!/bin/sh\n"
            "printf '%s\\n' \"$*\" > \"$CODEX_HOME/installed-command.log\"\n",
            encoding="utf-8",
        )
        fake_tool.chmod(0o755)
        hooks_json = json.loads((self.codex_home / "hooks.json").read_text(encoding="utf-8"))
        command = hooks_json["hooks"]["SessionStart"][0]["hooks"][0]["command"]
        env = {**os.environ, "CODEX_HOME": str(self.codex_home)}

        result = subprocess.run(
            command,
            shell=True,
            check=False,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            env=env,
            timeout=3,
        )

        self.assertEqual(result.returncode, 0)
        self.assertEqual(result.stdout, "")
        self.assertEqual(result.stderr, "")
        self.assertEqual(
            (self.codex_home / "installed-command.log").read_text(encoding="utf-8").strip(),
            "update --agent codex",
        )

    def test_codex_update_hook_kills_hung_update_child(self) -> None:
        fake_home = self.root / "hung-codex-home"
        fake_bin = fake_home / "bin"
        fake_bin.mkdir(parents=True)
        fake_tool = fake_bin / "config-agent-tool"
        fake_tool.write_text(
            "#!/bin/sh\n"
            "(i=0; while :; do i=$((i + 1)); "
            "printf '%s\\n' \"$i\" > \"$CODEX_HOME/heartbeat\"; sleep 1; done) &\n"
            "child=$!\n"
            "printf '%s\\n' \"$child\" > \"$CODEX_HOME/child.pid\"\n"
            "trap 'kill \"$child\" 2>/dev/null; exit 143' TERM INT\n"
            "wait \"$child\"\n",
            encoding="utf-8",
        )
        fake_tool.chmod(0o755)
        env = {
            **os.environ,
            "CODEX_HOME": str(fake_home),
            "HOME": str(self.root / "home"),
        }

        start = time.monotonic()
        result = subprocess.run(
            ["sh", str(REPO_ROOT / "codex" / "hooks" / "update-config.sh")],
            check=False,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            env=env,
            timeout=10,
        )
        elapsed = time.monotonic() - start

        self.assertEqual(result.returncode, 0)
        self.assertEqual(result.stdout, "")
        self.assertEqual(result.stderr, "")
        self.assertLess(elapsed, 8)
        child_pid = int((fake_home / "child.pid").read_text(encoding="utf-8"))
        heartbeat = fake_home / "heartbeat"
        first_heartbeat = heartbeat.read_text(encoding="utf-8")
        time.sleep(2)
        second_heartbeat = heartbeat.read_text(encoding="utf-8")
        if second_heartbeat != first_heartbeat:
            with contextlib.suppress(OSError):
                os.kill(child_pid, 9)
        self.assertEqual(second_heartbeat, first_heartbeat)

    def test_claude_set_me_up_onboarding_uses_merged_layout_commands(self) -> None:
        onboarding_files = [
            REPO_ROOT / ".claude" / "skills" / "set-me-up" / "SKILL.md",
            REPO_ROOT
            / ".claude"
            / "skills"
            / "set-me-up"
            / "references"
            / "onboarding-script.md",
        ]
        text = "\n".join(path.read_text(encoding="utf-8") for path in onboarding_files)

        self.assertIn("claude/skills", text)
        self.assertIn("claude/global/CLAUDE.md", text)
        self.assertIn("Auto-trigger", text)
        self.assertIn("set up, initialize, install, onboard, configure", text)
        self.assertIn("install --agent claude", text)
        self.assertIn("list-skills --agent claude --global", text)
        self.assertIn("link-skills --agent claude --global", text)
        self.assertIn("/work-cycle", text)
        self.assertNotIn("test -d skills", text)
        self.assertNotIn("global/CLAUDE.md", text.replace("claude/global/CLAUDE.md", ""))
        self.assertNotIn("install --global", text)
        self.assertNotIn("list-skills --global", text)
        self.assertNotIn("link-skills --global", text)
        self.assertNotIn("$work-cycle", text)


if __name__ == "__main__":
    unittest.main()
