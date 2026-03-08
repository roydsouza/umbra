# C-Tor (Legacy Core) - [DECOMMISSIONED]


**[📍 Back to Map](../CONTENTS.md)**


> [!WARNING]
> This implementation has been officially decommissioned as of 2026-01-21. 
> The project has standardized exclusively on **Arti** (Rust). 
> All binary artifacts and the `tor/` submodule have been removed.

---

## 🏗️ Archive Information

- **Source Code**: Located as a Git submodule in `umbra/tor/`.
- **Upstream**: Official Tor Project repository (`https://gitlab.torproject.org/tpo/core/tor.git`).
- **Build Configuration**: Compiled specifically for the **Apple Silicon M5 architecture** using a custom prefix.
- **Build Prefix**: `/usr/local/tor-m5`
- **Installation Command (Reference)**:
  ```bash
  ./configure --prefix=/usr/local/tor-m5 --enable-pqc --with-openssl-dir=$(brew --prefix openssl)
  make
  sudo make install
  ```

---

## 🚀 Binary & Configuration Locations

- **Binary**: `/usr/local/tor-m5/bin/tor`
- **Main Configuration (torrc)**: `/usr/local/tor-m5/etc/tor/torrc`
- **Data Directory**: `/usr/local/tor-m5/var/lib/tor`
- **Logs**: `/usr/local/tor-m5/var/log/tor/notices.log`
- **Launch Agent (macOS)**: `~/Library/LaunchAgents/org.torproject.tor.plist`

---

## 🔌 Network Ports

| Service | Port | Binding | Description |
|---------|------|---------|-------------|
| **SOCKS Proxy** | `9050` | `127.0.0.1` | Main Tor proxy for applications |
| **SOCKS Proxy (Iso)** | `9051` | `127.0.0.1` | Isolated destination port proxy |
| **ORPort** | `9001` | `0.0.0.0` | Inbound relay traffic |
| **ControlPort** | `9052` | `127.0.0.1` | Tor controller port |
| **obfs4 PT** | `9002` | `0.0.0.0` | Pluggable transport bridge port |

---

## 🧅 Hidden Service Identity

- **Onion Address**: [Stored in umbra/keys/ctor/ONION_ADDRESS]
- **Virtual Port**: `80` (mapped to `127.0.0.1:8080`)
- **Master Keys (Root of Trust)**: Replicated to `umbra/keys/ctor/hidden_service/`
- **Relay Keys**: Replicated to `umbra/keys/ctor/relay/`

> [!IMPORTANT]
> The primary identity keys are now safely archived in the `umbra/keys/ctor/` directory. This allows for total identity reuse when migrating to **Arti**.

---

## 🛠️ Administration & Monitoring

### Turning On/Off
Tor is managed by a macOS Launch Agent:
- **Start**: `launchctl load ~/Library/LaunchAgents/org.torproject.tor.plist`
- **Stop**: `launchctl unload ~/Library/LaunchAgents/org.torproject.tor.plist`
- **Restart**: `killall -HUP tor` (Sends a signal to reload configuration)

### Monitoring Progress
Follow the notice logs to see sync progress and circuit establishment:
```bash
tail -f /usr/local/tor-m5/var/log/tor/notices.log
```

---

## 📋 Logging & Monitoring

### Log Location
- **Notices Log**: `/usr/local/tor-m5/var/log/tor/notices.log`

### Monitoring (Real-time)
To watch C-Tor's progress:
```bash
tail -f /usr/local/tor-m5/var/log/tor/notices.log
```

### Log Management
- **Warning**: C-Tor logs in this custom prefix are **not** automatically rotated by the system.
- **Manual Cleanup**:
  ```bash
  true > /usr/local/tor-m5/var/log/tor/notices.log
  ```

---

## 🤝 Coexistence with Arti

Arti is currently configured in `umbra/arti.toml` to listen on **port 9150** (SOCKS). C-Tor uses **port 9050**.
- This separation allows you to run both simultaneously for testing.
- **Hidden Service Transition**: Once Arti is fully configured, disable the `HiddenServiceDir` in C-Tor's `torrc` and enable it in `arti.toml` using the keys archived in `umbra/keys`.

---

## 🛡️ Protecting Traffic

To route application traffic through this C-Tor instance, configure your application (e.g., Browser, CLI tool) to use:
- **SOCKS5 Host**: `127.0.0.1`
- **Port**: `9050`

For SSH through Tor:
```bash
ssh -o ProxyCommand='nc -X 5 -x 127.0.0.1:9050 %h %p' user@[Address in keys/ctor/ONION_ADDRESS]
```

---

## 🔄 Upgrading & Upstream Tracking

### Detecting Upstream Changes
You can check for upstream changes across both submodules from the Project Umbra root:
```bash
# Check current submodule status against HEAD
git submodule status

# Fetch and see what's changed upstream
git submodule update --remote --dry-run
```

### Performing the Upgrade
1. **Pull**: `cd umbra/tor && git pull origin main`
2. **Re-Initialize Build**: `./autogen.sh && ./configure --prefix=/usr/local/tor-m5`
3. **Compile**: `make`
4. **Deploy**: `sudo make install`
5. **Reload**: `killall -HUP tor`