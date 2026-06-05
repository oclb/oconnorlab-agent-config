---
name: set-me-up
description: Repo-local onboarding for this lab-agent-config repository. Auto-trigger inside this repository when the user asks to set up, initialize, install, onboard, configure, or get started with this repository or its Claude Code configuration.
---

# Set Me Up

Guide the user through first-run setup for this repository. This is a repo-local onboarding skill, not a symlinked content skill for downstream projects.

Use the exact onboarding script in `references/onboarding-script.md`. The script is mandatory: do not summarize it, reorder it, skip consent checkpoints, or install recommended skills by default.

Core rules:

- Auto-trigger this skill when the user asks to set up, initialize, install, onboard, configure, or get started with this repository or its Claude Code configuration. The user does not need to invoke `/set-me-up`.
- Use this skill only when the active working directory is this repository or a subdirectory of it. Do not use this skill for downstream project setup; use `init-project` for downstream projects.
- Onboard before installing. Explain what the repo contains before creating global files or symlinks.
- Ask before every installation step. Do not run `install --agent claude` until the user explicitly agrees to base setup. Do not link any optional skill until the user explicitly chooses it.
- Treat recommendations as explanations, not consent.
- Tell the user that skipped skills can still be installed project-locally later.
- If the user asks for a faster path, still state exactly what will be installed and ask for one explicit confirmation before installing anything.

## Workflow

1. Read `README.md` and `references/onboarding-script.md`.
2. Confirm the current directory is this config repo.

```bash
pwd
test -x bin/config-agent-tool && test -d claude/skills && test -f claude/global/CLAUDE.md && echo config-repo
```

3. Follow the script. The first user-facing message must welcome the user, summarize the three README features, and explain that setup is a guided choice process.
4. Only after the user consents, run `bin/config-agent-tool install --agent claude`.
5. For subsequent setup commands, use the installed tool path: `${CLAUDE_HOME:-$HOME/.claude}/bin/config-agent-tool`.
6. Run `${CLAUDE_HOME:-$HOME/.claude}/bin/config-agent-tool list-skills --agent claude --global` and continue the scripted skill walkthrough.
7. After collecting explicit choices, run:

```bash
${CLAUDE_HOME:-$HOME/.claude}/bin/config-agent-tool link-skills --agent claude --global --add <chosen-skill-names>
```

Skip this command if the user chooses no global skills.
8. Verify the files and symlinks created by setup:
   - `~/.claude/CLAUDE.md`
   - `~/.claude/settings.json`
   - `~/.claude/hooks`
   - `~/.claude/bin/config-agent-tool`
   - any chosen global skills under `~/.claude/skills/`
9. Finish with the scripted closing: restart Claude Code, rerun setup after edits if needed, and run `init-project` inside other repositories.

Do not use this skill for project onboarding outside this repository. Do not touch remotes, publishing settings, or GitHub visibility unless explicitly asked.
