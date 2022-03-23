// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use parsec_api_types::DateTime;
use thiserror::Error;

use crate::ApiVersion;

#[derive(Error, Debug, PartialEq)]
pub enum HandshakeError {
    #[error("Failed chalenge {0:?}")]
    FailedChallenge(String),
    #[error("Handshake bad administration token")]
    BadAdministrationToken,
    #[error("Handshake bad identity")]
    BadIdentity,
    #[error("Handshake organization expired")]
    OrganizationExpired,
    #[error("Handshake rvk mismatch")]
    RVKMismatch,
    #[error("Handshake revoked device")]
    RevokedDevice,
    #[error("Handshake out of ballpark {0:x?}")]
    OutOfBallpark(ChallengeDataReport),
    #[error("No overlap between client API versions {client_versions:?} and backend API versions {backend_versions:?}")]
    APIVersion {
        client_versions: Vec<ApiVersion>,
        backend_versions: Vec<ApiVersion>,
    },
    #[error("{0}")]
    InvalidMessage(String),
    #[error("`verify_key` param must be provided for authenticated handshake")]
    Authenticated,
}

impl From<&'static str> for HandshakeError {
    fn from(s: &'static str) -> Self {
        Self::InvalidMessage(s.into())
    }
}

#[derive(Debug, PartialEq)]
pub struct ChallengeDataReport {
    pub challenge: Vec<u8>,
    pub supported_api_versions: Vec<ApiVersion>,
    pub backend_timestamp: DateTime,
    pub client_timestamp: DateTime,
    pub ballpark_client_early_offset: f64,
    pub ballpark_client_late_offset: f64,
}
