# Tasks: MissionControl

> [!IMPORTANT]
> MissionControl is the **centralized command center** for the Umbra ecosystem. It must be effective, efficient, comprehensive, and easy to learn and use.

---

## 🎯 Mission Clarity

**Purpose:**
MissionControl is the single pane of glass for:
1. **Arti Management** — Monitor and control the Tor client.
2. **Guardian Integration** — Network leak detection and policy configuration.
3. **DarkMatter Integration** — Crypto node monitoring (Zebra, Monero).
4. **GravityLens Integration** — Forensics and wallet tracking (future).

**Design Principles:**
- **Privacy First**: All connections routed through Arti/Tor.
- **Zero-Leak**: Localhost binding only, header stripping, no external assets.
- **Easy to Learn**: Clear navigation, consistent UI patterns, contextual help.
- **Real-time**: WebSocket-powered live updates for metrics and alerts.

---

## 📋 Completed (Current State)

### Infrastructure
- [x] Rust project structure with Axum web server.
- [x] Arti bootstrap in background (non-blocking startup).
- [x] Configuration loading from `config/missioncontrol.toml`.
- [x] SQLite database for persistent state.
- [x] Security middleware (header stripping).
- [x] Graceful shutdown on SIGTERM.

### Pages (SSR HTML)
- [x] **Home** — Dashboard with Arti status, circuit count, integration placeholders.
- [x] **Circuits** — Circuit path visualization, active circuits table (static data).
- [x] **Services** — Onion service management, Arti status cards, quick reference.
- [x] **Metrics** — Bandwidth chart, latency rings, event log (static data).
- [x] **Integrations** — Node grid (Zcash, Bitcoin, Monero), DarkMatter/GravityLens status.
- [x] **Config** — Current configuration display.

### API Endpoints
- [x] `GET /api/status` — Overall system status.
- [x] `GET /api/arti/status` — Arti client status.

### Integrations
- [x] DarkMatter module scaffolded (`integrations/darkmatter.rs`).
- [x] Zebra metrics parsing (Prometheus format).

---

## 🚨 Critical Gaps

### 1. Guardian Integration (P0 — MISSING ENTIRELY)
Guardian is not integrated into MissionControl. This is critical for centralized monitoring.

**Required:**
- [ ] Create `src/integrations/guardian.rs` module.
- [ ] Add Guardian API client to fetch leak events.
- [ ] Add new navigation item: **"Guardian" / "Network Shield"**.
- [ ] Create dedicated Guardian page with:
  - Real-time leak stream (WebSocket).
  - Protected apps list with status indicators.
  - DNS policy configuration UI.
  - Severity filter (CRITICAL / WARNING / INFO).
  - Process attribution display.
- [ ] Add Guardian status to Home dashboard:
  ```
  [Guardian Status]
  🛡️ No leaks detected | ⚠️ 3 warnings | 🚨 CRITICAL: 2 leaks
  ```
- [ ] API endpoints:
  - `GET /api/guardian/status` — Leak counts, protected app status.
  - `GET /api/guardian/events` — Recent leak events.
  - `POST /api/guardian/config` — Update DNS policy, protected apps.
  - `WS /api/guardian/stream` — Real-time leak events.

---

### 2. Real-Time Data (P0)
All pages currently show static placeholder data.

**Current State:**
- Circuit table is hardcoded HTML.
- Metrics are static placeholders.
- No live data polling.

**Required:**
- [ ] Implement WebSocket infrastructure for real-time updates.
- [ ] Circuits page: Fetch real circuit data from Arti.
- [ ] Metrics page: Fetch real bandwidth/latency from Arti.
- [ ] Add auto-refresh with configurable interval.
- [ ] Add "Last Updated" timestamps.

---

### 3. DarkMatter Integration Completion (P1)
Module exists but is not wired to the UI.

**Current State:**
- `darkmatter.rs` can fetch Zebra metrics.
- Integrations page shows placeholder data.

**Required:**
- [ ] Wire DarkMatter module to Integrations page.
- [ ] Live Zebra node status (block height, peers, sync %).
- [ ] Add Monero integration (`monerod` metrics endpoint).
- [ ] API endpoint: `GET /api/integrations/darkmatter/status`.
- [ ] Add "View Metrics" button that expands detailed node view.
- [ ] Add restart/stop controls (if feasible via RPC).

---

### 4. Onion Service CRUD (P1)
Services page has UI but no functionality.

**Current State:**
- New Service modal exists but form submission does nothing.
- Service list is hardcoded.

**Required:**
- [ ] Implement service creation via Arti's onion service API.
- [ ] Store service config in database.
- [ ] List active services from Arti state.
- [ ] Enable start/stop/delete operations.
- [ ] Display actual onion addresses (not placeholders).
- [ ] Client authorization key generation and management.

---

### 5. Configuration UI (P1)
Config page is read-only display.

**Required:**
- [ ] Editable configuration form.
- [ ] Hot-reload configuration without restart.
- [ ] Guardian configuration section:
  - Protected directories.
  - Protected binaries.
  - DNS policies (local router, public DNS).
- [ ] Arti configuration section:
  - Vanguards mode.
  - Circuit isolation settings.
  - Log verbosity.
- [ ] Save/Apply button with validation.

---

### 6. Alerting System (P2)
No notification system for critical events.

**Required:**
- [ ] In-app notification banner for critical events.
- [ ] macOS native notifications via `notify-rust`.
- [ ] Alert configuration:
  - Arti bootstrap failure.
  - Guardian critical leak detected.
  - Node sync stalled.
- [ ] Notification history page or sidebar.

