# Set Me Up Onboarding Script

Follow this script when onboarding the user.

## Overall Guidance

- User-facing text can be modified if circumstances or user input deviates from the happy path; otherwise, it should be quoted exactly.
- Answer any user questions by consulting README.md and other documents, particularly the skill files themselves; then, resume the script.
- Interpret answers like "yea" or "y" as "yes", "nty" for "no", etc. Don't say "I am interpreting 'yea' as 'yes'" or similar.
- Avoid adding noise to the conversation or thinking out loud. Don't preface like "I am going to read this script" or add filler between steps like "/systematize is selected. Next is..."

## 1. Welcome Before Installing

Say:

```text
This repository configures Claude Code for scientific software development and other scientific research tasks. It has three major features:
1. Support for the O2 Cluster bridge so Claude Code can interact with O2 and submit jobs within a sandbox.
2. A project notebook, so Claude Code can keep durable project memory, record substantive work, and reduce irreproducibility.
3. Specialized skills that Claude Code can retrieve for software and scientific work.

For details, see [README.md](README.md) and [ADVICE.md](ADVICE.md).

Each component is take-it-or-leave-it. We will walk through a setup process together so that you can install the components that you want, and so that you understand what is being installed. At any time you may ask me questions.

How this works under the hood: when you open Claude Code, the app locates CLAUDE.md files and skills located at `~/.claude` and within your project directory. This setup will nondestructively add an import to `~/.claude/CLAUDE.md` and symlink settings, hooks, and skills to `~/.claude` so that they become globally available on your machine.

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
ls -l ~/.claude/CLAUDE.md ~/.claude/settings.json ~/.claude/hooks ~/.claude/bin/config-agent-tool
ls -l ~/.claude/skills/<chosen-skill>
command -v remote-bridge || true
```

If `remote-bridge` is missing, look for a nearby `claude-config` checkout. Tell the user O2 bridge setup requires that sibling repo if neither is available.

Finish with (assuming /init-project was installed):

```text
Setup is complete. If you wish to modify your choices or uninstall symlinks, run `claude` inside of this directory. To set up a specific project, navigate to that project, run `claude`, and run `/init-project`.
```
