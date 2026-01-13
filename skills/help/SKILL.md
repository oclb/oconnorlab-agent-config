---
name: help
description: This skill should be used when the user asks about Claude Code itself, wants to know what skills are available, asks "what can you do", "how do I use X feature", needs help with Claude Code configuration, or wants an overview of capabilities for scientific research.
version: 1.0.0
---

# Help Skill

Provides help and documentation about Claude Code and this configuration's capabilities for scientific research.

## When This Skill Applies

Use when the user:
- Asks about Claude Code features or capabilities
- Wants to know what skills are available
- Asks "what can you do?" or "help me get started"
- Needs help with Claude Code configuration or settings
- Asks about hooks, MCP servers, or other Claude Code features
- Wants an overview of how to use Claude Code for research

### Proactive Triggering (NewUser Mode)

When `NewUser=true` in behavior.conf, consider invoking this skill proactively:

- **User seems lost**: Asks very open-ended questions or seems unsure what to do
- **First session**: Provide a brief orientation after completing the first task
- **Confusion about capabilities**: User tries to do something that a skill handles better
- **Explicit curiosity**: User asks "how does this work?" or "what else can you do?"

When triggering proactively, keep it brief:
- Don't dump the full overview unless asked
- Mention 1-2 relevant skills for their current work
- Suggest they can ask for more with `/help`

## Process

### Step 1: Fetch Documentation

**Clone or update the Claude Code documentation:**

```bash
# Create temp directory for docs (use TMPDIR if set, e.g., on O2 cluster)
DOCS_DIR="${TMPDIR:-/tmp/claude}/claude-code-docs"

# Clone if not exists, otherwise pull latest
if [ -d "$DOCS_DIR" ]; then
    cd "$DOCS_DIR" && git pull --quiet
else
    git clone --depth 1 https://github.com/ericbuess/claude-code-docs.git "$DOCS_DIR"
fi
```

The documentation is organized in `$DOCS_DIR/docs/` with files like:
- `cli-reference.md` - Command line options and usage
- `hooks.md` - Hook system documentation
- `mcp.md` - MCP server configuration
- `memory.md` - Memory and context management
- `settings.md` - Configuration settings
- `tutorials/*.md` - Various tutorials

### Step 2: Read Configuration Repository README

Read the config repo README for skill-specific information:

```bash
# Get config repo location from behavior.conf
CONFIG_REPO=$(grep "^CONFIG_REPO=" ~/.claude/behavior.conf 2>/dev/null | cut -d= -f2)

# Fall back to default location if not set
CONFIG_REPO="${CONFIG_REPO:-$HOME/Dropbox/GitHub/claude-config}"

# Read the README
cat "$CONFIG_REPO/README.md"
```

### Step 3: Answer Query or Provide Overview

**If user has a specific question:**
1. Search the downloaded docs using grep/read
2. Find relevant documentation files
3. Provide a clear, concise answer with examples
4. Cite the source documentation

**If user wants a general overview, provide the Scientific Research Overview below.**

## Scientific Research Overview

When providing an overview for scientific research users, include ALL of the following sections:

### What is Claude Code?

Claude Code is an agentic coding assistant that runs in your terminal. Unlike chat-based interfaces, Claude Code can:
- Read and write files directly on your system
- Execute shell commands
- Search and navigate codebases
- Run analyses and generate outputs
- Work autonomously on multi-step tasks

### Available Skills

This configuration includes specialized skills for scientific research:

