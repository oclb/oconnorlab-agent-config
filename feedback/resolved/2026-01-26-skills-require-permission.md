**User:** Luke Jen O'Connor

Sometimes using skills still requires permission prompts, even though skills should be pre-approved. This creates friction when invoking skills that need to run Bash commands or access files.

Potential causes to investigate:
- Skills invoke Bash commands that don't match the patterns in global/settings.json
- Skills read/write files outside the explicitly allowed paths
- Permission patterns may be too narrow for what skills actually need

Next steps:
- Identify which specific skills and commands trigger permission prompts
- Review the skill SKILL.md files to understand what tools they invoke
- Update global/settings.json or templates/project-settings.json to cover common skill operations
