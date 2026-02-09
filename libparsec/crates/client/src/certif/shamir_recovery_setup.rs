// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, num::NonZeroU8};

use libparsec_client_connection::ConnectionError;
use libparsec_platform_storage::certificates::{PerTopicLastTimestamps, UpTo};
use libparsec_protocol::authenticated_cmds;
use libparsec_types::prelude::*;

use crate::{
    certif::store::{LastShamirRecovery, LastUserExistAndRevokedInfo},
    CertificateOps, EventTooMuchDriftWithServerClock, InvalidCertificateError,
};

use super::{greater_timestamp, CertifPollServerError, CertifStoreError, GreaterTimestampOffset};

#[derive(Debug, thiserror::Error)]
pub enum CertifSetupShamirRecoveryError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Threshold cannot be bigger than the total of share count")]
    ThresholdBiggerThanSumOfShares,
    #[error("There can be at most 255 shares")]
    TooManyShares,
    #[error("Author cannot be among the recipients")]
    AuthorAmongRecipients,
    #[error("A recipient user is not found")]
    RecipientNotFound,
    #[error("A recipient user is revoked")]
    RecipientRevoked,
    #[error("Shamir recovery already exists")]
    ShamirRecoveryAlreadyExists,
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

impl From<CertifStoreError> for CertifSetupShamirRecoveryError {
    fn from(value: CertifStoreError) -> Self {
        match value {
            CertifStoreError::Stopped => Self::Stopped,
            CertifStoreError::Internal(err) => err.into(),
        }
    }
}

pub struct ShamirRecoverySetupCertificateTimestamps {
    pub shamir_recovery_certificate_timestamp: DateTime,
    pub common_certificate_timestamp: DateTime,
}

pub(super) async fn setup_shamir_recovery(
    ops: &CertificateOps,
    per_recipient_shares: HashMap<UserID, NonZeroU8>,
    threshold: NonZeroU8,
) -> Result<ShamirRecoverySetupCertificateTimestamps, CertifSetupShamirRecoveryError> {
    let author_user_id = ops.device.user_id;

    // 1. Validate `per_recipient_shares` and `threshold` parameters

    let total_share_count = {
        let mut total_share_count: usize = 0;
        for (recipient_user_id, recipient_share_count) in per_recipient_shares.iter() {
            if *recipient_user_id == author_user_id {
                return Err(CertifSetupShamirRecoveryError::AuthorAmongRecipients);
            }
            total_share_count += recipient_share_count.get() as usize;
        }

        // Note this check also ensures that total_share_count > 0 since threshold > 0
        if total_share_count < threshold.get() as usize {
            return Err(CertifSetupShamirRecoveryError::ThresholdBiggerThanSumOfShares);
        }
        if total_share_count > 255 {
            return Err(CertifSetupShamirRecoveryError::TooManyShares);
        }

        NonZeroU8::try_from(u8::try_from(total_share_count).expect("already checked"))
            .expect("already checked")
    };

    // 2. Check the setup is possible according to our local certificates

    let mut recipients_with_share_count_and_pubkey =
        check_against_local_certificates(ops, &per_recipient_shares).await?;

    // 3. Create the recovery device

    // Keep looping while `RequireGreaterTimestamp` is returned
    let mut recovery_device_timestamp = ops.device.now();
    let recovery_device = loop {
        let outcome = create_shamir_recovery_device(ops, recovery_device_timestamp).await?;

        match outcome {
            CreateShamirRecoveryDeviceOutcome::Done(recovery_device) => break recovery_device,
            CreateShamirRecoveryDeviceOutcome::RequireGreaterTimestamp(strictly_greater_than) => {
                recovery_device_timestamp = greater_timestamp(
                    &ops.device.time_provider,
                    GreaterTimestampOffset::User,
                    strictly_greater_than,
                );
            }
        }
    };

    // 4. Do the actual shamir recovery setup

    // Keep looping while `RequireGreaterTimestamp` or `RequirePollingCertificates` is returned
    let mut recovery_setup_timestamp = ops.device.now();
    loop {
        let outcome = do_shamir_recovery_setup(
            ops,
            &recovery_device,
            &recipients_with_share_count_and_pubkey,
            total_share_count,
            threshold,
            recovery_setup_timestamp,
        )
        .await?;

        match outcome {
            DoShamirRecoverySetupOutcome::Done => break,
            DoShamirRecoverySetupOutcome::RequireGreaterTimestamp(strictly_greater_than) => {
                recovery_setup_timestamp = greater_timestamp(
                    &ops.device.time_provider,
                    GreaterTimestampOffset::User,
                    strictly_greater_than,
                );
            }
            DoShamirRecoverySetupOutcome::RequirePollingCertificates(per_topic_last_timestamps) => {
                ops.poll_server_for_new_certificates(Some(&per_topic_last_timestamps))
                    .await
                    .map_err(|e| match e {
                        CertifPollServerError::Stopped => CertifSetupShamirRecoveryError::Stopped,
                        CertifPollServerError::Offline(e) => {
                            CertifSetupShamirRecoveryError::Offline(e)
                        }
                        CertifPollServerError::InvalidCertificate(err) => {
                            CertifSetupShamirRecoveryError::InvalidCertificate(err)
                        }
                        CertifPollServerError::Internal(err) => err
                            .context("Cannot poll server for new certificates")
                            .into(),
                    })?;

                // Must redo this check since the certificates have changed on local !
                recipients_with_share_count_and_pubkey =
                    check_against_local_certificates(ops, &per_recipient_shares).await?;

                recovery_setup_timestamp = ops.device.time_provider.now();
            }
        }
    }

    Ok(ShamirRecoverySetupCertificateTimestamps {
        common_certificate_timestamp: recovery_device_timestamp,
        shamir_recovery_certificate_timestamp: recovery_setup_timestamp,
    })
}

