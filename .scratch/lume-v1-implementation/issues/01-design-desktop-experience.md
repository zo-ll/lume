# Design the desktop investigation and fork experience

Parent: [Implement Lume v1 Rust and Tauri desktop application](../map.md)
Type: `prototype`
Status: `resolved`

## Question

What desktop information architecture, visual language, interaction model, Vue component boundary, and adaptive-window behavior make Lume's live causal investigation and staged Trace Fork workflow clear, efficient, accessible, and faithful to the normative safety states?

## Required evidence

- Wide and compact investigation workspaces showing trace graph, timeline, inspector, integrity, completeness, trust, and runtime state.
- Checkpoint selection, typed Intervention editing, sandboxed/live mode selection, protected/write-only presentation, and live confirmation.
- Fork Request recovery, preparation, uncertainty, rejection, cancellation, acceptance, and immediate child failure.
- Linked-trace navigation and shared-prefix branch comparison.
- Keyboard and pointer flows, visible focus, semantic labels, empty/loading/error/offline states, and no color-only safety cues.
- A component and view-model inventory that keeps domain decisions in Rust and presentation state in Vue.

## Exit condition

Resolve only after the reviewed design covers every desktop acceptance state in the v1 specification and records which visual details remain intentionally flexible during implementation.

## Resolution

The approved, self-contained design is preserved at
[`docs/design/lume-desktop-design.html`](../../../docs/design/lume-desktop-design.html).
The acceptance review is recorded alongside it in
[`docs/design/desktop-design-review.md`](../../../docs/design/desktop-design-review.md).

The review found explicit coverage for H01-H10: wide and compact investigation,
staged fork creation, request-versus-child lifecycle, live updates, linked-trace
navigation, shared-prefix comparison, sensitive presentation, Rust-owned domain
authority, and the independent service/window boundary. The design also records
the visual and interaction details that remain flexible during implementation.
