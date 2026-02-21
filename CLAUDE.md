# Claude Code Configuration Repository

This repository customizes Claude Code for scientific research workflows. It contains skills, behavioral settings, hooks, and setup scripts.

## Quick Reference

**What this repo provides:**
- Custom skills for research: data analysis, data validation, tool learning, scientific writing
- O2 cluster access via remote SSH connection
- AFK mode for autonomous operation (per-turn)
- Local notifications via terminal-notifier

**When working in this repo:**
- Skills are in `skills/<skill-name>/SKILL.md` (the actual prompt) and `README.md` (documentation)
- Global config is in `global/CLAUDE.md` (behavioral instructions) and `global/settings.json` (hooks, permissions)
- Setup scripts configure `~/.claude/` to import/symlink these files (see "Setup Architecture" below)
- **Keep docs updated**: When making significant changes, identify and update all affected documentation and skills (see checklist below)

**Documentation update checklist** (for significant changes):
1. **This file (`CLAUDE.md`)** - Update if the change affects how users interact with or understand the repo
2. **`README.md`** - Update if the change affects installation, setup, or high-level overview
3. **`global/CLAUDE.md`** - Update if the change affects Claude's behavioral instructions
4. **Affected skill's `SKILL.md`** - Update the skill prompt if functionality changes
5. **Affected skill's `README.md`** - Update skill documentation for user-facing changes
6. **`/support` skill** - Update if new capabilities or skills are added
7. **Related skills** - Check if other skills reference the changed functionality

## Directory Structure

```
claude-config/
├── global/                    # Global configuration files
│   ├── CLAUDE.md             # Behavioral instructions for Claude (imported by user's CLAUDE.md)
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
│   └── notify.sh             # Local notification hook (macOS)
├── templates/                 # Templates for project setup
│   └── project-settings.json # Project permissions template (notebook access)
├── o2-scripts/                # Generated scripts for remote O2 access (gitignored)
└── setup.sh                   # Setup script for local machines
```

## How It Works

### Setup Architecture

Running `setup.sh` configures Claude Code with a mix of user-owned and repo-owned files:

| File | Type | Description |
|------|------|-------------|
| `~/.claude/CLAUDE.md` | User-owned | User's file with `@import` to repo's global/CLAUDE.md |
| `~/.claude/settings.json` | Symlink → repo | Shared settings, auto-updates when repo changes |
| `~/.claude/settings.local.json` | User-owned | User's personal settings + O2 permissions |
| `~/.claude/skills/` | Symlink → repo | Shared skills |
| `~/.claude/hooks/` | Symlink → repo | Notification hooks |

**Key benefits:**
- User keeps ownership of their CLAUDE.md (can add personal instructions before/after import)
- Repo updates to settings.json and skills are picked up automatically
- User's existing settings are preserved (renamed to settings.local.json if needed)

### AFK Mode

Include `(afk)` in any message for autonomous operation on that turn. Claude proceeds independently, only pausing for irreversible actions or critical decisions.

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

### Review Skills

| Skill | Trigger | Purpose |
|-------|---------|---------|
| `/postmortem` | User invokes manually | Reflective review of completed work: undiscussed choices, friction, improvements, test quality |

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

### use-o2 (SLURM Reference)

Reference material for O2 cluster and SLURM (used by remote-o2):
- Partition selection (priority, short, medium, long, gpu, highmem)
- Resource estimation strategies
- SLURM script templates
- Job monitoring and troubleshooting

## Lab Notebook System

The lab notebook provides archival memory of all work, separate from the curated CLAUDE.md.

### Two-Repo Architecture

The notebook is a **separate git repository** inside the project, gitignored from the main repo:

```
project/                              # Main git repo
├── .git/
├── .gitignore                        # Contains: notebook/
├── CLAUDE.md                         # Active context (references entries)
├── .claude/settings.json             # Project permissions
├── src/                              # Your code
└── notebook/                         # Separate git repo (gitignored)
    ├── .git/
    ├── INDEX.md                      # Entry index (Date, Name, Summary)
    ├── ARCHIVE.md                    # Archived entries (out of active index)
    ├── TODO.md                       # Active tasks
    ├── DONE.md                       # Completed tasks
    └── entries/                      # All memories
        └── YYYY-MM-DD-<slug>.md
```

