# Claude Code Configuration for Scientific Research

This repository contains Claude Code configurations, skills, and tools intended to make it more useful for scientific research.

As of early 2026, a current trend is the application of AI coding agents beyond "coding". The goal of this project is for us to integrate AI agents more deeply into our scientific work, while mitigating forseeable pitfalls of doing so.

This project has three major features:
1. **O2 Cluster bridge.** It includes an application that enables Claude to interact with O2, submit jobs, and read results. Its interactions are sandboxed to avoid accidental data deletion.
2. **Project notebook.** Claude will automatically keep a project notebook that sketches the current state of the project and permanently logs any work that you do together. This notebook is intended both to provide context to Claude and to mitigate non-reproducibility, which is a potential pitfall when you rely on agents without long-term memory.
3. **Specialized prompts.** Claude recently added support for "skills", which are specialized prompts that it retrieves automatically when they are relevant to a task. The project includes scientific skills that aim to improve the AI's performance at various tasks, like inspecting a new dataset or performing an analysis. Currently, these prompts are mostly untested; my hope is that they will improve over time.

Unlike basically everything else in this project, this README was written by Luke, not the AI.

## Background

**AI Agents** typically pair an LLM, like Claude Sonnet, with a set of tools that the LLM uses to perform actions and retrieve information. In particular, coding agents have tools to read and edit files (code), and often also to run bash commands and other code. **Claude Code** is a very popular coding agent whose abilities make it an attractive choice for scientific research. Claude Code is traditionally used via the terminal but can also be used inside of an IDE or via a web app.

In particular, Claude Code makes it convenient to add **skills**. A Claude skill is a specialized prompt that explains to Claude how to perform a task, potentially in great detail. Skills can be invoked explicitly using slash commands (e.g., `/support how do I use claude code?`), or Claude can automatically detect when a skill should be used. This repository includes skills that are designed to make Claude Code more useful for scientific research. 


## Quick start

Claude Code requires configuration files to be found at specific locations. This repository uses symlinks (shortcuts) to keep the actual files in a synced location while Claude Code reads them from the expected paths.

Using Claude Code requires an Anthropic paid plan.

0. Install Claude Code if you haven't already: on macOS, 
  ```bash
  brew install claude
  ```
  Then, you must connect Claude with your Anthropic account; enter Claude by running `claude`, then enter `/login`.

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
   ```
   /init-project
   ```
   This creates the notebook system, sets up permissions, and optionally creates GitHub remotes.

## Features

### Project notebook
A major limitation of existing AI models is their lack of long-term memory. One problem this creates is that you often need to provide  lots of project context to the AI. Even when the AI can infer what you mean from context - e.g., by reading your project's source code - you may not trust that it will do so. Another problem is that when the AI performs analyses, even with your close guidance, it can be easy to forget details of those analyses. If the AI also forgets those details, then they are simply lost.

To address these problems, the agent is instructed to maintain two types of project memory. First, it contains a state-of-the-project file (`~/my_project/CLAUDE.md`), which is loaded at the beginning of every conversation. Previously, I have done this manually using a project README file. Now, the agent is instructed to actively maintain this file for you. This file should contain information about the goals of the project, major progress so far, key datasets, and sofware usage.

Second, it maintains a notebook which acts as a stable repository for long-term memory. All substantive work done with the agent should be recorded in this notebook. The notebook is indexed, and the agent is instructed to search for relevant entries when it determines that it needs context. Although many approaches to AI memory and recall have been proposed, this one - just let it use the filesystem to search for relavent files - seems to be the simplest and the best.

The notebook lives in a GitHub repository that should be separate from your project repository and is managed entirely by the agent. Entries of this notebook include text files which describe what was done or learned, code which can be used to reproduce results, and results files. I envision that this notebook will be maintained by the agent independently, but of course, you can create entries yourself as well. An issue right now is that the AI may fail to create notebook entries without manual prompting. 

The notebook also includes a to-do list, which is totally optional; I find that it is a nice, low-effort way to start conversations ("what are our to-dos?").

### Remote O2 Access
The `bridge/` directory contains a Rust application which allows Claude (or a human) to interact with O2 from inside of a sandbox. To set this up, enter Claude Code and invoke `/remote-o2`; the first time, Claude will guide you through setup (SSH configuration, connection scripts). Subsequently, the `/use-o2` skill provides Claude with instructions to submit and monitor jobs on O2.

This bridge maintains a connetion over `ssh` with an O2 login node. It exposes an API with certain read-only commands (like `cat`), allowing Claude to read any file that you can read. It also exposes two commands that allow Claude to modify files: `git pull`, so that it can pull updates to your project repo, and `sbatch`, so that it can run jobs. Jobs run by Claude are dispatched into a sandboxed Singularity container. Containerization could make it minorly annoying to manage dependencies, but it brings major peace of mind: the agent cannot edit or delete files outside of the directories that you specify when you set up the sandbox.

### Performing analyses
The `/perform-analysis` skill is intended to improve the agent's ability to perform scientific analyses semi-autonomously. It instructs the agent to find notebook entries about related analyses; to understand motivation and expected results; to make a plan with the user; to perform troubleshooting and iteration; to produce digestible results; and to log its work in a notebook entry.

In my experience, AI agents do not have these skills without special prompting. I am unsure the extent to which special prompting will help. I would really value your feedback on this. 

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


### AFK Mode

Include `(afk)` in any message to encourage Claude to act autonomously, without asking for your help or feedback.

### Notifications

Desktop notifications alert you when Claude needs input or completes a task. Requires `terminal-notifier` on macOS (`brew install terminal-notifier`).

## Additional Configuration Files

### User-specific and project-specific instructions

At the beginning of every conversation, Claude Code reads the `CLAUDE.md` file from both `~/.claude` and from the project's root directory. You can add project-specific instructions to the latter; however, the former is managed by this repository, so if you customize it, your changes could cause a merge conflict. If you have custom instructions that you want to apply to all of your projects on a machine, create a file `global/CLAUDE.user.md` (within this repo). There is an example file `global/CLAUDE.user.md.example` that you can use as a template. Claude Code will automatically read this file.

### Project-Specific Settings

For project-specific settings that should be shared with your team or across machines:
- Create `.claude/settings.json` in your project repository
- These settings take precedence over user settings

For personal project-specific settings (not shared with team):
- Create `.claude/settings.local.json` in your project
- Add `settings.local.json` to your project's `.gitignore`

## Resources

- [Claude Code Documentation](https://docs.claude.ai/claude-code)
- [O2 User Guide](https://harvardmed.atlassian.net/wiki/spaces/O2/overview)
- [O2 SLURM Documentation](https://harvardmed.atlassian.net/wiki/spaces/O2/pages/1586793619/Using+Slurm+Basic)
- [O2 Research Computing Portal](https://rc.hms.harvard.edu/)
