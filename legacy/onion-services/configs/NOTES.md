# Configuration Directory

TOML-based configuration for orchestrating multiple onion services.

## Files

| File | Purpose |
|------|---------|
| `services.toml` | Master list of active `.onion` services with identity paths and ports |
| `networking.toml` | Shared Pluggable Transport (obfs4/Snowflake) configuration |

## Example `services.toml`

```toml
[[services]]
name = "Umbra-Vault"
identity_path = "../../keys/comms/vault.key"
port = 8080

[[services]]
name = "DeFi-RPC"
identity_path = "../../keys/defi/rpc.key"
port = 8545
```
