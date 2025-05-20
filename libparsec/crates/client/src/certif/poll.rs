// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;

use libparsec_client_connection::ConnectionError;
use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
use libparsec_protocol::authenticated_cmds;
use libparsec_types::prelude::*;

use super::{
    CertifAddCertificatesBatchError, CertificateOps, InvalidCertificateError, MaybeRedactedSwitch,
    store::CertifStoreError,
};

#[derive(Debug, thiserror::Error)]
pub enum CertifPollServerError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("A certificate provided by the server is invalid: {0}")]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<CertifAddCertificatesBatchError> for CertifPollServerError {
    fn from(value: CertifAddCertificatesBatchError) -> Self {
        match value {
            CertifAddCertificatesBatchError::InvalidCertificate(err) => err.into(),
            CertifAddCertificatesBatchError::Internal(err) => err.into(),
        }
    }
}

impl From<CertifStoreError> for CertifPollServerError {
    fn from(value: CertifStoreError) -> Self {
        match value {
            CertifStoreError::Stopped => Self::Stopped,
            CertifStoreError::Internal(err) => err.into(),
        }
    }
}

/// Poll the server for new certificates and add them to the local storage.
///
/// Returns the number of new certificates added to the storage.
///
/// If `requirements` is specified, no poll will be done if the current certificates
/// on the local storage already satisfy the requirements.
///
/// Note the write lock on the store as parameter: this is required so that certificates
/// fetch and add to storage are an atomic operation for the storage. This is important
/// to avoid concurrency access changing certificates and breaking the deterministic
/// order certificates must be added on.
pub(super) async fn poll_server_for_new_certificates(
    ops: &CertificateOps,
    requirements: Option<&PerTopicLastTimestamps>,
) -> Result<usize, CertifPollServerError> {
    let _guard = ops.update_lock.lock().await;

    loop {
        let last_stored_timestamps = ops
            .store
            .for_read(async |store| store.get_last_timestamps().await)
            .await??;
        // `requirements` is useful to detect outdated `CertificatesUpdated`
        // events given the server has already been polled in the meantime.
        if let Some(requirements) = requirements {
            if last_stored_timestamps.is_up_to_date(requirements) {
                return Ok(0);
            }
        }

        let new_certificates = poll_server(ops, last_stored_timestamps).await?;

        let outcome = ops
            .store
            .for_write(async |store| {
                super::add::add_certificates_batch(
                    ops,
                    store,
                    &new_certificates.common_certificates,
                    &new_certificates.sequester_certificates,
                    &new_certificates.shamir_recovery_certificates,
                    &new_certificates.realm_certificates,
                )
                .await
                .map_err(|err| match err {
                    CertifAddCertificatesBatchError::InvalidCertificate(err) => {
                        CertifPollServerError::InvalidCertificate(err)
                    }
                    CertifAddCertificatesBatchError::Internal(err) => {
                        CertifPollServerError::Internal(err)
                    }
                })
            })
            .await??;

        match outcome {
            MaybeRedactedSwitch::NoSwitch { new_certificates } => return Ok(new_certificates),
            // Unlike other profiles, Outsider is required to use the redacted
            // certificates, hence our local certificates have been flushed and we
            // must go back to the server to get the all certificates from scratch.
            //
            // Note we don't release the store write lock before retrying: this way
            // no concurrent operation can see the store while it is empty (and,
            // if we crash, the store will be rolled back to before the clear).
            // This allows us to consider the certificates are never removed while
            // we are doing an operation (e.g. A creates a certificate needed by B,
            // we can do A then B without keeping the store lock).
            MaybeRedactedSwitch::Switched => continue,
        }
    }
}

struct NewCertificates {
    common_certificates: Vec<Bytes>,
    realm_certificates: HashMap<VlobID, Vec<Bytes>>,
    sequester_certificates: Vec<Bytes>,
    shamir_recovery_certificates: Vec<Bytes>,
}

async fn poll_server(
    ops: &CertificateOps,
    last_stored_timestamps: PerTopicLastTimestamps,
) -> Result<NewCertificates, CertifPollServerError> {
    let request = authenticated_cmds::latest::certificate_get::Req {
        common_after: last_stored_timestamps.common,
        sequester_after: last_stored_timestamps.sequester,
        shamir_recovery_after: last_stored_timestamps.shamir_recovery,
        realm_after: last_stored_timestamps.realm,
    };
    let rep = ops.cmds.send(request).await?;
    let (
        common_certificates,
        realm_certificates,
        sequester_certificates,
        shamir_recovery_certificates,
    ) = match rep {
        authenticated_cmds::latest::certificate_get::Rep::Ok {
            common_certificates,
            realm_certificates,
            sequester_certificates,
            shamir_recovery_certificates,
        } => (
            common_certificates,
            realm_certificates,
            sequester_certificates,
            shamir_recovery_certificates,
        ),
        authenticated_cmds::latest::certificate_get::Rep::UnknownStatus {
            unknown_status, ..
        } => {
            return Err(CertifPollServerError::Internal(anyhow::anyhow!(
                "Unknown error status `{}` from server",
                unknown_status
            )));
        }
    };

    Ok(NewCertificates {
        common_certificates,
        realm_certificates,
        sequester_certificates,
        shamir_recovery_certificates,
    })
}
