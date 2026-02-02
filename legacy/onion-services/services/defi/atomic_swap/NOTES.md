# Atomic Swap Coordinator

P2P cross-chain atomic swap service (e.g., ETH/BTC).

## Concept

A lightweight Rust binary using Arti to coordinate swaps directly between peers, eliminating centralized matching engines.

## Implementation

- Distributed "order book" as a set of hidden services
- HTLC-based atomic swap protocol
- Peer discovery via onion addresses
- No permanent web presence

## Value

- Fully decentralized exchange
- No KYC or registration
- Censorship-resistant trading

## Key Location

`~/antigravity/umbra/keys/defi/atomic_swap.key`
