# User-Specific Configuration

This document explains how to add your own personal settings and instructions without creating git conflicts.

## Overview

The repository supports user-specific overrides that are automatically gitignored:

- **`CLAUDE.user.md`** - Personal instructions for Claude Code (appended to CLAUDE.md)
- **`settings.user.json`** - Personal settings overrides (merged with base settings)

These files are never committed to the repository, so you can customize freely without conflicts.

## CLAUDE.user.md - Personal Instructions

### Setup

1. Copy the example file:
   ```bash
   cp CLAUDE.user.md.example CLAUDE.user.md
   ```

2. Edit `CLAUDE.user.md` with your personal instructions

3. Claude Code will automatically read both `CLAUDE.md` and `CLAUDE.user.md`

### What to Put Here

- **Your context**: Role, projects, common paths
- **Your preferences**: Coding style, tools, workflows
- **Custom behavior**: How you like Claude to interact
- **Domain knowledge**: Specific topics you work on
- **Shortcuts**: Common commands or patterns

### Example

```markdown
# My Personal Context

I'm a bioinformatician at HMS working on GWAS.

## My Preferences

- Use base R over tidyverse
- Always explain statistical methods
- Prefer concise responses

## My Common Paths

- Projects: /n/data1/hms/dbmi/oconnor/projects/
- Data: /n/data1/hms/dbmi/oconnor/data/

## My Workflow

When I say "run standard GWAS":
1. Use plink with my default parameters
2. Submit to O2 short partition with 32GB RAM
3. Output results to my standard location
```

## settings.user.json - Personal Settings

### Setup

1. Copy the example file:
   ```bash
   cp settings.user.json.example settings.user.json
   ```

2. Edit `settings.user.json` with your overrides

3. **Note:** Currently requires manual merging with settings.json
   - See "Merging Settings" section below

### What to Override

- Model preferences (`model`, `maxTokens`)
- Custom hooks
- WebFetch permissions for private domains
- Plugin configurations
- Editor preferences

### Example

```json
{
  "model": "opus",
  "permissions": {
    "allowWithinAllow": {
      "WebFetch": [
        "my-private-gitlab.com",
        "internal-docs.company.com"
      ]
    }
  },
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "~/my-custom-hook.sh"
          }
        ]
      }
    ]
  }
}
```

## Merging Settings (Advanced)

### Option 1: Manual Merge (Current)

Edit your `settings.json` or `settings.local.json` directly with your overrides. These files are gitignored.

### Option 2: JSON Merge Script (Future)

We plan to add a merge script that automatically combines:
- `settings.template.json` (base settings from repo)
- `settings.user.json` (your overrides)
- → `settings.local.json` (final merged settings)

This would run automatically in setup scripts.

## Setup Scripts

### Local Setup

The `setup.sh` script:
- Creates symlinks for CLAUDE.md and skills
- Prompts you to create `CLAUDE.user.md` if it doesn't exist
- Configures behavior.conf

### O2 Setup

The `setup-o2.sh` script:
- Does everything setup.sh does
- Configures O2-specific paths and notifications
- Sets up TMPDIR and sandbox dependencies
- Prompts for user-specific configuration

## Best Practices

### What to Commit

✅ Commit to repo:
- Base configuration (CLAUDE.md, settings.template.json)
- Skills and plugins
- Documentation
- Setup scripts

### What NOT to Commit

❌ Don't commit:
- `CLAUDE.user.md` - Personal instructions
- `settings.user.json` - Personal overrides
- `settings.json` - Machine-specific settings
- `settings.local.json` - Generated merged settings
- API keys or credentials

### Sharing User Config

If you want to share your user config with teammates:
1. Keep a separate private repo with your `*.user.*` files
2. Or copy them manually to new machines
3. Or keep them in your dotfiles repo

## Migration Guide

If you have existing personal settings in the repo:

1. **Extract personal instructions:**
   ```bash
   # Move your personal notes from CLAUDE.md to CLAUDE.user.md
   vim CLAUDE.user.md
   ```

2. **Extract personal settings:**
   ```bash
   # Copy your overrides from settings.json to settings.user.json
   vim settings.user.json
   ```

3. **Test:**
   ```bash
   # Verify Claude Code still works
   claude "echo hello"
   ```

4. **Commit the cleanup:**
   ```bash
   git add CLAUDE.md settings.template.json
   git commit -m "Remove personal config from repo"
   ```

## Troubleshooting

### CLAUDE.user.md not being read

Claude Code reads CLAUDE.md, which now instructs Claude to also check for CLAUDE.user.md. If it's not working:
1. Verify the file exists: `ls -la $CONFIG_REPO/CLAUDE.user.md`
2. Check it's referenced in CLAUDE.md
3. Restart your Claude Code session

### Settings not applying

If your settings.user.json overrides aren't working:
1. Currently requires manual merging into settings.json
2. Verify JSON syntax: `jq . settings.user.json`
3. Check file is gitignored: `git status`

### Git conflicts

If you get conflicts despite using user files:
1. Make sure your personal changes are in `*.user.*` files
2. Check `.gitignore` includes the user files
3. Stash any uncommitted changes: `git stash`
4. Pull and re-apply: `git pull && git stash pop`

## See Also

- [Main README](README.md) - Repository overview
- [NOTIFICATIONS.md](NOTIFICATIONS.md) - Notification setup
- [skills/](skills/) - Available skills
