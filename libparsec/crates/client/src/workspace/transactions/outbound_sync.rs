// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::protocol::authenticated_cmds;
use libparsec_platform_storage::workspace::GetChildManifestError;
use libparsec_types::prelude::*;

use super::super::WorkspaceOps;
use super::{
    inbound_sync, update_storage_with_remote_child, update_storage_with_remote_root, SyncError,
};
use crate::certificates_ops::PollServerError;

pub async fn get_need_outbound_sync(ops: &WorkspaceOps) -> anyhow::Result<Vec<VlobID>> {
    let need_sync = ops.data_storage.get_need_sync_entries().await?;
    Ok(need_sync.local)
}

pub async fn outbound_sync(ops: &WorkspaceOps, entry_id: VlobID) -> Result<(), SyncError> {
    loop {
        let outcome = if entry_id == ops.realm_id {
            outbound_sync_root(ops).await?
        } else {
            outbound_sync_child(ops, entry_id).await?
        };

        match outcome {
            OutboundSyncOutcome::Done => return Ok(()),
            OutboundSyncOutcome::InboundSyncNeeded => {
                // Concurrency error, fetch and merge remote changes before
                // retrying the sync
                inbound_sync(ops, ops.realm_id).await?;
            }
        }
    }
    // TODO: send outbound sync event ?
}

enum OutboundSyncOutcome {
    Done,
    InboundSyncNeeded,
}

