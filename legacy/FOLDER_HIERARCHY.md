umbra/
├── arti/                    # Source for the Arti (Tor in Rust) core
├── mkp224o/                 # Onion address vanity miner
├── awatcher/                # TUI dashboard for real-time monitoring
│
├── keys/                    # Secure key vault (keep out of Git!)
│   ├── defi/                # Keys for RPC, relayer, swap tools
│   ├── comms/               # Keys for file drop, SSH, ontology
│   └── stealth/             # Keys for bridge and aggregator
│
├── services/                # Native Rust service implementations
│   ├── shared/              # Common logic (Arti wrapper, crypto traits)
│   │
│   ├── defi/                # (A) Cryptocurrencies & DeFi
│   │   ├── rpc_fortress/    # Personal Ethereum RPC tunnel
│   │   ├── zk_relayer/      # Privacy Pool / Railgun relayer
│   │   └── atomic_swap/     # P2P cross-chain coordinator
│   │
│   ├── comms/               # (B) Private Communication & Storage
│   │   ├── vault_drop/      # "Umbra" Ephemeral File Drop
│   │   ├── ghost_ssh/       # Air-gapped remote terminal
│   │   └── ontology_sync/   # Distributed knowledge graph
│   │
│   └── stealth/             # (C) Censorship & Network
│       ├── adaptive_pt/     # obfs4/Snowflake auto-switcher
│       ├── ghost_rss/       # Anonymous feed aggregator
│       └── riseup_bridge/   # Onion-native mail proxy
│
├── configs/                 # TOML files for multi-tenant orchestration
│   ├── services.toml        # Master list of active .onion services
│   └── networking.toml      # Shared PT (obfs4/Snowflake) configs
│
├── bin/                     # Compiled binaries (for easy execution)
└── tmp/                     # Unix sockets for inter-process comms

### Updated Umbra Hierarchy

To incorporate all the initiatives from **ONION_IDEAS.md**, I have expanded the `services/` directory into sub-categories. This ensures your **DeFi**, **Communication**, and **Stealth** projects remain logically separated while still leveraging the shared **Arti** core and **keys** vault.

---

### Umbra Monorepo Structure

```text
umbra/
├── arti/                    # Source for the Arti (Tor in Rust) core
├── mkp224o/                 # Onion address vanity miner
├── awatcher/                # TUI dashboard for real-time monitoring
│
├── keys/                    # Secure key vault (keep out of Git!)
│   ├── defi/                # Keys for RPC, relayer, swap tools
│   ├── comms/               # Keys for file drop, SSH, ontology
│   └── stealth/             # Keys for bridge and aggregator
│
├── services/                # Native Rust service implementations
│   ├── shared/              # Common logic (Arti wrapper, crypto traits)
│   │
│   ├── defi/                # (A) Cryptocurrencies & DeFi
│   │   ├── rpc_fortress/    # Personal Ethereum RPC tunnel
│   │   ├── zk_relayer/      # Privacy Pool / Railgun relayer
│   │   └── atomic_swap/     # P2P cross-chain coordinator
│   │
│   ├── comms/               # (B) Private Communication & Storage
│   │   ├── vault_drop/      # "Umbra" Ephemeral File Drop
│   │   ├── ghost_ssh/       # Air-gapped remote terminal
│   │   └── ontology_sync/   # Distributed knowledge graph
│   │
│   └── stealth/             # (C) Censorship & Network
│       ├── adaptive_pt/     # obfs4/Snowflake auto-switcher
│       ├── ghost_rss/       # Anonymous feed aggregator
│       └── riseup_bridge/   # Onion-native mail proxy
│
├── configs/                 # TOML files for multi-tenant orchestration
│   ├── services.toml        # Master list of active .onion services
│   └── networking.toml      # Shared PT (obfs4/Snowflake) configs
│
├── bin/                     # Compiled binaries (for easy execution)
└── tmp/                     # Unix sockets for inter-process comms

```

---

### Key Organizational Enhancements

* **Sub-categorized Services:** By nesting services under `defi/`, `comms/`, and `stealth/`, you can use **Cargo Workspaces** to compile specific groups of tools or the entire suite depending on your current focus.
* **Decoupled Configs:** The `configs/` directory acts as the "brain." Your embedded `arti-client` in each service can look here to find which Pluggable Transport (PT) to use without hardcoding it.
* **Socket Communication:** The `tmp/` directory is where your Rust services can place **Unix Domain Sockets**. Your `awatcher` TUI can then scan this folder to "attach" to running services and display their live telemetry.

### Next Step

Since you are managing a large set of keys now, would you like me to help you write a **Rust utility** for the `shared/` directory that automatically maps these key paths to your different onion service instances?

