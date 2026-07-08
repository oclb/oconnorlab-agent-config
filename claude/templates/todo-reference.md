# To-Do System Reference

Todos live in `notebook/todos.json` and are managed with the `todo` script. Do not edit `todos.json` by hand; every mutating command rewrites the file and commits (and pushes, when an origin remote exists) in the notebook repo in a single call.

## Invocation

```bash
TODO="${CLAUDE_HOME:-$HOME/.claude}/bin/todo"
```

Run it from anywhere inside the project; it discovers `notebook/` by walking up from the working directory. Pass `--notebook <path>` only when discovery cannot work.

## Operations

1. Add: `"$TODO" add "Title" [--description "..."] [--context notebook/entries/<slug>]` — prints the new id. Use `--context` when the todo arises from a notebook entry.
2. List: `"$TODO" list` (active, plus the next id), `"$TODO" list --done`, `"$TODO" list --all`, `--json` for raw data.
3. Show: `"$TODO" show <id>`
4. Complete: `"$TODO" complete <id> [--result notebook/entries/<slug>]` — most completed todos should result in a notebook entry; the `--result` link connects the task to its entry for traceability.
5. Edit: `"$TODO" edit <id> [--title ...] [--description ...] [--context ...] [--result ...]`
6. Delete without completing: `"$TODO" delete <id> [--reason "..."]`

Todo completion is usually handled by the agent creating or updating the notebook entry when work results in one.

## Data Format

`notebook/todos.json` holds `next_id` plus `active` and `done` arrays. Each todo has `id`, `title`, `description`, optional `context`, and `added`; done todos also have `completed` and optional `result`. Ids are never reused.

## Legacy Notebooks

Notebooks predating this system use `TODO.md`/`DONE.md`. Run `"$TODO" migrate` to convert one; mutating commands also auto-migrate on first use. Read-only commands (`list`, `show`) work on legacy notebooks without converting them.
