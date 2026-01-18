# Tasks

## Phase 1: Project Setup & Scaffolding

- [ ] Initialize Rust project with `cargo new`.
- [ ] Add core dependencies: `axum`, `tokio`, `serde`, `tower-http`, `arti-client`.
- [ ] Create initial `main.rs` with a basic "Hello World" Axum server.
- [ ] Set up `.gitignore` to exclude `target/`, `/keys`, `*.log`, and other sensitive files.
- [ ] Create `keys/` directory for storing Onion Service private keys (and add it to `.gitignore`).

## Phase 2: Core Onion Service Implementation

- [ ] Integrate `arti-client` to connect to the Tor network.
- [ ] Implement logic to launch and manage a hidden service.
- [ ] Configure the Axum server to bind only to `127.0.0.1`.

## Phase 3: Core Features

- [ ] **DeFi Balance Aggregator:** Implement a module to query blockchain RPCs via Tor to aggregate and display DeFi portfolio balances without leaking IP addresses.
- [ ] **System Status Monitor:** Create a service to gather and display key system metrics (e.g., CPU, memory, storage) from the host machine.
- [ ] **Health Data Dashboard:** Build a module to display private health metrics (e.g., glucose, sleep, exercise) synced from a personal device to a private endpoint.
- [ ] **Secure Notes Viewer:** Add a feature to read and display encrypted notes from a secure, pre-defined location.

## Phase 4: Security Hardening

- [ ] Implement header stripping middleware in Axum to remove `Server` and `Date` headers.
- [ ] Implement Onion Service Client Authentication to restrict access to authorized users/devices.
- [ ] Research and implement Vanguards for protection against traffic analysis attacks.
- [ ] Set up `cargo-audit` and `cargo-deny` to check for vulnerabilities and unwanted dependencies.
- [ ] Ensure all assets (CSS/JS) are served locally to prevent external leaks.
- [ ] Verify PQC (Post-Quantum Cryptography) handshake is enabled in the Arti configuration.

## Phase 5: UI/UX Development

- [ ] Design and implement a high-quality, dark, high-tech UI suitable for laptop use, in line with the GravityLens aesthetic.
- [ ] Create a hierarchical menu structure for the different features (Metrics, DeFi, Health, etc.).
- [ ] Implement a JSON/RPC API to allow programmatic access to the dashboard's data (e.g., for other tools like GravityLens).
- [ ] Ensure all UI elements are polished and provide a compelling user experience.

## Phase 6: Deployment & Resilience

- [ ] Create a `launchd` service file for macOS to ensure the service auto-restarts.
- [ ] Implement structured logging to a local file, ensuring IP addresses and sensitive data are masked.
- [ ] Create a health check script or internal supervisor task to periodically verify the Onion Service is reachable and healthy.
