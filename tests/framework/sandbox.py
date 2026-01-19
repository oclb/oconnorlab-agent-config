"""
Sandboxed workspace for isolated test execution.
"""

import shutil
import tempfile
from pathlib import Path
from contextlib import contextmanager
from typing import Generator
import uuid


class SandboxedWorkspace:
    """
    Context manager for isolated test workspace.

    Creates a temporary copy of a project template, runs tests in isolation,
    and cleans up afterward. Each instance gets a unique directory to support
    parallel execution.

    Usage:
        with SandboxedWorkspace(template_path) as workspace:
            # workspace is a Path to the isolated copy
            run_test(cwd=workspace)
        # Automatically cleaned up
    """

    def __init__(self, template_path: Path, prefix: str = "claude-test-"):
        """
        Args:
            template_path: Path to the project template to copy
            prefix: Prefix for the temp directory name
        """
        self.template = Path(template_path)
        self.prefix = prefix
        self.workspace: Path | None = None
        self._unique_id = uuid.uuid4().hex[:8]

    def __enter__(self) -> Path:
        """Create isolated workspace and return path."""
        # Create unique temp directory
        self.workspace = Path(tempfile.mkdtemp(
            prefix=f"{self.prefix}{self._unique_id}-"
        ))

        # Copy template if it exists
        if self.template.exists():
            shutil.copytree(
                self.template,
                self.workspace,
                dirs_exist_ok=True,
                ignore=shutil.ignore_patterns('.git', '__pycache__', '*.pyc')
            )

        return self.workspace

    def __exit__(self, exc_type, exc_val, exc_tb):
        """Clean up workspace."""
        if self.workspace and self.workspace.exists():
            shutil.rmtree(self.workspace, ignore_errors=True)
        return False  # Don't suppress exceptions

    @property
    def id(self) -> str:
        """Unique identifier for this workspace."""
        return self._unique_id


@contextmanager
def sandboxed_workspace(
    template_path: Path,
    prefix: str = "claude-test-"
) -> Generator[Path, None, None]:
    """
    Functional context manager for sandboxed workspace.

    Args:
        template_path: Path to project template
        prefix: Prefix for temp directory

    Yields:
        Path to the isolated workspace
    """
    workspace = None
    unique_id = uuid.uuid4().hex[:8]

    try:
        workspace = Path(tempfile.mkdtemp(prefix=f"{prefix}{unique_id}-"))

        if template_path.exists():
            shutil.copytree(
                template_path,
                workspace,
                dirs_exist_ok=True,
                ignore=shutil.ignore_patterns('.git', '__pycache__', '*.pyc')
            )

        yield workspace

    finally:
        if workspace and workspace.exists():
            shutil.rmtree(workspace, ignore_errors=True)


class WorkspacePool:
    """
    Pool of reusable workspaces for parallel test execution.

    Pre-creates N workspaces and hands them out as needed.
    More efficient than creating/destroying for each test.
    """

    def __init__(self, template_path: Path, size: int = 4):
        self.template = Path(template_path)
        self.size = size
        self._workspaces: list[Path] = []
        self._available: list[Path] = []
        self._initialized = False

    def initialize(self):
        """Pre-create workspace pool."""
        if self._initialized:
            return

        for i in range(self.size):
            ws = Path(tempfile.mkdtemp(prefix=f"claude-test-pool-{i}-"))
            if self.template.exists():
                shutil.copytree(
                    self.template,
                    ws,
                    dirs_exist_ok=True,
                    ignore=shutil.ignore_patterns('.git', '__pycache__', '*.pyc')
                )
            self._workspaces.append(ws)
            self._available.append(ws)

        self._initialized = True

    @contextmanager
    def acquire(self) -> Generator[Path, None, None]:
        """
        Acquire a workspace from the pool.

        Blocks if none available (in async context, would use asyncio.Semaphore).
        Resets workspace to clean state on release.
        """
        if not self._initialized:
            self.initialize()

        if not self._available:
            raise RuntimeError("No workspaces available in pool")

        workspace = self._available.pop()

        try:
            yield workspace
        finally:
            # Reset workspace to clean state
            self._reset_workspace(workspace)
            self._available.append(workspace)

    def _reset_workspace(self, workspace: Path):
        """Reset workspace to template state."""
        # Remove all contents
        for item in workspace.iterdir():
            if item.is_dir():
                shutil.rmtree(item)
            else:
                item.unlink()

        # Re-copy template
        if self.template.exists():
            shutil.copytree(
                self.template,
                workspace,
                dirs_exist_ok=True,
                ignore=shutil.ignore_patterns('.git', '__pycache__', '*.pyc')
            )

    def cleanup(self):
        """Destroy all workspaces in pool."""
        for ws in self._workspaces:
            if ws.exists():
                shutil.rmtree(ws, ignore_errors=True)
        self._workspaces.clear()
        self._available.clear()
        self._initialized = False

    def __enter__(self):
        self.initialize()
        return self

    def __exit__(self, *args):
        self.cleanup()
