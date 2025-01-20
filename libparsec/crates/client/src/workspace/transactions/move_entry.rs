// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, sync::Arc};

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::{
    certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError},
    workspace::{
        store::{
            EnsureManifestExistsWithParentError, FolderUpdater, ForUpdateFolderError,
            ForUpdateReparentingError, ReparentingUpdater, UpdateFolderManifestError,
        },
        WorkspaceOps,
    },
    EventWorkspaceOpsOutboundSyncNeeded,
};

#[derive(Debug, Clone, Copy)]
pub enum MoveEntryMode {
    /// Destination may or may not exist.
    CanReplace,
    /// Destination may or may not exist, but must be a file if it exists.
    ///
    /// This is a special case to support Windows behavior when `MOVEFILE_REPLACE_EXISTING`
    /// is set "If lpNewFileName names an existing directory, an error is reported".
    //
    /// (see https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-movefileexw)
    CanReplaceFileOnly,
    /// Destination must not exit so that source can be moved without overwriting anything.
    NoReplace,
    /// Destination and source entries will be swapped, i.e. both must exist and neither will be deleted.
    Exchange,
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceMoveEntryError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("Source doesn't exist")]
    SourceNotFound,
    #[error("Root path cannot be moved")]
    CannotMoveRoot,
    #[error("Only have read access on this workspace")]
    ReadOnlyRealm,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error("Destination already exists (ID `{}`)", .entry_id)]
    DestinationExists { entry_id: VlobID },
    #[error("Destination doesn't exist so exchange is not possible")]
    DestinationNotFound,
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidManifest(#[from] Box<InvalidManifestError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl From<ConnectionError> for WorkspaceMoveEntryError {
    fn from(value: ConnectionError) -> Self {
        match value {
            ConnectionError::NoResponse(_) => Self::Offline,
            err => Self::Internal(err.into()),
        }
    }
}

pub(crate) async fn rename_entry_by_id(
    ops: &WorkspaceOps,
    src_parent_id: VlobID,
    src_name: EntryName,
    dst_name: EntryName,
    mode: MoveEntryMode,
) -> Result<(), WorkspaceMoveEntryError> {
    // TODO: Moving a placeholder into a non-placeholder entry of similar type
    //       should copy the content of the placeholder into the non-placeholder
    //       in order to keep the entry ID.
    //       This is because rename is often used as a way to update an entry
    //       in an atomic way.

    if !ops
        .workspace_external_info
        .lock()
        .expect("Mutex is poisoned")
        .entry
        .role
        .can_write()
    {
        return Err(WorkspaceMoveEntryError::ReadOnlyRealm);
    }

    let (parent_updater, parent_manifest) = ops
        .store
        .for_update_folder(src_parent_id)
        .await
        .map_err(|err| match err {
            ForUpdateFolderError::Offline => WorkspaceMoveEntryError::Offline,
            ForUpdateFolderError::Stopped => WorkspaceMoveEntryError::Stopped,
            ForUpdateFolderError::EntryNotFound => WorkspaceMoveEntryError::SourceNotFound,
            // If the source's parent is not a folder... then the source cannot exist !
            ForUpdateFolderError::EntryNotAFolder => WorkspaceMoveEntryError::SourceNotFound,
            ForUpdateFolderError::NoRealmAccess => WorkspaceMoveEntryError::NoRealmAccess,
            ForUpdateFolderError::InvalidKeysBundle(err) => {
                WorkspaceMoveEntryError::InvalidKeysBundle(err)
            }
            ForUpdateFolderError::InvalidCertificate(err) => {
                WorkspaceMoveEntryError::InvalidCertificate(err)
            }
            ForUpdateFolderError::InvalidManifest(err) => {
                WorkspaceMoveEntryError::InvalidManifest(err)
            }
            ForUpdateFolderError::Internal(err) => {
                err.context("cannot lock for update parent").into()
            }
        })?;

    move_entry_same_parent(
        ops,
        parent_updater,
        parent_manifest,
        src_name,
        dst_name,
        mode,
    )
    .await
}

