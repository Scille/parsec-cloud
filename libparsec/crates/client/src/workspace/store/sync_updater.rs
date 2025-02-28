// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use crate::{
    workspace::store::cache::{
        populate_cache_from_local_storage, PopulateCacheFromLocalStorageError,
    },
    InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError,
};
use libparsec_platform_storage::workspace::UpdateManifestData;

use super::{
    cache::{
        populate_cache_from_local_storage_or_server, PopulateCacheFromLocalStorageOrServerError,
    },
    per_manifest_update_lock::ManifestUpdateLockGuard,
    resolve_path::RetrievePathFromIdAndLockForUpdateError,
    RetrievePathFromIDEntry,
};

pub(super) type UpdateManifestForSyncError = super::WorkspaceStoreOperationError;

#[derive(Debug, thiserror::Error)]
pub(crate) enum ForUpdateSyncLocalOnlyError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Entry is already being updated")]
    WouldBlock,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ForUpdateSyncError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Component has stopped")]
    Stopped,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error("Entry is already being updated")]
    WouldBlock,
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidManifest(#[from] Box<InvalidManifestError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum IntoSyncConflictUpdaterError {
    #[error("Parent doesn't exist")]
    ParentNotFound,
    #[error("Parent exists but is not a folder")]
    ParentIsNotAFolder,
    #[error("Parent is already being updated")]
    WouldBlock,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Component has stopped")]
    Stopped,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    InvalidManifest(#[from] Box<InvalidManifestError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn for_update_sync_local_only(
    store: &super::WorkspaceStore,
    entry_id: VlobID,
) -> Result<(SyncUpdater<'_>, Option<ArcLocalChildManifest>), ForUpdateSyncLocalOnlyError> {
    // Guard's drop will panic if the lock is not released
    macro_rules! release_guard_on_error {
        ($entry_guard:expr) => {
            store.data.with_current_view_cache(|cache| {
                cache.lock_update_manifests.release($entry_guard);
            });
        };
    }

    // Step 1, 2 and 3 are about retrieving the manifest and locking it for update

    let (entry_guard, manifest) = 'get_lock_and_manifest: {
        // 1) Lock for update

        enum LockForUpdateOutcome {
            GoToStep3(ManifestUpdateLockGuard),
            GoToStep4((ManifestUpdateLockGuard, ArcLocalChildManifest)),
            WouldBlock,
        }

        // Don't wait for the lock: we'd rather wait for the entry to settle
        // before synchronizing it anyway.
        let outcome = store.data.with_current_view_cache(|cache| {
            match cache.lock_update_manifests.try_take(entry_id) {
                Some(entry_guard) => {
                    // 2) Cache lookup for entry...

                    let found = cache.manifests.get(&entry_id);
                    if let Some(manifest) = found {
                        // Cache hit ! We go to step 4.
                        LockForUpdateOutcome::GoToStep4((entry_guard, manifest.clone()))
                    } else {
                        // The entry is not in cache, go to step 3 for a lookup in the local storage.
                        // Note we keep the update lock: this has no impact on read operation, and
                        // any other write operation taking the lock will have no choice but to try
                        // to populate the cache just like we are going to do.
                        LockForUpdateOutcome::GoToStep3(entry_guard)
                    }
                }

                None => {
                    // Note it's not a big deal to return `WouldBlock` here: the caller
                    // will just re-schedule the operation and try again later.
                    LockForUpdateOutcome::WouldBlock
                }
            }
        });
        let entry_guard = match outcome {
            LockForUpdateOutcome::GoToStep3(entry_guard) => entry_guard,
            LockForUpdateOutcome::GoToStep4((entry_guard, manifest)) => {
                break 'get_lock_and_manifest (entry_guard, Some(manifest))
            }
            LockForUpdateOutcome::WouldBlock => {
                return Err(ForUpdateSyncLocalOnlyError::WouldBlock);
            }
        };

        // Be careful here: `entry_guard` must be manually released in case of error !

        // 3) ...and, in case of cache miss, fetch from local storage

        let outcome = populate_cache_from_local_storage(store, entry_id).await;

        match outcome {
            Ok(manifest) => break 'get_lock_and_manifest (entry_guard, Some(manifest)),
            Err(PopulateCacheFromLocalStorageError::EntryNotFound) => {
                break 'get_lock_and_manifest (entry_guard, None)
            }
            Err(err) => {
                release_guard_on_error!(entry_guard);
                return Err(match err {
                    PopulateCacheFromLocalStorageError::Stopped => {
                        ForUpdateSyncLocalOnlyError::Stopped
                    }
                    PopulateCacheFromLocalStorageError::Internal(err) => err.into(),
                    PopulateCacheFromLocalStorageError::EntryNotFound => unreachable!(),
                });
            }
        }
    };

    // 4) We have locked the entry and got the corresponding manifest !

    let updater = SyncUpdater {
        store,
        update_guard: Some(entry_guard),
        original_manifest: manifest.clone(),
        #[cfg(debug_assertions)]
        entry_id,
    };

    Ok((updater, manifest))
}

