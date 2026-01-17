This project, which we will name **ShadowMenu**, is the ideal mobile "command center" for your **EventHorizon** ecosystem. By using **Rust** with **Axum**, you leverage the M5’s high-performance asynchronous capabilities, while **Arti** provides a modern, memory-safe bridge to the Tor network.

### 1. Project Placement & Hierarchy

To maintain your established organizational structure, place ShadowMenu as a specialized service within the **Umbra** directory.

* **Root Path:** `~/antigravity/umbra/shadowmenu/`
* **Artifacts:**
* `MISSION.md`: Defines the security and UI goals.
* `TASKS.md`: Tracks the migration from static to dynamic leaf nodes.
* `SYNC_LOG.md`: Records M5 build optimizations and PQC status.



---

### 2. Implementation Guide: ShadowMenu

#### A. Installation & Build (Targeting Apple Silicon M5)

Since you are on the M5, use the `aarch64-apple-darwin` target with specific LLVM flags for the best performance.

1. **Initialize:** `cargo new shadowmenu && cd shadowmenu`
2. **Dependencies (`Cargo.toml`):**
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.1", features = ["derive"] }
tower-http = { version = "0.5", features = ["fs", "set-header"] }
arti-client = { version = "1.2", features = ["full", "onion-service-service"] }

```


3. **M5 Optimization:** Compile with hardware-accelerated crypto flags:
`RUSTFLAGS="-C target-cpu=apple-m5" cargo build --release`

#### B. The "Zero-Leak" Architecture

As a cryptographer, you know the biggest risk is the "Talkative Server." ShadowMenu must be configured to be silent.

* **Bind to Loopback:** Ensure your Axum code only binds to `127.0.0.1:3000`. This prevents any local network neighbor from seeing the service—it is *only* accessible via the internal Tor pipe.
* **Header Stripping:** Use a `Layer` in Axum to remove the `Server` and `Date` headers. This prevents an attacker from fingerprinting your MacBook’s uptime or OS version.
* **No External Links:** All assets (CSS/JS) must be served locally. Use a single-file approach or embed assets into the binary using `include_str!`.

---

### 3. Mobile UI: The "Cyber-Router" Aesthetic

To make it "compelling" on your phone, use a **Retro-Terminal TUI** style. It’s high-contrast, easy to read on OLED screens, and fits your vibe.

* **Layout:** Use a vertically scrollable list with large touch targets.
* **Interaction:** 1. **Root:** [ 📊 Metrics ] [ ⛓️ DeFi ] [ 🛡️ Umbra Status ]
2. **Branch (DeFi):** [ 🪙 ETH ] [ 🦀 SOL ] [ 📉 BTC ]
3. **Leaf:** A high-speed, static text dump of the specific cost-basis or network status.

---

### 4. Resilience & Monitoring

To ensure the service stays alive without manual intervention on your M5:

* **Persistence:** Use `launchd` (macOS native) to keep the binary running. If the process crashes or the M5 reboots, `launchd` will restart it immediately.
* **Access Logs:** Direct all Axum logs to `~/antigravity/umbra/shadowmenu/access.log`. Use `SafeLogging 1` in your `arti.toml` to mask IPs in these logs.
* **Health Check:** Periodically run `curl --socks5-hostname localhost:9150 http://[your-onion].onion` from your **Wormhole** terminal to verify the service is "punching through" the network.

---

### 5. Advanced: Post-Quantum Handshake

To make the service "future-proof" against quantum adversaries, ensure your **Arti** configuration enables the hybrid PQC handshake for Onion Services. This ensures that even if an attacker records the traffic today, they cannot decrypt the hidden service location or content with a future quantum computer.

> **Warning:** Never store your Onion Service private keys (the `hs_ed25519_secret_key`) in your Git-tracked **Wormhole** folder. Keep them in the `umbra/keys` directory and add that directory to your `.gitignore`.

Would you like me to generate the **`main.rs`** file for this Axum server, including the specific **Tailwind CSS** template for the "Cyber-Router" mobile UI?

---

[Mastering Onion Services in Rust](https://www.youtube.com/watch?v=WqFFK2bHZx4)

This video explores the nuances of building and hosting hidden services using Rust-based tools, which is helpful for ensuring your ShadowMenu remains both performant and anonymous.

