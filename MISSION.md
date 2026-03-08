# Project Umbra: Secure & Private Communication Layer


**[📍 Back to Map](CONTENTS.md)**


Instructions for Antigravity, Gemini, and other models: please also see CGO_NOTES.md for the Arti/CGO mission.

## 1. Vision & Purpose
Umbra is the dedicated privacy and anonymity subsystem for the **EventHorizon** ecosystem. Its primary goal is to provide a "Shadow Layer" for all outgoing and incoming communications, ensuring metadata resistance and identity obfuscation. It serves as a shared utility for subordinate projects like **GravityLens** and **RedShift**.

## 2. Architectural Roadmap
* **Phase 1 (Legacy Core):** Deploy and configure the standard C-based Tor implementation as a **Hidden Service** (No Relay).
* **Phase 2 (Modernization):** Integrate **Arti** (the Rust implementation of Tor) and configure it for hidden service operations.
* **Phase 3 (Migration):** [COMPLETE] Transition primary hidden service traffic to Arti, decommissioning the legacy C-Tor implementation.

## 3. Technical Constraints & Optimizations
* **Hardware Target:** Apple Silicon M5 (MacBook Pro)
* **Tor Backend**: Standardized 100% on **Arti** (Rust). Legacy C-Tor has been decommissioned and removed.
* **Port Protocol**: Standard Port **9050** is reserved exclusively for the Arti SOCKS gateway.
* **Compilation Strategy:** - All binaries must be compiled with specific optimizations for the M5 microarchitecture (e.g., `-march=apple-m5` or appropriate LLVM target features).
    - Prioritize hardware-accelerated AES and SHA instructions for cryptographic throughput.
* **Security Posture:** - **Post-Quantum Readiness:** All implementations must be configured with PQC support (e.g., `--enable-pqc` for Tor/Arti builds).
    - **Identity Security:** Identity keys are stored within the `umbra/keys` directory. Agents must treat this directory as "Strictly Confidential".
    - **NO RELAY:** Under no circumstances should this node be configured as a public relay or exit node. It is strictly a client and hidden service host.
* **Configuration:** Maintain a custom `arti.toml`.

## 4. Operational Protocols & State Management
The agent is responsible for maintaining project state across three mandatory files:

### A. TASKS.md (The Queue)
- Maintains a prioritized list of active and upcoming engineering tasks.
- Must be updated after every session to reflect progress on compilation, testing, or migration.

### B. LISTS.md (The Ledger)
- A historical record of reported bugs, feature requests, and technical debt.
- Serves as the memory bank for user-reported issues during "vibe coding" sessions.

### C. SYNC_LOG.md (The Checkpoint)
- **Pre-Push:** The agent must log a summary of changes and validation results before pushing to GitHub.
- **Post-Pull:** Upon opening the project, the agent must read the last entry of `SYNC_LOG.md` to reconcile its internal state with the latest remote changes.

### D. Automated Rituals (The Workflow)
- All agents MUST execute the [Sync Workflow](.agent/workflows/sync.md) upon entering the project and before checkpointing.
- This include mandatory **Upstream Software Checks** for the `tor` and `arti` submodules.

## 5. Agent Constraints
- Do not attempt to write outside the `~/antigravity/umbra/` hierarchy without explicit user confirmation.
- Respect `.antigravityignore` to shield sensitive key backups.
- Always provide a "Plan Artifact" before executing complex build sequences involving cross-compilation or optimization flags.
