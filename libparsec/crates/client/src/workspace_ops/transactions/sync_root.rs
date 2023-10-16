// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::{protocol::authenticated_cmds, ConnectionError};
use libparsec_types::prelude::*;

use super::super::WorkspaceOps;
use crate::certificates_ops::{
    InvalidCertificateError, InvalidManifestError, PollServerError, ValidateManifestError,
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
pub enum FetchRemoteManifestError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Server has no such version for this manifest")]
    BadVersion,
    #[error(transparent)]
    InvalidCertificate(#[from] InvalidCertificateError),
    #[error(transparent)]
    InvalidManifest(#[from] InvalidManifestError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for FetchRemoteManifestError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

pub async fn sync_root(ops: &WorkspaceOps) -> Result<(), SyncError> {
    let workspace_manifest = ops.data_storage.get_workspace_manifest();

    if workspace_manifest.need_sync {
        outbound_sync_root(ops).await?;
        // TODO: event
        // ops.event_bus.send(&EventWorkspaceOpsSynced);
        Ok(())
    } else {
        inbound_sync_root(ops).await
    }
}

enum UploadManifestOutcome {
    Success(WorkspaceManifest),
    VersionConflict,
}

async fn upload_manifest(
    ops: &WorkspaceOps,
    base_wm: &LocalWorkspaceManifest,
) -> Result<UploadManifestOutcome, SyncError> {
    let mut timestamp = ops.device.now();

    loop {
        // Build vlob
        let to_sync_wm = base_wm.to_remote(ops.device.device_id.to_owned(), timestamp);

        let signed = to_sync_wm.dump_and_sign(&ops.device.signing_key);
        let ciphered = ops.device.user_realm_key.encrypt(&signed).into();
        let sequester_blob = ops
            .certificates_ops
            .encrypt_for_sequester_services(&signed)
            .await?;

        // Sync the vlob with server

        return if to_sync_wm.version == 1 {
            use authenticated_cmds::latest::vlob_create::{Rep, Req};
            let req = Req {
                realm_id: ops.realm_id,
                // Always 1 given user manifest realm is never reencrypted
                encryption_revision: 1,
                vlob_id: ops.realm_id,
                timestamp,
                blob: ciphered,
                sequester_blob,
            };
            let rep = ops.cmds.send(req).await?;
            match rep {
                Rep::Ok => Ok(UploadManifestOutcome::Success(to_sync_wm)),
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
                vlob_id: ops.realm_id,
                version: to_sync_wm.version,
                timestamp,
                blob: ciphered,
                sequester_blob,
            };
            let rep = ops.cmds.send(req).await?;
            match rep {
                Rep::Ok => Ok(UploadManifestOutcome::Success(to_sync_wm)),
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

async fn outbound_sync_root_inner(ops: &WorkspaceOps) -> Result<OutboundSyncOutcome, SyncError> {
    let base_wm = ops.data_storage.get_workspace_manifest();
    if !base_wm.need_sync {
        return Ok(OutboundSyncOutcome::Done);
    }

    // If the manifest's vlob has already been synced we know the realm exists,
    // otherwise we have to make sure it is the case !
    if base_wm.base.version == 0 {
        ops.certificates_ops
            .ensure_realms_created(&[ops.realm_id])
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

    match upload_manifest(ops, &base_wm).await? {
        UploadManifestOutcome::VersionConflict => Ok(OutboundSyncOutcome::InboundSyncNeeded),
        UploadManifestOutcome::Success(remote_wm) => {
            // Merge back the manifest in local
            let (updater, local_wm) = ops.data_storage.for_update_workspace_manifest().await;
            // Final merge could have been achieved by a concurrent operation
            if let Some(merged_wm) = super::super::merge::merge_local_workspace_manifests(
                &ops.device.device_id,
                ops.device.now(),
                &local_wm,
                remote_wm,
            ) {
                updater
                    .update_workspace_manifest(Arc::new(merged_wm))
                    .await?;
            }

            Ok(OutboundSyncOutcome::Done)
        }
    }
}

/// Upload local changes to the server.
///
/// This also requires to download and merge any remote changes. Hence the
/// client is fully synchronized with the server once this function returns.
async fn outbound_sync_root(ops: &WorkspaceOps) -> Result<(), SyncError> {
    loop {
        match outbound_sync_root_inner(ops).await? {
            OutboundSyncOutcome::Done => return Ok(()),
            OutboundSyncOutcome::InboundSyncNeeded => {
                // Concurrency error, fetch and merge remote changes before
                // retrying the sync
                inbound_sync_root(ops).await?;
            }
        }
    }
}

async fn find_last_valid_manifest(
    ops: &WorkspaceOps,
    last_version: VersionInt,
) -> Result<WorkspaceManifest, SyncError> {
    let local_manifest = ops.data_storage.get_workspace_manifest();
    let local_base_version = local_manifest.base.version;

    for candidate_version in (local_base_version + 1..last_version).rev() {
        match fetch_remote_workspace_manifest(ops, Some(candidate_version)).await {
            // Finally found a valid manifest !
            Ok(manifest) => {
                return Ok(manifest)
            },

            // Yet another invalid manifest, just skip it
            Err(FetchRemoteManifestError::InvalidManifest(_)) => continue,

            // Errors that prevent us from continuing :(

            Err(FetchRemoteManifestError::Offline) => {
                return Err(SyncError::Offline)
            }
            Err(FetchRemoteManifestError::InvalidCertificate(err)) => {
                return Err(SyncError::InvalidCertificate(err))
            },
            // The version we sent was lower than the one of the invalid manifest previously
            // sent by the server, so this error should not occur in theory (unless the server
            // have just done a rollback, but this is very unlikely !)
            Err(FetchRemoteManifestError::BadVersion) => {
                return Err(SyncError::Internal(
                    anyhow::anyhow!(
                        "Server sent us vlob `{}` with version {} but now complains version {} we ask for doesn't exist",
                        ops.device.user_realm_id,
                        last_version,
                        candidate_version
                    )
                ))
            },
            Err(err @ FetchRemoteManifestError::Internal(_)) => {
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
async fn inbound_sync_root(ops: &WorkspaceOps) -> Result<(), SyncError> {
    // Retrieve remote
    let remote_wm = match fetch_remote_workspace_manifest(ops, None).await {
        Ok(manifest) => manifest,

        // The last version of the manifest appear to be invalid (uploaded by
        // a buggy Parsec client ?), however we cannot just fail here otherwise
        // the system would be stuck for good !
        Err(FetchRemoteManifestError::InvalidManifest(err)) => {
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
        Err(FetchRemoteManifestError::InvalidCertificate(err)) => {
            return Err(SyncError::InvalidCertificate(err));
        }

        Err(FetchRemoteManifestError::Offline) => {
            return Err(SyncError::Offline);
        }

        // We didn't specified a `version` argument in the request
        Err(FetchRemoteManifestError::BadVersion) => {
            unreachable!()
        }

        // D'Oh :/
        Err(err @ FetchRemoteManifestError::Internal(_)) => {
            return Err(SyncError::Internal(err.into()))
        }
    };

    // New things in remote, merge is needed
    let (updater, local_wm) = ops.data_storage.for_update_workspace_manifest().await;
    // Note merge may end up with nothing to sync, typically if the remote version is
    // already the one local is based on
    if let Some(merged_wm) = super::super::merge::merge_local_workspace_manifests(
        &ops.device.device_id,
        ops.device.now(),
        &local_wm,
        remote_wm,
    ) {
        updater
            .update_workspace_manifest(Arc::new(merged_wm))
            .await?;
    }

    // TODO: we used to send a SHARING_UPDATED event here, however it would simpler
    // to send such event from the CertificatesOps (i.e. when a realm role certificate
    // is added). The downside of this approach is we don't have the guarantee that
    // WorkspaceOps have processed the changes (e.g. GUI receive an event about an new
    // workspace shared with us, but got an error when trying to get the workspace
    // from WorkspaceOps...)

    Ok(())
}

async fn fetch_remote_workspace_manifest(
    ops: &WorkspaceOps,
    version: Option<VersionInt>,
) -> Result<WorkspaceManifest, FetchRemoteManifestError> {
    use authenticated_cmds::latest::vlob_read::{Rep, Req};

    let req = Req {
        // `encryption_revision` is always 1 given we never re-encrypt the user manifest's realm
        encryption_revision: 1,
        timestamp: None,
        version,
        vlob_id: ops.realm_id,
    };

    let rep = ops.cmds.send(req).await?;

    let outcome = match rep {
        Rep::Ok { certificate_index, author: expected_author, version: version_according_to_server, timestamp: expected_timestamp, blob } => {
            let expected_version = match version {
                Some(version) => version,
                None => version_according_to_server,
            };
            let realm_key = {
                let config = ops.user_dependant_config.lock().expect("Mutex is poisoned");
                config.realm_key.clone()
            };
            ops.certificates_ops.validate_workspace_manifest(
                ops.realm_id,
                &realm_key,
                certificate_index, &expected_author, expected_version, expected_timestamp, &blob
            ).await
        }
        // Expected errors
        Rep::BadVersion if version.is_some() => {
            return Err(FetchRemoteManifestError::BadVersion);
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
            FetchRemoteManifestError::InvalidCertificate(err)
        }
        ValidateManifestError::InvalidManifest(err) => {
            FetchRemoteManifestError::InvalidManifest(err)
        }
        ValidateManifestError::Offline => FetchRemoteManifestError::Offline,
        err @ ValidateManifestError::Internal(_) => FetchRemoteManifestError::Internal(err.into()),
    })
}
