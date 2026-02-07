---
description: Generate an Arti status report and prepend it to STATUS.md
---

# /status-report — Arti Health Check Workflow

This workflow generates a comprehensive status report for Arti and prepends it to `umbra/STATUS.md`.

## Prerequisites
- Must be run from the `umbra` directory (or a subdirectory)

## Steps

### 1. Review Arti Logs
// turbo
```bash
tail -100 ~/antigravity/umbra/var/log/arti.err.log
```

Look for:
- `INFO` messages showing normal operation (consensus updates, bootstrapping)
- `WARN` messages indicating issues (circuit failures, connection problems)
- `ERROR` messages (critical failures)

### 2. Check Arti Process Status
// turbo
```bash
launchctl list | grep arti
```

### 3. Verify Port Listening
// turbo
```bash
lsof -iTCP -sTCP:LISTEN -P | grep 9050 || echo "Port 9050 not listening"
```

### 4. Test Tor Connectivity (Optional)
```bash
curl --socks5 127.0.0.1:9050 https://check.torproject.org/api/ip
```

### 5. Generate Status Report

Create a new status entry with the current date/time as heading. Include:
- Overall status (🟢 OPERATIONAL / 🟡 DEGRADED / 🔴 DOWN)
- Version and configuration summary
- CGO (Counter Galois Onion) status
- Notable warnings from logs
- Recommendations

### 6. Prepend to STATUS.md

**IMPORTANT**: New reports go at the TOP of the file (after the header section).

The file structure should be:
```markdown
# Umbra Status Reports
> Header text...
---
## 📡 [NEW DATE/TIME] — "[Clever Title]"
[New report content]
---
## 📡 [PREVIOUS DATE/TIME] — "[Previous Title]"
[Previous report content]
```

### 7. Update DEFECTS.md (If Issues Found)

If any `ERROR` or persistent `WARN` patterns are found, add them to `umbra/DEFECTS.md`.

### 8. Update TASKS.md (If Enhancements Identified)

If analysis reveals improvement opportunities, add them to the backlog in `umbra/TASKS.md`.

## Example Status Entry

```markdown
## 📡 2026-02-06 16:46 PST — "The Onions Are Healthy"

**Inspector**: AntiGravity AI  
**Mood**: ☕ Cautiously Optimistic

### Executive Summary
🟢 **Overall Status: OPERATIONAL**

| Metric | Value | Status |
|--------|-------|--------|
| **Version** | 2.0.0 | 🟢 Current |
| **SOCKS5 Port** | 9050 | 🟢 Listening |
...
```
