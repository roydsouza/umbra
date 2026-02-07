# Mission

`awatcher` provides deep observability and real-time control for the Arti anonymity network, ensuring the station's shadow layer is healthy, responsive, and leak-free.

## Objectives

### Primary
- **Real-time Arti Monitoring**: Track process health, circuit establishment, and bandwidth usage.
- **Onion Service Management**: View status and control locally hosted `.onion` services.
- **Dependent Application Tracking**: Monitor applications routing traffic through Arti (Zebra, Guardian, etc.).

### Secondary
- **Privacy Verification**: Active detection of DNS leaks and clearnet traffic via integration with Guardian/Penumbra.
- **Circuit Visualization**: Display active circuits with relay information (guard, middle, exit).
- **Log Aggregation**: Scrollable, filterable view of Arti logs and errors.

### Tertiary
- **Alerting**: Notify when circuits fail, onion services go offline, or privacy violations detected.
- **Action-Capable**: Start, stop, restart Arti and related services from within the TUI.

## Alignment
`awatcher` is the Umbra counterpart to `zwatcher` (DarkMatter/Zcash). Together, they form the station's observability layer for privacy-critical infrastructure.