pub(super) async fn for_update_sync(
    store: &super::WorkspaceStore,
    entry_id: VlobID,
    wait: bool,
) -> Result<(SyncUpdater<'_>, RetrievePathFromIDEntry), ForUpdateSyncError> {
    let (entry, update_guard) =
        super::resolve_path::retrieve_path_from_id_and_lock_for_update(store, entry_id, wait)
            .await
            .map_err(|err| match err {
                RetrievePathFromIdAndLockForUpdateError::Offline(e) => {
                    ForUpdateSyncError::Offline(e)
                }
                RetrievePathFromIdAndLockForUpdateError::Stopped => ForUpdateSyncError::Stopped,
                RetrievePathFromIdAndLockForUpdateError::NoRealmAccess => {
                    ForUpdateSyncError::NoRealmAccess
                }
                RetrievePathFromIdAndLockForUpdateError::WouldBlock => {
                    ForUpdateSyncError::WouldBlock
                }
                RetrievePathFromIdAndLockForUpdateError::InvalidKeysBundle(err) => {
                    ForUpdateSyncError::InvalidKeysBundle(err)
                }
                RetrievePathFromIdAndLockForUpdateError::InvalidCertificate(err) => {
                    ForUpdateSyncError::InvalidCertificate(err)
                }
                RetrievePathFromIdAndLockForUpdateError::InvalidManifest(err) => {
                    ForUpdateSyncError::InvalidManifest(err)
                }
                RetrievePathFromIdAndLockForUpdateError::Internal(err) => {
                    err.context("cannot resolve path").into()
                }
            })?;

    // From now on we shouldn't fail given `update_guard` doesn't release the lock on drop...

    let original_manifest = match &entry {
        RetrievePathFromIDEntry::Missing => None,
        RetrievePathFromIDEntry::Unreachable { manifest } => Some(manifest.clone()),
        RetrievePathFromIDEntry::Reachable { manifest, .. } => Some(manifest.clone()),
    };

    let updater = SyncUpdater {
        store,
        update_guard: Some(update_guard),
        original_manifest,
        #[cfg(debug_assertions)]
        entry_id,
    };

    // ...until this point, where `SyncUpdater`'s drop will take care of releasing the lock !

    Ok((updater, entry))
}

pub(crate) struct SyncUpdater<'a> {
    store: &'a super::WorkspaceStore,
    update_guard: Option<ManifestUpdateLockGuard>,
    /// Keeping the original manifest has two role here:
    /// - To keep track of child&parent ID since they are required when switching
    ///   to a conflict updater.
    /// - For sanity check to ensure the updated manifest the caller will provide
    ///   us later on doesn't contains invalid changes:
    ///   - modifying the base
    ///   - changing the type of the manifest
    original_manifest: Option<ArcLocalChildManifest>,
    #[cfg(debug_assertions)]
    /// Given `original_manifest` can be `None`, we need to keep the entry ID
    /// here to be able to do the sanity checks.
    entry_id: VlobID,
}

