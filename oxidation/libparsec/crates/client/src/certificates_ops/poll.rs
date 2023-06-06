// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_platform_async::RwLockReadGuard;
use libparsec_protocol::authenticated_cmds;
use libparsec_types::prelude::*;

use super::{
    storage::CertificatesCachedStorage, AddCertificateError, CertificatesOps,
    InvalidCertificateError, MaybeRedactedSwitch,
};

#[derive(Debug, thiserror::Error)]
pub enum PollServerError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("A certificate provided by the server is invalid: {0}")]
    InvalidCertificate(#[from] InvalidCertificateError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for PollServerError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

impl From<AddCertificateError> for PollServerError {
    fn from(value: AddCertificateError) -> Self {
        match value {
            AddCertificateError::InvalidCertificate(err) => err.into(),
            AddCertificateError::Internal(err) => err.into(),
        }
    }
}

impl CertificatesOps {
    pub(super) async fn ensure_certificates_available_and_read_lock<'a>(
        &'a self,
        certificate_index: IndexInt,
    ) -> Result<RwLockReadGuard<'a, CertificatesCachedStorage>, PollServerError> {
        loop {
            self.poll_server_for_new_certificates(Some(certificate_index))
                .await?;
            let storage = self.storage.read().await;
            let last_index = self
                .storage
                .read()
                .await
                .get_last_certificate_index()
                .await?
                .unwrap_or(0);
            if last_index >= certificate_index {
                return Ok(storage);
            }
        }
    }

    pub async fn poll_server_for_new_certificates(
        &self,
        latest_known_index: Option<IndexInt>,
    ) -> Result<IndexInt, PollServerError> {
        loop {
            // 1) Retrieve the last certificate index when are currently aware of

            let last_index = self
                .storage
                .read()
                .await
                .get_last_certificate_index()
                .await?;

            // `latest_known_index` is useful to detect outdated `CertificatesUpdated`
            // events given the server has already been polled in the meantime.
            let offset = match (&last_index, latest_known_index) {
                (Some(last_index), Some(latest_known_index))
                    if *last_index >= latest_known_index =>
                {
                    return Ok(*last_index)
                }
                // Certificate index starts at 1, so can be used as-is as offset
                (None, _) => 0,
                (Some(last_index), _) => *last_index,
            };

            // 2) We are missing some certificates, time to ask the server about them...

            let request = authenticated_cmds::latest::certificate_get::Req { offset };
            let rep = self.cmds.send(request).await?;
            let certificates = match rep {
                authenticated_cmds::latest::certificate_get::Rep::Ok { certificates } => {
                    certificates
                }
                authenticated_cmds::latest::certificate_get::Rep::UnknownStatus {
                    unknown_status,
                    ..
                } => {
                    return Err(anyhow::anyhow!(
                        "Unknown error status `{}` from server",
                        unknown_status
                    )
                    .into());
                }
            };

            // 3) Integrate the new certificates. The lock must be help for the whole
            // time here to avoid concurrency access changing certificate and breaking
            // the deterministic order certificate must be added on.

            let mut storage = self.storage.write().await;

            let new_offset = storage.get_last_certificate_index().await?.unwrap_or(0);
            let final_index = offset + certificates.len() as IndexInt;

            let certificates = match offset.cmp(&new_offset) {
                // Certificates hasn't changed while we were requesting the server
                std::cmp::Ordering::Equal => certificates.into_iter().skip(0),
                // New certificates has been added while we were requesting the server
                std::cmp::Ordering::Less => {
                    let to_skip = (new_offset - offset) as usize;
                    certificates.into_iter().skip(to_skip)
                }
                // Certificates has changed, now we have less certificates than we used to o_O
                // This special case may occur if the certificate storage got cleared (see below
                // `MaybeRedactodSwitch`), in this case we just redo everything from the start.
                std::cmp::Ordering::Greater => continue,
            };

            let outcome = self
                .add_certificates_batch(&mut storage, new_offset, certificates)
                .await?;
            match outcome {
                MaybeRedactedSwitch::NoSwitch => (),
                // Unlike other profiles, Outsider is required to use the redacted
                // certificates, hence our local certificate has been flushed and we
                // must go back to the server to get the all certificates from scratch.
                MaybeRedactedSwitch::Switched => continue,
            }

            return Ok(final_index);
        }
    }
}
