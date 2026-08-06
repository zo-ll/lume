use std::env;

use anyhow::{Context, Result};
use lume_protocol::v1::{
    ItemLifecycle, ItemOutcome, OperationObserved, OperationType, PayloadAvailability,
    ProducerFinalized, TraceEnded, TraceStarted, observation::Evidence,
};
use lume_sdk::RuntimeClient;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    let endpoint = env::var("LUME_ENDPOINT").unwrap_or_else(|_| "http://127.0.0.1:43191".into());
    let token = env::var("LUME_AUTH_TOKEN").unwrap_or_else(|_| "local-development-token".into());
    let trace_id = Uuid::new_v4().to_string();
    let producer_id = "reference-runtime";
    let observations = vec![
        observation(
            producer_id,
            1,
            Evidence::TraceStarted(TraceStarted {
                trace_id: trace_id.clone(),
                agent_id: "reference-agent".into(),
                objective: "Demonstrate the Lume headless walking skeleton".into(),
            }),
        ),
        observation(
            producer_id,
            2,
            Evidence::OperationObserved(OperationObserved {
                trace_id: trace_id.clone(),
                operation_id: Uuid::new_v4().to_string(),
                operation_type: OperationType::ToolCall.into(),
                structural_parent_id: None,
                causal_dependency_ids: Vec::new(),
                lifecycle: ItemLifecycle::Ended.into(),
                outcome: Some(ItemOutcome::Succeeded.into()),
                payload_availability: PayloadAvailability::Present.into(),
                payload: b"reference operation".to_vec(),
            }),
        ),
        observation(
            producer_id,
            3,
            Evidence::TraceEnded(TraceEnded {
                trace_id: trace_id.clone(),
                outcome: ItemOutcome::Succeeded.into(),
                stable_code: None,
            }),
        ),
        observation(
            producer_id,
            4,
            Evidence::ProducerFinalized(ProducerFinalized {
                trace_id: trace_id.clone(),
                final_sequence: 4,
            }),
        ),
    ];

    let client = RuntimeClient::new(endpoint, "reference-runtime", token.into_bytes());
    let receipt = client
        .deliver(observations)
        .await
        .context("failed to deliver the reference Execution Trace")?;
    println!(
        "delivered trace {trace_id}; session {}; acknowledged {:?}",
        receipt.session_id, receipt.acknowledged_positions
    );
    Ok(())
}

fn observation(
    producer_id: &str,
    producer_sequence: u64,
    evidence: Evidence,
) -> lume_protocol::v1::Observation {
    lume_protocol::v1::Observation {
        producer_id: producer_id.into(),
        producer_sequence,
        evidence_id: Uuid::new_v4().to_string(),
        evidence: Some(evidence),
    }
}