impl<'a> SyncUpdater<'a> {
    pub async fn update_manifest(
        self,
        manifest: ArcLocalChildManifest,
    ) -> Result<(), UpdateManifestForSyncError> {
        // Sanity check to ensure the caller is not buggy
        #[cfg(debug_assertions)]
        {
            match &self.original_manifest {
                Some(original_manifest) => match (original_manifest, &manifest) {
                    (ArcLocalChildManifest::File(om), ArcLocalChildManifest::File(m)) => {
                        assert_eq!(om.base.id, m.base.id);
                        assert!(om.base.version <= m.base.version);
                    }
                    (ArcLocalChildManifest::Folder(om), ArcLocalChildManifest::Folder(m)) => {
                        assert_eq!(om.base.id, m.base.id);
                        assert!(om.base.version <= m.base.version);
                    }
                    _ => panic!("Type has changed !"),
                },
                None => match &manifest {
                    ArcLocalChildManifest::File(m) => {
                        assert_eq!(m.base.id, self.entry_id);
                    }
                    ArcLocalChildManifest::Folder(m) => {
                        assert_eq!(m.base.id, self.entry_id);
                    }
                },
            }
        }

        let update_data = match &manifest {
            ArcLocalChildManifest::File(manifest) => UpdateManifestData {
                entry_id: manifest.base.id,
                base_version: manifest.base.version,
                need_sync: manifest.need_sync,
                encrypted: manifest.dump_and_encrypt(&self.store.device.local_symkey),
            },
            ArcLocalChildManifest::Folder(manifest) => UpdateManifestData {
                entry_id: manifest.base.id,
                base_version: manifest.base.version,
                need_sync: manifest.need_sync,
                encrypted: manifest.dump_and_encrypt(&self.store.device.local_symkey),
            },
        };
        self.store
            .data
            .with_storage(|maybe_storage| async move {
                let storage = maybe_storage
                    .as_mut()
                    .ok_or_else(|| UpdateManifestForSyncError::Stopped)?;

                storage
                    .update_manifest(&update_data)
                    .await
                    .map_err(UpdateManifestForSyncError::Internal)
            })
            .await?;

        // Finally update cache
        self.store.data.with_current_view_cache(|cache| {
            cache.manifests.insert(manifest);
        });

        Ok(())
    }

    pub async fn update_file_manifest_and_chunks(
        self,
        manifest: Arc<LocalFileManifest>,
        new_chunks: impl Iterator<Item = (ChunkID, &[u8])>,
        removed_chunks: impl Iterator<Item = ChunkID>,
    ) -> Result<(), UpdateManifestForSyncError> {
        // Sanity check to detect obvious consistency bug in the caller
        #[cfg(debug_assertions)]
        {
            match &self.original_manifest {
                Some(ArcLocalChildManifest::File(om)) => {
                    assert_eq!(om.base, manifest.base);
                }
                Some(_) => panic!("Type has changed !"),
                None => {
                    assert_eq!(manifest.base.id, self.entry_id);
                }
            }
        }

        let update_data = UpdateManifestData {
            entry_id: manifest.base.id,
            base_version: manifest.base.version,
            need_sync: manifest.need_sync,
            encrypted: manifest.dump_and_encrypt(&self.store.device.local_symkey),
        };
        let new_chunks = new_chunks.map(|(chunk_id, cleartext)| {
            (chunk_id, self.store.device.local_symkey.encrypt(cleartext))
        });
        self.store
            .data
            .with_storage(|maybe_storage| async move {
                let storage = maybe_storage
                    .as_mut()
                    .ok_or_else(|| UpdateManifestForSyncError::Stopped)?;

                storage
                    .update_manifest_and_chunks(&update_data, new_chunks, removed_chunks)
                    .await
                    .map_err(UpdateManifestForSyncError::Internal)
            })
            .await?;

        // Finally update cache
        self.store.data.with_current_view_cache(|cache| {
            cache
                .manifests
                .insert(ArcLocalChildManifest::File(manifest));
        });

        Ok(())
    }

