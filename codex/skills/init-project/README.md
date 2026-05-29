# init-project

Initialize a project for Codex-assisted research work.

After installing this config from the config repo, initialize normal projects with:

```text
init project
```

The `set-me-up` workflow is repo-local to `codex-config` and is not symlinked into downstream projects.

## What It Does

1. Checks git status and remotes.
2. Offers to create a GitHub remote when missing.
3. Creates `notebook/{entries,feedback,plans}/`.
4. Initializes `notebook/` as a separate git repository.
5. Adds `notebook/` to the main repo `.gitignore`.
6. Creates or updates project-local `AGENTS.md`.
7. Offers an optional private notebook remote.
8. Points users to next workflows such as manuscript checks through `artifacts`, `remind-resume`, `documentation`, and optional O2 setup through the internal `setup-o2` subskill.

## Subskills

- `subskills/setup-o2/`: O2/SLURM setup and remote-bridge access guidance for projects that need cluster compute.

## Why Separate Notebook Repo?

The notebook grows as analyses, plans, and decisions accumulate. Keeping it as a separate repo:

- keeps the main project history focused on source/data artifacts
- allows different backup and sharing rules
- preserves reproducibility notes without bloating the main repo
- gives Codex a stable place to search when resuming work

## Idempotence

The workflow is intended to be safe to run repeatedly. Existing `AGENTS.md`, `.gitignore`, and notebook files should be updated conservatively rather than replaced.
