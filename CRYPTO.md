# Umbra Cryptography Configuration

**[📍 Back to Map](CONTENTS.md)**


> **Purpose**: Document Arti crypto settings, CGO support, and pluggable transports.

---

## Pinned Version

| Component | Version | Tag |
|-----------|---------|-----|
| Arti | 2.0.0 | `arti-v2.0.0` |
| tor-llcrypto | 0.39.0 | — |

---

## Counter Galois Onion (CGO)

CGO is an improved relay cell encryption scheme with:
- **Tagging Resistance**: Non-malleable encryption
- **Forward Secrecy**: Keys updated per-cell
- **Authenticated Encryption**: UIV+ construction

### Modes

| Mode | Behavior |
|------|----------|
| `disabled` | Standard AES-CTR (default) |
| `fallback` | Try CGO, fall back to AES-CTR if relays don't support |
| `strict` | Require CGO, fail if unavailable |

### Usage

```bash
# Build with CGO support
./bin/build-arti cgo

# Run with fallback
./bin/start-arti cgo-fallback

# Run strict (experimental)
./bin/start-arti cgo-strict
```

---

## Pluggable Transports (OBFS4)

OBFS4 disguises Tor traffic to bypass censorship.

### Configuration

1. Build with PT support:
   ```bash
   ./bin/build-arti obfs4
   ```

2. Add bridge lines to `etc/arti-profiles.toml`:
   ```toml
   [profiles.obfs4]
   bridge_lines = [
       "obfs4 IP:PORT FINGERPRINT cert=CERT iat-mode=0"
   ]
   ```

3. Start:
   ```bash
   ./bin/start-arti obfs4
   ```

---

## Build Profiles

| Profile | CGO | OBFS4 | Command |
|---------|-----|-------|---------|
| default | ❌ | ❌ | `./bin/build-arti` |
| cgo | ✅ | ❌ | `./bin/build-arti cgo` |
| obfs4 | ❌ | ✅ | `./bin/build-arti obfs4` |
| full | ✅ | ✅ | `./bin/build-arti full` |

---

## Verification

```bash
# Check version and features
./bin/arti --version

# Verify no unexpected C dependencies
otool -L ./bin/arti

# Test config
./bin/arti proxy -c etc/arti.toml --config-test
```