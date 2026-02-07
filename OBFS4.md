# OBFS4 — Traffic Obfuscation Guide

> **When to use**: Only when connecting from networks that actively block Tor.
> **Current status**: Compiled in, disabled by default.

*"OBFS4 makes your Tor traffic look like a cat walked across the keyboard."* — Not an official description, but accurate.

---

## Quick Start (Emergency Mode)

If you suddenly find yourself on a hostile network:

```bash
# 1. Get bridges (do this from a non-blocked network first!)
open https://bridges.torproject.org/bridges?transport=obfs4

# 2. Edit config
nano ~/antigravity/umbra/etc/arti.toml

# 3. Add your bridges (see below)

# 4. Restart Arti
launchctl kickstart -k gui/$(id -u)/org.torproject.arti
```

---

## Configuration

### Enable OBFS4 in `etc/arti.toml`

```toml
[bridges]
enabled = true

# Get these from bridges.torproject.org
bridges = [
    "obfs4 198.51.100.1:443 FINGERPRINT cert=BASE64CERT iat-mode=0",
    "obfs4 203.0.113.2:9001 FINGERPRINT cert=BASE64CERT iat-mode=0",
]
```

### Using Profiles

We have pre-configured profiles in `etc/arti-profiles.toml`:

| Profile | CGO | OBFS4 | Use Case |
|---------|-----|-------|----------|
| `default` | ✅ fallback | ❌ | Normal networks |
| `paranoid` | ✅ strict | ❌ | Maximum security |
| `obfs4` | ✅ fallback | ✅ | Censored networks |
| `paranoid-obfs4` | ✅ strict | ✅ | Max security + censorship bypass |

---

## How OBFS4 Works

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│ Your Device │────▶│ OBFS4 Bridge│────▶│ Tor Network │
└─────────────┘     └─────────────┘     └─────────────┘
       │                   │
       │ Looks like        │ Normal Tor
       │ random noise      │ from here
       ▼                   ▼
   (DPI can't tell     (Guard relay
    it's Tor)           onwards)
```

1. **Your traffic** → encrypted + obfuscated
2. **Bridge decodes** → forwards to Tor network
3. **Rest of circuit** → normal Tor (with CGO if available)

---

## Obtaining Bridges

### Method 1: Web (Easiest)
1. Go to https://bridges.torproject.org/bridges?transport=obfs4
2. Solve CAPTCHA
3. Copy the bridge lines

### Method 2: Email
Send email to `bridges@torproject.org` with body:
```
get transport obfs4
```
(Must use Gmail, Riseup, or Yahoo)

### Method 3: Telegram
Message `@GetBridgesBot` on Telegram

---

## Pros & Cons

| ✅ Pros | ❌ Cons |
|---------|---------|
| Bypasses Tor blocks | Adds latency (~50-200ms) |
| Evades DPI | Limited bridge pool |
| Works in China/Iran | Bridges can go offline |
| Traffic looks random | Slightly more complex setup |

---

## When NOT to Use OBFS4

- ✅ Your home network (Tor isn't blocked)
- ✅ Most coffee shops/airports
- ✅ Standard VPN connections

**Use OBFS4 when:**
- ❌ Corporate networks blocking Tor
- ❌ Countries with active censorship (China, Iran, Russia, etc.)
- ❌ ISPs throttling/blocking Tor
- ❌ Any network where direct Tor fails

---

## Troubleshooting

### "No bridges responding"
- Bridges may be offline or blocked
- Get fresh bridges from torproject.org
- Try multiple bridges (add 3-5)

### "Connection timeout"
- Check if port 443 is blocked (try different bridge)
- Some networks block all non-HTTP traffic

### "OBFS4 plugin not found"
- Verify Arti was built with `pt-client` feature:
  ```bash
  ./bin/build-arti full
  ```

---

## Security Notes

> [!IMPORTANT]
> OBFS4 hides *that* you're using Tor, not *what* you're doing on Tor.
> Once connected, your anonymity is the same as regular Tor.

> [!TIP]
> Combine with `paranoid` profile for maximum protection:
> CGO encryption + traffic obfuscation.

---

*Prepared for hostile network conditions. Hopefully never needed.*
