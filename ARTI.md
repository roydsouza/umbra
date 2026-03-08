# Arti Administration & Operations


**[📍 Back to Map](CONTENTS.md)**


This document defines the operational procedures for the **Arti** (Rust Tor) implementation within the Umbra station.

> [!IMPORTANT]
> **Station Standard**: Arti is the **primary and only** Tor implementation used by Umbra.
> - **Port**: **9050** (SOCKS5 proxy) - *Note: Changed from 9150 to match standard Tor ports.*
> - **Config**: `~/antigravity/umbra/etc/arti.toml`
> - **Service**: `org.torproject.arti` (macOS LaunchAgent)

## Quick Reference

| Resource | Path/Command |
| :--- | :--- |
| **Binary** | `~/antigravity/umbra/bin/arti` |
| **Configuration** | `~/antigravity/umbra/etc/arti.toml` |
| **Log File** | `~/antigravity/umbra/var/log/arti.log` |
| **Port Check** | `lsof -iTCP -sTCP:LISTEN -P | grep 9050` |
| **Service Status** | `launchctl list | grep arti` |

---

## 1. Client Configuration (Zebra, Neovim, etc.)

To route applications through Arti, you must configure them to use the **SOCKS5 proxy** and, crucially, **resolve DNS remotely** through the proxy to prevent leaks.

### Standard Configuration
- **Proxy Type**: SOCKS5
- **Host**: `127.0.0.1` (localhost)
- **Port**: `9050`
- **DNS**: Remote / Proxy DNS (Do NOT use local DNS)

### Example: Zebra (Zcash Node)
In your `zebrad.toml` configuration:
```toml
[network]
# Route all traffic through Arti
proxy = "127.0.0.1:9050"

# Initial peer connection
# Ensure you are connecting to peers that support Tor or use Tor-only peers if strictly required.
```

### Verification
Always verify that your application is actually using the proxy:
1. Start Arti.
2. Start your application.
3. Check for leakages (e.g., DNS queries appearing in system logs) or use `lsof` to ensure the app is ONLY connecting to localhost:9050.

---

## 2. Checking for Updates

Arti is tracked as a Git submodule. To check for upstream releases:

1. **Check Upstream**:
   ```bash
   cd ~/antigravity/umbra/arti
   git fetch origin
   git tag | sort -V | tail -n 5
   ```

> [!TIP]
> **Automated Workflow**: This folder contains a **Skill** (`SKILL.md`) that automates this.
> - **Just Enter**: When you explicitly open this folder with AntiGravity, it will auto-run `scripts/check_status.py`.
> - **Auto-Analysis**: It compares versions and scans the CHANGELOG for "Security" or "Breaking" alerts.
> - **One-Step Upgrade**: If you approve, it runs `scripts/deploy.sh`.

2. **Compare**: Check currently checked-out version:
   ```bash
   git describe --tags
   ```
3. **Review Changelog**:
   Check `umbra/arti/CHANGELOG.md` for critical security fixes or breaking changes before upgrading.

---

## 3. Building & Deploying

To upgrade or rebuild the local Arti binary:

```bash
# 1. Enter submodule
cd ~/antigravity/umbra/arti

# 2. Update code (if upgrading)
git checkout arti-v1.x.x  # Replace with desired tag

# 3. Build Release Binary (optimized for M5)
cargo build --release -p arti --locked 

# 4. Deploy to Station
# Stop running service first
launchctl kickstart -k gui/$(id -u)/org.torproject.arti

# Copy binary to station bin/
cp target/release/arti ~/antigravity/umbra/bin/arti

# Verify version
~/antigravity/umbra/bin/arti --version
```

---

## 4. Monitoring Status

You can monitor Arti through multiple layers:

### A. System Level (Launchd)
Check if the service is running and its PID:
```bash
launchctl list | grep arti
# Output: PID  Status  Label
#         1234 0       org.torproject.arti
```

### B. Logs
Real-time log streaming:
```bash
tail -f ~/antigravity/umbra/var/log/arti.log
```

### C. MissionControl Dashboard
The **Station Dashboard** connects to Arti via the control port/RPC (future) or by monitoring the process.
- **Status**: Shows ONLINE/OFFLINE.
- **Circuits**: Visualizes active paths through the Tor network.

---

## 5. Debugging

If Arti fails to start or connect:

1. **Check Error Logs**:
   ```bash
   tail -n 50 ~/antigravity/umbra/var/log/arti.log
   ```
2. **Verbose Mode (Manual Run)**:
   Stop the service and run manually to see debug output:
   ```bash
   launchctl stop org.torproject.arti
   ~/antigravity/umbra/bin/arti -c ~/antigravity/umbra/etc/arti.toml proxy -l debug
   ```
3. **Common Issues**:
   - **Port Conflict**: Ensure no other Tor instance is on port 9150.
   - **Clock Skew**: Tor requires accurate system time. Check date/time settings.
   - **Permissions**: Ensure `umbra/var/` is writable.

---

## 6. Onion Services

To configure a Hidden Service (Onion Service):

1. **Edit Configuration**:
   Open `~/antigravity/umbra/etc/arti.toml`:
   ```toml
   [onion_services."my-service"]
   # Map local port 80 to web server
   proxy_ports = [ [ 80, "127.0.0.1:8080" ] ]
   ```
2. **Reload Arti**:
   ```bash
   launchctl kickstart -k gui/$(id -u)/org.torproject.arti
   ```
3. **Get Hostname**:
   The `.onion` address is stored in the state directory:
   ```bash
   cat ~/antigravity/umbra/var/lib/arti/onion_services/my-service/hostname
   ```

> [!TIP]
> **Programmatic Usage**: You can also launch onion services directly from Rust code using `arti-client`. See `umbra/arti/examples/axum/axum-hello-world` for a reference implementation.

---

## 7. Resilience Configuration

Arti is configured to be **"Always On"** and self-healing.

### Layer 1: macOS LaunchAgent
- **File**: `~/Library/LaunchAgents/org.torproject.arti.plist`
- **Mechanism**: `KeepAlive = true`
- **Behavior**: If the `arti` process crashes or is killed, macOS immediately restarts it.

### Layer 2: MissionControl Supervisor
- **Component**: `MissionControl` (Tauri App)
- **Mechanism**: Use `Arti Supervisor` logic in the backend.
- **Behavior**: Monitors connectivity to the proxy port. If unresponsive, it can trigger alerts or attempt operational fixes (though LaunchAgent handles the process restart).

### Layer 3: Configuration Hardening
- **Config**: `umbra/etc/arti.toml`
- **Paths**: Uses absolute paths for logs and state to prevent CWD-dependent failures.
- **Socks**: Binds to `127.0.0.1` to prevent external network leaks.