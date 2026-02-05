# Umbra: Stable Defects

## Open Defects

- [ ] Verify documentation sync (Documentation stale by ~23h)
- [ ] **Zebra Log Split Personality**: Logs appear in both `darkmatter/var/log/zebrad.log` and `darkmatter/zcash/var/log/zebrad.log`. This split behavior confuses observability. Needs consolidation.
## Resolved

- [x] **Arti "Ghost Configuration" Error**: Fixed by creating symlink `umbra/arti.toml` -> `umbra/etc/arti.toml`. Arti's `watch_configuration` was looking for the default path, not our `etc/` location.
