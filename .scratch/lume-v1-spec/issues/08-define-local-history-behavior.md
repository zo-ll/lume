# Define local trace history and persistence behavior

Parent: [Specify Lume v1 causal debugging and trace forking](../map.md)
Type: `grilling`
Status: `resolved`
Blocked by: 01, 03, 06

## Question

What trace, link, checkpoint, intervention, and runtime-availability information must Lume retain locally so users can close and reopen the client, investigate historical activity, and initiate a durable Trace Fork with honest availability and sensitivity cues?

## Answer

Local History is Lume's durable record of what it observed. It preserves accepted and invalid evidence, provenance, user actions, and policy-filtered values so historical investigation does not depend on a connected runtime. It is not authoritative for current runtime state or the existence of runtime-owned checkpoint state.

### Retained evidence

Lume persists the append-only evidence behind its canonical views, including:

- Execution Trace declarations, Operation observations, corrections, producer cursors, finalization, and declared loss;
- quarantined observations and integrity diagnostics;
- source and child Trace Link assertions, including pending and conflicted evidence;
- immutable Fork Checkpoint declarations and timestamped Checkpoint Availability assertions;
- Fork Requests, confirmations, state transitions, rejection and cancellation evidence, deadlines, and idempotency outcomes;
- policy-filtered Intervention audit records and validation issues;
- Agent and runtime identities observed in those records;
- references to locally configured Runtime Recovery Profiles, without embedding credentials or executable launch data; and
- local receipt times, schema versions, provenance, and deletion markers.

Canonical traces, ancestry, checkpoint views, request history, indexes, and summaries are derived from this evidence. Lume may materialize them for performance, but they are rebuildable and are not allowed to erase corrections, conflicts, quarantined records, or provenance.

Runtime-owned restorable execution state remains outside Local History. Lume stores checkpoint declarations and opaque identity metadata, not portable snapshots or sufficient restoration material.

### Default retention and pruning

The default policy is keep until explicit deletion. Closing Lume, ending a trace, runtime disconnection, checkpoint unavailability, elapsed time, or local storage growth never silently expires history.

Users may opt into age- or size-based pruning. Before enabling it, Lume shows:

- which record classes and age or size boundary the policy covers;
- that graph ancestors may become History Tombstones;
- how protected values are handled; and
- that pruning Local History does not delete runtime-owned checkpoint state.

Automated pruning uses the same deletion and tombstone rules as explicit deletion. It records policy identity, deletion time, and affected scope so absence is distinguishable from data that was never observed.

### Reopening Lume

Lume opens Local History before requiring any runtime connection. Users may inspect stored traces, links, checkpoints, requests, integrity issues, and non-sensitive audit data fully offline.

Persisted runtime and checkpoint availability assertions are historical evidence, not current truth. On startup:

- runtime connection state begins unknown;
- previously available checkpoints are presented as unknown until refreshed;
- an active trace remains semantically active but becomes unverified while its owner is unavailable; and
- stored outcomes, completeness evidence, confirmed links, and immutable declarations do not change merely because Lume restarted.

After loading history, Lume may passively reconnect to already configured running endpoints and reconcile evidence. Reopening never launches a runtime, restores a checkpoint, submits a Fork Request, confirms live execution, or performs another action with execution side effects.

### Nonterminal Fork Requests

On restart, Lume automatically queries reachable owners by `fork_request_id` and adopts any durable runtime outcome. This is reconciliation only:

- requests awaiting live confirmation remain awaiting confirmation;
- requests that were confirmed but not known accepted remain visibly nonterminal;
- accepted requests recover their original child trace and link identities;
- Lume does not resubmit, launch, or resume preparation automatically; and
- an explicit user action is required to resume a request that has no authoritative outcome.

If the runtime cannot be reached, Lume preserves the last nonterminal state and reports that reconciliation is unavailable. It does not guess rejection, cancellation, or timeout from restart or elapsed client time.

### Protected Store

