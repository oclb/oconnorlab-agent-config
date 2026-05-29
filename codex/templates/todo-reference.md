# To-Do System Reference

## File Formats

### notebook/TODO.md (active tasks)

~~~markdown
# To-Do

Next ID: 3

- [ ] #1 **Task name** - Brief description
  - Context: `notebook/entries/related-entry` (if applicable)
  - Added: YYYY-MM-DD

- [ ] #2 **Another task** - Description
  - Added: YYYY-MM-DD
~~~

### notebook/DONE.md (completed tasks)

~~~markdown
# Completed

- [x] #0 **Example task** - Original description preserved
  - Context: `notebook/entries/related-entry` (if it had one)
  - Added: YYYY-MM-DD
  - Completed: YYYY-MM-DD
  - Result: `notebook/entries/resulting-entry`
~~~

## Operations

Todo completion is usually handled automatically by the memory agent when work results in a notebook entry. The manual completion flow below is a fallback.

### Adding a todo
1. Read `Next ID:` counter from TODO.md, use that number, then increment the counter
2. If the todo arises from a notebook entry, add a `Context:` line linking to it
3. Commit:
   ```bash
   git -C notebook add TODO.md && git -C notebook commit -m "todo: add #N - <task name>"
   git -C notebook remote | grep -q origin && git -C notebook push
   ```

### Completing a todo (manual fallback)
1. Read TODO.md to find the item; if it has a `Context:` link, read that entry for background
2. Make a plan and execute the work
3. Move the entire item to DONE.md, preserving all original fields, adding:
   - `Completed:` date
   - `Result:` link if the work created a notebook entry
4. Commit:
   ```bash
   git -C notebook add TODO.md DONE.md && git -C notebook commit -m "todo: complete #N - <task name>"
   git -C notebook remote | grep -q origin && git -C notebook push
   ```

### Editing a todo
1. Update the description or context as needed
2. Commit:
   ```bash
   git -C notebook add TODO.md && git -C notebook commit -m "todo: update #N - <brief change>"
   git -C notebook remote | grep -q origin && git -C notebook push
   ```

### Deleting a todo (without completing)
1. Remove the item entirely (don't move to DONE.md)
2. Commit:
   ```bash
   git -C notebook add TODO.md && git -C notebook commit -m "todo: remove #N - <reason>"
   git -C notebook remote | grep -q origin && git -C notebook push
   ```

### Integration with Notebook

Most completed todos should result in a notebook entry. The `Result:` link in DONE.md connects the task to its entry for traceability.
