# Source Map

This merged repository combines the previous `claude-config` and `codex-config` repositories.

## Sources

- `claude/` is adapted from the older Claude configuration plus the newer Codex skill taxonomy.
- `codex/` is copied from `codex-config` as the current Codex content source.
- `remote-bridge/`, `.github/`, hooks, Claude settings, and the Claude behavior-test framework come from `claude-config`.
- `bin/config-agent-tool` is generalized from the newer Codex setup tool.

## Intentional omissions

The first merge intentionally drops old flat Claude-only analysis and personal skills: `perform-analysis`, `new-data`, `new-software`, `implement`, `update-notebook`, `teaching-mode`, `figure-it-out`, `revise-scientific-writing`, and `todoist-cli`.