    pub async fn into_sync_conflict_updater(
        mut self,
    ) -> Result<
        (SyncConflictUpdater<'a>, Arc<LocalFolderManifest>),
        (SyncUpdater<'a>, IntoSyncConflictUpdaterError),
    > {
        let child_manifest = match &self.original_manifest {
            Some(manifest) => manifest,
            // This should never happen as a conflict must involve a local manifest !
            None => {
                let err = anyhow::anyhow!(
                    "Sync updater with no local manifest cannot lead to a conflict"
                )
                .into();
                return Err((self, err));
            }
        };
        let parent_id = match child_manifest {
            ArcLocalChildManifest::File(m) => m.parent,
            ArcLocalChildManifest::Folder(m) => m.parent,
        };

        // Given the sync updater already locked child, we only have to lock the parent.
        // However if the parent's lock is already taken, we should under no circumstances
        // wait for it: given child & parent locking is not done in a atomic way this could
        // lead to a deadlock !

        let (parent_update_guard, parent_manifest) = 'get_lock_and_manifest: {
            // 1) Lock for update

            enum LockForUpdateOutcome {
                GoToStep3(ManifestUpdateLockGuard),
                GoToStep4((ManifestUpdateLockGuard, ArcLocalChildManifest)),
                WouldBlock,
            }

            let outcome = self.store.data.with_current_view_cache(|cache| {
                // Don't try to wait for the lock as it may lead to a deadlock !
                match cache.lock_update_manifests.try_take(parent_id) {
                    Some(parent_update_guard) => {
                        // 2) Cache lookup for entry...

                        let found = cache.manifests.get(&parent_id);
                        if let Some(parent_manifest) = found {
                            // Cache hit ! We go to step 4.
                            LockForUpdateOutcome::GoToStep4((
                                parent_update_guard,
                                parent_manifest.to_owned(),
                            ))
                        } else {
                            // The entry is not in cache, go to step 3 for a lookup in the local storage.
                            // Note we keep the update lock: this has no impact on read operation, and
                            // any other write operation taking the lock will have no choice but to try
                            // to populate the cache just like we are going to do.
                            LockForUpdateOutcome::GoToStep3(parent_update_guard)
                        }
                    }

                    None => LockForUpdateOutcome::WouldBlock,
                }
            });
            let parent_update_guard = match outcome {
                LockForUpdateOutcome::GoToStep4((parent_update_guard, parent_manifest)) => {
                    break 'get_lock_and_manifest (parent_update_guard, parent_manifest);
                }
                LockForUpdateOutcome::GoToStep3(parent_update_guard) => parent_update_guard,
                LockForUpdateOutcome::WouldBlock => {
                    return Err((self, IntoSyncConflictUpdaterError::WouldBlock));
                }
            };

            // Be careful here: `parent_update_guard` must be manually released in case of error !

            // 3) ...and, in case of cache miss, fetch from local storage

            let parent_manifest =
                match populate_cache_from_local_storage_or_server(self.store, parent_id).await {
                    Ok(ok) => ok,
                    Err(err) => {
                        let err = match err {
                            PopulateCacheFromLocalStorageOrServerError::Offline(e) => {
                                IntoSyncConflictUpdaterError::Offline(e)
                            }
                            PopulateCacheFromLocalStorageOrServerError::Stopped => {
                                IntoSyncConflictUpdaterError::Stopped
                            }
                            PopulateCacheFromLocalStorageOrServerError::EntryNotFound => {
                                IntoSyncConflictUpdaterError::ParentNotFound
                            }
                            PopulateCacheFromLocalStorageOrServerError::NoRealmAccess => {
                                IntoSyncConflictUpdaterError::NoRealmAccess
                            }
                            PopulateCacheFromLocalStorageOrServerError::InvalidKeysBundle(err) => {
                                IntoSyncConflictUpdaterError::InvalidKeysBundle(err)
                            }
                            PopulateCacheFromLocalStorageOrServerError::InvalidCertificate(err) => {
                                IntoSyncConflictUpdaterError::InvalidCertificate(err)
                            }
                            PopulateCacheFromLocalStorageOrServerError::InvalidManifest(err) => {
                                IntoSyncConflictUpdaterError::InvalidManifest(err)
                            }
                            PopulateCacheFromLocalStorageOrServerError::Internal(err) => {
                                err.context("cannot populate cache").into()
                            }
                        };
                        self.store.data.with_current_view_cache(|cache| {
                            cache.lock_update_manifests.release(parent_update_guard)
                        });
                        return Err((self, err));
                    }
                };

            (parent_update_guard, parent_manifest)
        };

        let parent_manifest = match parent_manifest {
            ArcLocalChildManifest::Folder(parent_manifest) => parent_manifest,
            ArcLocalChildManifest::File(_) => {
                return Err((self, IntoSyncConflictUpdaterError::ParentIsNotAFolder))
            }
        };

        // `update_guard` is always set when building `SyncUpdater`, then only set to
        // `None` either in the drop or here (where we have ownership over `self`, so
        // it is going to be dropped and hence will never be reused).
        let child_update_guard = self.update_guard.take().expect("always present");

        let updater = SyncConflictUpdater {
            store: self.store,
            update_guards: Some((child_update_guard, parent_update_guard)),
            #[cfg(debug_assertions)]
            original_parent_manifest: parent_manifest.clone(),
            #[cfg(debug_assertions)]
            original_child_manifest: child_manifest.to_owned(),
        };

        Ok((updater, parent_manifest))
    }
}

impl Drop for SyncUpdater<'_> {
    fn drop(&mut self) {
        if let Some(update_guard) = self.update_guard.take() {
            self.store.data.with_current_view_cache(|cache| {
                cache.lock_update_manifests.release(update_guard);
            });
        }
    }
}

