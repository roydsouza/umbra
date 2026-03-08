# Architecture

**[📍 Back to Map](CONTENTS.md)**


`awatcher` is a **Rust-native Terminal User Interface (TUI)** that provides real-time observability and control for Arti and the Umbra privacy layer.

## Technology Stack

| Component | Technology |
|-----------|------------|
| **Language** | Rust |
| **TUI Framework** | Ratatui + Crossterm |
| **Async Runtime** | Tokio |
| **HTTP Client** | Reqwest |
| **Process Monitor** | Sysinfo |
| **Log Watching** | Notify |

## Data Sources

### Local
- **Arti Process**: Via `sysinfo` crate (running state, PID, CPU, Memory).
- **Arti Logs**: `~/antigravity/umbra/var/log/arti.log` (tailed in real-time).
- **SOCKS Proxy**: `127.0.0.1:9050` (connection test for health).
- **Config File**: `~/antigravity/umbra/arti.toml` (parse onion service definitions).
- **State Directory**: `~/antigravity/umbra/var/lib/arti/` (onion service hostnames).
- **Guardian API**: (Future) Real-time leak detection events.
- **Penumbra API**: (Future) DNS query statistics.

### Dependent Applications
Track applications routing through SOCKS5:
- `zebrad` (Zcash node)
- `zaino` (Zcash indexer)
- `monerod` (Monero node, if running)

## Core Components

```
┌─────────────────────────────────────────────────────────────┐
│                      awatcher TUI                           │
├─────────────────────────────────────────────────────────────┤
│  UI Layer          │ Ratatui widgets (header, panels, logs) │
├────────────────────┼────────────────────────────────────────┤
│  State Manager     │ App state, event handling              │
├────────────────────┼────────────────────────────────────────┤
│  Data Collectors   │ Process, SOCKS, Logs, Config Parser    │
├────────────────────┼────────────────────────────────────────┤
│  Action Handlers   │ Start/Stop/Restart Arti (launchctl)    │
└─────────────────────────────────────────────────────────────┘
```

## Design Principles

1. **Tokyo Night Harmony**: Visual palette matches Ghostty theme.
2. **Vim-First Navigation**: All keybindings follow Vim/NeoVim conventions.
3. **Real-time Updates**: Log streaming and metrics refresh in real-time.
4. **Action-Capable**: Start, stop, restart Arti and view circuit details.
5. **Privacy-Focused**: Prominently displays privacy indicators.

## Integration Points

- **grav CLI**: Will be registered as `grav status awatcher` and `grav start awatcher`.
- **zwatcher**: Shares design language and keybindings for consistent UX.
- **MissionControl**: Complements the GUI dashboard with terminal-native monitoring.

## Visual Alignment

`awatcher` and `zwatcher` share:
- Same Tokyo Night color palette.
- Same keyboard bindings.
- Same layout philosophy (header + panels + logs).
- Same Vim-style navigation.

This ensures muscle memory transfers between tools.

See `DESIGN.md` for comprehensive implementation details.