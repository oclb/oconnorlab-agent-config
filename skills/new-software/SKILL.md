---
name: new-software
description: This skill should be used when the user asks to "learn", "explore", "try out", "get started with", "set up", or "install and test" a new tool, library, framework, CLI, package, or technology.
version: 1.0.0
---

# Learn New Tool Skill

Quickly learn and set up new tools, libraries, and frameworks by finding documentation, installing them, and verifying they work correctly.

## Notebook Integration

This skill writes to `notebook/entries/` to track external tools used in the project.

**Before starting:** Check if software is already documented:
```bash
ls notebook/entries/ 2>/dev/null | grep -i "<tool-name>"
```

If an entry exists, read it to see what's already known. You may just need to update version info or add new issues.

## AFK Mode Behavior

If `(afk)` appears in the user's message, apply autonomous mode for this turn:
- Auto-select recommended installation method without asking (prefer official/standard approach)
- Proceed with installation without "do you want me to proceed?" prompts
- Respect current tool permissions (sandbox settings) - only use allowed tools
- Attempt autonomous troubleshooting on errors (max 2 attempts), then stop and report
- Document installation choices and any issues encountered
- Only pause for: requires credentials/API keys, conflicts with existing tools, needs sudo

**Without AFK mode (default):**
- Confirm installation method if multiple valid options exist
- Ask before global vs. local installation if unclear
- Confirm before making system-wide changes

## When This Skill Applies

Use when the user wants to:
- Learn a new tool, library, or framework
- Set up and test a new technology
- Explore what a tool does and how to use it
- Get started with a new package or CLI
- Verify a tool is properly installed and working

## Process

### 1. Search for Documentation

**Use WebSearch to find:**
- Official documentation website
- Installation instructions
- Quick start / getting started guides
- GitHub repository
- Common use cases and examples

**Example searches:**
- "[tool name] official documentation 2026"
- "[tool name] getting started guide"
- "[tool name] installation"

### 2. Install the Tool

**Check first:** `which [command]` or `[command] --version`

**Package managers:**

| Language | Command |
|----------|---------|
| Node.js | `npm install [-g] [package]` |
| Python | `pip install [package]` |
| Ruby | `gem install [package]` |
| Rust | `cargo install [package]` |
| Go | `go install [package]@latest` |
| macOS | `brew install [package]` |
| Linux | `apt-get install [package]` |

**After installing:**
1. Verify: `which [command]` and `[command] --version`
2. Note installed version
3. Check for post-install setup (PATH, config files)

### 3. Run Sanity Checks

**For CLI tools:**
```bash
[tool] --help
[tool] --version
# Try simple command from docs
```

**For libraries:**
```python
# Python
python3 -c "import [lib]; print([lib].__version__)"
```
```javascript
// Node.js
node -e "const x = require('[lib]'); console.log(x)"
```

**For frameworks:**
- Create minimal "Hello World" project
- Follow official quick start
- Run dev server or build process

### 4. Provide Summary and Next Steps

```
✓ Installed: [tool] v[version]
✓ Location: [path]
✓ Sanity check: Passed

Basic Usage:
- [example 1]
- [example 2]
- [example 3]

Key Documentation:
- [Link to docs]
- [Link to API reference]

Next Steps:
- [Suggestion 1]
- [Suggestion 2]
```

**In AFK mode:** Include a "Choices Made" section documenting installation decisions.

### 5. Write to Notebook

**Create or update `notebook/entries/YYYY-MM-DD-<tool-name>.md`:**

```markdown
# <Tool Name>

**Date:** YYYY-MM-DD

## Summary
[One sentence: what this tool is and why we're using it]

## Details
- **Version:** X.Y.Z
- **Installation:** [pip/npm/brew/etc.]
- **Official docs:** [URL]
- **Primary use:** [e.g., "differential expression analysis"]

## Notes
[Any bugs, gotchas, or limitations discovered]
- [e.g., "Requires R >= 4.0"]
- [e.g., "Memory-intensive for >50k genes"]
- [e.g., "Output format changed in v2.0"]

## References
[Related entries if applicable]
```

**Important:**
- Do NOT replicate documentation (it goes stale)
- DO record issues/bugs/limitations you encounter
- Link to specific docs sections relevant to your usage

**Update notebook/INDEX.md:**
Add a row:
```
| YYYY-MM-DD | <tool-name> | <tool> v<version>: <primary use> |
```

**Announce and commit:**
State "Created notebook entry: `<tool-name>`"

```bash
mkdir -p notebook/entries
git -C notebook add entries/ INDEX.md
git -C notebook commit -m "entry: <tool-name> setup"
git -C notebook remote | grep -q origin && git -C notebook push
```

### 6. Offer Project Skill

After setup is complete, consider whether a project-specific skill for this tool would be useful. Identify what knowledge you've gathered that would be valuable in future sessions — then offer it to the user with specifics.

**What makes a tool worth a skill:**
- **Project-specific workflows** — the tool is used in a particular way in this project that differs from its general usage
- **Non-obvious configuration or invocation patterns** — flags, environment variables, or setup steps that would need to be re-discovered each session
- **Integration gotchas** — how the tool interacts with other project tools or data in ways that aren't obvious from its documentation

**When a skill is NOT needed:**
- You already know how to use the tool well from training data
- The tool is used in a standard way and its docs are sufficient
- The project revolves around this one tool — the project CLAUDE.md is the right place for that
- The notebook entry already captures everything relevant

**How to offer:** Tell the user what specific information you would put in the skill and why it would be useful. For example:

> "bcftools in this project always needs the `--regions-overlap 2` flag because your BED files use half-open coordinates. I could create a short skill capturing that default, the reference genome path, and the filtering presets you use. Want me to?"

Skills can be short — even a paragraph or two of context is valuable if it saves re-discovery. There is little cost to creating one. Use `/skill-creator` if the user agrees.

## Best Practices

1. **Search for current docs** - Tools change; verify latest information
2. **Prefer official sources** - Official docs, GitHub, package registries
3. **Global vs. local:** Global for CLI tools, local for project dependencies
4. **Check before installing** - Don't reinstall if already present
5. **Test minimally but thoroughly** - Confirm it works, don't overcomplicate
6. **Handle errors gracefully** - Search for common issues, try alternatives
7. **Always create a notebook entry** - New tool setups warrant memory creation; this skill handles it in Step 5

## Special Cases

### Web Services/APIs
- Find API documentation
- Show how to get API keys
- Provide authentication examples
- Demo basic API call with curl

### IDE Extensions
- Identify user's editor first
- Provide editor-specific installation
- Show configuration
- Demo basic usage

### Docker/Containers
- Verify Docker installed
- Pull container image
- Show how to run
- Provide docker-compose if applicable

### Version Managers (nvm, pyenv, rbenv)
- Install version manager
- Show how to install language versions
- Demo switching versions
- Configure shell integration

## Integration with Other Skills

- **perform-analysis**: Invokes this skill when needing unfamiliar tools
- **sanity-check-data**: Invokes this skill when data requires specialized tools
