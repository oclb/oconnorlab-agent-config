# Claude Code Configuration Repository

This repository customizes Claude Code for scientific research workflows. It contains skills, behavioral settings, hooks, and setup scripts.

## Quick Reference

**What this repo provides:**
- Custom skills for research: data analysis, data validation, tool learning, scientific writing
- O2 cluster access via remote SSH connection
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
│   ├── remote-o2/            # Remote O2 access via SSH
│   ├── use-o2/               # SLURM reference (used by remote-o2)
│   ├── perform-analysis/     # 8-step analysis framework with notebook integration
│   ├── update-notebook/      # Sync notebook for external work
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
├── o2-scripts/                # Generated scripts for remote O2 access (gitignored)
├── notify-helpers.sh          # Shell functions for notifications
└── setup.sh                   # Setup script for local machines
```

## How It Works

### Setup and Symlinks

Running `setup.sh` creates symlinks:

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
| `Environment` | `local` | Always local; use `/remote-o2` for cluster access |
| `NewUser` | `true`/`false` | When true, proactively explain features and suggest `/help` |
| `CONFIG_REPO` | Path | Location of this repo (for `/help` skill) |

Toggle AFK by including `(afk)` or `(back)` in a message. Toggle NewUser by explicitly asking Claude to enable/disable onboarding mode.

### Skills

Skills are specialized prompts in `skills/<name>/SKILL.md`. They can be:
- **Auto-activated**: Claude detects when relevant (e.g., "analyze this data" triggers `/perform-analysis`)
- **Explicitly invoked**: User types `/skill-name`

## Available Skills

### Core Research Skills

| Skill | Trigger | Purpose |
|-------|---------|---------|
| `/perform-analysis` | "analyze data", "run experiment" | 8-step analysis framework with lab notebook |
| `/update-notebook` | "sync notebook", "what's changed" | Sync notebook for work done outside Claude |
| `/new-data` | "validate data", "check this dataset" | Data validation and exploration |
| `/new-software` | "learn [tool]", "set up [library]" | Tool installation and learning |
| `/remote-o2` | "run on O2", auto-triggers for heavy compute | Remote O2 access via SSH+tmux |
| `/use-o2` | (reference skill) | SLURM reference material for remote-o2 |
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

### perform-analysis (8-Step Framework + Lab Notebook)

0. **Setup** - Initialize notebook entry, retrieve related analyses
1. **Understand Motivation** - Why is this question being asked?
2. **Set Expectations** - What results do you expect?
3. **Verify Resources** - Check data and tools are available
4. **Make a Plan** - Create step-by-step analysis plan
5. **Perform Analysis** - Execute with progress monitoring
6. **Display Results** - Create tables/figures, highlight key finding
7. **Document Choices** - Explain decisions and challenges
8. **Finalize** - Complete notebook, commit, evaluate for CLAUDE.md

Each step writes to the notebook incrementally. Git commits preserve script history.

### new-data (Data Validation)

Validates datasets by examining:
- File format and structure
- Dimensions and identifiers
- Data types and ranges
- Missing values
- Domain-specific requirements (e.g., expression data, VCF files)

### remote-o2 (Remote O2 Access)

Enables Claude Code to access O2 from a local machine via SSH + tmux:
- **First-time setup**: Collects username, lab directory, scratch directory; generates setup scripts
- **Connection management**: Establishes SSH connection; prompts reconnection when needed (Duo auth)
- **Command execution**: Sends commands via `tmux send-keys`, captures output
- **Duo behavior**: Each command = 1 Duo push off-campus; use harvard-secure wifi to avoid

Stores config in `~/.claude/behavior.conf`: `O2_USER`, `O2_LAB_DIR`, `O2_SCRATCH_DIR`, `O2_SOCKET`, `O2_REMOTE_SETUP`

### use-o2 (SLURM Reference)

Reference material for O2 cluster and SLURM (used by remote-o2):
- Partition selection (priority, short, medium, long, gpu, highmem)
- Resource estimation strategies
- SLURM script templates
- Job monitoring and troubleshooting

## Lab Notebook System

The lab notebook provides archival tracking of analyses, separate from the curated CLAUDE.md.

### Two-Tier Context System

| Layer | Location | Purpose | Lifecycle |
|-------|----------|---------|-----------|
| **Notebook** | `notebook/analyses/` | Complete archival record | Append-only, grows forever |
| **CLAUDE.md** | Project root | Curated active context | Actively pruned, current-relevant only |

### Notebook Structure

```
project/
├── CLAUDE.md                         # Key findings, current directions
├── notebook/
│   ├── INDEX.md                      # Quick-reference summary of all memories
│   ├── TODO.md                       # Persistent task list across sessions
│   ├── analyses/                     # Analysis logs and scripts
│   │   └── <analysis-name>/
│   │       ├── README.md
│   │       ├── <script>.py
│   │       └── outputs/
│   ├── data/                         # Dataset documentation
│   │   └── <dataset-name>.md         # Location, source, characteristics, issues
│   ├── software/                     # External software documentation
│   │   └── <tool-name>.md            # Installation, docs URL, issues
│   └── methods/                      # Methodological changes to codebase
│       └── YYYY-MM-DD-<description>.md
```

### How It Works

**During `/perform-analysis`:**
1. Reads `notebook/INDEX.md` for quick retrieval of related memories
2. Generates a specific, descriptive name (or uses user-provided)
3. Writes to notebook incrementally after each step
4. Updates INDEX.md with summary row on completion
5. Commits each version to current branch
6. Updates CLAUDE.md only for important findings

**Version management:**
- v0 often a pilot (subset data), v1 the full run
- All versions in single README.md
- Scripts can be modified - git tracks history

**CLAUDE.md curation:**
- Add: Important findings, working solutions, current directions
- Prune: Superseded findings, abandoned directions, stale context
- Goal: Only what affects ongoing work

### Syncing External Work

When work is done outside Claude Code, use `/update-notebook` to:
1. Review git history for methodological changes
2. Ask about recent analyses and findings
3. Create retrospective notebook entries
4. Update CLAUDE.md with current context

### Notebook Index

`notebook/INDEX.md` provides quick-reference summaries of all memories:
- **Analyses**: ID, summary, date, tags
- **Data**: ID, location, source, status
- **Software**: ID, version, purpose
- **Methods**: date, change, impact

Skills that create memories update the index in the same commit. Retrieval reads the index first, then full entries only for relevant items.

### Persistent To-Do List

`notebook/TODO.md` tracks tasks across sessions:
- Items have numbers (#1, #2, ...) and names
- Can link to related notebook entries via `Context:` field
- Completed items link to their results via `Result:` field
- Each add/complete/edit triggers a git commit

When starting work on a todo with a `Context:` link, Claude reads the linked notebook entry for background.

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
| `o2-scripts/` | Generated scripts for remote O2 access (gitignored) |
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
