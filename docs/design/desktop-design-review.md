# Desktop design acceptance review

Reviewed: 2026-08-06

Artifact: [`lume-desktop-design.html`](lume-desktop-design.html)

The approved Claude Design artifact is the implementation reference for the Lume
desktop investigation and Trace Fork experience. It is self-contained so the
reviewed version remains reproducible without an external design-service link.

## Acceptance result

The design passes ticket 01 and covers the desktop acceptance contract:

- H01: the investigation workspace combines linked-trace navigation, a causal
  outline, Operation selection, inspection, and contextual keyboard and pointer
  actions.
- H02: the wide and compact compositions retain trace identity, lineage, safety
  state, selection, inspector content, and primary actions without overlap.
- H03: checkpoint selection, typed Intervention fields, execution mode, and live
  confirmation are separate stages that preserve request context.
- H04: recovery and preparation render only the Fork Request. A child Trace and
  Fork Link appear only after acceptance.
- H05: accepted children retain request, link, execution mode, outcome, and live
  append context, including immediate child failure.
- H06: lineage navigation switches between independently owned traces instead of
  nesting one trace's Operations inside another.
- H07: branch comparison renders the shared prefix once and the independently
  ordered original and fork suffixes side by side.
- H08: protected, write-only, unavailable, deleted, and integrity-limited evidence
  use explicit text and structural cues rather than blank values or color alone.
- H09: the component and view-model inventory leaves validation, completeness,
  sensitivity, trust, authorization, and fork lifecycle decisions in Rust. Vue
  owns only transient interaction and draft state.
- H10: offline/window-independent states treat the desktop as a client of the
  separately running Lume Service; closing a window does not imply stopping trace
  capture.

The design additionally covers visible focus, semantic tree behavior, live-region
updates, command discovery, empty/loading/degraded/offline/service-error states,
and no-color safety signals.

## Intentionally flexible details

Implementation may tune exact pane proportions and the approximate 1100 px compact
breakpoint, long-trace duration scaling, non-tree causal-edge hairlines, accent
hues, and the exact text required for live confirmation. Final key combinations
may vary, but the documented navigation model must remain intact.

## Follow-up design scope

These product areas are intentionally not blockers for ticket 01 and need their
own focused design before their implementation tickets close:

- Runtime Trust and Runtime Recovery Profile review screens
- Protected Store unlock and reveal flows
- Diagnostic Export scope and omission-manifest review
- Telemetry Projection settings
- quarantined-evidence review
- dense-trace search and filtering
- tray or menu-bar behavior while no window is open

## Implementation boundary

The HTML artifact is a visual and interaction reference, not production source.
Production UI is implemented as Vue components backed by purpose-built Rust view
models. The frontend must not copy domain decisions, receive write-only values, or
persist revealed protected values.
