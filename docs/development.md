# Development

## Prerequisites

Lume pins Rust in `rust-toolchain.toml`, JavaScript tools in `package.json`, and resolved dependencies in `Cargo.lock` and `pnpm-lock.yaml`.

On Fedora, install the native Tauri prerequisites:

```bash
sudo dnf install webkit2gtk4.1-devel openssl-devel curl wget file libappindicator-gtk3-devel librsvg2-devel
```

Install Rust with rustup, then enable pnpm through the Node Corepack installation:

```bash
rustup show
corepack pnpm --version
corepack pnpm install --frozen-lockfile
```

Protobuf compilation uses `protoc-bin-vendored`; a system `protoc` is not required and generated Rust remains in Cargo's `OUT_DIR`.

## Commands

```bash
make check          # Rust formatting, Clippy, tests, Vue type-check, static build
make check-headless # all non-Tauri Rust crates
make check-desktop  # native Tauri core; requires platform libraries
make dev            # Vite and the Tauri development application
```

Run the headless walking skeleton in separate terminals:

```bash
LUME_AUTH_TOKEN=development cargo run -p lume-service
LUME_AUTH_TOKEN=development cargo run -p lume-reference-runtime
```

Set `LUME_ENDPOINT` and `LUME_HISTORY` to isolate development and test services. The TCP endpoint and environment token are development transport only; production local IPC, identity proof, service supervision, and credential storage remain later implementation work.

## Dependency policy

- Commit both lockfiles and review dependency changes explicitly.
- Rust workspace dependencies are declared once at the root. Individual crates opt into only what they use.
- Frontend runtime and build dependencies use exact versions because the application ships as one bundled artifact.
- TypeScript is pinned to `5.9.3`, the newest version verified with `vue-tsc 3.3.9`. TypeScript 7.0.2 currently breaks the checker by removing its internal `./lib/tsc` export. Upgrade when Vue Language Tools publishes a compatible checker; do not remove Vue template type-checking to force the upgrade.
- Protobuf field and enum numbers are never reused. Deleted fields must be reserved, and incompatible semantics require a new protocol version.
- Generated Protobuf Rust is build output, never hand-edited or committed.

## Workspace boundaries

- `lume-domain`: protocol-independent identities, causal semantics, completeness, links, checkpoints, Interventions, and Fork Request idempotency.
- `lume-conformance`: reusable canonical fixtures and assertions shared by the first-party runtime and future SDK conformance paths.
- `lume-protocol`: the versioned Protobuf/gRPC contract, generated bindings, negotiation, and wire hashing.
- `lume-storage`: append-only SQLite Local History primitives and quarantine. Its schema is intentionally distinct from the live protocol.
- `lume-service`: the independent receiver and sole Local History writer.
- `lume-sdk`: runtime-side connection, negotiation, bounded buffering, delivery, and acknowledgement behavior.
- `lume-reference-runtime`: executable conformance path using the first-party SDK.
- `lume-desktop`: the Tauri privileged boundary.
- `@lume/desktop-ui`: the static Vue/Vite application.

Rust owns domain truth, validation, persistence, sensitivity, trust, and authorization. Vue owns presentation and transient interaction state only.

## Current design boundary

The Vue application deliberately contains only a neutral placeholder. Do not add production layout, investigation components, view models, styling, or interactions until the completed desktop design is transferred into the repository and ticket 01 passes review. The headless runtime-to-service path may continue independently.
