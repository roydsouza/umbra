# MISSION: MissionControl

## 1. Vision & Purpose
MissionControl is a resilient, high-performance "Router" interface for the EventHorizon ecosystem. Its purpose is to provide a secure, private, and interactive "command center" accessible via a Tor Onion Service. This allows the user to interact with their ecosystem (viewing DeFi metrics, system status, secure notes) from any device, anywhere, with maximum anonymity.

## 2. Core Directives
*   **Privacy First:** All design and implementation choices must prioritize user privacy and metadata resistance. The service must not leak any information about the host system, user's IP address, or location.
*   **High Performance:** The service must be fast and responsive, leveraging Rust's asynchronous capabilities and optimized for the Apple Silicon M5 architecture.
*   **Compelling UI/UX:** The interface must be a high-quality, compelling experience for laptop use, featuring a dark, high-tech aesthetic consistent with the GravityLens project.
*   **Resilience:** The service must be robust, with automated restart capabilities to ensure it remains available without manual intervention.

## 3. Technical & Security Mandates
*   **Language:** Rust (Stable).
*   **Framework:** Axum web server.
*   **Anonymity Layer:** Arti (the Rust implementation of Tor).
*   **Zero-Leak Architecture:** The service must only bind to localhost, strip identifying HTTP headers, and serve all assets locally.
*   **Post-Quantum Readiness:** The underlying Arti connection must be configured to use PQC-hybrid handshakes for the Onion Service.
*   **Secret Management:** Private keys for the Onion Service must be stored in a git-ignored directory and never committed to version control.
