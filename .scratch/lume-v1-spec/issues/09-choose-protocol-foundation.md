# Choose the instrumentation and control protocol foundation

Parent: [Specify Lume v1 causal debugging and trace forking](../map.md)
Type: `grilling`
Status: `resolved`
Blocked by: 01, 02, 03, 04, 05, 06

## Question

Given Lume's resolved trace, checkpoint, intervention, fork, and linked-trace semantics and the research into existing standards, which protocol foundation should v1 adopt, and where should it conform to, extend, or deliberately diverge from that foundation?

## Answer

Lume v1 uses a Lume-owned, versioned Protocol Buffers contract over one bidirectional gRPC Runtime Session. The Instrumented Agent Runtime initiates the local connection. OpenTelemetry compatibility sits at an adapter boundary: Lume adopts compatible identifiers and telemetry concepts where they preserve meaning, but its native protocol remains authoritative for causal evidence, completeness, checkpoints, interventions, sensitivity, and fork control.

OTLP is not the native live protocol, and OpAMP is not the control foundation. OTLP/gRPC uses unary export requests rather than a bidirectional stream, while OpAMP is a beta protocol for managing telemetry-agent fleets. Neither supplies Lume's restoration and fork semantics. A gRPC bidirectional streaming RPC directly provides two independently ordered message streams, and Protobuf supplies a mature additive-evolution model. See the official [OTLP specification](https://opentelemetry.io/docs/specs/otlp/), [OpAMP specification](https://opentelemetry.io/docs/specs/opamp/), [gRPC streaming model](https://grpc.io/docs/what-is-grpc/core-concepts/), and [Protobuf evolution practices](https://protobuf.dev/best-practices/dos-donts/).

### Native session

A Runtime Session is one authenticated logical connection between a runtime and the local Lume receiver. One bidirectional stream multiplexes semantically distinct envelopes for:

- trace, Operation, Trace Link, checkpoint, availability, loss, correction, and finalization observations;
- acknowledgements and flow-control limits;
- runtime identity, health, and capability changes;
- Fork Requests, cancellation and reconciliation commands; and
- validation, recovery, preparation, acceptance, rejection, and other command results.

Multiplexing does not merge these contracts. Every envelope declares its kind, stable identity, producer, sequence position where applicable, and protocol version context. Observation evidence remains append-only. Commands and results retain the idempotency and authority rules of the domain contract.

The protocol schema used by the live RPC is not the Local History storage schema. Lume translates accepted, conflicting, and quarantined wire evidence into its durable evidence model so either representation may evolve without silently changing the other.

### Connection and access boundary

The runtime connects to a discoverable Lume endpoint and begins with a handshake; Lume does not probe arbitrary processes or require each short-lived runtime to host a server.

The default endpoint is OS-local IPC protected by operating-system access controls: a Unix domain socket on Unix-like systems and a named pipe on Windows. An authenticated loopback TCP endpoint may be configured as a compatibility fallback. Unauthenticated TCP, non-loopback listeners, and remote access are outside the v1 contract.

Endpoint discovery, installation, profile setup, multiple local Lume-instance behavior, and the exact receiver process lifetime belong to the onboarding and operability specification. Regardless of process layout, a runtime sees one logical local Lume receiver and must not treat connection failure as permission to expose sensitive values elsewhere.

### Handshake and capability negotiation

No observation or control message is canonical until both sides complete a handshake. The runtime declares at least:

```text
runtime_id
runtime_instance_id
supported_protocol_versions
supported_capabilities[]
required_lume_capabilities[]
producer_resume_positions[]
authentication_evidence
```

Lume responds with at least:

```text
selected_protocol_version
enabled_capabilities[]
required_runtime_capabilities[]
acknowledged_producer_positions[]
session_id
flow_control_limits
```

The protocol has an explicit incompatible version boundary plus named capabilities for independently deployable features. Additive Protobuf changes may cross a compatible version boundary; incompatible wire or semantic changes require a new incompatible version. Field numbers and enum values are never reused, removed numbers are reserved, and new fields cannot become silently required merely because they exist in the schema.

Either side rejects the session with a stable reason when no common protocol version exists or a required capability is unavailable. Optional capabilities may remain disabled. Checkpoint restoration, protected or write-only field handling, live-execution confirmation, idempotent fork reconciliation, and other safety-relevant behavior never silently downgrade to a weaker interpretation.

### Delivery, replay, and acknowledgement

