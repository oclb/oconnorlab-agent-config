# Global Claude Configuration

## Configuration Repository

Your settings and skills are managed in a Git repository, symlinked to their standard locations by setup scripts. Everything should "just work" - no special paths needed.

The repo location is stored in `~/.claude/behavior.conf` as `CONFIG_REPO`. If you need to understand the configuration setup, modify settings at the source, or read documentation about available skills, check `$CONFIG_REPO/README.md`.

## User-Specific Configuration

**IMPORTANT:** If the file `$CONFIG_REPO/global/CLAUDE.user.md` exists, read it AFTER reading this file. It contains user-specific instructions and preferences that override or extend these global settings. The user file is gitignored and won't cause conflicts when pulling from the repo.

## Behavior Flags

At the start of each session, read `~/.claude/behavior.conf` to check the current flag values. These flags modify how you should behave.

### Flag Definitions

| Flag | Default | Behavior |
|------|---------|----------|
| `AFK` | `false` | When `true`: Be more independent. Make reasonable decisions without asking. Proceed with likely interpretations rather than clarifying ambiguities. Complete multi-step tasks autonomously. Only pause for critical decisions that would be difficult to reverse. |
| `Environment` | `local` | Indicates the compute environment. When `O2`: Use the `use-o2` skill for compute-intensive tasks. Invoke the skill at the start of complex analyses or tasks requiring substantial resources. When `local`: Run tasks locally. |
| `NewUser` | `true` | When `true`: Be proactive about explaining Claude Code features and capabilities. Consider invoking the `/help` skill when the user might benefit. Offer brief explanations of relevant skills as they come up. When `false`: Assume the user is familiar with the system and focus on efficient task execution. |

### NewUser Onboarding Behavior

When `NewUser=true`, guide users through the system naturally as you work:

1. **Mention relevant skills** - When a task matches a skill, briefly note it exists (e.g., "I'll analyze this data. By the way, `/perform-analysis` provides a structured 8-step framework for this kind of work.")

2. **Offer context after tasks** - Occasionally ask "Would you like me to explain what I did?" or suggest "Type `/help` to see all available skills."

3. **Suggest help when appropriate** - If the user seems unsure or asks open-ended questions, consider invoking the help skill to orient them.

4. **Introduce AFK mode** - When you need to ask multiple questions, mention that `(afk)` mode exists for autonomous work.

5. **Don't overwhelm** - Limit explanations to once per skill/feature per session. After mentioning something, don't repeat it.

**Toggling NewUser mode**: Unlike AFK which uses keywords, NewUser mode changes only when the user explicitly asks (e.g., "I'm comfortable now, turn off onboarding" or "Enable NewUser mode again"). Use sed to update the flag in behavior.conf.

### Auto-Detection Keywords

Watch for these keywords in user prompts and automatically update the behavior.conf file:

| Keyword | Action |
|---------|--------|
| `(afk)` | Set `AFK=true` in behavior.conf |
| `(back)` | Set `AFK=false` in behavior.conf |

When you detect these keywords:
1. Use sed or similar to update the flag in `~/.claude/behavior.conf`
2. Confirm the change briefly (e.g., "AFK mode enabled")
3. Apply the new behavior immediately

### Flag File Format

The file uses simple `KEY=value` format:
- Lines starting with `#` are comments
- Blank lines are ignored
- Flag names are case-sensitive
- Boolean flags use `true` or `false`
- String flags use their defined values (e.g., `Environment=local` or `Environment=O2`)

### How to Apply Flags

1. Read `~/.claude/behavior.conf` at session start
2. For each flag defined above, check if it exists in behavior.conf
3. If a flag is missing from behavior.conf (or the file doesn't exist), use the Default from the table above
4. Adjust your behavior according to the flag definitions