**Setup:** Run `/init-project` to create this structure automatically.

### Entries

All memories go in `notebook/entries/`. Any task that produces knowledge worth recalling creates an entry: analyses, features, research, discussions, tool setup, presentations, etc.

**Entry format:**
```markdown
# <Descriptive Title>

**Date:** YYYY-MM-DD

## Summary
[What was done and why]

## Details
[The work itself - updated as work progresses]

## References
- `<entry-name>`: <why it was useful>
```

The **References** section records which previous entries informed this work and why. This creates a lightweight knowledge graph that helps retrieval.

### Two-Tier Context System

| Layer | Location | Purpose | Lifecycle |
|-------|----------|---------|-----------|
| **Notebook** | `notebook/entries/` | Complete archival record | Grows forever |
| **CLAUDE.md** | Project root | Curated active context | Pruned when stale |

**CLAUDE.md references entries:** Important findings get a reference like "Variant filtering excludes singletons (see `variant-filtering-v2`)". Remove references when superseded.

### Index and Retrieval

`notebook/INDEX.md` is a minimal table: Date, Name, Summary. Claude reads it at session start to know what memories exist.

**Retrieval:** Use Explore subagent to find relevant entries when user references past work or when starting multi-step planning. Explore starts at INDEX.md, then reads full entries as needed.

**Archiving:** When INDEX.md grows large (~100+ entries), old entries that are no longer actively relevant can be moved from INDEX.md to ARCHIVE.md. Entry files stay in `notebook/entries/` - only the index row moves. Archived entries are still searchable as a fallback. See `global/CLAUDE.md` for full archiving criteria and workflow.

### Persistent To-Do List

- `notebook/TODO.md` - Active tasks (includes a `Next ID:` counter at the top to prevent numbering collisions)
- `notebook/DONE.md` - Completed tasks with `Result:` links to entries

### Feedback

Feedback about Claude's behavior goes to this repository's `feedback/` directory (not the project's notebook). This centralizes feedback for contribution back to the project.

## Notifications

Local notifications via macOS Notification Center:

- **Requires**: `brew install terminal-notifier`
- **Notification hook**: Fires when Claude needs user input
- **Stop hook**: Fires when Claude completes a task

Falls back to `osascript` if terminal-notifier is not installed.

## Key Files

| File | Purpose |
|------|---------|
| `global/CLAUDE.md` | Behavioral instructions (imported via `@import` by user's CLAUDE.md) |
| `global/settings.json` | Model, permissions, hooks configuration (symlinked to ~/.claude/) |
| `~/.claude/CLAUDE.md` | User-owned file that imports global/CLAUDE.md |
| `~/.claude/settings.local.json` | User-specific permissions (user-owned, O2 perms added by setup.sh) |
| `templates/project-settings.json` | Template for project notebook permissions |
| `o2-scripts/` | Generated scripts for remote O2 access (gitignored) |
| `skills/<name>/SKILL.md` | Skill prompt definition |
| `skills/<name>/README.md` | Skill documentation |
| `hooks/notify.sh` | Local notification hook (macOS) |

## Customization

**User-specific behavioral instructions:**
- Edit your `~/.claude/CLAUDE.md` directly (it's user-owned, not a symlink)
- Add personal instructions before or after the `@import` line
- The imported repo config provides the base; your additions override or extend it

**User-specific settings:**
- Edit `~/.claude/settings.local.json` (takes precedence over the symlinked settings.json)
- Add your own permissions, model preferences, etc.

**Adding or modifying skills:**
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
- `WebFetch(domain:*)` for any domain
- `Bash` for remote-bridge

**User local settings** (`~/.claude/settings.local.json`, created by `setup.sh`):
- `Bash` permissions for O2 scripts (user-specific paths)
- `Read`/`Write` for feedback directory

### Project Setup

For projects using the notebook system, run `/init-project` to automatically set up permissions, or manually copy the template:

```bash
mkdir -p .claude
cp <config-repo>/templates/project-settings.json .claude/settings.json
```

(where `<config-repo>` is this repository's location)

This pre-approves:
- `Read(/**)` for project-wide search (enables Glob/Grep tools)
- `Read`/`Edit`/`Write` for `notebook/**`
- `Bash(git -C notebook *)` for notebook git operations

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
