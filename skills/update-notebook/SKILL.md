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
- `methods/` - Everything else: features, bugfixes, datasets, tools, decisions

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

### Step 5: Update INDEX.md and CLAUDE.md

**Update notebook/INDEX.md:**
For each retrospective entry created, add a row to the appropriate table (Analyses or Methods).

**Update CLAUDE.md:**
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

### Opening Message

Start with a welcoming introduction:

> "I can see this is the first time using Claude Code in this project. I'll walk you through setting up a lab notebook to track your work. This will help me understand your project and maintain context across sessions.
>
> I'll ask you some questions about the project, then create documentation in a `notebook/` directory. Let's start!"

### Step 1: Gather Project Context

1. **Ask about existing write-ups:**
   > "Is there an existing write-up I can reference? This could be a Word doc, PDF, README, or any document describing the project goals, methods, or findings."

   If a write-up exists (e.g., `.docx`, `.pdf`, draft paper):
   - Read it using the appropriate skill (docx, pdf)
   - Use it to build an informed CLAUDE.md
   - Extract: research questions, key findings, methods used, current status

2. **Ask clarifying questions - don't guess:**
   - If you encounter acronyms, project names, or terminology you're unsure about, **ask the user**
   - Don't guess what something means - wrong guesses are worse than asking
   - Example: "What does 'KNEE' stand for in this project?"

### Step 2: Create Notebook Structure

```bash
mkdir -p notebook/analyses notebook/methods
```

**Initialize INDEX.md:**
```markdown
# Notebook Index

## Analyses
| ID | Summary | Date | Tags |
|----|---------|------|------|

## Methods
| Date | Type | Summary |
|------|------|---------|
```

### Step 3: Document Datasets and Tools

**Prompt the user about datasets:**
> "What datasets are you using in this project? For each one, I'd like to know:
> - Where is it located?
> - Where did it come from (downloaded, generated, provided)?
> - Any known issues or quirks?"

**Prompt the user about specialized software:**
> "Are there any specialized tools, libraries, or software specific to this project that I should know about? (I don't need to know about standard tools like Python, PyTorch, pandas, etc. - just project-specific or unusual dependencies.)"

For each dataset or tool mentioned, create a methods entry `notebook/methods/YYYY-MM-DD-<name>.md` with:
- `Type: data` or `Type: tool`
- Location and source (for data) or version and docs URL (for tools)
- Any issues or gotchas mentioned by user

**Skip tool documentation for:**
- Standard, well-known tools (Python, R, PyTorch, TensorFlow, pandas, numpy, etc.)
- Common CLI tools
- Anything Claude would know from training

### Step 4: Document Analyses

**Ask about completed analyses:**
> "Have there been any analyses or experiments completed in this project? What were the key findings?"

Create retrospective entries for analyses the user describes.

### Step 5: Initialize CLAUDE.md

Based on all gathered information:
- Current research questions
- Key context Claude needs to know
- Available datasets and their locations
- Any specialized tools being used
- Recent important findings

**Remember:** Only include what you learned from the user or documents. Don't invent details.

## Best Practices

1. **Ask, don't guess** - If you're unsure about anything (acronyms, terminology, what something means), ask the user. Wrong guesses create confusion.
2. **Brief is better** - Retrospective entries should be concise
3. **Mark retrospective clearly** - Distinguish from real-time entries
4. **Focus on context** - Goal is to not be misled, not to have complete history
5. **Respect user's time** - Don't ask about every small change
6. **Document datasets and tools** - These are as important as analyses for project context
