// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::{protocol::authenticated_cmds, ConnectionError};
use libparsec_types::prelude::*;

use super::{UserOps, UserStoreUpdateError};
use crate::{
    certif::{
        CertifEncryptForSequesterServicesError, CertifEnsureRealmCreatedError,
        CertifPollServerError, CertifValidateManifestError, InvalidCertificateError,
        InvalidManifestError,
    },
    EventUserOpsSynced,
};

#[derive(Debug, thiserror::Error)]
pub enum UserSyncError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("Sequester service `{service_id}` rejected the data: `{reason}`")]
    RejectedBySequesterService {
        service_id: SequesterServiceID,
        reason: String,
    },
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    // Note `InvalidManifest` here, this is because we self-repair in case of invalid
    // user manifest (given otherwise the client would be stuck for good !)
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for UserSyncError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FetchRemoteUserManifestError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("Server has no such version for this user manifest")]
    BadVersion,
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidManifest(#[from] Box<InvalidManifestError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for FetchRemoteUserManifestError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

pub async fn sync(ops: &UserOps) -> Result<(), UserSyncError> {
    let user_manifest = ops.store.get_user_manifest();

    if user_manifest.need_sync {
        outbound_sync(ops).await?;
        ops.event_bus.send(&EventUserOpsSynced);
        Ok(())
    } else {
        inbound_sync(ops).await
    }
}

enum UploadManifestOutcome {
    Success(UserManifest),
    VersionConflict,
}

async fn upload_manifest(
    ops: &UserOps,
    base_um: &LocalUserManifest,
) -> Result<UploadManifestOutcome, UserSyncError> {
    let mut timestamp = ops.device.now();

    loop {
        // Build vlob
        let to_sync_um = base_um.to_remote(ops.device.device_id.to_owned(), timestamp);

        let signed = to_sync_um.dump_and_sign(&ops.device.signing_key);
        let ciphered = ops.device.user_realm_key.encrypt(&signed).into();
        let sequester_blob = ops
            .certificates_ops
            .encrypt_for_sequester_services(&signed)
            .await
            .map_err(|e| match e {
                CertifEncryptForSequesterServicesError::Stopped => UserSyncError::Stopped,
                CertifEncryptForSequesterServicesError::Internal(err) => {
                    err.context("Cannot encrypt for sequester services").into()
                }
            })?;

        // Sync the vlob with server

        return if to_sync_um.version == 1 {
            use authenticated_cmds::latest::vlob_create::{Rep, Req};
            let req = Req {
                realm_id: ops.device.user_realm_id,
                vlob_id: ops.device.user_realm_id,
                key_index: 0,
                timestamp,
                blob: ciphered,
                sequester_blob: sequester_blob.map(|v| v.into_iter().collect()),
            };
            let rep = ops.cmds.send(req).await?;
            match rep {
                Rep::Ok => Ok(UploadManifestOutcome::Success(to_sync_um)),
                Rep::VlobAlreadyExists { .. } => Ok(UploadManifestOutcome::VersionConflict),
                Rep::RequireGreaterTimestamp {
                    strictly_greater_than,
                } => {
                    timestamp =
                        std::cmp::max(strictly_greater_than, ops.device.time_provider.now());
                    continue;
                }
                // Timeout is about sequester service webhook not being available, no need
                // for a custom handling of such an exotic error
                Rep::SequesterServiceUnavailable => Err(UserSyncError::Offline),
                Rep::SequesterInconsistency { .. } => {
                    // Sequester services must have been concurrently modified in our back,
                    // update and retry
                    ops.certificates_ops
                        .poll_server_for_new_certificates(None)
                        .await
                        .map_err(|err| match err {
                            CertifPollServerError::Offline => UserSyncError::Offline,
                            CertifPollServerError::Stopped => UserSyncError::Stopped,
                            CertifPollServerError::InvalidCertificate(what) => {
                                UserSyncError::InvalidCertificate(what)
                            }
                            err @ CertifPollServerError::Internal(_) => UserSyncError::Internal(err.into()),
                        })?;
                    continue;
                }
                Rep::RejectedBySequesterService {
                    service_id,
                    reason,
                } => Err(UserSyncError::RejectedBySequesterService { service_id, reason }),
                Rep::TimestampOutOfBallpark {
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                    client_timestamp,
                    server_timestamp,
                } => Err(UserSyncError::TimestampOutOfBallpark{
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                    client_timestamp,
                    server_timestamp,
                }),
                // Unexpected responses
                bad_rep @ (
                    // A user realm is never shared, and hence it initial owner cannot lose access to it
                    Rep::AuthorNotAllowed
                    // `outbound_sync_inner` ensures the realm is created before calling us
                    | Rep::RealmNotFound
                    // We only encrypt for sequester services when the realm is sequestered
                    | Rep::OrganizationNotSequestered
                    // The user realm is never supposed to do key rotation
                    | Rep::BadKeyIndex { .. }
                    | Rep::UnknownStatus { .. }
                ) => Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into()),
            }
        } else {
            use authenticated_cmds::latest::vlob_update::{Rep, Req};
            let req = Req {
                vlob_id: ops.device.user_realm_id,
                version: to_sync_um.version,
                key_index: 0,
                timestamp,
                blob: ciphered,
                sequester_blob: sequester_blob.map(|v| v.into_iter().collect()),
            };
            let rep = ops.cmds.send(req).await?;
            match rep {
                Rep::Ok => Ok(UploadManifestOutcome::Success(to_sync_um)),
                Rep::BadVlobVersion => Ok(UploadManifestOutcome::VersionConflict),
                Rep::RequireGreaterTimestamp {
                    strictly_greater_than,
                } => {
                    timestamp =
                        std::cmp::max(strictly_greater_than, ops.device.time_provider.now());
                    continue;
                }
                // Timeout is about sequester service webhook not being available, no need
                // for a custom handling of such an exotic error
                Rep::SequesterServiceUnavailable => Err(UserSyncError::Offline),
                Rep::SequesterInconsistency { .. } => {
                    // Sequester services must have been concurrently modified in our back,
                    // update and retry
                    ops.certificates_ops
                        .poll_server_for_new_certificates(None)
                        .await
                        .map_err(|err| match err {
                            CertifPollServerError::Offline => UserSyncError::Offline,
                            CertifPollServerError::Stopped => UserSyncError::Stopped,
                            CertifPollServerError::InvalidCertificate(what) => {
                                UserSyncError::InvalidCertificate(what)
                            }
                            err @ CertifPollServerError::Internal(_) => UserSyncError::Internal(err.into()),
                        })?;
                    continue;
                }
                Rep::RejectedBySequesterService {
                    service_id,
                    reason,
                } => Err(UserSyncError::RejectedBySequesterService { service_id, reason }),
                Rep::TimestampOutOfBallpark {
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                    client_timestamp,
                    server_timestamp,
                } => Err(UserSyncError::TimestampOutOfBallpark{
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                    client_timestamp,
                    server_timestamp,
                }),
                // Unexpected responses
                bad_rep @ (
                    // A user realm is never shared, and hence it initial owner cannot lose access to it
                    Rep::AuthorNotAllowed
                    // The vlob must exists since we are at version > 1
                    | Rep::VlobNotFound
                    // We only encrypt for sequester services when the realm is sequestered
                    | Rep::OrganizationNotSequestered
                    // The user realm is never supposed to do key rotation
                    | Rep::BadKeyIndex { .. }
                    | Rep::UnknownStatus { .. }
                ) => Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into()),
            }
        };
    }
}

