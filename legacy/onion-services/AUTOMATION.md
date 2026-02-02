# Onion Service Automation

This document allows AntiGravity to automatically provision new Onion Services by reading this file.

## Instructions for Agents

When a user asks to "add an onion service" or "expose X via Tor":

1.  **Identify the Service**: Determine the local port and the desired service name.
2.  **Edit `arti.toml`**: Add the service configuration to `/Users/rds/antigravity/umbra/arti.toml`.

### Configuration Format

Add the following block to the `[onion_services]` section (create it if it doesn't exist):

```toml
[onion_services."SERVICE_NAME"]
# The port Arti listens on (inside the onion network) -> Local service port
proxy_ports = [
    [ 80, "127.0.0.1:LOCAL_PORT" ]
]
```

### Example

To expose a web server running on port 8080 as `my-website`:

```toml
[onion_services."my-website"]
proxy_ports = [
    [ 80, "127.0.0.1:8080" ]
]
```

## Post-Action
After editing `arti.toml`, `arti` should automatically pick up the changes (if `watch_configuration = true` is set). Check `awatcher` to confirm the service appears and get its hostname.
