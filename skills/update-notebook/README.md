# Update Notebook Skill

Synchronizes the lab notebook when work has been done outside Claude Code, or onboards Claude to an existing project.

## Usage

```
/update-notebook
```

Or Claude may suggest it when detecting an unfamiliar project with existing work.

## What It Does

1. **Reviews git history** for recent changes, especially methodological updates
2. **Asks about unclear changes** that might be important
3. **Prompts for updates** on analyses, findings, and current research direction
4. **Creates retrospective entries** in the notebook for work done outside Claude
5. **Updates CLAUDE.md** with current context and prunes stale information

For new projects, also asks about existing write-ups (Word docs, PDFs, draft papers) to bootstrap CLAUDE.md with informed context.

## When to Use

- After working on a project outside Claude Code
- When starting Claude Code on an existing project
- When the notebook feels out of sync with actual project state
- Periodically to ensure CLAUDE.md reflects current priorities

## What It Won't Do

- Reconstruct complete project history
- Fabricate details about past analyses
- Create entries for every historical commit
- Ask about trivial changes (formatting, docs, CLI updates)

## Example

```
User: /update-notebook

Claude: Let me check what's changed recently...

I see several updates in the last week:
- New dependency: scikit-survival
- Changes to survival_analysis.py
- New results in results/km_plots/

Can you tell me what the scikit-survival package is being used for?

[User explains]

Have there been any analyses completed? What were the key findings?

[User describes analysis]

I'll create a retrospective notebook entry for that analysis and update CLAUDE.md with the current research direction.
```

## Related Skills

- **perform-analysis**: Creates notebook entries in real-time during analysis
- **new-data**: Validates datasets and writes to `notebook/data/`
- **new-software**: Sets up tools and writes to `notebook/software/`
