**User:** Luke Jen O'Connor

## Problem: /init-project O2 SSH key setup was hallucinated

During `/init-project` Phase 7 (O2 setup), the skill instructions for SSH key setup contain hallucinated steps:

1. **Hallucinated URL**: The skill says to add SSH keys at `https://rc.hms.harvard.edu/` via an "SSH Keys" settings page. This page has no login button and no such feature exists.

2. **Wrong approach**: The skill suggests a web-based key management flow that doesn't exist for O2.

3. **Bridge password auth is broken**: The bridge's interactive PTY-based password authentication has a known bug. SSH key auth is required for the bridge to work.

## Correct SSH key setup for O2

The correct and simple process is:

```bash
# 1. Generate key
ssh-keygen -t ed25519 -f ~/.ssh/o2_ed25519 -N "" -C "o2-cluster-access"

# 2. Add IdentityFile to ~/.ssh/config (under the O2 Host entry)
#    IdentityFile ~/.ssh/o2_ed25519

# 3. Copy to O2 (will prompt for password once via standard SSH)
ssh-copy-id -i ~/.ssh/o2_ed25519 <username>@o2.hms.harvard.edu

# 4. Bridge should now work without password
remote-bridge start o2 --user <username>
```

That's it. Three commands + one config line. No web portals, no HMS RC website.

## Action items

- Update the `/init-project` skill Phase 7 (SSH key setup sections 7.2.1-7.2.4) to use the correct `ssh-copy-id` approach
- Remove references to `https://rc.hms.harvard.edu/` for key management
- Update the `/remote-o2` skill Phase 3 (SSH Key Setup) similarly
- Note that bridge password auth is broken; SSH key is required, not optional
- The bridge's `start` command does NOT have an `--identity` flag; it picks up keys from `~/.ssh/config`
