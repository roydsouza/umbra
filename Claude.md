# Project Umbra Context for Claude

This document provides a consolidated context for the Project Umbra, intended for AI agents like Claude.

## 1. Project Overview

Project Umbra serves as the dedicated privacy and anonymity subsystem for the user's "EventHorizon" ecosystem. Its primary goal is to establish a "Shadow Layer" for all communications, ensuring metadata resistance and identity obfuscation for other projects within the `antigravity` workspace (e.g., DarkMatter, GravityLens). It specifically focuses on managing and integrating Tor implementations.

## 2. Architectural Roadmap and Strategy

Umbra follows a three-phase roadmap for evolving its privacy layer:
*   **Phase 1 (Legacy Core):** Deployment and configuration of the standard C-based Tor implementation, primarily as a Hidden Service (No Relay).
*   **Phase 2 (Modernization):** Integration and configuration of Arti (the Rust implementation of Tor) for hidden service operations.
*   **Phase 3 (Migration):** Gradual transition of primary hidden service traffic to Arti as it achieves feature parity and stability, with eventual decommissioning of the legacy C-Tor implementation.

## 3. Technical Constraints and Optimizations

*   **Hardware Target:** Optimized for Apple Silicon M5 (MacBook Pro), leveraging hardware-accelerated AES and SHA instructions for cryptographic throughput. This includes specific M5 microarchitecture compilation optimizations (`-C target-cpu=native` for Rust, and equivalent for C builds).
*   **Security Posture:**
    *   **Post-Quantum Readiness:** Implementations (Tor/Arti) are configured with Post-Quantum Cryptography (PQC) support.
    *   **Identity Security:** Sensitive identity keys are stored in the git-ignored `umbra/keys` directory, treated as "Strictly Confidential."
    *   **NO RELAY:** The node is strictly a client and hidden service host; it must never be configured as a public relay or exit node.
*   **Configuration:** Custom `torrc` and `arti.toml` files are maintained, ensuring `ContactInfo` in `torrc` avoids leaking PII.

## 4. Key Components and Files

*   **`arti/`:** Git submodule for the Arti (Rust Tor) source.
*   **`tor/`:** Git submodule for the C-based Tor source.
*   **`ARTI.md`:** Documentation for the local Arti instance.
*   **`C-TOR.md`:** Documentation for the legacy C-based Tor installation.
*   **`arti.toml`:** Configuration file for Arti.
*   **`torrc`:** Configuration file for C-Tor.
*   **`umbra/keys/`:** Git-ignored directory for sensitive identity keys.

## 5. Operational Protocols and Agent Interaction

*   **State Management:** Agents are responsible for maintaining project state across `TASKS.md` (active queue), `LISTS.md` (historical ledger), and `SYNC_LOG.md` (checkpoint).
*   **Automated Rituals:** The `.agent/workflows/sync.md` workflow is mandatory upon project entry and before checkpointing, including upstream software checks for `tor` and `arti` submodules.
*   **Agent Constraints:**
    *   Do not write outside `~/antigravity/umbra/` without explicit user confirmation.
    *   Respect `.antigravityignore` to shield sensitive key backups.
    *   Provide a "Plan Artifact" before executing complex build sequences.
