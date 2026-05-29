# Lab Agent Config

This repository configures AI coding agents for scientific research workflows. It supports separate Claude Code and Codex configuration surfaces while keeping their content aligned unless the products require different behavior.

The repository has three major pieces:

1. **Agent configuration trees.** `claude/` contains Claude Code prompts, settings, hooks, and skills. `codex/` contains Codex global instructions, setup templates, and skills.
2. **Shared infrastructure.** `remote-bridge/` provides a product-neutral SSH-backed bridge for controlled O2 access. `bin/config-agent-tool` installs and updates either agent surface.
3. **Project notebook conventions.** Both agents are instructed to use a separate `notebook/` git repo for durable project memory, plans, TODOs, and substantive work records.

## Quick Start

Clone this repository, then install the configuration for exactly one agent:

```bash
bin/config-agent-tool install --agent claude
bin/config-agent-tool install --agent codex
```

The installer never installs skills automatically. List and link skills explicitly:

```bash
bin/config-agent-tool list-skills --agent claude --global
bin/config-agent-tool link-skills --agent claude --global --add init-project work-cycle artifacts

bin/config-agent-tool list-skills --agent codex --global
bin/config-agent-tool link-skills --agent codex --global --add init-project work-cycle artifacts
```

Use project-local skills from a project root:

```bash
config-agent-tool list-skills --agent claude
config-agent-tool link-skills --agent claude --add use-o2 dx-jobs run-graphld-o2

config-agent-tool list-skills --agent codex
config-agent-tool link-skills --agent codex --add use-o2 dx-jobs run-graphld-o2
```

## Layout

| Path | Purpose |
| --- | --- |
| `claude/` | Claude Code global prompt, settings, hooks, templates, and skills. |
| `codex/` | Codex global instructions, templates, and skills. |
| `bin/config-agent-tool` | Unified installer and skill-link manager. |
| `remote-bridge/` | Shared remote execution bridge for O2 and similar SSH-backed hosts. |
| `tests/` | Installer tests and Claude behavior-test framework. |

## Installation Targets

Claude install:

| Local path | Target |
| --- | --- |
| `~/.claude/CLAUDE.md` | User-owned file with `@<repo>/claude/global/CLAUDE.md` import. |
| `~/.claude/settings.json` | Symlink to `claude/global/settings.json`. |
| `~/.claude/hooks` | Symlink to `claude/hooks`. |
| `~/.claude/skills/<name>` | Selected global Claude skills. |
| `<project>/.claude/skills/<name>` | Selected project-local Claude skills. |

Codex install:

| Local path | Target |
| --- | --- |
| `~/.codex/user/AGENTS.md` | User-owned personal Codex instructions. |
| `~/.codex/AGENTS.override.md` | Generated active global instructions. |
| `~/.codex/skills/<name>` | Selected global Codex skills. |
| `<project>/.agents/skills/<name>` | Selected project-local Codex skills. |

After pulling updates, run:

```bash
config-agent-tool update --agent claude
config-agent-tool update --agent codex
```
