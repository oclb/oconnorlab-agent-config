---
name: learn-tool
description: This skill should be used when the user asks to "learn", "explore", "try out", "get started with", "set up", or "install and test" a new tool, library, framework, CLI, package, or technology. Also triggers when the user mentions they want to understand how something works or needs help with initial setup.
version: 1.0.0
---

# Learn New Tool Skill

This skill helps you quickly learn and set up new tools, libraries, and frameworks by finding documentation, installing them, and verifying they work correctly.

## When This Skill Applies

Use this skill when the user wants to:
- Learn a new tool, library, or framework
- Set up and test a new technology
- Explore what a tool does and how to use it
- Get started with a new package or CLI
- Verify a tool is properly installed and working

## Process

Follow these steps systematically when helping the user learn a new tool:

### 1. Search for Documentation

**Search online for official documentation, getting started guides, and tutorials:**

- Use WebSearch to find:
  - Official documentation website
  - Installation instructions
  - Quick start or "getting started" guides
  - GitHub repository (if applicable)
  - Common use cases and examples

**What to look for:**
- Latest version and release notes
- System requirements
- Installation methods (npm, pip, brew, cargo, etc.)
- Basic usage examples
- Configuration requirements

**Example searches:**
- "[tool name] official documentation 2026"
- "[tool name] getting started guide"
- "[tool name] installation"
- "[tool name] quick start tutorial"

### 2. Install the Tool

**Determine the appropriate installation method based on the tool type:**

**Package Managers:**
- **Node.js/JavaScript**: `npm install -g [package]` or `npm install [package]` (local)
- **Python**: `pip install [package]` or `pip3 install [package]`
- **Ruby**: `gem install [package]`
- **Rust**: `cargo install [package]`
- **Go**: `go install [package]@latest`
- **System packages (macOS)**: `brew install [package]`
- **System packages (Linux)**: `apt-get install [package]` or `yum install [package]`

**Before installing:**
1. Check if the tool is already installed: `which [command]` or `[command] --version`
2. Verify system requirements and dependencies
3. Choose global vs. local installation based on use case

**After installing:**
1. Verify installation: `which [command]` and `[command] --version`
2. Note the installed version
3. Check if any post-installation setup is required (PATH updates, configuration files, etc.)

### 3. Run Sanity Checks

**Create and run basic tests to verify the tool works correctly:**

**For CLI tools:**
- Run `[tool] --help` or `[tool] -h` to see available commands
- Run `[tool] --version` to confirm installation
- Try a simple command from the documentation
- Test basic functionality with minimal input

**For libraries/packages:**
- Create a minimal test script/program
- Import/require the library
- Run a simple example from the docs
- Verify no import/runtime errors

**For frameworks:**
- Create a "Hello World" or minimal starter project
- Follow the official quick start guide
- Run the development server or build process
- Verify the expected output

**Example sanity checks by tool type:**

```bash
# CLI tool example (jq)
echo '{"name":"test"}' | jq '.name'

# Python library example
python3 -c "import numpy; print(numpy.__version__)"

# Node.js library example
node -e "const _ = require('lodash'); console.log(_.VERSION)"

# Framework example (Next.js)
npx create-next-app@latest test-app --typescript --no-install
```

### 4. Provide Summary and Next Steps

**After completing the setup, provide:**

1. **Installation confirmation**: What was installed and which version
2. **Basic usage**: Show 2-3 simple examples from the documentation
3. **Common commands**: List the most frequently used commands/functions
4. **Configuration**: Note any config files or environment variables
5. **Resources**: Link to key documentation pages
6. **Next steps**: Suggest what to learn next or try

**Summary template:**

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
- [Link to examples]

Next Steps:
- [Suggestion 1]
- [Suggestion 2]
```

## Best Practices

1. **Always search for current documentation** - Tools change frequently, so verify you're finding the latest information (2026 docs)

2. **Prefer official sources** - Official documentation, GitHub repos, and package registries are most reliable

3. **Install globally vs. locally appropriately:**
   - Global: CLI tools you'll use across projects
   - Local: Project-specific libraries and dependencies

4. **Check before installing** - Don't reinstall if already present unless specifically requested

5. **Test minimally but thoroughly** - Run enough tests to confirm it works, but don't overcomplicate

6. **Consider the user's environment:**
   - Check OS (macOS, Linux, Windows)
   - Check existing package managers
   - Consider version conflicts with existing tools

7. **Handle errors gracefully:**
   - If installation fails, search for common issues
   - Suggest alternative installation methods
   - Check for dependency problems

8. **Document the process** - Show what you're doing so the user can reproduce it

## Special Cases

### Web-based Tools or Services
If the "tool" is a web service or API:
- Find API documentation
- Show how to get API keys/credentials
- Provide authentication examples
- Show a basic API call with curl or similar

### IDE Extensions or Plugins
If it's an editor extension:
- Identify the user's editor first
- Provide installation instructions for that editor
- Show how to configure it
- Demonstrate basic usage within the editor

### Docker/Containerized Tools
If the tool runs in Docker:
- Verify Docker is installed
- Pull the container image
- Show how to run it
- Provide common docker-compose examples if applicable

### Version Managers
If learning a language version manager (nvm, pyenv, rbenv):
- Install the version manager
- Show how to install language versions
- Demonstrate switching between versions
- Configure shell integration

## Example Usage

**User**: "Help me learn ripgrep"

**Response**:
1. Search for ripgrep documentation and installation
2. Find it's a fast grep alternative written in Rust
3. Install: `brew install ripgrep`
4. Verify: `rg --version`
5. Run sanity check: `rg "test" .` (search for "test" in current directory)
6. Show common examples:
   - `rg "pattern" path/` - Basic search
   - `rg -i "pattern"` - Case insensitive
   - `rg -t py "pattern"` - Search only Python files
7. Provide link to GitHub and official guide
8. Suggest next steps: "Try searching your codebase for common patterns"

**User**: "I want to try out the axios library for Node.js"

**Response**:
1. Search for axios documentation
2. Find it's a popular HTTP client for Node.js
3. Check if Node.js is installed: `node --version`
4. Install locally: `npm install axios`
5. Create test script to make a simple GET request
6. Run the script to verify it works
7. Show examples of GET, POST requests
8. Provide link to axios docs and GitHub
9. Suggest next steps: "Try making requests to a public API like JSONPlaceholder"
