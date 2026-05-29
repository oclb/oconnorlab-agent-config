# Lab Agent Config Repository

This repository maintains configuration surfaces for both Claude Code and Codex. Keep product-specific prompt content in the matching agent tree and shared implementation infrastructure at the repository root.

## Layout

| Path | Purpose |
| --- | --- |
| `claude/` | Claude Code global prompt, settings, hooks, templates, and skills. |
| `codex/` | Codex global instructions, templates, repo-local setup support, and skills. |
| `bin/config-agent-tool` | Unified installer and skill-link manager. Commands require `--agent claude` or `--agent codex`. |
| `remote-bridge/` | Product-neutral O2/SSH bridge shared by both agents. |
| `tests/` | Installer tests plus the pruned legacy Claude behavior-test harness. |

## Maintenance Rules

1. When changing reusable workflow content, update both `claude/` and `codex/` unless there is a product-specific reason to diverge.
2. Keep Claude wording Claude-native: `/skill`, `CLAUDE.md`, `~/.claude`, Claude hooks/settings.
3. Keep Codex wording Codex-native: `$skill`, `AGENTS.md`, `~/.codex`, `.agents/skills`.
4. Keep `remote-bridge/` agent-neutral; do not introduce Claude-only or Codex-only runtime paths.
5. Do not reintroduce the dropped old flat Claude skills without an explicit decision to rebuild an analysis taxonomy.

## Validation

For installer changes, run:

```bash
PYTHONDONTWRITEBYTECODE=1 python3 -m unittest tests/test_config_agent_tool.py
```

For bridge changes, run from `remote-bridge/`:

```bash
cargo fmt --check
cargo test
```
