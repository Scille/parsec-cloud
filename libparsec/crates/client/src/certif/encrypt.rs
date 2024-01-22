// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use super::{
    store::{CertificatesStoreReadGuard, GetCertificateError},
    UpTo,
};

#[derive(Debug, thiserror::Error)]
pub enum CertifEncryptForUserError {
    /// Stopped is not used by `encrypt_for_user`, but is convenient anyways given
    /// it is needed by the wrapper `CertificateOps::encrypt_for_user`.
    #[error("Component has stopped")]
    Stopped,
    #[error("User not found")]
    UserNotFound,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn encrypt_for_user<'a>(
    store: &mut CertificatesStoreReadGuard<'a>,
    user_id: UserID,
    data: &[u8],
) -> Result<Vec<u8>, CertifEncryptForUserError> {
    match store.get_user_certificate(UpTo::Current, user_id).await {
        Ok(recipient_certif) => {
            let encrypted = recipient_certif.public_key.encrypt_for_self(data);
            Ok(encrypted)
        }
        Err(GetCertificateError::NonExisting | GetCertificateError::ExistButTooRecent { .. }) => {
            Err(CertifEncryptForUserError::UserNotFound)
        }
        Err(GetCertificateError::Internal(err)) => Err(err.into()),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CertifEncryptForSequesterServicesError {
    /// Stopped is not used by `encrypt_for_sequester_services`, but is convenient anyways given
    /// it is needed by the wrapper `CertificateOps::encrypt_for_sequester_services`.
    #[error("Component has stopped")]
    Stopped,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn encrypt_for_sequester_services<'a>(
    store: &mut CertificatesStoreReadGuard<'a>,
    data: &[u8],
) -> Result<Option<Vec<(SequesterServiceID, Bytes)>>, CertifEncryptForSequesterServicesError> {
    let is_sequestered = store.get_sequester_authority_certificate().await?.is_some();
    if !is_sequestered {
        return Ok(None);
    }

    let services = store
        .get_sequester_service_certificates(UpTo::Current)
        .await?;
    let per_service_encrypted = services
        .into_iter()
        .map(|service| {
            (
                service.service_id,
                service.encryption_key_der.encrypt(data).into(),
            )
        })
        .collect();

    Ok(Some(per_service_encrypted))
}
