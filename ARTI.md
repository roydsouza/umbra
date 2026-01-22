# Arti Administration & Monitoring

Arti is the next-generation Tor client implementation in Rust. This document outlines how to manage and monitor Arti within the Umbra station.

## 1. Standalone Administration

When running as an independent service or during manual debugging:

### Logs
Primary logs are stored in:
`/Users/rds/antigravity/umbra/arti/var/log/arti.log` (if configured) or emitted to `stdout/stderr`.

### SOCKS Proxy
By default, Arti provides a SOCKS5 proxy on:
- **`127.0.0.1:9150`** (Preferred)
- **`127.0.0.1:9050`** (Legacy compatibility)

### Manual Restart
If running via `launchctl`:
```bash
launchctl kickstart -k gui/$(id -u)/org.torproject.arti
```

---

## 2. Administration via MissionControl

MissionControl provides a unified interface for the embedded Arti instance.

### Monitoring
- **Dashboard**: Shows the "ARTI Status" (ONLINE/BOOT...).
- **Circuit Map**: Real-time visualization of active paths, including relay names, countries, and status.
- **Circuit Count**: Aggregate number of active circuits for bandwidth and connectivity health.

### Resilience (Supervisor)
MissionControl implements an **Arti Supervisor** loop. 
- If the embedded Arti client loses connection or fails to bootstrap, MissionControl will automatically attempt to restart it with an exponential backoff.
- Status is broadcast to the UI as an "Error" state if the supervisor reaches a retry limit.

### Configuration
Configuration for the embedded instance is managed via:
`~/antigravity/umbra/missioncontrol/config/missioncontrol.toml`

The `[arti]` section defines the state and cache directories:
```toml
[arti]
state_dir = "data/arti/state"
cache_dir = "data/arti/cache"
```
