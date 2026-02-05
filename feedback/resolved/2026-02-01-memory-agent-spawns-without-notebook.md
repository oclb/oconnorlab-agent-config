**User:** Kangcheng Hou

## Memory Agent Spawns Without Notebook Initialization

### Problem

When the memory agent is spawned in contexts without an initialized notebook (e.g., temporary chats, projects without `/init-project` setup), it fails silently:
- Agent launches and consumes tokens without creating the entry
- Fails when executed from subdirectories (can't locate `notebook/`)
- No user feedback indicating the failure

### Root Cause

The memory agent prompt assumes the notebook exists and is in the project root. It doesn't validate prerequisites or handle working directory context.

### Suggested Solution

Add project root detection and notebook validation to the memory agent prompt template:

```bash
# Locate project root from any working directory
PROJECT_ROOT=$(git rev-parse --show-toplevel 2>/dev/null) || exit 1
cd "$PROJECT_ROOT"

# Validate notebook is initialized
if [[ ! -d notebook/.git ]]; then
  echo "Error: notebook not initialized. Run /init-project first."
  exit 1
fi

# Proceed with notebook operations
git -C notebook add entries/ INDEX.md && git -C notebook commit -m "..." && git -C notebook remote | grep -q origin && git -C notebook push
```

This handles:
- **Subdirectory execution** - Locates project root from any working directory
- **Missing notebook** - Fails fast with clear error message, prevents token waste
- **Robust git operations** - Uses `-C` for explicit directory targeting