pub(crate) struct SyncConflictUpdater<'a> {
    store: &'a super::WorkspaceStore,
    /// Child & parent update guards
    update_guards: Option<(ManifestUpdateLockGuard, ManifestUpdateLockGuard)>,
    #[cfg(debug_assertions)]
    /// Keep the original manifest to ensure the updated one the caller will
    /// provide us later on doesn't contains invalid changes:
    /// - modifying the base
    /// - changing the type of the manifest
    original_parent_manifest: Arc<LocalFolderManifest>,
    #[cfg(debug_assertions)]
    /// Keep the original manifest to ensure the updated one the caller will
    /// provide us later on doesn't contains invalid changes:
    /// - modifying the base
    /// - changing the type of the manifest
    original_child_manifest: ArcLocalChildManifest,
}

impl SyncConflictUpdater<'_> {
    pub async fn update_manifests(
        self,
        child_manifest: ArcLocalChildManifest,
        parent_manifest: Arc<LocalFolderManifest>,
        conflicting_new_child_manifest: ArcLocalChildManifest,
    ) -> Result<(), UpdateManifestForSyncError> {
        // Sanity check to detect obvious consistency bug in the caller
        #[cfg(debug_assertions)]
        {
            assert_ne!(
                conflicting_new_child_manifest.id(),
                self.original_child_manifest.id()
            );
            assert_eq!(
                conflicting_new_child_manifest.parent(),
                self.original_parent_manifest.base.id
            );
            assert_eq!(parent_manifest.base, self.original_parent_manifest.base);
            assert!(parent_manifest
                .children
                .iter()
                .any(|(_, id)| *id == conflicting_new_child_manifest.id()));
            assert!(parent_manifest
                .children
                .iter()
                .any(|(_, id)| *id == child_manifest.id()));
            match (&self.original_child_manifest, &child_manifest) {
                (ArcLocalChildManifest::File(_), ArcLocalChildManifest::File(_)) => (),
                (ArcLocalChildManifest::Folder(_), ArcLocalChildManifest::Folder(_)) => (),
                _ => panic!("Type has changed !"),
            }
        }

        let child_update_data = match &child_manifest {
            ArcLocalChildManifest::File(manifest) => UpdateManifestData {
                entry_id: manifest.base.id,
                base_version: manifest.base.version,
                need_sync: manifest.need_sync,
                encrypted: manifest.dump_and_encrypt(&self.store.device.local_symkey),
            },
            ArcLocalChildManifest::Folder(manifest) => UpdateManifestData {
                entry_id: manifest.base.id,
                base_version: manifest.base.version,
                need_sync: manifest.need_sync,
                encrypted: manifest.dump_and_encrypt(&self.store.device.local_symkey),
            },
        };
        let parent_update_data = UpdateManifestData {
            entry_id: parent_manifest.base.id,
            base_version: parent_manifest.base.version,
            need_sync: parent_manifest.need_sync,
            encrypted: parent_manifest.dump_and_encrypt(&self.store.device.local_symkey),
        };
        let conflicting_new_child_update_data = match &conflicting_new_child_manifest {
            ArcLocalChildManifest::File(manifest) => UpdateManifestData {
                entry_id: manifest.base.id,
                base_version: manifest.base.version,
                need_sync: manifest.need_sync,
                encrypted: manifest.dump_and_encrypt(&self.store.device.local_symkey),
            },
            ArcLocalChildManifest::Folder(manifest) => UpdateManifestData {
                entry_id: manifest.base.id,
                base_version: manifest.base.version,
                need_sync: manifest.need_sync,
                encrypted: manifest.dump_and_encrypt(&self.store.device.local_symkey),
            },
        };
        self.store
            .data
            .with_storage(|maybe_storage| async move {
                let storage = maybe_storage
                    .as_mut()
                    .ok_or_else(|| UpdateManifestForSyncError::Stopped)?;

                storage
                    .update_manifests(
                        [
                            child_update_data,
                            parent_update_data,
                            conflicting_new_child_update_data,
                        ]
                        .into_iter(),
                    )
                    .await
                    .map_err(UpdateManifestForSyncError::Internal)
            })
            .await?;

        // Finally update cache
        self.store.data.with_current_view_cache(|cache| {
            cache
                .manifests
                .insert(ArcLocalChildManifest::Folder(parent_manifest));
            cache.manifests.insert(child_manifest);
            cache.manifests.insert(conflicting_new_child_manifest);
        });

        Ok(())
    }
}

impl Drop for SyncConflictUpdater<'_> {
    fn drop(&mut self) {
        if let Some((child_update_guard, parent_update_guard)) = self.update_guards.take() {
            self.store.data.with_current_view_cache(|cache| {
                cache.lock_update_manifests.release(child_update_guard);
                cache.lock_update_manifests.release(parent_update_guard);
            });
        }
    }
}
