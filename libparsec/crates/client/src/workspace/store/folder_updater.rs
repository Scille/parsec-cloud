// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::ConnectionError;
use libparsec_platform_async::event::EventListener;
use libparsec_types::prelude::*;

use crate::certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError};
use libparsec_platform_storage::workspace::UpdateManifestData;

use super::per_manifest_update_lock::ManifestUpdateLockTakeOutcome;
use super::{
    cache::{
        populate_cache_from_local_storage_or_server, PopulateCacheFromLocalStorageOrServerError,
    },
    per_manifest_update_lock::ManifestUpdateLockGuard,
};
use super::{PathConfinementPoint, ResolvePathError};

pub(super) type UpdateFolderManifestError = super::WorkspaceStoreOperationError;

#[derive(Debug, thiserror::Error)]
pub(crate) enum ForUpdateFolderError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Component has stopped")]
    Stopped,
    #[error("Path doesn't exist")]
    EntryNotFound,
    #[error("Path points to entry that is not a folder")]
    EntryNotAFolder,
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

pub(super) async fn for_update_folder(
    store: &super::WorkspaceStore,
    entry_id: VlobID,
) -> Result<(FolderUpdater<'_>, Arc<LocalFolderManifest>), ForUpdateFolderError> {
    // Guard's drop will panic if the lock is not released
    macro_rules! release_guard_on_error {
        ($update_guard:expr) => {
            store.data.with_current_view_cache(|cache| {
                cache.lock_update_manifests.release($update_guard);
            });
        };
    }

    // Step 1, 2 and 3 are about retrieving the manifest and locking it for update

    let mut maybe_need_wait = None;
    let (update_guard, manifest) = loop {
        if let Some(listener) = maybe_need_wait {
            listener.await;
        }

        enum LockForUpdateOutcome {
            WaitAndRetryStep1(EventListener),
            GoToStep3(ManifestUpdateLockGuard),
            GoToStep4((ManifestUpdateLockGuard, ArcLocalChildManifest)),
        }
        let outcome = store.data.with_current_view_cache(|cache| {
            match cache.lock_update_manifests.take(entry_id) {
                ManifestUpdateLockTakeOutcome::Taken(update_guard) => {
                    // 2) Cache lookup for entry...

                    let found = cache.manifests.get(&entry_id);
                    if let Some(manifest) = found {
                        // Cache hit ! We go to step 4.
                        LockForUpdateOutcome::GoToStep4((update_guard, manifest.clone()))
                    } else {
                        // The entry is not in cache, go to step 3 for a lookup in the local storage.
                        // Note we keep the update lock: this has no impact on read operation, and
                        // any other write operation taking the lock will have no choice but to try
                        // to populate the cache just like we are going to do.
                        LockForUpdateOutcome::GoToStep3(update_guard)
                    }
                }

                ManifestUpdateLockTakeOutcome::NeedWait(listener) => {
                    LockForUpdateOutcome::WaitAndRetryStep1(listener)
                }
            }
        });
        let update_guard = match outcome {
            LockForUpdateOutcome::GoToStep3(update_guard) => update_guard,
            LockForUpdateOutcome::GoToStep4((update_guard, manifest)) => {
                break (update_guard, manifest)
            }
            LockForUpdateOutcome::WaitAndRetryStep1(listener) => {
                maybe_need_wait = Some(listener);
                continue;
            }
        };

        // Be careful here: `update_guard` must be manually released in case of error !

        // 3) ...and, in case of cache miss, fetch from local storage or server

        let outcome = populate_cache_from_local_storage_or_server(store, entry_id)
            .await
            .map_err(|err| match err {
                PopulateCacheFromLocalStorageOrServerError::Offline(e) => {
                    ForUpdateFolderError::Offline(e)
                }
                PopulateCacheFromLocalStorageOrServerError::Stopped => {
                    ForUpdateFolderError::Stopped
                }
                PopulateCacheFromLocalStorageOrServerError::EntryNotFound => {
                    ForUpdateFolderError::EntryNotFound
                }
                PopulateCacheFromLocalStorageOrServerError::NoRealmAccess => {
                    ForUpdateFolderError::NoRealmAccess
                }
                PopulateCacheFromLocalStorageOrServerError::InvalidKeysBundle(err) => {
                    ForUpdateFolderError::InvalidKeysBundle(err)
                }
                PopulateCacheFromLocalStorageOrServerError::InvalidCertificate(err) => {
                    ForUpdateFolderError::InvalidCertificate(err)
                }
                PopulateCacheFromLocalStorageOrServerError::InvalidManifest(err) => {
                    ForUpdateFolderError::InvalidManifest(err)
                }
                PopulateCacheFromLocalStorageOrServerError::Internal(err) => err.into(),
            });

        match outcome {
            Ok(manifest) => break (update_guard, manifest),
            Err(err) => {
                release_guard_on_error!(update_guard);
                return Err(err);
            }
        }
    };

    // 4) We have locked the entry and got the corresponding manifest !

    let manifest = match manifest {
        ArcLocalChildManifest::Folder(manifest) => manifest,
        ArcLocalChildManifest::File(_) => {
            release_guard_on_error!(update_guard);
            return Err(ForUpdateFolderError::EntryNotAFolder);
        }
    };

    let updater = FolderUpdater {
        store,
        update_guard: Some(update_guard),
        #[cfg(debug_assertions)]
        entry_id: manifest.base.id,
    };

    Ok((updater, manifest))
}

