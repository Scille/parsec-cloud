// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, num::NonZeroU8};

pub use crate::certif::CertifSetupShamirRecoveryError as ClientSetupShamirRecoveryError;
use crate::CertifPollServerError;
use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
use libparsec_types::UserID;

use super::Client;

pub async fn setup_shamir_recovery(
    client_ops: &Client,
    per_recipient_shares: HashMap<UserID, NonZeroU8>,
    threshold: NonZeroU8,
) -> Result<(), ClientSetupShamirRecoveryError> {
    let certificate_timestamps = client_ops
        .certificates_ops
        .setup_shamir_recovery(per_recipient_shares, threshold)
        .await?;

    let per_topic_last_timestamps = PerTopicLastTimestamps::new_for_common_and_shamir(
        certificate_timestamps.common_certificate_timestamp,
        certificate_timestamps.shamir_recovery_certificate_timestamp,
    );
    client_ops
        .certificates_ops
        .poll_server_for_new_certificates(Some(&per_topic_last_timestamps))
        .await
        .map_err(|e| match e {
            CertifPollServerError::Stopped => ClientSetupShamirRecoveryError::Stopped,
            CertifPollServerError::Offline(e) => ClientSetupShamirRecoveryError::Offline(e),
            CertifPollServerError::InvalidCertificate(err) => {
                ClientSetupShamirRecoveryError::InvalidCertificate(err)
            }
            CertifPollServerError::Internal(err) => err
                .context("Cannot poll server for new certificates")
                .into(),
        })
}
