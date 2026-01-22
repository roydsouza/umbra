# Tasks: MissionControl

> [!IMPORTANT]
> For architecture details and integration specs, see [ARCHITECTURE.md](ARCHITECTURE.md).

---

## Current Sprint: Penumbra Integration & UI Fixes

### P0: Penumbra DNS Integration
- [ ] **Create `PenumbraClient`** in `core/src/integrations/penumbra.rs`
    - Connect to `http://127.0.0.1:9110/status` for health check.
    - Implement `start_service()` / `stop_service()` process control.
    - (Future) WebSocket stream for real-time DNS queries.
- [ ] **Add Tauri Commands** in `src-tauri/src/commands.rs`:
    - `get_penumbra_status` — returns DNS query count, Arti connectivity.
    - `start_penumbra_service` — launches Penumbra process.
    - `stop_penumbra_service` — kills Penumbra process.
- [ ] **Create `Penumbra.tsx` Page**:
    - DNS Privacy status card (similar to Guardian).
    - Query count display.
    - Start/Stop controls.
- [ ] **Update Dashboard.tsx**:
    - Add "Penumbra DNS" to Station Services list.
    - Add Penumbra status to quick stats row.
- [ ] **Update `App.tsx` Router**:
    - Add `/penumbra` route.
    - Add sidebar navigation item.

### P1: Dashboard Real Metrics
- [ ] **Fix Hardcoded System Metrics** in `Dashboard.tsx:113-115`:
    - Replace static CPU/MEM/NET (24%, 68%, 12%) with real values.
    - Implement `get_system_metrics` Tauri command using `sysinfo` crate.
- [ ] **Create `useSystemMetrics` Hook**:
    - Poll backend every 5 seconds.
    - Return `{ cpu: number, memory: number, network: number }`.

### P2: Guardian UI Controls
- [ ] **Add Service Controls to `Guardian.tsx`**:
    - Start/Stop/Restart buttons.
    - Wire to existing `start_guardian_service` / `stop_guardian_service` commands.
- [ ] **Sync Status Logic**:
    - Ensure `stats.guardian_connected` in Dashboard matches Guardian page status.
    - Reference: [Guardian ARCHITECTURE.md](../guardian/ARCHITECTURE.md) for API spec.

---

## Backlog

### Observability
- [ ] **Unified Log Viewer**: Create a page that streams logs from Arti, Guardian, and Penumbra.
- [ ] **GeoIP Integration**: Map Tor relay fingerprints to actual countries (currently "??").

### Production Readiness
- [ ] **Production Build**: Verify `cargo tauri build` works.
- [ ] **Code Cleanup**: Remove unused fields/imports (Rust warnings).
- [ ] **Error States**: Add clickable "Fix" buttons for offline services.

---

## Completed

- [x] Tauri 2 Migration (Phase 1-3)
- [x] GravityLens Theme (GlassCard, NeonBorder, SpaceBackground)
- [x] ArtiManager with Supervisor
- [x] GuardianClient with WebSocket Events
- [x] CryptoManager (Zebra, Monero)
- [x] Circuit Visualizer (React Flow)
- [x] Launch Script (`mc`) with Zombie Cleanup

---

## References
-   [ARCHITECTURE.md](ARCHITECTURE.md) — Technical design, integration specs, IPC commands.
-   [Guardian ARCHITECTURE.md](../guardian/ARCHITECTURE.md) — Guardian API spec.
-   [Penumbra ARCHITECTURE.md](../penumbra/ARCHITECTURE.md) — Penumbra API spec (Phase 3).