---

### 7. Log Viewer Enhancement (P2)
Event log on Metrics page is static.

**Required:**
- [ ] Stream real logs from Arti (`var/log/arti.log`).
- [ ] Add Guardian leak events to log viewer.
- [ ] Log filtering by level (INFO, WARN, ERROR).
- [ ] Log search functionality.
- [ ] Log download/export.

---

### 8. Circuit Management (P2)
Circuit visualization exists but is non-functional.

**Required:**
- [ ] Display real relay information (fingerprint, country, bandwidth).
- [ ] Implement "New Circuit" button functionality.
- [ ] Implement "Close Circuit" button functionality.
- [ ] Circuit age tracking.
- [ ] Guard selection statistics.

---

### 9. Mobile Responsiveness (P3)
Current UI is optimized for laptop.

**Required:**
- [ ] Responsive CSS for tablet/phone access via Tor Browser on mobile.
- [ ] Collapsible navigation menu.
- [ ] Touch-friendly controls.
- [ ] **Contextual Help Button (P2)**:
  - Add a "?" or "Help" button in the top right of every page (similar to GravityLens).
  - Provide a slide-over or modal with page-specific explanations.
  - Explain metrics, controls, and security implications for each view.

---

### 10. Authentication & Authorization (P3)
No authentication on localhost.

**Required (for Onion Service access):**
- [ ] Implement session-based authentication.
- [ ] Password/PIN entry on first access.
- [ ] Session timeout configuration.
- [ ] Audit log for authentication events.

---

## 📊 Priority Matrix

| Task | Priority | Complexity | Impact |
|------|----------|------------|--------|
| Guardian Integration | P0 | Medium | Critical — centralized leak monitoring |
| Real-Time Data | P0 | Medium | Critical — usability and trust |
| DarkMatter Wiring | P1 | Low | High — node visibility |
| Onion Service CRUD | P1 | High | High — core functionality |
| Configuration UI | P1 | Medium | High — operability |
| Alerting System | P2 | Medium | Medium — awareness |
| Log Viewer Enhancement | P2 | Low | Medium — debugging |
| Circuit Management | P2 | Medium | Medium — power user feature |
| Mobile Responsiveness | P3 | Medium | Low — nice-to-have |
| Authentication | P3 | High | High (for remote) — security |

---

## 🛡️ Guardian Integration Details

### Page Layout: `/guardian`

```
┌────────────────────────────────────────────────────────────────┐
│  🛡️ Guardian Network Shield                          [Status] │
├────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │ 0 CRITICAL   │  │ 3 WARNINGS   │  │ 142 INFO     │         │
│  │ Leaks        │  │ DNS Local    │  │ Packets      │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
├────────────────────────────────────────────────────────────────┤
│  Protected Applications                                        │
│  ┌────────────────────────────────────────────────────────┐   │
│  │ 🛡️ zebrad     │ ~/antigravity/darkmatter/zebra   │ ✅  │   │
│  │ ₿ monerod    │ ~/antigravity/darkmatter/monero  │ ✅  │   │
│  │ 🧅 arti      │ ~/antigravity/umbra/bin          │ ✅  │   │
│  └────────────────────────────────────────────────────────┘   │
├────────────────────────────────────────────────────────────────┤
│  DNS Policy                                    [Configure]     │
│  • Local Router (192.168.1.1): Allowed                        │
│  • Public DNS: Critical Alert                                  │
├────────────────────────────────────────────────────────────────┤
│  Live Event Stream                             [Filter ▼]      │
│  ┌────────────────────────────────────────────────────────┐   │
│  │ 17:24:32  INFO   curl       → 93.184.216.34:80   ✓    │   │
│  │ 17:24:30  WARN   mDNS       → 192.168.1.1:53    DNS   │   │
│  │ 17:24:28  CRIT   zebrad     → 8.8.8.8:53       LEAK   │   │
│  └────────────────────────────────────────────────────────┘   │
└────────────────────────────────────────────────────────────────┘
```

### Guardian API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/guardian/status` | GET | Overall status (leak counts, protected apps) |
| `/api/guardian/events` | GET | Paginated leak events |
| `/api/guardian/config` | GET | Current configuration |
| `/api/guardian/config` | POST | Update configuration |
| `/api/guardian/stream` | WS | Real-time event stream |

---

## 🔧 Technical Requirements

### Guardian Communication Protocol
Guardian should expose:
1. **HTTP API** on `127.0.0.1:9109` for status/config.
2. **WebSocket** on `127.0.0.1:9109/ws` for real-time events.
3. **Prometheus metrics** on `127.0.0.1:9109/metrics`.

MissionControl will act as a proxy/aggregator for these endpoints.

---

## 🏗️ Immediate Next Steps

1. **Create Guardian Integration Module**
   - `src/integrations/guardian.rs`
   - API client for Guardian HTTP endpoints.
   - WebSocket client for real-time events.

2. **Add Guardian Page**
   - New route `/guardian`.
   - Page handler in `src/web/pages.rs`.
   - JavaScript for real-time updates.

3. **Wire DarkMatter to Integrations Page**
   - Fetch real Zebra metrics.
   - Display live block height and sync status.

4. **Implement Real-Time Circuit Data**
   - Query Arti for active circuits.
   - Update Circuits page with live data.

---

## Defects

- [ ] Static placeholder data throughout the UI.
- [ ] "New Circuit" button does nothing.
- [ ] "New Service" form submission not implemented.
- [ ] No error handling for Arti bootstrap failure in UI.
- [ ] Log viewer is static HTML, not connected to real logs.