pub(crate) async fn move_entry(
    ops: &WorkspaceOps,
    src: FsPath,
    dst: FsPath,
    mode: MoveEntryMode,
) -> Result<(), WorkspaceMoveEntryError> {
    // TODO: Moving a placeholder into a non-placeholder entry of similar type
    //       should copy the content of the placeholder into the non-placeholder
    //       in order to keep the entry ID.
    //       This is because rename is often used as a way to update an entry
    //       in an atomic way.

    if !ops
        .workspace_external_info
        .lock()
        .expect("Mutex is poisoned")
        .entry
        .role
        .can_write()
    {
        return Err(WorkspaceMoveEntryError::ReadOnlyRealm);
    }

    let (src_parent_path, src_child_name) = src.into_parent();
    let (dst_parent_path, dst_child_name) = dst.into_parent();

    let (src_child_name, dst_child_name) = match (src_child_name, dst_child_name) {
        (Some(src_name), Some(dst_child_name)) => (src_name, dst_child_name),
        _ => return Err(WorkspaceMoveEntryError::CannotMoveRoot),
    };

    if src_parent_path == dst_parent_path {
        let (parent_manifest, _, parent_updater) = ops
            .store
            .resolve_path_for_update_folder(&src_parent_path)
            .await
            .map_err(|err| match err {
                ForUpdateFolderError::Offline => WorkspaceMoveEntryError::Offline,
                ForUpdateFolderError::Stopped => WorkspaceMoveEntryError::Stopped,
                ForUpdateFolderError::EntryNotFound => WorkspaceMoveEntryError::SourceNotFound,
                // If the source's parent is not a folder... then the source cannot exist !
                ForUpdateFolderError::EntryNotAFolder => WorkspaceMoveEntryError::SourceNotFound,
                ForUpdateFolderError::NoRealmAccess => WorkspaceMoveEntryError::NoRealmAccess,
                ForUpdateFolderError::InvalidKeysBundle(err) => {
                    WorkspaceMoveEntryError::InvalidKeysBundle(err)
                }
                ForUpdateFolderError::InvalidCertificate(err) => {
                    WorkspaceMoveEntryError::InvalidCertificate(err)
                }
                ForUpdateFolderError::InvalidManifest(err) => {
                    WorkspaceMoveEntryError::InvalidManifest(err)
                }
                ForUpdateFolderError::Internal(err) => err
                    .context("cannot resolve and lock for update common parent")
                    .into(),
            })?;

        move_entry_same_parent(
            ops,
            parent_updater,
            parent_manifest,
            src_child_name,
            dst_child_name,
            mode,
        )
        .await
    } else {
        let maybe_dst_child_name = match mode {
            MoveEntryMode::Exchange => Some(&dst_child_name),
            _ => None,
        };
        let updater = ops
            .store
            .resolve_path_for_update_reparenting(
                &src_parent_path,
                &src_child_name,
                &dst_parent_path,
                maybe_dst_child_name,
            )
            .await
            .map_err(|err| match err {
                ForUpdateReparentingError::Offline => WorkspaceMoveEntryError::Offline,
                ForUpdateReparentingError::Stopped => WorkspaceMoveEntryError::Stopped,
                ForUpdateReparentingError::SourceNotFound => {
                    WorkspaceMoveEntryError::SourceNotFound
                }
                ForUpdateReparentingError::DestinationNotFound => {
                    WorkspaceMoveEntryError::DestinationNotFound
                }
                ForUpdateReparentingError::NoRealmAccess => WorkspaceMoveEntryError::NoRealmAccess,
                ForUpdateReparentingError::InvalidKeysBundle(err) => {
                    WorkspaceMoveEntryError::InvalidKeysBundle(err)
                }
                ForUpdateReparentingError::InvalidCertificate(err) => {
                    WorkspaceMoveEntryError::InvalidCertificate(err)
                }
                ForUpdateReparentingError::InvalidManifest(err) => {
                    WorkspaceMoveEntryError::InvalidManifest(err)
                }
                ForUpdateReparentingError::Internal(err) => err
                    .context("cannot resolve and lock for reparenting update")
                    .into(),
            })?;

        move_entry_different_parents(ops, updater, src_child_name, dst_child_name, mode).await
    }
}

