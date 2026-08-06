# Pivot v1 to a Rust, Tauri, and Vue desktop application

Parent: [Specify Lume v1 causal debugging and trace forking](../map.md)
Type: `grilling`
Status: `resolved`
Blocked by: 07, 10, 11

## Question

Should Lume remain a Go and Bubble Tea terminal application, add a desktop client alongside it, or make a Rust and Tauri desktop application the primary v1 product; and if Tauri is selected, does its webview need a frontend framework?

## Answer

Lume v1 pivots fully to a desktop-first implementation. Rust owns the Lume Service, domain model, storage, protocol implementation, Tauri native core, first-party instrumentation SDK, reference runtime, and conformance fixtures. Tauri 2 is the desktop shell. Vue 3 with TypeScript and Vite renders the static single-page interface.

Go and Bubble Tea are no longer part of the production v1 stack. The language-neutral Protobuf/gRPC contract remains unchanged, so future Python, TypeScript, or other runtime SDKs can implement the same conformance contract. The first-party v1 SDK and reference runtime are Rust.

Vue is a presentation boundary, not a second domain implementation. Rust remains authoritative for trace derivation, completeness, trust, sensitivity, validation, fork lifecycle, persistence, and control authorization. The frontend receives purpose-built view models and batched deltas, holds only transient interaction state, and cannot acquire direct Local History, Protected Store, runtime credential, or recovery-profile access.

This boundary follows Tauri's separation between the privileged Rust core and OS webview processes, while Vue supplies declarative components and reactivity for the state-heavy investigation interface. Vite produces the static frontend assets expected by the Tauri client. See the official [Tauri process model](https://v2.tauri.app/concept/process-model/), [Tauri frontend guidance](https://v2.tauri.app/start/frontend/), [Vue 3 guide](https://vuejs.org/guide/introduction.html), and [Vite guide](https://vite.dev/guide/).

The Tauri window and the Lume Service remain separate processes. Closing every window does not stop Runtime Sessions or trace capture. Packaging may bundle the service binary with the desktop application, but installation registers it as the independently supervised per-user service defined by ticket 10.

The Bubble Tea prototype in ticket 07 remains primary-source interaction research. Its investigation-first progression, explicit Fork Request states, linked navigation, and shared-prefix comparison carry forward. Its terminal layout, responsive column rules, keybindings, and implementation do not constrain the production desktop interface.

Linux and macOS remain the required v1 release platforms, and macOS-specific development remains isolated on `platform/macos`. Windows remains a preserved protocol and packaging target after v1 rather than a v1 release blocker.

This decision supersedes the implementation-stack portions of tickets 07, 10, and 11. Their domain, service-lifecycle, workflow, and acceptance decisions remain valid unless this ticket explicitly replaces them.