enum OutboundSyncOutcome {
    Done,
    InboundSyncNeeded,
}

async fn outbound_sync_inner(ops: &UserOps) -> Result<OutboundSyncOutcome, UserSyncError> {
    let base_um = ops.store.get_user_manifest();
    if !base_um.need_sync {
        return Ok(OutboundSyncOutcome::Done);
    }

    // If the user manifest's vlob has already been synced we know the realm exists,
    // otherwise we have to make sure it is the case !
    if base_um.base.version == 0 {
        ops.certificates_ops
            .ensure_realm_created(ops.device.user_realm_id)
            .await
            .map_err(|err| match err {
                CertifEnsureRealmCreatedError::Stopped => UserSyncError::Stopped,
                CertifEnsureRealmCreatedError::Offline => UserSyncError::Offline,
                CertifEnsureRealmCreatedError::TimestampOutOfBallpark {
                    server_timestamp,
                    client_timestamp,
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                } => UserSyncError::TimestampOutOfBallpark {
                    server_timestamp,
                    client_timestamp,
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                },
                CertifEnsureRealmCreatedError::Internal(err) => err
                    .context("Cannot create the workspace on the server")
                    .into(),
            })?;
    }

    // Note, unlike Parsec < v3.0, we don't have to ensure newly created workspaces
    // have been created on the server. This is because the reference to those
    // workspaces in the local user manifest (i.e. `local_workspaces` field)
    // is never synchronized.

    match upload_manifest(ops, &base_um).await? {
        UploadManifestOutcome::VersionConflict => Ok(OutboundSyncOutcome::InboundSyncNeeded),
        UploadManifestOutcome::Success(remote_um) => {
            // Merge back the manifest in local
            let (updater, local_um) = ops.store.for_update().await;
            // Final merge could have been achieved by a concurrent operation
            if let Some(merged_um) = super::merge::merge_local_user_manifests(&local_um, remote_um)
            {
                updater
                    .set_user_manifest(Arc::new(merged_um))
                    .await
                    .map_err(|e| match e {
                        UserStoreUpdateError::Stopped => UserSyncError::Stopped,
                        UserStoreUpdateError::Internal(err) => {
                            err.context("Cannot set user manifest").into()
                        }
                    })?;
            }

            Ok(OutboundSyncOutcome::Done)
        }
    }
}

