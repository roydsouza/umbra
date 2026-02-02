# Riseup Bridge

Onion-native mail proxy for Riseup SMTP/IMAP.

## Concept

Local mail proxy that forces all SMTP/IMAP traffic to Riseup's `.onion` endpoints using Arti.

## Implementation

- Local SMTP/IMAP server on localhost
- Tunnel to Riseup's onion addresses
- Standard mail client connects locally
- Traffic never leaves Tor network

## Value

- Eliminates exit node sniffing risk
- Email metadata protected by onion routing
- Compatible with any standard mail client

## Riseup Onion Endpoints

- SMTP: `riseup7.....onion:465`
- IMAP: `riseup7.....onion:993`

## Key Location

Configuration-based (uses Riseup credentials)
