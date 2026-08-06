# Define runtime onboarding and operability

Parent: [Specify Lume v1 causal debugging and trace forking](../map.md)
Type: `grilling`
Status: `resolved`
Blocked by: 03, 05, 08, 09
Superseded in part by: 12

## Question

How does a developer install, discover, connect, diagnose, recover, and safely operate an Instrumented Agent Runtime with Lume's local Runtime Session model, including receiver lifetime, multiple local instances, unavailable receivers, and Runtime Recovery Profiles?

## Answer

Lume v1 runs one OS-supervised Lume Service per operating-system user. The service owns the default Runtime Session endpoint, is the sole writer of that user's Local History and Protected Store, and remains available when no desktop window is open. Instrumented Agent Runtimes and any number of desktop clients connect to the service; neither owns it.

The normal path from installation to first evidence is:

1. install the Lume executable and register its per-user service;
2. verify that the service and default local endpoint are available;
3. add a Lume SDK to the agent runtime and emit an explicitly instrumented trace;
4. see the first valid runtime identity and trace appear as untrusted observation evidence;
5. explicitly grant Runtime Trust before using execution-affecting control; and
6. optionally review and save a proposed Runtime Recovery Profile when durable checkpoint recovery is needed.

No project registration, process scanning, framework injection, or manually copied endpoint is required for the normal first-trace path.

### Lume Service lifecycle

Installation registers an OS-supervised user service. It becomes available automatically for the signed-in user, either eagerly after login or through platform-supported activation, and the supervisor restarts it after an unexpected process failure. The guarantee is service availability, not a particular platform mechanism.

Opening or closing a desktop window only attaches or detaches a client. It does not start a separate receiver, close runtime connections, or change trace lifecycle. Multiple desktop clients may read the same Local History concurrently. All writes, trust changes, profile changes, deletion, exports, and fork actions pass through the one service so they share one authoritative state and confirmation boundary.

Only one service may own the default per-user endpoint and history stores. A second process attempting to become the default service attaches as a client where appropriate or exits with a diagnostic identifying the current owner; it never steals the endpoint or opens the same stores as another writer.

Explicitly isolated services are allowed for development and tests. Each uses a distinct endpoint, Local History, Protected Store, service identity, and trust registry. Pointing two services at the same writable stores is invalid.

Service stop, restart, update, and uninstall are explicit administrative actions. Stopping warns that live runtimes will disconnect and future evidence may be buffered or lost. Restarting or updating preserves Local History, trust, and recovery profiles. Uninstalling the service does not delete Local History, the Protected Store, trust decisions, or profiles; destructive data removal is a separate scoped and confirmed action under the Local History deletion rules.

### Endpoint discovery

Official SDKs derive a stable platform-standard endpoint for the current user. The endpoint contains no project identity or secret and is not written into repository configuration. Runtimes do not scan ports, processes, or the network.

`LUME_ENDPOINT` is the explicit override for an isolated development or test service. An override selects a receiver; it does not weaken authentication, grant Runtime Trust, enable remote access, or authorize that service to reuse the default stores. Invalid, unreachable, non-local, or unauthenticated endpoints fail with distinct diagnostics.

The default OS-local endpoint uses the access boundary from ticket 09. A loopback TCP fallback additionally uses a generated, scoped credential delivered outside trace data. Endpoint and credential details never appear in ordinary trace payloads, diagnostics copied by default, or project files.

### First connection and runtime identity

Observation onboarding is SDK-first and self-registering. A runtime supplies its stable `runtime_id`, persistent identity proof, instance identity, versions, and capabilities during the Runtime Session handshake. The Lume Service records the identity and exposes valid incoming evidence without requiring a prior configuration entry.

Runtime identity proof is stable across expected process restarts and is bound to the runtime's local credential or key fingerprint. Reuse of a `runtime_id` with a different proof is an identity conflict, not an automatic credential rotation. Lume preserves both sources' evidence with conflict provenance, withholds control authority, and requires an explicit user resolution. A runtime cannot gain trust by merely claiming a previously trusted `runtime_id`.

Malformed or unauthenticated sessions are rejected before their observations become canonical. An authenticated but untrusted runtime may contribute evidence because first-trace discovery is part of onboarding. Its evidence is visibly attributed as untrusted, subject to bounded service ingestion limits, and never merged with a trusted identity on name alone.

### Runtime Trust

Runtime Trust is a persistent per-user authorization bound to the exact runtime identity proof. Before trust, Lume disables:

- delivery of Fork Requests and cancellation commands;
- checkpoint recovery or availability commands that can cause execution;
- invocation or creation of an executable Runtime Recovery Profile; and
- any other negotiated capability that can change runtime or external state.

The user grants trust from a view that shows the runtime ID, identity fingerprint, first and most recent connection, executable or SDK provenance when available, requested control capabilities, and any identity conflicts. Trusting a runtime does not trust all runtimes from the same project, executable path, SDK, or OS user.

