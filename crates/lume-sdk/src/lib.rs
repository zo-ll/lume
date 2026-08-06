use std::collections::{BTreeMap, HashMap, VecDeque};

use lume_protocol::{
    CAPABILITY_LIVE_CONFIRMATION, CAPABILITY_WRITE_ONLY, PROTOCOL_VERSION,
    v1::{
        Handshake, Observation, RuntimeEnvelope, runtime_envelope,
        runtime_ingest_client::RuntimeIngestClient, service_envelope,
    },
};
use thiserror::Error;
use tonic::transport::Channel;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct RuntimeClient {
    endpoint: String,
    runtime_id: String,
    instance_id: String,
    authentication_token: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeliveryReceipt {
    pub session_id: String,
    pub acknowledged_positions: BTreeMap<String, u64>,
}

#[derive(Debug, Error)]
pub enum SdkError {
    #[error("the service rejected the handshake: {code}: {message}")]
    HandshakeRejected { code: String, message: String },
    #[error("the service ended the session before accepting the handshake")]
    MissingHandshakeResponse,
    #[error("invalid service endpoint: {0}")]
    InvalidEndpoint(String),
    #[error("the service reported an integrity conflict for {evidence_id}: {message}")]
    IntegrityConflict {
        evidence_id: String,
        message: String,
    },
    #[error(transparent)]
    Transport(#[from] tonic::transport::Error),
    #[error(transparent)]
    Status(#[from] tonic::Status),
}

impl RuntimeClient {
    pub fn new(
        endpoint: impl Into<String>,
        runtime_id: impl Into<String>,
        authentication_token: impl Into<Vec<u8>>,
    ) -> Self {
        Self {
            endpoint: endpoint.into(),
            runtime_id: runtime_id.into(),
            instance_id: Uuid::new_v4().to_string(),
            authentication_token: authentication_token.into(),
        }
    }

    pub async fn deliver(
        &self,
        observations: impl IntoIterator<Item = Observation>,
    ) -> Result<DeliveryReceipt, SdkError> {
        let channel = Channel::from_shared(self.endpoint.clone())
            .map_err(|error| SdkError::InvalidEndpoint(error.to_string()))?
            .connect()
            .await?;
        let mut client = RuntimeIngestClient::new(channel);
        let handshake = RuntimeEnvelope {
            message: Some(runtime_envelope::Message::Handshake(Handshake {
                runtime_id: self.runtime_id.clone(),
                instance_id: self.instance_id.clone(),
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
                authentication_evidence: self.authentication_token.clone(),
            })),
        };
        let mut envelopes = Vec::from([handshake]);
        envelopes.extend(observations.into_iter().map(|observation| RuntimeEnvelope {
            message: Some(runtime_envelope::Message::Observation(observation)),
        }));
        let mut inbound = client
            .runtime_session(tokio_stream::iter(envelopes))
            .await?
            .into_inner();
        let first = inbound
            .message()
            .await?
            .ok_or(SdkError::MissingHandshakeResponse)?;
        let Some(first_message) = first.message else {
            return Err(SdkError::MissingHandshakeResponse);
        };
        let accepted = match first_message {
            service_envelope::Message::HandshakeAccepted(accepted) => accepted,
            service_envelope::Message::HandshakeRejected(rejected) => {
                return Err(SdkError::HandshakeRejected {
                    code: rejected.code,
                    message: rejected.message,
                });
            }
            _ => return Err(SdkError::MissingHandshakeResponse),
        };
        let mut acknowledged_positions: BTreeMap<_, _> =
            accepted.acknowledged_positions.into_iter().collect();
        while let Some(envelope) = inbound.message().await? {
            match envelope.message {
                Some(service_envelope::Message::Acknowledgement(acknowledgement)) => {
                    acknowledged_positions.insert(
                        acknowledgement.producer_id,
                        acknowledgement.highest_contiguous_sequence,
                    );
                }
                Some(service_envelope::Message::IntegrityError(error)) => {
                    return Err(SdkError::IntegrityConflict {
                        evidence_id: error.evidence_id,
                        message: error.message,
                    });
                }
                _ => {}
            }
        }
        Ok(DeliveryReceipt {
            session_id: accepted.session_id,
            acknowledged_positions,
        })
    }
}

#[derive(Clone, Debug)]
pub struct ObservationBuffer {
    capacity: usize,
    observations: VecDeque<Observation>,
}

impl ObservationBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            observations: VecDeque::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, observation: Observation) -> Result<(), Box<Observation>> {
        if self.observations.len() == self.capacity {
            return Err(Box::new(observation));
        }
        self.observations.push_back(observation);
        Ok(())
    }

    pub fn drain(&mut self) -> impl Iterator<Item = Observation> + '_ {
        self.observations.drain(..)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use lume_protocol::v1::{TraceStarted, observation::Evidence};
    use lume_service::{IngestService, serve_tcp};
    use lume_storage::HistoryStore;
    use tempfile::tempdir;
    use tokio::{net::TcpListener, sync::oneshot};

    use super::*;

    fn started(sequence: u64) -> Observation {
        Observation {
            producer_id: "reference".into(),
            producer_sequence: sequence,
            evidence_id: format!("evidence-{sequence}"),
            evidence: Some(Evidence::TraceStarted(TraceStarted {
                trace_id: "trace".into(),
                agent_id: "agent".into(),
                objective: "exercise headless ingestion".into(),
            })),
        }
    }

    #[tokio::test]
    async fn runtime_delivery_is_durable_and_replay_is_idempotent() {
        let directory = tempdir().unwrap();
        let history = directory.path().join("history.sqlite");
        let store = Arc::new(HistoryStore::open(&history).unwrap());
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let server_store = Arc::clone(&store);
        let server = tokio::spawn(async move {
            serve_tcp(
                listener,
                IngestService::new(server_store, b"test-token".to_vec()),
                async {
                    let _ = shutdown_rx.await;
                },
            )
            .await
            .unwrap();
        });

        let client = RuntimeClient::new(
            format!("http://{address}"),
            "runtime",
            b"test-token".to_vec(),
        );
        let receipt = client.deliver([started(1)]).await.unwrap();
        assert_eq!(receipt.acknowledged_positions["reference"], 1);
        let replay = client.deliver([started(1)]).await.unwrap();
        assert_eq!(replay.acknowledged_positions["reference"], 1);
        shutdown_tx.send(()).unwrap();
        server.await.unwrap();
        drop(store);

        let reopened = Arc::new(HistoryStore::open(history).unwrap());
        assert_eq!(reopened.observation_count().unwrap(), 1);

        let restarted_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let restarted_address = restarted_listener.local_addr().unwrap();
        let (restarted_shutdown_tx, restarted_shutdown_rx) = oneshot::channel();
        let restarted_store = Arc::clone(&reopened);
        let restarted_server = tokio::spawn(async move {
            serve_tcp(
                restarted_listener,
                IngestService::new(restarted_store, b"test-token".to_vec()),
                async {
                    let _ = restarted_shutdown_rx.await;
                },
            )
            .await
            .unwrap();
        });
        let restarted_client = RuntimeClient::new(
            format!("http://{restarted_address}"),
            "runtime",
            b"test-token".to_vec(),
        );
        let after_restart = restarted_client
            .deliver([started(1), started(2)])
            .await
            .unwrap();
        assert_eq!(after_restart.acknowledged_positions["reference"], 2);
        assert_eq!(reopened.observation_count().unwrap(), 2);
        restarted_shutdown_tx.send(()).unwrap();
        restarted_server.await.unwrap();
    }

    #[test]
    fn bounded_buffer_never_grows_past_capacity() {
        let mut buffer = ObservationBuffer::new(1);
        assert!(buffer.push(started(1)).is_ok());
        assert!(buffer.push(started(2)).is_err());
    }
}
