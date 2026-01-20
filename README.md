# Claude Code Configuration for Scientific Research

This repository contains Claude Code configurations and skills customized for scientific research and a bridge which allows it to access to the Harvard O2 cluster.

## Background

**Claude Code** is an AI agent with tools to read and edit files, run bash commands, search codebases, and access the internet. It is widely used in software engineering, and it has a rich set of features that make it an attractive choice for scientific research. Currently, this repo is designed around Claude Code, but it is likely that it could be adapted for use with other agents (like Cursor or Codex). 

In particular, Claude Code makes it convenient to add **skills**. A Claude skill is a specialized prompt that explains to Claude how to perform a task, potentially in great detail. Skills can be invoked explicitly using slash commands (e.g., `/support how do I use claude code?`), or Claude can automatically detect when a skill should be used. This repository includes skills that are designed to make Claude Code more useful for scientific research. 

Claude Code is traditionally used via its CLI. It can also be used inside of an IDE or via a web app.

## What's Included

- `global/` - Global configuration files
  - `CLAUDE.md` - Behavioral configuration and instructions
  - `settings.json` - Claude Code settings (hooks, permissions)
- `skills/` - Custom Claude Code skills for scientific research
  - `init-project/` - Initialize a project with notebook system and permissions
  - `support/` - Gives Claude Code access to its own up-to-date documentation, as well as documentation for this repository
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

## Configuration Features

### Remote O2 Access
Use the `/remote-o2` skill to access the O2 cluster from your local machine. The first time you run `/remote-o2`, Claude will guide you through setup (SSH configuration, connection scripts), then execute commands on O2, submit SLURM jobs, and monitor progress.




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

The notebook is a **separate git repository** inside your project, keeping your main repo clean while preserving full analysis history. Run `/init-project` to set this up automatically.

```
my-project/           # Main repo (your code)
├── .gitignore        # Contains: notebook/
├── CLAUDE.md
└── notebook/         # Separate repo (analysis logs)
    ├── .git/
    ├── INDEX.md
    ├── analyses/
    └── methods/
```

**Why separate repos?**
- Main repo git log stays clean (only code changes)
- Notebook can have different backup/sharing rules
- Easy to share code without sharing exploratory work

**How it works:**
- Each analysis gets its own directory with a README, scripts, and outputs
- Claude writes to the notebook incrementally during analysis
- When starting a new analysis, Claude retrieves related past analyses for context
- Commits go to the notebook repo; pushes to its own GitHub remote

**Managing the notebook:** Use `/perform-analysis` for new analyses, `/update-notebook` to sync work done outside Claude Code.

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


### AFK Mode

Include `(afk)` in any message to enable autonomous operation for that turn. Claude will proceed independently without asking clarifying questions, only pausing for irreversible actions or critical decision points.

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
- For Claude Code questions: Run the `/support` skill or check Claude Code documentation
- For configuration issues: Check this README or open an issue in the repository
- For O2 cluster issues: Contact RC help (rchelp@hms.harvard.edu)
