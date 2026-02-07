# Umbra: Stable Defects

> **The Bug Zoo** 🐛 — Where we keep our digital gremlins under observation.
> 
> *"A bug in the hand is worth two in production."* — Ancient Developer Proverb

---

## 🔴 Open Defects

### DEF-001: The Great Zebra Log Split Personality Crisis

**Severity**: Medium | **Status**: Open | **First Seen**: 2026-02-05

**The Problem**: 
Our Zebra (Zcash node) has developed a case of multiple personality disorder when it comes to logging. Like a teenager who can't decide which room is "theirs," logs appear in *two* different locations:
- `darkmatter/var/log/zebrad.log` 
- `darkmatter/zcash/var/log/zebrad.log`

**Why This Matters**: 
When you're doing forensics at 2 AM and need to grep through logs, you don't want to play "Where's Waldo?" with your log files. Observability should be boring and predictable.

**Suggested Fix**: 
Consolidate logging by explicitly setting `log_path` in `zebrad.toml` and removing any duplicate configurations.

---

### ~~DEF-002: Log Rotation Needed for Arti~~ ✅ RESOLVED

**Severity**: Low | **Status**: Resolved | **Fixed**: 2026-02-06

**The Fix**: Enabled Arti's built-in log rotation in `etc/arti.toml`:
- File logging: `var/log/arti.log` with `rotate = "daily"`
- Console reduced to `warn` level (no more "consensus usable" heartbeats every 2 minutes)

*The tribbles have been contained.* 🐹

---

### DEF-003: Missing stop-arti Script

**Severity**: Low | **Status**: Open | **First Seen**: 2026-02-06

**The Problem**:
We have `start-arti` but no `stop-arti`. It's like having a car with an ignition but no brakes. Currently, stopping requires:
```bash
launchctl stop org.torproject.arti
# or
pkill arti
```

**Why This Matters**:
Operational symmetry. Also, graceful shutdowns are nice.

---

## 🟢 Resolved Defects

### DEF-000: Arti "Ghost Configuration" Error ✅

**Resolution Date**: 2026-02-05 | **Severity**: High (was blocking startup)

**The Problem**:
The `start-arti` script was throwing a "File not found" error for `${HOME}/antigravity/umbra/arti.toml`, even though our config lives in `etc/arti.toml`. Arti's `watch_configuration` feature was looking in the wrong place.

**Root Cause**:
Arti looks for configuration files in default paths when not explicitly specified. The symlink was missing.

**The Fix**:
Created symlink: `umbra/arti.toml` → `umbra/etc/arti.toml`

*And there was much rejoicing.* 🎉

---

## 📊 Defect Metrics

| Category | Open | Resolved |
|----------|------|----------|
| Critical | 0 | 0 |
| High | 0 | 1 |
| Medium | 1 | 0 |
| Low | 2 | 0 |

---

*Last updated: 2026-02-06 by AntiGravity AI*  
*"We don't have bugs, we have undocumented features."*
