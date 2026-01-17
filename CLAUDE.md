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
│   ├── support/              # Documentation and help system
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
├── templates/                 # Templates for project setup
│   └── project-settings.json # Project permissions template (notebook access)
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
| `CONFIG_REPO` | Path | Location of this repo (for `/support` skill) |

Toggle AFK by including `(afk)` or `(back)` in a message.

### Skills

Skills are specialized prompts in `skills/<name>/SKILL.md`. They can be:
- **Auto-activated**: Claude detects when relevant (e.g., "analyze this data" triggers `/perform-analysis`)
- **Explicitly invoked**: User types `/skill-name`

## Available Skills

### Setup Skills

| Skill | Trigger | Purpose |
|-------|---------|---------|
| `/init-project` | First time with a project | Initialize project with notebook, permissions, CLAUDE.md |

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
| `/support` | "what can you do", questions about Claude Code | Documentation and capability overview |

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

### Two-Repo Architecture

The notebook is a **separate git repository** inside the project, gitignored from the main repo:

```
project/                              # Main git repo
├── .git/
├── .gitignore                        # Contains: notebook/
├── CLAUDE.md                         # Key findings, current directions
├── .claude/settings.json             # Project permissions
├── src/                              # Your code
└── notebook/                         # Separate git repo (gitignored)
    ├── .git/
    ├── INDEX.md                      # Quick-reference summary
    ├── TODO.md                       # Active tasks
    ├── DONE.md                       # Completed tasks
    ├── analyses/                     # Analysis logs and scripts
    │   └── <analysis-name>/
    │       ├── README.md
    │       ├── <script>.py
    │       └── outputs/
    ├── methods/                      # Features, bugfixes, data, tools, decisions
    │   └── YYYY-MM-DD-<description>.md
    └── feedback/                     # Self-improvement feedback
        └── YYYY-MM-DD-<description>.md
```

**Why separate repos?**
- Main repo git log stays clean (only code changes)
- Notebook can have different backup/sharing rules
- Prevents repo bloat from analysis outputs
- Easy to share code without sharing exploratory work

**Setup:** Run `/init-project` to create this structure automatically.

### Two-Tier Context System

| Layer | Location | Purpose | Lifecycle |
|-------|----------|---------|-----------|
| **Notebook** | `notebook/` (separate repo) | Complete archival record | Append-only, grows forever |
| **CLAUDE.md** | Project root (main repo) | Curated active context | Actively pruned, current-relevant only |

### How It Works

**During `/perform-analysis`:**
1. Reads `notebook/INDEX.md` for quick retrieval of related memories
2. Generates a specific, descriptive name (or uses user-provided)
3. Writes to notebook incrementally after each step
4. Updates INDEX.md with summary row on completion
5. Commits to **notebook repo**: `git -C notebook add . && git -C notebook commit`
6. Pushes to notebook remote (if configured)
7. Updates CLAUDE.md only for important findings (committed to main repo)

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
- **Methods**: date, type (feature/bugfix/data/tool/decision), summary

Skills that create memories update the index in the same commit. Retrieval reads the index first, then full entries only for relevant items.

### Persistent To-Do List

Two files track tasks across sessions:
- `notebook/TODO.md` - Active tasks (kept small)
- `notebook/DONE.md` - Completed tasks with full original record + result

Items have numbers (#1, #2, ...) and can link to related notebook entries via `Context:` field. When starting work on a todo with a `Context:` link, Claude reads the linked notebook entry for background.

### Feedback Logging

`notebook/feedback/` captures issues with Claude's skills and behavior for improvement. Claude proactively suggests logging feedback when:
- Skill detection failed (user had to invoke manually)
- O2 approach failed on first attempt
- User signals skepticism ("hmm", corrections)
- Memory retrieval or creation failed

Feedback is freeform - no template. User manages review.

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
| `global/settings.json` | Model, permissions, hooks configuration (git-tracked) |
| `~/.claude/settings.local.json` | User-specific permissions (created by setup.sh) |
| `~/.claude/behavior.conf` | Runtime flags (created by setup scripts) |
| `templates/project-settings.json` | Template for project notebook permissions |
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

## Permissions

Claude Code uses a layered permission system. This repo configures permissions at multiple levels:

### Permission Layers (highest to lowest precedence)

1. **Project local** - `.claude/settings.local.json` (gitignored, user-specific)
2. **Project shared** - `.claude/settings.json` (committed, shared with team)
3. **User local** - `~/.claude/settings.local.json` (user-specific)
4. **User global** - `~/.claude/settings.json` (symlinked to `global/settings.json`)

### What This Repo Configures

**Global settings** (`global/settings.json`):
- `Read`/`Edit` for `~/.claude/behavior.conf` (behavioral flags)
- `WebFetch` for allowed domains (GitHub, O2 docs, ntfy.sh)

**User local settings** (`~/.claude/settings.local.json`, created by `setup.sh`):
- `Bash` permissions for O2 scripts (user-specific paths)

### Project Setup

For projects using the notebook system, copy the template to enable notebook permissions:

```bash
mkdir -p .claude
cp $CONFIG_REPO/templates/project-settings.json .claude/settings.json
```

This pre-approves:
- `Read`/`Edit`/`Write` for `notebook/**`
- `Bash` for notebook-related git commits

### Permission Syntax Reference

| Pattern | Meaning |
|---------|---------|
| `Read(/path/**)` | Read files under path (relative to settings file) |
| `Read(~/path)` | Read from home directory |
| `Read(//absolute/path)` | Read from absolute filesystem path |
| `Edit(/notebook/**)` | Edit files in notebook directory |
| `Bash(command:*)` | Allow bash commands starting with "command" |
| `Bash(*pattern*)` | Allow bash commands containing "pattern" |
| `WebFetch(domain:example.com)` | Allow fetching from domain |

## Resources

- [Claude Code Documentation](https://docs.claude.ai/claude-code)
- [O2 Wiki](https://harvardmed.atlassian.net/wiki/spaces/O2/overview)
- [ntfy.sh Documentation](https://docs.ntfy.sh)
