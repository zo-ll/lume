# Lume v1 behavioral and interface specification

Status: implementation-ready product contract

This specification defines Lume v1: a local-first desktop application for observing custom instrumented AI agents as causal Execution Traces and creating durable Trace Forks from runtime-owned Fork Checkpoints. It consolidates the resolved decision tickets in this effort. Those tickets retain rationale and edge-case detail; this document is the normative release boundary.

The terms **MUST**, **MUST NOT**, **SHOULD**, **SHOULD NOT**, and **MAY** express requirement strength. Domain terms use the definitions in [`CONTEXT.md`](../../CONTEXT.md).

## 1. V1 release boundary

V1 MUST ship:

- a Linux and macOS Rust Lume Service and Tauri 2 desktop application;
- the language-neutral Lume Runtime Protocol as versioned Protocol Buffers over bidirectional gRPC;
- a first-party Rust SDK and Rust reference Instrumented Agent Runtime exercising observation, checkpoints, interventions, recovery, and forks;
- a Vue 3 and TypeScript static single-page interface built with Vite;
- an automated protocol and domain conformance suite reusable by future SDKs;
- Local History, Protected Store, retention, deletion, and Diagnostic Export behavior defined here; and
- outbound OTLP Telemetry Projection with an explicit conversion manifest.

Windows endpoint and named-pipe semantics remain part of the portable protocol design, but a Windows service, credential-store integration, and desktop release are not v1 release blockers. OTLP ingestion and additional first-party language SDKs are deferred.

macOS-specific service, credential-store, IPC, packaging, and acceptance development MUST occur on the dedicated `platform/macos` branch. That branch implements this same normative protocol and behavioral contract; it is not a platform-specific fork of Lume semantics. It may merge into the release line only after the complete macOS acceptance matrix passes. Shared protocol or domain changes MUST be resolved in this specification rather than introduced as silent branch-only behavior.

Final visual identity, exact colors, typography, panel proportions, shortcuts, dense-trace navigation refinements, and detailed field-editor styling are deferred to a later design pass. The interaction states, safety boundaries, adaptive structure, and keyboard-operable workflows in this specification are release requirements.

## 2. System boundary

Lume has four collaborating roles:

- The **Lume Service** is one OS-supervised process per operating-system user. It owns the default local endpoint, Runtime Sessions, Local History, the Protected Store, trust decisions, recovery profiles, and all state mutations.
- The **Tauri desktop application** is an attachable client. Its Rust core mediates access between the Vue frontend and the Lume Service. Opening or closing it MUST NOT start or end traces, terminate Runtime Sessions, or become a second history writer.
- An **Instrumented Agent Runtime** owns agent execution, trace declarations, checkpoint state, restoration, and authoritative fork acceptance. It initiates a Runtime Session to the Lume Service.
- An **Agent** is the logical actor whose bounded attempts are Execution Traces. It is not a process, runtime connection, or desktop session.

Lume MUST NOT infer instrumentation by watching arbitrary processes, proxying agent traffic, or scanning a network. Agent orchestration, dispatch, scheduling, cloud hosting, remote Lume access, multi-user operation, vendor-specific adapters, portable checkpoint archives, browser-hosted web clients, mobile clients, and a production terminal client are outside v1.

## 3. Execution Trace model

### 3.1 Identity and envelope

One Execution Trace is one runtime-declared attempt by one Agent to achieve one objective. The runtime MUST explicitly start and end it. Process exit, connection loss, inactivity, timestamps, or a closed desktop window MUST NOT create a trace boundary or outcome.

Each trace MUST contain the semantic equivalent of:

```text
schema_version
trace_id
agent_id
runtime_id
objective
started_at
lifecycle
outcome?
completeness
operations[]
labels?
```

The runtime assigns a globally unique opaque `trace_id` before emission. It MUST remain stable across retransmission, reconnection, and runtime restart. Native IDs SHOULD use OpenTelemetry-compatible binary trace and span representations where the concepts align. Every trace has one stable runtime owner; v1 does not transfer active trace ownership.

`objective` is required human-readable context. Prompts and structured inputs are separate sensitivity-governed payloads. Runtime occurrence time and Lume receipt time are evidence, but neither establishes causality.

