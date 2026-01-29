# Tasks

## Phase 1: Foundation
- [x] Finalize tech stack: Rust TUI (Ratatui + Crossterm + Tokio). <!-- id: 100 -->
- [ ] Implement terminal setup and basic layout. <!-- id: 101 -->
- [ ] Implement Tokyo Night color theme. <!-- id: 102 -->
- [ ] Implement basic event loop with quit handling. <!-- id: 103 -->

## Phase 2: Arti Monitoring
- [ ] Detect Arti process status (sysinfo). <!-- id: 200 -->
- [ ] Parse Arti log file for events. <!-- id: 201 -->
- [ ] Test SOCKS5 proxy connectivity. <!-- id: 202 -->
- [ ] Display circuit information (if available from logs). <!-- id: 203 -->

## Phase 3: Onion Services & Dependencies
- [ ] Detect configured onion services from arti.toml. <!-- id: 300 -->
- [ ] Monitor onion service hostnames and status. <!-- id: 301 -->
- [ ] Detect dependent applications using SOCKS proxy. <!-- id: 302 -->
- [ ] Display connection status for each dependent app. <!-- id: 303 -->

## Phase 4: Actions & Navigation
- [ ] Implement start/stop/restart for Arti (launchctl). <!-- id: 400 -->
- [ ] Implement Vim-style log navigation (j/k, gg/G). <!-- id: 401 -->
- [ ] Implement log search with /. <!-- id: 402 -->
- [ ] Implement log filter by level (INFO/WARN/ERROR). <!-- id: 403 -->
- [ ] Implement help overlay (?). <!-- id: 404 -->

## Phase 5: Privacy & Polish
- [ ] Integrate with Guardian for leak detection status. <!-- id: 500 -->
- [ ] Display privacy indicator in header. <!-- id: 501 -->
- [ ] Handle error states gracefully. <!-- id: 502 -->
- [ ] Responsive layout for terminal resize. <!-- id: 503 -->

---

## Future Enhancements (Feature Requests)

### Circuit Topology Visualization <!-- id: F001 -->
- [ ] Migrate to Tauri GUI for rich circuit visualization.
- [ ] Display circuit paths with relay geolocation.
- [ ] Animate circuit establishment.

### Background Daemon Mode <!-- id: F002 -->
- [ ] Refactor into daemon + TUI client architecture.
- [ ] Expose metrics via Prometheus-compatible endpoint.
- [ ] TUI client connects to daemon for display.

### Pluggable Transport Monitoring <!-- id: F003 -->
- [ ] Display status of obfs4/Snowflake bridges.
- [ ] Test and report bridge connectivity.
- [ ] Alert on bridge failures.
