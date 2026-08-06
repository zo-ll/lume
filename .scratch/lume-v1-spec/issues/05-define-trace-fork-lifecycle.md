# Define the Trace Fork lifecycle

Parent: [Specify Lume v1 causal debugging and trace forking](../map.md)
Type: `grilling`
Status: `resolved`
Blocked by: 03, 04, 06

## Question

From checkpoint selection through completion, what states and transitions define a Trace Fork, including validation, sandboxed-versus-live confirmation, runtime startup or reconnection, cancellation, rejection, timeout, partial execution, failure, and preservation of ancestry?

## Answer

Lume distinguishes a Fork Request from the Trace Fork it may create. A request describes desired work and may fail without producing a trace. A Trace Fork begins only at Fork Acceptance, the runtime's atomic commit of restored execution, child trace identity, and causal ancestry.

### Fork Request

Lume assigns a globally unique opaque `fork_request_id` before it contacts or launches a runtime. The immutable request contains the semantic equivalent of:

```text
fork_request_id
checkpoint_id
intervention
fork_execution_mode
created_at
acceptance_deadline
confirmation_evidence
```

The request binds one immutable Fork Checkpoint, one validated Intervention, and one checkpoint-supported Fork Execution Mode. Changing any of these creates a new request ID. Reusing a request ID with different immutable content is an integrity error, not an update.

No child `trace_id` exists yet. Rejected, cancelled, and timed-out requests remain request history and never appear as traces that merely failed to start.

### Confirmation

The selected execution mode is visible before submission and cannot be silently changed by Lume or the runtime.

- Submitting a `sandboxed` request is sufficient confirmation because the runtime has declared that restored execution is isolated from live side effects.
- A `live` request enters `awaiting_confirmation`. Lume shows an explicit live-side-effect warning, the checkpoint and source trace, and a policy-filtered summary of changed Intervention Fields. A distinct final user action is required before any recovery or restoration work begins.

Protected values remain masked unless separately revealed, and write-only values appear only as changed markers. Confirmation evidence records the selected mode, confirmation time, and request identity without weakening Sensitive Field policy.

Viewing a checkpoint, editing a form, or abandoning confirmation never starts or contacts a runtime for restoration.

### Request states

A Fork Request moves through explicit operational states:

- `awaiting_confirmation`: a live request is waiting for final user confirmation.
- `recovering_runtime`: Lume is reconnecting to the owner or launching it through the checkpoint's Runtime Recovery Profile.
- `preparing`: the runtime is atomically revalidating ownership, checkpoint state, Intervention, execution mode, deadline, and capacity.
- `accepted`: Fork Acceptance committed exactly one child trace.
- `rejected`: the runtime or Lume authoritatively refused the request before acceptance.
- `cancelled`: cancellation was confirmed before acceptance, and no child was committed.
- `timed_out`: the acceptance deadline passed and reconciliation proved that no child was committed and none may be committed later.

States that do not apply are skipped. A sandboxed request may move directly to runtime recovery; a request whose owner is already connected may move directly to preparation.

`accepted`, `rejected`, `cancelled`, and `timed_out` are terminal. Every state transition is append-only evidence with runtime and receipt times where applicable. Rejection carries a stable stage, reason code, human summary, and policy-filtered details.

If communication is lost after a request may have reached the runtime, Lume retains the last nonterminal state and visibly reports that it is reconciling. Elapsed client waiting time alone cannot produce a terminal result.

### Idempotency and reconciliation

Every delivery, retry, cancellation, status query, and recovery attempt uses the same `fork_request_id`. The runtime durably records request identity and terminal outcome sufficiently to guarantee:

- one request ID creates at most one child trace;
- an accepted request always returns the same child `trace_id` and Fork Link identity;
- a terminal rejection, cancellation, or timeout can never later become accepted; and
- a retry with conflicting immutable content is rejected as an integrity error.

After an uncertain connection failure, Lume queries by request ID before retrying or allowing the user to create a replacement. A delayed response is reconciled with recorded runtime outcome rather than treated as a second transition.

### Runtime recovery and preparation

After confirmation, Lume either uses an existing authenticated owner connection or invokes the checkpoint's Runtime Recovery Profile. The recovered process must prove the checkpoint's stable `runtime_id`; possession of request or restoration references is insufficient.

During `preparing`, the runtime atomically checks:

