# Claude Code Configuration for O'Connor Lab

This repository contains Claude Code configuration customized for scientific research workflows in the O'Connor Lab, with special support for the Harvard O2 HPC cluster.

## Purpose

This configuration optimizes Claude Code for:
- **Scientific research workflows** - Data analysis, experiment design, statistical computing
- **O2 cluster integration** - Seamless job submission and resource management on Harvard's O2 HPC
- **Reproducible research** - Systematic approaches to data validation and analysis

## What's Included

- `settings.json` - User-level Claude Code settings (model preferences, hooks, skills, etc.)
- `setup.sh` - Setup script for local machines (macOS/Linux)
- `setup-o2.sh` - Setup script specifically configured for the O2 cluster environment
- `skills/` - Custom Claude Code skills for scientific research
  - `perform-analysis/` - Systematic 8-step framework for data analyses and experiments
  - `new-data/` - Dataset acquisition, validation, and exploration
  - `new-software/` - Tool learning and setup assistance
  - `use-o2/` - O2 cluster job submission and resource management
  - `help/` - Documentation and capability reference
  - Additional skills for teaching, writing, and document handling
- `CLAUDE.md` - Global behavioral configuration (AFK mode, environment detection)

## How It Works

Claude Code requires configuration files at specific locations:
- User settings: `~/.claude/settings.json`
- Skills directory: `~/.claude/skills/`
- Behavior flags: `~/.claude/behavior.conf`

This repository uses symlinks to keep the actual files in a synced location (Dropbox/GitHub) while Claude Code reads them from the expected paths. This allows:
- **Version control** - Track configuration changes over time
- **Sharing** - Lab members can use the same optimized configuration
- **Multi-machine sync** - Consistent setup across local machines and O2

## Setup for O'Connor Lab Members

### For O2 Cluster (Recommended for compute-intensive work)

1. SSH into O2 and clone this repository:
   ```bash
   ssh USERNAME@o2.hms.harvard.edu
   cd ~
   git clone https://github.com/USERNAME/claude-config.git
   ```

2. Run the O2-specific setup script:
   ```bash
   cd ~/claude-config
   ./setup-o2.sh
   ```

3. Start a new terminal session or run:
   ```bash
   source ~/.bashrc
   ```

The O2 setup script will:
- Configure scratch directory for TMPDIR (required for O2)
- Install sandbox dependencies (socat) via conda
- Create symlinks for settings and skills
- Set `Environment=O2` flag for automatic O2 skill integration

### For Local Machines (macOS/Linux)

1. Clone this repository:
   ```bash
   cd ~/Dropbox/GitHub/  # or your preferred location
   git clone https://github.com/USERNAME/claude-config.git
   ```

2. Run the local setup script:
   ```bash
   cd ~/Dropbox/GitHub/claude-config
   ./setup.sh
   ```

The local setup script will:
- Create the `~/.claude/` directory if it doesn't exist
- Backup any existing configuration files
- Create symlinks for settings and skills
- Set `Environment=local` flag for local execution

### Verification

Verify the symlink was created correctly:

```bash
ls -la ~/.claude/settings.json
```

You should see output like:
```
lrwxr-xr-x  1 user  staff  ... ~/.claude/settings.json -> /Users/user/Dropbox/GitHub/claude-config/settings.json
```

## Usage

### Updating Settings

Simply edit the `settings.json` file in this repository. Changes will be immediately available to Claude Code since it's symlinked.

You can edit either:
- The file in this repo: `~/Dropbox/GitHub/claude-config/settings.json`
- The symlinked file: `~/.claude/settings.json` (they're the same)

### Syncing Changes

After making changes, commit and push:

```bash
cd ~/Dropbox/GitHub/claude-config
git add settings.json
git commit -m "Update settings"
git push
```

On your other computer, pull the changes:

```bash
cd ~/Dropbox/GitHub/claude-config
git pull
```

Changes will be immediately available since the file is symlinked.

## For New Lab Members

To adopt this configuration:

1. **Choose your primary environment:**
   - Working primarily on O2? Use the O2 setup (recommended for data analysis)
   - Working primarily on your laptop? Use the local setup

2. **Clone and run setup:**
   - Follow the appropriate setup instructions above
   - All sensitive information has been removed from the shared configuration

3. **Customize as needed:**
   - The configuration works out of the box, but you can personalize it
   - Add your own skills to the `skills/` directory
   - Modify `settings.json` for personal preferences
   - Settings are version controlled, so you can always revert changes

### Note on Notifications

The `settings.json` includes terminal notification hooks:
- **macOS**: Requires `terminal-notifier` (install via: `brew install terminal-notifier`)
- **Linux/O2**: Notifications are disabled by default (no terminal-notifier available)

## Configuration Features

### Scientific Research Skills

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

- **use-o2** - O2 HPC cluster integration
  - Submits SLURM jobs with appropriate resources
  - Monitors job status and retrieves results
  - Handles compute-intensive tasks automatically on O2
  - Integrated with analysis workflows

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

## Troubleshooting

### Symlink Issues

If the symlink breaks or points to the wrong location:

1. Remove the broken symlink:
   ```bash
   rm ~/.claude/settings.json
   ```

2. Re-run the setup script:
   ```bash
   cd ~/Dropbox/GitHub/claude-config
   ./setup.sh
   ```

### Settings Not Loading

If Claude Code isn't picking up your settings:

1. Verify the symlink exists and points to the correct location:
   ```bash
   ls -la ~/.claude/settings.json
   ```

2. Check file permissions:
   ```bash
   ls -l ~/Dropbox/GitHub/claude-config/settings.json
   ```

3. Restart your Claude Code session

### Merge Conflicts

If you make changes on both computers before syncing:

1. Pull the latest changes:
   ```bash
   git pull
   ```

2. If there's a conflict, Git will mark it in the file
3. Edit `settings.json` to resolve the conflict
4. Commit the resolved version:
   ```bash
   git add settings.json
   git commit -m "Resolve settings conflict"
   git push
   ```

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
