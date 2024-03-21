// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
use libparsec_types::prelude::*;

use crate::certif::realm_keys_bundle::{self, KeyFromIndexError, LoadLastKeysBundleError};

use super::{
    store::CertifForReadWithRequirementsError, CertifOps, InvalidCertificateError,
    InvalidKeysBundleError,
};
#[derive(Debug, thiserror::Error)]
pub enum InvalidBlockAccessError {
    #[error("Block access `{block_id}` from manifest `{manifest_id}` version {manifest_version} (in realm `{realm_id}`, create by `{manifest_author}` on {manifest_timestamp}): cannot be decrypted by key index {key_index} !")]
    CannotDecrypt {
        realm_id: VlobID,
        manifest_id: VlobID,
        manifest_version: VersionInt,
        manifest_timestamp: DateTime,
        manifest_author: DeviceID,
        block_id: BlockID,
        key_index: IndexInt,
    },
    #[error("Block access `{block_id}` from manifest `{manifest_id}` version {manifest_version} (in realm `{realm_id}`, create by `{manifest_author}` on {manifest_timestamp}): at that time, key index {key_index} didn't exist !")]
    NonExistentKeyIndex {
        realm_id: VlobID,
        manifest_id: VlobID,
        manifest_version: VersionInt,
        manifest_timestamp: DateTime,
        manifest_author: DeviceID,
        block_id: BlockID,
        key_index: IndexInt,
    },
    #[error("Block access `{block_id}` from manifest `{manifest_id}` version {manifest_version} (in realm `{realm_id}`, create by `{manifest_author}` on {manifest_timestamp}): encrypted by key index {key_index} which appears corrupted !")]
    CorruptedKey {
        realm_id: VlobID,
        manifest_id: VlobID,
        manifest_version: VersionInt,
        manifest_timestamp: DateTime,
        manifest_author: DeviceID,
        block_id: BlockID,
        key_index: IndexInt,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum CertifValidateBlockError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("Not allowed to access this realm")]
    NotAllowed,
    #[error(transparent)]
    InvalidBlockAccess(#[from] Box<InvalidBlockAccessError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn validate_block(
    ops: &CertifOps,
    realm_id: VlobID,
    manifest: &FileManifest,
    access: &BlockAccess,
    encrypted: &[u8],
) -> Result<Vec<u8>, CertifValidateBlockError> {
    // Special case for legacy blocks created with Parsec < v3
    if let Some(key) = &access.key {
        return key.decrypt(encrypted).map_err(|_| {
            let err = Box::new(InvalidBlockAccessError::CannotDecrypt {
                realm_id,
                manifest_id: manifest.id,
                manifest_version: manifest.version,
                manifest_timestamp: manifest.timestamp,
                manifest_author: manifest.author.clone(),
                block_id: access.id,
                key_index: 0,
            });
            CertifValidateBlockError::InvalidBlockAccess(err)
        });
    }

    let needed_timestamps = PerTopicLastTimestamps::new_for_realm(realm_id, manifest.timestamp);

    ops.store
        .for_read_with_requirements(ops, &needed_timestamps, |store| async move {
            // 1) Retrieve the realm key

            let realm_keys = realm_keys_bundle::load_last_realm_keys_bundle(ops, store, realm_id)
                .await
                .map_err(|e| match e {
                    LoadLastKeysBundleError::Offline => CertifValidateBlockError::Offline,
                    LoadLastKeysBundleError::NotAllowed => CertifValidateBlockError::NotAllowed,
                    LoadLastKeysBundleError::NoKey => {
                        // TODO: FileManifest should check the block accesses' key indexes as
                        // part of it validation process ?
                        let err = Box::new(InvalidBlockAccessError::NonExistentKeyIndex {
                            realm_id,
                            manifest_id: manifest.id,
                            manifest_version: manifest.version,
                            manifest_timestamp: manifest.timestamp,
                            manifest_author: manifest.author.clone(),
                            block_id: access.id,
                            key_index: access.key_index,
                        });
                        CertifValidateBlockError::InvalidBlockAccess(err)
                    }
                    LoadLastKeysBundleError::InvalidKeysBundle(err) => {
                        CertifValidateBlockError::InvalidKeysBundle(err)
                    }
                    LoadLastKeysBundleError::Internal(err) => err.into(),
                })?;

            let key = realm_keys
                .key_from_index(access.key_index, manifest.timestamp)
                .map_err(|e| match e {
                    KeyFromIndexError::CorruptedKey => {
                        CertifValidateBlockError::InvalidBlockAccess(Box::new(
                            InvalidBlockAccessError::CorruptedKey {
                                realm_id,
                                manifest_id: manifest.id,
                                manifest_version: manifest.version,
                                manifest_timestamp: manifest.timestamp,
                                manifest_author: manifest.author.clone(),
                                block_id: access.id,
                                key_index: access.key_index,
                            },
                        ))
                    }
                    KeyFromIndexError::KeyNotFound => CertifValidateBlockError::InvalidBlockAccess(
                        Box::new(InvalidBlockAccessError::NonExistentKeyIndex {
                            realm_id,
                            manifest_id: manifest.id,
                            manifest_version: manifest.version,
                            manifest_timestamp: manifest.timestamp,
                            manifest_author: manifest.author.clone(),
                            block_id: access.id,
                            key_index: access.key_index,
                        }),
                    ),
                })?;

            // 2) Do the actual decryption

            key.decrypt(encrypted).map_err(|_| {
                let err = Box::new(InvalidBlockAccessError::CannotDecrypt {
                    realm_id,
                    manifest_id: manifest.id,
                    manifest_version: manifest.version,
                    manifest_timestamp: manifest.timestamp,
                    manifest_author: manifest.author.clone(),
                    block_id: access.id,
                    key_index: 0,
                });
                CertifValidateBlockError::InvalidBlockAccess(err)
            })
        })
        .await
        .map_err(|err| match err {
            CertifForReadWithRequirementsError::Offline => CertifValidateBlockError::Offline,
            CertifForReadWithRequirementsError::Stopped => CertifValidateBlockError::Stopped,
            CertifForReadWithRequirementsError::InvalidCertificate(err) => {
                CertifValidateBlockError::InvalidCertificate(err)
            }
            CertifForReadWithRequirementsError::Internal(err) => err.into(),
        })?
}
