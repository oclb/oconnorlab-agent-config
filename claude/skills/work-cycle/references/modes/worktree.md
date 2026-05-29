# Worktree Mode

Use worktree mode when the user requests it for repository work: plan, implement in a separate worktree, open a PR, and review the PR.

While planning, check `git status --porcelain` in the current repo and identify dirty files that might overlap with the change. If overlapping local changes make the worktree unsafe, ask the user; otherwise preserve unrelated changes and continue.

After the plan is approved:
1. implement in a dedicated git worktree, making commits as you go
2. test and iterate
3. Make a PR
4. Run fresh-context review with two reviewers: one for correctness and one for regressions and test coverage
5. Message the user with a high signal-to-noise report containing deviations from plan and reviewer findings
