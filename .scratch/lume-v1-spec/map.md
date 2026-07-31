# Specify Lume v1 causal debugging and trace forking

Label: `wayfinder:map`

## Destination

Reach an implementation-ready behavioral and interface specification for Lume v1: a local-first Bubble Tea TUI that lets AI tooling developers observe custom instrumented agents live, understand their behavior through causal Execution Traces, and create durable Trace Forks from runtime-declared Fork Checkpoints using typed Interventions.

The specification defines workflows, trace semantics, instrumentation and control interfaces, TUI behavior, failure handling, and acceptance scenarios without prescribing internal architecture or implementation tasks.

## Notes

- Use the `grilling` and `domain-modeling` skills for human decisions; keep `CONTEXT.md` current as terminology resolves.
- Planning only: do not implement Lume while working this map.
- The Instrumented Agent Runtime owns checkpoint restoration and continued execution. Lume initiates and observes Trace Forks live.
- Fork Checkpoints remain addressable across runtime restarts and expose typed editable inputs.
- Fork Execution Mode is runtime-declared as sandboxed or live; Lume makes the mode visible and confirms live execution.
- Sensitive Fields are marked by the runtime and hidden by default.
- Each agent attempt is a separate Execution Trace. Delegation and forking create typed Trace Links.
- The primary comparison experience shares pre-checkpoint history and contrasts original and forked branches after the checkpoint.

## Decisions so far

<!-- Closed ticket resolutions are indexed here by name; decision detail remains in the ticket. -->

- [Assess existing telemetry foundations for Lume](issues/02-assess-telemetry-foundations.md) — OpenTelemetry fits causal observation and cross-trace links, but durable checkpoints, typed interventions, sensitivity policy, live lifecycle fidelity, and fork control require Lume-owned contracts.

## Not yet specified

- Onboarding, configuration, and day-two operability details that depend on the eventual protocol and runtime connection model.
- Retention, deletion, and export policy details that depend on the persistence and Sensitive Field decisions.
- Protocol compatibility and evolution concerns that cannot be made concrete until the protocol foundation is chosen.
- The final acceptance-scenario suite, which depends on the trace, fork, persistence, and TUI workflows.

## Out of scope

- Implementing Lume or producing implementation tickets.
- A desktop, web, or mobile interface.
- Cloud hosting, remote Lume access, authentication, or multi-user operation.
- Generic process watchers, API proxies, and vendor-specific adapters.
- Agent orchestration, task planning, dispatching, scheduling, or coordination.
- Portable cross-machine Trace Fork archives.
