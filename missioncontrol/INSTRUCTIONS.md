# Instructions: MissionControl


**[📍 Back to Map](CONTENTS.md)**


This document provides instructions for setting up, building, and running the MissionControl service.

## 1. Prerequisites

*   **Rust Toolchain:** Ensure you have the latest stable Rust toolchain installed. You can install it via `rustup.rs`.
*   **Arti Client:** This project assumes that the `arti` client from the parent `[Umbra](../)` project is correctly configured and running. MissionControl will connect to this Arti instance.
*   **Dependencies:** For compiling `arti`, you may need `pkg-config` and `openssl`. On macOS, this can be installed with `brew install pkg-config openssl`.

## 2. Initial Setup

1.  **Clone the Repository:**
    ```bash
    # This repository is located at umbra/missioncontrol
    # Ensure the parent `umbra` repository and its submodules are cloned correctly.
    ```

2.  **Create Secrets Directory:**
    The service requires a directory to store its private keys. This directory is intentionally ignored by Git.
    ```bash
    mkdir -p keys
    ```

## 3. Build & Run

### A. Development Build

For a standard development build with debugging symbols:
```bash
cargo build
```

### B. M5-Optimized Release Build

To build a high-performance release binary optimized for Apple Silicon (M5):
```bash
RUSTFLAGS="-C target-cpu=native" cargo build --release
```
The final binary will be located at `target/release/missioncontrol`.

### C. Running the Service

Once built, you can run the service with:
```bash
./target/release/missioncontrol
```
The service will attempt to connect to the running Arti daemon and launch itself as an Onion Service. The `.onion` address will be printed to the console upon successful launch.

## 4. Deployment (macOS)

For persistent operation, it is recommended to run MissionControl as a `launchd` service.

1.  **Create a `plist` file** in `~/Library/LaunchAgents/com.roy.missioncontrol.plist`.
2.  **Add the following content** to the file, ensuring the `ProgramArguments` path points to the compiled release binary:
    ```xml
    <?xml version="1.0" encoding="UTF-8"?>
    <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
    <plist version="1.0">
    <dict>
        <key>Label</key>
        <string>com.roy.missioncontrol</string>
        <key>ProgramArguments</key>
        <array>
            <string>/path/to/your/antigravity/umbra/missioncontrol/target/release/missioncontrol</string>
        </array>
        <key>RunAtLoad</key>
        <true/>
        <key>KeepAlive</key>
        <true/>
        <key>StandardOutPath</key>
        <string>/path/to/your/antigravity/umbra/missioncontrol/missioncontrol.log</string>
        <key>StandardErrorPath</key>
        <string>/path/to/your/antigravity/umbra/missioncontrol/missioncontrol_error.log</string>
    </dict>
    </plist>
    ```
3.  **Load and start the service:**
    ```bash
    launchctl load ~/Library/LaunchAgents/com.roy.missioncontrol.plist
    launchctl start com.roy.missioncontrol
    ```