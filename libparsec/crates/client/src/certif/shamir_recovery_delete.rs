// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_platform_storage::certificates::UpTo;
use libparsec_protocol::authenticated_cmds;
use libparsec_types::prelude::*;

use crate::{certif::store::LastShamirRecovery, CertificateOps, EventTooMuchDriftWithServerClock};

use super::{
    greater_timestamp, CertifStoreError, CertificateBasedActionOutcome, GreaterTimestampOffset,
};

#[derive(Debug, thiserror::Error)]
pub enum CertifDeleteShamirRecoveryError {
    #[error("Component has stopped")]
    Stopped,
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
    Internal(#[from] anyhow::Error),
}

impl From<CertifStoreError> for CertifDeleteShamirRecoveryError {
    fn from(value: CertifStoreError) -> Self {
        match value {
            CertifStoreError::Stopped => Self::Stopped,
            CertifStoreError::Internal(err) => err.into(),
        }
    }
}

pub(super) async fn delete_shamir_recovery(
    ops: &CertificateOps,
) -> Result<CertificateBasedActionOutcome, CertifDeleteShamirRecoveryError> {
    let author_user_id = ops.device.user_id;

    // 1. Check the deletion is possible according to our local certificates

    let last_shamir_recovery = ops
        .store
        .for_read({
            async |store| {
                store
                    .get_last_shamir_recovery_for_author(UpTo::Current, author_user_id)
                    .await
            }
        })
        .await??;
    let brief = match last_shamir_recovery {
        LastShamirRecovery::Valid(brief) => brief,

        // Return early if the deletion already occured
        LastShamirRecovery::Deleted(_, _) => {
            return Ok(CertificateBasedActionOutcome::LocalIdempotent);
        }
        LastShamirRecovery::NeverSetup => {
            // Since we consider the caller is supposed to first check the existence
            // of the shamir recovery before deciding to delete it, we don't bother
            // to provide a dedicated status for this case.
            // Instead we just consider the caller has achieved its intent (i.e.
            // the author no longer has a shamir recovery).
            return Ok(CertificateBasedActionOutcome::LocalIdempotent);
        }
    };

    // 2. Do the actual shamir recovery delete

    // Keep looping while `RequireGreaterTimestamp` is returned
    let mut to_use_timestamp = ops.device.now();
    loop {
        let outcome = do_shamir_recovery_delete(ops, &brief, to_use_timestamp).await?;

        match outcome {
            DoShamirRecoverySetupOutcome::Done => break,
            DoShamirRecoverySetupOutcome::RequireGreaterTimestamp(strictly_greater_than) => {
                to_use_timestamp = greater_timestamp(
                    &ops.device.time_provider,
                    GreaterTimestampOffset::User,
                    strictly_greater_than,
                );
            }
            DoShamirRecoverySetupOutcome::RemoteIdempotent(last_shamir_certificate_timestamp) => {
                return Ok(CertificateBasedActionOutcome::RemoteIdempotent {
                    certificate_timestamp: last_shamir_certificate_timestamp,
                });
            }
        }
    }

    Ok(CertificateBasedActionOutcome::Uploaded {
        certificate_timestamp: to_use_timestamp,
    })
}

#[derive(Debug)]
enum DoShamirRecoverySetupOutcome {
    Done,
    RequireGreaterTimestamp(DateTime),
    RemoteIdempotent(DateTime),
}

async fn do_shamir_recovery_delete(
    ops: &CertificateOps,
    brief: &ShamirRecoveryBriefCertificate,
    timestamp: DateTime,
) -> Result<DoShamirRecoverySetupOutcome, CertifDeleteShamirRecoveryError> {
    let author_device_id = ops.device.device_id;

    // 1. Generate certificate

    let shamir_recovery_deletion_certificate: Bytes = ShamirRecoveryDeletionCertificate {
        author: author_device_id,
        timestamp,
        setup_to_delete_timestamp: brief.timestamp,
        setup_to_delete_user_id: brief.user_id,
        share_recipients: brief.per_recipient_shares.keys().cloned().collect(),
    }
    .dump_and_sign(&ops.device.signing_key)
    .into();

    // 2. Send certificate

    use authenticated_cmds::latest::shamir_recovery_delete::{Rep, Req};

    let req = Req {
        shamir_recovery_deletion_certificate,
    };
    let rep = ops.cmds.send(req).await?;

    match rep {
        Rep::Ok => Ok(DoShamirRecoverySetupOutcome::Done),

        Rep::RequireGreaterTimestamp {
            strictly_greater_than,
        } => {
            // The retry is handled by the caller
            Ok(DoShamirRecoverySetupOutcome::RequireGreaterTimestamp(
                strictly_greater_than,
            ))
        }
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

            Err(CertifDeleteShamirRecoveryError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            })
        }
        Rep::ShamirRecoveryAlreadyDeleted {
            last_shamir_certificate_timestamp,
        } => Ok(DoShamirRecoverySetupOutcome::RemoteIdempotent(
            last_shamir_certificate_timestamp,
        )),
        bad_rep @ (Rep::InvalidCertificateCorrupted
        | Rep::InvalidCertificateUserIdMustBeSelf
        | Rep::ShamirRecoveryNotFound
        | Rep::RecipientsMismatch
        | Rep::UnknownStatus { .. }) => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}
