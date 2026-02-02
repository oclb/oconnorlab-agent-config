# Claude Code Configuration for Scientific Research

A current trend in artificial intelligence is the application of AI coding agents beyond "coding". The goal of this project is for us to integrate AI agents more deeply into our scientific work, while mitigating forseeable pitfalls of doing so.

The project has three major features:
1. **O2 Cluster bridge.** It includes an application that enables Claude to interact with O2, submit jobs, and read results. Its interactions are sandboxed to avoid accidental data deletion.
2. **Project notebook.** It configures Claude to keep a project notebook that tracks the current state of the project and logs any work that you do with it. This notebook is intended both to provide context to Claude and to mitigate non-reproducibility, which is a potential pitfall when you rely on agents without long-term memory.
3. **Specialized prompts.** Claude recently added support for "skills", which are specialized prompts that it retrieves automatically when they are relevant to a task. This project includes scientific skills that aim to improve the AI's performance at various tasks, like inspecting a new dataset or performing an analysis.

## Background

**AI Agents** typically pair an LLM, like Claude Opus, with a set of tools that the LLM uses to perform actions and retrieve information. In particular, coding agents have tools to read and edit files, and to run the code that they write. **Claude Code** is a very popular coding agent whose abilities make it an attractive choice for scientific research. Claude Code is traditionally used via the terminal but can also be used inside of an IDE or via a web app.

Claude Code supports custom **skills**. A Claude skill is a specialized prompt that explains to Claude how to perform a task, potentially in great detail. Skills can be invoked explicitly using slash commands (`/name-of-skill`), or Claude can automatically detect when a skill should be used. This repository includes skills that are designed to make Claude Code more useful for scientific research. 

Using Claude Code requires an Anthropic paid plan.

## Quick start

Claude Code requires configuration files to be found at specific locations. This repository uses symlinks (shortcuts) to keep the actual files in a synced location while Claude Code reads them from the expected paths.

0. Install Claude Code if you haven't already: on macOS, 
  ```bash
  brew install claude-code
  ```
  Then, you must connect Claude with your paid Anthropic account; enter Claude by running `claude`, then enter the slash command `/login`.

1. Clone this repository at a chosen location:
   ```bash
   git clone https://github.com/oclb/claude.git
   ```

2. Run the local setup script:
   ```bash
   cd claude
   ./setup.sh
   ```
   This will create symlinks, configure notifications, and set up permissions.

3. Go to your project directory and start Claude Code:
   ```bash
   cd ~/my-project
   claude
   ```

4. Initialize your project (first time only):
   ```bash
   /init-project
   ```
   This creates the notebook system, sets up permissions, and optionally creates GitHub remotes.

## Features

### Project notebook
A major limitation of existing AI models is their lack of long-term memory. One problem this creates is that you often need to guess what context the AI will need for a task and include it manually. Even when the AI can gather context itself - e.g., by reading your project's source code - you may not trust that it will do so reliably. Another problem is irreproducibility: when the AI performs analyses, even with your close guidance, it can be easy to forget exactly what was done and why. If the AI also forgets those details, then they are simply lost.

To address these problems, the agent is instructed to maintain two types of project memory. First, it contains a state-of-the-project file (`~/my_project/CLAUDE.md`), which is loaded at the beginning of every conversation. Previously, I have done this manually by tagging a README file. Now, the agent is instructed to actively maintain this file for you. It should contain information about the goals of the project, major progress so far, key datasets, and sofware usage.

Second, it maintains a notebook which acts as a stable repository for long-term memory. All substantive work done with the agent should be recorded in this notebook. The notebook is indexed, and the agent is instructed to search for relevant entries when it determines that it needs context. Although many approaches to AI memory and recall have been proposed, this one - just let it use the filesystem to search for relavent files - seems to be the simplest and the best.

The notebook lives in a GitHub repository that should be separate from your project repository and is managed entirely by the agent. Entries of this notebook include text files which describe what was done or learned, code which can be used to reproduce results, and results files. I envision that this notebook will be maintained by the agent independently, but of course, you can create entries yourself as well. An issue right now is that the AI may fail to create notebook entries without manual prompting. 

The notebook also includes a to-do list, which is totally optional; I find that it is a nice, low-effort way to start conversations ("what are our to-dos?").

### Remote O2 Access
The `bridge/` directory contains a Rust application which allows Claude (or a human) to interact with O2 from inside of a sandbox. To set this up, enter Claude Code and invoke `/remote-o2`; the first time, Claude will guide you through setup (SSH configuration, connection scripts). Subsequently, the `/use-o2` skill provides Claude with instructions to submit and monitor jobs on O2.

