use std::{pin::Pin, sync::Arc};

use async_stream::try_stream;
use futures::{Stream, StreamExt};
use lume_protocol::{
    PROTOCOL_VERSION, SERVICE_CAPABILITIES, negotiate,
    v1::{
        Acknowledgement, HandshakeAccepted, HandshakeRejected, IntegrityError, RuntimeEnvelope,
        ServiceEnvelope, runtime_envelope,
        runtime_ingest_server::{RuntimeIngest, RuntimeIngestServer},
        service_envelope,
    },
};
use lume_storage::{AppendOutcome, HistoryStore};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::{Request, Response, Status, Streaming, transport::Server};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct IngestService {
    store: Arc<HistoryStore>,
    authentication_token: Arc<[u8]>,
    maximum_in_flight: u32,
}

impl IngestService {
    pub fn new(store: Arc<HistoryStore>, authentication_token: impl Into<Vec<u8>>) -> Self {
        Self {
            store,
            authentication_token: authentication_token.into().into(),
            maximum_in_flight: 128,
        }
    }
}

type ResponseStream = Pin<Box<dyn Stream<Item = Result<ServiceEnvelope, Status>> + Send + 'static>>;

#[tonic::async_trait]
impl RuntimeIngest for IngestService {
    type RuntimeSessionStream = ResponseStream;

    async fn runtime_session(
        &self,
        request: Request<Streaming<RuntimeEnvelope>>,
    ) -> Result<Response<Self::RuntimeSessionStream>, Status> {
        let mut inbound = request.into_inner();
        let store = Arc::clone(&self.store);
        let expected_token = Arc::clone(&self.authentication_token);
        let maximum_in_flight = self.maximum_in_flight;

        let output = try_stream! {
            let first = inbound
                .next()
                .await
                .ok_or_else(|| Status::invalid_argument("runtime session requires a handshake"))??;
            let Some(runtime_envelope::Message::Handshake(handshake)) = first.message else {
                Err(Status::invalid_argument("first runtime envelope must be a handshake"))?;
                unreachable!();
            };

            if handshake.authentication_evidence.as_slice() != expected_token.as_ref() {
                yield rejected("authentication_failed", "runtime authentication evidence was rejected");
                return;
            }
            let runtime_id = handshake.runtime_id.clone();
            if runtime_id.trim().is_empty() || handshake.instance_id.trim().is_empty() {
                yield rejected("invalid_identity", "runtime and instance identities are required");
                return;
            }
            let (selected_version, selected_capabilities) = match negotiate(&handshake) {
                Ok(negotiated) => negotiated,
                Err(rejection) => {
                    yield rejected(rejection.code(), &format!("protocol negotiation rejected: {rejection:?}"));
                    return;
                }
            };
            let acknowledged_positions = store
                .acknowledged_positions(&runtime_id)
                .map_err(internal)?
                .into_iter()
                .collect();
            yield ServiceEnvelope {
                message: Some(service_envelope::Message::HandshakeAccepted(HandshakeAccepted {
                    selected_version,
                    selected_capabilities,
                    required_runtime_capabilities: SERVICE_CAPABILITIES.map(str::to_owned).to_vec(),
                    acknowledged_positions,
                    session_id: Uuid::new_v4().to_string(),
                    maximum_in_flight,
                })),
            };

            while let Some(envelope) = inbound.next().await {
                let envelope = envelope?;
                let Some(runtime_envelope::Message::Observation(observation)) = envelope.message else {
                    Err(Status::invalid_argument("handshake may only appear once"))?;
                    unreachable!();
                };
                let producer_id = observation.producer_id.clone();
                let evidence_id = observation.evidence_id.clone();
                let outcome = store
                    .append_observation(&runtime_id, &observation)
                    .map_err(internal)?;
                match outcome {
                    AppendOutcome::Conflict { .. } => {
                        yield ServiceEnvelope {
                            message: Some(service_envelope::Message::IntegrityError(IntegrityError {
                                evidence_id,
                                code: "identity_conflict".into(),
                                message: "the evidence identity or producer position was reused with different content".into(),
                            })),
                        };
                    }
                    AppendOutcome::Inserted { .. } | AppendOutcome::Duplicate { .. } => {}
                }
                yield ServiceEnvelope {
                    message: Some(service_envelope::Message::Acknowledgement(Acknowledgement {
                        producer_id,
                        highest_contiguous_sequence: outcome.highest_contiguous_sequence(),
                    })),
                };
            }
        };
        Ok(Response::new(Box::pin(output)))
    }
}

fn rejected(code: &str, message: &str) -> ServiceEnvelope {
    ServiceEnvelope {
        message: Some(service_envelope::Message::HandshakeRejected(
            HandshakeRejected {
                code: code.into(),
                message: message.into(),
            },
        )),
    }
}

fn internal(error: impl std::fmt::Display) -> Status {
    Status::internal(format!("local history failure: {error}"))
}

pub async fn serve_tcp(
    listener: TcpListener,
    service: IngestService,
    shutdown: impl Future<Output = ()> + Send + 'static,
) -> Result<(), tonic::transport::Error> {
    Server::builder()
        .add_service(RuntimeIngestServer::new(service))
        .serve_with_incoming_shutdown(TcpListenerStream::new(listener), shutdown)
        .await
}

pub fn protocol_version() -> u32 {
    PROTOCOL_VERSION
}