async fn move_entry_same_parent(
    ops: &WorkspaceOps,
    parent_updater: FolderUpdater<'_>,
    mut parent_manifest: Arc<LocalFolderManifest>,
    src_child_name: EntryName,
    dst_child_name: EntryName,
    mode: MoveEntryMode,
) -> Result<(), WorkspaceMoveEntryError> {
    let parent_id = parent_manifest.base.id;
    let mut_parent = Arc::make_mut(&mut parent_manifest);

    let Some(child_id) = mut_parent.children.get(&src_child_name).copied() else {
        return Err(WorkspaceMoveEntryError::SourceNotFound);
    };

    // The parent's `children` field may contain invalid data (i.e. referencing
    // a non existing child ID, or a child which `parent` field doesn't correspond
    // to us). In this case we just pretend the entry doesn't exist.
    let maybe_src_child = ops
        .store
        .ensure_manifest_exists_with_parent(child_id, parent_id)
        .await
        .map_err(|err| match err {
            EnsureManifestExistsWithParentError::Offline => WorkspaceMoveEntryError::Offline,
            EnsureManifestExistsWithParentError::Stopped => WorkspaceMoveEntryError::Stopped,
            EnsureManifestExistsWithParentError::NoRealmAccess => {
                WorkspaceMoveEntryError::NoRealmAccess
            }
            EnsureManifestExistsWithParentError::InvalidKeysBundle(err) => {
                WorkspaceMoveEntryError::InvalidKeysBundle(err)
            }
            EnsureManifestExistsWithParentError::InvalidCertificate(err) => {
                WorkspaceMoveEntryError::InvalidCertificate(err)
            }
            EnsureManifestExistsWithParentError::InvalidManifest(err) => {
                WorkspaceMoveEntryError::InvalidManifest(err)
            }
            EnsureManifestExistsWithParentError::Internal(err) => {
                err.context("cannot ensure child/parent coherence").into()
            }
        })?;

    if maybe_src_child.is_none() {
        return Err(WorkspaceMoveEntryError::SourceNotFound);
    }

    let exchange_src_child_id = match mut_parent.children.get(&dst_child_name).copied() {
        // The destination is maybe taken by an existing child...
        Some(dst_child_id) => {
            let maybe_existing_dst_child = ops
                .store
                .ensure_manifest_exists_with_parent(dst_child_id, parent_id)
                .await
                .map_err(|err| match err {
                    EnsureManifestExistsWithParentError::Offline => {
                        WorkspaceMoveEntryError::Offline
                    }
                    EnsureManifestExistsWithParentError::Stopped => {
                        WorkspaceMoveEntryError::Stopped
                    }
                    EnsureManifestExistsWithParentError::NoRealmAccess => {
                        WorkspaceMoveEntryError::NoRealmAccess
                    }
                    EnsureManifestExistsWithParentError::InvalidKeysBundle(err) => {
                        WorkspaceMoveEntryError::InvalidKeysBundle(err)
                    }
                    EnsureManifestExistsWithParentError::InvalidCertificate(err) => {
                        WorkspaceMoveEntryError::InvalidCertificate(err)
                    }
                    EnsureManifestExistsWithParentError::InvalidManifest(err) => {
                        WorkspaceMoveEntryError::InvalidManifest(err)
                    }
                    EnsureManifestExistsWithParentError::Internal(err) => {
                        err.context("cannot ensure child/parent coherence").into()
                    }
                })?;

            match maybe_existing_dst_child {
                Some(existing_dst_child) => {
                    // ...it is an actual child !
                    match mode {
                        MoveEntryMode::CanReplace => None,

                        MoveEntryMode::CanReplaceFileOnly => match existing_dst_child {
                            ArcLocalChildManifest::File(_) => None,
                            ArcLocalChildManifest::Folder(_) => {
                                return Err(WorkspaceMoveEntryError::DestinationExists {
                                    entry_id: dst_child_id,
                                });
                            }
                        },

                        MoveEntryMode::NoReplace => {
                            return Err(WorkspaceMoveEntryError::DestinationExists {
                                entry_id: dst_child_id,
                            });
                        }

                        // Modify the source child to point to the destination child
                        MoveEntryMode::Exchange => Some(dst_child_id),
                    }
                }

                None => {
                    // ...the entry was not a valid child, ignore it
                    if matches!(mode, MoveEntryMode::Exchange) {
                        return Err(WorkspaceMoveEntryError::DestinationNotFound);
                    }
                    None
                }
            }
        }

        // The destination doesn't exist
        None => {
            if matches!(mode, MoveEntryMode::Exchange) {
                return Err(WorkspaceMoveEntryError::DestinationNotFound);
            }
            None
        }
    };

    let now = ops.device.time_provider.now();
    let data = HashMap::from_iter([
        (src_child_name, exchange_src_child_id),
        (dst_child_name, Some(child_id)),
    ]);

    mut_parent.evolve_children_and_mark_updated(data, &ops.config.prevent_sync_pattern, now);

    parent_updater
        .update_folder_manifest(parent_manifest, None)
        .await
        .map_err(|err| match err {
            UpdateFolderManifestError::Stopped => WorkspaceMoveEntryError::Stopped,
            UpdateFolderManifestError::Internal(err) => {
                err.context("cannot update manifest").into()
            }
        })?;

    let event = EventWorkspaceOpsOutboundSyncNeeded {
        realm_id: ops.realm_id,
        entry_id: parent_id,
    };
    ops.event_bus.send(&event);

    Ok(())
}

