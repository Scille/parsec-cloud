// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
use libparsec_types::prelude::*;

use crate::{certif::realm_keys_bundle, CertifDecryptForRealmError, EncrytionUsage};

use super::{
    store::{CertifForReadWithRequirementsError, CertificatesStoreReadGuard, GetCertificateError},
    CertificateOps, InvalidCertificateError, InvalidKeysBundleError, UpTo,
};

#[derive(Debug, thiserror::Error)]
pub enum InvalidManifestError {
    #[error("Manifest from vlob `{vlob}` version {version} (in realm `{realm}`, create by `{author}` on {timestamp}): cannot be decrypted by key index {key_index} !")]
    CannotDecrypt {
        realm: VlobID,
        vlob: VlobID,
        version: VersionInt,
        author: DeviceID,
        timestamp: DateTime,
        key_index: IndexInt,
    },
    #[error("Manifest from vlob `{vlob}` version {version} (in realm `{realm}`, create by `{author}` on {timestamp}) can be decrypted but its content is corrupted: {error}")]
    CleartextCorrupted {
        realm: VlobID,
        vlob: VlobID,
        version: VersionInt,
        author: DeviceID,
        timestamp: DateTime,
        error: Box<DataError>,
    },
    #[error("Manifest from vlob `{vlob}` version {version} (in realm `{realm}`, create by `{author}` on {timestamp}): at that time, key index {key_index} didn't exist !")]
    NonExistentKeyIndex {
        realm: VlobID,
        vlob: VlobID,
        version: VersionInt,
        author: DeviceID,
        timestamp: DateTime,
        key_index: IndexInt,
    },
    #[error("Manifest from vlob `{vlob}` version {version} (in realm `{realm}`, create by `{author}` on {timestamp}): encrypted by key index {key_index} which appears corrupted !")]
    CorruptedKey {
        realm: VlobID,
        vlob: VlobID,
        version: VersionInt,
        author: DeviceID,
        timestamp: DateTime,
        key_index: IndexInt,
    },
    #[error("Manifest from vlob `{vlob}` version {version} (in realm `{realm}`, create by `{author}` on {timestamp}): at that time author didn't exist !")]
    NonExistentAuthor {
        realm: VlobID,
        vlob: VlobID,
        version: VersionInt,
        author: DeviceID,
        timestamp: DateTime,
    },
    #[error("Manifest from vlob `{vlob}` version {version} (in realm `{realm}`, create by `{author}` on {timestamp}): at that time author was already revoked !")]
    RevokedAuthor {
        realm: VlobID,
        vlob: VlobID,
        version: VersionInt,
        author: DeviceID,
        timestamp: DateTime,
    },
    #[error("Manifest from vlob `{vlob}` version {version} (in realm `{realm}`, create by `{author}` on {timestamp}): at that time author couldn't write in the realm given it role was `{author_role:?}`")]
    AuthorRealmRoleCannotWrite {
        realm: VlobID,
        vlob: VlobID,
        version: VersionInt,
        author: DeviceID,
        timestamp: DateTime,
        author_role: RealmRole,
    },
    #[error("Manifest from vlob `{vlob}` version {version} (in realm `{realm}`, create by `{author}` on {timestamp}): at that time author didn't have access to the realm and hence couldn't write in it")]
    AuthorNoAccessToRealm {
        realm: VlobID,
        vlob: VlobID,
        version: VersionInt,
        author: DeviceID,
        timestamp: DateTime,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum CertifValidateManifestError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("Not allowed to access this realm")]
    NotAllowed,
    #[error(transparent)]
    InvalidManifest(#[from] Box<InvalidManifestError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn validate_user_manifest(
    ops: &CertificateOps,
    needed_realm_certificate_timestamp: DateTime,
    needed_common_certificate_timestamp: DateTime,
    author: DeviceID,
    version: VersionInt,
    timestamp: DateTime,
    encrypted: &[u8],
) -> Result<UserManifest, CertifValidateManifestError> {
    let realm_id = ops.device.user_realm_id;
    let vlob_id = ops.device.user_realm_id;

    let needed_timestamps = PerTopicLastTimestamps::new_for_common_and_realm(
        needed_common_certificate_timestamp,
        ops.device.user_realm_id,
        needed_realm_certificate_timestamp,
    );

    ops.store
        .for_read_with_requirements(ops, &needed_timestamps, |store| async move {
            // 1) Decrypt the vlob

            let cleartext = &ops.device.user_realm_key.decrypt(encrypted).map_err(|_| {
                CertifValidateManifestError::InvalidManifest(Box::new(
                    InvalidManifestError::CannotDecrypt {
                        realm: realm_id,
                        vlob: vlob_id,
                        version,
                        author: author.to_owned(),
                        timestamp,
                        // User realm never use key rotation
                        key_index: 0,
                    },
                ))
            })?;

            // 2) Deserialize the manifest and do the additional validations

            let res = validate_manifest(
                store,
                realm_id,
                vlob_id,
                author,
                version,
                timestamp,
                cleartext,
                UserManifest::verify_and_load,
            )
            .await?;

            Ok(res)
        })
        .await
        .map_err(|err| match err {
            CertifForReadWithRequirementsError::Offline => CertifValidateManifestError::Offline,
            CertifForReadWithRequirementsError::Stopped => CertifValidateManifestError::Stopped,
            CertifForReadWithRequirementsError::InvalidCertificate(err) => {
                CertifValidateManifestError::InvalidCertificate(err)
            }
            CertifForReadWithRequirementsError::InvalidRequirements => {
                CertifValidateManifestError::Internal(anyhow::anyhow!(
                    "Unexpected invalid requirements"
                ))
            }
            CertifForReadWithRequirementsError::Internal(err) => err.into(),
        })?
}

#[allow(clippy::too_many_arguments)]
pub(super) async fn validate_workspace_manifest(
    ops: &CertificateOps,
    needed_realm_certificate_timestamp: DateTime,
    needed_common_certificate_timestamp: DateTime,
    realm_id: VlobID,
    key_index: IndexInt,
    author: DeviceID,
    version: VersionInt,
    timestamp: DateTime,
    encrypted: &[u8],
) -> Result<FolderManifest, CertifValidateManifestError> {
    let vlob_id = realm_id;

    let needed_timestamps = PerTopicLastTimestamps::new_for_common_and_realm(
        needed_common_certificate_timestamp,
        realm_id,
        needed_realm_certificate_timestamp,
    );

    ops.store
        .for_read_with_requirements(ops, &needed_timestamps, |store| async move {
            // 1) Decrypt the vlob

            let cleartext = realm_keys_bundle::decrypt_for_realm(
                ops,
                store,
                EncrytionUsage::Vlob(realm_id),
                realm_id,
                key_index,
                encrypted,
            )
            .await
            .map_err(|err| match err {
                CertifDecryptForRealmError::Stopped => CertifValidateManifestError::Stopped,
                CertifDecryptForRealmError::Offline => CertifValidateManifestError::Offline,
                CertifDecryptForRealmError::NotAllowed => CertifValidateManifestError::NotAllowed,
                CertifDecryptForRealmError::KeyNotFound => {
                    CertifValidateManifestError::InvalidManifest(Box::new(
                        InvalidManifestError::NonExistentKeyIndex {
                            realm: realm_id,
                            vlob: vlob_id,
                            version,
                            author: author.to_owned(),
                            timestamp,
                            key_index,
                        },
                    ))
                }
                CertifDecryptForRealmError::CorruptedKey => {
                    CertifValidateManifestError::InvalidManifest(Box::new(
                        InvalidManifestError::CorruptedKey {
                            realm: realm_id,
                            vlob: vlob_id,
                            version,
                            author: author.to_owned(),
                            timestamp,
                            key_index,
                        },
                    ))
                }
                CertifDecryptForRealmError::CorruptedData => {
                    CertifValidateManifestError::InvalidManifest(Box::new(
                        InvalidManifestError::CannotDecrypt {
                            realm: realm_id,
                            vlob: vlob_id,
                            version,
                            author: author.to_owned(),
                            timestamp,
                            key_index,
                        },
                    ))
                }
                CertifDecryptForRealmError::InvalidCertificate(err) => {
                    CertifValidateManifestError::InvalidCertificate(err)
                }
                CertifDecryptForRealmError::InvalidKeysBundle(err) => {
                    CertifValidateManifestError::InvalidKeysBundle(err)
                }
                CertifDecryptForRealmError::Internal(err) => err.into(),
            })?;

            // 2) Deserialize the manifest and do the additional validations

            let res = validate_manifest(
                store,
                realm_id,
                vlob_id,
                author,
                version,
                timestamp,
                &cleartext,
                WorkspaceManifest::verify_and_load,
            )
            .await?;

            Ok(res.into())
        })
        .await
        .map_err(|err| match err {
            CertifForReadWithRequirementsError::Offline => CertifValidateManifestError::Offline,
            CertifForReadWithRequirementsError::Stopped => CertifValidateManifestError::Stopped,
            CertifForReadWithRequirementsError::InvalidCertificate(err) => {
                CertifValidateManifestError::InvalidCertificate(err)
            }
            CertifForReadWithRequirementsError::InvalidRequirements => {
                CertifValidateManifestError::Internal(anyhow::anyhow!(
                    "Unexpected invalid requirements"
                ))
            }
            CertifForReadWithRequirementsError::Internal(err) => err.into(),
        })?
}

#[allow(clippy::too_many_arguments)]
pub(super) async fn validate_child_manifest(
    ops: &CertificateOps,
    needed_realm_certificate_timestamp: DateTime,
    needed_common_certificate_timestamp: DateTime,
    realm_id: VlobID,
    key_index: IndexInt,
    vlob_id: VlobID,
    author: DeviceID,
    version: VersionInt,
    timestamp: DateTime,
    encrypted: &[u8],
) -> Result<ChildManifest, CertifValidateManifestError> {
    let needed_timestamps = PerTopicLastTimestamps::new_for_common_and_realm(
        needed_common_certificate_timestamp,
        realm_id,
        needed_realm_certificate_timestamp,
    );

    ops.store
        .for_read_with_requirements(ops, &needed_timestamps, |store| async move {
            // 1) Decrypt the vlob

            let cleartext = realm_keys_bundle::decrypt_for_realm(
                ops,
                store,
                EncrytionUsage::Vlob(vlob_id),
                realm_id,
                key_index,
                encrypted,
            )
            .await
            .map_err(|err| match err {
                CertifDecryptForRealmError::Stopped => CertifValidateManifestError::Stopped,
                CertifDecryptForRealmError::Offline => CertifValidateManifestError::Offline,
                CertifDecryptForRealmError::NotAllowed => CertifValidateManifestError::NotAllowed,
                CertifDecryptForRealmError::KeyNotFound => {
                    CertifValidateManifestError::InvalidManifest(Box::new(
                        InvalidManifestError::NonExistentKeyIndex {
                            realm: realm_id,
                            vlob: vlob_id,
                            version,
                            author: author.to_owned(),
                            timestamp,
                            key_index,
                        },
                    ))
                }
                CertifDecryptForRealmError::CorruptedKey => {
                    CertifValidateManifestError::InvalidManifest(Box::new(
                        InvalidManifestError::CorruptedKey {
                            realm: realm_id,
                            vlob: vlob_id,
                            version,
                            author: author.to_owned(),
                            timestamp,
                            key_index,
                        },
                    ))
                }
                CertifDecryptForRealmError::CorruptedData => {
                    CertifValidateManifestError::InvalidManifest(Box::new(
                        InvalidManifestError::CannotDecrypt {
                            realm: realm_id,
                            vlob: vlob_id,
                            version,
                            author: author.to_owned(),
                            timestamp,
                            key_index,
                        },
                    ))
                }
                CertifDecryptForRealmError::InvalidCertificate(err) => {
                    CertifValidateManifestError::InvalidCertificate(err)
                }
                CertifDecryptForRealmError::InvalidKeysBundle(err) => {
                    CertifValidateManifestError::InvalidKeysBundle(err)
                }
                CertifDecryptForRealmError::Internal(err) => err.into(),
            })?;

            // 2) Deserialize the manifest and do the additional validations

            let res = validate_manifest(
                store,
                realm_id,
                vlob_id,
                author,
                version,
                timestamp,
                &cleartext,
                ChildManifest::verify_and_load,
            )
            .await?;

            Ok(res)
        })
        .await
        .map_err(|err| match err {
            CertifForReadWithRequirementsError::Offline => CertifValidateManifestError::Offline,
            CertifForReadWithRequirementsError::Stopped => CertifValidateManifestError::Stopped,
            CertifForReadWithRequirementsError::InvalidCertificate(err) => {
                CertifValidateManifestError::InvalidCertificate(err)
            }
            CertifForReadWithRequirementsError::InvalidRequirements => {
                CertifValidateManifestError::Internal(anyhow::anyhow!(
                    "Unexpected invalid requirements"
                ))
            }
            CertifForReadWithRequirementsError::Internal(err) => err.into(),
        })?
}

#[allow(clippy::too_many_arguments)]
async fn validate_manifest<M>(
    store: &mut CertificatesStoreReadGuard<'_>,
    realm_id: VlobID,
    vlob_id: VlobID,
    author: DeviceID,
    version: VersionInt,
    timestamp: DateTime,
    cleartext: &[u8],
    verify_and_load: impl FnOnce(
        &[u8],
        &VerifyKey,
        DeviceID,
        DateTime,
        Option<VlobID>,
        Option<VersionInt>,
    ) -> DataResult<M>,
) -> Result<M, CertifValidateManifestError> {
    // 1) Author must exist and not be revoked at the time the manifest was created !

    // 1.1) Check device exists (this also imply the user exists)

    let author_certif = match store
        .get_device_certificate(UpTo::Timestamp(timestamp), author.to_owned())
        .await
    {
        // Exists, as expected :)
        Ok(certif) => certif,

        // Doesn't exist at the considered timestamp :(
        Err(GetCertificateError::NonExisting | GetCertificateError::ExistButTooRecent { .. }) => {
            let what = Box::new(InvalidManifestError::NonExistentAuthor {
                realm: realm_id,
                vlob: vlob_id,
                version,
                author: author.to_owned(),
                timestamp,
            });
            return Err(CertifValidateManifestError::InvalidManifest(what));
        }

        // D'oh :/
        Err(err @ GetCertificateError::Internal(_)) => {
            return Err(CertifValidateManifestError::Internal(err.into()))
        }
    };
    let author_user_id = author_certif.user_id;

    // 1.2) Check author is not revoked

    match store
        .get_revoked_user_certificate(UpTo::Timestamp(timestamp), author_certif.user_id)
        .await
    {
        // Not revoked at the considered timestamp, as we expected :)
        Ok(None) => (),

        // Revoked :(
        Ok(Some(_)) => {
            let what = Box::new(InvalidManifestError::RevokedAuthor {
                realm: realm_id,
                vlob: vlob_id,
                version,
                author: author.to_owned(),
                timestamp,
            });
            return Err(CertifValidateManifestError::InvalidManifest(what));
        }

        // D'oh :/
        Err(err) => return Err(CertifValidateManifestError::Internal(err)),
    }

    // 2) Actually validate the manifest

    let manifest = verify_and_load(
        cleartext,
        &author_certif.verify_key,
        author,
        timestamp,
        Some(vlob_id),
        Some(version),
    )
    .map_err(|error| {
        let what = Box::new(InvalidManifestError::CleartextCorrupted {
            realm: realm_id,
            vlob: vlob_id,
            version,
            author: author.to_owned(),
            timestamp,
            error: Box::new(error),
        });
        CertifValidateManifestError::InvalidManifest(what)
    })?;

    // 3) Finally we have to check the manifest content is consistent with the system
    // (i.e. the author had the right to create this manifest)
    let author_role = store
        .get_last_user_realm_role(UpTo::Timestamp(timestamp), author_user_id, realm_id)
        .await?
        .and_then(|certif| certif.role);
    match author_role {
        // All good :)
        Some(role) if role.can_write() => (),

        // The author wasn't part of the realm :(
        None => {
            let what = Box::new(InvalidManifestError::AuthorNoAccessToRealm {
                realm: realm_id,
                vlob: vlob_id,
                version,
                author: author.to_owned(),
                timestamp,
            });
            return Err(CertifValidateManifestError::InvalidManifest(what));
        }

        // The author doesn't have write access to the realm :(
        Some(role) => {
            let what = Box::new(InvalidManifestError::AuthorRealmRoleCannotWrite {
                realm: realm_id,
                vlob: vlob_id,
                version,
                author: author.to_owned(),
                timestamp,
                author_role: role,
            });
            return Err(CertifValidateManifestError::InvalidManifest(what));
        }
    }

    Ok(manifest)
}
