use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

macro_rules! opaque_id {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub String);

        impl $name {
            pub fn new() -> Self {
                Self(Uuid::new_v4().to_string())
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.to_owned())
            }
        }
    };
}

opaque_id!(AgentId);
opaque_id!(CheckpointId);
opaque_id!(EvidenceId);
opaque_id!(FieldId);
opaque_id!(LinkId);
opaque_id!(OperationId);
opaque_id!(ProducerId);
opaque_id!(RequestId);
opaque_id!(RuntimeId);
opaque_id!(TraceId);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Lifecycle {
    Active,
    Ended,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    Succeeded,
    Failed,
    Cancelled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraceCompleteness {
    Provisional,
    Complete,
    Incomplete,
    Unverified,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayloadAvailability {
    Present,
    Redacted,
    Omitted,
    Truncated,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Sensitivity {
    Ordinary,
    Protected,
    WriteOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationType {
    AgentStep,
    ModelCall,
    ToolCall,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TraceOperation {
    pub operation_id: OperationId,
    pub operation_type: OperationType,
    pub structural_parent_id: Option<OperationId>,
    pub causal_dependency_ids: BTreeSet<OperationId>,
    pub producer_id: ProducerId,
    pub producer_sequence: u64,
    pub lifecycle: Lifecycle,
    pub outcome: Option<Outcome>,
    pub payload_availability: PayloadAvailability,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExecutionTrace {
    pub trace_id: TraceId,
    pub agent_id: AgentId,
    pub runtime_id: RuntimeId,
    pub objective: String,
    pub lifecycle: Lifecycle,
    pub outcome: Option<Outcome>,
    operations: BTreeMap<OperationId, TraceOperation>,
    final_cursors: BTreeMap<ProducerId, u64>,
    known_loss: BTreeMap<ProducerId, Vec<SequenceRange>>,
    connected: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SequenceRange {
    pub first: u64,
    pub last: u64,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum TraceError {
    #[error("objective must not be empty")]
    EmptyObjective,
    #[error("ended lifecycle requires an outcome and active lifecycle forbids one")]
    InvalidLifecycleOutcome,
    #[error("operation identity was reused with different content: {0:?}")]
    ConflictingOperation(OperationId),
    #[error("causal graph contains a cycle")]
    CausalCycle,
    #[error("invalid sequence range {first}..={last}")]
    InvalidSequenceRange { first: u64, last: u64 },
}

impl ExecutionTrace {
    /// Creates an active trace with no observations.
    ///
    /// # Errors
    ///
    /// Returns [`TraceError::EmptyObjective`] when the objective has no visible content.
    pub fn new(
        trace_id: TraceId,
        agent_id: AgentId,
        runtime_id: RuntimeId,
        objective: impl Into<String>,
    ) -> Result<Self, TraceError> {
        let objective = objective.into();
        if objective.trim().is_empty() {
            return Err(TraceError::EmptyObjective);
        }
        Ok(Self {
            trace_id,
            agent_id,
            runtime_id,
            objective,
            lifecycle: Lifecycle::Active,
            outcome: None,
            operations: BTreeMap::new(),
            final_cursors: BTreeMap::new(),
            known_loss: BTreeMap::new(),
            connected: true,
        })
    }

    pub fn operations(&self) -> impl Iterator<Item = &TraceOperation> {
        self.operations.values()
    }

    /// Adds canonical operation evidence, returning `false` for an identical replay.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid lifecycle state, conflicting identity reuse, or a cycle.
    pub fn record_operation(&mut self, operation: TraceOperation) -> Result<bool, TraceError> {
        validate_lifecycle_outcome(operation.lifecycle, operation.outcome)?;
        if let Some(existing) = self.operations.get(&operation.operation_id) {
            return if existing == &operation {
                Ok(false)
            } else {
                Err(TraceError::ConflictingOperation(operation.operation_id))
            };
        }
        let operation_id = operation.operation_id.clone();
        self.operations.insert(operation_id.clone(), operation);
        if self.has_cycle() {
            self.operations.remove(&operation_id);
            return Err(TraceError::CausalCycle);
        }
        Ok(true)
    }

    pub fn end(&mut self, outcome: Outcome) {
        self.lifecycle = Lifecycle::Ended;
        self.outcome = Some(outcome);
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
    }

    pub fn finalize_producer(&mut self, producer: ProducerId, sequence: u64) {
        self.final_cursors.insert(producer, sequence);
    }

    /// Records an exact producer sequence range known to be lost.
    ///
    /// # Errors
    ///
    /// Returns [`TraceError::InvalidSequenceRange`] for zero or inverted ranges.
    pub fn declare_loss(
        &mut self,
        producer: ProducerId,
        range: SequenceRange,
    ) -> Result<(), TraceError> {
        if range.first == 0 || range.first > range.last {
            return Err(TraceError::InvalidSequenceRange {
                first: range.first,
                last: range.last,
            });
        }
        self.known_loss.entry(producer).or_default().push(range);
        Ok(())
    }

    pub fn completeness(&self) -> TraceCompleteness {
        if !self.known_loss.is_empty() {
            return TraceCompleteness::Incomplete;
        }
        if self.lifecycle == Lifecycle::Active {
            return if self.connected {
                TraceCompleteness::Provisional
            } else {
                TraceCompleteness::Unverified
            };
        }

        let producers: BTreeSet<_> = self
            .operations
            .values()
            .map(|operation| operation.producer_id.clone())
            .collect();
        if producers.is_empty()
            || producers
                .iter()
                .any(|id| !self.final_cursors.contains_key(id))
        {
            return TraceCompleteness::Unverified;
        }

        for producer in producers {
            let Some(final_cursor) = self.final_cursors.get(&producer) else {
                return TraceCompleteness::Unverified;
            };
            let positions: BTreeSet<_> = self
                .operations
                .values()
                .filter(|operation| operation.producer_id == producer)
                .map(|operation| operation.producer_sequence)
                .collect();
            if (1..=*final_cursor).any(|position| !positions.contains(&position)) {
                return TraceCompleteness::Incomplete;
            }
        }
        TraceCompleteness::Complete
    }

    fn has_cycle(&self) -> bool {
        fn visit(
            id: &OperationId,
            operations: &BTreeMap<OperationId, TraceOperation>,
            visiting: &mut HashSet<OperationId>,
            visited: &mut HashSet<OperationId>,
        ) -> bool {
            if visited.contains(id) {
                return false;
            }
            if !visiting.insert(id.clone()) {
                return true;
            }
            if let Some(operation) = operations.get(id) {
                let dependencies = operation
                    .structural_parent_id
                    .iter()
                    .chain(operation.causal_dependency_ids.iter());
                for dependency in dependencies {
                    if operations.contains_key(dependency)
                        && visit(dependency, operations, visiting, visited)
                    {
                        return true;
                    }
                }
            }
            visiting.remove(id);
            visited.insert(id.clone());
            false
        }

        let mut visiting = HashSet::new();
        let mut visited = HashSet::new();
        self.operations
            .keys()
            .any(|id| visit(id, &self.operations, &mut visiting, &mut visited))
    }
}

fn validate_lifecycle_outcome(
    lifecycle: Lifecycle,
    outcome: Option<Outcome>,
) -> Result<(), TraceError> {
    if matches!(lifecycle, Lifecycle::Ended) == outcome.is_some() {
        Ok(())
    } else {
        Err(TraceError::InvalidLifecycleOutcome)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkKind {
    Delegation,
    Fork,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkEvidenceState {
    Pending,
    Confirmed,
    Conflicted,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TraceLink {
    pub link_id: LinkId,
    pub kind: LinkKind,
    pub source_trace_id: TraceId,
    pub source_operation_id: Option<OperationId>,
    pub source_checkpoint_id: Option<CheckpointId>,
    pub child_trace_id: TraceId,
    pub evidence_state: LinkEvidenceState,
}

#[derive(Clone, Debug, Default)]
pub struct AncestryGraph {
    incoming: HashMap<TraceId, TraceLink>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum AncestryError {
    #[error("a trace cannot be its own ancestor")]
    SelfLink,
    #[error("child trace already has a confirmed incoming core link: {0:?}")]
    MultipleOrigins(TraceId),
    #[error("confirmed link would create an ancestry cycle")]
    Cycle,
    #[error("only confirmed links belong in canonical ancestry")]
    NotConfirmed,
}

impl AncestryGraph {
    /// Adds a confirmed core link to canonical ancestry.
    ///
    /// # Errors
    ///
    /// Rejects non-confirmed evidence, self-links, multiple origins, and cycles.
    pub fn add(&mut self, link: TraceLink) -> Result<(), AncestryError> {
        if link.evidence_state != LinkEvidenceState::Confirmed {
            return Err(AncestryError::NotConfirmed);
        }
        if link.source_trace_id == link.child_trace_id {
            return Err(AncestryError::SelfLink);
        }
        if self.incoming.contains_key(&link.child_trace_id) {
            return Err(AncestryError::MultipleOrigins(link.child_trace_id));
        }
        let mut cursor = &link.source_trace_id;
        while let Some(parent_link) = self.incoming.get(cursor) {
            if parent_link.source_trace_id == link.child_trace_id {
                return Err(AncestryError::Cycle);
            }
            cursor = &parent_link.source_trace_id;
        }
        self.incoming.insert(link.child_trace_id.clone(), link);
        Ok(())
    }

    pub fn incoming(&self, trace_id: &TraceId) -> Option<&TraceLink> {
        self.incoming.get(trace_id)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointAvailability {
    Available,
    Unknown,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForkExecutionMode {
    Sandboxed,
    Live,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FieldType {
    String {
        min_length: Option<usize>,
        max_length: Option<usize>,
    },
    Integer {
        minimum: Option<i64>,
        maximum: Option<i64>,
    },
    Number {
        minimum: Option<f64>,
        maximum: Option<f64>,
    },
    Boolean,
    Enum {
        choices: BTreeSet<String>,
    },
    Object,
    List {
        min_items: Option<usize>,
        max_items: Option<usize>,
    },
    Unsupported {
        type_name: String,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InterventionField {
    pub field_id: FieldId,
    pub label: String,
    pub field_type: FieldType,
    pub nullable: bool,
    pub sensitivity: Sensitivity,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InterventionSchema {
    pub schema_id: String,
    pub fields: BTreeMap<FieldId, InterventionField>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InterventionValue {
    Null,
    String(String),
    Integer(i64),
    Number(f64),
    Boolean(bool),
    Object(BTreeMap<String, InterventionValue>),
    List(Vec<InterventionValue>),
}

pub type Intervention = BTreeMap<FieldId, InterventionValue>;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum InterventionError {
    #[error("intervention must contain at least one change")]
    Empty,
    #[error("unknown intervention field: {0:?}")]
    UnknownField(FieldId),
    #[error("field is not editable because its type is unsupported: {0:?}")]
    UnsupportedField(FieldId),
    #[error("invalid value for field {field_id:?}: {reason}")]
    InvalidValue { field_id: FieldId, reason: String },
}

impl InterventionSchema {
    /// Validates the portable shape and constraints of a sparse Intervention.
    ///
    /// # Errors
    ///
    /// Returns an error for an empty Intervention, unknown or unsupported fields, or invalid values.
    pub fn validate(&self, intervention: &Intervention) -> Result<(), InterventionError> {
        if intervention.is_empty() {
            return Err(InterventionError::Empty);
        }
        for (field_id, value) in intervention {
            let field = self
                .fields
                .get(field_id)
                .ok_or_else(|| InterventionError::UnknownField(field_id.clone()))?;
            validate_intervention_value(field, value)?;
        }
        Ok(())
    }
}

fn invalid(field_id: &FieldId, reason: impl Into<String>) -> InterventionError {
    InterventionError::InvalidValue {
        field_id: field_id.clone(),
        reason: reason.into(),
    }
}

#[allow(clippy::too_many_lines)]
fn validate_intervention_value(
    field: &InterventionField,
    value: &InterventionValue,
) -> Result<(), InterventionError> {
    if matches!(value, InterventionValue::Null) {
        return if field.nullable {
            Ok(())
        } else {
            Err(invalid(&field.field_id, "null is not allowed"))
        };
    }
    match (&field.field_type, value) {
        (
            FieldType::String {
                min_length,
                max_length,
            },
            InterventionValue::String(value),
        ) => {
            let length = value.chars().count();
            if min_length.is_some_and(|minimum| length < minimum)
                || max_length.is_some_and(|maximum| length > maximum)
            {
                return Err(invalid(&field.field_id, "string length is outside bounds"));
            }
        }
        (FieldType::Integer { minimum, maximum }, InterventionValue::Integer(value)) => {
            if minimum.is_some_and(|minimum| *value < minimum)
                || maximum.is_some_and(|maximum| *value > maximum)
            {
                return Err(invalid(&field.field_id, "integer is outside bounds"));
            }
        }
        (FieldType::Number { minimum, maximum }, InterventionValue::Number(value)) => {
            if !value.is_finite()
                || minimum.is_some_and(|minimum| *value < minimum)
                || maximum.is_some_and(|maximum| *value > maximum)
            {
                return Err(invalid(&field.field_id, "number is outside bounds"));
            }
        }
        (FieldType::Boolean, InterventionValue::Boolean(_))
        | (FieldType::Object, InterventionValue::Object(_)) => {}
        (FieldType::Enum { choices }, InterventionValue::String(value)) => {
            if !choices.contains(value) {
                return Err(invalid(&field.field_id, "value is not an enum member"));
            }
        }
        (
            FieldType::List {
                min_items,
                max_items,
            },
            InterventionValue::List(values),
        ) => {
            if min_items.is_some_and(|minimum| values.len() < minimum)
                || max_items.is_some_and(|maximum| values.len() > maximum)
            {
                return Err(invalid(&field.field_id, "list length is outside bounds"));
            }
        }
        (FieldType::Unsupported { .. }, _) => {
            return Err(InterventionError::UnsupportedField(field.field_id.clone()));
        }
        _ => return Err(invalid(&field.field_id, "value has the wrong type")),
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForkCheckpoint {
    pub checkpoint_id: CheckpointId,
    pub source_trace_id: TraceId,
    pub included_producer_positions: BTreeMap<ProducerId, u64>,
    pub runtime_id: RuntimeId,
    pub state_identity: String,
    pub restore_reference: String,
    pub intervention_schema: InterventionSchema,
    pub execution_modes: BTreeSet<ForkExecutionMode>,
    pub availability: CheckpointAvailability,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum CausalCutError {
    #[error("checkpoint source trace does not match the inspected trace")]
    WrongTrace,
    #[error("causal cut omits dependency {dependency:?} required by {operation:?}")]
    NotClosed {
        operation: OperationId,
        dependency: OperationId,
    },
}

impl ForkCheckpoint {
    /// Verifies that every known dependency of an included operation is also inside the cut.
    ///
    /// # Errors
    ///
    /// Returns an error when the checkpoint names another trace or its cut is not causally closed.
    pub fn validate_causal_cut(&self, trace: &ExecutionTrace) -> Result<(), CausalCutError> {
        if self.source_trace_id != trace.trace_id {
            return Err(CausalCutError::WrongTrace);
        }
        let included = |operation: &TraceOperation| {
            self.included_producer_positions
                .get(&operation.producer_id)
                .is_some_and(|position| operation.producer_sequence <= *position)
        };
        for operation in trace
            .operations
            .values()
            .filter(|operation| included(operation))
        {
            for dependency_id in operation
                .structural_parent_id
                .iter()
                .chain(operation.causal_dependency_ids.iter())
            {
                if let Some(dependency) = trace.operations.get(dependency_id)
                    && !included(dependency)
                {
                    return Err(CausalCutError::NotClosed {
                        operation: operation.operation_id.clone(),
                        dependency: dependency_id.clone(),
                    });
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForkRequestState {
    AwaitingConfirmation,
    Recovering,
    Preparing,
    Uncertain,
    Rejected,
    Cancelled,
    Accepted,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForkRequest {
    pub request_id: RequestId,
    pub checkpoint_id: CheckpointId,
    pub intervention: Intervention,
    pub mode: ForkExecutionMode,
    pub state: ForkRequestState,
    pub accepted_child: Option<TraceId>,
}

#[derive(Clone, Debug, Default)]
pub struct ForkRequestLedger {
    requests: HashMap<RequestId, ForkRequest>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
#[error("fork request identity was reused with different content: {0:?}")]
pub struct ForkRequestConflict(pub RequestId);

impl ForkRequestLedger {
    /// Records a new request, returning `false` when the same request is replayed unchanged.
    ///
    /// # Errors
    ///
    /// Returns [`ForkRequestConflict`] when a request ID is reused with different content.
    pub fn record(&mut self, request: ForkRequest) -> Result<bool, ForkRequestConflict> {
        if let Some(existing) = self.requests.get(&request.request_id) {
            return if existing == &request {
                Ok(false)
            } else {
                Err(ForkRequestConflict(request.request_id))
            };
        }
        self.requests.insert(request.request_id.clone(), request);
        Ok(true)
    }

    pub fn get(&self, request_id: &RequestId) -> Option<&ForkRequest> {
        self.requests.get(request_id)
    }
}

pub fn index_operations_by_producer(
    operations: impl IntoIterator<Item = TraceOperation>,
) -> HashMap<ProducerId, BTreeMap<u64, TraceOperation>> {
    let mut index: HashMap<ProducerId, BTreeMap<u64, TraceOperation>> = HashMap::new();
    for operation in operations {
        index
            .entry(operation.producer_id.clone())
            .or_default()
            .insert(operation.producer_sequence, operation);
    }
    index
}

#[cfg(test)]
mod tests {
    use super::*;

    fn operation(id: &str, sequence: u64, dependencies: &[&str]) -> TraceOperation {
        TraceOperation {
            operation_id: id.into(),
            operation_type: OperationType::ToolCall,
            structural_parent_id: None,
            causal_dependency_ids: dependencies.iter().map(|id| (*id).into()).collect(),
            producer_id: "producer".into(),
            producer_sequence: sequence,
            lifecycle: Lifecycle::Ended,
            outcome: Some(Outcome::Succeeded),
            payload_availability: PayloadAvailability::Present,
        }
    }

    fn trace() -> ExecutionTrace {
        ExecutionTrace::new("trace".into(), "agent".into(), "runtime".into(), "test")
            .expect("valid trace")
    }

    #[test]
    fn parallel_operations_remain_unordered() {
        let mut trace = trace();
        trace.record_operation(operation("left", 1, &[])).unwrap();
        trace.record_operation(operation("right", 2, &[])).unwrap();
        trace
            .record_operation(operation("join", 3, &["left", "right"]))
            .unwrap();
        assert_eq!(trace.operations().count(), 3);
    }

    #[test]
    fn cycles_are_rejected_without_poisoning_valid_evidence() {
        let mut trace = trace();
        trace.record_operation(operation("a", 1, &["b"])).unwrap();
        assert_eq!(
            trace.record_operation(operation("b", 2, &["a"])),
            Err(TraceError::CausalCycle)
        );
        assert_eq!(trace.operations().count(), 1);
    }

    #[test]
    fn completeness_requires_final_contiguous_positions() {
        let mut trace = trace();
        trace.record_operation(operation("one", 1, &[])).unwrap();
        trace.record_operation(operation("three", 3, &[])).unwrap();
        trace.end(Outcome::Succeeded);
        trace.finalize_producer("producer".into(), 3);
        assert_eq!(trace.completeness(), TraceCompleteness::Incomplete);
    }

    #[test]
    fn disconnect_does_not_invent_an_outcome() {
        let mut trace = trace();
        trace.disconnect();
        assert_eq!(trace.lifecycle, Lifecycle::Active);
        assert_eq!(trace.outcome, None);
        assert_eq!(trace.completeness(), TraceCompleteness::Unverified);
    }

    #[test]
    fn sparse_intervention_distinguishes_zero_empty_and_null() {
        let fields = [
            InterventionField {
                field_id: "count".into(),
                label: "Count".into(),
                field_type: FieldType::Integer {
                    minimum: Some(0),
                    maximum: None,
                },
                nullable: false,
                sensitivity: Sensitivity::Ordinary,
            },
            InterventionField {
                field_id: "note".into(),
                label: "Note".into(),
                field_type: FieldType::String {
                    min_length: Some(0),
                    max_length: None,
                },
                nullable: true,
                sensitivity: Sensitivity::Protected,
            },
        ];
        let schema = InterventionSchema {
            schema_id: "schema-v1".into(),
            fields: fields
                .into_iter()
                .map(|field| (field.field_id.clone(), field))
                .collect(),
        };
        let intervention = BTreeMap::from([
            (FieldId::from("count"), InterventionValue::Integer(0)),
            (FieldId::from("note"), InterventionValue::Null),
        ]);
        assert_eq!(schema.validate(&intervention), Ok(()));
    }

    #[test]
    fn unsupported_fields_never_degrade_to_strings() {
        let field = InterventionField {
            field_id: "custom".into(),
            label: "Custom".into(),
            field_type: FieldType::Unsupported {
                type_name: "vendor.special".into(),
            },
            nullable: false,
            sensitivity: Sensitivity::Ordinary,
        };
        let schema = InterventionSchema {
            schema_id: "schema-v1".into(),
            fields: BTreeMap::from([(field.field_id.clone(), field)]),
        };
        assert_eq!(
            schema.validate(&BTreeMap::from([(
                FieldId::from("custom"),
                InterventionValue::String("guess".into())
            )])),
            Err(InterventionError::UnsupportedField("custom".into()))
        );
    }

    #[test]
    fn fork_request_replay_is_idempotent_but_identity_reuse_conflicts() {
        let request = ForkRequest {
            request_id: "request".into(),
            checkpoint_id: "checkpoint".into(),
            intervention: BTreeMap::from([(FieldId::from("count"), InterventionValue::Integer(2))]),
            mode: ForkExecutionMode::Sandboxed,
            state: ForkRequestState::Preparing,
            accepted_child: None,
        };
        let mut ledger = ForkRequestLedger::default();
        assert_eq!(ledger.record(request.clone()), Ok(true));
        assert_eq!(ledger.record(request.clone()), Ok(false));

        let changed = ForkRequest {
            mode: ForkExecutionMode::Live,
            ..request
        };
        assert_eq!(
            ledger.record(changed),
            Err(ForkRequestConflict("request".into()))
        );
    }

    #[test]
    fn canonical_ancestry_requires_confirmation_and_single_origin() {
        let pending = TraceLink {
            link_id: "pending".into(),
            kind: LinkKind::Delegation,
            source_trace_id: "root".into(),
            source_operation_id: Some("operation".into()),
            source_checkpoint_id: None,
            child_trace_id: "child".into(),
            evidence_state: LinkEvidenceState::Pending,
        };
        let mut graph = AncestryGraph::default();
        assert_eq!(graph.add(pending.clone()), Err(AncestryError::NotConfirmed));
        graph
            .add(TraceLink {
                evidence_state: LinkEvidenceState::Confirmed,
                ..pending
            })
            .unwrap();
        assert_eq!(
            graph.add(TraceLink {
                link_id: "other".into(),
                kind: LinkKind::Fork,
                source_trace_id: "other-root".into(),
                source_operation_id: None,
                source_checkpoint_id: Some("checkpoint".into()),
                child_trace_id: "child".into(),
                evidence_state: LinkEvidenceState::Confirmed,
            }),
            Err(AncestryError::MultipleOrigins("child".into()))
        );
    }

    #[test]
    fn checkpoint_cut_must_include_both_parallel_dependencies() {
        let mut trace = trace();
        trace.record_operation(operation("left", 1, &[])).unwrap();
        trace.record_operation(operation("right", 2, &[])).unwrap();
        trace
            .record_operation(operation("join", 3, &["left", "right"]))
            .unwrap();
        let checkpoint = ForkCheckpoint {
            checkpoint_id: "checkpoint".into(),
            source_trace_id: "trace".into(),
            included_producer_positions: BTreeMap::from([(ProducerId::from("producer"), 3)]),
            runtime_id: "runtime".into(),
            state_identity: "state".into(),
            restore_reference: "opaque".into(),
            intervention_schema: InterventionSchema {
                schema_id: "schema".into(),
                fields: BTreeMap::new(),
            },
            execution_modes: BTreeSet::from([ForkExecutionMode::Sandboxed]),
            availability: CheckpointAvailability::Available,
        };
        assert_eq!(checkpoint.validate_causal_cut(&trace), Ok(()));
    }
}
