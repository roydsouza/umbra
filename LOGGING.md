# Umbra Logging Architecture

**[📍 Back to Map](CONTENTS.md)**


This document clarifies the various log files you may encounter in `umbra/var/log/`.

## 1. The Canonical Log (Trust This One)
**File**: `umbra/var/log/arti.log`
*   **Source**: Generated internally by the Arti daemon.
*   **Configuration**: Defined in `umbra/etc/arti.toml` under `[logging]`.
*   **Behavior**:
    *   **Rotation**: Rotates daily (`arti.log.YYYY-MM-DD`).
    *   **Retention**: Keeps 5 historical copies.
    *   **Content**: Structured info/warn messages. This is the source of truth.

## 2. Shell Capture Logs (Context)
These files are created by wrapper scripts (like `start-arti` or manual commands) to capture "Process Noise" that happens *before* Arti initializes its internal logger, or crashes that bypass it.

*   **`arti.err` / `arti.err.log`**: Standard Error (stderr).
    *   Captures panics, config parsing errors, or "thread crashed" messages.
    *   *Note*: If you see multiple versions (`.err`, `.err.log`), they are likely from different launch methods (e.g., one used `2> arti.err`, another `2> arti.err.log`).
*   **`arti.out.log`**: Standard Output (stdout).
    *   Usually empty because `arti.toml` sets `console = "off"`.

## 3. Wrapper Logs
*   **`start-arti.log`**: Output from the bash launcher script itself (e.g., echoes like "Starting with profile...").

## 4. Profiles
When you run `./bin/start-arti`, it defaults to the **`default`** profile defined in `umbra/etc/arti-profiles.toml`.

```toml
[profiles.default]
description = "Standard Arti - no experimental features"
cgo = false
```

## Summary
| File | Reliability | Source | Usage |
| :--- | :--- | :--- | :--- |
| **`arti.log`** | ⭐⭐⭐⭐⭐ | **Internal** | **Primary Debugging / Auditing** |
| `arti.err.log` | ⭐⭐⭐ | Shell | Startup Crashes / Panics |
| `arti.out.log` | ⭐ | Shell | Usually Empty |