// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::{CertifDecryptForRealmError, EncrytionUsage};

use super::{
    realm_keys_bundle, store::CertifStoreError, CertificateOps, InvalidKeysBundleError, UpTo,
};

#[derive(Debug, thiserror::Error)]
pub enum InvalidEncryptedRealmNameError {
    #[error("Realm name certificate (in realm `{realm}`, create by `{author}` on {timestamp}): cannot be decrypted by key index {key_index} !")]
    CannotDecrypt {
        realm: VlobID,
        author: DeviceID,
        timestamp: DateTime,
        key_index: IndexInt,
    },
    #[error("Realm name certificate (in realm `{realm}`, create by `{author}` on {timestamp}) can be decrypted but its content is corrupted: {error}")]
    CleartextCorrupted {
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
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
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
    ops: &CertificateOps,
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

            // 2) Decrypt the realm name

            let author = certif.author;
            let key_index = certif.key_index;
            let timestamp = certif.timestamp;
            let encrypted = &certif.encrypted_name;

            let decrypted = realm_keys_bundle::decrypt_for_realm(
                ops,
                store,
                EncrytionUsage::RealmRename,
                realm_id,
                key_index,
                encrypted,
            )
            .await
            .map_err(|err| match err {
                CertifDecryptForRealmError::Stopped => CertifDecryptCurrentRealmNameError::Stopped,
                CertifDecryptForRealmError::Offline(e) => {
                    CertifDecryptCurrentRealmNameError::Offline(e)
                }
                CertifDecryptForRealmError::NotAllowed => {
                    CertifDecryptCurrentRealmNameError::NotAllowed
                }
                CertifDecryptForRealmError::KeyNotFound => {
                    CertifDecryptCurrentRealmNameError::InvalidEncryptedRealmName(Box::new(
                        InvalidEncryptedRealmNameError::NonExistentKeyIndex {
                            realm: realm_id,
                            author,
                            timestamp,
                            key_index,
                        },
                    ))
                }
                CertifDecryptForRealmError::CorruptedKey => {
                    CertifDecryptCurrentRealmNameError::InvalidEncryptedRealmName(Box::new(
                        InvalidEncryptedRealmNameError::CorruptedKey {
                            realm: realm_id,
                            author,
                            timestamp,
                            key_index,
                        },
                    ))
                }
                CertifDecryptForRealmError::CorruptedData => {
                    CertifDecryptCurrentRealmNameError::InvalidEncryptedRealmName(Box::new(
                        InvalidEncryptedRealmNameError::CannotDecrypt {
                            realm: realm_id,
                            author,
                            timestamp,
                            key_index,
                        },
                    ))
                }
                CertifDecryptForRealmError::InvalidKeysBundle(err) => {
                    CertifDecryptCurrentRealmNameError::InvalidKeysBundle(err)
                }
                CertifDecryptForRealmError::Internal(err) => err.into(),
                // Error not actually used by `decrypt_for_realm`
                CertifDecryptForRealmError::InvalidCertificate(_) => unreachable!(),
            })?;

            // 3) Finally parse the realm name

            let name = match std::str::from_utf8(&decrypted)
                .ok()
                .and_then(|decrypted_str| decrypted_str.parse().ok())
            {
                Some(name) => name,
                None => {
                    return Err(
                        CertifDecryptCurrentRealmNameError::InvalidEncryptedRealmName(Box::new(
                            InvalidEncryptedRealmNameError::CleartextCorrupted {
                                realm: realm_id,
                                author: certif.author.to_owned(),
                                timestamp: certif.timestamp,
                                key_index: certif.key_index,
                                error: Box::new(DataError::BadSerialization {
                                    format: None,
                                    step: "cleartext entry name deserialization",
                                }),
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
