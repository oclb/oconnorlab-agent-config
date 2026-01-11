# Testing the Learn Tool Skill

## Setup

Before testing, ensure the plugin is installed:

1. The plugin files are in `~/Dropbox/GitHub/claude-config/plugins/learn-tool/`
2. Your `settings.json` includes the pluginDirs setting
3. Restart your Claude Code session (start a new conversation)

## How to Test

The skill activates automatically when you use trigger phrases. Here are some test cases:

### Test 1: Learn a Simple CLI Tool

**Prompt**: "Help me learn jq"

**Expected behavior:**
- Claude searches for jq documentation
- Explains what jq is (JSON processor)
- Installs jq via brew (or appropriate package manager)
- Runs `jq --version` to verify
- Shows simple jq examples (parsing JSON)
- Provides links to documentation
- Suggests next steps

### Test 2: Learn a Python Library

**Prompt**: "I want to explore the requests library for Python"

**Expected behavior:**
- Claude finds requests documentation
- Checks if Python/pip is installed
- Installs requests via pip
- Creates a simple test script
- Makes a sample HTTP GET request
- Shows common usage patterns
- Links to docs.python-requests.org
- Suggests API testing next steps

### Test 3: Learn a JavaScript Package

**Prompt**: "Set up chalk and show me how it works"

**Expected behavior:**
- Claude searches for chalk (terminal colors) docs
- Checks for Node.js/npm
- Installs chalk locally or globally
- Creates a test script with colored output
- Runs the script to demonstrate
- Shows different color/style options
- Provides npm link
- Suggests use cases

### Test 4: Learn a Framework

**Prompt**: "Get me started with FastAPI"

**Expected behavior:**
- Claude finds FastAPI documentation
- Explains it's a Python web framework
- Installs FastAPI and uvicorn
- Creates a minimal "Hello World" API
- Runs the dev server
- Makes a test request
- Shows how to add endpoints
- Links to FastAPI docs
- Suggests building a simple API

### Test 5: Learn a System Utility

**Prompt**: "Try out ripgrep and help me understand it"

**Expected behavior:**
- Claude searches for ripgrep info
- Explains it's a fast grep alternative
- Installs via brew/cargo
- Runs version check
- Performs test searches in current directory
- Shows common flags and options
- Compares to traditional grep
- Links to GitHub
- Suggests using it for code search

## Verification Checklist

For each test, verify that Claude:

- [ ] Searches online for current (2026) documentation
- [ ] Finds official sources (docs, GitHub, package registries)
- [ ] Checks if tool is already installed before attempting
- [ ] Uses the correct package manager for your system
- [ ] Verifies installation with version check
- [ ] Runs appropriate sanity tests
- [ ] Provides 2-3 practical examples
- [ ] Links to official documentation
- [ ] Suggests concrete next steps

## Common Issues

### Skill Not Activating

**Symptoms**: Claude doesn't follow the learn-tool process, just gives generic advice

**Causes:**
- Plugin not loaded (check `settings.json` has correct `pluginDirs`)
- Session started before plugin was installed (restart needed)
- Trigger phrase not used (try "learn", "explore", "set up")

**Solutions:**
- Verify plugin files exist at correct path
- Restart Claude Code session
- Use explicit trigger phrases

### Installation Failures

**Symptoms**: Tool fails to install

**Expected behavior:**
- Claude should catch the error
- Search for common installation issues
- Suggest alternative installation methods
- Provide manual installation steps

**Test**: Try installing a non-existent package to see error handling

### Wrong Package Manager

**Symptoms**: Claude tries wrong installation method

**Expected behavior:**
- Should detect OS and available package managers
- Use appropriate method (brew on macOS, apt on Linux, etc.)
- Fall back to alternative if primary fails

## Advanced Tests

### Test Offline Behavior

Disable network temporarily - Claude should:
- Attempt to use cached knowledge
- Explain it can't search for latest docs
- Still attempt installation if package manager works offline

### Test Already Installed Tools

**Prompt**: "Learn git"

**Expected behavior:**
- Check if git exists (`which git`)
- Report it's already installed with version
- Skip installation
- Still provide usage examples and docs
- Show advanced features or tips

### Test Multiple Tools

**Prompt**: "Help me learn both prettier and eslint"

**Expected behavior:**
- Process both tools
- Install both
- Show how they work together (code formatting + linting)
- Provide configuration examples

## Success Criteria

The skill is working correctly if:

1. **Automatic activation** - No need to explicitly mention "use the learn-tool skill"
2. **Comprehensive process** - Follows all 4 steps (search, install, test, summarize)
3. **Current information** - Finds 2026 documentation
4. **Practical examples** - Provides runnable code/commands
5. **Error handling** - Gracefully handles failures
6. **Helpful summary** - Clear next steps and resources

## Example Session Output

Here's what a successful test should look like:

```
You: Help me learn jq

Claude: I'll help you learn jq, a lightweight and flexible command-line JSON processor.

[Searches online for jq documentation and installation...]

Found: jq is a command-line tool for parsing and manipulating JSON data. Let me install it and show you how it works.

[Installs jq via brew...]

✓ Installed: jq version 1.7.1
✓ Location: /opt/homebrew/bin/jq

Let me run a quick sanity check...

[Runs test JSON through jq...]

✓ Sanity check passed!

Basic Usage:

1. Parse JSON and extract a field:
   echo '{"name":"John","age":30}' | jq '.name'
   # Output: "John"

2. Pretty-print JSON:
   echo '{"compact":"json"}' | jq '.'

3. Filter arrays:
   echo '[1,2,3,4,5]' | jq '.[] | select(. > 2)'
   # Output: 3, 4, 5

Key Documentation:
- Official site: https://jqlang.github.io/jq/
- Manual: https://jqlang.github.io/jq/manual/
- Tutorial: https://jqlang.github.io/jq/tutorial/

Next Steps:
- Try parsing a real JSON file: `jq '.' yourfile.json`
- Learn jq filters and transformations
- Explore jq in shell scripts for JSON processing
```

## Debugging

If things aren't working:

1. **Check plugin structure:**
   ```bash
   ls -la ~/Dropbox/GitHub/claude-config/plugins/learn-tool/
   cat ~/Dropbox/GitHub/claude-config/plugins/learn-tool/.claude-plugin/plugin.json
   ```

2. **Verify settings:**
   ```bash
   cat ~/.claude/settings.json | grep -A 3 pluginDirs
   ```

3. **Check symlink:**
   ```bash
   ls -la ~/.claude/settings.json
   ```

4. **Restart session:**
   - Exit current Claude Code session
   - Start new conversation
   - Try test again

## Feedback

After testing, note:
- Which trigger phrases work best
- Any missing package managers or tool types
- Improvements to the sanity check process
- Additional examples that would be helpful

Use this feedback to update `skills/learn-tool/SKILL.md`.
