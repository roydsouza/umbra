# Logs Directory

Centralized logging for all onion services.

## Structure

Each service writes logs to a subdirectory here:

```
logs/
├── rpc_fortress/
├── zk_relayer/
├── vault_drop/
└── ...
```

## Log Rotation

Consider using `logrotate` or a Rust-based solution to prevent unbounded growth.

## Integration

- `awatcher` can tail these logs for real-time display
- Structured JSON logging recommended for parsing
