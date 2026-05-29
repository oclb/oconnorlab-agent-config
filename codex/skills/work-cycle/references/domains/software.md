# Software Domain

Always use this reference for software tasks. It preserves the behavior of the former `$software` skill; flag any intentional differences when reporting the work.

## Architecture

Partition the codebase into modules. Align with the user on the module partition, seams between modules, and which modules are core modules.

A highly modular codebase optimizes two competing objectives: (1) minimizing each module's responsibilities and (2) minimizing edges between modules. Avoid generic dumping grounds such as `utils.py`, which often have many responsibilities and many dependents.

## Exploration

Explore the codebase as if implementing now. Read relevant entry points, tests, configs, existing patterns, and notebook entries for related or parallel work. If the task adds a capability that might have an existing open-source solution, search the web and consider an external dependency or adapted fragment.

## Planning

When not in AFK mode, nontrivial changes that create new seams or alter install/setup behavior require Plan Mode and user alignment before implementation.

In the user dialogue, state the software shape plainly:

1. If a seam is created or changed, propose its specification.
2. If edges are modified, state how and why.
3. If core module internals change, state exactly how.
4. If public APIs, schemas, migrations, or compatibility behavior change, call that out.

Ask questions when user intent is unclear, the request has implications the user may not realize, or the straightforward approach differs from the robust approach. Most planning sessions should involve 1-5 questions.

## Implementation And Review

Prefer existing patterns, focused edits, meaningful tests, and iterative validation. Add abstractions only when they remove real complexity or match a local pattern.

If the plan proves flawed during implementation, stop for user input only when the flaw changes the intended behavior or architecture substantially. Otherwise, use best judgment and report deviations.

Run tests and iteratively debug. For substantial software work, or worktree mode, trigger an external review and report reviewer findings.

## Notebook

Software changes should include a notebook entry if they change the interpretation of scientific results obtained before versus after the change, capture a codebase investigation, or record a significant decision.
