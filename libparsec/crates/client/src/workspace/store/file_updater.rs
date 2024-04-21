// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

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

pub(super) type UpdateFileManifestAndContinueError = super::WorkspaceStoreOperationError;

#[derive(Debug, thiserror::Error)]
pub(crate) enum ForUpdateFileError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("Path doesn't exist")]
    EntryNotFound,
    #[error("Path points to entry that is not a file")]
    EntryNotAFile,
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

pub(super) async fn for_update_file(
    store: &super::WorkspaceStore,
    entry_id: VlobID,
    wait: bool,
) -> Result<(FileUpdater, Arc<LocalFileManifest>), ForUpdateFileError> {
    // Guard's drop will panic if the lock is not released
    macro_rules! release_guard_on_error {
        ($entry_guard:expr) => {
            let mut cache_guard = store.current_view_cache.lock().expect("Mutex is poisoned");
            cache_guard.lock_update_manifests.release($entry_guard);
        };
    }

    // Step 1, 2 and 3 are about retrieving the manifest and locking it for update

    let mut maybe_need_wait = None;
    let (entry_guard, manifest) = loop {
        if let Some(listener) = maybe_need_wait {
            listener.await;
        }

        let entry_guard = {
            let mut cache_guard = store.current_view_cache.lock().expect("Mutex is poisoned");

            // 1) Lock for update

            let outcome = cache_guard.lock_update_manifests.take(entry_id);
            match outcome {
                ManifestUpdateLockTakeOutcome::Taken(entry_guard) => {
                    // 2a) Special case if the entry to modify is the root dir...

                    if entry_id == store.realm_id {
                        break (
                            entry_guard,
                            ArcLocalChildManifest::Folder(cache_guard.root_manifest.clone()),
                        );
                    }

                    // 2b) Non-root dir requires cache lookup...

                    let found = cache_guard.child_manifests.get(&entry_id);
                    if let Some(manifest) = found {
                        // Cache hit ! We go to step 3.
                        break (entry_guard, manifest.clone());
                    }
                    // The entry is not in cache, from there we release the cache
                    // lock and jump to step 2.
                    entry_guard
                }

                ManifestUpdateLockTakeOutcome::NeedWait(listener) => {
                    if !wait {
                        return Err(ForUpdateFileError::WouldBlock);
                    }
                    maybe_need_wait = Some(listener);
                    continue;
                }
            }
        };

        // 3) ...and, in case of cache miss, fetch from local storage or server

        let outcome = populate_cache_from_local_storage_or_server(store, entry_id)
            .await
            .map_err(|err| match err {
                PopulateCacheFromLocalStorageOrServerError::Offline => ForUpdateFileError::Offline,
                PopulateCacheFromLocalStorageOrServerError::Stopped => ForUpdateFileError::Stopped,
                PopulateCacheFromLocalStorageOrServerError::EntryNotFound => {
                    ForUpdateFileError::EntryNotFound
                }
                PopulateCacheFromLocalStorageOrServerError::NoRealmAccess => {
                    ForUpdateFileError::NoRealmAccess
                }
                PopulateCacheFromLocalStorageOrServerError::InvalidKeysBundle(err) => {
                    ForUpdateFileError::InvalidKeysBundle(err)
                }
                PopulateCacheFromLocalStorageOrServerError::InvalidCertificate(err) => {
                    ForUpdateFileError::InvalidCertificate(err)
                }
                PopulateCacheFromLocalStorageOrServerError::InvalidManifest(err) => {
                    ForUpdateFileError::InvalidManifest(err)
                }
                PopulateCacheFromLocalStorageOrServerError::Internal(err) => err.into(),
            });

        match outcome {
            Ok(manifest) => break (entry_guard, manifest),
            Err(err) => {
                release_guard_on_error!(entry_guard);
                return Err(err);
            }
        }
    };

    // 4) We have locked the entry and got the corresponding manifest !

    let manifest = match manifest {
        ArcLocalChildManifest::File(manifest) => manifest,
        ArcLocalChildManifest::Folder(_) => {
            release_guard_on_error!(entry_guard);
            return Err(ForUpdateFileError::EntryNotAFile);
        }
    };

    let updater = FileUpdater {
        _update_guard: entry_guard,
    };

    Ok((updater, manifest))
}

