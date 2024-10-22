// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;

pub use crate::certif::CertifShamirError as ClientShamirError;
use crate::{CertifPollServerError, CertificateBasedActionOutcome};
use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
use libparsec_types::UserID;

use super::Client;

pub async fn shamir_setup_create(
    client_ops: &Client,
    share_recipients: HashMap<UserID, u8>,
    threshold: u8,
) -> Result<(), ClientShamirError> {
    let outcome = client_ops
        .certificates_ops
        .shamir_setup_create(share_recipients, threshold)
        .await?;

    let latest_known_timestamps = match outcome {
        CertificateBasedActionOutcome::LocalIdempotent => return Ok(()),
        CertificateBasedActionOutcome::Uploaded {
            certificate_timestamp,
        }
        | CertificateBasedActionOutcome::RemoteIdempotent {
            certificate_timestamp, // the timestamp here is the same for both device creation and shamir setup
        } => PerTopicLastTimestamps::new_for_common_and_shamir(
            certificate_timestamp,
            certificate_timestamp,
        ),
    };
    client_ops
        .certificates_ops
        .poll_server_for_new_certificates(Some(&latest_known_timestamps))
        .await
        .map_err(|e| match e {
            CertifPollServerError::Stopped => ClientShamirError::Stopped,
            CertifPollServerError::Offline => ClientShamirError::Offline,
            CertifPollServerError::InvalidCertificate(err) => {
                ClientShamirError::InvalidCertificate(err)
            }
            CertifPollServerError::Internal(err) => err
                .context("Cannot poll server for new certificates")
                .into(),
        })
}
