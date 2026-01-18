# MissionControl: Onion-Routed Command Interface

MissionControl is a resilient, high-performance "Router" interface for the EventHorizon ecosystem. It provides a secure and private "command center" accessible via a Tor Onion Service, allowing interaction with the user's ecosystem (DeFi metrics, system status, etc.) from any device.

## Core Features

*   **Secure & Anonymous:** Built on Rust and Arti (the Rust implementation of Tor) for modern, memory-safe security. All connections are routed through the Tor network.
*   **High-Performance:** Asynchronous from the ground up using Axum and Tokio, optimized for Apple Silicon M5.
*   **High-Quality UI:** Features a dark, high-tech, and compelling aesthetic designed for laptop use, consistent with the visual style of GravityLens.
*   **Zero-Leak Architecture:** Designed to prevent accidental leakage of IP addresses, server details, or other identifying information.

## Tor Implementation

This project is designed to use **Arti exclusively**. It will not use, connect to, or interact with the legacy C-Tor implementation. All Tor network connectivity is to be handled programmatically via the `arti-client` crate, which connects to the running Arti daemon. This decision is based on the project's security-first principle, leveraging Arti's memory-safe Rust implementation.

## Project Status

This project is in the pre-planning and research phase. The core architecture and security posture have been defined. See the following documents for more details:

*   **[MISSION.md](MISSION.md):** The high-level vision and goals.
*   **[ARCHITECTURE.md](ARCHITECTURE.md):** The proposed technical architecture.
*   **[SECURITY.md](SECURITY.md):** The security strategy and threat mitigations.
*   **[TASKS.md](TASKS.md):** The implementation plan and task list.

## Adjacent Projects

This service relies heavily on the **Arti** instance managed by the parent **[Umbra](../)** project for its connection to the Tor network.