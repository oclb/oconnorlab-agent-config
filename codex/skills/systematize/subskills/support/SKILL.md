---
name: support
description: Use when the user asks about their Codex configuration, or encounters issues related to missing skills, the AGENTS.md file, or the O2 bridge.
---

# Support

This skill is provided by the lab-agent-config repo, which adds a global AGENTS.override.md file to the user profile, provides `config-agent-tool`, symlinks selected skills, and includes guidance for agents to access the O2 compute cluster.

Diagnose whether the issue or question is related to this repo or to Codex itself. For Codex support specifically, use the Codex-provided `openai-docs` skill.

Locate this repo as follows:

```bash
CONFIG_AGENT_TOOL="${CODEX_HOME:-$HOME/.codex}/bin/config-agent-tool"
CONFIG_REPO="$("$CONFIG_AGENT_TOOL" repo-dir)"
```

If the user is encountering an issue, start by checking if their Codex installation is current; if not, run `$CONFIG_AGENT_TOOL update --agent codex` and determine if this fixes the issue.

Then, the following are logical starting points for your investigation:

- `$CONFIG_REPO/README.md`
- `$CONFIG_REPO/AGENTS.md`
- `$CONFIG_REPO/codex/.agents/skills/set-me-up/SKILL.md`
- `$CONFIG_REPO/bin/config-agent-tool`

## User feedback

The user may be using this skill because they are having an issue with the repository. Discern whether the issue that the user had with the repository is something that other users are also likely to encounter. If so, suggest to the user that you create a GitHub issue or make a PR. State that the maintainer (Luke) would appreciate it!
