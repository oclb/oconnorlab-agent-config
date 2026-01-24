# SSH ControlMaster Socket Validation Bug

**Date:** 2026-01-16
**Component:** o2-scripts/connect-o2.sh
**Severity:** Medium (causes workflow interruption)

## Problem

The `connect-o2.sh` script used `ssh -O check` to validate whether an existing SSH ControlMaster socket was functional. This check can report "Master running" even when the socket is stale and won't accept new sessions.

## Symptoms

1. User runs `ssh -M -S /tmp/o2-socket ...` when socket file exists
2. SSH says "ControlSocket already exists, disabling multiplexing" and connects normally (without creating master)
3. `ssh -S /tmp/o2-socket -O check` reports "Master running (pid=XXXXX)"
4. Actual `scp` or `ssh` commands fail with "Master refused session request: Permission denied"
5. Commands fall back to password auth which fails after too many attempts

## Root Cause

`-O check` only verifies the socket file exists and can be connected to - it doesn't verify the master will accept new multiplexed sessions. A stale socket from a previous session can pass `-O check` but refuse actual session requests.

## Fix

Changed socket validation from:
```bash
if ! ssh -S "$SOCKET" -O check $HOST 2>/dev/null; then
```

To:
```bash
if ssh -S "$SOCKET" -o ConnectTimeout=5 $HOST "echo ok" 2>/dev/null | grep -q ok; then
```

This tests with an actual command execution, which properly detects stale sockets.

## Lessons Learned

- SSH ControlMaster `-O check` is not sufficient for validating socket health
- Always test with actual command execution when socket reliability matters
- When socket exists but seems broken, remove it and recreate rather than trying to reuse