async fn check_against_local_certificates(
    ops: &CertificateOps,
    per_recipient_shares: &HashMap<UserID, NonZeroU8>,
) -> Result<Vec<(UserID, NonZeroU8, PublicKey)>, CertifSetupShamirRecoveryError> {
    let author_user_id = ops.device.user_id;
    ops.store
        .for_read({
            async |store| {
                // 1. Shamir already exists ?

                match store
                    .get_last_shamir_recovery_for_author(UpTo::Current, author_user_id)
                    .await?
                {
                    LastShamirRecovery::NeverSetup | LastShamirRecovery::Deleted(_, _) => (),
                    LastShamirRecovery::Valid(_) => {
                        return Err(CertifSetupShamirRecoveryError::ShamirRecoveryAlreadyExists);
                    }
                }

                // 2. Recipients exist ?

                let mut recipients = Vec::with_capacity(per_recipient_shares.len());
                for (&recipient_user_id, &recipient_share_count) in per_recipient_shares {
                    let recipient_pubkey = match store
                        .get_last_user_exist_and_revoked_info(UpTo::Current, recipient_user_id)
                        .await?
                    {
                        LastUserExistAndRevokedInfo::Valid(certif) => certif.public_key.clone(),
                        LastUserExistAndRevokedInfo::Revoked(_, _) => {
                            return Err(CertifSetupShamirRecoveryError::RecipientRevoked)
                        }
                        LastUserExistAndRevokedInfo::Unknown => {
                            return Err(CertifSetupShamirRecoveryError::RecipientNotFound)
                        }
                    };

                    recipients.push((recipient_user_id, recipient_share_count, recipient_pubkey));
                }

                Ok(recipients)
            }
        })
        .await?
}

#[derive(Debug)]
enum CreateShamirRecoveryDeviceOutcome {
    Done(Box<LocalDevice>),
    RequireGreaterTimestamp(DateTime),
}

