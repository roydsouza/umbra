# awatcher Implementation Prompt for Gemini

> **Purpose**: This document provides structured prompts for delegating the implementation of `awatcher` to Gemini 3 Pro (or equivalent LLM).  
> **Approach**: Phase-by-phase implementation with review checkpoints.  
> **Last Updated**: 2026-01-29

---

## Project Context

You are implementing **awatcher**, a Rust-native Terminal User Interface (TUI) for monitoring and controlling Arti (the Rust Tor implementation) and the Umbra privacy layer. The application runs in Ghostty terminal with the **Tokyo Night** colorscheme.

**Key Documents to Reference**:
- `DESIGN.md` - Comprehensive specification (layout, colors, architecture)
- `ARCHITECTURE.md` - High-level technical overview
- `TASKS.md` - Implementation phases and requirements

**Working Directory**: `~/antigravity/umbra/awatcher/`

**Important Constraints**:
- Use **Vim/NeoVim-style keybindings** for navigation.
- Match the **Tokyo Night** color palette exactly (see DESIGN.md Section 5).
- All async operations use **Tokio**.
- Target macOS (Apple Silicon M5).
- Control Arti via **launchctl** (macOS service manager).

**Related Project**: `~/antigravity/darkmatter/zcash/zwatcher/` - A sibling TUI with identical design language. You can reference its implementation for patterns.

---

## ⚡ Optimization Notes

### 1. Shared Crate Strategy (Future Consideration)

Both `awatcher` and `zwatcher` share ~60% identical code:
- Theme constants (Tokyo Night palette)
- Log viewer widget with Vim bindings (`j/k`, `gg/G`, search)
- Help overlay widget
- Terminal setup/cleanup patterns
- State machine for `gg` binding

**Recommendation**: After both applications are stable, extract shared code into a workspace crate:
```
~/antigravity/wormhole/tui-common/
├── Cargo.toml
└── src/
    ├── theme.rs       # Tokyo Night colors
    ├── log_viewer.rs  # Scrollable log widget
    ├── vim_keys.rs    # Vim keybinding state machine
    ├── help.rs        # Help overlay
    └── terminal.rs    # Setup/cleanup utilities
```

For now, implement independently to move fast, but keep code structure parallel for easy extraction later.

### 2. Dependent App Detection (Improved)

The original design uses `lsof` which is slow (~200ms per call) and requires shell parsing. 

**Better approach**: Use `sysinfo` crate's network connection APIs:

```rust
use sysinfo::{System, Networks};

pub fn detect_socks_clients(system: &System) -> Vec<DependentApp> {
    // sysinfo 0.30+ can enumerate network connections per process
    // Filter for connections to 127.0.0.1:9050
    // Much faster than spawning lsof
}
```

If `sysinfo` doesn't expose socket info on macOS, fall back to `lsof` but cache results for 5 seconds.

### 3. Log Rotation Handling

Arti uses daily log rotation (`arti.log.2026-01-29`). The log watcher should:
1. Detect when `arti.log` is rotated (file truncated or recreated).
2. Automatically switch to watching the new file.
3. Optionally show "[LOG ROTATED]" indicator.

### 4. Circuit Detection Strategy

Parsing logs for circuit counts is fragile. Better approaches (in order of preference):
1. **Arti Control Port** (if enabled): Query via RPC for real circuit state.
2. **Log Parsing** (fallback): Count `Circuit established` and `Circuit closed` messages.
3. **Estimate Only**: Show "~N circuits (estimated from logs)".

For Phase 1, use log parsing. Document Control Port as future enhancement.

---

## Phase 1: Core TUI Shell

### Prompt

```
I need you to create the foundation for a Rust TUI application called "awatcher".

## Project Setup

Create a new Rust project in the current directory with these dependencies:

```toml
[package]
name = "awatcher"
version = "0.1.0"
edition = "2021"

