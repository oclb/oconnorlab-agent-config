# Claude Code Configuration

This repository contains my Claude Code settings and configuration files, synced across multiple computers.

## What's Included

- `settings.json` - User-level Claude Code settings (model preferences, hooks, etc.)
- `setup.sh` - Automated setup script to create symlinks on new machines

## How It Works

Claude Code requires settings files to be at specific locations:
- User settings: `~/.claude/settings.json`
- Project settings: `.claude/settings.json` (in each project)

This repository uses symlinks to keep the actual files in a synced location (Dropbox/GitHub) while Claude Code reads them from the expected paths.

## Setup on a New Computer

### Prerequisites

1. Clone this repository to the same location on all computers:
   ```bash
   cd ~/Dropbox/GitHub/
   git clone <your-repo-url> claude-config
   ```

2. Ensure Dropbox is synced (if using Dropbox sync)

### Installation

Run the setup script to create symlinks:

```bash
cd ~/Dropbox/GitHub/claude-config
./setup.sh
```

The script will:
- Create the `~/.claude/` directory if it doesn't exist
- Backup any existing `settings.json` to `settings.json.backup`
- Create a symlink from `~/.claude/settings.json` to this repo's `settings.json`

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

## Sharing with Coworkers

To share this configuration:

1. Make sure sensitive information is removed from `settings.json` (API keys, personal paths, etc.)
2. Share the repository URL
3. Coworkers can clone and run `./setup.sh` to set up on their machines
4. They can customize their own settings as needed

### Note on Hooks

The `settings.json` includes hooks that run shell commands. Review these before sharing:
- Terminal notifications may require `terminal-notifier` to be installed
- Adjust paths and commands for different environments if needed

## Current Settings

### Model Preference
- Default model: `sonnet` (Claude Sonnet 4.5)

### Hooks
- **Notification Hook**: Shows a terminal notification when tasks complete
  - Requires: `terminal-notifier` (install via Homebrew: `brew install terminal-notifier`)

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

- [Claude Code Documentation](https://docs.claude.ai/claude-code)
- [Claude Code Settings Reference](https://docs.claude.ai/claude-code/settings)
- [Claude Code Hooks](https://docs.claude.ai/claude-code/hooks)
