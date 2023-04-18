// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_client_connection::CommandError;
use thiserror::Error;

use libparsec_types::prelude::*;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum TrustchainError {
    #[error("{path}: Invalid certificate: {exc}")]
    InvalidCertificate { path: String, exc: String },

    #[error("{user_id}: Invalid self-signed user certificate")]
    InvalidSelfSignedUserCertificate { user_id: UserID },

    #[error("{user_id}: Invalid self-signed user revocation certificate")]
    InvalidSelfSignedUserRevocationCertificate { user_id: UserID },

    #[error("{path}: Invalid signature given {user_id} is not admin")]
    InvalidSignatureGiven { path: String, user_id: UserID },

    #[error("{path}: Invalid signature loop detected")]
    InvalidSignatureLoopDetected { path: String },

    #[error("{path}: Missing device certificate for {device_id}")]
    MissingDeviceCertificate { path: String, device_id: DeviceID },

    #[error("{path}: Missing user certificate for {user_id}")]
    MissingUserCertificate { path: String, user_id: UserID },

    #[error(
        "{path}: Signature ({verified_timestamp}) is posterior to user revocation ({user_timestamp})"
    )]
    SignaturePosteriorUserRevocation {
        path: String,
        verified_timestamp: DateTime,
        user_timestamp: DateTime,
    },

    #[error("Unexpected certificate: expected `{expected}` but got `{got}`")]
    UnexpectedCertificate { expected: UserID, got: UserID },
}

pub type TrustchainResult<T> = Result<T, TrustchainError>;

pub(crate) fn build_signature_path(sign_chain: &[String]) -> String {
    sign_chain.join(" <-sign- ")
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum RemoteDevicesManagerError {
    #[error("User `{user_id}` is not in local cache and we are offline.")]
    BackendOffline { user_id: UserID },

    #[error("Failed to fetch invitation creator for device `{device_id}`: {reason}")]
    FailedFetchInvitationCreatorForDevice { device_id: DeviceID, reason: String },

    #[error("Failed to fetch invitation creator for user `{user_id}`: {reason}")]
    FailedFetchInvitationCreatorForUser { user_id: UserID, reason: String },

    #[error("Failed to fetch user {user_id}: `{reason}`")]
    FailedFetchUser { user_id: UserID, reason: String },

    #[error("{exc}")]
    InvalidTrustchain { exc: TrustchainError },

    #[error("User `{user_id}` doesn't have a device `{device_id}`")]
    DeviceNotFound {
        user_id: UserID,
        device_id: DeviceID,
    },

    #[error("User `{user_id}` doesn't exist in backend")]
    UserNotFound { user_id: UserID },
}

pub type RemoteDevicesManagerResult<T> = Result<T, RemoteDevicesManagerError>;

impl From<TrustchainError> for RemoteDevicesManagerError {
    fn from(exc: TrustchainError) -> Self {
        Self::InvalidTrustchain { exc }
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum InviteError {
    #[error("Invitation not found")]
    NotFound,

    #[error("Invitation already used")]
    AlreadyUsed,

    #[error("Claim operation reset by peer")]
    PeerReset,

    #[error("Active users limit reached")]
    ActiveUsersLimitReached,

    #[error("Backend error during {0}")]
    Backend(String),

    #[error("{0}")]
    Command(CommandError),

    #[error("{0}")]
    Custom(String),
}

pub type InviteResult<T> = Result<T, InviteError>;

impl From<CommandError> for InviteError {
    fn from(exc: CommandError) -> Self {
        match exc {
            CommandError::InvitationAlreadyDeleted => Self::AlreadyUsed,
            CommandError::InvitationNotFound => Self::NotFound,
            _ => Self::Command(exc),
        }
    }
}
