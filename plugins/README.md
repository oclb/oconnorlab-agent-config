# Claude Code Plugins

This directory contains custom plugins for Claude Code.

## Available Plugins

### perform-analysis

A comprehensive skill that provides a systematic framework for performing data analyses and experiments.

**Triggers when you ask to:**
- "Perform an analysis"
- "Run an experiment"
- "Analyze this data"
- "Test if X correlates with Y"
- "Compute statistics"

**What it does:**
1. Understands the motivation for the analysis
2. Sets expectations about results
3. Verifies all data and tools are available
4. Creates an analysis plan
5. Performs the analysis with time estimates
6. Displays results with key takeaways
7. Documents choices and challenges
8. Lists all created files

**Integrates with:** learn-tool, sanity-check-data (future), submit-O2-job (future)

See `perform-analysis/README.md` for full documentation.

### learn-tool

A skill that helps you quickly learn and set up new tools, libraries, and frameworks.

**Triggers when you ask to:**
- "Learn [tool]"
- "Explore [library]"
- "Try out [framework]"
- "Set up [package]"
- "Get started with [tool]"

**What it does:**
1. Searches for official documentation
2. Installs the tool (with your permission)
3. Runs sanity checks to verify it works
4. Provides examples and next steps

See `learn-tool/README.md` for full documentation.

## Using These Plugins

These plugins are automatically loaded via the `pluginDirs` setting in `settings.json`:

```json
{
  "pluginDirs": [
    "~/Dropbox/GitHub/claude-config/plugins"
  ]
}
```

As long as settings.json is symlinked correctly, all plugins in this directory will be available in Claude Code.

## Adding New Plugins

To add a new plugin:

1. Create a directory: `plugins/my-plugin/`
2. Add plugin metadata: `plugins/my-plugin/.claude-plugin/plugin.json`
3. Add skills, commands, or agents as needed
4. See the `learn-tool` plugin as an example

## Plugin Types

### Skills
Auto-activated based on context. Define in `skills/skill-name/SKILL.md`

### Commands
User-invoked with `/command-name`. Define in `commands/command-name.md`

### Agents
Spawned by Claude for specific tasks. Define in `agents/agent-name/AGENT.md`

For more information, see the [Claude Code plugin documentation](https://docs.claude.ai/claude-code/plugins).
