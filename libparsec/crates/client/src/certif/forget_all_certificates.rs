// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use super::{CertificateOps, store::CertifStoreError};

#[derive(Debug, thiserror::Error)]
pub enum CertifForgetAllCertificatesError {
    #[error("Component has stopped")]
    Stopped,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn forget_all_certificates(
    ops: &CertificateOps,
) -> Result<(), CertifForgetAllCertificatesError> {
    let _guard = ops.update_lock.lock().await;

    ops.store
        .for_write(async |store| store.forget_all_certificates().await)
        .await
        .map_err(|err| match err {
            CertifStoreError::Stopped => CertifForgetAllCertificatesError::Stopped,
            CertifStoreError::Internal(err) => err.into(),
        })??;

    Ok(())
}