async fn outbound_sync_root(ops: &WorkspaceOps) -> Result<OutboundSyncOutcome, SyncError> {
    let local = ops.data_storage.get_workspace_manifest();
    if !local.need_sync {
        return Ok(OutboundSyncOutcome::Done);
    }

    // If the manifest's vlob has already been synced we know the realm exists,
    // otherwise we have to make sure it is the case !
    if local.base.version == 0 {
        ops.certificates_ops
            .ensure_realms_created(&[ops.realm_id])
            .await
            .map_err(|err| match err {
                crate::certificates_ops::EnsureRealmsCreatedError::Stopped => SyncError::Stopped,
                crate::certificates_ops::EnsureRealmsCreatedError::Offline => SyncError::Offline,
                crate::certificates_ops::EnsureRealmsCreatedError::TimestampOutOfBallpark {
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

    let remote = match upload_manifest(ops, &local).await? {
        UploadManifestOutcome::VersionConflict => {
            return Ok(OutboundSyncOutcome::InboundSyncNeeded)
        }
        UploadManifestOutcome::Success(remote) => remote,
    };

    // The manifest has been updated on the server ! Now merge it back into the current
    // local manifest that may have been concurrently modified in the meantime.
    update_storage_with_remote_root(ops, remote).await?;

    Ok(OutboundSyncOutcome::Done)
}

async fn outbound_sync_child(
    ops: &WorkspaceOps,
    entry_id: VlobID,
) -> Result<OutboundSyncOutcome, SyncError> {
    let local = match ops.data_storage.get_child_manifest(entry_id).await {
        Ok(local) => local,
        // If the entry cannot be found locally, then there is nothing to sync !
        Err(GetChildManifestError::NotFound) => return Ok(OutboundSyncOutcome::Done),
        Err(GetChildManifestError::Internal(err)) => return Err(err.into()),
    };
    let (need_sync, base_version) = match &local {
        ArcLocalChildManifest::File(m) => (m.need_sync, m.base.version),
        ArcLocalChildManifest::Folder(m) => (m.need_sync, m.base.version),
    };
    if !need_sync {
        return Ok(OutboundSyncOutcome::Done);
    }

    // If the manifest's vlob has already been synced we know the realm exists,
    // otherwise we have to make sure it is the case !
    if base_version == 0 {
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

    let remote: ChildManifest = match upload_manifest(ops, &local).await? {
        UploadManifestOutcome::VersionConflict => {
            return Ok(OutboundSyncOutcome::InboundSyncNeeded)
        }
        UploadManifestOutcome::Success(remote) => remote,
    };

    // The manifest has been updated on the server ! Now merge it back into the current
    // local manifest that may have been concurrently modified in the meantime.
    update_storage_with_remote_child(ops, remote).await?;

    Ok(OutboundSyncOutcome::Done)
}

trait RemoteManifest: Sized {
    type LocalManifest: Sized;

    fn from_local(base: &Self::LocalManifest, device_id: DeviceID, timestamp: DateTime) -> Self;

    fn version(&self) -> VersionInt;

    fn id(&self) -> VlobID;

    fn dump_and_sign(&self, key: &SigningKey) -> Vec<u8>;
}

impl RemoteManifest for WorkspaceManifest {
    type LocalManifest = Arc<LocalWorkspaceManifest>;

    fn from_local(base: &Self::LocalManifest, author: DeviceID, timestamp: DateTime) -> Self {
        base.to_remote(author, timestamp)
    }

    fn version(&self) -> VersionInt {
        self.version
    }

    fn id(&self) -> VlobID {
        self.id
    }

    fn dump_and_sign(&self, author_signkey: &SigningKey) -> Vec<u8> {
        self.dump_and_sign(author_signkey)
    }
}

impl RemoteManifest for ChildManifest {
    type LocalManifest = ArcLocalChildManifest;

    fn from_local(base: &Self::LocalManifest, author: DeviceID, timestamp: DateTime) -> Self {
        match base {
            ArcLocalChildManifest::File(_) => {
                todo!()
                // TODO: must handle reshape
                // Self::File(m.to_remote(author, timestamp))
            }
            ArcLocalChildManifest::Folder(m) => Self::Folder(m.to_remote(author, timestamp)),
        }
    }

    fn version(&self) -> VersionInt {
        match self {
            ChildManifest::File(m) => m.version,
            ChildManifest::Folder(m) => m.version,
        }
    }

    fn id(&self) -> VlobID {
        match self {
            ChildManifest::File(m) => m.id,
            ChildManifest::Folder(m) => m.id,
        }
    }

    fn dump_and_sign(&self, author_signkey: &SigningKey) -> Vec<u8> {
        match self {
            ChildManifest::File(m) => m.dump_and_sign(author_signkey),
            ChildManifest::Folder(m) => m.dump_and_sign(author_signkey),
        }
    }
}

enum UploadManifestOutcome<M> {
    Success(M),
    VersionConflict,
}

async fn upload_manifest<M: RemoteManifest>(
    ops: &WorkspaceOps,
    local_manifest: &M::LocalManifest,
) -> Result<UploadManifestOutcome<M>, SyncError> {
    let mut timestamp = ops.device.now();

    loop {
        // Build vlob
        let to_sync = M::from_local(local_manifest, ops.device.device_id.to_owned(), timestamp);

        let signed = to_sync.dump_and_sign(&ops.device.signing_key);
        let ciphered = ops.device.user_realm_key.encrypt(&signed).into();
        let sequester_blob = ops
            .certificates_ops
            .encrypt_for_sequester_services(&signed)
            .await?;

        // Sync the vlob with server

        return if to_sync.version() == 1 {
            use authenticated_cmds::latest::vlob_create::{Rep, Req};
            let req = Req {
                realm_id: ops.realm_id,
                // TODO: Damn you encryption_revision ! Your days are numbered !!!!
                encryption_revision: 1,
                vlob_id: to_sync.id(),
                timestamp,
                blob: ciphered,
                sequester_blob,
            };
            let rep = ops.cmds.send(req).await?;
            match rep {
                Rep::Ok => Ok(UploadManifestOutcome::Success(to_sync)),
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
                vlob_id: to_sync.id(),
                version: to_sync.version(),
                timestamp,
                blob: ciphered,
                sequester_blob,
            };
            let rep = ops.cmds.send(req).await?;
            match rep {
                Rep::Ok => Ok(UploadManifestOutcome::Success(to_sync)),
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
