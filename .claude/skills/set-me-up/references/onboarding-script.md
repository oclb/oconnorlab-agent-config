# Set Me Up Onboarding Script

Follow this script when onboarding the user.

## Overall Guidance

- User-facing text can be modified if circumstances or user input deviates from the happy path; otherwise, it should be quoted exactly.
- Answer any user questions by consulting README.md and other documents, particularly the skill files themselves; then, resume the script.
- Interpret answers like "yea" or "y" as "yes", "nty" for "no", etc. Don't say "I am interpreting 'yea' as 'yes'" or similar.
- Avoid adding noise to the conversation or thinking out loud. Don't preface like "I am going to read this script" or add filler between steps like "/systematize is selected. Next is..."

## Cross-Agent Setup Or Update

Use this section when the user says `set me up for both Claude and Codex`, whether the active agent is Claude or Codex. It takes precedence over the product-specific first-time and migration paths below.

Say:

```text
I can set up or update both Claude and Codex from this checkout. After you approve, I will first pull the latest lab-agent-config with a fast-forward-only update. I will then detect whether each configuration is new, current, or legacy; install or repair both managed surfaces; preserve user-owned Claude settings, Codex instructions, and personal hooks; retain existing skill choices; and verify both installations. The repository does not configure Claude permissions or edit Codex's config.toml.

Proceed with the base setup or update for both Claude and Codex?
```

Only after the user agrees, run:

```bash
bin/config-agent-tool setup --agent both
```

This command pulls and then restarts itself from the updated checkout before changing either agent configuration, so the newly downloaded migration logic is used. If the pull fails because of local changes, divergence, authentication, or network access, stop without forcing or resetting anything. If first-time Codex safety checks report an unmanaged `~/.codex/AGENTS.override.md`, `~/.codex/hooks.json`, or `~/.codex/hooks/` directory, explain the existing-file conflict and use the same consent-based resolution as the Codex onboarding workflow; never overwrite it silently.

If both sides were reported as existing installations, retain their current skill choices and continue to cross-agent verification. If either side was first-time, list the currently available global skills for both agents, then offer the ordinary skill walkthrough once:

```bash
${CLAUDE_HOME:-$HOME/.claude}/bin/config-agent-tool list-skills --agent claude --global
${CODEX_HOME:-$HOME/.codex}/bin/config-agent-tool list-skills --agent codex --global
```

Describe both invocation forms (`/skill-name` in Claude and `$skill-name` in Codex), ask once per conceptual skill, and state that each accepted choice will be linked for both agents. Honor different per-agent choices if requested. Include `notebook-entry` for each agent only if the user chooses the notebook system. After collecting all choices, run the applicable commands:

```bash
${CLAUDE_HOME:-$HOME/.claude}/bin/config-agent-tool link-skills --agent claude --global --add <chosen-claude-skill-names>
${CODEX_HOME:-$HOME/.codex}/bin/config-agent-tool link-skills --agent codex --global --add <chosen-codex-skill-names>
```

Skip either command if no skills were chosen for that agent. Verify both installations:

```bash
ls -l ~/.claude/CLAUDE.md ~/.claude/settings.json ~/.claude/skills/lab-config ~/.claude/hooks ~/.claude/bin/config-agent-tool
test -f ~/.claude/settings.json && test ! -L ~/.claude/settings.json
claude plugin list
ls -l ~/.codex/user/AGENTS.md ~/.codex/AGENTS.override.md ~/.codex/bin/config-agent-tool ~/.codex/hooks.json ~/.codex/hooks/update-config.sh
```

Confirm that `claude plugin list` shows `lab-config@skills-dir`, Codex hooks render successfully, personal hooks/settings remain intact, and chosen skill links resolve into the matching agent tree. Finish by telling the user to restart both Claude and Codex; mention `/hooks` if Codex asks the user to review its startup hook.

## Migration From A Pre-Plugin Install

Use this section instead of the welcome walkthrough when `~/.claude/settings.json` is a symlink into this repo, or when `~/.claude/bin/config-agent-tool` exists but the repo-managed `~/.claude/skills/lab-config` plugin link is absent. The installed tool remains after a successful migration and is not itself evidence of a legacy install. Existing installs do not migrate on their own: the pre-plugin startup hook could not pull this repo, so the machine stays on the old layout until an update runs.

Say:

