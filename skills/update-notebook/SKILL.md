---
name: update-notebook
description: This skill should be used when the user asks to "sync the notebook", "update the notebook", "catch up on changes", or when Claude detects an unfamiliar project with existing work that would benefit from notebook initialization. Also triggers for "what's changed" or "what did I miss" in the context of project work done outside Claude Code.
version: 1.0.0
---

# Update Notebook Skill

Synchronizes the lab notebook when work has been done outside Claude Code, or onboards Claude to an existing project.

## When to Use

- **Manual invocation:** User types `/update-notebook`
- **Auto-suggest:** When Claude recognizes a project has existing work but no notebook, or the notebook seems stale relative to recent git activity

## Purpose

The lab notebook (`notebook/`) is the archival record of the project:
- `analyses/` - Analysis logs, scripts, and outputs
- `data/` - Dataset documentation
- `software/` - External tool documentation
- `methods/` - Codebase change log

When users work outside Claude Code, the notebook can become out of sync. This skill:

1. Reviews recent changes to understand what happened
2. Asks the user about important updates
3. Creates retrospective notebook entries (clearly marked as such)
4. Updates CLAUDE.md with current project state

## Workflow

### Step 1: Assess Current State

1. **Check if notebook exists:**
   ```bash
   ls -la notebook/ 2>/dev/null || echo "No notebook found"
   ```

2. **Review recent git history:**
   ```bash
   git log --oneline -20
   git diff --stat HEAD~10..HEAD  # What files changed recently
   ```

3. **Identify analysis-related changes:**
   - Look for changes to scripts, data processing code, results files
   - Note new directories or significant file additions
   - Identify methodological changes (new packages, algorithm updates)

### Step 2: Ask About Method Changes

For each significant methodological change identified:

**Ask the user** (unless trivial):
- "I see changes to [file/module]. Can you briefly explain what changed and why?"
- "There's a new dependency on [package]. What is it used for?"

**Skip asking about:**
- Formatting/style changes
- Documentation updates
- CLI/tooling changes unrelated to analysis methods
- Changes you can clearly understand from context

**Record important method changes** for context, but don't try to exhaustively document everything.

### Step 3: Open-Ended Sync

Ask the user open-ended questions:

1. **Recent analyses:**
   > "Have there been any analyses or experiments completed since we last synced? What were the key findings?"

2. **Current direction:**
   > "What's the current research focus or question you're working on?"

3. **Status changes:**
   > "Is there anything that was blocked that's now working, or vice versa?"

4. **Priorities:**
   > "What are the most important next steps?"

### Step 4: Create Retrospective Entries

For analyses done outside Claude Code, create notebook entries:

1. **Create analysis directory:**
   ```
   notebook/analyses/<analysis-name>/
   ├── README.md
   └── outputs/  (if applicable)
   ```

2. **Mark as retrospective in README.md:**
   ```markdown
   # <Analysis Name>

   **Analysis ID:** `<analysis-name>`
   **Performed:** YYYY-MM-DD (approximate)
   **Recorded:** YYYY-MM-DD (retrospective entry)
   **Status:** Complete

   > Note: This analysis was performed outside Claude Code. This entry was created retrospectively to maintain project context.

   ## Motivation
   [From user description]

   ## Findings
   [Key results as described by user]

   ## Output Files
   [If known - paths to relevant outputs]

   ## Notes
   [Any additional context]
   ```

3. **Don't fabricate details:**
   - Only record what the user tells you or what's clearly documented
   - Mark uncertainties: "Exact parameters unknown"
   - Keep entries brief - the goal is context, not reconstruction

### Step 5: Update CLAUDE.md

Based on the sync conversation:

1. **Add new context:**
   - Current research questions/directions
   - Key recent findings
   - What's working now
   - Important methodological decisions

2. **Prune stale content:**
   - Remove findings superseded by newer work
   - Remove directions no longer being pursued
   - Remove context that's no longer relevant

3. **Keep it concise:**
   - CLAUDE.md should be scannable
   - Only include what affects ongoing work
   - The notebook has the full history

### Step 6: Commit Changes

```bash
git add notebook/ CLAUDE.md
git commit -m "notebook: sync retrospective entries and update project context"
```

## For New Projects (No Existing Notebook)

When encountering a project for the first time:

1. **Ask about existing write-ups:**
   > "Is there an existing write-up I can reference? This could be a Word doc, PDF, README, or any document describing the project goals, methods, or findings."

   If a write-up exists (e.g., `.docx`, `.pdf`, draft paper):
   - Read it using the appropriate skill (docx, pdf)
   - Use it to build an informed CLAUDE.md
   - Extract: research questions, key findings, methods used, current status

2. **Create notebook structure:**
   ```bash
   mkdir -p notebook/analyses notebook/data notebook/software notebook/methods
   ```

3. **Don't try to reconstruct full history:**
   - Ask about current state and recent important work
   - Create entries only for analyses the user describes
   - Focus on what's needed for going forward

4. **Initialize CLAUDE.md project section** (if not present):
   - Current research questions
   - Key context Claude needs to know
   - Recent important findings
   - (Use write-up content if available)

## Best Practices

1. **Ask, don't assume** - Don't guess what happened; ask the user
2. **Brief is better** - Retrospective entries should be concise
3. **Mark retrospective clearly** - Distinguish from real-time entries
4. **Focus on context** - Goal is to not be misled, not to have complete history
5. **Respect user's time** - Don't ask about every small change
