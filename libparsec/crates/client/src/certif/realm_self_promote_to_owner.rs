// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_protocol::authenticated_cmds;
use libparsec_types::prelude::*;

use super::{
    greater_timestamp, store::CertifStoreError, CertificateBasedActionOutcome, CertificateOps,
    GreaterTimestampOffset, InvalidCertificateError,
};
use crate::EventTooMuchDriftWithServerClock;

#[derive(Debug, thiserror::Error)]
pub enum CertifSelfPromoteToOwnerError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Author not allowed")]
    AuthorNotAllowed,
    #[error("Workspace realm not found")]
    RealmNotFound,
    #[error("An active user is already OWNER of this workspace")]
    ActiveOwnerAlreadyExists,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<CertifStoreError> for CertifSelfPromoteToOwnerError {
    fn from(value: CertifStoreError) -> Self {
        match value {
            CertifStoreError::Stopped => Self::Stopped,
            CertifStoreError::Internal(err) => err.into(),
        }
    }
}

pub(super) async fn self_promote_to_owner(
    ops: &CertificateOps,
    realm_id: VlobID,
) -> Result<CertificateBasedActionOutcome, CertifSelfPromoteToOwnerError> {
    // Loop is needed to deal with server requiring greater timestamp
    let mut timestamp = ops.device.now();
    loop {
        let outcome = do_server_command(ops, realm_id, timestamp).await?;

        match outcome {
            DoServerCommandOutcome::Done(outcome) => return Ok(outcome),
            DoServerCommandOutcome::RequireGreaterTimestamp(strictly_greater_than) => {
                // TODO: handle `strictly_greater_than` out of the client ballpark by
                // returning an error
                timestamp = greater_timestamp(
                    &ops.device.time_provider,
                    GreaterTimestampOffset::Realm,
                    strictly_greater_than,
                );
            }
        }
    }
}

enum DoServerCommandOutcome {
    Done(CertificateBasedActionOutcome),
    RequireGreaterTimestamp(DateTime),
}

async fn do_server_command(
    ops: &CertificateOps,
    realm_id: VlobID,
    timestamp: DateTime,
) -> Result<DoServerCommandOutcome, CertifSelfPromoteToOwnerError> {
    // Build role certificate with author promoting themselves to OWNER
    let signed_certificate = RealmRoleCertificate {
        author: ops.device.device_id,
        timestamp,
        realm_id,
        user_id: ops.device.user_id,
        role: Some(RealmRole::Owner),
    }
    .dump_and_sign(&ops.device.signing_key);

    use authenticated_cmds::latest::realm_self_promote_to_owner::{Rep, Req};

    let req = Req {
        realm_role_certificate: signed_certificate.into(),
    };
    let rep = ops.cmds.send(req).await?;
    match rep {
        Rep::Ok => Ok(DoServerCommandOutcome::Done(
            CertificateBasedActionOutcome::Uploaded {
                certificate_timestamp: timestamp,
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
        Rep::ActiveOwnerAlreadyExists { .. } => {
            Err(CertifSelfPromoteToOwnerError::ActiveOwnerAlreadyExists)
        }
        Rep::AuthorNotAllowed => Err(CertifSelfPromoteToOwnerError::AuthorNotAllowed),
        Rep::RealmNotFound => Err(CertifSelfPromoteToOwnerError::RealmNotFound),
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

            Err(CertifSelfPromoteToOwnerError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            })
        }
        bad_rep @ (Rep::InvalidCertificate | Rep::UnknownStatus { .. }) => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}
