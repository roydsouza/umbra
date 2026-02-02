# RPC Fortress

Personal Ethereum RPC tunnel exposed only via Tor Onion Service.

## Concept

Host a local Ethereum execution client (Geth/Nethermind) and expose its JSON-RPC interface exclusively through a v3 onion address.

## Implementation

- Use `arti-client` to create a hidden service tunneling to the RPC port
- Single-binary solution embedding Arti
- No external ports exposed

## Value

- Query your own node from mobile/hardware wallet anywhere
- No home IP exposure or VPN required
- Full RPC privacy for DeFi operations

## Key Location

`~/antigravity/umbra/keys/defi/rpc_fortress.key`