The Runtime Session provides at-least-once delivery, not an exactly-once claim:

- each durable producer stream has a stable `producer_id` and monotonically increasing sequence;
- an acknowledgement advances the highest contiguous position durably committed to Local History, not merely received in memory;
- a reconnect resumes after the last mutually acknowledged position;
- retransmission preserves observation identity and content so Lume can deduplicate it;
- conflicting reuse of an identity remains an integrity error;
- sequence loss is declared explicitly and feeds Trace Completeness rather than being hidden; and
- flow-control limits bound in-flight delivery without redefining causal order.

Commands use stable request IDs. Retrying a Fork Request, cancellation, or reconciliation command returns or advances the same durable outcome rather than repeating the action. This transport behavior supports the Fork Request rules; it does not imply exactly-once execution of arbitrary model or tool side effects.

The protocol does not require an agent process to block indefinitely when Lume is unavailable. A runtime may retain evidence for later delivery according to its declared capability and limits. Evidence it can no longer supply must become explicit loss or unavailable evidence when the session resumes.

### OpenTelemetry boundary

OpenTelemetry interoperability is optional and adapter-based:

- native trace and Operation identifiers use OpenTelemetry-compatible trace and span identifier representations where the concepts align;
- core Operations may export as spans, point observations as span events or logs, and confirmed cross-trace causality as typed span links;
- relevant OTLP data may be ingested as observation evidence when an adapter can establish the required Lume identity, lifecycle, causality, provenance, and availability semantics;
- sampling, truncation, missing lifecycle evidence, or lossy conversion must be reflected in Trace Completeness; and
- adapter output records that it is a projection when Lume semantics cannot round-trip.

OTLP input is never sufficient by itself to advertise a restorable Fork Checkpoint, declare an editable Intervention Field, reveal a Sensitive Field, or accept a Fork Request. OTLP exporters cannot receive Lume control commands. OpAMP may coexist in a deployment for telemetry-agent management but has no authority in the Lume domain.

### Extensions and unknown evidence

Runtime-specific additions use an explicit extension envelope rather than unstructured attributes being treated as core semantics. An extension declares:

```text
namespaced_type
schema_identifier
schema_version
payload_encoding
sensitivity_metadata
semantic_impact
payload_bytes
```

Lume preserves an unknown extension's original bytes and metadata losslessly, labels it unsupported, and never claims to validate or interpret its payload. If the runtime declares that an unknown extension affects causal structure, completeness, checkpoint restoration, intervention validation, sensitivity, or fork behavior, the affected interpretation is incomplete or unavailable rather than guessed.

Extensions may enrich observations but cannot replace required core handshake, identity, ordering, loss, sensitivity, checkpoint, Intervention, or Fork Request fields. An unknown extension cannot authorize disclosure or execution. Lume may omit protected extension payloads from ordinary storage under the same policy as core Sensitive Fields, but must retain an honest structural placeholder.

### Conformance boundary

Lume v1 therefore:

- **conforms** to binary Protobuf compatibility practices, gRPC bidirectional streaming semantics, and OpenTelemetry-compatible identifiers and observational mappings where meaning aligns;
- **extends** conventional telemetry with explicit live lifecycle, causal ordering, completeness, typed trace links, checkpoint availability, and per-field sensitivity evidence; and
- **diverges** from OTLP and OpAMP by defining a native loss-aware observation stream and authoritative bidirectional checkpoint, Intervention, and fork-control contract.

### Acceptance examples

- A runtime and Lume with compatible protocol versions but different optional capabilities connect with only their common optional capabilities enabled.
- A runtime requiring write-only Intervention support is rejected by a Lume receiver that cannot guarantee it; the session does not downgrade the field to protected or ordinary data.
- Lume acknowledges an observation only after its evidence is durably represented in Local History. Reconnection then retransmits only positions after that acknowledgement.
- A lost Fork Acceptance response is reconciled with the same request ID and returns the original child trace rather than executing a second fork.
- An OTLP adapter exports a Fork Link as cross-trace telemetry while clearly omitting the non-representable checkpoint restoration contract.
- An unknown runtime extension remains available as opaque evidence after restart, while the UI states that its meaning is unsupported.
- A runtime that loses buffered observations reports the missing producer range, causing honest incomplete evidence rather than a seamless-looking trace.
- A local process without OS endpoint permission or the configured loopback credential cannot open a Runtime Session or submit control messages.
