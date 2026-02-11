**User:** Luke Jen O'Connor

## Issue: Notebook entries created too often

The memory agent was triggered too aggressively, creating notebook entries for minor work like discussions, quick fixes, and routine operations. The "tangible contribution" criteria were too broad (included things like "expressing an idea" and "giving a new name").

### Resolution

- Raised the bar in global/CLAUDE.md: entries now only for multi-step analyses, significant implementations, non-obvious discoveries, or tool setup with gotchas
- Added explicit "Do NOT create entries for" list: discussions, minor fixes, routine operations, planning without action
- Updated memory-reminder hook text to match the higher threshold
