# Ghost RSS

Anonymous RSS feed aggregator served over onion service.

## Concept

Self-hosted RSS reader that fetches all feeds via Tor and serves the UI over a hidden service.

## Implementation

- Fetch feeds through Tor circuits
- Local SQLite for article storage
- Web UI accessible only via `.onion`
- Optional: Multi-user with auth

## Value

- Prevents news sites from tracking your interests
- CDNs cannot correlate your IP with topics
- Access from any Tor-capable device

## Key Location

`~/antigravity/umbra/keys/stealth/ghost_rss.key`
