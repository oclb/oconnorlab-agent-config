# Claude Code Notifications

This configuration includes a comprehensive notification system that works both locally (macOS) and on the O2 cluster.

## Overview

You'll receive notifications in two scenarios:
1. **Awaiting Input**: When Claude Code needs your input or permission
2. **Task Completed**: When Claude Code finishes responding

**Notifications include the conversation name** (directory name) so you can identify which project needs attention.

## How It Works

### Local (macOS)
- Uses macOS native notifications (terminal-notifier or osascript)
- Plays distinct sounds for different notification types
- Shows notifications with Claude's icon

### O2 Cluster
- Uses [ntfy.sh](https://ntfy.sh/) to send notifications to your phone/desktop
- Works from both login and compute nodes
- Cross-platform (receive on any device)

### Notification Hook Script

The `claude-notify-hook.sh` script automatically detects your environment:
- On O2: Uses ntfy.sh (requires `NTFY_TOPIC` to be set)
- On macOS: Uses terminal-notifier or osascript
- Fallback: Prints to stderr

The script reads JSON input from Claude Code hooks via stdin to extract:
- **Current working directory**: Used to show the conversation/project name
- **Transcript path**: Available for future enhancements

Notifications will appear as: **"Claude Code: project-name"** followed by the message.

## Setup

### Local (macOS)

**Option 1: Use terminal-notifier (recommended for better icons)**
```bash
brew install terminal-notifier
```

**Option 2: Use built-in osascript (works out of the box)**

No installation needed - osascript is built into macOS.

**Configuration**

The setup script has already configured your `settings.json` with notification hooks. No additional setup needed!

### O2 Cluster

Notifications on O2 are set up automatically by the `setup-o2.sh` script:

1. **Run setup** (if you haven't already):
   ```bash
   cd ~/path/to/claude-config
   ./setup-o2.sh
   ```

2. **Subscribe to notifications**:
   - **Phone**: Install ntfy app, subscribe to `your_username_o2_notifications`
   - **Desktop**: Visit `https://ntfy.sh/your_username_o2_notifications`

3. **Test it**:
   ```bash
   source ~/.bashrc
   test_notify
   ```

## Configuration Details

### Hooks Configured

Your `settings.json` includes these hooks:

```json
{
  "hooks": {
    "Notification": [
      {
        "matcher": "*",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/claude-notify-hook.sh 'Claude Code' 'Awaiting your input'",
            "timeout": 5
          }
        ]
      }
    ],
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/claude-notify-hook.sh 'Claude Code' 'Task completed'",
            "timeout": 5
          }
        ]
      }
    ]
  }
}
```

### Hook Types

- **Notification**: Fires when Claude Code needs input
- **Stop**: Fires when Claude Code finishes responding

### Sounds (macOS only)

- **Awaiting Input**: Glass.aiff (gentle bell)
- **Task Completed**: Hero.aiff (triumphant tone)

## Testing

### Local
```bash
# Test the notification script directly
~/path/to/claude-config/claude-notify-hook.sh "Test" "Hello from Claude!"

# Use Claude Code - you'll see notifications when it stops
claude "echo hello"
```

### O2
```bash
# Test ntfy system
test_notify

# Use Claude Code in an interactive session
srun -p interactive -t 0-1:00 --mem=4G --pty /bin/bash
claude "echo hello"
```

You should receive notifications on your subscribed devices when Claude Code completes.

## Troubleshooting

### Local: No notifications appearing

**Check if terminal-notifier is installed:**
```bash
which terminal-notifier
```

If not found, install it:
```bash
brew install terminal-notifier
```

**Test osascript fallback:**
```bash
osascript -e 'display notification "Test" with title "Claude Code"'
```

**Check notification permissions:**
- System Settings → Notifications → Terminal (or your terminal app)
- Ensure "Allow Notifications" is enabled

### O2: No notifications received

**Check NTFY_TOPIC is set:**
```bash
echo $NTFY_TOPIC
```

Should show something like: `your_username_o2_notifications`

**Test manually:**
```bash
curl -d "Test message" https://ntfy.sh/$NTFY_TOPIC
```

**Check subscription:**
- Verify you're subscribed to the correct topic on your device
- Visit `https://ntfy.sh/$NTFY_TOPIC` in a browser to see messages

### Hooks not firing

**Check settings.json:**
```bash
cat ~/.claude/settings.json | grep -A 5 hooks
```

Should show the hooks configuration.

**Check hook script is executable:**
```bash
ls -la ~/path/to/claude-config/claude-notify-hook.sh
```

Should start with `-rwxr-xr-x` (executable).

**Enable debug mode:**
```bash
claude --debug hooks
```

This will show hook execution in real-time.

## Customization

### Change notification messages

Edit `settings.json` and modify the messages:

```json
"command": "/path/to/claude-notify-hook.sh 'Custom Title' 'Custom message'"
```

### Change sounds (macOS)

Edit `claude-notify-hook.sh` and modify the `afplay` lines:

```bash
# Available system sounds:
ls /System/Library/Sounds/

# Examples: Basso, Blow, Bottle, Frog, Funk, Glass, Hero, Morse, etc.
```

### Add more hooks

See [Claude Code Hooks Documentation](https://code.claude.com/docs/en/hooks) for other available hooks:
- `SessionStart`: When a new session starts
- `SubagentStop`: When subagents finish
- `Error`: When errors occur

## Privacy & Security

### ntfy.sh (O2)
- Topics are public by default (anyone who knows your topic name can see messages)
- Don't send sensitive data in notifications
- Use a random topic name for obscurity
- For sensitive environments, consider [self-hosting ntfy](https://docs.ntfy.sh/install/)

### Local notifications
- Notifications only visible on your local machine
- May appear in notification history/notification center
- Can be disabled in System Settings

## Resources

- [Claude Code Hooks Guide](https://code.claude.com/docs/en/hooks-guide)
- [ntfy.sh Documentation](https://docs.ntfy.sh/)
- [terminal-notifier GitHub](https://github.com/julienXX/terminal-notifier)
- [Claude Code Notification Examples](https://www.d12frosted.io/posts/2026-01-05-claude-code-notifications)
