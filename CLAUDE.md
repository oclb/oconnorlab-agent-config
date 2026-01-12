# Claude Code Configuration Repository

This repository customizes Claude Code for scientific research workflows. It contains skills, behavioral settings, hooks, and setup scripts for both local machines and the Harvard O2 HPC cluster.

## Quick Reference

**What this repo provides:**
- Custom skills for research: data analysis, data validation, tool learning, scientific writing
- O2 cluster integration with automatic SLURM job submission
- Behavioral flags (AFK mode, environment detection)
- Push notifications via ntfy.sh

**When working in this repo:**
- Skills are in `skills/<skill-name>/SKILL.md` (the actual prompt) and `README.md` (documentation)
- Global config is in `global/CLAUDE.md` (behavioral instructions) and `global/settings.json` (hooks, permissions)
- Setup scripts symlink these files to `~/.claude/` where Claude Code reads them
- **Keep docs updated**: Significant changes should be reflected in this file (`CLAUDE.md`) and possibly `README.md`

## Directory Structure

```
claude-config/
├── global/                    # Global configuration files
│   ├── CLAUDE.md             # Behavioral instructions for Claude
│   ├── CLAUDE.user.md        # User-specific overrides (gitignored)
│   └── settings.json         # Claude Code settings (hooks, permissions, model)
├── skills/                    # Custom skills for scientific research
│   ├── help/                 # Documentation and help system
│   ├── use-o2/               # O2 cluster job submission
│   ├── perform-analysis/     # 8-step analysis framework
│   ├── new-data/             # Data validation and exploration
│   ├── new-software/         # Tool installation and learning
│   ├── teaching-mode/        # Educational explanations
│   ├── revise-scientific-writing/  # Scientific writing guidance
│   ├── pdf/                  # PDF manipulation
│   ├── docx/                 # Word document handling
│   ├── pptx/                 # PowerPoint handling
│   └── skill-creator/        # Guide for creating new skills
├── hooks/                     # Shell scripts for Claude Code hooks
│   └── notify.sh             # Cross-platform notification hook
├── notify-helpers.sh          # Shell functions for notifications
├── setup.sh                   # Setup script for local machines
└── setup-o2.sh               # Setup script for O2 cluster
```

## How It Works

### Setup and Symlinks

Running `setup.sh` (local) or `setup-o2.sh` (cluster) creates symlinks:

```
~/.claude/CLAUDE.md      → global/CLAUDE.md
~/.claude/settings.json  → global/settings.json
~/.claude/skills/        → skills/
~/.claude/hooks/         → hooks/
```

Setup also creates `~/.claude/behavior.conf` and configures shell notifications.

### Behavior Flags

Claude reads `~/.claude/behavior.conf` at session start:

| Flag | Values | Effect |
|------|--------|--------|
| `AFK` | `true`/`false` | When true, work autonomously without asking questions |
| `Environment` | `local`/`O2` | Local execution vs. SLURM job submission |
| `CONFIG_REPO` | Path | Location of this repo (for `/help` skill) |

Toggle AFK by including `(afk)` or `(back)` in a message.

### Skills

Skills are specialized prompts in `skills/<name>/SKILL.md`. They can be:
- **Auto-activated**: Claude detects when relevant (e.g., "analyze this data" triggers `/perform-analysis`)
- **Explicitly invoked**: User types `/skill-name`

## Available Skills

### Core Research Skills

| Skill | Trigger | Purpose |
|-------|---------|---------|
| `/perform-analysis` | "analyze data", "run experiment" | 8-step systematic analysis framework |
| `/new-data` | "validate data", "check this dataset" | Data validation and exploration |
| `/new-software` | "learn [tool]", "set up [library]" | Tool installation and learning |
| `/use-o2` | "submit to O2", resource-intensive tasks | SLURM job submission on O2 |
| `/teaching-mode` | "teach me", "explain how" | Educational explanations with replication steps |
| `/revise-scientific-writing` | "revise manuscript", "edit abstract" | Scientific writing improvement |
| `/help` | "what can you do", questions about Claude Code | Documentation and capability overview |

### Document Skills

| Skill | Purpose |
|-------|---------|
| `/pdf` | Extract text/tables, create/merge PDFs, fill forms |
| `/docx` | Create/edit Word documents, track changes |
| `/pptx` | Create/edit PowerPoint presentations |

### Meta Skills

| Skill | Purpose |
|-------|---------|
| `/skill-creator` | Guide for creating new custom skills |

## Key Skill Details

### perform-analysis (8-Step Framework)

1. **Understand Motivation** - Why is this question being asked?
2. **Set Expectations** - What results do you expect?
3. **Verify Resources** - Check data and tools are available
4. **Make a Plan** - Create step-by-step analysis plan
5. **Perform Analysis** - Execute with progress monitoring
6. **Display Results** - Create tables/figures, highlight key finding
7. **Document Choices** - Explain decisions and challenges
8. **List Files** - Provide paths to all created files

### new-data (Data Validation)

Validates datasets by examining:
- File format and structure
- Dimensions and identifiers
- Data types and ranges
- Missing values
- Domain-specific requirements (e.g., expression data, VCF files)

### use-o2 (SLURM Integration)

Automatically determines:
- Appropriate partition (short, medium, long, gpu, highmem)
- Resource requirements (memory, cores, time)
- Creates submission scripts
- Provides monitoring commands

## Notifications

The repository includes a cross-platform notification system using ntfy.sh:

### Automatic Notifications (via hooks)
- **Notification hook**: Fires when Claude needs user input
- **Stop hook**: Fires when Claude completes a task

### Manual Notifications (via helper functions)
```bash
notify "Message"                    # Simple notification
notify "Message" "Title" high       # With title and priority
notifyme long_running_command       # Notify when command completes
test_notify                         # Test notification setup
```

### In SLURM Scripts
```bash
notify_job_complete $?              # Notify job success/failure
```

## Key Files

| File | Purpose |
|------|---------|
| `global/CLAUDE.md` | Behavioral instructions Claude follows |
| `global/settings.json` | Model, permissions, hooks configuration |
| `~/.claude/behavior.conf` | Runtime flags (created by setup scripts) |
| `skills/<name>/SKILL.md` | Skill prompt definition |
| `skills/<name>/README.md` | Skill documentation |
| `hooks/notify.sh` | Cross-platform notification hook |
| `notify-helpers.sh` | Shell functions: `notify`, `notifyme`, `test_notify` |

## Customization

**User-specific behavioral instructions** (not committed to git):
- Create `global/CLAUDE.user.md` with personal preferences
- Claude reads this after `global/CLAUDE.md` and applies overrides

**Adding or modifying skills**:
```
skills/my-skill/
├── SKILL.md          # The prompt Claude follows
└── README.md         # Documentation
```

Use `/skill-creator` for guided skill creation or manually create the directory with both files.

## Resources

- [Claude Code Documentation](https://docs.claude.ai/claude-code)
- [O2 Wiki](https://harvardmed.atlassian.net/wiki/spaces/O2/overview)
- [ntfy.sh Documentation](https://docs.ntfy.sh)