[dependencies]
ratatui = "0.26"
crossterm = "0.27"
tokio = { version = "1", features = ["full"] }
color-eyre = "0.6"
```

## Requirements

1. **Terminal Setup**:
   - Initialize crossterm with raw mode and alternate screen.
   - Handle terminal cleanup on exit (normal or panic).
   - Use `color-eyre` for error handling.

2. **Main Loop**:
   - Create an event loop that:
     - Polls for keyboard input every 100ms.
     - Renders the UI on each tick.
     - Exits cleanly on 'q' key.

3. **Basic Layout**:
   - Create a layout with:
     - **Header** (top 3 lines): Title "A-WATCHER v0.1.0" centered with privacy/status indicators.
     - **Top Row**: 2 columns (Arti Status | Onion Services).
     - **Middle Row**: 2 columns (Dependent Apps | Privacy Monitor).
     - **Bottom**: Full-width log viewer.
     - **Footer** (bottom 1 line): "[q] Quit  [?] Help"
   
   Visually:
   ```
   ┌─────────────────────────────────────────┐
   │      A-WATCHER v0.1.0  [Privacy: 🟢]   │  <- Header
   ├──────────────────┬──────────────────────┤
   │  Arti Status     │  Onion Services      │  <- Top row
   ├──────────────────┼──────────────────────┤
   │  Dependent Apps  │  Privacy Monitor     │  <- Middle row
   ├──────────────────┴──────────────────────┤
   │           Logs (arti.log)               │  <- Bottom (full width)
   ├─────────────────────────────────────────┤
   │ [q] Quit  [?] Help                      │  <- Footer
   └─────────────────────────────────────────┘
   ```

4. **Tokyo Night Colors**:
   Define these color constants and use them:
   ```rust
   use ratatui::style::Color;
   
   const TOKYO_BLUE: Color = Color::Rgb(122, 162, 247);
   const TOKYO_CYAN: Color = Color::Rgb(125, 207, 255);
   const TOKYO_GREEN: Color = Color::Rgb(158, 206, 106);
   const TOKYO_ORANGE: Color = Color::Rgb(255, 158, 100);
   const TOKYO_RED: Color = Color::Rgb(247, 118, 142);
   const TOKYO_PURPLE: Color = Color::Rgb(187, 154, 247);
   const TOKYO_GOLD: Color = Color::Rgb(224, 175, 104);
   const TOKYO_BORDER: Color = Color::Rgb(86, 95, 137);
   const TOKYO_FG: Color = Color::Rgb(169, 177, 214);
   ```
   
   - Use `TOKYO_BLUE` for the header title.
   - Use `TOKYO_PURPLE` for "A-WATCHER" (privacy-focused identity).
   - Use `TOKYO_BORDER` for all panel borders.
   - Use `TOKYO_FG` for default text.

5. **File Structure**:
   ```
   src/
   ├── main.rs      # Entry point, terminal setup
   ├── app.rs       # App struct, event handling
   ├── ui.rs        # All rendering logic
   └── theme.rs     # Color constants
   ```

Please implement this foundation. The panels should show placeholder text like "Panel: Arti Status" etc.
```

### Verification Checklist
- [ ] `cargo build` succeeds without warnings.
- [ ] Running the app shows the layout correctly.
- [ ] Pressing 'q' exits cleanly.
- [ ] Colors match Tokyo Night theme.
- [ ] Terminal is restored properly on exit.

---

## Phase 2: Arti Monitoring

### Prompt

```
Now extend awatcher to monitor the Arti process and SOCKS5 proxy.

## New Dependencies

Add to Cargo.toml:
```toml
sysinfo = "0.30"
tokio-stream = "0.1"
```

## Requirements

1. **Process Monitoring** (`src/data/process.rs`):
   - Use `sysinfo` crate to detect the Arti process.
   - Look for process name "arti".
   - Return: running (bool), PID, memory (MB), CPU %.

   ```rust
   pub struct ArtiStatus {
       pub running: bool,
       pub pid: Option<u32>,
       pub memory_mb: f64,
       pub cpu_percent: f32,
   }
   ```

2. **SOCKS5 Connectivity Test** (`src/data/socks.rs`):
   - Attempt a TCP connection to `127.0.0.1:9050`.
   - Return true if connection succeeds, false otherwise.
   - Timeout after 500ms.

   ```rust
   pub async fn test_socks_proxy() -> bool {
       use tokio::net::TcpStream;
       use tokio::time::{timeout, Duration};
       
       timeout(Duration::from_millis(500), TcpStream::connect("127.0.0.1:9050"))
           .await
           .map(|r| r.is_ok())
           .unwrap_or(false)
   }
   ```

3. **App State** (`src/app.rs`):
   - Store Arti status in the App struct.
   - Spawn Tokio tasks to refresh:
     - Process status: every 2 seconds.
     - SOCKS5 test: every 5 seconds.
   - Use channels (tokio::sync::mpsc) to send updates.

4. **Display Real Data**:
   - "Arti Status" panel shows:
     - Process: 🟢 RUNNING / 🔴 STOPPED
     - PID, Memory, CPU
     - SOCKS5: 🟢 :9050 OK / 🔴 :9050 FAIL

## File Structure Update
```
src/
├── main.rs
├── app.rs
├── ui.rs
├── theme.rs
└── data/
    ├── mod.rs
    ├── process.rs
    └── socks.rs
