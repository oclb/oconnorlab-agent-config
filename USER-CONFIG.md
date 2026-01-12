# User-Specific Configuration

This document explains how to add your own personal instructions without creating git conflicts.

## Overview

The repository supports a user-specific instruction file that is gitignored:

- **`global/CLAUDE.user.md`** - Personal instructions for Claude Code (read after CLAUDE.md)

This file is never committed to the repository, so you can customize freely without conflicts.

## CLAUDE.user.md - Personal Instructions

### Setup

1. Copy the example file:
   ```bash
   cp global/CLAUDE.user.md.example global/CLAUDE.user.md
   ```

2. Edit `global/CLAUDE.user.md` with your personal instructions

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

## Editing Shared Settings

Settings are stored in `global/settings.json` and tracked in git. To modify settings:

1. Edit `global/settings.json` directly
2. Commit and push your changes

If you need personal settings that differ from the shared config, you can:
- Create a project-level `.claude/settings.json` that overrides user settings
- Or maintain your own fork of this repository

## What's Tracked vs Gitignored

**Tracked in git:**
- `global/CLAUDE.md` - Shared instructions
- `global/settings.json` - Shared settings
- `skills/` - Shared skills
- `hooks/` - Shared hooks

**Gitignored (personal):**
- `global/CLAUDE.user.md` - Personal instructions

## Troubleshooting

### CLAUDE.user.md not being read

Claude Code reads CLAUDE.md, which instructs Claude to also check for CLAUDE.user.md. If it's not working:
1. Verify the file exists: `ls -la $CONFIG_REPO/global/CLAUDE.user.md`
2. Check it's referenced in CLAUDE.md
3. Restart your Claude Code session

## See Also

- [Main README](README.md) - Repository overview
- [NOTIFICATIONS.md](NOTIFICATIONS.md) - Notification setup
- [skills/](skills/) - Available skills
