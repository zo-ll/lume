use prost::Message;
use sha2::{Digest, Sha256};

#[allow(clippy::all, clippy::pedantic)]
pub mod v1 {
    tonic::include_proto!("lume.runtime.v1");
}

pub const PROTOCOL_VERSION: u32 = 1;
pub const CAPABILITY_WRITE_ONLY: &str = "lume.write_only.v1";
pub const CAPABILITY_LIVE_CONFIRMATION: &str = "lume.live_confirmation.v1";
pub const SERVICE_CAPABILITIES: [&str; 2] = [CAPABILITY_WRITE_ONLY, CAPABILITY_LIVE_CONFIRMATION];

pub fn encoded_observation(observation: &v1::Observation) -> Vec<u8> {
    observation.encode_to_vec()
}

pub fn observation_digest(observation: &v1::Observation) -> [u8; 32] {
    Sha256::digest(encoded_observation(observation)).into()
}

/// Selects a compatible version and the intersection of supported capabilities.
///
/// # Errors
///
/// Returns a stable rejection when no version overlaps or a required capability is absent.
pub fn negotiate(handshake: &v1::Handshake) -> Result<(u32, Vec<String>), NegotiationRejection> {
    if !handshake.supported_versions.contains(&PROTOCOL_VERSION) {
        return Err(NegotiationRejection::NoCommonVersion);
    }
    for required in &handshake.required_lume_capabilities {
        if !SERVICE_CAPABILITIES.contains(&required.as_str()) {
            return Err(NegotiationRejection::MissingRequiredCapability(
                required.clone(),
            ));
        }
    }
    let selected = handshake
        .supported_capabilities
        .iter()
        .filter(|capability| SERVICE_CAPABILITIES.contains(&capability.as_str()))
        .cloned()
        .collect();
    Ok((PROTOCOL_VERSION, selected))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NegotiationRejection {
    NoCommonVersion,
    MissingRequiredCapability(String),
}

impl NegotiationRejection {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NoCommonVersion => "no_common_version",
            Self::MissingRequiredCapability(_) => "missing_required_capability",
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn handshake() -> v1::Handshake {
        v1::Handshake {
            runtime_id: "runtime".into(),
            instance_id: "instance".into(),
            supported_versions: vec![1],
            supported_capabilities: SERVICE_CAPABILITIES.map(str::to_owned).to_vec(),
            required_lume_capabilities: vec![CAPABILITY_WRITE_ONLY.into()],
            resume_positions: HashMap::default(),
            authentication_evidence: b"token".to_vec(),
        }
    }

    #[test]
    fn negotiation_selects_common_capabilities() {
        let (version, capabilities) = negotiate(&handshake()).unwrap();
        assert_eq!(version, 1);
        assert_eq!(capabilities.len(), 2);
    }

    #[test]
    fn required_safety_capabilities_never_silently_downgrade() {
        let mut candidate = handshake();
        candidate
            .required_lume_capabilities
            .push("lume.unknown_safety.v1".into());
        assert_eq!(
            negotiate(&candidate),
            Err(NegotiationRejection::MissingRequiredCapability(
                "lume.unknown_safety.v1".into()
            ))
        );
    }

    #[test]
    fn opaque_extension_bytes_round_trip_losslessly() {
        let original = v1::OpaqueExtension {
            namespace: "vendor.example".into(),
            type_name: "thought_summary".into(),
            schema_identity: "example/schema".into(),
            schema_version: 4,
            encoding: "application/cbor".into(),
            sensitivity: v1::PayloadSensitivity::Protected.into(),
            semantic_impact: v1::SemanticImpact::PresentationOnly.into(),
            original_bytes: vec![0, 1, 2, 254, 255],
        };
        let bytes = original.encode_to_vec();
        assert_eq!(
            v1::OpaqueExtension::decode(bytes.as_slice()).unwrap(),
            original
        );
    }
}
