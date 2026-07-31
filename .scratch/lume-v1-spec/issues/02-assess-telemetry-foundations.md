# Assess existing telemetry foundations for Lume

Parent: [Specify Lume v1 causal debugging and trace forking](../map.md)
Type: `research`
Status: `resolved`

## Question

How far can OpenTelemetry or another established telemetry standard carry Lume's causal agent data, linked Execution Traces, durable Fork Checkpoints, typed Interventions, control requests, sensitivity metadata, and live streaming, and which requirements would need extensions or a Lume-specific protocol?

## Answer

OpenTelemetry is a strong, stable substrate for recording and correlating observations, but it does not supply Lume's checkpoint, intervention, or fork-control contract.

- **Causal activity:** stable trace and log primitives provide trace identity, parent/child spans, attributes, timestamped events, status, and structured logs. The directly relevant GenAI conventions remain in development, so Lume must own its required agent vocabulary and completeness rules. See the [OpenTelemetry overview](https://opentelemetry.io/docs/specs/otel/overview/) and [GenAI semantic conventions](https://github.com/open-telemetry/semantic-conventions-genai/tree/main/docs/gen-ai).
- **Delegation and fork ancestry:** stable Span Links can express causality across independent traces, but Lume must define relationship types such as delegation and forking plus their invariants. See [links between spans](https://opentelemetry.io/docs/specs/otel/overview/#links-between-spans).
- **Durable checkpoints:** OpenTelemetry can carry a checkpoint identifier or reference as custom data, but defines no durability, restoration, compatibility, runtime-ownership, or reconnection lifecycle.
- **Typed Interventions:** OTLP's `AnyValue` can encode primitives, arrays, maps, and bytes, but supplies no editable-field schema, constraints, defaults, sensitivity, validation, or application semantics. See [OpenTelemetry common concepts](https://opentelemetry.io/docs/specs/otel/common/).
- **Fork control:** OTLP is a client-to-server export protocol, not a bidirectional control protocol. OpAMP is beta, targets telemetry-agent fleet management, and does not define fork semantics. See [OTLP](https://opentelemetry.io/docs/specs/otlp/) and [OpAMP](https://opentelemetry.io/docs/specs/opamp/).
- **Sensitive Fields:** OpenTelemetry supports redaction and transformation mechanisms but has no standard per-field sensitivity marker or reveal/storage policy; it explicitly leaves identification of sensitive data to implementers. See [handling sensitive data](https://opentelemetry.io/docs/security/handling-sensitive-data/).
- **Live observation:** repeated OTLP exports and correlated structured logs can deliver incremental data, but standard span export centers on completed spans and OTLP/gRPC uses unary requests rather than a bidirectional event stream. Lume needs explicit lifecycle events or its own live channel. See the [OTLP protocol](https://opentelemetry.io/docs/specs/otlp/) and [tracing SDK processors](https://opentelemetry.io/docs/specs/otel/trace/sdk/).
- **Fidelity:** standard SDK sampling and signal limits can make telemetry lossy. A Lume contract built on OpenTelemetry must require suitable no-sampling and limit behavior, explicit loss reporting, or a separate lossless channel.

The later protocol decision should treat OpenTelemetry as a compatibility-friendly telemetry data-model foundation and possible ingestion format, while keeping Lume's observation schema versioned separately and defining a Lume-owned runtime-control protocol.
