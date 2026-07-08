import json
import subprocess
import tempfile
import unittest
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
TODO_TOOL = REPO_ROOT / "bin" / "todo"

LEGACY_TODO = """# To-Do

Next ID: 5

- [ ] #3 **Fix the frobnicator** - It frobnicates twice
  - Context: `notebook/entries/2026-01-01-frobnicator`
  - Added: 2026-01-01

- [ ] #4 **Write docs**
  - Added: 2026-01-02
"""

LEGACY_DONE = """# Completed

- [x] #1 **Old task** - Done long ago
  - Added: 2025-12-01
  - Completed: 2025-12-05
  - Result: `notebook/entries/2025-12-05-old-task`
"""


class TodoToolTest(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.project = Path(self._tmp.name) / "project"
        self.notebook = self.project / "notebook"
        self.notebook.mkdir(parents=True)
        self._git("init", "-q")
        self._git("config", "user.email", "test@example.com")
        self._git("config", "user.name", "Test")
        (self.notebook / "INDEX.md").write_text("# Notebook Index\n")
        self._git("add", "-A")
        self._git("commit", "-q", "-m", "init")

    def tearDown(self):
        self._tmp.cleanup()

    def _git(self, *args):
        subprocess.run(["git", "-C", str(self.notebook), *args], check=True)

    def _todo(self, *args, cwd=None, check=True):
        return subprocess.run(
            ["python3", str(TODO_TOOL), *args],
            cwd=str(cwd or self.project),
            capture_output=True,
            text=True,
            check=check,
        )

    def _data(self):
        return json.loads((self.notebook / "todos.json").read_text())

    def _last_commit_message(self):
        result = subprocess.run(
            ["git", "-C", str(self.notebook), "log", "-1", "--format=%s"],
            capture_output=True,
            text=True,
            check=True,
        )
        return result.stdout.strip()

    def test_add_creates_and_commits(self):
        out = self._todo("add", "First task", "--description", "Do the thing")
        self.assertIn("added #1", out.stdout)
        data = self._data()
        self.assertEqual(data["next_id"], 2)
        self.assertEqual(data["active"][0]["title"], "First task")
        self.assertEqual(data["active"][0]["description"], "Do the thing")
        self.assertEqual(self._last_commit_message(), "todo: add #1 - First task")

    def test_complete_moves_item_with_result(self):
        self._todo("add", "Task")
        self._todo("complete", "1", "--result", "notebook/entries/x")
        data = self._data()
        self.assertEqual(data["active"], [])
        self.assertEqual(data["done"][0]["result"], "notebook/entries/x")
        self.assertIn("completed", data["done"][0])
        self.assertEqual(self._last_commit_message(), "todo: complete #1 - Task")

    def test_edit_and_delete(self):
        self._todo("add", "Task")
        self._todo("edit", "1", "--title", "Renamed", "--context", "notebook/entries/y")
        data = self._data()
        self.assertEqual(data["active"][0]["title"], "Renamed")
        self.assertEqual(data["active"][0]["context"], "notebook/entries/y")
        self._todo("delete", "1", "--reason", "obsolete")
        self.assertEqual(self._data()["active"], [])

    def test_ids_not_reused_after_delete(self):
        self._todo("add", "A")
        self._todo("delete", "1")
        self._todo("add", "B")
        self.assertEqual(self._data()["active"][0]["id"], 2)

    def test_list_reads_without_creating_json(self):
        (self.notebook / "TODO.md").write_text(LEGACY_TODO)
        out = self._todo("list")
        self.assertIn("#3 Fix the frobnicator", out.stdout)
        self.assertFalse((self.notebook / "todos.json").exists())

    def test_migrate_parses_legacy_and_removes_markdown(self):
        (self.notebook / "TODO.md").write_text(LEGACY_TODO)
        (self.notebook / "DONE.md").write_text(LEGACY_DONE)
        self._git("add", "-A")
        self._git("commit", "-q", "-m", "legacy todos")
        self._todo("migrate")
        data = self._data()
        self.assertEqual(data["next_id"], 5)
        self.assertEqual([i["id"] for i in data["active"]], [3, 4])
        self.assertEqual(
            data["active"][0]["context"], "notebook/entries/2026-01-01-frobnicator"
        )
        self.assertEqual(data["done"][0]["result"], "notebook/entries/2025-12-05-old-task")
        self.assertFalse((self.notebook / "TODO.md").exists())
        self.assertFalse((self.notebook / "DONE.md").exists())
        self.assertEqual(self._last_commit_message(), "todo: migrate to todos.json")

    def test_mutation_auto_migrates_legacy(self):
        (self.notebook / "TODO.md").write_text(LEGACY_TODO)
        self._git("add", "-A")
        self._git("commit", "-q", "-m", "legacy todos")
        self._todo("add", "New task")
        data = self._data()
        self.assertEqual([i["id"] for i in data["active"]], [3, 4, 5])
        self.assertFalse((self.notebook / "TODO.md").exists())

    def test_next_id_repaired_when_missing(self):
        (self.notebook / "TODO.md").write_text(
            "# To-Do\n\n- [ ] #7 **Task** - No counter here\n  - Added: 2026-01-01\n"
        )
        self._todo("migrate")
        self.assertEqual(self._data()["next_id"], 8)

    def test_notebook_discovery_from_subdirectory(self):
        subdir = self.project / "src" / "deep"
        subdir.mkdir(parents=True)
        self._todo("add", "From deep", cwd=subdir)
        self.assertEqual(self._data()["active"][0]["title"], "From deep")

    def test_missing_id_fails(self):
        self._todo("add", "Task")
        result = self._todo("complete", "9", check=False)
        self.assertEqual(result.returncode, 1)
        self.assertIn("no todo #9", result.stderr)


if __name__ == "__main__":
    unittest.main()
