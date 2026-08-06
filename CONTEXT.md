# Lume

Lume's language for describing instrumented AI-agent activity and the evidence used to understand it.

## Language

**Agent**:
A logical actor with a stable runtime-supplied identity that may make multiple attempts to achieve objectives. An Agent is distinct from both an individual Execution Trace and the Instrumented Agent Runtime that executes it.
_Avoid_: Runtime, model, trace

**Execution Trace**:
A runtime-declared, bounded attempt by one agent to achieve an objective, containing its causally related model calls, tool calls, and other operations. Work delegated to another agent belongs to a separate, linked Execution Trace.
_Avoid_: Run, session, event stream

**Trace Completeness**:
The evidence-backed judgment that an Execution Trace is provisional, complete, incomplete, or unverified. It is independent of the trace's lifecycle and outcome.
_Avoid_: Status, success

**Local History**:
Lume's durable record of the evidence it received, validated, quarantined, and presented. It remains investigable without a connected runtime but is not authoritative for live runtime state or checkpoint restoration.
_Avoid_: Runtime cache, checkpoint store

**Protected Store**:
Encrypted local storage for protected values, unlocked separately from ordinary Local History. When locked or unavailable, Lume retains placeholders rather than falling back to plaintext.
_Avoid_: Main history store, write-only storage

**History Tombstone**:
The minimal non-sensitive identity and relationship record retained after local evidence is deleted so surviving causal ancestry remains honest.
_Avoid_: Deleted trace, retained payload

**Diagnostic Export**:
A user-selected, policy-filtered copy of Local History for investigation outside Lume. It contains neither write-only values nor checkpoint restoration authority.
_Avoid_: Backup, portable fork archive

**Telemetry Projection**:
A policy-filtered, potentially lossy representation of Lume observation evidence in an external telemetry model. It carries an omission manifest and never becomes authoritative Lume evidence or runtime-control authority.
_Avoid_: Native trace, backup, round-trip export

**Trace Operation**:
A causally positioned unit of activity within an Execution Trace, with its own identity, type, structural parent, causal dependencies, lifecycle, and optional result. Point-in-time observations belong to a Trace Operation rather than standing alone as the trace model.
_Avoid_: Flat event, log entry

**Causal Cut**:
A runtime-declared, causally closed boundary in an Execution Trace that identifies exactly which Operations and producer positions are included in the state at that point.
_Avoid_: Timestamp, single preceding event

**Agent Step**:
An optional, runtime-declared Trace Operation representing one coherent unit of agent control or decision-making. It may group the model calls, tool calls, and other Operations caused by that unit.
_Avoid_: Turn, mandatory model-call cycle, arbitrary group

**Trace Link**:
A typed causal relationship between two Execution Traces, established by matching immutable assertions from their owning runtimes.
_Avoid_: Nested run

**Delegation Link**:
A Trace Link from the source Operation that initiated another Agent's Execution Trace to the start of that separate trace.
_Avoid_: Child operation, nested trace

**Fork Link**:
A Trace Link from a Fork Checkpoint and Intervention to the start of the resulting Trace Fork.
_Avoid_: Retry link, copied trace

**Trace Fork**:
An accepted new Execution Trace that resumes from a Fork Checkpoint with an Intervention while retaining its ancestry. It exists only after the owning runtime commits restoration and declares the child trace.
_Avoid_: Replay, rerun, retry

**Fork Request**:
A user's not-yet-accepted request to create a Trace Fork from a Fork Checkpoint with an Intervention and Fork Execution Mode.
_Avoid_: Pending trace, Trace Fork

**Fork Acceptance**:
The atomic boundary at which the owning runtime commits restored execution, declares the child Execution Trace, and establishes its Fork Link. Failures before it reject the Fork Request; failures after it belong to the Trace Fork.
_Avoid_: Submission, first operation

**Instrumented Agent Runtime**:
The cooperating agent system with a stable identity that owns and emits an Execution Trace throughout its lifetime, and performs a requested Trace Fork by restoring its own execution state and continuing from it.
_Avoid_: Lume runtime, watcher

**Fork Checkpoint**:
A durable, runtime-restorable state at a Causal Cut in an Execution Trace, together with the typed inputs the user may edit before continuing. It remains addressable across runtime restarts.
_Avoid_: Snapshot, arbitrary event

**Checkpoint Availability**:
The owning runtime's fresh evidence that a Fork Checkpoint is available, unavailable, or of unknown availability. Availability is advisory and must be revalidated when restoration begins.
_Avoid_: Existence, guarantee, cached status

**Runtime Recovery Profile**:
A user-reviewed local definition of how Lume may reconnect to or launch a specific trusted Instrumented Agent Runtime for checkpoint restoration. Executable configuration and protected-value references remain outside trace evidence.
_Avoid_: Checkpoint command, embedded launcher

**Intervention**:
A validated, non-empty sparse set of checkpoint-defined input changes applied when creating a Trace Fork. An omitted field retains checkpoint state; omission is distinct from an explicit value or null.
_Avoid_: State patch, replay edit

**Intervention Field**:
A checkpoint-declared editable input with a stable identity, structural type, constraints, and disclosure policy. It is the only state an Intervention may change.
_Avoid_: Arbitrary state path, form label

**Fork Execution Mode**:
A checkpoint-supported, user-selected declaration that a Trace Fork will continue in either a sandboxed environment or a live environment with real side effects.
_Avoid_: Safety level

**Sensitive Field**:
A field the Instrumented Agent Runtime marks as protected or write-only. Protected values require protected storage and hidden-by-default presentation; write-only values are never disclosed or retained by Lume.
_Avoid_: Detected secret

**Lume Runtime Protocol**:
Lume's versioned Protocol Buffers contract for loss-aware causal observation and authoritative runtime control over a bidirectional gRPC connection. OpenTelemetry compatibility is provided through adapters rather than defining this contract.
_Avoid_: OTLP extension, OpAMP profile

**Runtime Session**:
An authenticated bidirectional connection initiated by an Instrumented Agent Runtime to the local Lume receiver, carrying negotiated observation, acknowledgement, capability, and control envelopes.
_Avoid_: Execution Trace, UI session

**Protocol Capability**:
A named, negotiated feature of the Lume Runtime Protocol. Required capabilities cause an incompatible session to be rejected; safety-relevant behavior is never silently downgraded.
_Avoid_: Version number, feature guess

**Lume Service**:
The persistent per-user local receiver that owns Runtime Sessions and is the sole writer of Local History. Desktop clients attach to it and may come and go without ending trace capture.
_Avoid_: Desktop process, Instrumented Agent Runtime, system-wide daemon

**Runtime Trust**:
The user's local authorization for an identified Instrumented Agent Runtime to participate in execution-affecting control. An untrusted runtime may contribute visibly attributed observation evidence but cannot receive checkpoint recovery, launch, or fork authority.
_Avoid_: Connection, runtime identity, operating-system user
