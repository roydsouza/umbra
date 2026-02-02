# Vault Drop

Ephemeral file sharing with self-destructing onion addresses.

## Concept

A CLI tool that spins up an ad-hoc onion service to share a specific file (stored in RAM) with one peer.

## Implementation

- Use Arti's ephemeral hidden service capability
- Generate one-time `.onion` address
- Self-destruct after first successful download
- File never touches disk (RAM-only)

## Value

- No permanent URL exists
- No third-party server (WeTransfer, etc.)
- Perfect forward secrecy for file transfers

## Key Location

Ephemeral (generated per-session)