/// Upload local changes to the server.
///
/// This also requires to download and merge any remote changes. Hence the
/// client is fully synchronized with the server once this function returns.
async fn outbound_sync(ops: &UserOps) -> Result<(), UserSyncError> {
    loop {
        match outbound_sync_inner(ops).await? {
            OutboundSyncOutcome::Done => return Ok(()),
            OutboundSyncOutcome::InboundSyncNeeded => {
                // Concurrency error, fetch and merge remote changes before
                // retrying the sync
                inbound_sync(ops).await?;
            }
        }
    }
}

async fn find_last_valid_manifest(
    ops: &UserOps,
    last_version: VersionInt,
) -> Result<UserManifest, UserSyncError> {
    let local_manifest = ops.store.get_user_manifest();
    let local_base_version = local_manifest.base.version;

    for candidate_version in (local_base_version + 1..last_version).rev() {
        match fetch_old_remote_user_manifest(ops, candidate_version).await {
            // Finally found a valid manifest !
            Ok(manifest) => {
                return Ok(manifest)
            },

            // Yet another invalid manifest, just skip it
            Err(FetchRemoteUserManifestError::InvalidManifest(_)) => continue,

            // Errors that prevent us from continuing :(

            Err(FetchRemoteUserManifestError::Stopped) => {
                return Err(UserSyncError::Stopped);
            }
            Err(FetchRemoteUserManifestError::Offline) => {
                return Err(UserSyncError::Offline)
            }
            Err(FetchRemoteUserManifestError::InvalidCertificate(err)) => {
                return Err(UserSyncError::InvalidCertificate(err))
            },
            // The version we sent was lower than the one of the invalid manifest previously
            // sent by the server, so this error should not occur in theory (unless the server
            // have just done a rollback, but this is very unlikely !)
            Err(FetchRemoteUserManifestError::BadVersion) => {
                return Err(UserSyncError::Internal(
                    anyhow::anyhow!(
                        "Server sent us vlob `{}` with version {} but now complains version {} we ask for doesn't exist",
                        ops.device.user_realm_id,
                        last_version,
                        candidate_version
                    )
                ))
            },
            Err(err @ FetchRemoteUserManifestError::Internal(_)) => {
                return Err(UserSyncError::Internal(err.into()))
            }
        }
    }

    // It seems the last valid manifest is the one we already have
    let manifest = local_manifest.base.clone();
    Ok(manifest)
}

