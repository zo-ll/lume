# Deliver the live-trace walking skeleton

Parent: [Implement Lume v1 Rust and Tauri desktop application](../map.md)
Type: `task`
Status: `open`
Blocked by: 03

## Outcome

Deliver the first complete vertical slice: the Rust reference runtime emits one causal trace through the Rust SDK, the independently running Lume Service authenticates and durably records it, and the Tauri/Vue client attaches later and renders the live and historical trace. Include reconnect, acknowledgement, replay deduplication, service restart, and client-window independence.
