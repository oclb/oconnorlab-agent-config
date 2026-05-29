# Notebook Entry Examples

These examples show the expected style and level of detail for notebook entries.

## Analysis Entry

~~~markdown
# Gene Expression Batch Effect Analysis

**Date:** 2026-01-15
**Author:** Codex
**User:** jsmith

## Summary
Investigated unexpected clustering in PCA of RNA-seq data. Identified sequencing batch as primary driver of PC1 (explaining 23% of variance). Recommended ComBat correction before downstream analysis.

## Details
Initial PCA showed samples clustering by an unknown factor rather than treatment group. Systematically tested metadata variables:

| Variable | PC1 correlation | PC2 correlation |
|----------|-----------------|-----------------|
| Batch | 0.89 | 0.12 |
| Treatment | 0.15 | 0.67 |
| RIN score | 0.34 | 0.08 |

Batch correction with ComBat reduced batch-PC1 correlation to 0.11 while preserving treatment signal. QC plots saved to `figures/batch_correction_qc.png`.

**Decision:** Use ComBat-corrected counts for all downstream analyses. Raw counts preserved in `data/raw/` for reproducibility.

## References
- `rnaseq-pipeline-setup`: sample metadata location and format
~~~

## Feature Implementation Entry

~~~markdown
# User Authentication with OAuth2

**Date:** 2026-01-10
**Author:** Codex
**User:** jsmith

## Summary
Implemented OAuth2 authentication flow with Google provider. Users can now sign in via Google account, with session persistence via HTTP-only cookies.

## Details
Architecture decisions:
- **Provider:** Google OAuth2 (most users have accounts, good documentation)
- **Session storage:** HTTP-only cookies with 7-day expiry (balances security and UX)
- **Token refresh:** Silent refresh 5 minutes before expiry

Key files modified:
- `src/auth/oauth.ts` - OAuth flow implementation
- `src/middleware/session.ts` - Session validation middleware
- `src/routes/auth.ts` - `/auth/login`, `/auth/callback`, `/auth/logout` endpoints

Edge cases handled:
- Token refresh failure → redirect to login with flash message
- Revoked Google permissions → clear session, prompt re-auth
- Concurrent sessions → allowed (no single-session enforcement)

## References
- `api-route-structure`: followed existing route patterns for consistency
~~~

## Debugging/Research Entry

~~~markdown
# SSH Socket Validation Findings

**Date:** 2026-01-17
**Author:** Codex
**User:** jsmith

## Summary
Discovered that ControlMaster `-O check` returns success even for stale sockets. Implemented inode-based validation as reliable alternative.

## Details
Initial approach used `ssh -O check` but this only verifies socket file exists, not connection validity. After testing, found that comparing socket inode before/after connection attempt reliably detects stale sockets.

Validation function:
```bash
validate_socket() {
    local socket="$1"
    local inode_before=$(stat -f %i "$socket" 2>/dev/null)
    ssh -O check -S "$socket" user@host 2>/dev/null
    local inode_after=$(stat -f %i "$socket" 2>/dev/null)
    [[ "$inode_before" == "$inode_after" ]]
}
```

**Root cause:** ControlMaster checks socket file descriptor, not actual TCP connection state. When connection drops, socket file persists until explicitly removed or new connection attempted.

## References
- `o2-connection-setup`: original socket implementation this extends
~~~
