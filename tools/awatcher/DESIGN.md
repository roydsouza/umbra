# awatcher Design Document

**[📍 Back to Map](CONTENTS.md)**


> **Version**: 1.0  
> **Status**: Architecture & Design (Pre-Implementation)  
> **Last Updated**: 2026-01-29  

---

## 1. Executive Summary

`awatcher` is a **Rust-native Terminal User Interface (TUI)** application that provides real-time observability and control for Arti (the Rust Tor implementation) and the broader Umbra privacy layer. It is designed to run in Ghostty with visual harmony to the **Tokyo Night** colorscheme.

### Key Features
- **Real-time Arti Monitoring**: Process health, circuit status, bandwidth metrics.
- **Onion Service Tracking**: Monitor locally hosted `.onion` services and their status.
- **Dependent Application Tracking**: See which apps are using the SOCKS5 proxy.
- **Privacy Verification**: Active detection of leaks via Guardian integration.
- **Live Log Streaming**: Scrollable, filterable log viewer with Vim bindings.
- **Action-Capable**: Start, stop, restart Arti directly from the TUI.
- **Vim/NeoVim Bindings**: Familiar navigation for keyboard-centric workflows.

---

## 2. Technology Stack

| Component | Technology | Rationale |
|-----------|------------|-----------|
| **Language** | Rust | Native performance, consistent with Arti codebase. |
| **TUI Framework** | `ratatui` (0.26+) | Modern fork of `tui-rs`, active development, rich widgets. |
| **Terminal Backend** | `crossterm` | Cross-platform, well-supported on macOS. |
| **Async Runtime** | `tokio` | Required for concurrent data fetching and log streaming. |
| **HTTP Client** | `reqwest` | For future Guardian/Penumbra API integration. |
| **TOML Parsing** | `toml` | For parsing `arti.toml` configuration. |
| **Log Parsing** | `notify` + custom | Filesystem watching for log file changes. |
| **Process Detection** | `sysinfo` | For detecting Arti process and resource usage. |

### Cargo.toml Dependencies (Design)
```toml
[dependencies]
ratatui = "0.26"
crossterm = "0.27"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1", features = ["derive"] }
toml = "0.8"
notify = "6"
sysinfo = "0.30"
color-eyre = "0.6"
```

---

## 3. Data Sources

### 3.1 Primary Sources

| Source | Path/Endpoint | Data Type |
|--------|---------------|-----------|
| **Arti Process** | `sysinfo` crate | PID, CPU%, Memory |
| **Arti Logs** | `~/antigravity/umbra/var/log/arti.log` | Text (tailed) |
| **Arti Error Logs** | `~/antigravity/umbra/var/log/arti.err.log` | Text (tailed) |
| **Arti Config** | `~/antigravity/umbra/arti.toml` | TOML |
| **SOCKS Proxy** | `127.0.0.1:9050` | TCP connection test |
| **Onion Hostnames** | `~/antigravity/umbra/var/lib/arti/onion_services/*/hostname` | Text files |

### 3.2 Dependent Application Detection

To detect applications using the SOCKS proxy, scan for processes connected to port 9050:

```bash
lsof -iTCP:9050 -sTCP:ESTABLISHED
```

Known applications to track:
- `zebrad` (Zcash)
- `zaino` (Zcash indexer)
- `monerod` (Monero, if configured)
- `curl` / `wget` (manual testing)

### 3.3 Guardian Integration (Future)

| Source | Endpoint | Data |
|--------|----------|------|
| **Guardian API** | `http://127.0.0.1:9110/leaks` | Leak event stream |
| **Penumbra API** | `http://127.0.0.1:9111/stats` | DNS query counts |

---

## 4. TUI Layout Design

The interface uses a **header + three-panel body** layout optimized for wide terminals (120+ columns recommended).

