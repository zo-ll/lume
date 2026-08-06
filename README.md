# lume

A local-first desktop application for causally debugging instrumented AI agents and creating durable trace forks.

See the [Lume v1 specification](./.scratch/lume-v1-spec/spec.md) for the implementation-ready behavioral and interface contract. The [Wayfinder map](./.scratch/lume-v1-spec/map.md) indexes its decision record.

Implementation sequencing is tracked in the [Rust/Tauri/Vue implementation map](./.scratch/lume-v1-implementation/map.md). Desktop experience design is the first blocking step before production scaffolding.

Developer prerequisites, workspace boundaries, and commands are documented in [Development](./docs/development.md).

## Status

Pre-alpha. The nonvisual Rust/Tauri/Vue workspace, protocol/domain conformance core, and headless runtime-to-service ingestion path are under implementation. Production desktop presentation remains gated by design review.

## Availability

There is no runnable release yet. The reference runtime and service can exercise the development ingestion path; packaging, production local IPC, service supervision, protected storage, control workflows, and the designed Vue investigation experience remain incomplete.
