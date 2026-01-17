# Project Umbra: Directory Contents

This file provides an alphabetical index of the documentation and configuration files managed within Project Umbra.

---

### 📂 Configuration Files

- [`arti.toml`](arti.toml): The primary configuration file for the **Arti** (Rust) client. Manages SOCKS ports (`9150`), logging, and local storage paths.
- [`torrc`](file:///usr/local/tor-m5/etc/tor/torrc): The primary configuration file for the **C-Tor** (Legacy) client. Manages ORPorts, Hidden Services, and M5-specific hardware acceleration settings.

---

### 📄 Documentation (Markdown)

- [`ARTI.md`](ARTI.md): Comprehensive guide for the Rust-based Tor implementation. Includes build instructions, network ports, programmatic usage examples, and logging details.
- [`C-TOR.md`](C-TOR.md): Detailed documentation for the legacy C-based Tor installation. Covers the custom build prefix, key archiving, and coexistence with Arti.
- [`CONTENTS.md`](CONTENTS.md): *You are here.* An alphabetical map of the project's meta-documentation and configuration layers.
- [`CONTEXT.md`](CONTEXT.md): Provides relevant background information, history, and dependencies for this project.
- [`INSTRUCTIONS.md`](INSTRUCTIONS.md): Detailed instructions on how to set up, build, run, and use this project.
- [`keys/README.md`](keys/README.md): Security documentation for the ignored identities and keys directory.
- [`LISTS.md`](LISTS.md): The historical ledger for the project. Tracks user-reported bugs, feature requests, and long-term technical debt as specified in the Mission.
- [`MISSION.md`](MISSION.md): The core technical and strategic directive. Defines the "Hidden Service Only" goal, M5 optimization requirements, and operational protocols.
- [`PROMPTS.md`](PROMPTS.md): [Description of PROMPTS.md content]
- [`README.md`](README.md): The entry-point documentation providing a high-level architectural overview and the three-phase roadmap (Legacy -> Modernization -> Migration).
- [`SYNC_LOG.md`](SYNC_LOG.md): A sequential record of all automated and manual sessions. Used to maintain state between the AI agent and the user across push/pull cycles.
- [`TASKS.md`](TASKS.md): The active engineering queue. Tracks prioritized items like Arti integration progress and key migration status.