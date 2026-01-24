# Remote Bridge Hangs at "Verifying connection..."

**Date:** 2026-01-21
**Component:** remote-bridge
**Severity:** Blocking - prevents O2 setup

## Issue

The remote-bridge successfully establishes an SSH connection but hangs indefinitely at the "Verifying connection..." step. No Duo push is received even though Duo authentication should be required.

## Environment

- Machine: MacBook (wm25f-3b9)
- Shell: zsh
- SSH key authentication configured and working (direct `ssh ljo8@o2.hms.harvard.edu` works with Duo)

## Steps to Reproduce

1. Build and install remote-bridge
2. Configure permissions.toml
3. Run `remote-bridge start o2 --user ljo8`

## Observed Behavior

```
2026-01-21T10:57:31.039714Z  INFO Starting remote-bridge 'o2'
2026-01-21T10:57:31.040035Z  INFO Config integrity verified
2026-01-21T10:57:31.040230Z  INFO Config loaded: /Users/loconnor/.config/remote-bridge/permissions.toml
Connecting to ljo8@o2.hms.harvard.edu...
(You may need to approve Duo authentication)

2026-01-21T10:57:31.040246Z  INFO Starting persistent SSH session to ljo8@o2.hms.harvard.edu
Problems logging in?
Use your lower case HMS ID, like abc123, not ABC123.
If locked out, see:
https://it.hms.harvard.edu/i-want/reset-password-or-unlock-your-hms-account
echo __READY_1fd49a2bc1ac45808e605448926eecee__
2026-01-21T10:57:36.135052Z  INFO Shell ready (sentinel received)
2026-01-21T10:57:36.135135Z  INFO SSH session established
Verifying connection...
```

Then it hangs. No Duo push notification is sent.

## Earlier Error

An earlier attempt showed this error:
```
Error: Command execution failed: Failed to enable raw mode: Device not configured (os error 6)
```

This suggests a PTY/terminal configuration issue.

## Expected Behavior

- Should either complete verification and show "Bridge ready"
- Or prompt for Duo authentication

## Workaround

Direct SSH to O2 works fine. Can run jobs manually via SSH.

## Notes

- The SSH session does establish successfully (sentinel received)
- The issue is in the verification step after connection
- May be related to how the bridge handles the PTY or expects Duo to work with SSH keys
- On another machine with SSH key already configured, the bridge reportedly works

## Suggested Investigation

1. Check what the `verify_connection` step actually does
2. Check if there's a timeout or expected response that's not being received
3. Consider if SSH key auth changes the Duo flow in a way the bridge doesn't expect

## Resolution

**Fixed in commit 68d806b** (2026-01-21)

### Root Causes

1. **False shell detection**: Local PTY echo caused the sentinel probe (`echo __READY__`) to be immediately reflected back, triggering false "shell ready" detection before authentication completed.

2. **Missing SSH agent**: `SSH_AUTH_SOCK` wasn't passed to the spawned SSH process, preventing key-based auth.

3. **Probe sent during Duo prompt**: The probe was being typed into the Duo passcode prompt instead of a shell.

### Fixes Applied

1. **Shell expansion detection**: Changed probe to `echo __READY__$$` and detect `__READY__<digits>`. Local echo shows literal `$$`, but shell expands to PID - proving actual execution.

2. **SSH agent passthrough**: Added `SSH_AUTH_SOCK` environment variable to spawned SSH command.

3. **Auto Duo push**: Detect "Passcode or option" prompt and automatically send "1" to trigger push notification. Only start shell probes after Duo auth initiated (5s delay).

**Status: RESOLVED**