This bridge maintains a connetion over `ssh` with an O2 login node. It exposes an API with certain read-only commands (like `cat`), allowing Claude to read any file that you can read. It also exposes two commands that allow Claude to modify files: `git pull`, so that it can pull updates to your project repo, and `sbatch`, so that it can run jobs. Jobs run by Claude are dispatched into a sandboxed Singularity container. Containerization could make it minorly annoying to manage dependencies, but it brings major peace of mind: the agent cannot edit or delete files outside of the directories that you specify when you set up the sandbox.

### Performing analyses
The `/perform-analysis` skill is intended to improve the agent's ability to perform scientific analyses semi-autonomously. It instructs the agent to find notebook entries about related analyses; to understand motivation and expected results; to make a plan with the user; to perform troubleshooting and iteration; to produce digestible results; and to log its work in a notebook entry.

In my experience, AI agents do not have these skills without special prompting. I am unsure the extent to which special prompting will help. I would really value feedback on this. 

### Getting help
Using the `/support` skill gives Claude Code access to its own up-to-date documentation, as well as the contents of this repository.

### Additional skills
To an extent, these are aspirational placeholders for abilities I would like the AI to have. Please try them and provide feedback. 
- **/new-data** - initial investigation of a new dataset
  - Downloads and acquires datasets from various sources
  - Determines data format and describes contents
  - Reports basic statistics, like the number of genes
  - Hopefully identifies data quality issues

- **new-software** - onboarding existing tools and libraries
  - Searches documentation and best practices
  - Installs and configures tools
  - Runs sanity checks
  - Provides usage examples
  
- **revise-scientific-writing** - I do not yet recommend this skill.

- **teaching-mode** - I do not yet recommend this skill.

- **File format support**: There are skills copied from Anthropic so that the AI can manipulate .docx, .pptx, and .pdf files. For example, you could ask it to create a powerpoint with figures from the last week's notebook entries - I have not actually tried this!

- **skill-creator**: An Anthropic-provided skill to be used when creating other skills.


### AFK Mode

Include `(afk)` in any message to encourage Claude to act autonomously, without asking for your help or feedback.

### Notifications

Desktop notifications alert you when Claude needs input or completes a task. Requires `terminal-notifier` on macOS (`brew install terminal-notifier`).

## Configuration Files

### Setup Architecture

Running `setup.sh` configures Claude Code with a mix of user-owned and repo-owned files:

| File | Type | Purpose |
|------|------|---------|
| `~/.claude/CLAUDE.md` | User-owned | Your personal instructions (with `@import` to repo) |
| `~/.claude/settings.json` | Symlink → repo | Shared settings, auto-updates when repo changes |
| `~/.claude/settings.local.json` | User-owned | Your personal settings + O2 permissions |
| `~/.claude/skills/` | Symlink → repo | Shared skills |
| `~/.claude/hooks/` | Symlink → repo | Notification hooks |

### User-Specific Instructions

Your `~/.claude/CLAUDE.md` is user-owned (not a symlink). It contains an `@import` line that pulls in the shared configuration from this repo. You can add personal instructions before or after the import line:

```markdown
# My Personal Instructions
- Always use verbose logging
- Prefer Python over R

# Import shared configuration from claude-config repo
@/path/to/claude-config/global/CLAUDE.md
```

### User-Specific Settings

Your `~/.claude/settings.local.json` is user-owned and takes precedence over the symlinked `settings.json`. Use it for personal preferences and machine-specific settings (like the O2 permissions added by `setup.sh`).

### Project-Specific Configuration

For project-specific instructions:
- Edit the `CLAUDE.md` in your project's root directory

For project-specific settings shared with your team:
- Create `.claude/settings.json` in your project repository

For personal project-specific settings (not shared):
- Create `.claude/settings.local.json` in your project
- Add `settings.local.json` to your project's `.gitignore`

## Contributing

Please contribute back to this project to help improve it for others. There is a `feedback/` directory in which Claude is instructed to log user feedback; please use this mechanism and feel free to push new entries at any time. 

Also feel free to edit skills or create new ones. If you do this, please create a pull request so that others can benefit from your work as well.

## Resources

- [Claude Code Documentation](https://docs.claude.ai/claude-code)
- [O2 User Guide](https://harvardmed.atlassian.net/wiki/spaces/O2/overview)
- [O2 SLURM Documentation](https://harvardmed.atlassian.net/wiki/spaces/O2/pages/1586793619/Using+Slurm+Basic)
- [O2 Research Computing Portal](https://rc.hms.harvard.edu/)
