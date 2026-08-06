use std::collections::{BTreeMap, BTreeSet, HashMap};

use lume_domain::{
    AgentId, CheckpointAvailability, ExecutionTrace, FieldId, FieldType, ForkCheckpoint,
    ForkExecutionMode, InterventionField, InterventionSchema, InterventionValue, Lifecycle,
    OperationId, OperationType, Outcome, PayloadAvailability, ProducerId, RuntimeId, Sensitivity,
    TraceId, TraceOperation,
};
use lume_protocol::{
    CAPABILITY_LIVE_CONFIRMATION, CAPABILITY_WRITE_ONLY, PROTOCOL_VERSION,
    v1::{Handshake, OpaqueExtension, PayloadSensitivity, SemanticImpact},
};

pub const RUNTIME_ID: &str = "conformance-runtime";
pub const TRACE_ID: &str = "conformance-trace";
pub const PRODUCER_ID: &str = "conformance-producer";

/// A partial-order trace with two unordered tool operations and one explicit join.
///
/// # Panics
///
/// Panics only if the statically defined fixture violates the domain contract.
pub fn parallel_trace() -> ExecutionTrace {
    let mut trace = ExecutionTrace::new(
        TraceId::from(TRACE_ID),
        AgentId::from("conformance-agent"),
        RuntimeId::from(RUNTIME_ID),
        "Validate causal trace semantics",
    )
    .expect("fixture has a non-empty objective");
    trace
        .record_operation(operation("left", 1, &[]))
        .expect("left operation is valid");
    trace
        .record_operation(operation("right", 2, &[]))
        .expect("right operation is valid");
    trace
        .record_operation(operation("join", 3, &["left", "right"]))
        .expect("join operation is valid");
    trace
}

/// A causally closed checkpoint containing the complete parallel fixture.
pub fn parallel_checkpoint() -> ForkCheckpoint {
    ForkCheckpoint {
        checkpoint_id: "conformance-checkpoint".into(),
        source_trace_id: TRACE_ID.into(),
        included_producer_positions: BTreeMap::from([(PRODUCER_ID.into(), 3)]),
        runtime_id: RUNTIME_ID.into(),
        state_identity: "state-v1".into(),
        restore_reference: "opaque-runtime-owned-reference".into(),
        intervention_schema: sensitivity_schema(),
        execution_modes: BTreeSet::from([ForkExecutionMode::Sandboxed, ForkExecutionMode::Live]),
        availability: CheckpointAvailability::Available,
    }
}

/// A schema exercising ordinary, protected, write-only, nullable, and unsupported fields.
pub fn sensitivity_schema() -> InterventionSchema {
    let fields = [
        field(
            "count",
            FieldType::Integer {
                minimum: Some(0),
                maximum: Some(10),
            },
            false,
            Sensitivity::Ordinary,
        ),
        field(
            "context",
            FieldType::String {
                min_length: Some(0),
                max_length: Some(256),
            },
            true,
            Sensitivity::Protected,
        ),
        field(
            "credential",
            FieldType::String {
                min_length: Some(1),
                max_length: Some(256),
            },
            false,
            Sensitivity::WriteOnly,
        ),
        field(
            "vendor",
            FieldType::Unsupported {
                type_name: "vendor.custom".into(),
            },
            false,
            Sensitivity::Ordinary,
        ),
    ];
    InterventionSchema {
        schema_id: "conformance-intervention-v1".into(),
        fields: fields
            .into_iter()
            .map(|field| (field.field_id.clone(), field))
            .collect(),
    }
}

/// A compatible handshake that requires both v1 safety capabilities.
pub fn compatible_handshake() -> Handshake {
    Handshake {
        runtime_id: RUNTIME_ID.into(),
        instance_id: "conformance-instance".into(),
        supported_versions: vec![PROTOCOL_VERSION],
        supported_capabilities: vec![
            CAPABILITY_WRITE_ONLY.into(),
            CAPABILITY_LIVE_CONFIRMATION.into(),
        ],
        required_lume_capabilities: vec![
            CAPABILITY_WRITE_ONLY.into(),
            CAPABILITY_LIVE_CONFIRMATION.into(),
        ],
        resume_positions: HashMap::new(),
        authentication_evidence: b"conformance-only".to_vec(),
    }
}

