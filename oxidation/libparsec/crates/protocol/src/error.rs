// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_types::DateTime;
use thiserror::Error;

use crate::{ApiVersion, HANDSHAKE_CHALLENGE_SIZE};

#[derive(Error, Debug, PartialEq)]
pub enum HandshakeError {
    #[error("Handshake invalid answer (bad signature or challenge value)")]
    FailedChallenge,
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
    #[error("Handshake invalid message: {0}")]
    InvalidMessage(&'static str),
}

#[derive(Debug, PartialEq)]
pub struct ChallengeDataReport {
    pub challenge: [u8; HANDSHAKE_CHALLENGE_SIZE],
    pub supported_api_versions: Vec<ApiVersion>,
    pub backend_timestamp: DateTime,
    pub client_timestamp: DateTime,
    pub ballpark_client_early_offset: f64,
    pub ballpark_client_late_offset: f64,
}

/// Error while deserializing data.
pub type DecodeError = rmp_serde::decode::Error;
