// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
use libparsec_types::prelude::*;

use crate::certif::realm_keys_bundle::{self, KeyFromIndexError, LoadLastKeysBundleError};

use super::{
    store::{CertifForReadWithRequirementsError, CertificatesStoreReadGuard, GetCertificateError},
    CertifOps, InvalidCertificateError, InvalidKeysBundleError, UpTo,
};

#[derive(Debug, thiserror::Error)]
pub enum InvalidManifestError {
    #[error("Manifest from vlob `{vlob}` version {version} (in realm `{realm}`, create by `{author}` on {timestamp}) is corrupted: {error}")]
    Corrupted {
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
    InvalidManifest(#[from] InvalidManifestError),
    #[error(transparent)]
    InvalidCertificate(#[from] InvalidCertificateError),
    #[error(transparent)]
    InvalidKeysBundle(#[from] InvalidKeysBundleError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn validate_user_manifest(
    ops: &CertifOps,
    needed_realm_certificate_timestamp: DateTime,
    needed_common_certificate_timestamp: DateTime,
    author: &DeviceID,
    version: VersionInt,
    timestamp: DateTime,
    encrypted: &[u8],
) -> Result<UserManifest, CertifValidateManifestError> {
    let needed_timestamps = PerTopicLastTimestamps::new_for_common_and_realm(
        needed_common_certificate_timestamp,
        ops.device.user_realm_id,
        needed_realm_certificate_timestamp,
    );

    ops.store
        .for_read_with_requirements(ops, &needed_timestamps, |store| async move {
            // Do the actual validation

            let res = validate_manifest(
                store,
                ops.device.user_realm_id,
                &ops.device.user_realm_key,
                ops.device.user_realm_id,
                author,
                version,
                timestamp,
                encrypted,
                UserManifest::decrypt_verify_and_load,
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
            CertifForReadWithRequirementsError::Internal(err) => err.into(),
        })?
}

#[allow(clippy::too_many_arguments)]
pub(super) async fn validate_workspace_manifest(
    ops: &CertifOps,
    needed_realm_certificate_timestamp: DateTime,
    needed_common_certificate_timestamp: DateTime,
    realm_id: VlobID,
    key_index: IndexInt,
    author: &DeviceID,
    version: VersionInt,
    timestamp: DateTime,
    encrypted: &[u8],
) -> Result<WorkspaceManifest, CertifValidateManifestError> {
    let vlob_id = realm_id;

    let needed_timestamps = PerTopicLastTimestamps::new_for_common_and_realm(
        needed_common_certificate_timestamp,
        realm_id,
        needed_realm_certificate_timestamp,
    );

    ops.store
        .for_read_with_requirements(ops, &needed_timestamps, |store| async move {
            // 1) Retrieve the realm key

            let realm_keys = realm_keys_bundle::load_last_realm_keys_bundle(ops, store, realm_id)
                .await
                .map_err(|e| match e {
                    LoadLastKeysBundleError::Offline => CertifValidateManifestError::Offline,
                    LoadLastKeysBundleError::NotAllowed => CertifValidateManifestError::NotAllowed,
                    LoadLastKeysBundleError::NoKey => {
                        let err = InvalidManifestError::NonExistentKeyIndex {
                            realm: realm_id,
                            vlob: vlob_id,
                            version,
                            author: author.to_owned(),
                            timestamp,
                            key_index,
                        };
                        CertifValidateManifestError::InvalidManifest(err)
                    }
                    LoadLastKeysBundleError::InvalidKeysBundle(err) => {
                        CertifValidateManifestError::InvalidKeysBundle(err)
                    }
                    LoadLastKeysBundleError::Internal(err) => err.into(),
                })?;
            let key = realm_keys
                .key_from_index(key_index, timestamp)
                .map_err(|e| match e {
                    KeyFromIndexError::CorruptedKey => {
                        CertifValidateManifestError::InvalidManifest(
                            InvalidManifestError::CorruptedKey {
                                realm: realm_id,
                                vlob: vlob_id,
                                version,
                                author: author.to_owned(),
                                timestamp,
                                key_index,
                            },
                        )
                    }
                    KeyFromIndexError::KeyNotFound => CertifValidateManifestError::InvalidManifest(
                        InvalidManifestError::NonExistentKeyIndex {
                            realm: realm_id,
                            vlob: vlob_id,
                            version,
                            author: author.to_owned(),
                            timestamp,
                            key_index,
                        },
                    ),
                })?;

            // 2) Do the actual validation

            let res = validate_manifest(
                store,
                realm_id,
                key,
                vlob_id,
                author,
                version,
                timestamp,
                encrypted,
                WorkspaceManifest::decrypt_verify_and_load,
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
            CertifForReadWithRequirementsError::Internal(err) => err.into(),
        })?
}

#[allow(clippy::too_many_arguments)]
pub(super) async fn validate_child_manifest(
    ops: &CertifOps,
    needed_realm_certificate_timestamp: DateTime,
    needed_common_certificate_timestamp: DateTime,
    realm_id: VlobID,
    key_index: IndexInt,
    vlob_id: VlobID,
    author: &DeviceID,
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
            // 1) Retrieve the realm key

            let realm_keys = realm_keys_bundle::load_last_realm_keys_bundle(ops, store, realm_id)
                .await
                .map_err(|e| match e {
                    LoadLastKeysBundleError::Offline => CertifValidateManifestError::Offline,
                    LoadLastKeysBundleError::NotAllowed => CertifValidateManifestError::NotAllowed,
                    LoadLastKeysBundleError::NoKey => {
                        let err = InvalidManifestError::NonExistentKeyIndex {
                            realm: realm_id,
                            vlob: vlob_id,
                            version,
                            author: author.to_owned(),
                            timestamp,
                            key_index,
                        };
                        CertifValidateManifestError::InvalidManifest(err)
                    }
                    LoadLastKeysBundleError::InvalidKeysBundle(err) => {
                        CertifValidateManifestError::InvalidKeysBundle(err)
                    }
                    LoadLastKeysBundleError::Internal(err) => err.into(),
                })?;
            let key = realm_keys
                .key_from_index(key_index, timestamp)
                .map_err(|e| match e {
                    KeyFromIndexError::CorruptedKey => {
                        CertifValidateManifestError::InvalidManifest(
                            InvalidManifestError::CorruptedKey {
                                realm: realm_id,
                                vlob: vlob_id,
                                version,
                                author: author.to_owned(),
                                timestamp,
                                key_index,
                            },
                        )
                    }
                    KeyFromIndexError::KeyNotFound => CertifValidateManifestError::InvalidManifest(
                        InvalidManifestError::NonExistentKeyIndex {
                            realm: realm_id,
                            vlob: vlob_id,
                            version,
                            author: author.to_owned(),
                            timestamp,
                            key_index,
                        },
                    ),
                })?;

            // 2) Do the actual validation

            let res = validate_manifest(
                store,
                realm_id,
                key,
                vlob_id,
                author,
                version,
                timestamp,
                encrypted,
                ChildManifest::decrypt_verify_and_load,
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
            CertifForReadWithRequirementsError::Internal(err) => err.into(),
        })?
}

#[allow(clippy::too_many_arguments)]
async fn validate_manifest<M>(
    store: &mut CertificatesStoreReadGuard<'_>,
    realm_id: VlobID,
    key: &SecretKey,
    vlob_id: VlobID,
    author: &DeviceID,
    version: VersionInt,
    timestamp: DateTime,
    encrypted: &[u8],
    decrypt_verify_and_load: impl FnOnce(
        &[u8],
        &SecretKey,
        &VerifyKey,
        &DeviceID,
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
            let what = InvalidManifestError::NonExistentAuthor {
                realm: realm_id,
                vlob: vlob_id,
                version,
                author: author.to_owned(),
                timestamp,
            };
            return Err(CertifValidateManifestError::InvalidManifest(what));
        }

        // D'oh :/
        Err(err @ GetCertificateError::Internal(_)) => {
            return Err(CertifValidateManifestError::Internal(err.into()))
        }
    };

    // 1.2) Check author is not revoked

    match store
        .get_revoked_user_certificate(UpTo::Timestamp(timestamp), author.user_id().clone())
        .await
    {
        // Not revoked at the considered timestamp, as we expected :)
        Ok(None) => (),

        // Revoked :(
        Ok(Some(_)) => {
            let what = InvalidManifestError::RevokedAuthor {
                realm: realm_id,
                vlob: vlob_id,
                version,
                author: author.to_owned(),
                timestamp,
            };
            return Err(CertifValidateManifestError::InvalidManifest(what));
        }

        // D'oh :/
        Err(err) => return Err(CertifValidateManifestError::Internal(err)),
    }

    // 2) Actually validate the manifest

    let manifest = decrypt_verify_and_load(
        encrypted,
        key,
        &author_certif.verify_key,
        author,
        timestamp,
        Some(vlob_id),
        Some(version),
    )
    .map_err(|error| {
        let what = InvalidManifestError::Corrupted {
            realm: realm_id,
            vlob: vlob_id,
            version,
            author: author.to_owned(),
            timestamp,
            error: Box::new(error),
        };
        CertifValidateManifestError::InvalidManifest(what)
    })?;

    // 3) Finally we have to check the manifest content is consistent with the system
    // (i.e. the author had the right to create this manifest)
    let author_role = store
        .get_user_realm_role(
            UpTo::Timestamp(timestamp),
            author.user_id().to_owned(),
            realm_id,
        )
        .await?
        .and_then(|certif| certif.role);
    match author_role {
        // All good :)
        Some(role) if role.can_write() => (),

        // The author wasn't part of the realm :(
        None => {
            let what = InvalidManifestError::AuthorNoAccessToRealm {
                realm: realm_id,
                vlob: vlob_id,
                version,
                author: author.to_owned(),
                timestamp,
            };
            return Err(CertifValidateManifestError::InvalidManifest(what));
        }

        // The author doesn't have write access to the realm :(
        Some(role) => {
            let what = InvalidManifestError::AuthorRealmRoleCannotWrite {
                realm: realm_id,
                vlob: vlob_id,
                version,
                author: author.to_owned(),
                timestamp,
                author_role: role,
            };
            return Err(CertifValidateManifestError::InvalidManifest(what));
        }
    }

    Ok(manifest)
}
