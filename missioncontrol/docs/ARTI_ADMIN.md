# Arti Administration Guide

> **MissionControl → Onion Services & Arti**
> 
> This guide covers monitoring and controlling Arti on EventHorizon (M5 MacBook Pro).

---

## Quick Start

```bash
# Start MissionControl (includes embedded Arti)
cd ~/antigravity/umbra/missioncontrol
cargo run

# Dashboard: http://127.0.0.1:3030
# Services:  http://127.0.0.1:3030/services
```

---

## Directory Structure

| Path | Purpose |
|------|---------|
| `umbra/arti.toml` | Main configuration |
| `umbra/keys/arti/` | Onion service private keys |
| `umbra/var/lib/arti/` | Persistent state |
| `umbra/var/cache/arti/` | Consensus cache |
| `umbra/var/log/arti.log` | Logs (rotated daily) |

---

## Configuration (arti.toml)

### SOCKS Proxy
```toml
[proxy]
socks_listen = "127.0.0.1:9150"
```

### Onion Service
```toml
[onion_services."event-horizon"]
enabled = true
# Keys stored in umbra/keys/arti/event-horizon/
[onion_services."event-horizon".proxy_ports]
# Virtual port → Local target
80 = ["127.0.0.1:3030"]
```

### Client Authorization
```toml
[onion_services."event-horizon".client_auth]
# Restrict access to authorized clients only
required = true
# Authorized clients defined in separate file
```

---

## Common Operations

### View Logs
```bash
tail -f ~/antigravity/umbra/var/log/arti.log
```

### Check Status (via API)
```bash
curl http://127.0.0.1:3030/api/arti/status
```

### Restart Arti
Use the MissionControl UI: **Services → Restart** button

Or programmatically (when API is wired):
```bash
curl -X POST http://127.0.0.1:3030/api/arti/restart
```

### Rebuild Circuits
```bash
curl -X POST http://127.0.0.1:3030/api/arti/circuits/rebuild
```

---

## Onion Service Management

### Create New Service
1. Open **Services** page in MissionControl
2. Click **+ New Service**
3. Enter nickname, ports, and auth settings
4. The configuration will be added to `arti.toml`

### Get Onion Address
After Arti starts with the new service:
```bash
cat ~/antigravity/umbra/keys/arti/[nickname]/hostname
```

### Client Authorization Setup
1. Generate keypair on the *client* machine
2. Add public key to service config
3. Distribute private key to authorized clients

---

## C-Tor Migration

Since Arti is becoming the primary Tor implementation:

1. **Stop C-Tor**:
   ```bash
   brew services stop tor
   ```

2. **Update port references**:
   - Old C-Tor SOCKS: `9050`
   - New Arti SOCKS: `9150`

3. **Migrate onion services**:
   - Copy keys from `C-Tor` HiddenServiceDir to `umbra/keys/arti/`
   - Update `arti.toml` with service configuration

4. **Verify**:
   - Test with Tor Browser pointing to your `.onion`

---

## Troubleshooting

| Symptom | Solution |
|---------|----------|
| Bootstrap stuck | Check internet connectivity, try different guard |
| High latency | Rebuild circuits via UI |
| Service unreachable | Verify `arti.toml` config, check keys directory |
| Permission denied | Ensure `keys/` directory has correct permissions |

---

## Security Checklist

- [ ] Keys directory is git-ignored
- [ ] Client auth enabled for sensitive services
- [ ] Logs don't contain sensitive data
- [ ] SOCKS proxy bound to localhost only
