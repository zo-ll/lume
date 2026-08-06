# Specify Lume v1 causal debugging and trace forking

Label: `wayfinder:map`

## Destination

Reach an implementation-ready behavioral and interface specification for Lume v1: a local-first Rust and Tauri desktop application that lets AI tooling developers observe custom instrumented agents live, understand their behavior through causal Execution Traces, and create durable Trace Forks from runtime-declared Fork Checkpoints using typed Interventions.

The specification defines workflows, trace semantics, instrumentation and control interfaces, desktop behavior, failure handling, and acceptance scenarios without expanding into implementation tasks.

## Notes

- Use the `grilling` and `domain-modeling` skills for human decisions; keep `CONTEXT.md` current as terminology resolves.
- Planning only: do not implement Lume while working this map.
- The Instrumented Agent Runtime owns checkpoint restoration and continued execution. Lume initiates and observes Trace Forks live.
- macOS-specific implementation and acceptance work uses the dedicated `platform/macos` branch and merges only after the macOS suite passes.
- Fork Checkpoints remain addressable across runtime restarts and expose typed editable inputs.
- Fork Execution Mode is runtime-declared as sandboxed or live; Lume makes the mode visible and confirms live execution.
- Sensitive Fields are marked by the runtime and hidden by default.
- Each agent attempt is a separate Execution Trace. Delegation and forking create typed Trace Links.
- The primary comparison experience shares pre-checkpoint history and contrasts original and forked branches after the checkpoint.

## Decisions so far

<!-- Closed ticket resolutions are indexed here by name; decision detail remains in the ticket. -->

- [Define the semantics of an Execution Trace](issues/01-define-execution-trace-semantics.md) — An Execution Trace is a runtime-declared single-Agent attempt built from typed, causally ordered Operations, with lifecycle, outcome, and evidence completeness modeled independently.
- [Assess existing telemetry foundations for Lume](issues/02-assess-telemetry-foundations.md) — OpenTelemetry fits causal observation and cross-trace links, but durable checkpoints, typed interventions, sensitivity policy, live lifecycle fidelity, and fork control require Lume-owned contracts.
- [Define the durable Fork Checkpoint contract](issues/03-define-durable-fork-checkpoint-contract.md) — A Fork Checkpoint is an immutable runtime-owned restoration contract at a Causal Cut, with separate evidence-based availability and safe recovery through a preconfigured local profile.
- [Define the Intervention and Sensitive Field contract](issues/04-define-intervention-contract.md) — An Intervention is a non-empty sparse set of stable, typed checkpoint fields, validated locally and authoritatively by the runtime with explicit protected and write-only handling.
- [Define the Trace Fork lifecycle](issues/05-define-trace-fork-lifecycle.md) — Idempotent Fork Requests move through confirmation, recovery, and preparation to an atomic acceptance boundary; only accepted requests create independent child traces and Fork Links.
- [Define the linked Execution Trace graph](issues/06-define-linked-trace-graph.md) — Delegation and forking create immutable, two-sided Trace Links in a single-origin ancestry forest while every trace retains independent ownership, lifecycle, outcome, and evidence.
- [Prototype the Bubble Tea investigation and fork workflow](issues/07-prototype-bubble-tea-fork-workflow.md) — The prototype validated the staged investigation, fork, navigation, and comparison model; ticket 12 supersedes its terminal implementation and layout constraints.
- [Define local trace history and persistence behavior](issues/08-define-local-history-behavior.md) — Local History retains append-only observed evidence until explicit deletion, reopens offline with honest availability, protects sensitive values separately, and preserves deleted ancestry with tombstones.
- [Choose the instrumentation and control protocol foundation](issues/09-choose-protocol-foundation.md) — Lume uses a versioned Protobuf contract over one authenticated bidirectional gRPC Runtime Session, with resumable at-least-once delivery, explicit capabilities, opaque extensions, and OpenTelemetry adapters at the boundary.
- [Define runtime onboarding and operability](issues/10-define-runtime-onboarding-and-operability.md) — One OS-supervised per-user Lume Service owns Runtime Sessions and Local History, while SDK-first discovery, explicit Runtime Trust, bounded buffering, reviewed recovery profiles, and safe diagnostics define the operational boundary.
- [Consolidate the v1 specification and acceptance suite](issues/11-consolidate-v1-spec-and-acceptance-suite.md) — The normative v1 contract established the cross-platform release and acceptance boundary; ticket 12 supersedes its Go and TUI implementation stack.
- [Pivot v1 to a Rust, Tauri, and Vue desktop application](issues/12-pivot-v1-to-rust-tauri-vue.md) — Rust owns the service, domain, storage, protocol, SDK, and Tauri core; Vue 3 and TypeScript own presentation; the earlier Go/Bubble Tea release stack is superseded.

## Not yet specified

- Final visual styling, panel proportions, production shortcuts, and field-editor details, which the user has deferred to a later design pass.

## Out of scope

- Implementing Lume or producing implementation tickets.
- A browser-hosted web application, mobile application, or production terminal interface.
- Cloud hosting, remote Lume access or authentication, or multi-user operation.
- Generic process watchers, API proxies, and vendor-specific adapters.
- Agent orchestration, task planning, dispatching, scheduling, or coordination.
- Portable cross-machine Trace Fork archives.