Revoking trust prevents new control actions immediately and disconnects or disables the control side of an existing session. Revocation does not falsify prior evidence or claim that an already accepted fork was cancelled. Nonterminal requests remain visible and require reconciliation after the identity is trusted again or by another authoritative path.

Observation evidence received before trust remains labeled with its original trust-at-receipt provenance even if the identity is trusted later. Trust is execution authority, not retrospective validation of semantic truth.

### Unavailable service and buffering

Lume instrumentation does not become an availability dependency of the agent application. If the service is unavailable or applies sustained backpressure, the official SDK lets agent execution continue and uses a bounded in-memory buffer by default.

Within that bound, the SDK preserves mandatory structural and lifecycle evidence ahead of optional payload detail. It represents shed payload detail using the established redacted, omitted, truncated, or unavailable states where those states remain honest. If structural observations are lost, the producer records their sequence range and emits explicit declared-loss evidence after reconnecting. It never closes a gap by renumbering later observations.

The default SDK does not spool trace payloads to disk because that would create a second sensitive-data store outside Lume's Protected Store. A runtime may declare an optional durable-buffer capability, but its storage, encryption, retention, and sensitivity guarantees must be explicitly configured and are owned by that runtime rather than implied by Lume.

If a runtime terminates before it can reconnect, Lume can report only the evidence it actually received: a partially observed active trace becomes unverified, while a trace that never reached Lume does not appear as if it had been observed. The service does not infer missing traces by scanning processes or project files.

### Runtime Recovery Profiles

A trusted runtime may propose a non-secret Runtime Recovery Profile template. A proposal is inert observation data. It identifies at least:

```text
profile_template_id
runtime_id
runtime_identity_fingerprint
reconnect_method?
executable
arguments_and_placeholders[]
working_directory_policy
environment_field_declarations[]
supported_platforms[]
supported_recovery_capabilities[]
```

The proposal contains field declarations and references, never secret values, checkpoint restoration references, or authority to execute itself. Lume shows the resolved executable, arguments, working-directory rule, requested environment names and sensitivity classes, expected runtime identity, and supported actions before the user saves it.

Saving creates a local Runtime Recovery Profile with a stable identity and revision. Secret or protected values are entered separately and referenced from the Protected Store; write-only values are never echoed after entry. Executable launch data and credentials remain outside trace evidence and Diagnostic Export.

A material change to the executable, argument structure, working-directory policy, environment declarations, expected identity, or supported recovery actions creates a new revision requiring review. A trusted runtime may propose an update but cannot silently mutate an authorized profile.

Profile existence grants no background launch authority. Lume may passively use a configured reconnect method to find an already running owner, but it invokes an executable launch only while carrying out an explicit, confirmed Fork Request as defined by tickets 03 and 05. The recovered process must prove the profile's expected runtime identity before receiving the request.

### Operability and diagnostics

Lume provides behavioral equivalents of these local commands:

- `lume setup`: register or repair the user service and verify the default endpoint;
- `lume service status|start|stop|restart`: manage the supervised service without changing stored evidence;
- `lume doctor`: perform read-only checks and print actionable diagnostics; and
- `lume`: open or focus the Tauri desktop client attached to the current endpoint.

The exact command spelling may be refined with the final CLI design, but these operations and separations are required.

Diagnostics distinguish at least: service not installed, service stopped, endpoint unavailable, access denied, authentication failed, incompatible protocol, missing required capability, identity conflict, untrusted control, Local History unavailable, Protected Store locked, producer backlog, declared evidence loss, and invalid recovery profile.

`doctor` reports the selected endpoint source, service and store health, client/service versions, connected runtime identities and trust states, negotiated capabilities, acknowledgement lag, and profile validation. It redacts credentials, protected values, write-only values, restoration references, and full trace payloads. A copyable diagnostic summary is safe by default and states what was omitted.

### Acceptance examples

- A runtime started before any desktop client connects to the per-user service, records a trace, and that trace appears when the desktop application opens later.
- Two desktop clients attach to the same service and see one Local History; neither opens a competing writer.
- A test using `LUME_ENDPOINT` reaches an isolated service and cannot accidentally write to the default history stores.
- A newly instrumented runtime's first trace appears immediately, but its Fork action is disabled until the user trusts its exact identity.
- A process claiming a trusted `runtime_id` with a new key is shown as an identity conflict and receives no inherited control authority.
- During a service restart, an agent continues running, reconnects, and retransmits unacknowledged observations from memory without duplicate canonical Operations.
- Buffer overflow produces explicit missing ranges and an incomplete trace rather than silently renumbered evidence.
- A runtime-proposed recovery command cannot execute until the user reviews and saves the profile and later confirms a Fork Request.
- Changing a saved profile's executable requires a newly reviewed revision; a runtime cannot update it in place.
- Uninstalling the service leaves history intact until the user separately selects and confirms data deletion.
