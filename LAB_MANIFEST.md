# Ergosphere Lab Manifest

**[📍 Back to Map](CONTENTS.md)**


**The research lab for Arti and CGO (Counter Galois Onion).**

---

## 1. Core Concepts

### Umbra
> "The Shadow Layer" — Privacy and anonymity subsystem for the Antigravity ecosystem.

- **Purpose**: Metadata resistance, identity obfuscation for all station communications.
- **Core Technology**: Arti (Rust-based Tor implementation).
- **Role**: Shared utility for subordinate projects (DarkMatter, GravityLens, RedShift).

### DarkMatter
> Crypto node monitoring and secure transaction layer.

- **Location**: `~/antigravity/darkmatter/`
- **Components**: Zebra (Zcash), Monero nodes.
- **Integration**: Uses Umbra as its anonymity gateway.

### GravityLens
> Forensics and wallet tracking research.

- **Purpose**: Blockchain analysis with privacy-preserving data collection.
- **Future**: Will use Umbra for anonymous scraping and onion-native APIs.

### Ergosphere (This Lab)
> The research namespace for Arti internals and CGO protocol development.

- **Astrometrics**: Command & Control (C2) subsystem using CGO.
- **ergosphere-core**: Shared Rust utilities for all crates.

---

## 2. Project Intent

### DeFi Backup Plan
This station serves as a personal infrastructure for decentralized finance resilience:
- Self-hosted RPC nodes (Zcash, Monero).
- Privacy-preserving transaction pathways.
- No reliance on centralized exchanges or custodians.

### Cryptography Research
- Post-Quantum Cryptography (PQC) migration pathways for Tor/Arti.
- ZK-SNARKs integration with DarkMatter for shielded transactions.
- CGO (Counter Galois Onion) protocol experimentation.

---

## 3. Environment Specifications

| Spec | Value |
|------|-------|
| **Hardware** | Apple Silicon M5 (MacBook Pro) |
| **Location** | Mountain View, CA |
| **Compilation** | `-march=apple-m5`, hardware AES/SHA acceleration |
| **Security** | PQC-ready builds, Secure Enclave key storage |

### Local Health Monitor (Future Module)
- System resource monitoring integrated with Guardian.
- Health-aware routing decisions for Arti circuits.

---

## 4. Architectural Evolution

| Phase | Description | Status |
|-------|-------------|--------|
| **Phase 1** | Legacy C-Tor | ✅ Decommissioned |
| **Phase 2** | Arti modernization | ✅ Complete |
| **Phase 3** | Ergosphere Lab | 🔄 In Progress |
| **Phase 4** | CGO integration | 📋 Planned |

---

## 5. Active Components

| Component | Path | Purpose |
|-----------|------|---------|
| `arti-upstream/` | Arti submodule | Upstream Tor/Arti source |
| `modules/astrometrics-core/` | Shared lib | SE-backed crypto utilities |
| `modules/monitor-flow/` | Module | Traffic and bandwidth monitoring |
| `modules/network-topology/` | Module | Peer/bridge health visualization |
| `labs/arch-explorer/` | Lab | Arti call graph analysis |
| `labs/cgo-prototype/` | Lab | CGO protocol experiments |
| `missioncontrol/` | Service | Admin dashboard |
| `guardian/` | Service | Network shield |
| `penumbra/` | Service | DNS privacy forwarder |
| `tools/` | Scripts | Shell/vibe-coding helpers |

---

## 6. Archived Content

All legacy documentation and deprecated tools are preserved in `legacy/`:
- C-Tor docs, mkp224o, awatcher, onion-services.