pub async fn resolve_path_for_update_file_manifest(
    store: &super::WorkspaceStore,
    path: &FsPath,
) -> Result<(Arc<LocalFileManifest>, PathConfinementPoint, FileUpdater), ForUpdateFileError> {
    let (manifest, confinement, update_guard) =
        super::resolve_path::resolve_path_and_lock_for_update(store, path)
            .await
            .map_err(|err| match err {
                ResolvePathError::Offline => ForUpdateFileError::Offline,
                ResolvePathError::Stopped => ForUpdateFileError::Stopped,
                ResolvePathError::EntryNotFound => ForUpdateFileError::EntryNotFound,
                ResolvePathError::NoRealmAccess => ForUpdateFileError::NoRealmAccess,
                ResolvePathError::InvalidKeysBundle(err) => {
                    ForUpdateFileError::InvalidKeysBundle(err)
                }
                ResolvePathError::InvalidCertificate(err) => {
                    ForUpdateFileError::InvalidCertificate(err)
                }
                ResolvePathError::InvalidManifest(err) => ForUpdateFileError::InvalidManifest(err),
                ResolvePathError::Internal(err) => err.context("cannot resolve path").into(),
            })?;

    // From now on we shouldn't fail given `update_guard` doesn't release the lock on drop.

    let updater = FileUpdater {
        _update_guard: update_guard,
    };

    match manifest {
        ArcLocalChildManifest::File(manifest) => Ok((manifest, confinement, updater)),
        ArcLocalChildManifest::Folder(_) => Err(ForUpdateFileError::EntryNotAFile),
    }
}

/// /!\ The underlying lock doesn't get released on drop /!\
///
/// Instead `FileUpdater::close` must be called once you are done.
/// The reason for this is that, unlike folder updater, the file udpater is expected to
/// have a long lifetime (i.e. the time the file is open). Hence the file update doesn't
/// keep hold on the store.
pub(crate) struct FileUpdater {
    _update_guard: ManifestUpdateLockGuard,
}

impl FileUpdater {
    pub async fn update_file_manifest_and_continue(
        &self,
        store: &super::WorkspaceStore,
        manifest: Arc<LocalFileManifest>,
        new_chunks: &[(ChunkID, Vec<u8>)],
        removed_chunks: &[ChunkID],
    ) -> Result<(), UpdateFileManifestAndContinueError> {
        let mut storage_guard = store.storage.lock().await;
        let storage = storage_guard
            .as_mut()
            .ok_or_else(|| UpdateFileManifestAndContinueError::Stopped)?;

        let update_data = UpdateManifestData {
            entry_id: manifest.base.id,
            base_version: manifest.base.version,
            need_sync: manifest.need_sync,
            encrypted: manifest.dump_and_encrypt(&store.device.local_symkey),
        };

        let new_chunks = new_chunks
            .iter()
            .map(|(chunk_id, cleartext)| (*chunk_id, store.device.local_symkey.encrypt(cleartext)));
        let removed_chunks = removed_chunks.iter().copied();
        storage
            .update_manifest_and_chunks(&update_data, new_chunks, removed_chunks)
            .await?;

        // Finally update cache
        let mut cache = store.current_view_cache.lock().expect("Mutex is poisoned");
        cache
            .child_manifests
            .insert(manifest.base.id, ArcLocalChildManifest::File(manifest));

        Ok(())
    }

    pub fn close(self, store: &super::WorkspaceStore) {
        store
            .current_view_cache
            .lock()
            .expect("Mutex is poisoned")
            .lock_update_manifests
            .release(self._update_guard);
    }
}
