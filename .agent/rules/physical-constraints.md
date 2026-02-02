# Schwarzschild Radius (Operational Constraints) - (The "How")


* **Context Truncation:** When analyzing files in `arti-upstream`, the agent MUST only read relevant blocks (struct definitions, impl blocks). Do not ingest more than 300 lines of a single file at once.
* **State Persistence:** Always summarize the "State of the Circuit" before concluding a turn to ensure context is preserved if the window resets.
* **M5 Memory Limit:** Limit simultaneous active file buffers to 5 to maintain IDE responsiveness on the 24 GB unified memory.
* **Tracing Priority:** Focus primarily on identifying `tracing::instrument` and `event!` macros within `tor-proto` and `tor-circmgr`.

