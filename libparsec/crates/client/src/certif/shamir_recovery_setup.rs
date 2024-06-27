// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU64,
};

use libparsec_client_connection::ConnectionError;
use libparsec_protocol::authenticated_cmds::{
    latest::shamir_recovery_setup::{Rep, Req},
    v4::shamir_recovery_setup::ShamirRecoverySetup,
};
use libparsec_types::{
    anyhow, thiserror, DateTime, DeviceID, ShamirRecoveryBriefCertificate,
    ShamirRecoveryShareCertificate, UserID,
};

use crate::{CertificateBasedActionOutcome, CertificateOps, EventTooMuchDriftWithServerClock};

use super::CertifStoreError;

#[derive(Debug, thiserror::Error)]
pub enum CertifShamirSetupError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot reach the server")]
    Offline,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for CertifShamirSetupError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            // TODO: handle organization expired and user revoked here ?
            err => Self::Internal(err.into()),
        }
    }
}

impl From<CertifStoreError> for CertifShamirSetupError {
    fn from(value: CertifStoreError) -> Self {
        match value {
            CertifStoreError::Stopped => Self::Stopped,
            CertifStoreError::Internal(err) => err.into(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CertifShamirError {
    #[error("todo")]
    InvalidThreshold,
    #[error("todo")]
    AuthorIncludedAsRecipient,
    #[error("todo")]
    ShamirSetupAlreadyExist,
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot reach the server")]
    Offline,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("todo")]
    UserRevoked(UserID),
    #[error("todo")]
    ThresholdIsZero,
    #[error("todo")]
    ShareRecipientHasZeroShares,
    #[error("todo")]
    MissingUser(UserID),
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
}

impl From<CertifStoreError> for CertifShamirError {
    fn from(value: CertifStoreError) -> Self {
        match value {
            CertifStoreError::Stopped => Self::Stopped,
            CertifStoreError::Internal(err) => err.into(),
        }
    }
}

impl From<ConnectionError> for CertifShamirError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            // TODO: handle organization expired and user revoked here ?
            err => Self::Internal(err.into()),
        }
    }
}
pub(super) async fn shamir_setup_create(
    certificate_ops: &CertificateOps,
    author_device: DeviceID,
    author_id: UserID,
    share_recipients: HashMap<UserID, u8>,
    threshold: u8,
) -> Result<CertificateBasedActionOutcome, CertifShamirError> {
    // Loop is needed to deal with server requiring greater timestamp
    let mut timestamp = certificate_ops.device.now();
    loop {
        let outcome = shamir_internals(
            certificate_ops,
            author_device,
            author_id,
            &share_recipients,
            threshold,
            timestamp,
        )
        .await?;

        match outcome {
            ShamirInternalsOutcome::Done(outcome) => return Ok(outcome),
            ShamirInternalsOutcome::RequireGreaterTimestamp(strictly_greater_than) => {
                // TODO: handle `strictly_greater_than` out of the client ballpark by
                // returning an error
                timestamp = std::cmp::max(
                    certificate_ops.device.time_provider.now(),
                    strictly_greater_than,
                );
            }
        }
    }
}

enum ShamirInternalsOutcome {
    Done(CertificateBasedActionOutcome),
    RequireGreaterTimestamp(DateTime),
}