async fn create_shamir_recovery_device(
    ops: &CertificateOps,
    timestamp: DateTime,
) -> Result<CreateShamirRecoveryDeviceOutcome, CertifSetupShamirRecoveryError> {
    let author = &ops.device;

    let recovery_device = LocalDevice::from_existing_device_for_user(
        &author.clone(),
        DeviceLabel::try_from(format!("shamir-recovery-{timestamp}").as_str())
            .expect("Invalid device label"),
    );

    let device_cert = DeviceCertificate {
        author: CertificateSigner::User(author.device_id),
        timestamp,
        purpose: DevicePurpose::ShamirRecovery,
        user_id: recovery_device.user_id,
        device_id: recovery_device.device_id,
        device_label: MaybeRedacted::Real(recovery_device.device_label.clone()),
        verify_key: recovery_device.verify_key(),
        algorithm: SigningKeyAlgorithm::Ed25519,
    };

    let device_certificate = device_cert.dump_and_sign(&author.signing_key).into();

    let redacted_device_cert = device_cert.into_redacted();

    let redacted_device_certificate = redacted_device_cert
        .dump_and_sign(&author.signing_key)
        .into();

    use authenticated_cmds::latest::device_create::{Rep, Req};

    match ops
        .cmds
        .send(Req {
            device_certificate,
            redacted_device_certificate,
        })
        .await?
    {
        Rep::Ok => Ok(CreateShamirRecoveryDeviceOutcome::Done(Box::new(
            recovery_device,
        ))),
        Rep::RequireGreaterTimestamp {
            strictly_greater_than,
        } =>
        // The retry is handled by the caller
        {
            Ok(CreateShamirRecoveryDeviceOutcome::RequireGreaterTimestamp(
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

            Err(CertifSetupShamirRecoveryError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            })
        }
        bad_rep @ (Rep::UnknownStatus { .. }
        | Rep::InvalidCertificate
        | Rep::DeviceAlreadyExists) => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}

#[derive(Debug)]
enum DoShamirRecoverySetupOutcome {
    Done,
    RequireGreaterTimestamp(DateTime),
    RequirePollingCertificates(PerTopicLastTimestamps),
}

async fn do_shamir_recovery_setup(
    ops: &CertificateOps,
    recovery_device: &LocalDevice,
    recipients_with_share_count_and_pubkey: &Vec<(UserID, NonZeroU8, PublicKey)>,
    total_share_count: NonZeroU8,
    threshold: NonZeroU8,
    timestamp: DateTime,
) -> Result<DoShamirRecoverySetupOutcome, CertifSetupShamirRecoveryError> {
    let author_device_id = ops.device.device_id;
    let author_user_id = ops.device.user_id;

    // 1. Generate certificates

    let shamir_recovery_brief_certificate: Bytes = ShamirRecoveryBriefCertificate {
        author: author_device_id,
        timestamp,
        user_id: author_user_id,
        threshold,
        per_recipient_shares: HashMap::from_iter(
            recipients_with_share_count_and_pubkey
                .iter()
                .map(|(user_id, share_count, _)| (*user_id, *share_count)),
        ),
    }
    .dump_and_sign(&ops.device.signing_key)
    .into();

    let data_key = SecretKey::generate();
    let reveal_token = AccessToken::default();

    let ciphered_data = data_key.encrypt(&recovery_device.dump()).into();

    let mut shark_shares = ShamirRecoverySecret {
        data_key,
        reveal_token,
    }
    .dump_and_encrypt_into_shares(threshold, total_share_count)
    .into_iter();

    let mut shamir_recovery_share_certificates =
        Vec::with_capacity(recipients_with_share_count_and_pubkey.len());
    for (recipient_user_id, recipient_share_count, recipient_pubkey) in
        recipients_with_share_count_and_pubkey
    {
        let weighted_share: Vec<_> = (0..recipient_share_count.get() as usize)
            .map(|_| shark_shares.next().expect("enough share generated"))
            .collect();

        let ciphered_share = ShamirRecoveryShareData {
            author: author_device_id,
            timestamp,
            weighted_share,
        }
        .dump_sign_and_encrypt_for(&ops.device.signing_key, recipient_pubkey);

        let shamir_recovery_share_certificate = ShamirRecoveryShareCertificate {
            author: author_device_id,
            timestamp,
            user_id: author_user_id,
            recipient: *recipient_user_id,
            ciphered_share,
        }
        .dump_and_sign(&ops.device.signing_key)
        .into();
        shamir_recovery_share_certificates.push(shamir_recovery_share_certificate);
    }

    assert!(shark_shares.next().is_none()); // Sanity check

    // 2. Send certificates

    use authenticated_cmds::latest::shamir_recovery_setup::{Rep, Req};

    let req = Req {
        ciphered_data,
        reveal_token,
        shamir_recovery_brief_certificate,
        shamir_recovery_share_certificates,
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

            Err(CertifSetupShamirRecoveryError::TimestampOutOfBallpark {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            })
        }
        // If the server complains the shamir already exists or a recipient is
        // revoked, it means we are missing certificates in local (otherwise
        // the certificates ops would have informed us in the first place !).
        //
        // Hence we must fetch the missing certificates before returning an error
        // (to avoid weird state where we get an error about a revoked user, but
        // see it has valid when asking the certificates ops...)
        //
        // Finally note that the operation will be retried once the caller is done
        // with polling the server. This is okay since in practice the operation
        // will errors out (with the same error we got here) early on when querying
        // the now up-to-date certificates ops (so no new request is sent to the server).
        Rep::ShamirRecoveryAlreadyExists {
            last_shamir_certificate_timestamp,
        } => Ok(DoShamirRecoverySetupOutcome::RequirePollingCertificates(
            PerTopicLastTimestamps::new_for_shamir(last_shamir_certificate_timestamp),
        )),
        Rep::RevokedRecipient {
            last_common_certificate_timestamp,
        } => Ok(DoShamirRecoverySetupOutcome::RequirePollingCertificates(
            PerTopicLastTimestamps::new_for_common(last_common_certificate_timestamp),
        )),
        // Note this error should never occur in practice given we have already
        // retrieve the user on our side.
        Rep::RecipientNotFound => Err(CertifSetupShamirRecoveryError::RecipientNotFound),
        bad_rep @ (Rep::InvalidCertificateBriefCorrupted
        | Rep::InvalidCertificateShareCorrupted
        | Rep::InvalidCertificateShareRecipientNotInBrief
        | Rep::InvalidCertificateDuplicateShareForRecipient
        | Rep::InvalidCertificateAuthorIncludedAsRecipient
        | Rep::InvalidCertificateMissingShareForRecipient
        | Rep::InvalidCertificateShareInconsistentTimestamp
        | Rep::InvalidCertificateUserIdMustBeSelf
        | Rep::UnknownStatus { .. }) => {
            Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
        }
    }
}
