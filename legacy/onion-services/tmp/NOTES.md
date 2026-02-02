# Temporary Directory

Unix domain sockets and runtime state for inter-process communication.

## Purpose

- **Unix Sockets**: Services expose control sockets here for `awatcher` integration
- **PID Files**: Track running service instances
- **Runtime State**: Ephemeral data that doesn't persist across restarts

## Socket Naming Convention

```
tmp/
├── rpc_fortress.sock
├── vault_drop.sock
└── ...
```

## Security

This directory should have restricted permissions (700).
