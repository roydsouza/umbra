# Tasks - awatcher

**[📍 Back to Map](CONTENTS.md)**


## UI Polish
- [x] **Fix Notification UI**: 
    - ~~The current "Notice" popup (Red background) is too aggressive/ugly.~~
    - Reduced size to 50x15%, using TOKYO_ORANGE text on dark background.

## Investigation
- [ ] **Investigate Persistent Log Errors**:
    - User reports "Unable to launch SOCKS proxy" errors persist in the log window.
    - **Hypothesis**: `awatcher` might be reading the entire log file from the beginning (if file is < 50KB), displaying old errors from previous failed starts.
    - **Action**: Check if log timestamps are recent. If generic "Unable to bind", check if orphan `arti` processes are valid or zombies.