```

Please implement Arti process monitoring and SOCKS5 testing.
```

### Verification Checklist
- [ ] Status shows correctly when Arti is running.
- [ ] Status shows 🔴 STOPPED when Arti is not running.
- [ ] SOCKS5 indicator reflects actual connectivity.
- [ ] No crashes when Arti is not running.

---

## Phase 3: Log Viewer

### Prompt

```
Implement the log viewer panel with real-time tailing and Vim-style navigation.

## New Dependencies

Add to Cargo.toml:
```toml
notify = "6"
```

## Requirements

1. **Log File Watching** (`src/data/logs.rs`):
   - Watch `~/antigravity/umbra/var/log/arti.log`.
   - Use the `notify` crate to detect file changes.
   - Read new lines when file is modified.
   - Store last 1000 lines in a ring buffer.
   - Also watch `arti.err.log` for errors.

2. **Log Viewer Widget** (`src/ui/logs.rs`):
   - Display log lines in the bottom panel.
   - Color-code by log level:
     - `INFO` → TOKYO_FG (normal)
     - `WARN` → TOKYO_ORANGE
     - `ERROR` → TOKYO_RED
     - `DEBUG` → TOKYO_BORDER (dimmed)
   - Show timestamps in TOKYO_CYAN.
   - Show `[scrubbed]` in TOKYO_PURPLE (privacy indicator).

3. **Vim-Style Navigation**:
   - `j` / `↓`: Scroll down one line.
   - `k` / `↑`: Scroll up one line.
   - `Ctrl+d`: Page down (half screen).
   - `Ctrl+u`: Page up (half screen).
   - `gg`: Jump to top (oldest log).
   - `G`: Jump to bottom (newest, enable follow mode).
   - When at bottom, auto-scroll as new logs arrive (follow mode).
   - When user scrolls up, disable follow mode.
   - Show "[FOLLOW]" indicator in panel title when following.

4. **State Machine for `gg`**:
   ```rust
   enum KeyState {
       Normal,
       WaitingForG,  // After first 'g' press
   }
   ```
   - On first 'g': Enter WaitingForG state, start 500ms timeout.
   - On second 'g' within timeout: Execute "go to top".
   - On timeout or other key: Return to Normal state.

5. **Panel Title**:
   - Show: "LOGS (arti.log) [FOLLOW]" or "LOGS (arti.log) [LINE 234/1000]"

Please implement the log viewer with navigation.
```

### Verification Checklist
- [ ] Log file is tailed in real-time.
- [ ] Colors match log levels.
- [ ] `[scrubbed]` appears in purple.
- [ ] `j`/`k` scrolling works smoothly.
- [ ] `gg` goes to top, `G` goes to bottom.
- [ ] Follow mode activates at bottom.

---

## Phase 4: Onion Services & Dependencies

### Prompt

