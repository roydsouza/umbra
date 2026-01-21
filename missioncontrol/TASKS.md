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

### Phase 4: Crypto Integration (DarkMatter)
- [x] **Monero Client**: Implement `monero.rs` (RPC/Metrics) in `core`.
- [x] **Crypto Manager**: Unified manager for Zcash/Monero in `core`.
- [x] **Tauri Bridge**: Expose `get_crypto_status` command.
- [x] **DarkMatter Page**: UI for Node Status (Sync, Peers, Height).

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
