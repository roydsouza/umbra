# Services Directory

Native Rust implementations of onion services, organized by domain.

## Categories

| Directory | Domain | Services |
|-----------|--------|----------|
| `shared/` | Common | Arti wrapper, crypto traits |
| `defi/` | Cryptocurrencies & DeFi | RPC fortress, ZK relayer, atomic swap |
| `comms/` | Private Communication | Vault drop, Ghost SSH, Ontology sync |
| `stealth/` | Censorship Avoidance | Adaptive PT, Ghost RSS, Riseup bridge |

## Cargo Workspace

Consider using a Cargo workspace to compile specific groups or the entire suite:

```toml
[workspace]
members = [
    "shared",
    "defi/*",
    "comms/*",
    "stealth/*",
]
```
