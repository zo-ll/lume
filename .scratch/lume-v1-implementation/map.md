# Implement Lume v1 Rust and Tauri desktop application

Label: `wayfinder:map`

## Destination

Ship the Linux and macOS Lume v1 defined by the [normative product specification](../lume-v1-spec/spec.md): a Rust Lume Service, language-neutral Protobuf/gRPC protocol, first-party Rust SDK and reference runtime, and a Tauri 2 desktop client with a Vue 3, TypeScript, and Vite interface.

The release is complete only when the required domain, protocol, persistence, sensitivity, desktop, OTLP, packaging, and platform acceptance scenarios pass.

## Notes

- Product semantics are fixed by the v1 specification. Implementation discoveries that contradict it must return to the specification explicitly rather than silently changing behavior.
- Complete the desktop experience design before scaffolding production code. The user may create this design with Claude; the resulting artifact must be reviewed against the normative workflow and safety states before ticket 01 resolves.
- The approved desktop design is preserved in `docs/design/` and unblocks production Vue presentation, interaction behavior, and design-dependent view models.
- Rust owns domain truth, storage, protocol behavior, authorization, and Tauri commands. Vue owns presentation and transient interaction state only.
- The Lume Service is an independently supervised per-user process. Closing the Tauri application must not stop trace capture.
- Shared and Linux implementation proceeds on `main`. macOS-specific service, IPC, credential-store, signing, packaging, and acceptance work proceeds on `platform/macos`.
- `platform/macos` must regularly incorporate shared `main` changes and must not introduce branch-only domain or protocol semantics. Merge it only after the macOS acceptance matrix passes.
- Work in vertical slices that produce observable end-to-end behavior. Avoid completing isolated layers that cannot yet be exercised through the reference runtime and desktop client.
- Preserve the Bubble Tea prototype branch as interaction research; do not merge its Go prototype into production.

## Decisions so far

<!-- Resolved implementation tickets are indexed here. Decision detail remains in each ticket. -->

- [Desktop investigation and fork experience](issues/01-design-desktop-experience.md): the approved wide/compact workspace, staged fork lifecycle, linked navigation, accessibility model, and Rust/Vue ownership boundary cover the desktop acceptance contract.
- [Workspace foundation](issues/02-establish-rust-tauri-vue-workspace.md): a pinned Rust/Tauri/Vue workspace separates privileged, domain, protocol, persistence, runtime, and presentation boundaries behind one verification command.
- [Protocol and conformance core](issues/03-build-protocol-and-conformance-core.md): protocol v1, core Rust semantics, and reusable fixtures establish the typed causal and control foundation.

## Initial frontier

- [Design the desktop investigation and fork experience](issues/01-design-desktop-experience.md)

## Fog

- Final desktop visual language, information density, navigation, component inventory, and adaptive-window behavior.
- Exact Rust crate boundaries, storage engine, migration strategy, async runtime, and generated-protocol layout.
- Vue state organization, graph rendering, list virtualization, and Rust-to-frontend delta batching.
- Linux and macOS installer, service registration, signing, notarization, update, and recovery details.
- Performance budgets and fixture scale for dense, long-running causal traces.

## Out of scope

- Reopening settled v1 product semantics without a demonstrated contradiction.
- A production terminal, browser-hosted web, mobile, cloud, remote-access, or multi-user client.
- Windows implementation for v1.
- Additional first-party runtime SDK languages before the Rust conformance path passes.
- Portable checkpoint archives, runtime orchestration, or vendor-specific agent adapters.
