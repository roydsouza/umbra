# Onion Services

This directory contains the infrastructure for hosting Tor Onion v3 services using the Arti (Rust) client.

## Structure

| Directory | Purpose |
|-----------|---------|
| `configs/` | TOML configuration for multi-tenant service orchestration |
| `services/` | Native Rust service implementations |
| `logs/` | Centralized service logs |
| `tmp/` | Unix domain sockets for IPC |

## Key Vault

Keys are stored separately in `~/antigravity/umbra/keys` (git-ignored).

## Related Documentation

- [ONION_IDEAS.md](../ONION_IDEAS.md) - Creative deployment concepts
- [ONION_SERVER.md](../ONION_SERVER.md) - Server architecture guidance
- [FOLDER_HIERARCHY.md](../FOLDER_HIERARCHY.md) - Structure rationale
- [MKP224O.md](../MKP224O.md) - Vanity address generation
