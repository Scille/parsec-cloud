// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::{
    certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError},
    workspace::{
        store::{
            FolderishManifestAndUpdater, FsPathResolutionAndManifest, GetEntryError,
            GetFolderishEntryError, UpdateFolderManifestError, UpdateWorkspaceManifestError,
        },
        WorkspaceOps,
    },
    EventWorkspaceOpsOutboundSyncNeeded,
};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceCopyEntryError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("Path doesn't exist")]
    EntryNotFound,
    #[error("Only have read access on this workspace")]
    ReadOnlyRealm,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error("Destination already exists")]
    DestinationExists { entry_id: VlobID },
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidManifest(#[from] Box<InvalidManifestError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for WorkspaceCopyEntryError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

pub(crate) async fn copy_entry(
    ops: &WorkspaceOps,
    src: FsPath,
    dst: FsPath,
    overwrite: bool,
) -> Result<(), WorkspaceCopyEntryError> {
    if !ops
        .workspace_entry
        .lock()
        .expect("Mutex is poisoned")
        .role
        .can_write()
    {
        return Err(WorkspaceCopyEntryError::ReadOnlyRealm);
    }

    // Do nothing if source and destination are the same !
    if src == dst {
        return Ok(());
    }

    let src_resolution = ops
        .store
        .resolve_path_and_get_manifest(&src)
        .await
        .map_err(|err| match err {
            GetEntryError::Offline => WorkspaceCopyEntryError::Offline,
            GetEntryError::Stopped => WorkspaceCopyEntryError::Stopped,
            GetEntryError::EntryNotFound => WorkspaceCopyEntryError::EntryNotFound,
            GetEntryError::NoRealmAccess => WorkspaceCopyEntryError::NoRealmAccess,
            GetEntryError::InvalidKeysBundle(err) => {
                WorkspaceCopyEntryError::InvalidKeysBundle(err)
            }
            GetEntryError::InvalidCertificate(err) => {
                WorkspaceCopyEntryError::InvalidCertificate(err)
            }
            GetEntryError::InvalidManifest(err) => WorkspaceCopyEntryError::InvalidManifest(err),
            GetEntryError::Internal(err) => err.context("cannot resolve path").into(),
        })?;

    let (dst_parent, dst_name) = dst.into_parent();
    // Cannot rename root !
    let dst_name = match dst_name {
        None => return Err(WorkspaceRenameEntryError::CannotRenameRoot),
        Some(name) => name,
    };

    let dst_parent_resolution = ops
        .store
        .resolve_path_for_update_folderish_manifest(&dst_parent)
        .await
        .map_err(|err| match err {
            GetFolderishEntryError::Offline => WorkspaceCopyEntryError::Offline,
            GetFolderishEntryError::Stopped => WorkspaceCopyEntryError::Stopped,
            GetFolderishEntryError::EntryNotFound => WorkspaceCopyEntryError::EntryNotFound,
            GetFolderishEntryError::EntryIsFile => WorkspaceCopyEntryError::EntryNotFound,
            GetFolderishEntryError::NoRealmAccess => WorkspaceCopyEntryError::NoRealmAccess,
            GetFolderishEntryError::InvalidKeysBundle(err) => {
                WorkspaceCopyEntryError::InvalidKeysBundle(err)
            }
            GetFolderishEntryError::InvalidCertificate(err) => {
                WorkspaceCopyEntryError::InvalidCertificate(err)
            }
            GetFolderishEntryError::InvalidManifest(err) => {
                WorkspaceCopyEntryError::InvalidManifest(err)
            }
            GetFolderishEntryError::Internal(err) => err.context("cannot resolve path").into(),
        })?;

    let build_dst_manifest = |dst_parent_id| {
        match src_resolution {
            FsPathResolutionAndManifest::Workspace {
                manifest: src_manifest,
            } => {
                // We deconstruct the source here to explicitly aknowledge what part of
                // it we are going to keep (this is useful to detect whenever a new field
                // is added to the manifest)
                let LocalWorkspaceManifest {
                    base: _,
                    need_sync: _,
                    updated: _,
                    children: src_children,
                    local_confinement_points: _,
                    remote_confinement_points: _,
                    speculative: _,
                } = src_manifest.as_ref();

                let mut dst_manifest = LocalFolderManifest::new(
                    ops.device.device_id.clone(),
                    dst_parent_id,
                    ops.device.now(),
                );
                dst_manifest.children = src_children.to_owned();

                (
                    ArcLocalChildManifest::Folder(Arc::new(dst_manifest)),
                    dst_manifest.base.id,
                )
            }
            FsPathResolutionAndManifest::Folder {
                manifest: src_manifest,
                ..
            } => {
                // We deconstruct the source here to explicitly aknowledge what part of
                // it we are going to keep (this is useful to detect whenever a new field
                // is added to the manifest)
                let LocalFolderManifest {
                    base: _,
                    need_sync: _,
                    updated: _,
                    children: src_children,
                    local_confinement_points: _,
                    remote_confinement_points: _,
                } = src_manifest.as_ref();

                let mut dst_manifest = LocalFolderManifest::new(
                    ops.device.device_id.clone(),
                    dst_parent_id,
                    ops.device.now(),
                );
                dst_manifest.children = src_children.to_owned();

                (
                    ArcLocalChildManifest::Folder(Arc::new(dst_manifest)),
                    dst_manifest.base.id,
                )
            }
            FsPathResolutionAndManifest::File {
                manifest: src_manifest,
                ..
            } => {
                // We deconstruct the source here to explicitly aknowledge what part of
                // it we are going to keep (this is useful to detect whenever a new field
                // is added to the manifest)
                let LocalFileManifest {
                    base: _,
                    need_sync: _,
                    updated: _,
                    size: src_size,
                    blocks: src_blocks,
                    blocksize: src_blocksize,
                } = src_manifest.as_ref();
                let mut dst_manifest = LocalFileManifest::new(
                    ops.device.device_id.clone(),
                    dst_parent_id,
                    ops.device.now(),
                );
                dst_manifest.size = *src_size;
                dst_manifest.blocks = src_blocks.to_owned();
                dst_manifest.blocksize = *src_blocksize;

                (
                    ArcLocalChildManifest::File(Arc::new(dst_manifest)),
                    dst_manifest.base.id,
                )
            }
        }
    };

    let dst_parent_id = match dst_parent_resolution {
        FolderishManifestAndUpdater::Folder {
            manifest: mut dst_parent_manifest,
            updater,
            ..
        } => {
            let mut_dst_parent_manifest = Arc::make_mut(&mut dst_parent_manifest);

            let (dst_manifest, dst_id) = build_dst_manifest(dst_parent_manifest.base.id);

            match mut_dst_parent_manifest.children.entry(dst_name) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    if !overwrite {
                        return Err(WorkspaceCopyEntryError::DestinationExists {
                            entry_id: *entry.get(),
                        });
                    }
                    entry.insert(dst_id);
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(dst_id);
                }
            }
            mut_dst_parent_manifest.updated = ops.device.time_provider.now();
            mut_dst_parent_manifest.need_sync = true;

            updater
                .update_folder_manifest(dst_parent_manifest, Some(dst_manifest))
                .await
                .map_err(|err| match err {
                    UpdateFolderManifestError::Stopped => WorkspaceCopyEntryError::Stopped,
                    UpdateFolderManifestError::Internal(err) => {
                        err.context("cannot update manifest").into()
                    }
                })?;

            dst_parent_manifest.base.id
        }

        FolderishManifestAndUpdater::Root {
            manifest: mut dst_parent_manifest,
            updater,
        } => {
            let mut_dst_parent_manifest = Arc::make_mut(&mut dst_parent_manifest);

            let (dst_manifest, dst_id) = build_dst_manifest(dst_parent_manifest.base.id);

            match mut_dst_parent_manifest.children.entry(dst_name) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    if !overwrite {
                        return Err(WorkspaceCopyEntryError::DestinationExists {
                            entry_id: *entry.get(),
                        });
                    }
                    entry.insert(dst_id);
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(dst_id);
                }
            }
            mut_dst_parent_manifest.updated = ops.device.time_provider.now();
            mut_dst_parent_manifest.need_sync = true;

            updater
                .update_workspace_manifest(dst_parent_manifest, Some(dst_manifest))
                .await
                .map_err(|err| match err {
                    UpdateFolderManifestError::Stopped => WorkspaceCopyEntryError::Stopped,
                    UpdateFolderManifestError::Internal(err) => {
                        err.context("cannot update manifest").into()
                    }
                })?;

            dst_parent_manifest.base.id
        }
    };

    let event = EventWorkspaceOpsOutboundSyncNeeded {
        realm_id: ops.realm_id,
        entry_id: dst_parent_id,
    };
    ops.event_bus.send(&event);

    Ok(())
}
