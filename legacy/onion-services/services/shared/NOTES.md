# Shared Library

Common utilities shared across all onion services.

## Planned Components

- **Arti Wrapper**: Simplified API for creating/managing onion services
- **Crypto Traits**: Unified interfaces for key management
- **Config Loader**: Parse `services.toml` and `networking.toml`
- **Telemetry**: Metrics export for `awatcher` integration

## Usage

All service crates should depend on this:

```toml
[dependencies]
shared = { path = "../shared" }
```
