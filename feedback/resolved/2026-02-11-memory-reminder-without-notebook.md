**User:** Luke Jen O'Connor

## Issue: Memory reminder fires even when notebook isn't set up

The UserPromptSubmit hook (memory-reminder.sh) always emitted the memory creation reminder, even in projects without a notebook. This caused Claude to attempt entry creation in projects that hadn't been initialized with `/init-project`.

### Resolution

- Updated memory-reminder.sh to check for `notebook/.git` existence before emitting the reminder
- Walks up from CWD to handle nested directory structures
- No output (and no wasted tokens) when notebook doesn't exist
