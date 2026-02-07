# Umbra: Stable Tasks

> **Mission Control** 🚀 — Where we turn "wouldn't it be nice if..." into "shipped it!"
> 
> *"The journey of a thousand commits begins with a single `git add`."*

---

## ⚡ Active Sprint

### Arti Verification & CGO Testing

The shiny new Arti 2.0.0 binary is built and running, but we haven't fully kicked the tires yet.

| Task | Status | Notes |
|------|--------|-------|
| Pin to `arti-v2.0.0` | ✅ Done | Locked and loaded |
| Build with CGO + PT support | ✅ Done | Binary includes `counter-galois-onion` |
| Create `bin/build-arti` script | ✅ Done | Makes rebuilding a breeze |
| Create `bin/start-arti` script | ✅ Done | One command to rule them all |
| Create `CRYPTO.md` documentation | ✅ Done | CGO and OBFS4 explained |
| Verify Arti startup with new binary | 🔄 **In Progress** | Logs show healthy operation |
| Test CGO fallback mode | ⏳ **Pending** | Need to verify graceful `tor1` fallback |

**Why CGO Fallback Matters**: 
Only ~20% of Tor relays currently support Counter Galois Onion (Relay=2 protocol). When you can't build a full CGO circuit, Arti should gracefully fall back to legacy `tor1` encryption. Think of it like your car switching to the spare tire—not ideal, but you still get home.

---

## 🏗️ Backlog

### Operations & Monitoring

- [x] **~~Implement Log Rotation~~** — Enabled in `etc/arti.toml` with daily rotation. Console reduced to warn level.
  
- [ ] **Create `bin/stop-arti` Script** — For operational symmetry and graceful shutdowns. Nobody likes using `pkill`.

- [ ] **Monitor CGO Adoption → Alert at 50%** — Run `./bin/check-cgo-adoption` weekly. When adoption reaches 50%, consider switching to `paranoid` profile as default.
  - **Current**: ~20% (Feb 2026)
  - **Target**: 50%
  - **Action**: Enable `./bin/start-arti paranoid` as default
  - **Script**: `./bin/check-cgo-adoption --alert` (sends macOS notification)

- [ ] **Automate Arti Health Monitoring** — A cron job or launchd agent that checks port 9050 and alerts if Arti goes MIA.

- [ ] **Add Arti to `heliosphere/config/services.json`** — Integration with the broader AntiGravity service registry.

### Documentation

- [ ] **Arti Observability Guide** — A comprehensive "How to Watch Your Onion" doc:
  - Quick health check: `curl --socks5 127.0.0.1:9050 https://check.torproject.org/api/ip`
  - Log locations and what to look for
  - Process monitoring (`ps aux | grep arti`, `lsof -i :9050`)
  - Circuit introspection (when RPC becomes available)
  - Common error patterns decoder ring

### Future Enhancements

- [ ] **Secure Enclave Integration** — Store Arti's long-term identity key in Apple's Secure Enclave. The circuit keys are already ephemeral (mutating per-cell thanks to CGO), but the master identity could be hardware-bound.

- [ ] **Bridge Configuration** — Set up OBFS4 bridges for hostile network environments. Currently disabled (`bridges.enabled = false`) but ready when needed.

- [ ] **MissionControl Integration** — Add Arti supervision to the Tauri dashboard with circuit visualization.

---

## ✅ Completed

### 2026-02 Milestones

- [x] **Arti v2.0.0 Build** — Compiled from source with `counter-galois-onion` and `pt-client` features. Apple Silicon optimized.

- [x] **Profile System** — Created `etc/arti-profiles.toml` for different operational modes.

- [x] **Documentation Overhaul** — `CRYPTO.md` now explains CGO, OBFS4, and why we care about "Rugged Pseudorandom Permutations."

- [x] **Ghost Configuration Fix** — Resolved the infamous "File not found" error by creating the proper symlink.

- [x] **STATUS.md Created** — Now tracking operational health with human-readable (and mildly entertaining) reports.

---

## 📊 Quick Status

| Component | Status | Version |
|-----------|--------|---------|
| Arti | 🟢 Running | v2.0.0 |
| CGO | 🟢 Available | Fallback mode ready |
| OBFS4 | ⚪ Disabled | Ready when needed |
| Guardian | 🟢 Active | Zebra excluded (temp) |
| Zebra | 🔄 Syncing | Clearnet mode |

---

## 🎯 Weekly Focus

**This Week**: Get CGO fallback testing completed and document the results.

**Why**: We want confidence that when we're browsing through hostile networks, our traffic stays protected even if we can't find three CGO-capable relays in a row.

---

*Last updated: 2026-02-06 by AntiGravity AI*  
*"The best time to configure Tor was 20 years ago. The second best time is now."*