```text
This machine has an existing install of this repo's Claude Code configuration, which predates the current plugin-based layout. Migrating will:
1. Convert `~/.claude/settings.json` from a symlink into a regular file that you own. This repo does not configure Claude permissions; permission grants and future interactive changes stay in your user-owned file instead of the shared repo.
2. Link the lab-config plugin into `~/.claude/skills/`, which takes over the managed hooks (startup auto-update, memory reminder, notifications). The old hook entries are removed from your settings file so nothing fires twice.
3. Repair the startup auto-update hook, which was silently broken in the old layout.

Your installed skills and `~/.claude/CLAUDE.md` import are unaffected. Shall I migrate?
```

If the user agrees, run:

```bash
git pull --ff-only
bin/config-agent-tool update --agent claude
```

Run the pull first and separately: `update` alone would execute the already-loaded pre-migration repair logic even after pulling new code. If the pull fails because of local changes or a diverged branch, stop and ask the user how to proceed rather than forcing it.

Then verify as in the Verify And Close section, and tell the user to restart Claude Code so the plugin's hooks load. Migration is one-time: after it, the repaired startup hook keeps the repo and managed surfaces current automatically.

## 1. Welcome Before Installing

Say:

```text
This repository configures Claude Code for scientific software development and other scientific research tasks. It has three major features:
1. Support for the O2 Cluster bridge so Claude Code can interact with O2 and submit jobs within a sandbox.
2. A project notebook, so Claude Code can keep durable project memory, record substantive work, and reduce irreproducibility.
3. Specialized skills that Claude Code can retrieve for software and scientific work.

For details, see [README.md](README.md) and [ADVICE.md](ADVICE.md).

Each component is take-it-or-leave-it. We will walk through a setup process together so that you can install the components that you want, and so that you understand what is being installed. At any time you may ask me questions.

How this works under the hood: when you open Claude Code, the app locates CLAUDE.md files, plugins, and skills located at `~/.claude` and within your project directory. This setup will nondestructively add an import to `~/.claude/CLAUDE.md`, link this repo's lab-config plugin (which provides the managed hooks, including a startup auto-update hook) into `~/.claude/skills/`, and symlink any skills you choose. Your `~/.claude/settings.json` stays a user-owned file: setup seeds non-permission preferences from this repo's template only if it is missing, and your own interactive changes are never written back to this repo. This repo does not configure Claude permission allowlists, permission modes, prompt suppression, or sandbox write policy; permissions and personal hooks belong in your own `~/.claude/settings.json`.

First question: do you wish to use the lab notebook system? This is recommended for all users; see [README.md: Project notebook](README.md#project-notebook) for how this works and its rationale. If so, I will install this repo's global [CLAUDE.md](claude/global/CLAUDE.md) import and install the /notebook-entry skill globally.
```

## 2. Base Setup

If user agrees, run:

```bash
bin/config-agent-tool install --agent claude
```

Next, say:

```text
Next I will show you the skills this repo provides. These skills can be installed globally now, or installed locally within specific projects when you set up those projects individually.

Important: several of the workflow skills in this repo are manual-invocation skills. Installing them does not make Claude Code use them automatically; you trigger them explicitly by mentioning them, for example `/init-project` or `/work-cycle`.
```

Then run:

```bash
${CLAUDE_HOME:-$HOME/.claude}/bin/config-agent-tool list-skills --agent claude --global
```

## 3. Skill Walkthrough

Ask each choice separately. Do not link any skill until all choices are collected.

### Notebook Auxiliaries

If the user indicated that they want to use the notebook system, say:

```text
Notebook auxiliary skills: /documentation, /defer, /remind-resume

These skills work together. /documentation updates developer-facing and agent-facing docs. /defer captures later work as notebook-backed TODOs. /remind-resume summarizes recent project state when you return after a break.

All three are manual-invocation skills. Installing them does not make Claude Code use them automatically; you trigger them explicitly with `/documentation`, `/defer`, or `/remind-resume`.

Recommendation: install these globally, since you indicated that you wish to use the lab notebook system.

Install the notebook system auxiliary skills globally?
```

If the user wants only some notebook skills, accept that and record exactly which ones. Do not include /notebook-entry in this list; you should always install this if the user indicates that they want the notebook system.

### Software Workflow

Say:

