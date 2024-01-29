// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use super::{
    realm_keys_bundle::{KeyFromIndexError, LoadLastKeysBundleError},
    store::CertifStoreError,
    CertifOps, InvalidKeysBundleError, UpTo,
};

#[derive(Debug, thiserror::Error)]
pub enum InvalidEncryptedRealmNameError {
    #[error("Realm name certificate (in realm `{realm}`, create by `{author}` on {timestamp}) is corrupted: {error}")]
    Corrupted {
        realm: VlobID,
        author: DeviceID,
        timestamp: DateTime,
        key_index: IndexInt,
        error: Box<DataError>,
    },
    #[error("Realm name certificate (in realm `{realm}`, create by `{author}` on {timestamp}): encrypted by key index {key_index} which didn't exist at that time !")]
    NonExistentKeyIndex {
        realm: VlobID,
        author: DeviceID,
        timestamp: DateTime,
        key_index: IndexInt,
    },
    #[error("Realm name certificate (in realm `{realm}`, create by `{author}` on {timestamp}): encrypted by key index {key_index} which appears corrupted !")]
    CorruptedKey {
        realm: VlobID,
        author: DeviceID,
        timestamp: DateTime,
        key_index: IndexInt,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum CertifDecryptCurrentRealmNameError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Cannot reach the server")]
    Offline,
    #[error("Not allowed")]
    NotAllowed,
    #[error("Realm has no name certificate yet")]
    NoNameCertificate,
    #[error(transparent)]
    InvalidEncryptedRealmName(#[from] Box<InvalidEncryptedRealmNameError>),
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn decrypt_current_realm_name(
    ops: &CertifOps,
    realm_id: VlobID,
) -> Result<(EntryName, DateTime), CertifDecryptCurrentRealmNameError> {
    ops.store
        .for_read(|store| async move {
            // 1) Retrieve the realm name certificate

            let maybe_certif = store
                .get_realm_last_name_certificate(UpTo::Current, realm_id)
                .await?;
            let certif = match maybe_certif {
                None => return Err(CertifDecryptCurrentRealmNameError::NoNameCertificate),
                Some(certif) => certif,
            };

            // 2) Retrieve the corresponding realm key

            let realm_keys =
                super::realm_keys_bundle::load_last_realm_keys_bundle(ops, store, realm_id)
                    .await
                    .map_err(|e| match e {
                        LoadLastKeysBundleError::Offline => {
                            CertifDecryptCurrentRealmNameError::Offline
                        }
                        LoadLastKeysBundleError::NotAllowed => {
                            CertifDecryptCurrentRealmNameError::NotAllowed
                        }
                        LoadLastKeysBundleError::NoKey => {
                            CertifDecryptCurrentRealmNameError::InvalidEncryptedRealmName(Box::new(
                                InvalidEncryptedRealmNameError::NonExistentKeyIndex {
                                    realm: realm_id,
                                    author: certif.author.to_owned(),
                                    timestamp: certif.timestamp,
                                    key_index: certif.key_index,
                                },
                            ))
                        }
                        LoadLastKeysBundleError::InvalidKeysBundle(err) => {
                            CertifDecryptCurrentRealmNameError::InvalidKeysBundle(err)
                        }
                        LoadLastKeysBundleError::Internal(err) => err.into(),
                    })?;
            let key = match realm_keys.key_from_index(certif.key_index, certif.timestamp) {
                Ok(key) => key,
                Err(KeyFromIndexError::CorruptedKey) => {
                    return Err(
                        CertifDecryptCurrentRealmNameError::InvalidEncryptedRealmName(Box::new(
                            InvalidEncryptedRealmNameError::CorruptedKey {
                                realm: realm_id,
                                author: certif.author.to_owned(),
                                timestamp: certif.timestamp,
                                key_index: certif.key_index,
                            },
                        )),
                    )
                }
                Err(KeyFromIndexError::KeyNotFound) => {
                    return Err(
                        CertifDecryptCurrentRealmNameError::InvalidEncryptedRealmName(Box::new(
                            InvalidEncryptedRealmNameError::NonExistentKeyIndex {
                                realm: realm_id,
                                author: certif.author.to_owned(),
                                timestamp: certif.timestamp,
                                key_index: certif.key_index,
                            },
                        )),
                    )
                }
            };

            // 3) Finally decrypt & parse the realm name

            let decrypted = match key.decrypt(&certif.encrypted_name) {
                Ok(decrypted) => decrypted,
                Err(_) => {
                    return Err(
                        CertifDecryptCurrentRealmNameError::InvalidEncryptedRealmName(Box::new(
                            InvalidEncryptedRealmNameError::Corrupted {
                                realm: realm_id,
                                author: certif.author.to_owned(),
                                timestamp: certif.timestamp,
                                key_index: certif.key_index,
                                error: Box::new(DataError::Decryption),
                            },
                        )),
                    )
                }
            };
            let name = match std::str::from_utf8(&decrypted)
                .ok()
                .and_then(|decrypted_str| decrypted_str.parse().ok())
            {
                Some(name) => name,
                None => {
                    return Err(
                        CertifDecryptCurrentRealmNameError::InvalidEncryptedRealmName(Box::new(
                            InvalidEncryptedRealmNameError::Corrupted {
                                realm: realm_id,
                                author: certif.author.to_owned(),
                                timestamp: certif.timestamp,
                                key_index: certif.key_index,
                                error: Box::new(DataError::Serialization),
                            },
                        )),
                    )
                }
            };

            Ok((name, certif.timestamp))
        })
        .await
        .map_err(|e| match e {
            CertifStoreError::Stopped => CertifDecryptCurrentRealmNameError::Stopped,
            CertifStoreError::Internal(err) => err.into(),
        })?
}
