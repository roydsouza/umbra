# Context: MissionControl

This project, MissionControl, originates from the need for a secure, high-performance "command center" for the user's "EventHorizon" ecosystem. The primary motivation is to enable remote interaction with personal services (like DeFi metric tracking and system status monitoring) without compromising on privacy or anonymity.

## Technical Context

*   **Parent Project:** MissionControl is a sub-project of **[Umbra](../)**, the central privacy and anonymity layer of the `antigravity` workspace. It leverages the Arti (Rust-Tor) instance provided and configured by Umbra.
*   **Technological Choice:** The selection of Rust, Axum, and Arti is deliberate.
    *   **Rust:** For its memory safety guarantees, performance, and robust ecosystem, which are critical for a security-focused service.
    *   **Axum:** A modern, ergonomic web framework built on Tokio, designed for high-performance, asynchronous services.
    *   **Arti:** The next-generation, memory-safe implementation of Tor, aligning with the project's security-first principles.

## Strategic Context

*   **Security Posture:** This project inherits the security-first, privacy-maximalist philosophy of Umbra. Key considerations include preventing IP leaks, mitigating traffic analysis, and ensuring secrets (like Onion Service keys) are never exposed. See [SECURITY.md](SECURITY.md) for a full breakdown.
*   **User Experience:** The desired user experience is a rich, high-quality interface for laptop use. It should feature a dark, high-tech aesthetic that is stylistically in sync with the GravityLens project, ensuring a compelling and cohesive feel.