```
┌──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│  A-WATCHER v0.1.0                                     [Privacy: 🟢 CLEAN]  [Arti: 🟢 ONLINE]           [q:Quit ?:Help] │
├────────────────────────────────────────────────────────┬─────────────────────────────────────────────────────────────┤
│  ARTI STATUS                                           │  ONION SERVICES                                             │
│  ──────────────────────────────────────────────────    │  ──────────────────────────────────────────────────────     │
│                                                        │                                                             │
│  ● Process   🟢 RUNNING   PID: 3996   23 MB   0.3%    │  ● my-service                                               │
│  ● SOCKS5    🟢 :9050 OK                              │    abc123...xyz.onion                                       │
│  ● Circuits  Active: 3   Building: 1                  │    Ports: 80 → 127.0.0.1:8080                               │
│                                                        │    Status: 🟢 ACTIVE                                        │
│  [s]tart  s[t]op  [r]estart                           │                                                             │
│                                                        │  (No other services configured)                             │
├────────────────────────────────────────────────────────┼─────────────────────────────────────────────────────────────┤
│  DEPENDENT APPLICATIONS                                │  PRIVACY MONITOR                                            │
│  ──────────────────────────────────────────────────    │  ──────────────────────────────────────────────────────     │
│                                                        │                                                             │
│  ● zebrad       🟢 Connected  (PID: 12345)            │  Guardian: 🟢 ACTIVE (0 leaks detected)                     │
│  ● zaino        🔴 Disconnected                       │  Penumbra: 🟢 ACTIVE (142 queries proxied)                  │
│  ● monerod      ⚪ Not Running                        │                                                             │
│                                                        │  Last 24h: No clearnet DNS leaks                           │
├────────────────────────────────────────────────────────┴─────────────────────────────────────────────────────────────┤
│  LOGS (arti.log)                                                                               [/]Search [f]ilter  │
│  ─────────────────────────────────────────────────────────────────────────────────────────────────────────────────── │
│  2026-01-29 07:30:12 INFO  tor_dirmgr: Directory is complete. attempt=8                                              │
│  2026-01-29 07:30:15 INFO  tor_circmgr: Circuit established to [scrubbed]                                            │
│  2026-01-29 07:30:18 WARN  tor_guardmgr: Could not connect to guard. Retrying...                                     │
│  2026-01-29 07:30:21 INFO  tor_chanmgr: Opening channel to [scrubbed]                                                │
│  2026-01-29 07:30:24 INFO  tor_circmgr: Circuit 5 is now ready                                                       │
│  ─────────────────────────────────────────────────────────────────────────────────────────── [FOLLOW] [j/k scroll] │
└──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

### 4.1 Layout Breakdown

| Region | Content | Refresh Rate |
|--------|---------|--------------|
| **Header** | App title, privacy indicator, Arti status, keybind hints | 1s |
| **Panel A** (Top-Left) | Arti process status, SOCKS test, circuit count | 2s |
| **Panel B** (Top-Right) | Onion services with hostnames and port mappings | 5s |
| **Panel C** (Mid-Left) | Dependent applications using SOCKS proxy | 5s |
| **Panel D** (Mid-Right) | Privacy monitor (Guardian/Penumbra status) | 2s |
| **Panel E** (Bottom) | Live log stream with Vim navigation | Real-time |

### 4.2 Responsive Behavior

- **< 100 columns**: Stack all panels vertically.
- **100-140 columns**: 2-column top, full-width logs.
- **> 140 columns**: Full layout as shown above.

---

## 5. Color Palette (Tokyo Night Harmony)

Identical to `zwatcher` for visual consistency:

| Element | Color Name | Hex | Usage |
|---------|------------|-----|-------|
| **Background** | Night BG | `#1a1b26` | Inherited from terminal |
| **Foreground** | Editor FG | `#a9b1d6` | Default text |
| **Border** | Comment | `#565f89` | Panel borders |
| **Accent Primary** | Blue | `#7aa2f7` | Headers, highlights |
| **Accent Secondary** | Cyan | `#7dcfff` | Links, info labels |
| **Success** | Green | `#9ece6a` | Running status, OK |
| **Warning** | Orange | `#ff9e64` | Warnings, partial status |
| **Error** | Red | `#f7768e` | Errors, stopped, critical |
| **Purple** | Purple | `#bb9af7` | Onion addresses, privacy |
| **Gold** | Yellow | `#e0af68` | Circuit info, highlights |

### Ratatui Color Mapping
```rust
use ratatui::style::Color;

const TOKYO_BLUE: Color = Color::Rgb(122, 162, 247);    // #7aa2f7
const TOKYO_CYAN: Color = Color::Rgb(125, 207, 255);    // #7dcfff
const TOKYO_GREEN: Color = Color::Rgb(158, 206, 106);   // #9ece6a
const TOKYO_ORANGE: Color = Color::Rgb(255, 158, 100);  // #ff9e64
const TOKYO_RED: Color = Color::Rgb(247, 118, 142);     // #f7768e
const TOKYO_PURPLE: Color = Color::Rgb(187, 154, 247);  // #bb9af7
const TOKYO_GOLD: Color = Color::Rgb(224, 175, 104);    // #e0af68
const TOKYO_BORDER: Color = Color::Rgb(86, 95, 137);    // #565f89
const TOKYO_FG: Color = Color::Rgb(169, 177, 214);      // #a9b1d6
```

---

## 6. Keyboard Bindings (Vim/NeoVim Style)

Identical to `zwatcher` for muscle memory:

