**User:** Luke Jen O'Connor

## Issue: TODO numbering collisions

When adding new TODO items, I assigned numbers that had already been used in DONE.md (specifically #51 and #52). This created confusion where completed tasks and new tasks shared the same ID.

### Root cause

The current system relies on manually checking both TODO.md and DONE.md to find the next available number. This is error-prone, especially when adding multiple items quickly.

### Suggestions for improvement

1. **Single source of truth**: Maintain a counter in a separate file (e.g., `notebook/.todo-counter`) that gets incremented for each new task, regardless of whether it goes to TODO or DONE.

2. **Automatic assignment**: When creating tasks, don't manually assign numbers. Instead, read the counter, increment it, and use that value.

3. **Validation on commit**: Add a pre-commit hook or CI check that verifies no duplicate task IDs exist across TODO.md and DONE.md.

4. **Alternative: Use dates or UUIDs**: Instead of incrementing integers, use date-based IDs (2026-02-05-a, 2026-02-05-b) or short UUIDs that are guaranteed unique.

The integer IDs are nice for human reference ("work on #14") but require careful coordination that's easy to mess up.