```
Add onion service monitoring and dependent application detection.

## New Dependencies

Add to Cargo.toml:
```toml
toml = "0.8"
serde = { version = "1", features = ["derive"] }
```

## Requirements

1. **Config Parsing** (`src/data/config.rs`):
   - Read `~/antigravity/umbra/arti.toml`.
   - Parse `[onion_services.*]` sections.
   - Extract service name and port mappings.

   ```rust
   pub struct OnionServiceConfig {
       pub name: String,
       pub proxy_ports: Vec<(u16, String)>,  // (external_port, local_target)
   }
   ```

2. **Hostname Discovery** (`src/data/onion.rs`):
   - Read hostname files from:
     `~/antigravity/umbra/var/lib/arti/onion_services/{name}/hostname`
   - Handle case where file doesn't exist (service not yet started).

   ```rust
   pub struct OnionService {
       pub name: String,
       pub hostname: Option<String>,  // The .onion address
       pub ports: Vec<(u16, String)>,
       pub status: OnionStatus,       // Active, Pending, Error
   }
   
   pub enum OnionStatus {
       Active,   // Hostname file exists
       Pending,  // Config exists but no hostname yet
       Error,    // Config error or missing
   }
   ```

3. **Dependent Application Detection** (`src/data/deps.rs`):
   - Use `lsof` to find processes connected to port 9050.
   - Parse output to extract process names and PIDs.
   - Track known applications: zebrad, zaino, monerod, curl.

   ```rust
   pub struct DependentApp {
       pub name: String,
       pub pid: Option<u32>,
       pub connected: bool,
   }
   ```

   Command to run:
   ```bash
   lsof -iTCP:9050 -sTCP:ESTABLISHED -F pcn
   ```

4. **Display Widgets**:
   - "Onion Services" panel shows:
     - Service name
     - `.onion` address (truncated with "...") in TOKYO_PURPLE
     - Port mappings
     - Status indicator (🟢/🟡/🔴)
   
   - "Dependent Applications" panel shows:
     - App name
     - 🟢 Connected / 🔴 Disconnected / ⚪ Not Running
     - PID when connected

Please implement onion service and dependency monitoring.
```

### Verification Checklist
- [ ] Config is parsed correctly (test with actual arti.toml).
- [ ] Onion hostname displays when service is running.
- [ ] Shows "Pending" when hostname file doesn't exist.
- [ ] Dependent apps detected when connected.
- [ ] lsof parsing handles edge cases.

---

## Phase 5: Actions & Control

### Prompt

```
Add action capabilities to control Arti via launchctl.

## Requirements

1. **launchctl Integration** (`src/actions/launchctl.rs`):
   - Implement start/stop/restart for Arti.
   - Use macOS launchctl commands:

   ```rust
   pub async fn start_arti() -> Result<(), String> {
       // launchctl start org.torproject.arti
       let output = Command::new("launchctl")
           .args(["start", "org.torproject.arti"])
           .output()
           .await
           .map_err(|e| e.to_string())?;
       
       if output.status.success() {
           Ok(())
       } else {
           Err(String::from_utf8_lossy(&output.stderr).to_string())
       }
   }
   
   pub async fn stop_arti() -> Result<(), String> {
       // launchctl stop org.torproject.arti
   }
   
   pub async fn restart_arti() -> Result<(), String> {
       // launchctl kickstart -k gui/$(id -u)/org.torproject.arti
       let uid = std::process::Command::new("id")
           .arg("-u")
           .output()
           .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
           .unwrap_or_default();
       
       let service = format!("gui/{}/org.torproject.arti", uid);
       // launchctl kickstart -k {service}
   }
   ```

2. **Action Keybindings**:
   - `s`: Start Arti (when stopped).
   - `t`: Stop Arti (when running).
   - `r`: Restart Arti.
   - Show confirmation message in status bar before action.
   - Show result (success/failure) after action.

3. **Status Bar**:
   - Add a one-line status bar above the footer.
   - Display action confirmations and results.
   - Auto-clear after 3 seconds.

4. **Log Search**:
   - `/`: Open search input.
   - Type query, press Enter.
   - `n`: Next match.
   - `N`: Previous match.
   - Highlight matches in TOKYO_GOLD.
   - `Esc`: Clear search.

5. **Log Filter**:
   - `f`: Cycle through: ALL → INFO+ → WARN+ → ERROR → ALL
   - Show current filter in panel title.

6. **Help Overlay**:
   - `?`: Toggle help overlay.
   - Display all keybindings.
   - Press `?` or `Esc` to close.

Please implement control actions and search.
```

### Verification Checklist
- [ ] Start/stop/restart Arti works.
- [ ] Status bar shows action results.
- [ ] Search highlights matches.
- [ ] Filter cycles correctly.
- [ ] Help overlay displays properly.

---

## Phase 6: Privacy Monitor & Polish

### Prompt

