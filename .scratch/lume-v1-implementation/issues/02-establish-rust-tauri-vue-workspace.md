# Establish the Rust, Tauri, and Vue workspace

Parent: [Implement Lume v1 Rust and Tauri desktop application](../map.md)
Type: `task`
Status: `resolved`

Dependency note: the user authorized the nonvisual workspace foundation to proceed before ticket 01 is reviewed. Production Vue presentation and interaction work remain blocked by ticket 01.

## Outcome

Create the minimal buildable workspace for shared Rust crates, the independently runnable Lume Service, Rust SDK and reference runtime, Tauri 2 application, and Vue 3/TypeScript/Vite frontend. Pin toolchains and dependency policy, define generated-code boundaries, and provide one command for formatting, linting, testing, and development startup without implementing product behavior prematurely.

## Comments

- The workspace, lockfiles, generated-code boundary, unified commands, static Vue build, and neutral Tauri shell are implemented.
- Headless Rust tests, native Tauri Clippy, Vue template type-checking, and the static production build pass through one `make check` command.

## Answer

The buildable Rust 1.97.1 workspace now separates domain, conformance, protocol, storage, service, SDK, reference runtime, Tauri, and Vue ownership. Cargo and pnpm lockfiles pin resolved dependencies; Protobuf generation uses a vendored compiler and Cargo `OUT_DIR`; project-local commands cover formatting, linting, tests, native desktop verification, static frontend builds, and development startup. The Vue application intentionally remains a neutral shell until ticket 01 supplies the approved presentation contract.