```text
/work-cycle

This is the planning-centric workflow for substantial software, analysis, artifact, documentation, and configuration work. It pushes Claude Code toward explicit alignment with the user before changing modules, APIs, edges, or core logic.

Example invocation: `/work-cycle todo 7 grillme worktree`

Its keywords can be combined:
- `/work-cycle` invokes the planning-centric workflow for substantial work.
- `todo 7` tells Claude Code to use notebook TODO item 7 as the task.
- `worktree` instructs Claude Code to implement changes in a Git worktree.
- `afk` instructs Claude Code to ask any questions immediately and then implement autonomously.
- `methods-first` instructs Claude Code to use a Methods document as the plan or specification.
- `grillme` is Matt Pocock-inspired and instructs Claude Code to stress-test the plan through structured questioning before implementation.

This is a manual-invocation skill. Installing it does not make Claude Code use it automatically; you must trigger it explicitly with `/work-cycle`.

Recommendation: install globally if you want this repo's software-development workflow across projects. Skip it if you already have a different coding workflow you prefer.

Install /work-cycle globally?
```

### Workflow Customization

Say:

```text
/systematize

This is the workflow-customization skill. It helps Claude Code modify its own instructions or diagnose problems with this configuration. It includes four subskills which you can trigger by mentioning their keywords:
- `/skill-creator`, to create or update a Claude Code skill
- `/agents-md`, to customize CLAUDE.md at user or project scope
- `/postmortem`, for when the agent did not behave as desired
- `/support`, for technical support related to this repository

Recommendation: install globally if you want Claude Code to help maintain and improve this configuration over time.

Install /systematize globally?
```

### Scientific Artifacts

Say:

```text
/artifacts

This is a subskill router, with six subskills which you can trigger by keyword:
- `/docx`, for Word documents
- `/pptx`, for PowerPoint presentations
- `/pdf`, for PDF files
- `/tikz-flowchart`, for LaTeX/TikZ flowcharts
- `/polished-manuscript-figure`, for polished manuscript figures
- `/finalize-manuscript`, for a long list of pre-submission manuscript checks

Recommendation: install globally if you use Claude Code for papers, figures, slides, or other public-facing scientific artifacts.

Install /artifacts globally?
```

### Project Setup And Local Skills

Say:

```text
/init-project and project-local skills

/init-project is the normal next step after global setup. It initializes an individual project for Claude Code by creating project instructions, setting up the notebook if you opted into the notebook system, and optionally adding project-scope skills.

These remaining skills are intended for project-local installation, not global installation:
1. /use-o2: operate the O2 bridge after first-time setup.
2. /dx-jobs: check, monitor, diagnose, and resubmit DNAnexus jobs.
3. /run-graphld-o2: install and run GraphLD graphREML on O2.

You can install these later inside a project with /init-project or with config-agent-tool link-skills --agent claude --add <skill>. I will not install them globally unless you explicitly ask me to override the recommendation.

This is a manual-invocation skill. Installing it does not make Claude Code run it automatically; you must trigger it explicitly with `/init-project`.

Recommendation: install /init-project globally, because users need it in other repositories to start project setup. Skipped global skills can still be installed project-locally later.

Install /init-project globally?
```

## 4. Confirm And Link

Run:

```bash
${CLAUDE_HOME:-$HOME/.claude}/bin/config-agent-tool link-skills --agent claude --global --add <chosen-skill-names>
```

Skip the command if no skills were chosen.

## 5. Verify And Close

Verify:

```bash
ls -l ~/.claude/CLAUDE.md ~/.claude/settings.json ~/.claude/skills/lab-config ~/.claude/hooks ~/.claude/bin/config-agent-tool
ls -l ~/.claude/skills/<chosen-skill>
claude plugin list
command -v remote-bridge || true
```

Confirm that `claude plugin list` shows `lab-config@skills-dir`; settings.json must be a regular file, not a symlink.

If `remote-bridge` is missing, look for a nearby `claude-config` checkout. Tell the user O2 bridge setup requires that sibling repo if neither is available.

Finish with (assuming /init-project was installed):

```text
Setup is complete. Restart Claude Code so the lab-config plugin's hooks load; they activate in the next session, not this one. If you wish to modify your choices or uninstall symlinks, run `claude` inside of this directory. Personal settings and hooks belong in your own `~/.claude/settings.json`. To set up a specific project, navigate to that project, run `claude`, and run `/init-project`.
```