/// Download and merge remote changes from the server.
///
/// If the client contains local changes, an outbound sync is still needed to
/// have the client fully synchronized with the server.
async fn inbound_sync(ops: &UserOps) -> Result<(), UserSyncError> {
    // Retrieve remote
    let target_um = match fetch_last_remote_user_manifest(ops).await {
        Ok(manifest) => manifest,

        // The last version of the manifest appear to be invalid (uploaded by
        // a buggy Parsec client ?), however we cannot just fail here otherwise
        // the system would be stuck for good !
        Err(FetchRemoteUserManifestError::InvalidManifest(err)) => {
            // Try to find the last valid version of the manifest and continue
            // from there

            let last_version = match *err {
                InvalidManifestError::Corrupted { version, .. } => version,
                InvalidManifestError::NonExistentKeyIndex { version, .. } => version,
                InvalidManifestError::CorruptedKey { version, .. } => version,
                InvalidManifestError::NonExistentAuthor { version, .. } => version,
                InvalidManifestError::RevokedAuthor { version, .. } => version,
                InvalidManifestError::AuthorRealmRoleCannotWrite { version, .. } => version,
                InvalidManifestError::AuthorNoAccessToRealm { version, .. } => version,
            };

            let mut manifest = find_last_valid_manifest(ops, last_version).await?;

            // If we return the manifest as-is, we won't be able to do outbound sync
            // given the server will always complain a more recent manifest exists
            // (i.e. the one that is corrupted !).
            // So we tweak this old manifest into pretending it is the corrupted one.
            // TODO: what if the server sends the manifest data to client A, but dummy
            // data to client B ? We would end up with clients not agreeing on what
            // contains a given version of the manifest...
            manifest.version = last_version;

            manifest
        }

        // We couldn't validate the manifest due to an invalid certificate
        // provided by the server. Hence there is nothing more we can do :(
        Err(FetchRemoteUserManifestError::InvalidCertificate(err)) => {
            return Err(UserSyncError::InvalidCertificate(err));
        }

        Err(FetchRemoteUserManifestError::Stopped) => {
            return Err(UserSyncError::Stopped);
        }

        Err(FetchRemoteUserManifestError::Offline) => {
            return Err(UserSyncError::Offline);
        }

        // We didn't specify a `version` argument in the request
        Err(FetchRemoteUserManifestError::BadVersion) => {
            unreachable!()
        }

        // D'Oh :/
        Err(err @ FetchRemoteUserManifestError::Internal(_)) => {
            return Err(UserSyncError::Internal(err.into()))
        }
    };

    // New things in remote, merge is needed
    let (updater, diverged_um) = ops.store.for_update().await;
    // Note merge may end up with nothing to sync, typically if the remote version is
    // already the one local is based on
    if let Some(merged_um) = super::merge::merge_local_user_manifests(&diverged_um, target_um) {
        updater
            .set_user_manifest(Arc::new(merged_um))
            .await
            .map_err(|e| match e {
                UserStoreUpdateError::Stopped => UserSyncError::Stopped,
                UserStoreUpdateError::Internal(err) => {
                    err.context("Cannot set user manifest").into()
                }
            })?;
    }

    Ok(())
}

