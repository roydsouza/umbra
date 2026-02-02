# Adaptive PT

Automatic pluggable transport switching for censorship resistance.

## Concept

Monitor network conditions and automatically switch Arti between obfs4 and Snowflake based on handshake success rates.

## Implementation

- Probe both transport types periodically
- Track success/failure metrics
- Auto-fallback on connection failures
- Configurable priority order

## Value

- Maintains continuous connection
- Adapts to firewall targeting specific obfuscation
- No manual intervention required

## Key Location

Configuration-based (no persistent identity key)
