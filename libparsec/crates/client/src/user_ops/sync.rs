// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::{protocol::authenticated_cmds, ConnectionError};
use libparsec_types::prelude::*;

use super::UserOps;
use crate::{
    certificates_ops::{
        InvalidCertificateError, InvalidManifestError, PollServerError, ValidateManifestError,
    },
    EventUserOpsSynced,
};

#[derive(Debug, thiserror::Error)]
pub enum SyncError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Cannot sync during realm maintenance")]
    InMaintenance,
    #[error(transparent)]
    InvalidCertificate(#[from] InvalidCertificateError),
    // Note `InvalidManifest` here, this is because we self-repair in case of invalid
    // user manifest (given otherwise the client would be stuck for good !)
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    BadTimestamp {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for SyncError {
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
    #[error("Server has no such version for this user manifest")]
    BadVersion,
    #[error(transparent)]
    InvalidCertificate(#[from] InvalidCertificateError),
    #[error(transparent)]
    InvalidManifest(#[from] InvalidManifestError),
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

pub async fn sync(ops: &UserOps) -> Result<(), SyncError> {
    let user_manifest = ops.storage.get_user_manifest();

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
) -> Result<UploadManifestOutcome, SyncError> {
    let mut timestamp = ops.device.now();

    loop {
        // Build vlob
        let to_sync_um = base_um.to_remote(ops.device.device_id.to_owned(), timestamp);

        let signed = to_sync_um.dump_and_sign(&ops.device.signing_key);
        let ciphered = ops.device.user_realm_key.encrypt(&signed).into();
        let sequester_blob = ops
            .certificates_ops
            .encrypt_for_sequester_services(&signed)
            .await?;

        // Sync the vlob with server

        return if to_sync_um.version == 1 {
            use authenticated_cmds::latest::vlob_create::{Rep, Req};
            let req = Req {
                realm_id: ops.device.user_realm_id,
                // Always 1 given user manifest realm is never reencrypted
                encryption_revision: 1,
                vlob_id: ops.device.user_realm_id,
                timestamp,
                blob: ciphered,
                sequester_blob,
            };
            let rep = ops.cmds.send(req).await?;
            match rep {
                Rep::Ok => Ok(UploadManifestOutcome::Success(to_sync_um)),
                Rep::AlreadyExists { .. } => Ok(UploadManifestOutcome::VersionConflict),
                Rep::InMaintenance => Err(SyncError::InMaintenance),
                Rep::RequireGreaterTimestamp {
                    strictly_greater_than,
                } => {
                    timestamp =
                        std::cmp::max(strictly_greater_than, ops.device.time_provider.now());
                    continue;
                }
                // Timeout is about sequester service webhook not being available, no need
                // for a custom handling of such an exotic error
                Rep::Timeout => Err(SyncError::Offline),
                Rep::SequesterInconsistency { .. } => {
                    // Sequester services must have been concurrently modified in our back,
                    // update and retry
                    ops.certificates_ops
                        .poll_server_for_new_certificates(None)
                        .await
                        .map_err(|err| match err {
                            PollServerError::Offline => SyncError::Offline,
                            PollServerError::InvalidCertificate(what) => {
                                SyncError::InvalidCertificate(what)
                            }
                            err @ PollServerError::Internal(_) => SyncError::Internal(err.into()),
                        })?;
                    continue;
                }
                Rep::RejectedBySequesterService {
                    service_id: _service_id,
                    service_label: _service_label,
                    ..
                } => todo!(),
                Rep::NotAllowed => todo!(),
                Rep::BadEncryptionRevision => todo!(),
                Rep::BadTimestamp { .. } => todo!(),
                Rep::NotASequesteredOrganization => todo!(),
                bad_rep @ Rep::UnknownStatus { .. } => {
                    Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
                }
            }
        } else {
            use authenticated_cmds::latest::vlob_update::{Rep, Req};
            let req = Req {
                // Always 1 given user manifest realm is never reencrypted
                encryption_revision: 1,
                vlob_id: ops.device.user_realm_id,
                version: to_sync_um.version,
                timestamp,
                blob: ciphered,
                sequester_blob,
            };
            let rep = ops.cmds.send(req).await?;
            match rep {
                Rep::Ok => Ok(UploadManifestOutcome::Success(to_sync_um)),
                Rep::BadVersion => Ok(UploadManifestOutcome::VersionConflict),
                Rep::InMaintenance => Err(SyncError::InMaintenance),
                Rep::RequireGreaterTimestamp {
                    strictly_greater_than,
                } => {
                    timestamp =
                        std::cmp::max(strictly_greater_than, ops.device.time_provider.now());
                    continue;
                }
                // Timeout is about sequester service webhook not being available, no need
                // for a custom handling of such an exotic error
                Rep::Timeout => Err(SyncError::Offline),
                Rep::SequesterInconsistency { .. } => {
                    // Sequester services must have been concurrently modified in our back,
                    // update and retry
                    ops.certificates_ops
                        .poll_server_for_new_certificates(None)
                        .await
                        .map_err(|err| match err {
                            PollServerError::Offline => SyncError::Offline,
                            PollServerError::InvalidCertificate(what) => {
                                SyncError::InvalidCertificate(what)
                            }
                            err @ PollServerError::Internal(_) => SyncError::Internal(err.into()),
                        })?;
                    continue;
                }
                // TODO: what do to here ?
                Rep::RejectedBySequesterService {
                    service_id: _service_id,
                    service_label: _service_label,
                    ..
                } => todo!(),
                // TODO: error handling !
                Rep::NotFound { .. } => todo!(),
                Rep::NotAllowed => todo!(),
                Rep::BadEncryptionRevision => todo!(),
                Rep::BadTimestamp { .. } => todo!(),
                Rep::NotASequesteredOrganization => todo!(),
                bad_rep @ Rep::UnknownStatus { .. } => {
                    Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into())
                }
            }
        };
    }
}

enum OutboundSyncOutcome {
    Done,
    InboundSyncNeeded,
}

async fn outbound_sync_inner(ops: &UserOps) -> Result<OutboundSyncOutcome, SyncError> {
    let base_um = ops.storage.get_user_manifest();
    if !base_um.need_sync {
        return Ok(OutboundSyncOutcome::Done);
    }

    // If the user manifest's vlob has already been synced we know the realm exists,
    // otherwise we have to make sure it is the case !
    if base_um.base.version == 0 {
        ops.certificates_ops
            .ensure_realms_created(&[ops.device.user_realm_id])
            .await
            .map_err(|err| match err {
                crate::certificates_ops::EnsureRealmsCreatedError::Offline => SyncError::Offline,
                crate::certificates_ops::EnsureRealmsCreatedError::BadTimestamp {
                    server_timestamp,
                    client_timestamp,
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                } => SyncError::BadTimestamp {
                    server_timestamp,
                    client_timestamp,
                    ballpark_client_early_offset,
                    ballpark_client_late_offset,
                },
                crate::certificates_ops::EnsureRealmsCreatedError::Internal(err) => err
                    .context("Cannot create the workspace on the server")
                    .into(),
            })?;
    }

    // In we have just created a workspace, it's possible it corresponding realm
    // hasn't been created yet on the server...
    let workspaces_ids: Vec<_> = base_um.workspaces.iter().map(|e| e.id).collect();
    ops.certificates_ops
        .ensure_realms_created(&workspaces_ids)
        .await
        .map_err(|err| match err {
            crate::certificates_ops::EnsureRealmsCreatedError::Offline => SyncError::Offline,
            crate::certificates_ops::EnsureRealmsCreatedError::BadTimestamp {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            } => SyncError::BadTimestamp {
                server_timestamp,
                client_timestamp,
                ballpark_client_early_offset,
                ballpark_client_late_offset,
            },
            crate::certificates_ops::EnsureRealmsCreatedError::Internal(err) => err
                .context("Cannot create the workspace on the server")
                .into(),
        })?;

    match upload_manifest(ops, &base_um).await? {
        UploadManifestOutcome::VersionConflict => Ok(OutboundSyncOutcome::InboundSyncNeeded),
        UploadManifestOutcome::Success(synced_um) => {
            // Merge back the manifest in local
            let (updater, diverged_um) = ops.storage.for_update().await;
            // Final merge could have been achieved by a concurrent operation
            if synced_um.version > diverged_um.base.version {
                let merged_um = super::merge::merge_local_user_manifests(&diverged_um, synced_um);
                updater.set_user_manifest(Arc::new(merged_um)).await?;
            }

            Ok(OutboundSyncOutcome::Done)
        }
    }
}

/// Upload local changes to the server.
///
/// This also requires to download and merge any remote changes. Hence the
/// client is fully synchronized with the server once this function returns.
async fn outbound_sync(ops: &UserOps) -> Result<(), SyncError> {
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
) -> Result<UserManifest, SyncError> {
    let local_manifest = ops.storage.get_user_manifest();
    let local_base_version = local_manifest.base.version;

    for candidate_version in (local_base_version + 1..last_version).rev() {
        match fetch_remote_user_manifest(ops, Some(candidate_version)).await {
            // Finally found a valid manifest !
            Ok(manifest) => {
                return Ok(manifest)
            },

            // Yet another invalid manifest, just skip it
            Err(FetchRemoteUserManifestError::InvalidManifest(_)) => continue,

            // Errors that prevent us from continuing :(

            Err(FetchRemoteUserManifestError::Offline) => {
                return Err(SyncError::Offline)
            }
            Err(FetchRemoteUserManifestError::InvalidCertificate(err)) => {
                return Err(SyncError::InvalidCertificate(err))
            },
            // The version we sent was lower than the one of the invalid manifest previously
            // sent by the server, so this error should not occur in theory (unless the server
            // have just done a rollback, but this is very unlikely !)
            Err(FetchRemoteUserManifestError::BadVersion) => {
                return Err(SyncError::Internal(
                    anyhow::anyhow!(
                        "Server sent us vlob `{}` with version {} but now complains version {} we ask for doesn't exist",
                        ops.device.user_realm_id,
                        last_version,
                        candidate_version
                    )
                ))
            },
            Err(err @ FetchRemoteUserManifestError::Internal(_)) => {
                return Err(SyncError::Internal(err.into()))
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
async fn inbound_sync(ops: &UserOps) -> Result<(), SyncError> {
    // Retrieve remote
    let target_um = match fetch_remote_user_manifest(ops, None).await {
        Ok(manifest) => manifest,

        // The last version of the manifest appear to be invalid (uploaded by
        // a buggy Parsec client ?), however we cannot just fail here otherwise
        // the system would be stuck for good !
        Err(FetchRemoteUserManifestError::InvalidManifest(err)) => {
            // Try to find the last valid version of the manifest and continue
            // from there

            let last_version = match err {
                InvalidManifestError::Corrupted { version, .. } => version,
                InvalidManifestError::NonExistantAuthor { version, .. } => version,
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
            return Err(SyncError::InvalidCertificate(err));
        }

        Err(FetchRemoteUserManifestError::Offline) => {
            return Err(SyncError::Offline);
        }

        // We didn't specified a `version` argument in the request
        Err(FetchRemoteUserManifestError::BadVersion) => {
            unreachable!()
        }

        // D'Oh :/
        Err(err @ FetchRemoteUserManifestError::Internal(_)) => {
            return Err(SyncError::Internal(err.into()))
        }
    };

    let diverged_um = ops.storage.get_user_manifest();
    if target_um.version == diverged_um.base.version {
        // Nothing new
        return Ok(());
    }

    // New things in remote, merge is needed
    let (updater, diverged_um) = ops.storage.for_update().await;
    if target_um.version == diverged_um.base.version {
        // Sync already achieved by a concurrent operation
        return Ok(());
    }

    let merged_um = super::merge::merge_local_user_manifests(&diverged_um, target_um);
    updater.set_user_manifest(Arc::new(merged_um)).await?;

    // TODO: we used to send a SHARING_UPDATED event here, however it would simpler
    // to send such event from the CertificatesOps (i.e. when a realm role certificate
    // is added). The downside of this approach is we don't have the guarantee that
    // UserOps have processed the changes (e.g. GUI receive an event about an new
    // workspace shared with us, but got an error when trying to get the workspace
    // from UserOps...)

    Ok(())
}

async fn fetch_remote_user_manifest(
    ops: &UserOps,
    version: Option<VersionInt>,
) -> Result<UserManifest, FetchRemoteUserManifestError> {
    use authenticated_cmds::latest::vlob_read::{Rep, Req};

    let req = Req {
        // `encryption_revision` is always 1 given we never re-encrypt the user manifest's realm
        encryption_revision: 1,
        timestamp: None,
        version,
        vlob_id: VlobID::from(ops.device.user_realm_id.as_ref().to_owned()),
    };

    let rep = ops.cmds.send(req).await?;

    let outcome = match rep {
        Rep::Ok { certificate_index, author: expected_author, version: version_according_to_server, timestamp: expected_timestamp, blob } => {
            let expected_version = match version {
                Some(version) => version,
                None => version_according_to_server,
            };
            ops.certificates_ops.validate_user_manifest(certificate_index, &expected_author, expected_version, expected_timestamp, &blob).await
        }
        // Expected errors
        Rep::BadVersion if version.is_some() => {
            return Err(FetchRemoteUserManifestError::BadVersion);
        }
        // Unexpected errors :(
        rep @ (
            // We didn't specified a `version` argument in the request
            Rep::BadVersion |
            // User never loses access to it user manifest's workspace
            Rep::NotAllowed |
            // User manifest's workspace never gets reencrypted !
            Rep::InMaintenance |
            Rep::BadEncryptionRevision |
            // User manifest's vlob is supposed to exists !
            Rep::NotFound { .. } |
            // Don't know what to do with this status :/
            Rep::UnknownStatus { .. }
        ) => {
            return Err(anyhow::anyhow!("Unexpected server response: {:?}", rep).into());
        }
    };

    outcome.map_err(|err| match err {
        ValidateManifestError::InvalidCertificate(err) => {
            FetchRemoteUserManifestError::InvalidCertificate(err)
        }
        ValidateManifestError::InvalidManifest(err) => {
            FetchRemoteUserManifestError::InvalidManifest(err)
        }
        ValidateManifestError::Offline => FetchRemoteUserManifestError::Offline,
        err @ ValidateManifestError::Internal(_) => {
            FetchRemoteUserManifestError::Internal(err.into())
        }
    })
}