```
Add privacy monitoring integration and final polish.

## Requirements

1. **Privacy Indicators** (`src/data/privacy.rs`):
   - Combine all checks into a privacy status:
     - Arti running? ✓
     - SOCKS5 responding? ✓
     - No leaks detected? ✓ (future: check Guardian)
   
   ```rust
   pub enum PrivacyStatus {
       Clean,    // All systems nominal
       Warning,  // Partial (Arti up but missing components)
       Danger,   // Leaks detected or Arti down
   }
   ```

2. **Header Updates**:
   - Display privacy indicator: `[Privacy: 🟢 CLEAN]` / `[🟡 PARTIAL]` / `[🔴 DANGER]`
   - Color-code appropriately.
   - Display Arti status: `[Arti: 🟢 ONLINE]` / `[🔴 OFFLINE]`

3. **Privacy Monitor Panel**:
   - Show Guardian status (placeholder until API ready).
   - Show Penumbra status (placeholder until API ready).
   - Display "Last 24h: No clearnet DNS leaks" (placeholder).

4. **Error States**:
   - When Arti not running, show "OFFLINE" prominently.
   - When logs unavailable, show "Log file not found".
   - When config parse fails, show "Config error".

5. **Responsive Layout**:
   - Detect terminal width.
   - < 80 cols: Stack all panels vertically.
   - 80-120 cols: 2-column layout.
   - > 120 cols: Full layout.

6. **Circuit Count** (Bonus):
   - Parse log for circuit establishment messages.
   - Count active circuits (rough estimate from logs).
   - Display "Circuits: Active: N  Building: M"

7. **Final Polish**:
   - Consistent spacing and alignment.
   - Smooth scrolling (no flicker).
   - Clean exit message: "awatcher closed cleanly."

Please implement privacy monitoring and final polish.
```

### Verification Checklist
- [ ] Privacy indicator in header works.
- [ ] Error states display gracefully.
- [ ] Layout adapts to terminal width.
- [ ] No visual glitches or flicker.
- [ ] Exit message appears.

---

## Final Integration

After all phases are complete:

1. **Test Full Flow**:
   - Start Arti, watch process status.
   - Stop Arti, verify UI shows offline.
   - Tail logs, search for patterns.
   - Test all keybindings.

2. **grav Integration**:
   - Register awatcher in `~/antigravity/wormhole/bin/grav`.
   - Add `grav start awatcher`, `grav stop awatcher`, `grav status awatcher`.

3. **Documentation**:
   - Update `README.md` with usage instructions.
   - Document all keybindings.

---

## Reference: Arti Log Patterns

Common log patterns to parse:

```
2026-01-29T07:30:12Z  INFO tor_dirmgr: Directory is complete. attempt=8
2026-01-29T07:30:15Z  INFO tor_circmgr: Circuit established to [scrubbed]
2026-01-29T07:30:18Z  WARN tor_guardmgr: Could not connect to guard [scrubbed]. Retrying...
2026-01-29T07:30:21Z  INFO tor_chanmgr: Opening channel to [scrubbed]
2026-01-29T07:30:24Z  WARN tor_circmgr::hspool: Too many preemptive onion service circuits failed
```

Key indicators:
- `Directory is complete` → Tor network bootstrapped
- `Circuit established` → New circuit active
- `Could not connect to guard` → Guard relay issue
- `Too many preemptive onion service circuits failed` → HS issues

---

## Reference Code Snippets

### Ratatui Basic App Structure
```rust
use std::io;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        execute,
    },
    Terminal,
};

fn main() -> io::Result<()> {
    // Setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Main loop
    loop {
        terminal.draw(|f| {
            // Render UI here
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    println!("awatcher closed cleanly.");
    Ok(())
}
```

### Layout with Multiple Rows
```rust
use ratatui::layout::{Constraint, Direction, Layout};

let main_chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(3),      // Header
        Constraint::Length(8),      // Top row (2 panels)
        Constraint::Length(6),      // Middle row (2 panels)
        Constraint::Min(10),        // Log viewer
        Constraint::Length(1),      // Status bar
        Constraint::Length(1),      // Footer
    ])
    .split(frame.size());

// Split top row into 2 columns
let top_row = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
    .split(main_chunks[1]);
```

---

## Notes for the Implementer

1. **Reuse patterns from zwatcher**: The design is intentionally parallel.
2. **Test incrementally**: Verify each phase before moving on.
3. **Handle macOS quirks**: File watching with `notify` can be tricky.
4. **Graceful degradation**: If data is unavailable, show placeholders.
5. **Ask for clarification**: If any requirement is unclear, ask before implementing.

Good luck! 🧅
