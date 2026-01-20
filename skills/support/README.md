# Help Skill

Provides help and documentation about Claude Code and this configuration's capabilities for scientific research.

## Usage

Invoke with `/support` or ask questions like:
- "What can you do?"
- "What skills are available?"
- "How do I use hooks?"
- "Help me get started with Claude Code"

## What It Does

1. **Fetches Claude Code Documentation**: Clones/updates the [claude-code-docs](https://github.com/ericbuess/claude-code-docs) repository to `/tmp/claude/claude-code-docs` for searchable access to official documentation.

2. **Reads Config Repository**: Loads this repository's README for skill-specific information.

3. **Answers Queries**: Either answers specific questions about Claude Code features or provides a comprehensive overview for scientific research users.

## Overview Topics

When providing a general overview, covers:
- What Claude Code is and how it differs from chat interfaces
- Available skills with descriptions
- Running on O2 cluster (setup, sandbox mode, TMPDIR)
- AFK mode for autonomous operation
- How to provide feedback and contribute

## Documentation Sources

- **Official docs**: https://github.com/ericbuess/claude-code-docs (auto-updated every 3 hours)
- **Config repo**: Found by resolving the `~/.claude/CLAUDE.md` symlink
