// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
use libparsec_types::prelude::*;

use crate::certif::{CertifPollServerError, CertificateBasedActionOutcome};

pub use crate::certif::CertifUpdateUserProfileError as ClientUserUpdateProfileError;

use super::Client;

pub async fn update_profile(
    client_ops: &Client,
    user_id: UserID,
    new_profile: UserProfile,
) -> Result<(), ClientUserUpdateProfileError> {
    let outcome = client_ops
        .certificates_ops
        .user_update_profile(user_id, new_profile)
        .await?;

    let latest_known_timestamps = match outcome {
        CertificateBasedActionOutcome::LocalIdempotent => return Ok(()),
        CertificateBasedActionOutcome::Uploaded {
            certificate_timestamp,
        }
        | CertificateBasedActionOutcome::RemoteIdempotent {
            certificate_timestamp,
        } => PerTopicLastTimestamps::new_for_common(certificate_timestamp),
    };
    client_ops
        .certificates_ops
        .poll_server_for_new_certificates(Some(&latest_known_timestamps))
        .await
        .map_err(|e| match e {
            CertifPollServerError::Stopped => ClientUserUpdateProfileError::Stopped,
            CertifPollServerError::Offline => ClientUserUpdateProfileError::Offline,
            CertifPollServerError::InvalidCertificate(err) => {
                ClientUserUpdateProfileError::InvalidCertificate(err)
            }
            CertifPollServerError::Internal(err) => err
                .context("Cannot poll server for new certificates")
                .into(),
        })
}
