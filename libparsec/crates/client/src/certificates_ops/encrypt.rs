// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;

use libparsec_types::prelude::*;

use super::{CertificatesOps, GetCertificateError, UpTo};
use crate::certificates_ops::store::CertificatesStoreReadExt;

pub(super) async fn encrypt_for_user(
    ops: &CertificatesOps,
    user_id: UserID,
    data: &[u8],
) -> anyhow::Result<Option<Vec<u8>>> {
    let store = ops.store.for_read().await;
    let recipient_certif = match store.get_user_certificate(UpTo::Current, user_id).await {
        Ok(certif) => certif,
        Err(GetCertificateError::NonExisting | GetCertificateError::ExistButTooRecent { .. }) => {
            return Ok(None)
        }
        Err(GetCertificateError::Internal(err)) => return Err(err),
    };

    let encrypted = recipient_certif.public_key.encrypt_for_self(data);

    Ok(Some(encrypted))
}

pub(super) async fn encrypt_for_sequester_services(
    ops: &CertificatesOps,
    data: &[u8],
) -> anyhow::Result<Option<HashMap<SequesterServiceID, Bytes>>> {
    let store = ops.store.for_read().await;
    match store
        .get_sequester_authority_certificate(UpTo::Current)
        .await
    {
        Ok(_) => (),
        // The organization is not sequestered (and `ExistButTooRecent` should never occur !)
        Err(GetCertificateError::NonExisting) => return Ok(None),
        Err(GetCertificateError::ExistButTooRecent { .. }) => unreachable!(),
        Err(GetCertificateError::Internal(err)) => return Err(err),
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
