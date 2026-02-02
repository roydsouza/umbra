# Ghost SSH

Secure SSH-over-Tor bridge for air-gapped remote terminal access.

## Concept

Expose a terminal on a secure home machine via onion service, bypassing NAT and firewalls without opening ports.

## Implementation

- Arti creates persistent hidden service
- SSH daemon listens only on localhost
- Onion service tunnels to SSH port
- Optional: Client authorization for access control

## Value

- Reach "Umbra" server behind any firewall
- End-to-end encryption (Tor + SSH)
- No port forwarding required
- Hidden from network scans

## Key Location

`~/antigravity/umbra/keys/comms/ghost_ssh.key`