- request identity, immutability, and acceptance deadline;
- checkpoint ownership, state identity, integrity, compatibility, and current availability;
- Intervention schema identity, portable and domain constraints, and at least one effective change;
- selected Fork Execution Mode and its runtime guarantees;
- single-origin ancestry and intended Fork Link anchors; and
- resources required to commit restored execution.

Any failure before Fork Acceptance rejects the request with no child trace, no Fork Link, and no mutation of checkpoint state. Temporary capacity failure is a rejection of that request; the durable checkpoint remains available unless the owner separately reports otherwise.

### Fork Acceptance

Fork Acceptance is one logical atomic boundary. The runtime durably:

1. assigns the globally unique child `trace_id`;
2. commits an independent restored execution instance with the Intervention applied;
3. records the selected Fork Execution Mode on that instance and child trace;
4. declares the child Execution Trace active; and
5. emits the matching source and child assertions for its Fork Link, anchored by checkpoint and Intervention audit identities.

These facts have one outcome even if their delivery to Lume is partial or reordered. Reconciliation by request ID recovers the same accepted child and link evidence. Execution begins only after this durable acceptance record exists.

After acceptance, the Fork Request remains `accepted`; all subsequent activity belongs to the child Execution Trace. The child has independent lifecycle, outcome, Trace Completeness, runtime availability, and evidence under the contracts defined by tickets 01 and 06.

### Failure and partial execution

The acceptance boundary, not elapsed time or number of emitted Operations, determines classification:

- Restoration, compatibility, validation, or startup failure before commit rejects the request and creates no trace.
- Failure after commit leaves the request accepted and ends the child trace `failed`, even when the child emitted no model or tool Operations.
- Cancellation after commit leaves the request accepted and may later end the child trace `cancelled` through separate trace-lifecycle evidence.
- Lost or partial telemetry affects the child Trace Completeness; it never retroactively converts acceptance into rejection.

Accepted traces, Fork Links, and audit history are never deleted or rolled back to make an early execution failure look like a rejected request.

### Cancellation

Before acceptance, the user may request cancellation by `fork_request_id`. Lume and the runtime stop recovery or preparation where possible, but mark the request `cancelled` only after proving that Fork Acceptance did not commit.

Acceptance and pre-acceptance cancellation race at one boundary:

- if cancellation commits first, no child may be created;
- if acceptance commits first, the request remains accepted.

In the second case, Lume clearly reports that the fork already started. Stopping it requires a separate explicit cancellation request against the child trace. Lume never deletes the trace or pretends live side effects were undone.

### Timeout

A Fork Request carries an acceptance deadline understood and enforced by the runtime. The runtime must not commit acceptance after that deadline. Client-side waiting thresholds may change UI messaging but do not determine semantic outcome.

Lume marks a request `timed_out` only when it knows that:

- the deadline has passed;
- no acceptance was committed under that request ID; and
- the runtime's idempotency record prevents later acceptance.

If acceptance might have committed but its response was lost, the request remains visibly nonterminal until reconciliation proves accepted or not accepted. Timeout is not cancellation and records no user intent to stop work.

### Reusable checkpoints and concurrency

Fork Checkpoints are reusable and non-consuming. Every request restores an independent execution instance from the same immutable state. Successful, failed, rejected, cancelled, or timed-out use does not mutate or invalidate the checkpoint.

Multiple requests against one checkpoint may prepare and execute concurrently. Each has its own request ID, Intervention, execution mode, child trace, and Fork Link. A runtime may reject an individual request for temporary capacity, but it may not serialize all checkpoints by semantic rule or consume checkpoint state on first use.

### Acceptance examples

- A sandboxed request with a connected runtime skips confirmation and recovery, then moves from preparation to accepted.
- A live request cancelled at its warning screen creates no runtime process, request side effect, or child trace.
- A lost acceptance response is retried with the same request ID and returns the original child trace rather than creating a duplicate.
- A cancellation racing with acceptance either proves no child exists or reports the already accepted child; it never hides that child.
- A request whose deadline expired while never reaching a runtime becomes timed out once non-acceptance is provable.
- A runtime that accepted before the deadline but lost its response reconciles as accepted, not timed out.
- A child that fails immediately after acceptance is an accepted Trace Fork with a failed trace and confirmed ancestry.
- Two concurrent requests from one checkpoint may produce two independent child traces with different Interventions or modes.