async fn shamir_internals(
    certificate_ops: &CertificateOps,
    author_device: DeviceID,
    author_id: UserID,
    share_recipients: &HashMap<UserID, u8>,
    threshold: u8,
    timestamp: DateTime,
) -> Result<ShamirInternalsOutcome, CertifShamirError> {
    // 1. Check if share_recipients and threshold are internally coherent
    if share_recipients
        .iter()
        .fold(0, |acc, (_, share_count)| acc + share_count)
        > threshold
    {
        return Err(CertifShamirError::InvalidThreshold);
    }

    if share_recipients
        .keys()
        .find(|&&recipient| recipient == author_id)
        .is_some()
    {
        return Err(CertifShamirError::AuthorIncludedAsRecipient);
    }

    if threshold == 0 {
        return Err(CertifShamirError::ThresholdIsZero);
    }

    if share_recipients.values().find(|&&v| v == 0).is_some() {
        return Err(CertifShamirError::ShareRecipientHasZeroShares);
    }

    // 2. Check for previous setup
    if get_latest_shamir_setup_for_author(certificate_ops, author_device, author_id)
        .await
        .is_some()
    {
        return Err(CertifShamirError::ShamirSetupAlreadyExist);
    }

    // 3. Check organization's and author's status

    // 4. Check recipients status

    let mut participants_id: HashSet<_> = share_recipients.keys().collect();
    participants_id.insert(&author_id);
    let participants = certificate_ops.list_users(true, None, None).await?;
    let participants = participants
        .iter()
        .filter(|info| participants_id.contains(&info.id));

    // no participant is missing
    for &&id in &participants_id {
        if participants.clone().find(|info| info.id == id).is_none() {
            return Err(CertifShamirError::MissingUser(id));
        }
    }

    // all participants not revoked
    for info in participants {
        if info.revoked_on.is_some() {
            return Err(CertifShamirError::UserRevoked(info.id));
        }
    }
    // 5. Generate certificates

    let brief = ShamirRecoveryBriefCertificate {
        author: author_device,
        timestamp,
        user_id: author_id,
        threshold: NonZeroU64::new(threshold.into()).expect("shamir threshold is zero"), // checked during the first step
        per_recipient_shares: share_recipients
            .iter()
            .map(|(&k, &v)| (k, NonZeroU64::new(v.into()).expect("Share count is zero")))
            .collect(),
    }
    .dump_and_sign(&certificate_ops.device.signing_key)
    .into();

    let shares = share_recipients
        .iter()
        .map(|(&share_recipient_id, share_count)| {
            ShamirRecoveryShareCertificate {
                author: author_device,
                timestamp,
                user_id: author_id,
                recipient: share_recipient_id,
                ciphered_share: todo!(),
            }
            .dump_and_sign(&certificate_ops.device.signing_key)
            .into()
        })
        .collect();

    // 6. Send certificates

    let req = Req {
        setup: Some(ShamirRecoverySetup {
            brief,
            ciphered_data: todo!(),
            reveal_token: todo!(),
            shares,
        }),
    };
    let rep = certificate_ops.cmds.send(req).await?;
    match rep {
        Rep::Ok => Ok(ShamirInternalsOutcome::Done(
            CertificateBasedActionOutcome::Uploaded {
                certificate_timestamp: timestamp,
            },
        )),

        Rep::RequireGreaterTimestamp {
            strictly_greater_than,
        } => {
            // The retry is handled by the caller
            Ok(ShamirInternalsOutcome::RequireGreaterTimestamp(
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
            certificate_ops.event_bus.send(&event);

            Err(CertifShamirError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            })
        }
        Rep::AuthorIncludedAsRecipient => Err(CertifShamirError::AuthorIncludedAsRecipient),
        Rep::InvalidRecipient => Err(CertifShamirError::MissingUser(todo!())),
        Rep::ShamirSetupAlreadyExists {
            last_shamir_certificate_timestamp,
        } => Err(CertifShamirError::ShamirSetupAlreadyExist),
        bad_rep @ (Rep::BriefInvalidData { .. }
        | Rep::ShareInvalidData { .. }
        | Rep::UnknownStatus { .. }
        | Rep::DuplicateShareForRecipient
        | Rep::MissingShareForRecipient
        | Rep::ShareInconsistentTimestamp
        | Rep::ShareRecipientNotInBrief) => {
            //
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}

async fn get_latest_shamir_setup_for_author(
    certificate_ops: &CertificateOps,
    author_device: DeviceID,
    author_id: UserID,
) -> Option<ShamirRecoveryBriefCertificate> {
    todo!()
}
