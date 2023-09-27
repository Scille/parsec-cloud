// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_types::prelude::*;

use super::{CertificatesOps, GetCertificateError, InvalidCertificateError, PollServerError, UpTo};

#[derive(Debug, thiserror::Error)]
pub enum InvalidManifestError {
    #[error("Manifest from vlob `{vlob}` version {version} (in realm {realm}, create by `{author}` on {timestamp}) is corrupted: {error}")]
    Corrupted {
        realm: VlobID,
        vlob: VlobID,
        version: VersionInt,
        author: DeviceID,
        timestamp: DateTime,
        error: DataError,
    },
    #[error("Manifest from vlob `{vlob}` version {version} (in realm {realm}, create by `{author}` on {timestamp}): at that time author didn't exist !")]
    NonExistantAuthor {
        realm: VlobID,
        vlob: VlobID,
        version: VersionInt,
        author: DeviceID,
        timestamp: DateTime,
    },
    #[error("Manifest from vlob `{vlob}` version {version} (in realm {realm}, create by `{author}` on {timestamp}): at that time author was already revoked !")]
    RevokedAuthor {
        realm: VlobID,
        vlob: VlobID,
        version: VersionInt,
        author: DeviceID,
        timestamp: DateTime,
    },
    #[error("Manifest from vlob `{vlob}` version {version} (in realm {realm}, create by `{author}` on {timestamp}): at that time author couldn't write in the realm given it role was `{author_role:?}`")]
    AuthorRealmRoleCannotWrite {
        realm: VlobID,
        vlob: VlobID,
        version: VersionInt,
        author: DeviceID,
        timestamp: DateTime,
        author_role: RealmRole,
    },
    #[error("Manifest from vlob `{vlob}` version {version} (in realm {realm}, create by `{author}` on {timestamp}): at that time author didn't have access to the realm and hence couldn't write in it")]
    AuthorNoAccessToRealm {
        realm: VlobID,
        vlob: VlobID,
        version: VersionInt,
        author: DeviceID,
        timestamp: DateTime,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum ValidateManifestError {
    #[error("Cannot reach the server")]
    Offline,
    #[error(transparent)]
    InvalidManifest(#[from] InvalidManifestError),
    #[error(transparent)]
    InvalidCertificate(#[from] InvalidCertificateError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn validate_user_manifest(
    ops: &CertificatesOps,
    certificate_index: IndexInt,
    author: &DeviceID,
    version: VersionInt,
    timestamp: DateTime,
    encrypted: &[u8],
) -> Result<UserManifest, ValidateManifestError> {
    let realm_id = ops.device.user_realm_id;
    let realm_key = &ops.device.user_realm_key;
    let vlob_id = ops.device.user_realm_id;

    validate_manifest(
        ops,
        realm_id,
        realm_key,
        vlob_id,
        certificate_index,
        author,
        version,
        timestamp,
        encrypted,
        UserManifest::decrypt_verify_and_load,
    )
    .await
}

#[allow(clippy::too_many_arguments)]
pub(super) async fn validate_workspace_manifest(
    ops: &CertificatesOps,
    realm_id: VlobID,
    realm_key: &SecretKey,
    certificate_index: IndexInt,
    author: &DeviceID,
    version: VersionInt,
    timestamp: DateTime,
    encrypted: &[u8],
) -> Result<WorkspaceManifest, ValidateManifestError> {
    let vlob_id = realm_id;

    validate_manifest(
        ops,
        realm_id,
        realm_key,
        vlob_id,
        certificate_index,
        author,
        version,
        timestamp,
        encrypted,
        WorkspaceManifest::decrypt_verify_and_load,
    )
    .await
}

#[allow(clippy::too_many_arguments)]
pub(super) async fn validate_child_manifest(
    ops: &CertificatesOps,
    realm_id: VlobID,
    realm_key: &SecretKey,
    vlob_id: VlobID,
    certificate_index: IndexInt,
    author: &DeviceID,
    version: VersionInt,
    timestamp: DateTime,
    encrypted: &[u8],
) -> Result<ChildManifest, ValidateManifestError> {
    validate_manifest(
        ops,
        realm_id,
        realm_key,
        vlob_id,
        certificate_index,
        author,
        version,
        timestamp,
        encrypted,
        ChildManifest::decrypt_verify_and_load,
    )
    .await
}

#[allow(clippy::too_many_arguments)]
async fn validate_manifest<M>(
    ops: &CertificatesOps,
    realm_id: VlobID,
    realm_key: &SecretKey,
    vlob_id: VlobID,
    certificate_index: IndexInt,
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
) -> Result<M, ValidateManifestError> {
    // 1) Make sure we have all the needed certificates

    let store = super::poll::ensure_certificates_available_and_read_lock(ops, certificate_index)
        .await
        .map_err(|err| match err {
            PollServerError::Offline => ValidateManifestError::Offline,
            PollServerError::InvalidCertificate(err) => {
                ValidateManifestError::InvalidCertificate(err)
            }
            err @ PollServerError::Internal(_) => ValidateManifestError::Internal(err.into()),
        })?;

    // 2) Author must exist and not be revoked at the time the manifest was created !

    // 2.1) Check device exists (this also imply the user exists)

    let author_certif = match store
        .get_device_certificate(UpTo::Index(certificate_index), author.to_owned())
        .await
    {
        // Exists, as expected :)
        Ok(certif) => certif,

        // Doesn't exist at the considered index :(
        Err(GetCertificateError::NonExisting | GetCertificateError::ExistButTooRecent { .. }) => {
            let what = InvalidManifestError::NonExistantAuthor {
                realm: realm_id,
                vlob: vlob_id,
                version,
                author: author.to_owned(),
                timestamp,
            };
            return Err(ValidateManifestError::InvalidManifest(what));
        }

        // D'oh :/
        Err(err @ GetCertificateError::Internal(_)) => {
            return Err(ValidateManifestError::Internal(err.into()))
        }
    };

    // 2.2) Check author is not revoked

    match store
        .get_revoked_user_certificate(UpTo::Index(certificate_index), author.user_id().clone())
        .await
    {
        // Not revoked at the considered index, as we expected :)
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
            return Err(ValidateManifestError::InvalidManifest(what));
        }

        // D'oh :/
        Err(err) => return Err(ValidateManifestError::Internal(err)),
    }

    // 3) Actually validate the manifest

    let manifest = decrypt_verify_and_load(
        encrypted,
        realm_key,
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
            error,
        };
        ValidateManifestError::InvalidManifest(what)
    })?;

    // 4) Finally we have to check the manifest content is consistent with the system
    // (i.e. the author had the right to create this manifest)
    let author_role = store
        .get_user_realm_role(
            UpTo::Index(certificate_index),
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
            return Err(ValidateManifestError::InvalidManifest(what));
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
            return Err(ValidateManifestError::InvalidManifest(what));
        }
    }

    Ok(manifest)
}