### 3.2 Trace Operations

A Trace Operation is the unit of causal activity. Its common envelope MUST provide:

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
outcome?
failure?
cancellation?
payload
```

V1 core types are `agent_step`, `model_call`, and `tool_call`. `agent_step` is optional and MUST NOT be treated as hidden chain-of-thought. Runtime-specific Operation and event types use namespaced extensions.

Every payload field MUST declare `present`, `redacted`, `omitted`, `truncated`, or `unavailable`. Intentional payload absence does not by itself make the trace structurally incomplete.

An Operation has at most one same-trace structural parent and zero or more same-trace causal dependencies. The combined graph MUST be acyclic. Multiple roots and concurrent siblings are valid. Cross-trace causality MUST use Trace Links. Producer sequence and explicit dependencies establish partial order; timestamps MUST NOT invent missing causal order.

### 3.3 Lifecycle, outcome, and evidence

Trace and Operation lifecycle is `active` or `ended`. Only an ended item has one outcome: `succeeded`, `failed`, or `cancelled`. Outcome is runtime-declared and MUST NOT be inferred from child failures. Failed and cancelled outcomes require structured, stable codes and policy-filtered human context.

Evidence is append-only. Re-delivery of one identity with identical content is idempotent. Reuse with different content is an integrity error. A correction is new evidence that explicitly supersedes old evidence; it never overwrites history. Trace end and evidence finalization are separate. Finalization declares each producer's final cursor. Late evidence at or below a final cursor may repair a gap; evidence beyond it is invalid.

Lume MUST quarantine malformed, conflicting, cyclic, out-of-range, or otherwise invalid evidence, retain it for diagnostics, exclude it from canonical views, and preserve the valid remainder.

### 3.4 Trace Completeness

Completeness is independent of lifecycle and outcome:

- `provisional`: active, owner available, and no structural loss known;
- `complete`: ended, all final cursors known, all required evidence present, and no canonical integrity error;
- `incomplete`: structural loss or invalidity is known; or
- `unverified`: evidence is insufficient to prove either completeness or known loss.

Known structural loss takes precedence. A disconnected active trace remains active but becomes unverified unless known loss makes it incomplete. Delayed valid evidence may improve completeness.

## 4. Linked trace graph

Every trace is independently owned. Core cross-trace ancestry uses immutable typed links:

- a Delegation Link anchors the source Operation that caused a separate Agent trace; and
- a Fork Link anchors the source Fork Checkpoint and Intervention audit record that caused a Trace Fork.

A core link uses one globally unique `link_id` and matching immutable source-owner and child-owner assertions. It is `pending` with incomplete evidence, `confirmed` only when both authoritative assertions match, and `conflicted` when assertions disagree or violate ownership, single-origin, or acyclic ancestry.

Every trace MUST be exactly one of a root with no incoming core link, a delegated trace with one incoming Delegation Link, or a Trace Fork with one incoming Fork Link. Confirmed core links form an acyclic single-origin ancestry forest. Pending and conflicted links remain diagnostic evidence but MUST NOT enter canonical ancestry.

Links express causality, not containment or lifecycle propagation. Source and child retain independent agents, objectives, Operations, lifecycles, outcomes, completeness, and runtime availability. No success, failure, cancellation, or disconnection propagates implicitly across a link.

## 5. Fork Checkpoints

A Fork Checkpoint is an immutable declaration that its owning runtime durably saved restorable state at one causally closed Causal Cut. The runtime owns and interprets the state; Lume stores only the declaration and evidence about it.

The declaration MUST provide:

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

The `checkpoint_id` is globally unique and stable across restarts. The Causal Cut includes every same-trace parent and dependency of every included Operation and identifies included producer positions. Concurrent work outside the cut is not shared fork history.

Checkpoint declaration fields are immutable. Any semantic change creates a new checkpoint identity. `restore_reference` is opaque to Lume, grants no authority by possession, is not portable state, and MUST NOT be exported as restoration material.

Checkpoint Availability is separately asserted by the owner as `available`, `unknown`, or `unavailable`, with time and provenance. Disconnect or stale evidence changes presentation to unknown, never unavailable. Only the owner can declare unavailability. Availability is advisory and MUST be revalidated during every fork preparation.

## 6. Intervention contract

Each checkpoint binds one immutable versioned Intervention schema. It is the complete set of state a user may edit. Fields have stable opaque IDs, types, nullability, labels, optional help and values, bounded constraints, sensitivity, optional rendering hints, and structural children.

V1 types are `string`, `integer`, `number`, `boolean`, `enum`, `object`, and `list`. Unsupported custom types MUST NOT degrade to editable strings. Portable constraints cover numeric bounds, string length and pattern, enum membership, list size, and required object children. Schemas contain no executable validation code.

An Intervention is a non-empty sparse map from field ID to explicit value:

- omission retains checkpoint state;
- explicit null clears only a nullable field;
- submitted containers replace that submitted field;
- a nested field may change independently only when it has its own field ID; and
- defaults are suggestions and are applied only through an explicit change.

Lume validates shape, identity, type, portable constraints, and sensitivity handling locally. During atomic preparation the runtime authoritatively revalidates all rules, resulting state, cross-field constraints, compatibility, and effective change. A locally valid form does not guarantee acceptance. Empty or semantically unchanged interventions MUST create no Trace Fork.

Validation issues expose stage, stable code, affected field IDs, a message, and optional typed details. Their content obeys the strongest referenced sensitivity and MUST NOT echo protected or write-only values.

### 6.1 Sensitive Fields

Every field is `ordinary`, `protected`, or `write_only`. Nested fields inherit the strongest enclosing policy.

- Protected values live only in the Protected Store, remain masked by default, and require separate unlock and reveal actions.
- Write-only checkpoint values are never disclosed to Lume. Replacement plaintext may exist only in transient edit/submission memory and MUST NOT be persisted, redisplayed, logged, diagnosed, or exported.
- Omission preserves a sensitive value. Blank presentation MUST NOT clear it.

Audit records retain ordinary values normally, protected values only through the Protected Store, and only a field identity plus changed marker for write-only values.

## 7. Trace Fork lifecycle

### 7.1 Fork Request and confirmation

A Fork Request is distinct from the Trace Fork it may create. Lume assigns `fork_request_id` before contacting or launching a runtime. Its checkpoint, Intervention, execution mode, creation time, acceptance deadline, and confirmation evidence are immutable. Changing semantic content creates a new ID; conflicting reuse is an integrity error.

The mode is `sandboxed` or `live` and MUST be supported by the checkpoint. Sandbox submission is sufficient confirmation. A live request MUST stop at `awaiting_confirmation` and show the source, checkpoint, selected mode, side-effect warning, and policy-filtered change summary. A separate final action is required before recovery or restoration. Viewing or editing MUST NOT contact or launch a runtime for restoration.

### 7.2 Request states

Applicable states are:

```text
awaiting_confirmation
recovering_runtime
preparing
accepted | rejected | cancelled | timed_out
```

Transitions are append-only evidence. The four final states are terminal. Lost communication after possible delivery leaves the request nonterminal and visibly reconciling; client waiting time alone cannot choose an outcome.

Every retry, query, recovery attempt, and cancellation uses the same request ID. The runtime durably guarantees at most one child, stable terminal outcome, and the same child and Fork Link identities after accepted retries. Conflicting content is rejected.

Cancellation before acceptance becomes terminal only after proving no acceptance committed. If acceptance won the race, the request remains accepted and stopping the child is a separate trace action. Timeout requires proof that the deadline passed, no acceptance committed, and later acceptance is impossible under the runtime's idempotency record.

### 7.3 Recovery, preparation, and acceptance

After confirmation, Lume uses an authenticated connected owner or the checkpoint's authorized Runtime Recovery Profile. The process MUST prove the expected runtime identity.

Preparation atomically validates request identity and deadline, owner, checkpoint identity and state integrity, runtime compatibility, Intervention and effective change, execution mode, ancestry, and resources needed to commit.

Fork Acceptance is the single logical boundary at which the runtime durably:

1. assigns the child trace ID;
2. commits an independent restored execution with the Intervention;
3. records execution mode;
4. declares the child trace active; and
5. records matching Fork Link assertions.

No child exists before this boundary. A pre-acceptance failure rejects the request without a child or link. A post-acceptance failure belongs to the child, even if it emits no model or tool call. Lost telemetry affects child completeness and never reverses acceptance.

Checkpoints are reusable and non-consuming. Multiple requests against one checkpoint may prepare and execute concurrently as independent restored instances. Capacity may reject an individual request but MUST NOT consume the checkpoint.

## 8. Local History and protected data

Local History is Lume's durable append-only evidence record, not runtime truth or checkpoint storage. It retains canonical and quarantined observations, corrections, cursors, loss, finalization, link assertions, checkpoint declarations and availability assertions, fork requests and outcomes, policy-filtered audits, identities, provenance, receipt times, schema versions, and deletion markers. Derived views MUST be rebuildable without erasing conflicts or provenance.

Default retention is until explicit deletion. Optional age- or size-based pruning requires prior disclosure of scope, tombstones, protected-data handling, and the fact that runtime-owned state is unaffected.

Lume MUST open stored history without a runtime. On service restart, runtime state and checkpoint availability begin unknown until refreshed. Immutable evidence and confirmed ancestry remain unchanged. Reconciliation may query already reachable owners but MUST NOT automatically confirm live work, launch a runtime, restore a checkpoint, or resubmit a request.

The Protected Store is encrypted and unlocked through OS-backed credentials or an explicit user secret. If locked, unavailable, or irrecoverable, ordinary causal evidence remains usable with honest placeholders. Lume MUST NOT fall back to plaintext. Unlock and reveal are distinct; revealed plaintext does not enter indexes, logs, diagnostics, crash reports, clipboard contents, or persisted frontend state controlled by Lume.

Deletion is local, scoped, confirmed, and non-cascading. Surviving references retain a minimal non-sensitive History Tombstone with identity, relationship anchors, deletion time, and scope. Descendant traces are not implicitly deleted. Local deletion MUST NOT claim deletion of runtime-owned checkpoint state; that requires a separate authenticated runtime operation and confirmed result.

Diagnostic Export is explicit and user-selected. It includes an omission manifest, excludes protected values by default, and may include selected protected values only after unlock and per-export confirmation. It never includes write-only values, restoration references, recovery credentials, executable launch data, store keys, or runtime-owned checkpoint state. It is not a Local History backup/import format or portable fork archive.

## 9. Lume Runtime Protocol

### 9.1 Transport and session

The native protocol is a Lume-owned versioned Protobuf API over one bidirectional gRPC Runtime Session. The runtime initiates the authenticated local connection. One stream carries distinct typed envelopes for observations, acknowledgements, flow control, status, capabilities, control requests, and results.

Default transport is OS-local IPC protected by user permissions: Unix domain sockets on v1 Linux and macOS. An authenticated loopback TCP fallback MAY be configured. Unauthenticated TCP, non-loopback listeners, and remote access are forbidden in v1.

The live RPC schema and Local History schema MUST be distinct contracts joined by explicit translation.

### 9.2 Handshake and evolution

Before canonical traffic, the runtime declares identity, instance, supported versions, supported capabilities, required Lume capabilities, resume positions, and authentication evidence. Lume selects a version and capabilities and returns required runtime capabilities, acknowledged positions, a session ID, and flow-control limits.

No common version or missing required capability rejects the session with a stable reason. Safety features MUST NOT silently downgrade. Protobuf changes follow additive binary evolution: field and enum numbers are never reused, deletions reserve numbers, and newly added fields are not silently required. Incompatible wire or semantic changes require an incompatible protocol version.

### 9.3 Delivery

Observation delivery is at least once. Every durable producer has a stable ID and monotonic sequence. Lume acknowledges only the highest contiguous position durably committed to Local History. Reconnect resumes after the last mutual acknowledgement. Retransmission preserves identity and content; conflicts are integrity errors. Lost ranges are explicit evidence. Flow control bounds in-flight work without changing causal order.

Commands use stable request IDs and idempotent outcomes. This does not promise exactly-once external model or tool side effects.

The Rust SDK MUST let agent execution continue during receiver absence or sustained backpressure. Its default buffer is bounded and in memory. It preserves structural evidence ahead of optional detail, declares structural loss exactly, and never closes a gap by renumbering. Durable runtime-side spooling is an optional declared capability with independently configured security and retention; it is not a Lume guarantee.

### 9.4 Extensions

Unknown extensions carry namespaced type, schema identity and version, encoding, sensitivity metadata, semantic impact, and original bytes. Lume preserves them losslessly where sensitivity permits, labels them unsupported, and never treats them as understood core semantics.

If an unknown extension affects causality, completeness, restoration, validation, sensitivity, or fork behavior, that interpretation becomes incomplete or unavailable. Extensions cannot replace required identity, ordering, loss, sensitivity, checkpoint, Intervention, or Fork Request fields and cannot authorize disclosure or execution.

## 10. Service onboarding and Runtime Trust

`lume setup` or its final CLI equivalent registers or repairs an automatically available OS-supervised per-user service. Official SDKs derive its stable platform endpoint without project configuration. `LUME_ENDPOINT` selects a fully isolated local service for tests or development; isolated services MUST use separate histories, protected stores, identities, and trust registries.

Only one service may own the default endpoint and stores. Multiple desktop clients may attach without becoming writers. Stop and restart preserve data. Uninstall does not delete data; deletion is separate and confirmed.

A runtime self-registers observation through a valid handshake. Its stable runtime ID is bound to persistent identity proof. A reused ID with a different proof is an identity conflict, not credential rotation.

Authenticated first-seen runtimes may contribute visibly untrusted observation evidence. Runtime Trust is explicit per-user authorization bound to the exact identity proof. Before trust, Lume MUST disable fork, recovery, launch, cancellation, and other execution-affecting control. Revocation blocks new control but neither rewrites past provenance nor pretends an accepted fork was cancelled.

A trusted runtime may propose an inert non-secret Runtime Recovery Profile template. The user reviews resolved executable, arguments, working-directory policy, declared environment fields and sensitivity, expected identity, platforms, and actions before saving. Secrets are entered separately into the Protected Store. Material changes create a new revision requiring review. A saved profile launches only during an explicitly confirmed Fork Request; mere viewing, startup, or availability checking MUST NOT launch it.

Lume MUST provide service setup/status/start/stop/restart operations and a read-only `doctor` equivalent. Diagnostics distinguish installation, service, endpoint, access, authentication, version, capability, identity, trust, stores, backlog, loss, and profile failures. Copyable output redacts secrets, sensitive values, restoration references, and trace payloads by default.

## 11. Desktop application contract

The production client uses Tauri 2 with a Vue 3 and TypeScript interface built as a static Vite single-page application. It MUST NOT depend on an SSR server. The Tauri Rust core owns the privileged client boundary, communicates with the Lume Service over authenticated local IPC, and exposes only capability-scoped commands and channels to Vue.

Rust remains authoritative for domain derivation, validation, sensitivity policy, Runtime Trust, persistence, and control authorization. Vue receives purpose-built view models and batched live deltas rather than direct database access or the raw Runtime Session stream. Vue MAY own transient selection, draft form, panel, focus, and navigation state; it MUST NOT independently derive Fork Acceptance, Trace Completeness, trust, checkpoint authority, or disclosure permission.

The frontend MUST NOT receive runtime credentials, recovery-profile secrets, Protected Store keys, or write-only values. A protected value may enter frontend memory only after Rust authorizes a deliberate reveal, only for the selected view, and only until that reveal is dismissed or the view closes. It MUST NOT enter browser persistence, development logs, analytics, clipboard contents without a separate copy action, or crash reports.

The desktop application and service are separate processes. Closing every application window leaves the service, Runtime Sessions, and trace capture running. Reopening attaches to existing Local History. Packaging MAY bundle the service executable with the application, but installation registers it under the independent service lifecycle in section 10.

Investigation is the home context. The baseline desktop workspace presents the linked-trace graph, causal timeline, and inspector together when space permits. A compact window MAY collapse the graph into a linked-trace breadcrumb or move secondary detail behind an explicit panel, but trace identity, safety state, request state, current selection, and primary actions MUST remain available. Branch comparison uses a dedicated side-by-side workspace.

The fork workflow is a visible staged progression:

```text
investigate trace
→ select checkpoint
→ edit typed Intervention
→ choose execution mode
→ confirm live mode when applicable
→ observe request recovery/preparation
→ enter accepted child trace
```

The selected trace and Operation remain contextual while drilling in. Actions are discoverable in the current view. All required workflows are keyboard-operable and pointer-operable, with visible focus, logical focus order, semantic labels, and no color-only safety or completeness cue.

The UI MUST distinguish a Fork Request from a Trace Fork, show no placeholder child before acceptance, preserve request state during uncertainty, and show runtime, checkpoint, trust, availability, mode, sensitivity, completeness, and integrity limitations where they affect an action.

After acceptance, child Operations append live. Linked navigation switches between independent traces rather than nesting their Operations. Comparison shows the shared pre-checkpoint history once, then contrasts original and forked suffixes side by side. Protected and unavailable content remain honest placeholders; write-only values appear only as changed markers.

## 12. OpenTelemetry Telemetry Projection

V1 supports outbound OTLP projection only. The user or configured local policy selects eligible observation evidence. Projection MAY map core Operations to spans, attached point observations to events or logs, and confirmed Trace Links to span links.

Every projection records a manifest identifying source scope, mapping/schema versions, creation time, excluded sensitivity classes, sampling or truncation, unsupported extension types, omitted Lume semantics, and resulting fidelity limitations.

Projection MUST NOT imply that OTLP can restore checkpoints or control forks. It MUST NOT export write-only values, restoration references, runtime credentials, executable recovery data, protected-store keys, or checkpoint state. Protected values are excluded by default and require the same explicit per-export inclusion boundary as Diagnostic Export.

OTLP ingestion is not a v1 feature. Native Lume evidence remains authoritative after projection.

## 13. Acceptance suite

All automated tests below MUST pass on Linux and macOS. The first-party Rust SDK and reference runtime MUST pass the same wire/domain conformance suite exposed to future SDKs. Tests MUST assert durable state after service restart, not only transient UI output.

### A. Installation, service, and trust

- **A01 First trace without desktop client:** setup the user service, run the reference runtime with no application window, then open the desktop application; the completed trace is present.
- **A02 Multiple clients:** two desktop clients attach to one service and observe the same history without a second writer.
- **A03 Isolated endpoint:** `LUME_ENDPOINT` selects separate stores and trust state and cannot write default history.
- **A04 First-seen runtime:** valid evidence is visible and labeled untrusted; all control actions are disabled.
- **A05 Grant and revoke trust:** trust enables negotiated control; revocation blocks new control without rewriting earlier evidence or accepted outcomes.
- **A06 Identity collision:** the same runtime ID with a different proof is conflicted and inherits no trust.
- **A07 Service administration:** stop warns about capture; restart preserves history and trust; uninstall preserves data.
- **A08 Safe diagnostics:** every required failure class is distinguishable and copyable output contains no protected, write-only, restoration, or credential value.

### B. Protocol, delivery, and evolution

- **B01 Negotiation:** compatible versions connect with the common optional capabilities.
- **B02 Required safety capability:** missing write-only or live-confirmation support rejects rather than downgrades.
- **B03 Durable acknowledgement:** a position is acknowledged only after durable Local History commit.
- **B04 Replay:** disconnect after receipt but before acknowledgement causes identical replay and one canonical observation.
- **B05 Identity conflict:** replaying one evidence identity with changed content creates a quarantined integrity error.
- **B06 Gap:** bounded-buffer loss emits exact missing ranges and produces incomplete evidence.
- **B07 Backpressure:** sustained service limits do not block reference agent execution indefinitely.
- **B08 Additive schema:** an older compatible peer preserves unknown fields and handles new enum values without reinterpreting safety.
- **B09 Opaque extension:** an unknown extension survives service restart byte-for-byte where policy permits and displays unsupported.
- **B10 Semantic extension:** an unknown causality-affecting extension makes interpretation incomplete rather than guessed.

### C. Trace causality and completeness

- **C01 Partial order:** two parallel tools remain unordered siblings and a later model call depends explicitly on both.
- **C02 Independent outcomes:** a failed tool followed by recovery may belong to a succeeded trace.
- **C03 Disconnect:** an active disconnected trace stays active with no invented outcome and becomes unverified.
- **C04 Final gap:** a succeeded ended trace with a missing producer position is incomplete.
- **C05 Missing finalization:** a succeeded ended trace without final cursors is unverified.
- **C06 Late repair:** valid late evidence at or below a final cursor repairs a gap; evidence above it is quarantined.
- **C07 Payload absence:** declared redaction or truncation alone does not make structure incomplete.
- **C08 Cycle:** cyclic evidence is quarantined while the valid remainder remains inspectable.

### D. Links and ancestry

- **D01 Delegation:** two parallel delegated traces have separate links, ownership, and outcomes.
- **D02 Pending link:** one authoritative assertion remains pending and does not create canonical ancestry.
- **D03 Confirmation:** matching source and child assertions permanently confirm one link.
- **D04 Conflict:** mismatched assertions are quarantined and do not merge traces.
- **D05 Single origin:** a child cannot acquire two incoming core links or become both delegated and forked.
- **D06 Deep ancestry:** a fork of a delegated trace retains the immediate Fork Link and traversal to prior delegation.
- **D07 No propagation:** child failure or disconnection does not rewrite its source lifecycle or outcome.

### E. Checkpoints and interventions

- **E01 Causal Cut:** a checkpoint whose state incorporates parallel results includes both dependency branches.
- **E02 Immutable restart:** checkpoint identity, cut, state identity, owner, and restore reference survive runtime restart.
- **E03 Availability honesty:** disconnection changes available to unknown; only the owner declares unavailable.
- **E04 Schema identity:** changed saved state or Intervention schema creates a new checkpoint.
- **E05 Sparse values:** omitted, zero, empty, and explicit null remain distinguishable.
- **E06 Local/runtime validation:** a locally valid form may receive a structured cross-field runtime rejection with no child.
- **E07 No-op:** empty and semantically unchanged Interventions create no Trace Fork.
- **E08 Sensitivity:** protected input is masked and separately revealed; write-only input is never redisplayed, logged, stored, diagnosed, or exported.
- **E09 Unsupported type:** an unknown custom field is non-editable and cannot degrade to string input.

### F. Fork lifecycle

- **F01 Sandboxed happy path:** a connected trusted runtime prepares and accepts without live confirmation.
- **F02 Live confirmation:** no recovery contact occurs before a distinct final live-side-effect confirmation.
- **F03 Pre-acceptance rejection:** restoration or validation failure creates no child or Fork Link.
- **F04 Lost acceptance response:** reconciliation with the same request ID returns exactly the original child and link.
- **F05 Cancellation race:** cancellation either proves no child or reports the already accepted child.
- **F06 Proven timeout:** timeout occurs only after the deadline and proof that acceptance cannot occur later.
- **F07 Immediate child failure:** failure after acceptance remains an accepted Fork Request with a failed child trace.
- **F08 Reusable checkpoint:** two concurrent requests produce independent children and do not consume the checkpoint.
- **F09 Recovery identity:** a launched process with the wrong identity receives no fork request.
- **F10 Restart reconciliation:** service restart preserves a nonterminal request and never silently resubmits or launches work.

### G. History, deletion, and export

- **G01 Offline reopen:** ended traces and confirmed ancestry remain inspectable with no runtime.
- **G02 Stale availability:** a previously available checkpoint reopens as unknown until refreshed.
- **G03 Locked Protected Store:** ordinary structure remains usable with placeholders and no plaintext fallback.
- **G04 Tombstoned source:** deleting a source leaves minimal ancestry for a surviving child and does not delete that child.
- **G05 Local versus runtime deletion:** local checkpoint deletion makes no claim about runtime state.
- **G06 Pruning disclosure:** automated pruning requires explicit policy and uses the same tombstone rules.
- **G07 Diagnostic Export:** default export omits protected values and always omits write-only and restoration material.
- **G08 Nonterminal history:** accepted reconciliation returns the same child; uncertain requests remain visibly nonterminal.

### H. Desktop workflow and accessibility

- **H01 Investigation baseline:** trace selection, causal timeline, Operation inspection, and contextual actions work with keyboard and pointer input.
- **H02 Adaptive layout:** a compact window may collapse secondary panels, but identity, breadcrumb, timeline, inspector, safety state, and primary actions remain reachable without content overlap.
- **H03 Staged fork:** the workflow preserves checkpoint, Intervention, mode, and request context through every stage.
- **H04 No phantom child:** recovery and preparation show a request only; the child appears exactly at acceptance.
- **H05 Live update:** accepted child Operations append without losing request, link, mode, or outcome context.
- **H06 Linked navigation:** source and child navigation preserves independent ownership.
- **H07 Comparison:** shared prefix appears once and original/fork suffixes appear side by side.
- **H08 Sensitive presentation:** masked, unavailable, deleted, and write-only states are distinguishable, receive semantic labels, and are never represented as blank ordinary values.
- **H09 Frontend authority:** manipulated Vue state cannot bypass Rust validation, trust, confirmation, or sensitivity checks.
- **H10 Window independence:** closing all Tauri windows does not stop the Lume Service or connected Runtime Sessions.

### I. OTLP Telemetry Projection

- **I01 Core mapping:** trace Operations, attached events, and confirmed links produce valid OTLP equivalents where representable.
- **I02 Manifest:** every projection declares its source, mapping version, exclusions, and non-round-trippable semantics.
- **I03 Safety:** no write-only, restoration, credential, executable-profile, or store-key data appears in OTLP output.
- **I04 Protected default:** protected values are absent unless separately unlocked and selected for that projection.
- **I05 Authority boundary:** exported OTLP data cannot advertise a native restorable checkpoint or issue a Fork Request.

## 14. Release evidence

A v1 release candidate is acceptable only when it provides:

- machine-readable Protobuf descriptors and compatibility checks;
- Linux and macOS results for the complete automated suite;
- conformance results for the Rust SDK and reference runtime;
- service restart and Local History durability results;
- protected/write-only leakage checks across storage, logs, diagnostics, exports, and OTLP;
- packaged Tauri application smoke results for pointer, keyboard, wide, and compact-window workflows;
- frontend-boundary tests proving Vue cannot bypass Rust authorization or persist prohibited sensitive values; and
- a manifest of any deferred visual-design items that do not violate the behavioral contract.

macOS release evidence MUST identify the tested `platform/macos` commit and the integration commit that incorporates it. macOS-specific development performed directly on the main development line does not satisfy the required platform isolation.

Passing a happy-path demonstration alone is insufficient. Known failures in identity, sensitivity, live confirmation, idempotency, ancestry, completeness, or deletion honesty block v1 release.

## 15. Decision sources

Detailed rationale and examples remain in:

1. [Execution Trace semantics](issues/01-define-execution-trace-semantics.md)
2. [Telemetry foundation research](issues/02-assess-telemetry-foundations.md)
3. [Fork Checkpoint contract](issues/03-define-durable-fork-checkpoint-contract.md)
4. [Intervention and Sensitive Field contract](issues/04-define-intervention-contract.md)
5. [Trace Fork lifecycle](issues/05-define-trace-fork-lifecycle.md)
6. [Linked trace graph](issues/06-define-linked-trace-graph.md)
7. [Bubble Tea workflow prototype](issues/07-prototype-bubble-tea-fork-workflow.md)
8. [Local History behavior](issues/08-define-local-history-behavior.md)
9. [Protocol foundation](issues/09-choose-protocol-foundation.md)
10. [Runtime onboarding and operability](issues/10-define-runtime-onboarding-and-operability.md)
11. [Release boundary and acceptance consolidation](issues/11-consolidate-v1-spec-and-acceptance-suite.md)
12. [Rust, Tauri, and Vue desktop pivot](issues/12-pivot-v1-to-rust-tauri-vue.md)
