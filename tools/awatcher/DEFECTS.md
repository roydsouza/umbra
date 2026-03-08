# Defects - awatcher

**[📍 Back to Map](CONTENTS.md)**


## Open

### [DEFECT-002] Persistent Log Errors in Display
- **Description**: User sees "Unable to launch SOCKS proxy: Address already in use" in the log window even when Arti is running correctly.
- **Root Cause**: 
  - The log file (`arti.log`) is small (< 50KB), so `tail_file` reads from the beginning
  - Historical errors from previous failed start attempts remain in the file
  - Lack of timestamps makes old errors indistinguishable from new ones
- **Evidence**: `lsof` confirms Arti IS running and listening on 9050
- **Suggested Fixes**:
  1. Truncate `arti.log` before starting Arti (clears history)
  2. Configure Arti to emit timestamps in logs
  3. Have `awatcher` seek to end-of-file on startup (skip history)

---

## Closed

### [DEFECT-007] UI Polish - Notification Style
- **Description**: Notice popup had aggressive red background and was oversized (60x20%)
- **Resolution**: Reduced to 50x15%, removed red background, using TOKYO_ORANGE text

### [DEFECT-006] Dependent Apps Showing "arti"
- **Description**: "arti" appeared in dependent apps list
- **Resolution**: Filtered "arti" from `scan_dependent_apps` results

### [DEFECT-005] Blank Notice Message
- **Description**: Notice popup showed empty content
- **Resolution**: Used `Span::styled` with explicit color

### [DEFECT-001] Logs Not Appearing
- **Description**: Log panel was empty
- **Resolution**: Fixed absolute path to `arti` binary and log file output redirection