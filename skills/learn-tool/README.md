# Learn Tool Plugin

A Claude Code skill that helps you quickly learn and set up new tools, libraries, and frameworks.

## What It Does

When you ask Claude to learn, explore, or set up a new tool, this skill automatically:

1. **Searches online** for official documentation, installation guides, and tutorials
2. **Installs the tool** using the appropriate package manager (npm, pip, brew, etc.)
3. **Runs sanity checks** to verify the installation works correctly
4. **Provides a summary** with examples, documentation links, and next steps

## Usage

Simply ask Claude to help you learn a tool:

- "Help me learn ripgrep"
- "I want to try out the axios library"
- "Set up and test Tailwind CSS"
- "Explore what jq does and how to use it"
- "Get started with pytest"

Claude will automatically:
- Find the latest documentation
- Install it (after asking if needed)
- Run basic tests to ensure it works
- Show you practical examples
- Provide resources for deeper learning

## Examples

### Learning a CLI Tool

```
You: Help me learn ripgrep

Claude will:
1. Search for ripgrep documentation
2. Install via brew (or appropriate package manager)
3. Verify with rg --version
4. Run test searches
5. Show common usage patterns
6. Link to official docs
```

### Learning a JavaScript Library

```
You: I want to try out axios

Claude will:
1. Find axios documentation
2. Install via npm
3. Create a test script
4. Make a sample HTTP request
5. Show GET/POST examples
6. Suggest next steps
```

### Learning a Python Package

```
You: Set up pytest and show me how it works

Claude will:
1. Search for pytest documentation
2. Install via pip
3. Create a sample test file
4. Run the tests
5. Explain test discovery and assertions
6. Provide best practices
```

## Installation

### Option 1: Install to User Plugin Directory

```bash
# Copy the plugin to your local plugins directory
mkdir -p ~/.claude/plugins/
cp -r plugins/learn-tool ~/.claude/plugins/

# Restart Claude Code or start a new session
```

### Option 2: Use from Repository (Recommended if using this config repo)

Add this to your `~/.claude/settings.json`:

```json
{
  "pluginDirs": [
    "~/Dropbox/GitHub/claude-config/plugins"
  ]
}
```

This way the plugin stays synced with your configuration repository.

## How It Works

This is a **skill**, not a command. Skills are capabilities that Claude automatically uses based on context:

- **Commands** (like `/commit`) are explicitly invoked by users
- **Skills** (like this) are automatically activated when relevant

The skill activates when you use phrases like:
- "learn [tool]"
- "explore [library]"
- "try out [framework]"
- "set up [package]"
- "get started with [tool]"

## Customization

You can modify `skills/learn-tool/SKILL.md` to:
- Adjust trigger phrases in the description
- Add specific tools you frequently use
- Customize the sanity check process
- Add company-specific package registries or tools

## What the Skill Covers

### Package Managers
- **JavaScript/Node**: npm, yarn, pnpm
- **Python**: pip, pip3, poetry
- **Ruby**: gem
- **Rust**: cargo
- **Go**: go install
- **System**: brew (macOS), apt-get (Linux)

### Tool Types
- CLI tools and utilities
- Programming libraries
- Web frameworks
- Build tools
- Testing frameworks
- Development tools

### Sanity Checks
- Version verification
- Basic command execution
- Simple test scripts
- "Hello World" examples
- Configuration validation

## Best Practices

1. **Let Claude search first** - The skill will find current documentation
2. **Review before installing** - Claude will explain what it's installing
3. **Check the sanity tests** - Verify the examples make sense for your use case
4. **Read the summary** - Key documentation links are provided for deeper learning

## Troubleshooting

### Skill not activating?

Make sure:
- The plugin is installed correctly (`claude plugin list` - note: there may not be a list command)
- You're using trigger phrases like "learn" or "explore"
- Your settings.json includes the pluginDirs if using Option 2

### Installation failures?

The skill will:
- Check if the tool is already installed
- Suggest alternative installation methods
- Search for common error solutions
- Provide manual installation steps if needed

## Contributing

Feel free to customize this skill for your needs. Common modifications:

- Add support for additional package managers
- Include company-specific tool registries
- Add pre-configured sanity checks for frequently used tools
- Customize the output format

## Version

Current version: 1.0.0

## License

Use and modify as needed for your team.
