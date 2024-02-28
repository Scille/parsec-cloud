// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
use libparsec_types::prelude::*;

use crate::certif::{CertifPollServerError, CertificateBasedActionOutcome};

pub use crate::certif::CertifRevokeUserError as ClientRevokeUserError;

use super::Client;

pub async fn revoke_user(client_ops: &Client, user: UserID) -> Result<(), ClientRevokeUserError> {
    let outcome = client_ops.certificates_ops.revoke_user(user).await?;

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
            CertifPollServerError::Stopped => ClientRevokeUserError::Stopped,
            CertifPollServerError::Offline => ClientRevokeUserError::Offline,
            CertifPollServerError::InvalidCertificate(err) => {
                ClientRevokeUserError::InvalidCertificate(err)
            }
            CertifPollServerError::Internal(err) => err
                .context("Cannot poll server for new certificates")
                .into(),
        })
}
