// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_platform_storage::certificates::PerTopicLastTimestamps;
use libparsec_types::prelude::*;

use crate::{
    certif::realm_keys_bundle::{self, EncrytionUsage},
    CertifDecryptForRealmError,
};

use super::{
    store::CertifForReadWithRequirementsError, CertificateOps, InvalidCertificateError,
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
    ops: &CertificateOps,
    needed_realm_certificate_timestamp: DateTime,
    realm_id: VlobID,
    key_index: IndexInt,
    manifest: &FileManifest,
    access: &BlockAccess,
    encrypted: &[u8],
) -> Result<Vec<u8>, CertifValidateBlockError> {
    let needed_timestamps =
        PerTopicLastTimestamps::new_for_realm(realm_id, needed_realm_certificate_timestamp);

    ops.store
        .for_read_with_requirements(ops, &needed_timestamps, |store| async move {
            realm_keys_bundle::decrypt_for_realm(
                ops,
                store,
                EncrytionUsage::Block(access.id),
                realm_id,
                key_index,
                encrypted,
            )
            .await
            .map_err(|err| match err {
                CertifDecryptForRealmError::Stopped => CertifValidateBlockError::Stopped,
                CertifDecryptForRealmError::Offline => CertifValidateBlockError::Offline,
                CertifDecryptForRealmError::NotAllowed => CertifValidateBlockError::NotAllowed,
                CertifDecryptForRealmError::KeyNotFound => {
                    CertifValidateBlockError::InvalidBlockAccess(Box::new(
                        InvalidBlockAccessError::NonExistentKeyIndex {
                            realm_id,
                            manifest_id: manifest.id,
                            manifest_version: manifest.version,
                            manifest_timestamp: manifest.timestamp,
                            manifest_author: manifest.author,
                            block_id: access.id,
                            key_index,
                        },
                    ))
                }
                CertifDecryptForRealmError::CorruptedKey => {
                    CertifValidateBlockError::InvalidBlockAccess(Box::new(
                        InvalidBlockAccessError::CorruptedKey {
                            realm_id,
                            manifest_id: manifest.id,
                            manifest_version: manifest.version,
                            manifest_timestamp: manifest.timestamp,
                            manifest_author: manifest.author,
                            block_id: access.id,
                            key_index,
                        },
                    ))
                }
                CertifDecryptForRealmError::CorruptedData => {
                    CertifValidateBlockError::InvalidBlockAccess(Box::new(
                        InvalidBlockAccessError::CannotDecrypt {
                            realm_id,
                            manifest_id: manifest.id,
                            manifest_version: manifest.version,
                            manifest_timestamp: manifest.timestamp,
                            manifest_author: manifest.author,
                            block_id: access.id,
                            key_index,
                        },
                    ))
                }
                CertifDecryptForRealmError::InvalidCertificate(err) => {
                    CertifValidateBlockError::InvalidCertificate(err)
                }
                CertifDecryptForRealmError::InvalidKeysBundle(err) => {
                    CertifValidateBlockError::InvalidKeysBundle(err)
                }
                CertifDecryptForRealmError::Internal(err) => err.into(),
            })
        })
        .await
        .map_err(|err| match err {
            CertifForReadWithRequirementsError::Offline => CertifValidateBlockError::Offline,
            CertifForReadWithRequirementsError::Stopped => CertifValidateBlockError::Stopped,
            CertifForReadWithRequirementsError::InvalidCertificate(err) => {
                CertifValidateBlockError::InvalidCertificate(err)
            }
            CertifForReadWithRequirementsError::InvalidRequirements => {
                // This shouldn't occur since `needed_realm_certificate_timestamp` is provided by the server along with the block to validate
                // (and the server is expected to only provide us with valid requirements !).
                CertifValidateBlockError::Internal(anyhow::anyhow!(
                    "Unexpected invalid requirements"
                ))
            }
            CertifForReadWithRequirementsError::Internal(err) => err.into(),
        })?
}
