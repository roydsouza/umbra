# Umbra Directory Hierarchy

This document maps the physical structure of the `umbra` workspace, designed for modular Rust development and rapid experimentation.

## Top-Level Structure

```text
umbra/
├── arti-upstream/           # Git Submodule: Official Tor Project Arti source
├── bin/                     # Operational scripts (build, start, orbit)
├── etc/                     # Active Configuration (arti.toml, profiles)
├── guardian/                # Submodule: Network Shield & Traffic Analysis
├── keys/                    # 🔒 Gitignored: Production Identity Keys
├── labs/                    # Experimental components & Research notes
├── legacy/                  # Archived tools and documentation
├── missioncontrol/          # Unified Station Dashboard (Tauri)
├── modules/                 # Rust Workspace Members (Shared Logic)
├── penumbra/                # Submodule: DNS Privacy & Resolution
├── tools/                   # Standalone Utilities (awatcher, mkp224o)
└── var/                     # Runtime State (Logs, PIDs, Sockets)
```

## Detailed Component Map

### 🛠️ Tools (`umbra/tools/`)
Active utilities promoted for daily operations.
- **awatcher/**: Rust TUI for real-time Arti monitoring.
- **mkp224o/**: Vanity Onion Address generator (C-based).

### 🧪 Labs (`umbra/labs/`)
Incubator for new features and architectural research.
- **arch-explorer/**: Call-graph and dependency analysis tools.
- **cgo-prototype/**: Counter-Galois Onion (Proposal 359) experiments.
- **ONION_IDEAS.md**: Research into RPC fortresses and ephemeral drops.

### 📦 Modules (`umbra/modules/`)
Core Rust crates shared across the workspace.
- **astrometrics-core/**: Data structures for network measurements.
- **monitor-flow/**: Bandwidth and traffic flow instrumentation.
- **network-topology/**: Consensus parsing and relay graphing.

### ⚙️ Configuration
- **Active Config**: `etc/arti.toml` (Promoted from legacy).
- **Profiles**: `etc/arti-profiles.toml` (Managed by `start-arti`).

### 📚 Documentation
- **ONION_SERVICES.md**: Guide to creating and hosting hidden services.
- **LOGGING.md**: Explains log files (arti.log, arti.err, etc.).
- **FOLDER_HIERARCHY.md**: This map.

### 🗑️ Legacy (`umbra/legacy/`)
- **Archives**: Old C-Tor docs, deprecated drafts.
