// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
use libparsec_types::prelude::*;

use crate::{
    certif::realm_keys_bundle::{self, EncryptionUsage},
    CertifDecryptForRealmError,
};

use super::{
    store::{CertifForReadWithRequirementsError, CertificatesStoreReadGuard, GetCertificateError},
    CertificateOps, InvalidCertificateError, InvalidKeysBundleError, UpTo,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CryptpadSessionKeys {
    pub view_key: String,
    pub edit_key: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum InvalidCryptpadSessionKeysError {
    #[error("Cryptpad session key for document `{document}` (in realm `{realm}`, created by `{author}` on {timestamp}): cannot be decrypted by key index {key_index} !")]
    CannotDecrypt {
        realm: VlobID,
        document: VlobID,
        author: DeviceID,
        timestamp: DateTime,
        key_index: IndexInt,
    },
    #[error("Cryptpad session key for document `{document}` (in realm `{realm}`, created by `{author}` on {timestamp}) can be decrypted but its content is corrupted: {error}")]
    CleartextCorrupted {
        realm: VlobID,
        document: VlobID,
        author: DeviceID,
        timestamp: DateTime,
        error: Box<DataError>,
    },
    #[error("Cryptpad session key for document `{document}` (in realm `{realm}`, created by `{author}` on {timestamp}): at that time, key index {key_index} didn't exist !")]
    NonExistentKeyIndex {
        realm: VlobID,
        document: VlobID,
        author: DeviceID,
        timestamp: DateTime,
        key_index: IndexInt,
    },
    #[error("Cryptpad session key for document `{document}` (in realm `{realm}`, created by `{author}` on {timestamp}): encrypted by key index {key_index} which appears corrupted !")]
    CorruptedKey {
        realm: VlobID,
        document: VlobID,
        author: DeviceID,
        timestamp: DateTime,
        key_index: IndexInt,
    },
    #[error("Cryptpad session key for document `{document}` (in realm `{realm}`, created by `{author}` on {timestamp}): at that time author didn't exist !")]
    NonExistentAuthor {
        realm: VlobID,
        document: VlobID,
        author: DeviceID,
        timestamp: DateTime,
    },
    #[error("Cryptpad session key for document `{document}` (in realm `{realm}`, created by `{author}` on {timestamp}): at that time author was already revoked !")]
    RevokedAuthor {
        realm: VlobID,
        document: VlobID,
        author: DeviceID,
        timestamp: DateTime,
    },
    #[error("Cryptpad session key for document `{document}` (in realm `{realm}`, created by `{author}` on {timestamp}): at that time author didn't have access to the realm")]
    AuthorNoAccessToRealm {
        realm: VlobID,
        document: VlobID,
        author: DeviceID,
        timestamp: DateTime,
    },
    #[error("Cryptpad session key for document `{document}` (in realm `{realm}`, created by `{author}` on {timestamp}): edit key provided, but at that time author couldn't write in the realm given its role was `{author_role:?}`")]
    AuthorRealmRoleCannotWrite {
        realm: VlobID,
        document: VlobID,
        author: DeviceID,
        timestamp: DateTime,
        author_role: RealmRole,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum CertifValidateCryptpadSessionKeysError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Component has stopped")]
    Stopped,
    #[error("Not allowed to access this realm")]
    NotAllowed,
    #[error("The workspace's realm has been deleted on the server")]
    RealmDeleted,
    #[error(transparent)]
    InvalidCryptpadSessionKeys(#[from] Box<InvalidCryptpadSessionKeysError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[expect(clippy::too_many_arguments)]
pub(super) async fn validate_cryptpad_session_keys(
    ops: &CertificateOps,
    needed_realm_certificate_timestamp: DateTime,
    needed_common_certificate_timestamp: DateTime,
    realm_id: VlobID,
    key_index: IndexInt,
    document_id: VlobID,
    author: DeviceID,
    timestamp: DateTime,
    encrypted_view_key: &[u8],
    encrypted_edit_key: Option<&[u8]>,
) -> Result<CryptpadSessionKeys, CertifValidateCryptpadSessionKeysError> {
    let needed_timestamps = PerTopicLastTimestamps::new_for_common_and_realm(
        needed_common_certificate_timestamp,
        realm_id,
        needed_realm_certificate_timestamp,
    );

    ops.store
        .for_read_with_requirements(ops, &needed_timestamps, async |store| {
            // 1) Decrypt the keys

            let view_cleartext = decrypt_session_key(
                ops,
                store,
                realm_id,
                key_index,
                document_id,
                author,
                timestamp,
                encrypted_view_key,
            )
            .await?;

            let edit_cleartext = match encrypted_edit_key {
                None => None,
                Some(encrypted_edit_key) => Some(
                    decrypt_session_key(
                        ops,
                        store,
                        realm_id,
                        key_index,
                        document_id,
                        author,
                        timestamp,
                        encrypted_edit_key,
                    )
                    .await?,
                ),
            };

            // 2) Author must exist and not be revoked at the time the keys were created !

            // 2.1) Check device exists (this also imply the user exists)

            let author_certif = match store
                .get_device_certificate(UpTo::Timestamp(timestamp), author.to_owned())
                .await
            {
                // Exists, as expected :)
                Ok(certif) => certif,

                // Doesn't exist at the considered timestamp :(
                Err(
                    GetCertificateError::NonExisting
                    | GetCertificateError::ExistButTooRecent { .. },
                ) => {
                    let what = Box::new(InvalidCryptpadSessionKeysError::NonExistentAuthor {
                        realm: realm_id,
                        document: document_id,
                        author: author.to_owned(),
                        timestamp,
                    });
                    return Err(
                        CertifValidateCryptpadSessionKeysError::InvalidCryptpadSessionKeys(what),
                    );
                }

                // D'oh :/
                Err(err @ GetCertificateError::Internal(_)) => {
                    return Err(CertifValidateCryptpadSessionKeysError::Internal(err.into()))
                }
            };
            let author_user_id = author_certif.user_id;

            // 2.2) Check author is not revoked

            match store
                .get_revoked_user_certificate(UpTo::Timestamp(timestamp), author_user_id)
                .await
            {
                // Not revoked at the considered timestamp, as we expected :)
                Ok(None) => (),

                // Revoked :(
                Ok(Some(_)) => {
                    let what = Box::new(InvalidCryptpadSessionKeysError::RevokedAuthor {
                        realm: realm_id,
                        document: document_id,
                        author: author.to_owned(),
                        timestamp,
                    });
                    return Err(
                        CertifValidateCryptpadSessionKeysError::InvalidCryptpadSessionKeys(what),
                    );
                }

                // D'oh :/
                Err(err) => return Err(CertifValidateCryptpadSessionKeysError::Internal(err)),
            }

            // 3) Check the session is consistent with the system (i.e. the author had the right to create it)

            let author_role = store
                .get_last_user_realm_role(UpTo::Timestamp(timestamp), author_user_id, realm_id)
                .await?
                .and_then(|certif| certif.role);
            match author_role {
                Some(role) => {
                    if encrypted_edit_key.is_some() && !role.can_write() {
                        // The author has provided an edit key but doesn't have write access to the realm :(
                        let what = Box::new(
                            InvalidCryptpadSessionKeysError::AuthorRealmRoleCannotWrite {
                                realm: realm_id,
                                document: document_id,
                                author: author.to_owned(),
                                timestamp,
                                author_role: role,
                            },
                        );
                        return Err(
                            CertifValidateCryptpadSessionKeysError::InvalidCryptpadSessionKeys(
                                what,
                            ),
                        );
                    } else {
                        // All good :)
                        role
                    }
                }

                // The author wasn't part of the realm :(
                None => {
                    let what = Box::new(InvalidCryptpadSessionKeysError::AuthorNoAccessToRealm {
                        realm: realm_id,
                        document: document_id,
                        author: author.to_owned(),
                        timestamp,
                    });
                    return Err(
                        CertifValidateCryptpadSessionKeysError::InvalidCryptpadSessionKeys(what),
                    );
                }
            };

            // 4) Actually validate the keys

            let view_key = verify_and_load_session_key(
                &view_cleartext,
                realm_id,
                document_id,
                author,
                timestamp,
                &author_certif.verify_key,
                false,
            )?;

            let edit_key = match edit_cleartext {
                None => None,
                Some(edit_cleartext) => Some(verify_and_load_session_key(
                    &edit_cleartext,
                    realm_id,
                    document_id,
                    author,
                    timestamp,
                    &author_certif.verify_key,
                    true,
                )?),
            };

            Ok(CryptpadSessionKeys { view_key, edit_key })
        })
        .await
        .map_err(|err| match err {
            CertifForReadWithRequirementsError::Offline(e) => {
                CertifValidateCryptpadSessionKeysError::Offline(e)
            }
            CertifForReadWithRequirementsError::Stopped => {
                CertifValidateCryptpadSessionKeysError::Stopped
            }
            CertifForReadWithRequirementsError::InvalidCertificate(err) => {
                CertifValidateCryptpadSessionKeysError::InvalidCertificate(err)
            }
            CertifForReadWithRequirementsError::InvalidRequirements => {
                // This shouldn't occur since `needed_realm_certificate_timestamp` and
                // `needed_common_certificate_timestamp` are provided by the server along
                // with the keys to validate (and the server is expected to only provide
                // us with valid requirements !).
                CertifValidateCryptpadSessionKeysError::Internal(anyhow::anyhow!(
                    "Unexpected invalid requirements"
                ))
            }
            CertifForReadWithRequirementsError::Internal(err) => err.into(),
        })?
}

#[expect(clippy::too_many_arguments)]
async fn decrypt_session_key(
    ops: &CertificateOps,
    store: &mut CertificatesStoreReadGuard<'_>,
    realm_id: VlobID,
    key_index: IndexInt,
    document_id: VlobID,
    author: DeviceID,
    timestamp: DateTime,
    encrypted: &[u8],
) -> Result<Vec<u8>, CertifValidateCryptpadSessionKeysError> {
    realm_keys_bundle::decrypt_for_realm(
        ops,
        store,
        EncryptionUsage::CryptpadSessionKey(document_id),
        realm_id,
        key_index,
        encrypted,
    )
    .await
    .map_err(|err| match err {
        CertifDecryptForRealmError::Stopped => CertifValidateCryptpadSessionKeysError::Stopped,
        CertifDecryptForRealmError::Offline(e) => {
            CertifValidateCryptpadSessionKeysError::Offline(e)
        }
        CertifDecryptForRealmError::NotAllowed => {
            CertifValidateCryptpadSessionKeysError::NotAllowed
        }
        CertifDecryptForRealmError::RealmDeleted => {
            CertifValidateCryptpadSessionKeysError::RealmDeleted
        }
        CertifDecryptForRealmError::KeyNotFound => {
            CertifValidateCryptpadSessionKeysError::InvalidCryptpadSessionKeys(Box::new(
                InvalidCryptpadSessionKeysError::NonExistentKeyIndex {
                    realm: realm_id,
                    document: document_id,
                    author: author.to_owned(),
                    timestamp,
                    key_index,
                },
            ))
        }
        CertifDecryptForRealmError::CorruptedKey => {
            CertifValidateCryptpadSessionKeysError::InvalidCryptpadSessionKeys(Box::new(
                InvalidCryptpadSessionKeysError::CorruptedKey {
                    realm: realm_id,
                    document: document_id,
                    author: author.to_owned(),
                    timestamp,
                    key_index,
                },
            ))
        }
        CertifDecryptForRealmError::CorruptedData => {
            CertifValidateCryptpadSessionKeysError::InvalidCryptpadSessionKeys(Box::new(
                InvalidCryptpadSessionKeysError::CannotDecrypt {
                    realm: realm_id,
                    document: document_id,
                    author: author.to_owned(),
                    timestamp,
                    key_index,
                },
            ))
        }
        CertifDecryptForRealmError::InvalidCertificate(err) => {
            CertifValidateCryptpadSessionKeysError::InvalidCertificate(err)
        }
        CertifDecryptForRealmError::InvalidKeysBundle(err) => {
            CertifValidateCryptpadSessionKeysError::InvalidKeysBundle(err)
        }
        CertifDecryptForRealmError::Internal(err) => err.into(),
    })
}

fn verify_and_load_session_key(
    cleartext: &[u8],
    realm_id: VlobID,
    document_id: VlobID,
    author: DeviceID,
    timestamp: DateTime,
    author_verify_key: &VerifyKey,
    is_edit_key: bool,
) -> Result<String, CertifValidateCryptpadSessionKeysError> {
    let obj = CryptpadSessionKey::verify_and_load(
        cleartext,
        author_verify_key,
        author,
        timestamp,
        document_id,
    )
    .map_err(|error| {
        CertifValidateCryptpadSessionKeysError::InvalidCryptpadSessionKeys(Box::new(
            InvalidCryptpadSessionKeysError::CleartextCorrupted {
                realm: realm_id,
                document: document_id,
                author: author.to_owned(),
                timestamp,
                error: Box::new(error),
            },
        ))
    })?;

    if obj.can_edit != is_edit_key {
        return Err(
            CertifValidateCryptpadSessionKeysError::InvalidCryptpadSessionKeys(Box::new(
                InvalidCryptpadSessionKeysError::CleartextCorrupted {
                    realm: realm_id,
                    document: document_id,
                    author: author.to_owned(),
                    timestamp,
                    error: Box::new(DataError::DataIntegrity {
                        data_type: std::any::type_name::<CryptpadSessionKey>(),
                        invariant: if is_edit_key {
                            "Expected edit key, got view key"
                        } else {
                            "Expected view key, got edit key"
                        },
                    }),
                },
            )),
        );
    }

    Ok(obj.key)
}
