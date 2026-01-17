This is a fantastic "Phase 2" for your **Umbra** project, Roy. Building a mobile-friendly TUI/Web interface that runs as a Tor Onion Service is the perfect way to interact with your **EventHorizon** ecosystem while you are away from your desk.

Since you want it to be **fast**, **colorful**, and **Rust-based**, the best architecture is a **Rust Axum web server** serving a highly optimized, mobile-responsive frontend, tunneled through **Arti**.

---

### Project Name: ShadowMenu

Following your theme, we will name this interface **ShadowMenu**. It acts as the "interactive map" of your dark sector.

### 1. Placement in Hierarchy

Keep this as a subordinate project within **Umbra** to share the same security context and Arti binaries.

* **Path:** `~/antigravity/umbra/shadowmenu/`
* **Key Files:** `MISSION.md`, `TASKS.md`, `SYNC_LOG.md` (as per your standard protocol).

---

### 2. Implementation Guide (`SHADOWMENU.md`)

Cut and paste the following into your new project directory.

```markdown
# ShadowMenu: Onion-Routed Mobile Interface

## 1. Vision
A resilient, mobile-optimized "Router" interface for the EventHorizon ecosystem, providing a scrollable, hierarchical menu of DeFi metrics, system status, and secure notes, all accessible via a `.onion` address.

## 2. Technical Stack
- **Language:** Rust (Stable).
- **Web Framework:** [Axum](https://github.com/tokio-rs/axum) (High-performance, async-first).
- **Routing Layer:** [Arti](https://arti.torproject.org/) (Rust implementation of Tor).
- **Styling:** Tailwind CSS (via CDN for speed) with a "Terminal/Cyberpunk" dark theme.
- **Process Management:** `launchd` (Native macOS service management).

## 3. Installation & Setup
### Prerequisites
- Rust/Cargo installed on the M5 Pro.
- Arti installed in `~/antigravity/umbra/arti`.

### Build Commands
1. `cargo new shadowmenu`
2. Add dependencies to `Cargo.toml`:
   ```toml
   [dependencies]
   axum = "0.7"
   tokio = { version = "1.0", features = ["full"] }
   serde = { version = "1.0", features = ["derive"] }
   tower-http = { version = "0.5", features = ["fs"] }

```

3. Compile for M5:
`cargo build --release --target aarch64-apple-darwin`

## 4. Onion Service Configuration

To expose this locally:

1. Configure `arti.toml` to enable Onion Services.
2. Point the hidden service port (80) to your local Axum port (e.g., 3000).
3. **Identity Security:** Your `.onion` private key must be kept in `umbra/keys/shadowmenu.key`.

## 5. Security & Leak Prevention (Critical)

* **Localhost Binding:** Ensure Axum only binds to `127.0.0.1:3000`, NEVER `0.0.0.0`. This ensures it is only reachable via the Tor proxy.
* **Header Stripping:** The Axum server must strip `Server` headers to avoid identifying the host as a MacBook M5.
* **No External Assets:** All CSS/JS must be bundled or served from the Onion address. No calls to Google Fonts or CDNs that could leak the visitor's IP to third parties.
* **PQC Enabled:** Run Arti with `--enable-pqc` to protect the hidden service handshake against quantum-capable surveillance.

## 6. Maintenance & Monitoring

### Resilience (Auto-Restart)

Create a `com.roy.shadowmenu.plist` in `~/Library/LaunchAgents/` to ensure the service restarts on crash or reboot.

### Monitoring

* **Access Logs:** View raw traffic via `tail -f ~/antigravity/umbra/shadowmenu/access.log`.
* **Anonymity Check:** Periodically verify your hostname is NOT visible in any response headers using `curl -I [your-onion-address].onion`.

## 7. The Architecture (Tree View)

* **Root Menu:** [System Status] | [DeFi Portfolio] | [Wormhole Logs]
* **Level 1 (DeFi):** [Ethereum Metrics] | [Solana Status] | [BTC Balance]
* **Leaf Node:** Static text display of the selected metric.

```

---

### 3. Making it "Colorful and Compelling"
For the UI, use a **Retro-Terminal Aesthetic**. Use CSS to give it a "scanline" effect and high-contrast greens/pinks against a deep black background. This looks stunning on an iPhone's OLED screen and feels consistent with your **Ghostty** and **NeoVim** environments.



---

### 4. Leak Prevention: The "Air-Gap" Mentality
Since you are a cryptographer, you know that metadata is often the weakest link. 

* **Clock Skew:** Ensure your M5's clock is synced via NTP, but be aware that slight clock skews can sometimes be used to fingerprint Tor nodes.
* **Memory Safety:** By writing this in **Rust**, you are already mitigating the most common buffer overflow vulnerabilities that plague C-based web servers.
* **Tor Over VPN?** If you want an extra layer, you can run Tor over a trusted VPN, but for a hidden service, the **Onion Service** itself provides the strongest anonymity for the *location* of your laptop.

### Next Step for ShadowMenu
Would you like me to generate the actual **Rust Axum code (`main.rs`)** that implements the hierarchical routing and the mobile-optimized HTML template? I can make sure it's optimized specifically for the **M5's async performance**.

```

