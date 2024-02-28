// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_protocol::authenticated_cmds;
use libparsec_types::prelude::*;

use super::{
    store::CertifStoreError, CertifOps, CertificateBasedActionOutcome, InvalidCertificateError,
    InvalidKeysBundleError,
};
use crate::EventTooMuchDriftWithServerClock;

#[derive(Debug, thiserror::Error)]
pub enum CertifRevokeUserError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot reach the server")]
    Offline,
    #[error("Cannot revoke oneself")]
    UserIsSelf,
    #[error("User not found")]
    UserNotFound,
    #[error("Author not allowed")]
    AuthorNotAllowed,
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error("There is no key available in this realm for encryption")]
    NoKey,
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for CertifRevokeUserError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            // TODO: handle organization expired and user revoked here ?
            err => Self::Internal(err.into()),
        }
    }
}

impl From<CertifStoreError> for CertifRevokeUserError {
    fn from(value: CertifStoreError) -> Self {
        match value {
            CertifStoreError::Stopped => Self::Stopped,
            CertifStoreError::Internal(err) => err.into(),
        }
    }
}

pub(super) async fn revoke_user(
    ops: &CertifOps,
    user: UserID,
) -> Result<CertificateBasedActionOutcome, CertifRevokeUserError> {
    if *ops.device.device_id.user_id() == user {
        return Err(CertifRevokeUserError::UserIsSelf);
    }

    // Loop is needed to deal with server requiring greater timestamp
    let mut timestamp = ops.device.now();
    loop {
        let outcome = do_server_command(ops, user.clone(), timestamp).await?;

        match outcome {
            DoServerCommandOutcome::Done(outcome) => return Ok(outcome),
            DoServerCommandOutcome::RequireGreaterTimestamp(strictly_greater_than) => {
                // TODO: handle `strictly_greater_than` out of the client ballpark by
                // returning an error
                timestamp = std::cmp::max(ops.device.time_provider.now(), strictly_greater_than);
            }
        }
    }
}

enum DoServerCommandOutcome {
    Done(CertificateBasedActionOutcome),
    RequireGreaterTimestamp(DateTime),
}

async fn do_server_command(
    ops: &CertifOps,
    recipient: UserID,
    timestamp: DateTime,
) -> Result<DoServerCommandOutcome, CertifRevokeUserError> {
    // 0) Sanity check to prevent generating and invalid certificate

    if recipient == *ops.device.device_id.user_id() {
        return Err(CertifRevokeUserError::UserIsSelf);
    }

    // 1) Build role certificate

    let signed_certificate = RevokedUserCertificate {
        author: ops.device.device_id.clone(),
        timestamp,
        user_id: recipient,
    }
    .dump_and_sign(&ops.device.signing_key);

    // 2) Actually send the command to the server

    use authenticated_cmds::latest::user_revoke::{Rep, Req};

    let req = Req {
        revoked_user_certificate: signed_certificate.into(),
    };
    let rep = ops.cmds.send(req).await?;
    match rep {
        Rep::Ok => Ok(DoServerCommandOutcome::Done(
            CertificateBasedActionOutcome::Uploaded {
                certificate_timestamp: timestamp,
            },
        )),
        Rep::UserAlreadyRevoked {
            last_common_certificate_timestamp,
        } => Ok(DoServerCommandOutcome::Done(
            CertificateBasedActionOutcome::RemoteIdempotent {
                certificate_timestamp: last_common_certificate_timestamp,
            },
        )),
        Rep::RequireGreaterTimestamp {
            strictly_greater_than,
        } => {
            // The retry is handled by the caller
            Ok(DoServerCommandOutcome::RequireGreaterTimestamp(
                strictly_greater_than,
            ))
        }
        Rep::AuthorNotAllowed { .. } => Err(CertifRevokeUserError::AuthorNotAllowed),
        Rep::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            ..
        } => {
            let event = EventTooMuchDriftWithServerClock {
                server_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
                client_timestamp,
            };
            ops.event_bus.send(&event);

            Err(CertifRevokeUserError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            })
        }
        // Unlike for the sharing, we didn't have retrieved the user on our side,
        // so this error can actually occur (this is only theoretical though, as the
        // user ID is supposed to have been obtained from certificates).
        Rep::UserNotFound { .. } => Err(CertifRevokeUserError::UserNotFound),
        bad_rep @ (Rep::InvalidCertificate { .. } | Rep::UnknownStatus { .. }) => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}
