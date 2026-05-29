---
name: support
description: Use when the user asks about their Claude Code configuration, or encounters issues related to missing skills, the CLAUDE.md file, or the O2 bridge.
---

# Support

This skill is provided by the lab-agent-config repo, which adds a global CLAUDE.md file to the user profile, provides `config-agent-tool`, symlinks selected skills, and includes guidance for agents to access the O2 compute cluster.

Diagnose whether the issue or question is related to this repo or to Claude Code itself. For Claude Code support specifically, use the Anthropic documentation when product behavior may have changed.

Locate this repo as follows:

```bash
CONFIG_AGENT_TOOL="${CLAUDE_HOME:-$HOME/.claude}/bin/config-agent-tool"
CONFIG_REPO="$("$CONFIG_AGENT_TOOL" repo-dir)"
```

If the user is encountering an issue, start by checking if their installation is current; if not, run `$CONFIG_AGENT_TOOL update --agent claude` and determine if this fixes the issue.

Then, the following are logical starting points for your investigation:

- `$CONFIG_REPO/README.md`
- `$CONFIG_REPO/claude/global/CLAUDE.md`
- `$CONFIG_REPO/claude/skills/init-project/SKILL.md`
- `$CONFIG_REPO/bin/config-agent-tool`

## User feedback

The user may be using this skill because they are having an issue with the repository. Discern whether the issue that the user had with the repository is something that other users are also likely to encounter. If so, suggest to the user that you create a GitHub issue or make a PR. State that the maintainer (Luke) would appreciate it!
