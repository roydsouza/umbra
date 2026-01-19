# MissionControl: Onion-Routed Command Interface

MissionControl is a resilient, high-performance command center for the EventHorizon ecosystem. It provides a secure and private dashboard accessible via a Tor Onion Service, unifying monitoring and control of the Umbra ecosystem.

## Core Features

*   **Secure & Anonymous:** Built on Rust and Arti (the Rust implementation of Tor) for modern, memory-safe security. All connections are routed through the Tor network.
*   **High-Performance:** Asynchronous from the ground up using Axum and Tokio, optimized for Apple Silicon M5.
*   **High-Quality UI:** Features a dark, high-tech, and compelling aesthetic designed for laptop use, consistent with the visual style of GravityLens.
*   **Zero-Leak Architecture:** Designed to prevent accidental leakage of IP addresses, server details, or other identifying information.

## Integrated Services

| Service | Purpose | Endpoint |
|---------|---------|----------|
| **Arti** | Tor connectivity, Onion Services | Internal |
| **Guardian** | Network leak detection & policy | `127.0.0.1:9109` |
| **DarkMatter** | Crypto node monitoring (Zebra, Monero) | Various |
| **GravityLens** | Wallet tracking & forensics (future) | TBD |

## Tor Implementation

This project uses **Arti exclusively**. It will not use, connect to, or interact with the legacy C-Tor implementation. All Tor network connectivity is handled programmatically via the `arti-client` crate.

## Project Status

This project is in active development. The core infrastructure (Axum server, Arti bootstrap, UI pages) is implemented. See the following documents for details:

*   **[MISSION.md](MISSION.md):** The high-level vision and goals.
*   **[ARCHITECTURE.md](ARCHITECTURE.md):** The technical architecture including Guardian integration.
*   **[SECURITY.md](SECURITY.md):** The security strategy and threat mitigations.
*   **[TASKS.md](TASKS.md):** The implementation plan and priorities.

## Adjacent Projects

*   **[Umbra](../):** Parent project managing the Arti instance.
*   **[Guardian](../guardian/):** Headless network leak detector — MissionControl is its primary UI.