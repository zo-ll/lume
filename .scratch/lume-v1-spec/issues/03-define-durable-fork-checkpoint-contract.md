# Define the durable Fork Checkpoint contract

Parent: [Specify Lume v1 causal debugging and trace forking](../map.md)
Type: `grilling`
Status: `resolved`
Blocked by: 01

## Question

What must an Instrumented Agent Runtime declare, persist, and later recover so Lume can identify a Fork Checkpoint, know whether it remains available, reconnect or relaunch the responsible runtime, and request restoration safely across runtime restarts?

## Answer

A Fork Checkpoint is an immutable declaration that the owning Instrumented Agent Runtime has durably saved restorable execution state at one Causal Cut. Lume persists and presents the declaration, but the runtime owns, stores, interprets, validates, and restores the underlying state.

### Identity and causal position

The runtime assigns every Fork Checkpoint a globally unique opaque `checkpoint_id`. It creates the identity before declaration and preserves it across retransmission, disconnection, process replacement, and runtime restart.

The checkpoint identifies exactly one source `trace_id` and one Causal Cut in that trace. The cut is causally closed: if it includes an Operation, it includes every same-trace structural parent and causal dependency of that Operation. Its contract identifies the complete included Operation set and producer positions semantically; the eventual protocol may choose a compact encoding that preserves this meaning.

The cut, rather than a timestamp or single preceding Operation, defines shared history. Work concurrent with but not included in the cut is not part of the restorable state or the later fork's shared history.

### Immutable checkpoint declaration

Every checkpoint declaration contains the semantic equivalent of:

```text
schema_version
checkpoint_id
source_trace_id
causal_cut
runtime_id
state_identity
restore_reference
intervention_schema
fork_execution_modes[]
recovery_profile_id
created_at
```

- `runtime_id` is the stable identity of the runtime that owns both the source trace and restoration authority.
- `state_identity` is the runtime's immutable identity for the saved state. It lets the runtime detect substitution or corruption; Lume need not understand its encoding.
- `restore_reference` is an opaque handle meaningful only to the owning runtime. It is not portable execution state and grants no authority by itself.
- `intervention_schema` is the immutable typed-editing contract associated with this saved state. Ticket 04 defines its field types, constraints, validation, and sensitivity semantics.
- `fork_execution_modes` declares the modes in which the runtime can restore this checkpoint. The lifecycle and confirmation rules are defined by ticket 05.
- `recovery_profile_id` refers to local, preconfigured recovery behavior; it does not contain executable recovery behavior itself.

The declaration is immutable. Its causal cut, owner, state identity, restoration reference, intervention schema, supported execution modes, and recovery profile reference do not change in place. A semantic change to any of them creates a new checkpoint with a new identity. Historical fork requests therefore continue to mean exactly what they meant when submitted.

Checkpoint Availability is a separate mutable fact and is not part of the immutable declaration.

### Persistence ownership

The runtime durably persists:

- the restorable execution state;
- the mapping from `checkpoint_id`, `state_identity`, and `restore_reference` to that state;
- the state-format and runtime-compatibility information required to validate and restore it;
- the intervention schema and execution-mode capabilities bound to it; and
- enough ownership metadata to reclaim the same logical `runtime_id` after a process restart.

Lume durably persists:

- the immutable checkpoint declaration;
- the source trace evidence and Causal Cut needed to explain its ancestry;
- timestamped availability assertions and their provenance; and
- restoration attempts and their eventual Trace Links once the fork lifecycle is defined.

Lume does not store or interpret the runtime's restorable execution state in v1. A `restore_reference` must therefore never be presented as a portable snapshot, exported as sufficient restoration data, or assumed usable by a different runtime.

### Checkpoint Availability

Availability is evidence about the owner's current ability to restore the checkpoint:

- `available`: the owning runtime recently confirmed that it can locate, validate, and attempt to restore the state.
- `unknown`: Lume has no sufficiently fresh authoritative confirmation.
- `unavailable`: the owning runtime authoritatively confirmed that restoration is impossible.

Every assertion records `checkpoint_id`, `runtime_id`, `checked_at`, and its provenance. An `unavailable` assertion also carries a stable namespaced reason code and may carry redacted diagnostic detail.

Availability is honest rather than optimistic:

- A newly declared checkpoint is available because its owner declares it together with the saved state.
- Runtime disconnection or stale evidence degrades availability to unknown, never directly to unavailable.
- Missing local metadata, a failed connection attempt, or elapsed time cannot prove unavailability.
- Only the owning runtime may authoritatively declare the checkpoint unavailable.
- Availability may later move from unknown or unavailable to available if the owner can again prove restoration capability, but this does not mutate the checkpoint declaration.

`available` is advisory evidence, not a reservation or lease. State can become unavailable between a check and a fork request.

### Runtime recovery

The immutable checkpoint declaration references a Runtime Recovery Profile configured locally outside trace data. The profile identifies the runtime integration and the supported ways Lume may:

- reconnect to an already running owner; or
- launch a new process instance that can reclaim the same logical runtime identity.

Recovery profiles may reference protected local configuration, but checkpoint and trace payloads contain neither credentials nor arbitrary commands to execute. Viewing a trace, checking availability, or selecting a checkpoint never launches a process. Lume invokes recovery only while carrying out an explicit, user-confirmed Trace Fork request.

After reconnect or launch, the process must prove that it represents the declared `runtime_id` through the eventual control protocol. A different runtime cannot claim the checkpoint merely because it possesses the opaque restoration reference.

### Atomic restoration validation

Before creating a new Execution Trace, the runtime atomically revalidates:

- checkpoint identity and runtime ownership;
- the immutable declaration expected by Lume;
- presence and integrity of the saved state;
- state-format and runtime-version compatibility;
- current restoration availability;
- the selected Fork Execution Mode; and
- the Intervention against the checkpoint's bound schema.

An earlier `available` assertion does not bypass this validation. Failure leaves the source trace and checkpoint unchanged, creates no fork trace, and returns a structured rejection suitable for updating availability and explaining the failure. Ticket 05 defines the request states, concurrency behavior, cancellation, and exact point at which a Trace Fork comes into existence.

### Acceptance examples

- A checkpoint taken after two parallel tool calls includes both branches in its Causal Cut when the saved state incorporates both results.
- Restarting the runtime process does not change the checkpoint ID, owner, declaration, or restoration reference.
- Disconnecting the runtime changes an available checkpoint to unknown; it does not claim that the state was lost.
- Deleting or corrupting runtime-owned state allows the owner to declare the checkpoint unavailable with a reason while its immutable historical declaration remains visible.
- Changing the editable input schema or saved state creates a new checkpoint rather than revising the old identity.
- A stopped runtime may be launched through its configured recovery profile only during a confirmed fork request.
- A runtime that reported availability must still reject restoration safely if atomic revalidation later detects loss or incompatibility.
