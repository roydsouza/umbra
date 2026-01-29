# awatcher

Always-on monitoring and control TUI for Arti and the Umbra privacy layer.

## Status
- **Phase**: Execution (Phase 6 Complete)
- **Role**: Active TUI for real-time monitoring and control of the Umbra privacy layer.

## Quick Start
```bash
cd ~/antigravity/umbra/awatcher
cargo run --release
```

## Features
- **Arti Status**: Real-time process health, circuit visualization, bandwidth metrics.
- **Onion Services**: Monitor and control locally hosted `.onion` services.
- **Dependent Apps**: Track applications using the Arti SOCKS proxy (Zebra, etc.).
- **Privacy Verification**: Active detection of DNS leaks and clearnet traffic.
- **Vim/NeoVim Bindings**: Keyboard-driven navigation.

## Design Documents
- `DESIGN.md`: Comprehensive implementation specification.
- `ARCHITECTURE.md`: High-level technical overview.
- `FOR_GEMINI.md`: Structured prompts for LLM-assisted implementation.
