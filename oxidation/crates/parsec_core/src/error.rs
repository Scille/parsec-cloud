// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use thiserror::Error;

use parsec_api_types::DateTime;
use parsec_api_types::{DeviceID, UserID};

#[derive(Error, Debug, PartialEq)]
pub enum TrustchainError {
    #[error("Unexpected certificate: expected `{expected}` but got `{got}`")]
    UnexpectedCertificate { expected: UserID, got: UserID },

    #[error("{path}: Invalid certificate: {exc}")]
    InvalidCertificate { path: String, exc: String },

    #[error("{user_id}: Invalid self-signed user certificate")]
    InvalidSelfSignedUserCertificate { user_id: UserID },

    #[error("{user_id}: Invalid self-signed user revocation certificate")]
    InvalidSelfSignedUserRevocationCertificate { user_id: UserID },

    #[error("{path}: Invalid signature given {user_id} is not admin")]
    InvalidSignatureGiven { path: String, user_id: UserID },

    #[error("{path}: Invalid signaure loop detected")]
    InvalidSignatureLoopDetected { path: String },

    #[error("{path}: Missing device certificate for {device_id}")]
    MissingDeviceCertificate { path: String, device_id: DeviceID },

    #[error("{path}: Missing user certificate for {user_id}")]
    MissingUserCertificate { path: String, user_id: UserID },

    #[error("Signature {verified_timestamp} is posterior to user revocation {user_timestamp}")]
    SignaturePosteriorUserRevocation {
        verified_timestamp: DateTime,
        user_timestamp: DateTime,
    },
}

pub type TrustchainResult<T> = Result<T, TrustchainError>;

pub fn build_signature_path(sign_chain: &[String]) -> String {
    sign_chain.join(" <-sign- ")
}
