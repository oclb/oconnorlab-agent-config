# Notebook Archiving Criteria

Archiving moves entries from `notebook/INDEX.md` to `notebook/ARCHIVE.md`. Entry files stay in `notebook/entries/` — only the index row moves.

## When to Archive

- User explicitly requests it ("archive old entries", "clean up the index")
- INDEX.md exceeds ~100 entries
- Running the `documentation` maintenance workflow

## What to Archive (candidates — confirm with user)

- Resolved one-time issues (bug fixes, environment setup problems long since fixed)
- Entries superseded by later work
- Knowledge fully incorporated into code, CLAUDE.md, or other entries
- Exploratory work that didn't lead anywhere or became moot

## What NOT to Archive

- Active architectural decisions still shaping the project
- Recent entries (< 2 months old unless clearly moot)
- Entries referenced by the project's CLAUDE.md
- Entries frequently referenced by other non-archived entries

## How to Archive

1. Read INDEX.md and identify candidates
2. Present candidates to user with brief rationale for each
3. For approved entries, move their rows from INDEX.md to ARCHIVE.md
4. Also remove any stale references from the project's CLAUDE.md
5. Commit:
   ```bash
   git -C notebook add INDEX.md ARCHIVE.md && git -C notebook commit -m "archive: move N entries to archive"
   git -C notebook remote | grep -q origin && git -C notebook push
   ```

## ARCHIVE.md Format

Same format as INDEX.md:

```markdown
# Notebook Archive

| Date | Name | Summary |
|------|------|---------|
| 2025-11-03 | stale-socket-fix | One-time fix for SSH socket issue, resolved |
| 2025-10-15 | initial-pca-attempt | Superseded by batch-effect-analysis |
```

## Retrieval from Archive

At session start, only INDEX.md is read. If an Explore agent can't find relevant context in INDEX.md, it should also check ARCHIVE.md — archived entries are still searchable, just not loaded by default.
