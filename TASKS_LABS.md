# Ergosphere Lab: Experimental Tasks

> **Lab Focus**: Arti internals research, CGO protocol development, and bleeding-edge experiments.

---

## 🧪 Next Steps (Priority Order)

### 1. Pin Stable Arti for Comparison
- [ ] Clone `arti-v1.8-stable/` from upstream at a specific tag
- [ ] Update `.gitmodules` to track both `arti-upstream` and `arti-v1.8-stable`
- [ ] Document version differences in `LAB_MANIFEST.md`

### 2. Populate `arch-explorer` Lab
- [ ] Add Arti crate dependencies (`arti-client`, `tor-proto`)
- [ ] Implement call graph tracer using `tracing` spans
- [ ] Output DOT/Mermaid diagrams of Arti internals

### 3. CGO Protocol Research
- [ ] Review CGO paper/spec and document in `labs/cgo-prototype/README.md`
- [ ] Identify Arti extension points for CGO integration
- [ ] Prototype Counter Galois Onion handshake in Rust

### 4. Monitoring Modules
- [ ] `monitor-flow`: Wire to Arti RPC for bandwidth telemetry
- [ ] `network-topology`: Parse consensus for relay visualization

---

## ✅ Completed (2026-02)

- [x] **Ergosphere Lab Reorganization**: Evolved umbra into modular Cargo workspace
- [x] Created `modules/`: astrometrics-core, monitor-flow, network-topology
- [x] Created `labs/`: arch-explorer, cgo-prototype
- [x] Updated root `Cargo.toml` with workspace members
- [x] Created `LAB_MANIFEST.md` with preserved project context
- [x] Moved legacy docs to `legacy/`
- [x] Renamed `arti/` → `arti-upstream/`
- [x] **MissionControl DarkMatter Integration**: Node controls, unified dashboard

---

## 📋 Experiment Backlog

- [ ] **Real-time Telemetry**: Repair RPC polling for offline node tiles
- [ ] **Guardian Stream**: Replace mock leak events with real bridging
- [ ] Experiment with programmatic Arti integration (Rust)
- [ ] Benchmark CGO vs legacy AES-CTR performance
