# Consolidate the v1 specification and acceptance suite

Parent: [Specify Lume v1 causal debugging and trace forking](../map.md)
Type: `grilling`
Status: `resolved`
Blocked by: 01, 02, 03, 04, 05, 06, 07, 08, 09, 10
Superseded in part by: 12

## Question

What exact release boundary, reference integration, interoperability commitment, and acceptance evidence turn the resolved Lume v1 decisions into one coherent implementation-ready behavioral and interface specification?

## Answer

> **Supersession:** Ticket 12 replaces the Go SDK, Go reference runtime, and TUI implementation portions of this resolution with Rust, Tauri 2, Vue 3, TypeScript, and Vite. The release platforms, protocol, OTLP boundary, and domain acceptance requirements remain in force.

The consolidated normative contract is [`../spec.md`](../spec.md).

V1 ships a language-neutral native protocol, first-party Go SDK, Go reference runtime, and reusable conformance suite. Outbound OTLP Telemetry Projection is required to prove the interoperability boundary; OTLP ingestion and additional first-party SDK languages are deferred. Linux and macOS define the release platform matrix, while the Windows local-transport contract is preserved for later implementation.

macOS-specific development is isolated on the dedicated `platform/macos` branch and merges only after that branch passes the full macOS acceptance matrix. It implements the same native protocol and domain semantics; branch isolation cannot introduce a divergent platform contract.

The acceptance suite combines automated protocol, trace, link, checkpoint, Intervention, fork, persistence, trust, service, and export scenarios with a focused manual TUI smoke pass. A release candidate must pass the same domain conformance suite through the Go SDK and reference runtime on both required platforms. Identity, sensitivity, confirmation, idempotency, ancestry, completeness, and deletion-honesty failures are release blockers.

Final styling remains deliberately deferred, but no visual-design choice may erase or merge the required workflow states, safety cues, evidence limitations, or responsive interaction structure.
