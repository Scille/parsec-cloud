// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
use libparsec_protocol::authenticated_cmds;
use libparsec_types::prelude::*;

use super::{
    store::{CertifStoreError, CertificatesStoreWriteGuard},
    CertifAddCertificatesBatchError, CertifOps, InvalidCertificateError, MaybeRedactedSwitch,
};

#[derive(Debug, thiserror::Error)]
pub enum CertifPollServerError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot reach the server")]
    Offline,
    #[error("A certificate provided by the server is invalid: {0}")]
    InvalidCertificate(#[from] InvalidCertificateError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for CertifPollServerError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            // TODO: handle organization expired and user revoked here ?
            err => Self::Internal(err.into()),
        }
    }
}

impl From<CertifAddCertificatesBatchError> for CertifPollServerError {
    fn from(value: CertifAddCertificatesBatchError) -> Self {
        match value {
            CertifAddCertificatesBatchError::InvalidCertificate(err) => err.into(),
            CertifAddCertificatesBatchError::Stopped => CertifPollServerError::Stopped,
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
/// If `requirements` is specified, no poll will be done if the current certificates
/// on the local storage already satisfy the requirements.
///
/// Note the write lock on the store as parameter: this is required so that certificates
/// fetch and add to storage are an atomic operation for the storage. This is important
/// to avoid concurrency access changing certificates and breaking the deterministic
/// order certificates must be added on.
pub(super) async fn poll_server_for_new_certificates(
    ops: &CertifOps,
    store: &mut CertificatesStoreWriteGuard<'_>,
    requirements: Option<&PerTopicLastTimestamps>,
) -> Result<(), CertifPollServerError> {
    loop {
        let last_stored_timestamps = store.get_last_timestamps().await?;
        // `requirements` is useful to detect outdated `CertificatesUpdated`
        // events given the server has already been polled in the meantime.
        if let Some(requirements) = requirements {
            if last_stored_timestamps.is_up_to_date(requirements) {
                return Ok(());
            }
        }

        let outcome = poll_server_and_add_certificates(ops, store, last_stored_timestamps).await?;

        match outcome {
            MaybeRedactedSwitch::NoSwitch => return Ok(()),
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

async fn poll_server_and_add_certificates(
    ops: &CertifOps,
    store: &mut CertificatesStoreWriteGuard<'_>,
    last_stored_timestamps: PerTopicLastTimestamps,
) -> Result<MaybeRedactedSwitch, CertifPollServerError> {
    // 1) Fetch certificates

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

    // 2) Integrate the new certificates.

    let outcome = super::add::add_certificates_batch(
        ops,
        store,
        &common_certificates,
        &sequester_certificates,
        &shamir_recovery_certificates,
        &realm_certificates,
    )
    .await
    .map_err(|err| match err {
        CertifAddCertificatesBatchError::InvalidCertificate(err) => {
            CertifPollServerError::InvalidCertificate(err)
        }
        CertifAddCertificatesBatchError::Stopped => CertifPollServerError::Stopped,
        CertifAddCertificatesBatchError::Internal(err) => CertifPollServerError::Internal(err),
    })?;

    Ok(outcome)
}
