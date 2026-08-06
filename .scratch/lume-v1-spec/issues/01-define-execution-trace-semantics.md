# Define the semantics of an Execution Trace

Parent: [Specify Lume v1 causal debugging and trace forking](../map.md)
Type: `grilling`
Status: `resolved`

## Question

What are the canonical contents, operation types, identities, causal ordering rules, lifecycle states, outcomes, and failure or partial-data semantics of a single-agent Execution Trace so every later protocol and interface decision has one stable behavioral model?

## Answer

An Execution Trace is one runtime-declared attempt by one Agent to achieve one objective. The Instrumented Agent Runtime explicitly starts and ends it; Lume never infers trace boundaries from process lifetime, inactivity, timestamps, UI sessions, or missing traffic. A runtime disconnection does not end a trace or invent an outcome.

### Identity and ownership

- The runtime assigns every Execution Trace a globally unique, opaque `trace_id` before emitting it. The identity remains stable across retransmission, disconnection, and runtime restart.
- Every trace references one stable runtime-supplied `agent_id`. The Agent may own many traces; each trace is one attempt.
- Every trace has one stable runtime owner. New process instances may reconnect as that same logical runtime, but v1 does not transfer an active trace to a different runtime owner.
- Every Trace Operation has a runtime-assigned opaque `operation_id` unique within its trace. External references use `(trace_id, operation_id)`.

### Canonical trace envelope

Every trace contains:

```text
schema_version
trace_id
agent_id
runtime_id
objective
started_at
lifecycle
outcome?            # present only when ended
completeness
operations[]
labels?             # optional portable or namespaced metadata
```

`objective` is a required concise, human-readable description supplied by the runtime. Original prompts and structured machine inputs are separate payload data rather than substitutes for the objective. They may have independent sensitivity and availability.

`started_at` is the runtime-declared occurrence time. Lume also retains its own receipt time for every observation. Runtime time is useful presentation evidence but never overrides explicit causality.

### Trace Operations

A Trace Operation is the canonical unit of activity. It has a shared structural envelope:

```text
operation_id
type
structural_parent_id?
causal_dependency_ids[]
producer_id
producer_sequence
occurred_at?
received_at
lifecycle
outcome?                  # present only when ended
failure?                  # required when failed
cancellation?             # required when cancelled
payload
```

The v1 core Operation types are:

- `agent_step`: an optional runtime-declared unit of agent control. It may carry a display label, concise intent, and result summary, and groups the calls caused by that step. Full internal reasoning is not required.
- `model_call`: standardizes provider, model identity, request content, response content, finish reason, and token usage.
- `tool_call`: standardizes tool name, the runtime's call reference, arguments, result, and structured error details.

The core fields are semantic, versioned contracts. Runtimes may add namespaced Operation types and namespaced fields. Lume preserves and displays unknown extensions without pretending to understand them.

Every payload field explicitly reports one availability state: `present`, `redacted`, `omitted`, `truncated`, or `unavailable`. A core Operation remains structurally valid when payload data is intentionally unavailable; complete payload capture is not required.

### Structure, causality, and ordering

- An Operation has at most one structural parent for stable navigation and presentation. Multiple root Operations are valid when the objective directly causes parallel top-level work.
- An Operation may additionally name zero or more causal dependencies. This represents joins, such as a model call that consumes the results of two parallel tool calls, without distorting the display hierarchy.
- Structural parents and causal dependencies must refer to Operations in the same trace, must not refer to the Operation itself, and together must remain acyclic. Cross-trace causality is represented by Trace Links rather than Operation references.
- A monotonically increasing sequence orders observations from the same producer. Operations from different producers or concurrent sibling branches remain partially ordered unless an explicit dependency orders them.
- Runtime and receipt timestamps never establish causal order and never repair a missing causal relationship.

### Lifecycle and outcomes

Lifecycle, outcome, runtime availability, and Trace Completeness are independent facts.

A trace or Operation is either `active` or `ended`. Only an ended item has one terminal outcome:

- `succeeded`: its runtime-defined success condition was reached.
- `failed`: it ended because of an error or unrecoverable condition.
- `cancelled`: it was deliberately stopped before completion.

A failed Operation does not automatically fail its trace. The Agent may recover from a failed model or tool call and the runtime may later declare the trace succeeded. Trace outcome is always an explicit runtime declaration, not a value derived by Lume from child Operations.

A `failed` outcome requires a Failure record with a stable namespaced code and concise summary. It may reference causing Operations and carry typed details. A `cancelled` outcome requires a Cancellation record with an initiator category and stable reason code; it may include a summary and causal Operation reference. Diagnostic payloads may be redacted, but their classification remains visible.

### Append-only evidence and corrections

Runtimes emit immutable observations from a small lifecycle core: starts, ends, explicit corrections or supersessions, declared loss, and trace finalization. Logs, progress, streamed chunks, cache activity, and similar point-in-time data use namespaced event types attached to a Trace Operation.

Lume derives the current trace view from this append-only evidence:

- Re-delivery of the same observation identity and content is idempotent.
- Reuse of an observation identity with different content is an integrity error.
- A correction is a new observation that explicitly supersedes prior evidence; it never overwrites history.
- Ending a trace and finalizing its evidence are separate acts. Finalization declares the last valid sequence cursor for every producer.
- Observations that arrive late may fill positions at or below a declared final cursor. New observations beyond a final cursor are invalid.

### Trace Completeness

Trace Completeness describes the quality of the causal evidence, not whether the objective succeeded:

- `provisional`: the trace is active, its owner is available, and no structural loss is known, so more valid evidence may arrive.
- `complete`: the trace ended, every producer supplied a final cursor, all required evidence through those cursors is present, and the canonical graph has no integrity errors.
- `incomplete`: structural evidence is known to be lost or invalid, including sequence gaps, missing required lifecycle records, unresolved causal references, cycles, or runtime-declared loss.
- `unverified`: Lume lacks enough evidence to prove completeness or known loss, such as when the runtime is unavailable without valid finalization.

Known structural loss takes precedence over provisional or unverified status. Declared redaction, omission, truncation, or unavailability of payload fields does not by itself make a trace structurally incomplete.

Completeness may improve when delayed valid evidence fills a gap. A disconnected active trace remains active with no outcome; it becomes unverified until its owner reconnects or supplies valid finalization.

### Invalid evidence

Lume quarantines malformed, conflicting, out-of-range, cyclic, or otherwise invalid observations. Quarantined evidence is retained for diagnostics but excluded from the canonical trace view. The valid remainder of the trace stays usable, an integrity issue is visible to the user, and Trace Completeness reflects any resulting structural loss.

### Acceptance examples

- A trace with two parallel tool calls and a later model call represents the tools as concurrent siblings and the model call as depending on both.
- A tool call may fail, be followed by a successful recovery step, and belong to a trace that ends `succeeded`.
- A runtime may disconnect and reconnect without changing `trace_id`, lifecycle, ownership, or prior evidence.
- A successfully ended trace with a missing producer sequence is `incomplete`, not failed.
- An ended trace without valid final producer cursors is `unverified`, even if its outcome is `succeeded`.
- A delayed observation at or below a final cursor may repair a gap; an observation beyond that cursor is quarantined.