/// An opaque extension whose bytes include values that commonly expose lossy conversions.
pub fn opaque_extension() -> OpaqueExtension {
    OpaqueExtension {
        namespace: "dev.lume.conformance".into(),
        type_name: "opaque".into(),
        schema_identity: "dev.lume.conformance/opaque".into(),
        schema_version: 1,
        encoding: "application/octet-stream".into(),
        sensitivity: PayloadSensitivity::Protected.into(),
        semantic_impact: SemanticImpact::PresentationOnly.into(),
        original_bytes: vec![0, 1, 127, 128, 254, 255],
    }
}

/// Sparse values that prove omission, zero, empty, and explicit null remain distinct.
pub fn sparse_intervention() -> BTreeMap<FieldId, InterventionValue> {
    BTreeMap::from([
        ("count".into(), InterventionValue::Integer(0)),
        ("context".into(), InterventionValue::Null),
        (
            "credential".into(),
            InterventionValue::String("write-only-fixture".into()),
        ),
    ])
}

fn field(
    id: &str,
    field_type: FieldType,
    nullable: bool,
    sensitivity: Sensitivity,
) -> InterventionField {
    InterventionField {
        field_id: id.into(),
        label: id.into(),
        field_type,
        nullable,
        sensitivity,
    }
}

fn operation(id: &str, sequence: u64, dependencies: &[&str]) -> TraceOperation {
    TraceOperation {
        operation_id: OperationId::from(id),
        operation_type: OperationType::ToolCall,
        structural_parent_id: None,
        causal_dependency_ids: dependencies.iter().map(|id| (*id).into()).collect(),
        producer_id: ProducerId::from(PRODUCER_ID),
        producer_sequence: sequence,
        lifecycle: Lifecycle::Ended,
        outcome: Some(Outcome::Succeeded),
        payload_availability: PayloadAvailability::Present,
    }
}

#[cfg(test)]
mod tests {
    use lume_domain::{InterventionError, TraceCompleteness};
    use lume_protocol::{encoded_observation, negotiate, v1};
    use prost::Message;

    use super::*;

    #[test]
    fn causal_checkpoint_and_sparse_intervention_are_valid() {
        let mut trace = parallel_trace();
        assert_eq!(trace.operations().count(), 3);
        let checkpoint = parallel_checkpoint();
        checkpoint.validate_causal_cut(&trace).unwrap();
        checkpoint
            .intervention_schema
            .validate(&sparse_intervention())
            .unwrap();

        trace.end(Outcome::Succeeded);
        trace.finalize_producer(PRODUCER_ID.into(), 3);
        assert_eq!(trace.completeness(), TraceCompleteness::Complete);
    }

    #[test]
    fn unsupported_fixture_field_is_not_editable() {
        let result = sensitivity_schema().validate(&BTreeMap::from([(
            FieldId::from("vendor"),
            InterventionValue::String("must-not-degrade".into()),
        )]));
        assert_eq!(
            result,
            Err(InterventionError::UnsupportedField("vendor".into()))
        );
    }

    #[test]
    fn wire_fixtures_negotiate_and_encode_deterministically() {
        let (version, capabilities) = negotiate(&compatible_handshake()).unwrap();
        assert_eq!(version, PROTOCOL_VERSION);
        assert_eq!(capabilities.len(), 2);

        let observation = v1::Observation {
            producer_id: PRODUCER_ID.into(),
            producer_sequence: 1,
            evidence_id: "opaque-evidence".into(),
            evidence: Some(v1::observation::Evidence::OpaqueExtension(
                opaque_extension(),
            )),
        };
        let encoded = encoded_observation(&observation);
        assert_eq!(
            v1::Observation::decode(encoded.as_slice()).unwrap(),
            observation
        );
    }
}
