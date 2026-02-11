**User:** Luke Jen O'Connor

## Issue: Not clear to users that they can customize settings and CLAUDE.md

Users didn't realize they could personalize their setup via `~/.claude/CLAUDE.md` (adding instructions before/after the @import) and `~/.claude/settings.local.json` (personal permissions and preferences).

### Resolution

- Added "Customization" subsection to global/CLAUDE.md near the top, explaining both files
- Improved setup.sh completion message with explicit customization instructions