Ordinary history and protected values have separate availability boundaries. Ordinary evidence, structural placeholders, sensitivity metadata, and indexes remain in the main local history store. Values marked `protected` are stored only in an encrypted Protected Store unlocked through OS-backed credentials or an explicit user secret.

When the Protected Store is locked, unavailable, or its key is lost:

- ordinary history remains usable;
- protected fields display policy-aware placeholders;
- filtering and comparison never substitute ciphertext, blank text, or stale plaintext for the value;
- Lume does not fall back to plaintext storage; and
- inability to decrypt is distinguished from deliberate deletion or runtime omission.

Unlock and reveal are separate deliberate actions. Unlocking permits policy-governed access but does not automatically reveal every protected value. Revealed plaintext is not copied into ordinary indexes, logs, diagnostics, crash reports, clipboard contents, or persisted frontend state controlled by Lume.

Write-only values never enter either store. Local History records only their field identities and changed markers. Validation errors, confirmations, request outcomes, and exports must not echo them.

### Explicit deletion

Deletion is local, scoped, and non-cascading. Before confirmation, Lume identifies the selected evidence, protected values, derived views, checkpoint declarations, requests, and audit records that will be removed. Descendant or otherwise linked traces are never selected implicitly.

When deleted evidence is still referenced by surviving history, Lume retains a History Tombstone containing only the minimum non-sensitive information required to remain honest:

```text
record_kind
stable_identity
relationship_endpoints_or_anchor_ids
deleted_at
deletion_scope
```

A tombstone contains no trace payload, objective, Intervention value, protected value, restoration reference, diagnostic detail, or executable recovery material. The UI shows that ancestry or an anchor was deleted rather than presenting the surviving record as a root or pretending the evidence never existed.

Deleting a source trace does not delete child traces. Deleting a Fork Checkpoint declaration locally does not delete a Trace Fork already created from it. Existing links retain tombstoned anchors sufficient to preserve graph shape, while detailed shared history or Intervention comparison becomes unavailable.

Lume cannot claim that local deletion removed runtime-owned checkpoint state. Deleting that state is a separate authenticated request to the owning runtime and is reported complete only after runtime confirmation. Local deletion may proceed independently and record that remote state deletion is unknown or was not requested.

### Diagnostic Export

V1 supports explicit, user-selected Diagnostic Export for investigation outside Lume. An export may include:

- selected trace and Operation evidence;
- canonical and pending Trace Links;
- Trace Completeness and integrity diagnostics;
- non-restorable checkpoint metadata;
- Fork Request states and policy-filtered Intervention audit records; and
- tombstones and an export manifest explaining omissions.

Ordinary selected data is included. Protected values are excluded by default and may be included only after the Protected Store is unlocked and the user explicitly selects their inclusion for that export. A general reveal or prior export choice does not create lasting export permission.

Write-only values are never exported. Restoration references, runtime recovery credentials, executable launch data, protected-store keys, and runtime-owned checkpoint state are never exported. Quarantined evidence is opt-in and clearly labeled rather than merged into canonical history.

The export manifest records schema version, creation time, selected scope, excluded sensitivity classes, deletions, and completeness limitations. A Diagnostic Export is evidence for inspection, not a supported Local History backup, import format, or portable Trace Fork archive.

### Acceptance examples

- Lume reopens with no network and immediately displays an ended trace, its confirmed ancestry, and an unavailable protected-value placeholder.
- A checkpoint shown as available before shutdown is shown as unknown until its owner confirms availability again.
- Reopening while a live Fork Request awaits confirmation does not contact or launch its runtime for restoration.
- A previously accepted request with a lost response reconciles to the original child trace rather than creating a duplicate.
- Pruning an old source trace leaves a tombstone so a surviving child still shows deleted ancestry.
- Deleting local checkpoint history does not claim to delete the runtime's restorable state.
- Losing access to the Protected Store leaves ordinary causal structure usable and does not expose plaintext.
- A Diagnostic Export excludes protected values by default and can never include a write-only credential or restoration reference.
