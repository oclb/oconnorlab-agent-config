# /disk-cleanup

Scan disk for large items, compare against previous inventory, and help free space via deletion or Dropbox online-only.

## When to Use

- Invoke manually with `/disk-cleanup` when you want to free disk space
- User-invoked only — never auto-trigger

## What It Does

1. Runs `ncdu` on `~` to scan disk usage
2. Shows available disk space and total usage
3. Clusters large items (>2GB) by project/purpose
4. Compares against previous inventory in `CLAUDE.md` to highlight **new or grown items**
5. Presents a table for the user to decide what to delete, move to Dropbox, or make online-only
6. Executes user's choices (delete, open in Finder for online-only)
7. Updates the inventory in `CLAUDE.md` for next time

## Prerequisites

- `ncdu` installed (`brew install ncdu`)
- Dropbox desktop app (for online-only via Finder)
- `dbxcli` at `~/bin/dbxcli` (for uploading non-Dropbox files to cloud)

## Limitations

- Making files "online-only" in Dropbox has no CLI — requires Finder right-click
- The skill opens folders in Finder and instructs the user what to right-click
