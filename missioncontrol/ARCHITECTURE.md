# MissionControl Architecture


**[📍 Back to Map](CONTENTS.md)**


This document describes the **current** technical architecture of MissionControl, the unified command center for the Umbra ecosystem. It serves as the canonical reference for design and implementation.

> [!IMPORTANT]
> MissionControl has migrated from Axum web server to **Tauri 2 Thick Client**. Legacy Axum documentation is archived in `legacy/`.

---

## High-Level Overview

```mermaid
graph TD
    subgraph MissionControl App (Tauri 2)
        A[React Frontend] --> B[Tauri Bridge]
        B --> C[Rust Backend]
    end
    subgraph Core Library (missioncontrol-core)
        C --> D[ArtiManager]
        C --> E[GuardianClient]
        C --> F[CryptoManager]
        C --> G[PenumbraClient - TODO]
    end
    subgraph Umbra Services
        D --> H[Arti Tor Client :9050]
        E --> I[Guardian Shield :9109]
        G --> J[Penumbra DNS :9110]
    end
    subgraph DarkMatter Services
        F --> K[Zebra/Zcash :9999]
        F --> L[Monero :18081]
    end
```

---

## Application Structure

```
missioncontrol/
├── tauri/                    # Tauri 2 Desktop App
│   ├── src/                  # React Frontend
│   │   ├── pages/            # Dashboard, Circuits, Guardian, DarkMatter
│   │   ├── components/       # GlassCard, Layout, etc.
│   │   └── hooks/            # useArtiStatus, useGuardianEvents, etc.
│   └── src-tauri/            # Rust Backend
│       ├── src/lib.rs        # App setup, supervisors, state
│       ├── src/commands.rs   # Tauri Commands (IPC)
│       └── src/state.rs      # AppState struct
├── core/                     # Shared Rust Library
│   ├── src/arti/             # ArtiManager
│   ├── src/integrations/     # Guardian, DarkMatter, (Penumbra TODO)
│   └── src/db.rs             # SQLite persistence
└── legacy/                   # Archived Axum implementation
```

---

## Frontend Pages

| Page | Component | Data Source | Status |
|:---|:---|:---|:---|
| Dashboard | `Dashboard.tsx` | All hooks aggregated | ⚠️ Hardcoded metrics |
| Circuits | `Circuits.tsx` | `get_circuits` command | ✅ Working |
| Guardian | `Guardian.tsx` | `useGuardianEvents` hook | ⚠️ Missing controls |
| DarkMatter | `DarkMatter.tsx` | `useCryptoStatus` hook | ✅ Working |
| **Penumbra** | *Not implemented* | — | ❌ Missing |

---

## Backend Integrations

### ArtiManager (`core/src/arti/`)
-   **Purpose**: Bootstrap and monitor the Tor connection.
-   **Supervisor**: Exponential backoff restart loop in `lib.rs`.
-   **Events**: `arti://ready`, `arti://error`.

### GuardianClient (`core/src/integrations/guardian.rs`)
-   **Purpose**: Connect to Guardian Shield API.
-   **API**: `http://127.0.0.1:9109`
-   **Features**:
    -   `GET /status` for health check.
    -   WebSocket `/stream` for real-time leak events.
    -   Start/stop service control.
-   **Supervisor**: Process monitor with auto-restart.
-   **Events**: `guardian://leak`.

### CryptoManager (`core/src/integrations/manager.rs`)
-   **Purpose**: Monitor DarkMatter nodes (Zebra, Monero).
-   **Metrics**: Prometheus scraping from Zebra `:9999`.

### PenumbraClient (`core/src/integrations/penumbra.rs`) — **TODO**
-   **Purpose**: Connect to Penumbra DNS service.
-   **API**: `http://127.0.0.1:9110` (when Phase 3 implemented)
-   **Features** (Planned):
    -   `GET /status` for health check (query count, Arti connectivity).
    -   Start/stop service control.
    -   Real-time DNS query stream (future).

---

## Known Issues & Gaps

### UI Issues
| Issue | Component | Description |
|:---|:---|:---|
| Hardcoded Metrics | `Dashboard.tsx:113-115` | CPU/MEM/NET values are static (24%, 68%, 12%) |
| Missing Controls | `Guardian.tsx` | No start/stop/restart buttons |
| Status Mismatch | Dashboard vs Guardian | Different logic for `guardian_connected` |

### Integration Gaps
| Gap | Priority | Description |
|:---|:---|:---|
| Penumbra | P1 | No integration exists — need `PenumbraClient` |
| Real System Metrics | P2 | Dashboard shows hardcoded CPU/MEM/NET |
| Guardian Controls | P2 | UI has no way to start/stop Guardian |
| Log Viewer | P3 | No unified log stream for Arti/Guardian/Penumbra |

---

## Tauri IPC Commands

| Command | Description | Status |
|:---|:---|:---|
| `get_arti_status` | Arti bootstrap state, circuit count | ✅ |
| `get_guardian_status` | Guardian connectivity, leak count | ✅ |
| `get_system_stats` | Aggregated service health | ⚠️ Partial |
| `get_circuits` | Active Tor circuit paths | ✅ |
| `get_crypto_status` | Zebra/Monero node status | ✅ |
| `start_guardian_service` | Launch Guardian process | ✅ |
| `stop_guardian_service` | Kill Guardian process | ✅ |
| `get_penumbra_status` | Penumbra DNS health | ❌ TODO |
| `start_penumbra_service` | Launch Penumbra process | ❌ TODO |
| `stop_penumbra_service` | Kill Penumbra process | ❌ TODO |

---

## Technology Stack

| Component | Crate / Framework |
|:---|:---|
| Desktop Framework | Tauri 2.0 |
| Frontend | React 18, Vite, Tailwind CSS |
| Rust Runtime | Tokio |
| Tor Client | `arti-client` |
| HTTP Client | `reqwest` |
| Database | SQLite (`rusqlite`) |
| Serialization | `serde`, `serde_json` |