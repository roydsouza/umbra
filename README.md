# Project Umbra: A Secure & Private Communication Layer

## 1. Vision & Purpose

As defined in `MISSION.md`, Umbra is the dedicated privacy and anonymity subsystem for the **EventHorizon** ecosystem. Its primary purpose is to provide a "Shadow Layer" for all outgoing and incoming communications, ensuring metadata resistance and identity obfuscation for other projects in the `antigravity` workspace.

## 2. Architectural Roadmap

The project follows a three-phase roadmap to ensure a stable and forward-looking privacy layer:

-   **Phase 1 (Legacy Core):** Deploy and utilize the standard C-based Tor implementation.
-   **Phase 2 (Modernization):** Integrate and build Arti, the next-generation Rust implementation of Tor.
-   **Phase 3 (Migration):** Gradually transition all traffic to Arti as it achieves feature-parity and stability.

## 3. Directory Contents

-   `.agent/`: Contains automated workflows and rituals (e.g., sessions sync, upstream checks).
-   `.git/`: The Git directory for the `umbra` project itself.
-   `.gitignore`: Specifies files for Git to ignore, most notably the `keys/` directory.
-   `.gitmodules`: Defines the Git submodules managed within this project.
-   `arti/`: A Git submodule containing the source code for the **Arti** project, a Rust-based implementation of Tor. This is the focus of Phase 2 and 3.
-   `ARTI.md`: Detailed documentation on the local Arti instance, including build, configuration, and administration instructions.
-   `arti.toml`: The local configuration file for the Arti client. This file is symlinked from the default system location (`~/Library/Application Support/org.torproject.Arti/`) to allow for version control and easy editing.
-   `keys/`: A git-ignored directory for storing sensitive, persistent identity keys for Tor and Arti services.
-   `MISSION.md`: The core directive, technical requirements, and operational protocols for the Umbra project.
-   `SYNC_LOG.md`: A log of significant changes, build processes, and session context for the `umbra` project.
-   `TASKS.md`: A file for tracking high-level feature requests, bugs, and other engineering tasks.
-   `tor/`: A Git submodule containing the source code for the C-based **Tor** project. This is the focus of Phase 1.
