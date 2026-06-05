---
name: work-cycle
description: Planning-centric workflow for substantial work across software, analyses, artifacts, documentation, and configuration. Invoke manually with /work-cycle when alignment matters; not for straightforward one- or two-edit requests.
recommended_scope: global
disable-model-invocation: true
---

# Work Cycle

Invoke this skill manually with `/work-cycle`. Do not rely on automatic trigger behavior.

This skill coordinates substantial work through a question-plan-implement-review cycle. It is domain-neutral at the top level; load the relevant domain reference before planning:

- Software work: always read `references/domains/software.md`.
- Analysis work: read `references/domains/analysis.md`.
- Artifact work: invoke `/artifacts` and its relevant subskill.

## Modes

The user may request one or more special modes, in which case read and follow the corresponding subprompt:

- *worktree:* read `references/modes/worktree.md`
- *afk:* read `references/modes/afk.md`
- *methods-first:* read `references/modes/methods-first.md`
- *grill-me* or *grillme:* read `references/modes/grill-me.md`

## Planning

**STOP: if you are not in Plan Mode and the user did not explicitly say AFK, immediately ask the user to switch you into Plan Mode with `Shift+Tab`, then wait.**

Most substantial work should go through a planning phase in which you go back and forth and align with the user. Planning must occur in Plan Mode unless AFK mode is explicit.

A shorthand request such as `todo 22`, an issue number, or a named skill is not by itself implementation approval and is not a substitute for Plan Mode.

During planning, loop at least once through:

1. Explore the current state as if implementing now; consult the notebook and web when relevant.
2. Engage with the user so you understand their intent and they understand the approach.
3. Draft or edit the plan.

Draft and edit plans in `notebook/plans/`. The plan should include:

1. *Goal:* user-specified goal.
2. *Approach:* high-level approach as discussed with user.
3. *Current state:* current behavior or artifact/data state, with names and line numbers when applicable.
4. *Changes:* concrete changes to make, with filenames when applicable.

After drafting the plan, assess whether a different approach would be simpler or better aligned with user intent; if so, ask an additional question and iterate.

## Implementation

Be open to issues with the plan or better approaches during implementation. If the plan is deeply flawed, stop and ask for user input; otherwise, use your best judgment and report afterward what changed from the plan and why.

Run relevant validation and iteratively debug. If the task is substantial, or if you are in worktree mode, trigger external review when complete and report findings to the user.

## Notebook

Create or update notebook entries for new analyses, code changes that alter interpretation of scientific results, codebase investigations, significant decisions, or substantial work whose context should survive the session. For entry creation after implementation, fork a subagent.

## Gotchas

- Failing to switch into Plan Mode and attempting a one-shot implementation.
- Failing to align with the user when the task seems straightforward.
- Treating the top-level skill as a substitute for domain-specific guidance.
