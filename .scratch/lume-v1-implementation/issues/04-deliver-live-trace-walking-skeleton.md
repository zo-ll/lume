# Deliver the live-trace walking skeleton

Parent: [Implement Lume v1 Rust and Tauri desktop application](../map.md)
Type: `task`
Status: `claimed`
Blocked by: 03

## Outcome

Deliver the first complete vertical slice: the Rust reference runtime emits one causal trace through the Rust SDK, the independently running Lume Service authenticates and durably records it, and the Tauri/Vue client attaches later and renders the live and historical trace. Include reconnect, acknowledgement, replay deduplication, service restart, and client-window independence.

## Comments

- The authorized headless portion is implemented: a reference runtime delivers through the Rust SDK to an authenticated independent gRPC service and append-only SQLite history.
- Tests cover durable contiguous acknowledgement, identical replay deduplication, changed replay quarantine, database restart persistence, and bounded SDK buffering. A real separate-process smoke test persisted one four-envelope trace and acknowledged position four.
- The Tauri/Vue attachment and rendering portion is intentionally paused at the approved design boundary, so this ticket remains claimed.