async fn fetch_last_remote_user_manifest(
    ops: &UserOps,
) -> Result<UserManifest, FetchRemoteUserManifestError> {
    use authenticated_cmds::latest::vlob_read_batch::{Rep, Req};

    let req = Req {
        realm_id: ops.realm_id(),
        vlobs: vec![ops.device.user_realm_id],
        at: None,
    };

    let rep = ops.cmds.send(req).await?;

    let outcome = match &rep {
        Rep::Ok {items, needed_common_certificate_timestamp, needed_realm_certificate_timestamp} => {
            let (_, _, author, version, timestamp, blob) = items.first().ok_or_else(|| {
                FetchRemoteUserManifestError::Internal(anyhow::anyhow!("Unexpected server response (no vlob data): {:?}", rep))
            })?;
            ops.certificates_ops.validate_user_manifest(
                *needed_realm_certificate_timestamp,
                *needed_common_certificate_timestamp,
                author,
                *version,
                *timestamp,
                blob,
            ).await
        }
        // Unexpected errors :(
        rep @ (
            // We provided only a single item...
            Rep::TooManyElements |
            // User never loses access to its user manifest's workspace
            Rep::AuthorNotAllowed |
            // User manifest's vlob is supposed to exist !
            Rep::RealmNotFound { .. } |
            // Don't know what to do with this status :/
            Rep::UnknownStatus { .. }
        ) => {
            return Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into());
        }
    };

    outcome.map_err(|err| match err {
        CertifValidateManifestError::InvalidCertificate(err) => {
            FetchRemoteUserManifestError::InvalidCertificate(err)
        }
        CertifValidateManifestError::InvalidManifest(err) => {
            FetchRemoteUserManifestError::InvalidManifest(err)
        }
        CertifValidateManifestError::Offline => FetchRemoteUserManifestError::Offline,
        CertifValidateManifestError::Stopped => FetchRemoteUserManifestError::Stopped,
        err @ CertifValidateManifestError::Internal(_) => {
            FetchRemoteUserManifestError::Internal(err.into())
        }
        // Unexpected errors :(
        err @ (
            // User realm doesn't use keys bundle
            CertifValidateManifestError::InvalidKeysBundle(_)
            // User realm is never shared (hence initial owner can never lose it role)
            | CertifValidateManifestError::NotAllowed
        ) => {
            FetchRemoteUserManifestError::Internal(anyhow::anyhow!("Unexpected error: {:?}", err))
        }
    })
}

async fn fetch_old_remote_user_manifest(
    ops: &UserOps,
    version: VersionInt,
) -> Result<UserManifest, FetchRemoteUserManifestError> {
    use authenticated_cmds::latest::vlob_read_versions::{Rep, Req};

    let req = Req {
        realm_id: ops.realm_id(),
        items: [(ops.device.user_realm_id, version)].into(),
    };

    let rep = ops.cmds.send(req).await?;

    let outcome = match rep {
        Rep::Ok {items, needed_common_certificate_timestamp, needed_realm_certificate_timestamp} => {
            let (_, _, author, version, timestamp, blob) = items.first().ok_or_else(|| {
                FetchRemoteUserManifestError::BadVersion
            })?;
            ops.certificates_ops.validate_user_manifest(
                needed_realm_certificate_timestamp,
                needed_common_certificate_timestamp,
                author,
                *version,
                *timestamp,
                blob,
            ).await
        }
        // Unexpected errors :(
        rep @ (
            // We provided only a single item...
            Rep::TooManyElements |
            // User never loses access to its user manifest's workspace
            Rep::AuthorNotAllowed |
            // User manifest's vlob is supposed to exist !
            Rep::RealmNotFound { .. } |
            // Don't know what to do with this status :/
            Rep::UnknownStatus { .. }
        ) => {
            return Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into());
        }
    };

    outcome.map_err(|err| match err {
        CertifValidateManifestError::InvalidCertificate(err) => {
            FetchRemoteUserManifestError::InvalidCertificate(err)
        }
        CertifValidateManifestError::InvalidManifest(err) => {
            FetchRemoteUserManifestError::InvalidManifest(err)
        }
        CertifValidateManifestError::Offline => FetchRemoteUserManifestError::Offline,
        CertifValidateManifestError::Stopped => FetchRemoteUserManifestError::Stopped,
        err @ CertifValidateManifestError::Internal(_) => {
            FetchRemoteUserManifestError::Internal(err.into())
        }
        // Unexpected errors :(
        err @ (
            // User realm doesn't use keys bundle
            CertifValidateManifestError::InvalidKeysBundle(_)
            // User realm is never shared (hence initial owner can never lose it role)
            | CertifValidateManifestError::NotAllowed
        ) => {
            FetchRemoteUserManifestError::Internal(anyhow::anyhow!("Unexpected error: {:?}", err))
        }
    })
}
