// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::{
    CertifGetRealmKeyForDecryption, CertifPollServerError, EncryptionUsage, InvalidCertificateError,
};

use super::{CertificateOps, InvalidKeysBundleError, realm_keys_bundle, store::CertifStoreError};

#[derive(Debug, thiserror::Error)]
pub enum CertifDecryptOpaqueDataForRealmError {
    /// Stopped is not used by `encrypt_for_realm`, but is convenient anyways given
    /// it is needed by the wrapper `CertificateOps::encrypt_for_realm`.
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Not allowed to access this realm")]
    NotAllowed,
    #[error("The workspace's realm has been deleted on the server")]
    RealmDeleted,
    #[error("The referenced key doesn't exist yet in this realm")]
    KeyNotFound,
    #[error("The referenced key appears to be corrupted !")]
    CorruptedKey,
    #[error("Cannot decrypt data")]
    CorruptedData,
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn decrypt_opaque_data_for_realm(
    ops: &CertificateOps,
    usage: EncryptionUsage,
    realm_id: VlobID,
    key_index: IndexInt,
    encrypted: &[u8],
) -> Result<Vec<u8>, CertifDecryptOpaqueDataForRealmError> {
    let mut cleartexts =
        decrypt_opaque_data_batch_for_realm(ops, usage, realm_id, key_index, &[encrypted]).await?;
    debug_assert_eq!(cleartexts.len(), 1);
    Ok(cleartexts.pop().unwrap_or_default())
}

pub(super) async fn decrypt_opaque_data_batch_for_realm(
    ops: &CertificateOps,
    usage: EncryptionUsage,
    realm_id: VlobID,
    key_index: IndexInt,
    encrypted_items: &[&[u8]],
) -> Result<Vec<Vec<u8>>, CertifDecryptOpaqueDataForRealmError> {
    let mut key_outcome = ops
        .store
        .for_read(async |store| {
            realm_keys_bundle::get_realm_key_for_decryption(ops, store, usage, realm_id, key_index)
                .await
        })
        .await
        .map_err(|e| match e {
            CertifStoreError::Stopped => CertifDecryptOpaqueDataForRealmError::Stopped,
            CertifStoreError::Internal(err) => err.context("cannot access storage").into(),
        })?;

    // If the current keys bundle doesn't contains the specified key index, we have
    // no choice but to poll the server & retry in case we were actually lagging
    // behind.

    if matches!(
        key_outcome,
        Err(CertifGetRealmKeyForDecryption::KeyNotFound)
    ) {
        ops.poll_server_for_new_certificates(None)
            .await
            .map_err(|err| match err {
                CertifPollServerError::Stopped => CertifDecryptOpaqueDataForRealmError::Stopped,
                CertifPollServerError::Offline(e) => {
                    CertifDecryptOpaqueDataForRealmError::Offline(e)
                }
                CertifPollServerError::InvalidCertificate(err) => {
                    CertifDecryptOpaqueDataForRealmError::InvalidCertificate(err)
                }
                CertifPollServerError::Internal(err) => err.context("cannot poll server").into(),
            })?;

        key_outcome = ops
            .store
            .for_read(async |store| {
                realm_keys_bundle::get_realm_key_for_decryption(
                    ops, store, usage, realm_id, key_index,
                )
                .await
            })
            .await
            .map_err(|e| match e {
                CertifStoreError::Stopped => CertifDecryptOpaqueDataForRealmError::Stopped,
                CertifStoreError::Internal(err) => err.context("cannot access storage").into(),
            })?;
    }

    let key = key_outcome.map_err(|err| match err {
        CertifGetRealmKeyForDecryption::Stopped => CertifDecryptOpaqueDataForRealmError::Stopped,
        CertifGetRealmKeyForDecryption::Offline(err) => {
            CertifDecryptOpaqueDataForRealmError::Offline(err)
        }
        CertifGetRealmKeyForDecryption::NotAllowed => {
            CertifDecryptOpaqueDataForRealmError::NotAllowed
        }
        CertifGetRealmKeyForDecryption::RealmDeleted => {
            CertifDecryptOpaqueDataForRealmError::RealmDeleted
        }
        CertifGetRealmKeyForDecryption::KeyNotFound => {
            CertifDecryptOpaqueDataForRealmError::KeyNotFound
        }
        CertifGetRealmKeyForDecryption::CorruptedKey => {
            CertifDecryptOpaqueDataForRealmError::CorruptedKey
        }
        CertifGetRealmKeyForDecryption::InvalidKeysBundle(err) => {
            CertifDecryptOpaqueDataForRealmError::InvalidKeysBundle(err)
        }
        CertifGetRealmKeyForDecryption::Internal(err) => err.context("cannot get realm key").into(),
    })?;

    let mut cleartexts = Vec::with_capacity(encrypted_items.len());
    for encrypted in encrypted_items {
        cleartexts.push(
            key.decrypt(encrypted)
                .map_err(|_| CertifDecryptOpaqueDataForRealmError::CorruptedData)?,
        );
    }

    Ok(cleartexts)
}
