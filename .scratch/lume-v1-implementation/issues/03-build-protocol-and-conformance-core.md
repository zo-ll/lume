# Build the protocol and domain conformance core

Parent: [Implement Lume v1 Rust and Tauri desktop application](../map.md)
Type: `task`
Status: `resolved`
Blocked by: 02

## Outcome

Implement the versioned Protobuf/gRPC handshake, capabilities, typed envelopes, acknowledgements, replay identities, extensions, and core Rust domain types. Provide reusable conformance fixtures for trace causality, completeness, links, checkpoints, interventions, sensitivity, and Fork Request idempotency before integrating the full desktop workflow.

## Comments

- The v1 handshake, safety-capability negotiation, observation/control envelopes, durable acknowledgements, replay hashing, opaque extensions, and core trace/link/checkpoint/Intervention/Fork Request types are implemented.
- Initial conformance tests cover negotiation rejection, byte-preserving extensions, partial order, cycle quarantine behavior, completeness, disconnect honesty, sparse values, unsupported field types, causal cuts, single-origin ancestry, and Fork Request idempotency.
- A dedicated `lume-conformance` crate exposes canonical trace, checkpoint, sensitivity, Intervention, handshake, and extension fixtures for the reference runtime and future SDK test paths. The complete release acceptance matrix remains ticket 10 rather than part of this core ticket.

## Answer

Protocol v1 now defines bidirectional Runtime Session negotiation, required safety capabilities, typed observation/control envelopes, durable acknowledgements, replay identities, opaque extensions, links, checkpoints, typed Interventions, and Fork Request results. The protocol-independent Rust domain enforces lifecycle/outcome separation, partial-order causality, cycle rejection, completeness, single-origin ancestry, causal cuts, portable Intervention constraints, unsupported-type refusal, sensitivity classes, and Fork Request idempotency. Reusable conformance fixtures exercise these contracts without depending on the desktop UI.
