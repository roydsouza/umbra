# Tasks: MissionControl

> [!IMPORTANT]
> **STRATEGY SHIFT**: MissionControl is migrating to a **Tauri 2 Thick Client**. 
> All Axum-specific web server tasks are DEPRECATED unless critical for the migration.

---

## 🚀 Tauri Migration Roadmap

### Phase 1: Foundation
- [x] Initialize `missioncontrol-tauri` (Tauri 2 + React + Vite + Tailwind).
- [x] Port "GravityLens" Theme (CSS to Tailwind config).
- [x] Establish "App Shell" (Sidebar, Header, Main Layout).
- [x] Configure `missioncontrol-core` library structure.

### Phase 2: Core Wiring
- [x] Refactor `ArtiManager` and `GuardianClient` into `missioncontrol-core`.
- [x] Implement Tauri Commands for System Status.
- [x] Implement Tauri Events for Real-time Data (replacing WebSockets).

### Phase 3: Feature Porting
- [x] **Home Dashboard**: Rebuild in React.
- [x] **Circuit Visualizer**: Interactive circuit map (React Flow).
- [x] **Guardian Shield**: Real-time leak monitor.

### Phase 5: Stabilization & Polish
- [x] **Launch Script**: Robust `mc` script with zombie cleanup.
- [x] **UI Polish**: Fix Tailwind v4 Config (Fonts, Borders).
- [x] **Reliability**: Fix Infinite Reload Loop (Ignore DB).
- [x] **Stability**: Fix Startup Crashes (Async Runtime).

### Phase 6: High Availability & Resilience
- [x] **Arti Supervisor**: Background loop with backoff for Tor reliability.
- [x] **Guardian Supervisor**: Process monitoring and auto-restart for the Shield.
- [x] **Documentation**: Created `ARTI.md` and updated `Walkthrough`.

## 🛡️ Defects & Issues
- [x] **Launcher Failure**: Fixed `mc` script environment resolution and missing dependencies.
- [x] **Graceful Exit**: `mc` script now traps signals and terminates process group on exit.
- [x] **ARTI Status**: Fixed race condition in `useArtiStatus` and implemented non-blocking bootstrap.
- [x] **Guardian Shield Offline**: Fixed status reporting and verified service connectivity.
- [x] **Circuit Metadata**: Replaced mocks with real `tor-circmgr` introspection.
- [x] **Empty Dashboard Sections**: Populated Services, Metrics, and Load stats.
- [x] **DarkMatter Telemetry**: Fixed Zebra Prometheus parsing and wired Monero controls.
- [ ] **Home vs Guardian Status Mismatch**: Home screen reports OFFLINE while Guardian tab claims ACTIVE.
    - [x] Fix: Decouple hardcoded "ACTIVE" in `Guardian.tsx`.
    - [ ] Fix: Sync `stats.guardian_connected` logic in `commands.rs` to include process health check.

### Reliability & Polish (Todo)
- [ ] **GeoIP Integration**: Map relay fingerprints to actual countries (currently "??").
- [ ] **Advanced Circuit Tracking**: Track circuit age and detailed usage (bandwidth per circuit).
- [ ] **Service Heartbeat**: Add more robust error states for offline services (e.g., clickable "Fix" buttons).
- [ ] **Log Stream Consolidation**: Create a unified log viewer for Arti, Guardian, and Nodes.

### Phase 6: Production Readiness (Todo)
- [ ] **Production Build**: Verify `cargo tauri build` works.
- [ ] **Code Cleanup**: Remove unused fields/imports (Rust warnings).
- [ ] **Arti Optimization**: Improve bootstrap speed/feedback.
- [ ] **Mobile Support**: Investigate iOS/Android targets.

---

## 🎯 Active Development
### P0: Architecture Setup
- [x] Create `missioncontrol/core` crate.
- [x] Create `missioncontrol/tauri` app.
- [x] Verify Arti bootstrap works inside Tauri `setup()`.

### P1: UI/UX (GravityLens Theme)
- [x] Implement `GlassCard` component.
- [x] Implement `NeonBorder` effects.
- [x] Implement `SpaceBackground` (Canvas/CSS).

---

## 🧊 Icebox (Legacy Axum Tasks)
*Protected for reference, but likely won't be implemented in Axum version.*
- [ ] Onion Service CRUD (Axum HTML).
- [ ] Alerting System (Axum).
- [ ] Log Viewer (Axum).