| Skill | Description |
|-------|-------------|
| **perform-analysis** | Systematic 8-step framework for data analyses and experiments. Understands motivation, verifies resources, creates plans, executes analysis with progress tracking, and documents all results and choices made. |
| **sanity-check-data** | Comprehensive dataset validation and exploration. Downloads/acquires data, examines format and structure, computes statistics, checks for issues, validates with domain-specific rules, and provides actionable recommendations. |
| **learn-tool** | Quickly learn and set up new tools, libraries, and frameworks. Searches documentation, handles installation, runs sanity checks, and provides usage examples. |
| **use-o2** | Submit jobs to Harvard's O2 HPC cluster using SLURM. Helps estimate resources, write submission scripts, monitor jobs, and troubleshoot issues. |
| **docx** | Create, edit, and analyze Word documents with support for tracked changes, comments, and formatting preservation. |
| **pdf** | PDF manipulation including text extraction, form filling, merging/splitting, and programmatic generation. |
| **pptx** | Create and edit PowerPoint presentations with layout support, speaker notes, and content modification. |
| **revise-scientific-writing** | Review and improve scientific manuscripts, applying writing principles and checking structure. |
| **teaching-mode** | Educational explanations that walk through concepts step-by-step, showing how to replicate analyses. |
| **skill-creator** | Guide for creating new skills to extend Claude Code's capabilities. |

### Running on O2 Cluster

Claude Code can be run on Harvard's O2 HPC cluster for computational work. Key points:

- **Setup**: Run `setup-o2.sh` from the config repository to configure TMPDIR, install sandbox dependencies, and create symlinks
- **Sandbox Mode**: Claude Code runs commands in a sandboxed environment by default for safety. On O2, the setup script installs `socat` via conda to enable sandbox functionality. Some operations may require disabling sandbox mode (Claude will prompt when needed)
- **TMPDIR**: O2 has limited space in `/tmp`; the setup script configures TMPDIR to use your scratch space
- **Job Submission**: Use the `use-o2` skill for help with SLURM job submission, resource estimation, and monitoring

See the `use-o2` skill documentation for detailed SLURM usage, partition selection, and best practices.

### AFK Mode

AFK (Away From Keyboard) mode allows Claude Code to work more autonomously:

- **Enable**: Include `(afk)` in your message
- **Disable**: Include `(back)` in your message

When AFK mode is enabled:
- Claude makes reasonable decisions without asking for confirmation
- Proceeds with likely interpretations rather than clarifying ambiguities
- Completes multi-step tasks autonomously
- Only pauses for critical decisions that would be difficult to reverse

This is useful for longer-running tasks where you want Claude to proceed independently.

### NewUser Mode (Onboarding)

NewUser mode is enabled by default when you first set up Claude Code. When enabled:

- Claude proactively mentions relevant skills as they become useful
- Offers brief explanations of what it's doing
- May suggest the `/help` command for more information
- Introduces features like AFK mode when appropriate

**Disabling NewUser mode**: Once you're comfortable with the system, ask Claude to "disable onboarding mode" or "turn off NewUser mode". Claude will update behavior.conf and switch to more efficient, less explanatory responses.

**Re-enabling**: Ask Claude to "enable onboarding mode" if you want more guidance again.

### Feedback and Contributions

Luke welcomes feedback on this configuration! You are encouraged to:

- **Report issues**: If something doesn't work as expected
- **Suggest improvements**: Ideas for new skills or better workflows
- **Fix bugs**: Submit PRs for issues you encounter
- **Add skills**: Create new skills and contribute them back

The configuration repository is designed to be shared and extended. Check the `skill-creator` skill for guidance on creating new skills.

## Example Responses

### For "What can you do?"

Provide the full Scientific Research Overview above.

### For "How do I use hooks?"

1. Search the docs in `$DOCS_DIR/docs/`
2. Read `hooks.md`
3. Provide explanation with examples from the docs
4. Mention that this config already has a notification hook configured

### For "What skills are available?"

List the skills table from the overview, with brief descriptions.

### For "How do I run on O2?"

1. Read the `use-o2` skill documentation
2. Explain the setup process (`setup-o2.sh`)
3. Cover sandbox mode and TMPDIR configuration
4. Point to the skill for detailed SLURM help

## Integration

This skill integrates with:
- **learn-tool**: For learning about Claude Code features
- **use-o2**: For cluster-specific help
- **skill-creator**: For extending capabilities