async fn move_entry_different_parents<'a>(
    ops: &WorkspaceOps,
    mut updater: ReparentingUpdater<'_>,
    src_child_name: EntryName,
    dst_child_name: EntryName,
    mode: MoveEntryMode,
) -> Result<(), WorkspaceMoveEntryError> {
    // Note at this point the source child `parent` field and source parent
    // `children` field have been checked against each other, so we are sure
    // they are actually related.

    let now = ops.device.time_provider.now();
    let src_parent_id = updater.src_parent_manifest.base.id;
    let child_id = updater.src_child_manifest.id();
    let dst_parent_id = updater.dst_parent_manifest.base.id;

    let mut_src_parent = Arc::make_mut(&mut updater.src_parent_manifest);

    // Change src child's parent to dst parent
    match &mut updater.src_child_manifest {
        ArcLocalChildManifest::File(manifest) => {
            let mut_manifest = Arc::make_mut(manifest);
            mut_manifest.parent = dst_parent_id;
            mut_manifest.updated = now;
            mut_manifest.need_sync = true;
        }
        ArcLocalChildManifest::Folder(manifest) => {
            let mut_manifest = Arc::make_mut(manifest);
            mut_manifest.parent = dst_parent_id;
            mut_manifest.updated = now;
            mut_manifest.need_sync = true;
        }
    }

    let mut_dst_parent = Arc::make_mut(&mut updater.dst_parent_manifest);

    let exchange_src_child_id = match mut_dst_parent.children.get(&dst_child_name).copied() {
        // The destination is maybe taken by an existing child...
        Some(dst_child_id) => {
            let maybe_existing_dst_child = ops
                .store
                .ensure_manifest_exists_with_parent(dst_child_id, dst_parent_id)
                .await
                .map_err(|err| match err {
                    EnsureManifestExistsWithParentError::Offline => {
                        WorkspaceMoveEntryError::Offline
                    }
                    EnsureManifestExistsWithParentError::Stopped => {
                        WorkspaceMoveEntryError::Stopped
                    }
                    EnsureManifestExistsWithParentError::NoRealmAccess => {
                        WorkspaceMoveEntryError::NoRealmAccess
                    }
                    EnsureManifestExistsWithParentError::InvalidKeysBundle(err) => {
                        WorkspaceMoveEntryError::InvalidKeysBundle(err)
                    }
                    EnsureManifestExistsWithParentError::InvalidCertificate(err) => {
                        WorkspaceMoveEntryError::InvalidCertificate(err)
                    }
                    EnsureManifestExistsWithParentError::InvalidManifest(err) => {
                        WorkspaceMoveEntryError::InvalidManifest(err)
                    }
                    EnsureManifestExistsWithParentError::Internal(err) => {
                        err.context("cannot ensure child/parent coherence").into()
                    }
                })?;

            match maybe_existing_dst_child {
                Some(existing_dst_child) => {
                    // ...it is an actual child !
                    match mode {
                        MoveEntryMode::CanReplace => None,

                        MoveEntryMode::CanReplaceFileOnly => match existing_dst_child {
                            ArcLocalChildManifest::File(_) => None,
                            ArcLocalChildManifest::Folder(_) => {
                                return Err(WorkspaceMoveEntryError::DestinationExists {
                                    entry_id: dst_child_id,
                                })
                            }
                        },

                        MoveEntryMode::NoReplace => {
                            return Err(WorkspaceMoveEntryError::DestinationExists {
                                entry_id: dst_child_id,
                            });
                        }

                        MoveEntryMode::Exchange => {
                            // Also move destination child into source location
                            match &mut updater
                                .dst_child_manifest
                                .as_mut()
                                .expect("always exists in exchange mode")
                            {
                                ArcLocalChildManifest::File(manifest) => {
                                    let mut_manifest = Arc::make_mut(manifest);
                                    mut_manifest.parent = src_parent_id;
                                    mut_manifest.updated = now;
                                    mut_manifest.need_sync = true;
                                    // Sanity check
                                    debug_assert_eq!(mut_manifest.base.id, dst_child_id);
                                }
                                ArcLocalChildManifest::Folder(manifest) => {
                                    let mut_manifest = Arc::make_mut(manifest);
                                    mut_manifest.parent = src_parent_id;
                                    mut_manifest.updated = now;
                                    mut_manifest.need_sync = true;
                                    // Sanity check
                                    debug_assert_eq!(mut_manifest.base.id, dst_child_id);
                                }
                            }
                            Some(dst_child_id)
                        }
                    }
                }

                None => {
                    // ...the entry was not a valid child, ignore it
                    if matches!(mode, MoveEntryMode::Exchange) {
                        return Err(WorkspaceMoveEntryError::DestinationNotFound);
                    }
                    None
                }
            }
        }

        // The destination doesn't exist
        None => {
            if matches!(mode, MoveEntryMode::Exchange) {
                return Err(WorkspaceMoveEntryError::DestinationNotFound);
            }
            None
        }
    };
    let src_parent_data = HashMap::from_iter([(src_child_name, exchange_src_child_id)]);
    let dst_parent_data = HashMap::from_iter([(dst_child_name, Some(child_id))]);

    mut_src_parent.evolve_children_and_mark_updated(
        src_parent_data,
        &ops.config.prevent_sync_pattern,
        now,
    );
    mut_dst_parent.evolve_children_and_mark_updated(
        dst_parent_data,
        &ops.config.prevent_sync_pattern,
        now,
    );

    updater.update_manifests().await.map_err(|err| match err {
        UpdateFolderManifestError::Stopped => WorkspaceMoveEntryError::Stopped,
        UpdateFolderManifestError::Internal(err) => err.context("cannot update manifest").into(),
    })?;

    let event = EventWorkspaceOpsOutboundSyncNeeded {
        realm_id: ops.realm_id,
        entry_id: dst_parent_id,
    };
    ops.event_bus.send(&event);
    let event = EventWorkspaceOpsOutboundSyncNeeded {
        realm_id: ops.realm_id,
        entry_id: child_id,
    };
    ops.event_bus.send(&event);
    let event = EventWorkspaceOpsOutboundSyncNeeded {
        realm_id: ops.realm_id,
        entry_id: src_parent_id,
    };
    ops.event_bus.send(&event);

    Ok(())
}