| Key | Action | Context |
|-----|--------|---------|
| `q` | Quit application | Global |
| `?` | Toggle help overlay | Global |
| `Tab` | Cycle focus between panels | Global |
| `j` / `↓` | Scroll down | Log viewer |
| `k` / `↑` | Scroll up | Log viewer |
| `gg` | Go to top | Log viewer |
| `G` | Go to bottom (follow mode) | Log viewer |
| `Ctrl+d` | Page down | Log viewer |
| `Ctrl+u` | Page up | Log viewer |
| `/` | Open search prompt | Log viewer |
| `n` | Next search result | Log viewer |
| `N` | Previous search result | Log viewer |
| `f` | Toggle log filter (INFO/WARN/ERROR) | Log viewer |
| `s` | Start Arti | Arti panel focused |
| `t` | Stop Arti | Arti panel focused |
| `r` | Restart Arti | Arti panel focused |
| `c` | Clear log display | Log viewer |
| `R` | Force refresh all data | Global |

---

## 7. Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              awatcher TUI                                    │
│ ┌─────────────────────────────────────────────────────────────────────────┐ │
│ │                           UI Layer (ratatui)                            │ │
│ │   ┌──────────┐  ┌───────────┐  ┌───────────┐  ┌──────────────────────┐  │ │
│ │   │  Header  │  │  Arti     │  │  Onion    │  │     Log Viewer       │  │ │
│ │   │  Widget  │  │  Status   │  │  Services │  │       Widget         │  │ │
│ │   └──────────┘  └───────────┘  └───────────┘  └──────────────────────┘  │ │
│ └─────────────────────────────────────────────────────────────────────────┘ │
│                                    ▲                                         │
│                                    │ App State                               │
│ ┌─────────────────────────────────────────────────────────────────────────┐ │
│ │                          State Manager                                   │ │
│ │   ┌───────────────┐  ┌───────────────┐  ┌───────────────┐               │ │
│ │   │  Arti State   │  │  Onion State  │  │   Log Ring    │               │ │
│ │   │  (process,    │  │  (hostnames,  │  │    Buffer     │               │ │
│ │   │   circuits)   │  │   ports)      │  │               │               │ │
│ │   └───────────────┘  └───────────────┘  └───────────────┘               │ │
│ └─────────────────────────────────────────────────────────────────────────┘ │
│                                    ▲                                         │
│                                    │ Data Updates                            │
│ ┌─────────────────────────────────────────────────────────────────────────┐ │
│ │                      Data Collector (Tokio Tasks)                       │ │
│ │   ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌────────────────┐    │ │
│ │   │  Process   │  │   SOCKS    │  │  Log File  │  │   Config       │    │ │
│ │   │  Monitor   │  │   Tester   │  │   Watcher  │  │   Parser       │    │ │
│ │   │ (sysinfo)  │  │  (tokio)   │  │  (notify)  │  │   (toml)       │    │ │
│ │   └────────────┘  └────────────┘  └────────────┘  └────────────────┘    │ │
│ └─────────────────────────────────────────────────────────────────────────┘ │
│                                    ▲                                         │
│                                    │                                         │
└────────────────────────────────────┼─────────────────────────────────────────┘
                                     │
          ┌──────────────────────────┼──────────────────────────┐
          │                          │                          │
          ▼                          ▼                          ▼
   ┌─────────────┐          ┌─────────────┐          ┌─────────────────┐
   │    arti     │          │  Guardian   │          │   Penumbra      │
   │    :9050    │          │   :9110     │          │     :9111       │
   └─────────────┘          └─────────────┘          └─────────────────┘
