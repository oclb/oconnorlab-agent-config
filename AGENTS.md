# Lab Agent Config Project Instructions

## Terminology

- `Claude tree`: `claude/`, the Claude Code configuration surface.
- `Codex tree`: `codex/`, the Codex configuration surface.
- `shared infrastructure`: repo-root tooling used by both agent trees, especially `bin/config-agent-tool` and `remote-bridge/`.

## Scope Separation

Keep Claude-specific content in `claude/` and Codex-specific content in `codex/`. Put only genuinely shared tooling or docs at the repository root.

When changing a reusable workflow, update both agent trees unless there is a product-specific reason for divergence. Preserve those divergences explicitly in wording, not by letting the trees drift accidentally.

## Setup Tooling

`bin/config-agent-tool` is the unified installer. Agent-specific commands require `--agent claude` or `--agent codex`; do not add implicit default agent behavior.

## Remote Bridge

`remote-bridge/` is product-neutral shared infrastructure. Do not introduce Claude-only or Codex-only runtime paths in bridge code unless the interface is explicitly parameterized.
