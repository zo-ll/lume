# Define the linked Execution Trace graph

Parent: [Specify Lume v1 causal debugging and trace forking](../map.md)
Type: `grilling`
Status: `resolved`
Blocked by: 01

## Question

What Trace Link types, invariants, ownership rules, and lifecycle behavior let Lume represent delegation to sub-agents and forking from prior traces without confusing one agent's activity with another's or losing causal ancestry?

## Answer

Every Execution Trace remains one Agent's independently owned attempt. Cross-trace causality is represented only by immutable typed Trace Links; linked activity is never nested into, merged with, or re-owned by another trace.

### Core link types

V1 standardizes two directed core link types:

- `delegated`: a source Trace Operation caused another Agent to begin a separate Execution Trace.
- `forked`: a Fork Checkpoint and Intervention caused a new Execution Trace to begin from restored state.

Runtimes may emit namespaced extension link types. Lume preserves and displays unknown extensions, but they do not alter core ancestry, shared-history calculation, branch comparison, or lifecycle behavior unless a future schema version standardizes their semantics.

### Identity and assertions

The source runtime assigns each intended relationship a globally unique opaque `link_id`. A canonical Trace Link is established from two immutable assertions:

```text
source assertion:
  link_id
  link_type
  source_trace_id
  source_anchor
  child_trace_id
  source_runtime_id

child assertion:
  link_id
  link_type
  source_trace_id
  source_anchor
  child_trace_id
  child_runtime_id
```

Each assertion also carries its schema version, runtime occurrence time when available, Lume receipt time, and producer evidence identity. The source assertion is authoritative only when emitted by the owner of the source trace. The child assertion is authoritative only when emitted by the owner of the child trace.

The immutable fields in both assertions must match exactly. The assertions prove different facts: the source owner proves that it initiated the relationship, and the child owner proves that this relationship is the origin of its attempt. Neither runtime may unilaterally attribute activity owned by the other.

When one runtime owns both traces, as in the normal Trace Fork flow, it may emit both assertions atomically while still preserving their separate roles.

### Typed causal anchors

A Delegation Link contains:

```text
source_trace_id
source_operation_id
child_trace_id
```

The source Operation is the exact Operation that initiated the delegated attempt. It belongs to the source trace; the delegated Agent's model calls, tools, lifecycle, and outcome belong only to the child trace. The link targets the declared start of the child trace.

A Fork Link contains:

```text
source_trace_id
checkpoint_id
intervention_record_id
child_trace_id
```

The checkpoint supplies the Causal Cut and therefore the exact shared-history boundary. The Intervention audit record explains the effective change. The link targets the declared start of the Trace Fork. A timestamp or nearby Operation is not an acceptable substitute for either typed anchor.

### Cardinality and graph invariants

For core ancestry, every trace is exactly one of:

- a root trace with no incoming core link;
- a delegated trace with one incoming Delegation Link; or
- a Trace Fork with one incoming Fork Link.

A trace may have any number of outgoing Delegation or Fork Links. A child cannot have two incoming core links, cannot be both delegated and forked, and cannot link to itself. Confirmed core links must form an acyclic ancestry forest, giving every trace one unambiguous path to its root.

Forking a delegated trace does not copy or flatten earlier links. The new trace has one incoming Fork Link to its immediate source trace, and the existing chain preserves the earlier delegation ancestry. Likewise, delegating from a Trace Fork creates a child with one Delegation Link while retaining the fork ancestry through its parent.

Operation dependencies remain inside one trace. Any relationship that crosses a `trace_id` boundary must use a Trace Link rather than an Operation parent or dependency.

### Link evidence lifecycle

A core link has one of three evidence states:

- `pending`: exactly one valid assertion is present, or the referenced counterpart trace has not yet arrived.
- `confirmed`: both authoritative assertions are present and their immutable fields match.
- `conflicted`: assertions reuse a link identity inconsistently, violate authority, disagree on immutable fields, or would violate single-origin or acyclic ancestry.

Confirmation is permanent. Assertions are append-only and cannot be edited into a different relationship. A correction requires new evidence under a new link identity; it cannot silently rewrite confirmed ancestry.

Pending and conflicted evidence remains visible for diagnostics but is excluded from canonical ancestry. Lume does not guess missing relationships from timestamps, objectives, Agent IDs, runtime metadata, or similar-looking activity. A child with an asserted but unconfirmed origin is presented as having unconfirmed ancestry rather than silently treated as a root.

Invalid or conflicting assertions are quarantined under the same evidence principles as invalid trace observations. They do not merge trace contents or destroy otherwise valid traces. Link integrity is reported separately from each trace's own Trace Completeness: a trace may be internally complete while its cross-trace ancestry is pending or conflicted.

### Independent ownership and lifecycle

Trace Links express causality, not containment or control inheritance:

- Each trace retains its own `trace_id`, `agent_id`, `runtime_id`, objective, Operations, lifecycle, outcome, and Trace Completeness.
- A source trace may end before or after a child and does not remain active merely because a child is active.
- Source success does not imply child success, and child failure does not imply source failure.
- Failure or cancellation never propagates automatically across a link. Any runtime policy that requests cancellation of another trace must appear as explicit control and lifecycle evidence rather than a derived graph rule.
- Runtime disconnection affects only the traces owned by that runtime and does not change confirmed links.
- Ending or finalizing either trace does not delete its links; matching link evidence may arrive late and confirm ancestry after trace evidence is finalized.

### Acceptance examples

- A parent Agent delegates twice in parallel, producing two child traces with separate Delegation Links and independent outcomes.
- A delegated child fails while its parent recovers and succeeds; neither outcome is rewritten.
- A fork of a delegated trace has one Fork Link to its immediate source, while traversal through that source still reaches the original delegation.
- A source assertion that arrives before the child trace is pending and becomes confirmed when the child owner supplies a matching assertion.
- A child runtime that claims an unacknowledged parent produces pending, not canonical, ancestry.
- Two assertions with the same link ID but different child traces are conflicted and quarantined from the canonical graph.
- Two otherwise complete traces may remain independently usable even when their proposed link is conflicted.
