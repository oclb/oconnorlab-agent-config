---
name: disk-cleanup
description: Disk space cleanup tool. Invoke manually with /disk-cleanup to scan the home directory for large items (>2GB), compare against previous inventory, cluster by project/purpose, and help free space via deletion or Dropbox online-only. User-invoked only - never auto-trigger.
---

# /disk-cleanup Skill

Scan disk for large items, compare against previous inventory, and help the user free space.

## Overview

This skill scans the home directory with `ncdu`, identifies items >2GB, clusters them by project/purpose, compares against the previous inventory stored in the project's `CLAUDE.md`, and helps the user free space via deletion or Dropbox online-only.

**Trigger**: Manual only — user invokes `/disk-cleanup`
**Project**: Must be run from `~/Dropbox/GitHub/tools/` (or wherever the tools project lives)

## Execution Flow

### Phase 1: Scan Disk

**1.1 Check disk space**

```bash
df -h /System/Volumes/Data | tail -1
```

Report: total, used, free, and percent used.

**1.2 Run ncdu scan**

```bash
ncdu -o /tmp/ncdu_home.json -x ~ 2>/dev/null
```

The `-x` flag stays on the same filesystem (avoids scanning network mounts). The `-o-` flag outputs JSON.

This may take 1-2 minutes. Use a 5-minute timeout.

**1.3 Parse results**

Write a Python script to `/tmp/parse_ncdu.py` that:

1. Loads the ncdu JSON (format: `[version_int, version_int, metadata_dict, root_array]`)
2. The root array is `[dir_info_dict, child1, child2, ...]` where each child is either a `{file_dict}` or `[dir_array]`
3. Recursively finds items >= 2GB
4. For directories >= 2GB, drills into children to find the most specific large items
5. Reports both "large items drilled down" and "notable items (1-2GB)"
6. Uses `asize` field for actual size

Key parsing details:
- `get_size(node)`: recursively sums `asize` for dicts, recurses into lists
- `get_name(node)`: returns `name` from dict or first element of list
- A directory node is a list; a file node is a dict

### Phase 2: Cluster and Present

**2.1 Read previous inventory**

Read the `## Disk Space Inventory` section from the project's `CLAUDE.md`. Parse the table to get previous item names and sizes.

**2.2 Cluster items**

Group items by project or purpose. Use judgment to create meaningful clusters:
- Items in the same Git repository → one cluster
- Items in the same parent directory serving the same purpose → one cluster
- System/cache items → group by type (browser caches, package manager caches, etc.)
- Standalone large files → list individually

Each cluster should be mutually non-overlapping (no double-counting).

**2.3 Compare with previous inventory**

For each cluster:
- If it existed before with similar size: mark as **unchanged**
- If it existed before but grew significantly (>20%): mark as **grown** with delta
- If it's new (not in previous inventory): mark as **new**

**2.4 Present results**

Display a table with all items >2GB, organized by cluster:

```
## Disk Space Report — YYYY-MM-DD

**Disk**: X GB total | Y GB used | Z GB free (N%)
**Home directory**: W GB

### Large Items by Category (>2GB, clustered)

| # | Category | Size | Location | Status |
|---|----------|------|----------|--------|
| 1 | Project Name | XX GB | ~/path/ | unchanged |
| | sub-item | X GB | | |
| 2 | New Thing | XX GB | ~/path/ | **NEW** |
...
```

**2.5 New/grown items section**

At the bottom, list new and grown items separately under their own heading:

```
### New or Growing Items

These items were not present (or were smaller) in the previous scan:

| Category | Size | Change | Location |
|----------|------|--------|----------|
| ... | | +X GB | |
```

If there are no new/grown items, state that.

### Phase 3: User Decisions

Wait for the user to tell you what to do. They may say things like:
- "Delete X" → `rm -rf` (confirm size first with `du -sh`)
- "X → online only" → open the folder/file in Finder for the user to right-click
- "Move X to Dropbox" → `mv` to `~/Dropbox/` then user makes online-only
- "Skip" or "done" → proceed to Phase 4

**For deletions:**
```bash
du -sh /path/to/item  # confirm size
rm -rf /path/to/item  # delete
```

**For online-only (items already in Dropbox):**
```bash
# For a single file:
open -R /path/to/file  # reveal in Finder

# For a directory's contents:
open /path/to/directory/  # open in Finder
```

Then instruct user: "Select all → right-click → Make online-only"

**For moving to Dropbox + online-only (items outside Dropbox):**
```bash
# Decide on a reasonable destination in Dropbox
mkdir -p ~/Dropbox/archive/path/
mv /source/path ~/Dropbox/archive/path/
# Then open in Finder for online-only
open ~/Dropbox/archive/path/
```

### Phase 4: Record and Verify

**4.1 Verify space reclaimed**

```bash
df -h /System/Volumes/Data | tail -1
```

Report new free space vs. starting free space.

**4.2 Update CLAUDE.md inventory**

Update the `## Disk Space Inventory` section in the project's `CLAUDE.md`:

1. Update the `_Last updated:` date
2. Update the disk usage line
3. Update the table with current sizes
4. Add an entry under `### Actions Taken (YYYY-MM-DD)` listing what was deleted/moved/made online-only
5. Items that were made online-only should be noted in the table (e.g., "made online-only" in Notes column)

The previous inventory table becomes the baseline for next time's comparison. Keep it as `### Large Items (>2GB) — Baseline`.

**4.3 Create notebook entry**

If significant cleanup was done (>10GB freed), create a notebook entry documenting:
- Starting and ending disk space
- What was deleted/moved/made online-only
- Any notable findings

## Important Notes

- **No CLI for Dropbox online-only**: There is no command-line way to make Dropbox files online-only. The Dropbox daemon handles eviction internally via Finder right-click only. The skill opens folders in Finder and instructs the user.
- **Check online-only status**: `xattr -p com.dropbox.placeholder /path` returns a value if the file is online-only (0 bytes on disk), or errors if it's local.
- **Dropbox folder**: `~/Dropbox` (legacy mode, not File Provider)
- **dbxcli**: Available at `~/bin/dbxcli` for uploading files to Dropbox cloud. Useful for files outside `~/Dropbox/` that the user wants backed up before deletion.
- **ncdu JSON**: Save to `/tmp/ncdu_home.json` to avoid output truncation issues.
- **APFS snapshots**: Deletions may not immediately show in `df` output due to APFS snapshot retention. This is normal.
