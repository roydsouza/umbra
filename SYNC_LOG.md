# Umbra Project Sync Log

This log maintains the context of automated work performed within the `umbra` directory.

---

### Session: 2026-01-15

- **Repository Initialization:**
    - Initialized `umbra` as a new Git repository.
    - Added the GitHub remote: `https://github.com/roydsouza/umbra`.

- **Submodule Management:**
    - Added the C-based Tor implementation as a submodule (`tor`) from `https://gitlab.torproject.org/tpo/core/tor.git`.
    - Added the Rust-based Tor implementation, Arti, as a submodule (`arti`) from `https://gitlab.torproject.org/tpo/core/arti.git`.

- **Builds & Compilation:**
    - Successfully compiled the `tor` submodule from source after installing dependencies.
    - Successfully compiled the `arti` submodule from source using `cargo build --release`.

- **Documentation & Task Management:**
    - Created `TASKS.md` to track feature requests, tasks, and bugs.
    - Created `ARTI.md` to document the purpose, build process, and administration of the local Arti instance.
    - Created `LISTS.md` as a historical ledger for bugs and feature requests, as specified in `MISSION.md`.

- **Configuration:**
    - Created a local `arti.toml` configuration file within the `umbra` directory.
    - Created a symbolic link from `~/Library/Application Support/org.torproject.Arti/arti.toml` to the local `arti.toml` file for easy configuration management.
    - Updated `ARTI.md` to clarify the location and symlinked nature of the `arti.toml` file.

### Session: 2026-01-16

- **Documentation Updates:**
    - Reviewed workspace structure; created `TASKS.md` for `darkmatter` and `gravitylens`.
    - Updated `MISSION.md` to explicitly strictly forbid Relay configuration and focus on Hidden Service migration.
    - Updated `TASKS.md` and successfully migrated Tor keys from `/usr/local/tor-m5`.
- **Arti Installation & Coexistence:**
    - Compiled Arti 1.9.0 from source in `umbra/arti`.
    - Installed Arti binary and configuration locally in `umbra/bin/` and `umbra/etc/`.
    - Configured Arti to use port `9150` to coexist with C-Tor on `9050`.
    - Established centralized key management in `umbra/keys/` with `ctor/` and `arti/` subdivisions.
    - Created a macOS Launch Agent for Arti at `~/Library/LaunchAgents/org.torproject.arti.plist`.
- **Documentation:**
    - Created `C-TOR.md` to document the legacy core.
    - Created `ARTI.md` to document the new Rust core.
- **Automated Rituals:**
    - Codified the [Sync Ritual](.agent/workflows/sync.md) for all future sessions.
    - Protocol includes mandatory upstream software checks for `tor` and `arti`.
    - Integrated the ritual into the `MISSION.md` directives.
- **Security Hardening:**
    - Redacted all sensitive `.onion` addresses and PII from public documentation.
    - Moved identity hostnames to the version-controlled-excluded `umbra/keys/` directory.
    - Created `umbra/keys/README.md` to document secure identity management.
- **Wormhole & Neovim Consolidation**:
    - Restructured Neovim configuration into a unified super-project submodule.
    - Stabilized Neovim 0.11+ configuration by transitioning to native LSP APIs.
    - Converted Ghostty configuration to a submodule for better portability.
- **Status:**
    - C-Tor and Arti are configured to run simultaneously.
    - Identity keys are securely archived and marked by implementation.
    - Automated rituals are fully operational and referenced in [CONTENTS.md](CONTENTS.md).
    - Repository is hardened against accidental PII leaks.
    - Neovim environment is error-free on 0.11.5.
