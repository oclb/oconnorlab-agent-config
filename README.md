# Claude Code Configuration for Scientific Research

This repository contains Claude Code configuration and skills customized for scientific research workflows, with special support for the Harvard O2 HPC cluster.

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
  - `use-o2/` - O2 cluster job submission and resource management
  - `perform-analysis/` - Gives step-by-step instructions for performing an analysis, examining results, potentially iterating, and creating a display item
  - `new-data/` - Gives instructions for exploring and performing sanity checks on a new dataset
  - `new-software/` - Gives instructions for installing and learning a new library or software tool
  - Additional skills for teaching, editing scientific writing, and editing .docx, .pptx, and .pdf file formats
- `hooks/` - Shell scripts for Claude Code hooks (notifications)
- `setup.sh` - Setup script for local machines (macOS/Linux)
- `setup-o2.sh` - Setup script specifically configured for the O2 cluster environment

## Quick start

Claude Code requires configuration files to be found at specific locations. This repository uses symlinks (sort of like shortcuts) to keep the actual files in a synced location while Claude Code reads them from the expected paths. These symlinks are created by provided setup scripts.

### For O2 Cluster

1. SSH into O2 and clone this repository at a chosen location:
   ```bash
   git clone https://github.com/oclb/claude-config.git
   ```

2. Run the O2-specific setup script:
   ```bash
   cd claude-config
   ./setup-o2.sh
   ```

3. This script edits your `.bashrc` file; make these changes apply to your current session:
   ```bash
   source ~/.bashrc
   ```

The O2 setup script will:
- Configure scratch directory for `TMPDIR` (required for O2)
- Install sandbox dependencies (`socat`) via `conda`
- Create symlinks for settings and skills
- Set `Environment=O2` flag so that Claude will automatically know that it is working in O2, triggering it to use the `use-o2` skill.

**O2 best practices:** Run Claude Code on a compute node in an interactive session; if you run it on a login node, it may use too many resources and get killed. Do not ever use the `--dangerously-skip-permissions` option. Instead, to reduce the number of times you need to approve tool usages, enable sandbox mode using the `/sandbox` command. For long-running Claude sessions, use `tmux` ([documentation](https://harvardmed.atlassian.net/wiki/spaces/O2/pages/1601700103/tmux+Keep+Linux+Sessions+Alive+so+you+can+go+back+to+the+same+terminal+window+from+anywhere+anytime)) so that the session persists if your computer sleeps or loses connection.

### For Local Machines (macOS/Linux)

1. Clone this repository at a chosen location:
   ```bash
   git clone https://github.com/oclb/claude-config.git
   ```

2. Run the local setup script:
   ```bash
   cd claude-config
   ./setup.sh
   ```

The local setup script will:
- Create the `~/.claude/` directory if it doesn't exist
- Backup any existing configuration files
- Create symlinks for settings and skills
- Set `Environment=local` flag for local execution

## Usage

## Configuration Features

### Using o2
Claude detects when it running is on the O2 cluster and uses the `/use-o2` skill to submit SLURM jobs, monitor them, and retrieve results.

### Notifications
Notification hooks are triggered when Claude needs input or completes a task. Notifications use [ntfy.sh](https://ntfy.sh) for push notifications to your phone/desktop. Setup scripts automatically configure your shell with:
- `NTFY_TOPIC` environment variable (your unique notification channel)
- Helper functions: `notify <msg>`, `test_notify`, `notifyme <long-running-command>`

To receive notifications, run `test_notify`, and visit the URL on your browser. This works either locally or on O2.

### Performing analyses
The `/perform-analysis` skill is intended to improve Claude's ability to implement, run, and troubleshoot analyses autonomously. 
This configuration includes specialized skills for research workflows:

- **perform-analysis** - Systematic 8-step analysis framework
  - Understands research questions and sets expectations
  - Verifies data and computational resources
  - Creates detailed analysis plans
  - Executes analysis with progress tracking
  - Documents all results, decisions, and output files
  - Automatically integrates with O2 cluster when appropriate

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

- **Default model**: Claude Sonnet 4.5
- **Environment detection**: Automatically adapts behavior for O2 vs local environments
- **AFK mode**: Optional autonomous operation mode for long-running tasks
- **Terminal notifications**: Desktop alerts on task completion (macOS only)

## Additional Configuration Files

### Project-Specific Settings

For project-specific settings that should be shared with your team:
- Create `.claude/settings.json` in your project repository
- These settings take precedence over user settings

### Local Project Settings

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
- For configuration issues: Check this README or open an issue in the repository
- For Claude Code questions: Run `/help` skill or check Claude Code documentation
- For O2 cluster issues: Contact RC help (rchelp@hms.harvard.edu)