```

---

## 8. Module Structure

```
awatcher/
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry point, terminal setup, main loop
│   ├── app.rs               # Application state, event handling
│   ├── ui/
│   │   ├── mod.rs           # UI module exports
│   │   ├── layout.rs        # Layout computation
│   │   ├── header.rs        # Header widget (privacy, Arti status)
│   │   ├── arti.rs          # Arti status widget
│   │   ├── onion.rs         # Onion services widget
│   │   ├── deps.rs          # Dependent applications widget
│   │   ├── privacy.rs       # Privacy monitor widget
│   │   ├── logs.rs          # Log viewer widget
│   │   └── help.rs          # Help overlay widget
│   ├── data/
│   │   ├── mod.rs           # Data module exports
│   │   ├── process.rs       # Arti process detection (sysinfo)
│   │   ├── socks.rs         # SOCKS5 proxy testing
│   │   ├── logs.rs          # Log file tailing
│   │   ├── config.rs        # arti.toml parsing
│   │   ├── onion.rs         # Onion service hostname discovery
│   │   └── deps.rs          # Dependent app detection
│   ├── actions/
│   │   ├── mod.rs           # Actions module exports
│   │   └── launchctl.rs     # Start/stop/restart via launchctl
│   └── theme.rs             # Tokyo Night color palette
├── config/
│   └── awatcher.toml        # Optional configuration file
└── README.md
```

---

## 9. Data Flow

### 9.1 Startup Sequence

1. Initialize terminal (crossterm raw mode).
2. Create Tokio runtime.
3. Parse `arti.toml` for onion service definitions.
4. Spawn background tasks:
   - Arti process monitor (every 2s)
   - SOCKS5 connection test (every 5s)
   - Onion hostname scanner (every 10s)
   - Log file watcher (real-time)
   - Dependent app scanner (every 5s)
5. Enter UI render loop.

### 9.2 Refresh Strategy

| Data Source | Refresh Interval | Method |
|-------------|------------------|--------|
| Arti process status | 2 seconds | Polling (sysinfo) |
| SOCKS5 connectivity | 5 seconds | TCP connect test |
| Onion hostnames | 10 seconds | File read |
| Dependent apps | 5 seconds | lsof parsing |
| Log file | Real-time | Filesystem notify |
| Guardian status | 2 seconds | HTTP poll (future) |

---

## 10. Arti Control (launchctl)

### Start Arti
```bash
launchctl start org.torproject.arti
```

### Stop Arti
```bash
launchctl stop org.torproject.arti
```

### Restart Arti
```bash
launchctl kickstart -k gui/$(id -u)/org.torproject.arti
```

**Important**: These commands require the process to be owned by the current user. The TUI should verify successful execution by checking process status after the command.

---

## 11. Onion Service Detection

### Configuration Parsing

Read `arti.toml` for `[onion_services.*]` sections:

```toml
[onion_services."my-service"]
proxy_ports = [ [ 80, "127.0.0.1:8080" ] ]
```

### Hostname Discovery

After Arti starts, hostnames are written to:
```
~/antigravity/umbra/var/lib/arti/onion_services/my-service/hostname
```

The file contains the full `.onion` address.

---

## 12. Implementation Phases

### Phase 1: Core TUI Shell (Est. 4-6 hours)
- [ ] Project scaffold with Cargo.toml
- [ ] Terminal initialization and cleanup
- [ ] Basic layout with placeholder panels
- [ ] Event loop with quit handling
- [ ] Tokyo Night theme constants

### Phase 2: Arti Monitoring (Est. 4-6 hours)
- [ ] Arti process detection with sysinfo
- [ ] SOCKS5 connectivity test
- [ ] Log file tailing with notify
- [ ] State management with channels

### Phase 3: Onion Services & Dependencies (Est. 4-6 hours)
- [ ] Parse arti.toml for onion service config
- [ ] Read hostname files
- [ ] Detect dependent apps via lsof
- [ ] Display in respective panels

### Phase 4: Actions & Navigation (Est. 4-6 hours)
- [ ] Vim keybindings implementation
- [ ] Log search and filter
- [ ] Start/stop/restart actions (launchctl)
- [ ] Help overlay

### Phase 5: Privacy Integration & Polish (Est. 2-4 hours)
- [ ] Privacy indicator in header
- [ ] (Future) Guardian API integration
- [ ] Error handling and edge cases
- [ ] Final styling and alignment

**Total Estimated Effort**: 18-28 hours

---

## 13. Delegation Assessment

### Is this suitable for Gemini 3 Pro?

**Assessment: Yes, with structured prompting.**

**Strengths for delegation**:
- Mirrors `zwatcher` design (can reuse patterns).
- Well-defined data sources and parsing logic.
- Clear module structure and separation of concerns.
- Phased implementation allows incremental progress.

**Recommended delegation approach**:
1. **Provide both `zwatcher/DESIGN.md` AND `awatcher/DESIGN.md`** in context.
2. **Highlight reusable components** from zwatcher.
3. **Phase-by-phase prompts**: Start with Phase 1.
4. **Review checkpoints**: After each phase, review and adjust.

**Potential challenges**:
- Onion hostname file may not exist until Arti fully starts.
- `lsof` parsing for dependent apps can be slow.
- Guardian/Penumbra integration requires their APIs to be running.

**Mitigation**: Gracefully handle missing data; show "Unavailable" states.

---

## 14. References

- [Ratatui Documentation](https://ratatui.rs/)
- [Arti Documentation](https://arti.torproject.org/)
- [Tokyo Night Theme](https://github.com/enkia/tokyo-night-vscode-theme)
- [Sysinfo Crate](https://docs.rs/sysinfo)

---

*Document prepared for the Antigravity Station · Umbra Subsystem*