pub async fn resolve_path_for_update_folder<'a>(
    store: &'a super::WorkspaceStore,
    path: &FsPath,
) -> Result<
    (
        Arc<LocalFolderManifest>,
        PathConfinementPoint,
        FolderUpdater<'a>,
    ),
    ForUpdateFolderError,
> {
    let (manifest, confinement, update_guard) =
        super::resolve_path::resolve_path_and_lock_for_update(store, path)
            .await
            .map_err(|err| match err {
                ResolvePathError::Offline(e) => ForUpdateFolderError::Offline(e),
                ResolvePathError::Stopped => ForUpdateFolderError::Stopped,
                ResolvePathError::EntryNotFound => ForUpdateFolderError::EntryNotFound,
                ResolvePathError::NoRealmAccess => ForUpdateFolderError::NoRealmAccess,
                ResolvePathError::InvalidKeysBundle(err) => {
                    ForUpdateFolderError::InvalidKeysBundle(err)
                }
                ResolvePathError::InvalidCertificate(err) => {
                    ForUpdateFolderError::InvalidCertificate(err)
                }
                ResolvePathError::InvalidManifest(err) => {
                    ForUpdateFolderError::InvalidManifest(err)
                }
                ResolvePathError::Internal(err) => err.context("cannot resolve path").into(),
            })?;

    // From now on we shouldn't fail given `update_guard` doesn't release the lock on drop...

    let updater = FolderUpdater {
        store,
        update_guard: Some(update_guard),
        #[cfg(debug_assertions)]
        entry_id: manifest.id(),
    };

    // ...until this point, where `FolderUpdater`'s drop will take care of releasing the lock !

    match manifest {
        ArcLocalChildManifest::Folder(manifest) => Ok((manifest, confinement, updater)),
        ArcLocalChildManifest::File(_) => Err(ForUpdateFolderError::EntryNotAFolder),
    }
}

pub(crate) struct FolderUpdater<'a> {
    store: &'a super::WorkspaceStore,
    update_guard: Option<ManifestUpdateLockGuard>,
    #[cfg(debug_assertions)]
    entry_id: VlobID,
}

impl<'a> FolderUpdater<'a> {
    pub async fn update_folder_manifest(
        self,
        manifest: Arc<LocalFolderManifest>,
        new_child: Option<ArcLocalChildManifest>,
    ) -> Result<(), UpdateFolderManifestError> {
        // Sanity check to ensure the caller is not buggy
        #[cfg(debug_assertions)]
        {
            assert_eq!(manifest.base.id, self.entry_id);
            if let Some(new_child) = &new_child {
                let child_id = match new_child {
                    ArcLocalChildManifest::File(manifest) => manifest.base.id,
                    ArcLocalChildManifest::Folder(manifest) => manifest.base.id,
                };
                assert_ne!(child_id, self.entry_id);
            }
        }

        let update_data = UpdateManifestData {
            entry_id: manifest.base.id,
            base_version: manifest.base.version,
            need_sync: manifest.need_sync,
            encrypted: manifest.dump_and_encrypt(&self.store.device.local_symkey),
        };
        match new_child {
            None => {
                self.store
                    .data
                    .with_storage(|maybe_storage| async move {
                        let storage = maybe_storage
                            .as_mut()
                            .ok_or_else(|| UpdateFolderManifestError::Stopped)?;

                        storage
                            .update_manifest(&update_data)
                            .await
                            .map_err(UpdateFolderManifestError::Internal)
                    })
                    .await?;

                // Finally update cache
                self.store.data.with_current_view_cache(|cache| {
                    cache
                        .manifests
                        .insert(ArcLocalChildManifest::Folder(manifest));
                });
            }

            Some(new_child) => {
                let new_child_update_data = match &new_child {
                    ArcLocalChildManifest::File(new_child) => UpdateManifestData {
                        entry_id: new_child.base.id,
                        base_version: new_child.base.version,
                        need_sync: new_child.need_sync,
                        encrypted: new_child.dump_and_encrypt(&self.store.device.local_symkey),
                    },
                    ArcLocalChildManifest::Folder(new_child) => UpdateManifestData {
                        entry_id: new_child.base.id,
                        base_version: new_child.base.version,
                        need_sync: new_child.need_sync,
                        encrypted: new_child.dump_and_encrypt(&self.store.device.local_symkey),
                    },
                };
                self.store
                    .data
                    .with_storage(|maybe_storage| async move {
                        let storage = maybe_storage
                            .as_mut()
                            .ok_or_else(|| UpdateFolderManifestError::Stopped)?;

                        storage
                            .update_manifests([update_data, new_child_update_data].into_iter())
                            .await
                            .map_err(UpdateFolderManifestError::Internal)
                    })
                    .await?;

                // Finally update cache
                self.store.data.with_current_view_cache(|cache| {
                    cache
                        .manifests
                        .insert(ArcLocalChildManifest::Folder(manifest));
                    cache.manifests.insert(new_child);
                });
            }
        }

        Ok(())
    }
}

impl Drop for FolderUpdater<'_> {
    fn drop(&mut self) {
        if let Some(update_guard) = self.update_guard.take() {
            self.store.data.with_current_view_cache(|cache| {
                cache.lock_update_manifests.release(update_guard);
            });
        }
    }
}
