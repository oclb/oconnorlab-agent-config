# Claude Code Configuration for Scientific Research

This repository contains Claude Code configuration and skills customized for scientific research workflows, including remote access to the Harvard O2 HPC cluster.

## Background

**Claude Code** is an AI agent with tools to read and edit files, run bash commands, search codebases, and access the internet. It is widely used in software engineering, and it has a rich set of features that make it an attractive choice for scientific research compared with alternatives (like the agents that come with Cursor or Windsurf). 

In particular, Claude Code makes it convenient to add **skills**. A Claude skill is a specialized prompt that explains to Claude how to perform a task, potentially in great detail. Skills can be invoked explicitly using slash commands (e.g., `/help how do I use claude code?`), or Claude can automatically detect when a skill should be used. This repository includes skills that are designed to make Claude Code more useful for scientific research. 

Claude Code is traditionally used via its CLI. It can also be used inside of an IDE or via a web app.

## What's Included

- `global/` - Global configuration files
  - `CLAUDE.md` - Behavioral configuration and instructions
  - `settings.json` - Claude Code settings (hooks, permissions)
- `skills/` - Custom Claude Code skills for scientific research
  - `help/` - Gives Claude Code access to its own up-to-date documentation, as well as documentation for this repository
  - `remote-o2/` - Remote O2 cluster access via SSH
  - `use-o2/` - SLURM reference material (used by remote-o2)
  - `perform-analysis/` - Systematic analysis framework with lab notebook integration
  - `update-notebook/` - Sync notebook when work is done outside Claude Code
  - `new-data/` - Gives instructions for exploring and performing sanity checks on a new dataset
  - `new-software/` - Gives instructions for installing and learning a new library or software tool
  - Additional skills for teaching, editing scientific writing, and editing .docx, .pptx, and .pdf file formats
- `hooks/` - Shell scripts for Claude Code hooks (notifications)
- `setup.sh` - Setup script for local machines (macOS/Linux)

## Quick start

Claude Code requires configuration files to be found at specific locations. This repository uses symlinks (shortcuts) to keep the actual files in a synced location while Claude Code reads them from the expected paths.

0. Install Claude Code if you haven't already: on macOS, 
  ```bash
  brew install claude
  ```

1. Clone this repository at a chosen location:
   ```bash
   git clone https://github.com/oclb/claude.git
   ```

2. Run the local setup script:
   ```bash
   cd claude
   ./setup.sh
   ```

The local setup script will:
- Create the `~/.claude/` directory if it doesn't exist
- Backup any existing configuration files
- Create symlinks for settings and skills
- Set `Environment=local` flag for local execution

## Configuration Features

### Remote O2 Access
Use the `/remote-o2` skill to access the O2 cluster from your local machine. The first time you run `/remote-o2`, Claude will guide you through setup (SSH configuration, connection scripts), then execute commands on O2, submit SLURM jobs, and monitor progress.

**Note:** Off-campus, every interaction between Claude and O2 triggers a Duo push which you must approve. Connect to the harvard-secure wifi or ethernet to avoid this.

### Notifications
Notification hooks are triggered when Claude needs input or completes a task. Notifications use [ntfy.sh](https://ntfy.sh) for push notifications to your phone/desktop. Setup scripts automatically configure your shell with:
- `NTFY_TOPIC` environment variable (your unique notification channel)
- Helper functions: `notify <msg>`, `test_notify`, `notifyme <long-running-command>`

To receive notifications, run `test_notify`, and visit the URL on your browser.

### Performing analyses
The `/perform-analysis` skill is intended to improve Claude's ability to design, run, troubleshoot, and log analyses. 
It:
- Determines motivation and expected results
- Retrieves context from related past analyses
- Determines if it has all of the context, data, and tools that it needs
- Makes a plan
  - Presents plan to user for approval (except in `afk` mode)
  - For long-running analyses, uses pilot runs and tracks progress
  - For resource-intensive work, uses O2 cluster via `/remote-o2`
- Troubleshoots iteratively
- Names the analysis, tracks versions, and logs it in the `notebook/` directory

### Lab Notebook

The lab notebook system (`notebook/analyses/`) provides archival tracking of all analyses, separate from `CLAUDE.md` which stays focused on current context. This two-tiered system follows the "progressive disclosure" pattern: instead of giving agents every piece of context they might need, start by giving context they will definitely need, and let them find additional context themselves using bash to navigate the filesystem.

**How the notebook works:**
- Each analysis gets its own directory with a README (log), scripts, and outputs
- Claude writes to the notebook incrementally during analysis, not just at the end
- When starting a new analysis, Claude retrieves 0-3 related past analyses for context
- Git commits track each analysis version and script history

**How CLAUDE.md works:** Stores essential context on the goals of the project, its current state, available tools/software and their locations, and available datasets and their locations

**Managing the notebook:** The notebook is automatically updated when you use the `/perform-analysis` skill. It is also updated when you use the `/new-data` and `/new-software` skills. To manage it manually, use the `/update-notebook` skill, particularly the first time you use Claude within an existing project and when you've performed significant analyses outside of Claude Code. Claude will review git history, ask you about changes, read project files (like a draft manuscript), and create retrospective entries.

### Additional skills
- **new-data** - Dataset handling and validation
  - Downloads and acquires datasets from various sources
  - Validates data format and structure
  - Computes descriptive statistics
  - Identifies data quality issues
  - Provides actionable recommendations

- **new-software** - Tool learning and setup
  - Searches documentation and best practices
  - Installs and configures tools
  - Runs sanity checks
  - Provides usage examples

- **Additional skills**: Teaching mode, scientific writing revision, document handling (PDF, DOCX, PPTX)


### Behavioral Configuration

Claude reads `~/.claude/behavior.conf` at startup for runtime flags:

- **NewUser mode**: Enabled by default on first setup. Claude proactively explains features and may suggest the `/help` skill to orient new users. Ask Claude to "disable onboarding mode" once you're comfortable.
- **AFK mode**: For autonomous operation. Include `(afk)` in a message to enable, `(back)` to disable.
- **O2 cluster access**: Use `/remote-o2` for cluster computing; Claude handles connection and SLURM job submission.
- **Terminal notifications**: Desktop alerts on task completion via ntfy.sh.

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

### MCP Servers

MCP (Model Context Protocol) server configurations can be added to `settings.json` under the `mcpServers` key. See [Claude Code MCP documentation](https://docs.claude.ai/claude-code/mcp) for details.

## Resources

### Claude Code Documentation
- [Claude Code Documentation](https://docs.claude.ai/claude-code)
- [Claude Code Settings Reference](https://docs.claude.ai/claude-code/settings)
- [Claude Code Skills](https://docs.claude.ai/claude-code/skills)
- [Claude Code Hooks](https://docs.claude.ai/claude-code/hooks)

### O2 Cluster Resources
- [O2 User Guide](https://harvardmed.atlassian.net/wiki/spaces/O2/overview)
- [O2 SLURM Documentation](https://harvardmed.atlassian.net/wiki/spaces/O2/pages/1586793619/Using+Slurm+Basic)
- [O2 Research Computing Portal](https://rc.hms.harvard.edu/)

### Getting Help
- For Claude Code questions: Run the `/help` skill or check Claude Code documentation
- For configuration issues: Check this README or open an issue in the repository
- For O2 cluster issues: Contact RC help (rchelp@hms.harvard.edu)
