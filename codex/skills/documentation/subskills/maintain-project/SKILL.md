---
name: maintain-project
description: Documentation subskill for comprehensive project maintenance audits. Use through $documentation when the user asks to "maintain", "audit", "clean up", "health check", or review overall project state. Checks AGENTS.md freshness, notebook index, TODO list, git state, test suite, and compiles actionable suggestions.
---

# Maintain Project

Run a comprehensive maintenance audit of the project. Use the task plan tool to define audit tasks, then execute them (parallelizing where possible), and compile findings into a concise report with questions for the user.

## Setup

1. Read the project's `AGENTS.md` and `notebook/INDEX.md`
2. Create all audit tasks below using the task plan tool
3. Work through tasks, marking in_progress/completed
4. Compile findings into the summary report (Task 10)

## Audit Tasks

Create all 10 tasks at once. Tasks 1-9 are independent and can run in any order. Task 10 depends on all others.

### Task 1: Audit AGENTS.md vs Codebase

Check every claim in the project's `AGENTS.md` against the actual codebase:

- **File references** — do referenced files/directories exist?
- **Described behavior** — does code still work as described?
- **Entry references** — do `(see \`entry-name\`)` references point to entries that still exist in INDEX.md?
- **Stale context** — anything describing old state that no longer applies?

**Action:** Fix factual inaccuracies directly. Flag subjective/uncertain items for Task 10.

### Task 2: Archive Notebook Index

Review `notebook/INDEX.md` and identify entries to archive.

Read `references/archiving.md` for detailed criteria on what to archive vs keep.

**Action:** Present archive candidates to user with brief rationale. Move approved entries from INDEX.md to ARCHIVE.md. Remove any corresponding stale references from AGENTS.md.

### Task 3: Audit AGENTS.md Length

Assess whether the project's root `AGENTS.md` is too long or contains content better placed elsewhere:

- Count approximate lines/tokens
- Identify sections that are **subdir-specific** (only relevant to one directory) — these belong in a `<subdir>/AGENTS.md`
- Identify sections that are **stale** or duplicating what's obvious from code
- Identify sections that are **too detailed** for a quick-reference context file

**Guideline:** AGENTS.md should be scannable. If it takes >30 seconds to read, it's too long. Prefer links to notebook entries over inline explanations.

**Action:** Draft proposed changes (splits, deletions, moves) for user approval.

### Task 4: Audit Subdir AGENTS.md Files

Find all `**/AGENTS.md` files in the project (excluding `notebook/`).

For each:
- Is the content still accurate?
- Does it contradict the root AGENTS.md?
- Is it stale or redundant?

**Action:** Fix inaccuracies directly. Flag contradictions for Task 10.

### Task 5: Check Commit History vs Notebook

```bash
git log --oneline -50
```

Compare recent commits against notebook/INDEX.md. Look for:
- Major features or refactors with no corresponding notebook entry
- Significant debugging sessions (multi-commit fix sequences) not documented
- New data, tools, or dependencies added without an entry

**Action:** For significant gaps, create brief retrospective notebook entries and mark them clearly as retrospective. Review recent history, ask only about significant unclear changes, and record what is known without trying to reconstruct every detail. For minor gaps, note them for Task 10.

### Task 6: Git Cleanliness

Check:
```bash
git status        # untracked/modified files
git stash list    # forgotten stashes
git branch        # stale branches
gh pr list --state open  # open pull requests
```

Look for:
- Open PRs that have been merged, closed, or forgotten
- Untracked files that should be gitignored or committed
- Modified files that look like abandoned work
- Stale branches (merged or very old)
- Forgotten stashes

**Action:** Do NOT clean anything up automatically. If git is messy, describe what's found and ask the user whether they want to clean up (in Task 10).

### Task 7: Audit TODO List

Read `notebook/TODO.md` and `notebook/DONE.md`.

For each open TODO:
- Check if the work has already been done (search notebook entries and recent commits)
- Check if the item is stale (>2 months old with no progress)
- Check if it's still relevant given current project state

**Action:** Mark completed items as done (move to DONE.md with `Result:` link if applicable). Flag stale items for user review in Task 10.

### Task 8: Run Test Suite

Detect and run the project's test suite:
- Look for `pytest`, `npm test`, `cargo test`, `make test`, or similar
- If no test suite exists, note this and skip

**Action:** Flag failures with file:line and brief description. Do NOT attempt fixes. If no tests exist, note it for Task 10.

### Task 9: Test Coverage Audit

Compare the test suite against the codebase:

- **Undertested:** Core logic, complex functions, or critical paths with no tests
- **Overtested:** Trivial code with extensive tests, or tests that just test the framework

Use heuristics:
- Functions >20 lines with no test coverage → likely undertested
- Test files with no corresponding source changes in recent commits → possibly stale tests
- Directories with source files but no test files → coverage gap

**Action:** Compile a brief coverage assessment for Task 10. Do NOT write tests.

### Task 10: Compile Report

After all other tasks complete, synthesize findings into a report for the user.

**Format:**

```markdown
## Maintenance Report

### Changes Made
- [list of things fixed directly]

### Needs Your Input
- [archiving candidates from Task 2]
- [AGENTS.md restructuring proposals from Task 3]
- [git cleanup from Task 6]
- [stale TODOs from Task 7]

### Test Status
- [failures from Task 8]
- [coverage gaps from Task 9]

### Questions
- [only ask if: significant, <80% sure of answer, user likely has a preference]
```

**Question filtering:** For each potential question, apply ALL three criteria:
1. Is this significant enough to warrant user attention?
2. Am I less than 80% confident in the best answer?
3. Does the user likely have a preference (vs. a clear best practice)?

Only include questions that pass all three. Aim for 0-5 questions. Zero is fine if nothing qualifies.
