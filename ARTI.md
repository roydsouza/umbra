# Arti (Next-Gen Rust Core)

This document provides a comprehensive overview of the Arti (Rust-based Tor) implementation as configured and deployed in Project Umbra.

---

## 🏗️ Build & Installation

### Build Process
Arti was built from the official source submodule in `umbra/arti/` specifically for the **Apple Silicon M5**.
- **Build Tool**: Cargo (Rust package manager)
- **Profile**: Release (optimized)
- **Command**: `cargo build --release`

### Installation Path
To maintain a portable and localized workspace, Arti is installed directly within the `umbra` directory:
- **Binary**: `/Users/rds/antigravity/umbra/bin/arti`
- **Config**: `/Users/rds/antigravity/umbra/arti.toml`
- **Data (State)**: `/Users/rds/antigravity/umbra/var/lib/arti`
- **Cache**: `/Users/rds/antigravity/umbra/var/cache/arti`

---

## 🔌 Network Ports

Arti is configured to coexist with C-Tor by using the `91xx` port range:

| Service | Port | Binding | Description |
|---------|------|---------|-------------|
| **SOCKS Proxy** | `9150` | `127.0.0.1` | Main Arti proxy |
| **RPC / Control** | `9152` | `127.0.0.1` | Proposed future RPC port |

---

## 🔐 Identity & Keys

Identity keys are stored in a centralized, hierarchical structure to separate them from legacy C-Tor keys.

- **Storage Location**: `/Users/rds/antigravity/umbra/keys/arti/`
- **Internal Link**: Symlinked into the Arti state directory at `umbra/var/lib/arti/keystore`.
- **Permissions**: Strictly `700` (directory) and `600` (files).

### Onion Services
To establish a new Hidden Service address with Arti, we add an entry to `arti.toml`:
```toml
[onion_services."event-horizon"]
address_filter = "auto"
proxy_ports = [["80", "127.0.0.1:8080"]]
```
Once started, Arti will generate the keys in the keystore and the onion address will be accessible via the `keystore` file system.

---

## 🤝 Coexistence with C-Tor

- **Network**: Both can run simultaneously since C-Tor uses `9050` and Arti uses `9150`.
- **Filesystem**: Configurations and binaries are separated in different folders (`/usr/local/tor-m5` vs `umbra/bin`).
- **Identity**: Identity keys are archived separately in `umbra/keys/ctor` and `umbra/keys/arti`.

---

## 🛠️ Administration & Monitoring

### Turning On/Off
Managed via a custom Launch Agent:
- **Start**: `launchctl load ~/Library/LaunchAgents/org.torproject.arti.plist`
- **Stop**: `launchctl unload ~/Library/LaunchAgents/org.torproject.arti.plist`
- **Logs**:
  ```bash
  tail -f /Users/rds/antigravity/umbra/var/log/arti.log
  ```

---

## 📋 Logging & Monitoring

### Log Locations
- **Standard Output**: `/Users/rds/antigravity/umbra/var/log/arti.log`
- **Error Log**: `/Users/rds/antigravity/umbra/var/log/arti.err`

### Monitoring (Real-time)
To watch Arti's progress (syncing, circuit establishment):
```bash
tail -f /Users/rds/antigravity/umbra/var/log/arti.log
```

### Log Rotation & Cleanup
- **Automatic**: Configured in `arti.toml` for **daily rotation** with a retention count of 5.
- **Manual Cleanup**:
  ```bash
  true > /Users/rds/antigravity/umbra/var/log/arti.log
  ```

---

## 💻 Integration

### Network Traffic
Configure your applications to use:
- **Proxy**: `SOCKS5`
- **Host**: `127.0.0.1`
- **Port**: `9150`

### Programmatic Integration (Rust)
Arti is uniquely designed as a set of libraries. You can integrate it directly into your own Rust projects (like `darkmatter` or `gravitylens`) by adding it as a dependency:
```toml
[dependencies]
arti-client = "1.9.0"
tor-rtcompat = "1.9.0"
tokio = { version = "1", features = ["full"] }
```
Example code to create a proxy-capable client:
```rust
let config = TorClientConfig::default();
let client = TorClient::create_bootstrapped(config).await?;
let stream = client.connect(("[address].onion", 80)).await?;
```

---

## 🔄 Upstream Tracking & Updates

### Detecting Changes
Check the status of the submodule:
```bash
git submodule status arti/
```
To check for *new* upstream versions without pulling:
```bash
cd umbra/arti
git fetch origin
git log HEAD..origin/main --oneline
```

### Proactive Notification
- **GitHub Watch**: It is recommended to "Watch" the `arti` repository on GitLab/GitHub for Release notifications.
- **Agent Check**: The AI assistant will check submodule status during every "Update Documentation" task.

### Performing an Update
1. **Pull**: `cd umbra/arti && git pull origin main`
2. **Re-build**: `cargo build --release`
3. **Commit**: `git add arti && git commit -m "Upgrade Arti to latest"`
4. **Restart**: `killall arti` (or use launchctl)
