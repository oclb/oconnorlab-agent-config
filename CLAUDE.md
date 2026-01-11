# Global Claude Configuration

## Configuration Repository

Your settings and skills are managed in a Git repository, symlinked to their standard locations by setup scripts. Everything should "just work" - no special paths needed.

The repo location is stored in `~/.claude/behavior.conf` as `CONFIG_REPO`. If you need to understand the configuration setup, modify settings at the source, or read documentation about available skills, check `$CONFIG_REPO/README.md`.

## Behavior Flags

At the start of each session, read `~/.claude/behavior.conf` to check the current flag values. These flags modify how you should behave.

### Flag Definitions

| Flag | Default | Behavior |
|------|---------|----------|
| `AFK` | `false` | When `true`: Be more independent. Make reasonable decisions without asking. Proceed with likely interpretations rather than clarifying ambiguities. Complete multi-step tasks autonomously. Only pause for critical decisions that would be difficult to reverse. |
| `USING_O2` | `false` | When `true`: Use the `use-o2` skill for tasks. Invoke the skill at the start of complex reasoning or problem-solving tasks. |

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

### How to Apply Flags

1. Read `~/.claude/behavior.conf` at session start
2. For each flag defined above, check if it exists in behavior.conf
3. If a flag is missing from behavior.conf (or the file doesn't exist), use the Default from the table above
4. Adjust your behavior according to the flag definitions
