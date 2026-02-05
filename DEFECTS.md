# Umbra: Stable Defects

## Open Defects

- [ ] Verify documentation sync (Documentation stale by ~23h)
- [ ] **Zebra Log Split Personality**: Logs appear in both `darkmatter/var/log/zebrad.log` and `darkmatter/zcash/var/log/zebrad.log`. This split behavior confuses observability. Needs consolidation.
- [ ] **Arti "Ghost Configuration" Error**: `start-arti` consistently reports `File or directory "${HOME}/antigravity/umbra/arti.toml" not found` on startup.
    - **Impact**: Harmless to operation, but incorrectly signals failure.
    - **Diagnosis**: Not present in `arti.toml` (verified clean), not hardcoded in binary (verified via grep). Likely an environment variable (`ARTI_CONFIG`?) or hidden default in the `start-arti` execution context.
    - **Workaround**: Logging restored by enabling `console="info"` and capturing stderr in `arti.err.log`. `arti.log` (file target) remains empty/broken due to this error.

## Resolved

*(None yet)*
