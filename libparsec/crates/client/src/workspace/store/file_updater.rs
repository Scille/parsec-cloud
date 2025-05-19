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
        PopulateCacheFromLocalStorageOrServerError, populate_cache_from_local_storage_or_server,
    },
    per_manifest_update_lock::ManifestUpdateLockGuard,
};

pub(super) type UpdateFileManifestAndContinueError = super::WorkspaceStoreOperationError;

#[derive(Debug, thiserror::Error)]
pub(crate) enum ForUpdateFileError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Component has stopped")]
    Stopped,
    #[error("Path doesn't exist")]
    EntryNotFound,
    #[error("Path points to entry (ID: `{}`) that is not a file", .entry_id)]
    EntryNotAFile { entry_id: VlobID },
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

        // 1) Lock for update

        enum LockForUpdateOutcome {
            WaitAndRetryStep1(EventListener),
            // There is no GoToStep2 because it's the dispatching step
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
                break (update_guard, manifest);
            }
            LockForUpdateOutcome::WaitAndRetryStep1(listener) => {
                if !wait {
                    return Err(ForUpdateFileError::WouldBlock);
                }
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
                    ForUpdateFileError::Offline(e)
                }
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
            Ok(manifest) => break (update_guard, manifest),
            Err(err) => {
                release_guard_on_error!(update_guard);
                return Err(err);
            }
        }
    };

    // 4) We have locked the entry and got the corresponding manifest !

    let manifest = match manifest {
        ArcLocalChildManifest::File(manifest) => manifest,
        ArcLocalChildManifest::Folder(manifest) => {
            release_guard_on_error!(update_guard);
            return Err(ForUpdateFileError::EntryNotAFile {
                entry_id: manifest.base.id,
            });
        }
    };

    let updater = FileUpdater {
        update_guard,
        #[cfg(debug_assertions)]
        entry_id: manifest.base.id,
    };

    Ok((updater, manifest))
}

/// /!\ The underlying lock doesn't get released on drop /!\
///
/// Instead `FileUpdater::close` must be called once you are done.
/// The reason for this is that, unlike folder updater, the file updater is expected to
/// have a long lifetime (i.e. the time the file is open). Hence the file update doesn't
/// keep hold on the store.
#[derive(Debug)]
pub(crate) struct FileUpdater {
    update_guard: ManifestUpdateLockGuard,
    #[cfg(debug_assertions)]
    entry_id: VlobID,
}

impl FileUpdater {
    pub async fn update_file_manifest_and_continue(
        &self,
        store: &super::WorkspaceStore,
        manifest: Arc<LocalFileManifest>,
        new_chunks: impl Iterator<Item = (ChunkID, &[u8])>,
        removed_chunks: impl Iterator<Item = ChunkID>,
    ) -> Result<(), UpdateFileManifestAndContinueError> {
        // Sanity check to ensure the caller is not buggy
        #[cfg(debug_assertions)]
        assert_eq!(manifest.base.id, self.entry_id);

        let update_data = UpdateManifestData {
            entry_id: manifest.base.id,
            base_version: manifest.base.version,
            need_sync: manifest.need_sync,
            encrypted: manifest.dump_and_encrypt(&store.device.local_symkey),
        };
        let new_chunks = new_chunks
            .map(|(chunk_id, cleartext)| (chunk_id, store.device.local_symkey.encrypt(cleartext)));
        store
            .data
            .with_storage(async |maybe_storage| {
                let storage = maybe_storage
                    .as_mut()
                    .ok_or_else(|| UpdateFileManifestAndContinueError::Stopped)?;

                storage
                    .update_manifest_and_chunks(&update_data, new_chunks, removed_chunks)
                    .await
                    .map_err(UpdateFileManifestAndContinueError::Internal)
            })
            .await?;

        // Finally update cache
        store.data.with_current_view_cache(|cache| {
            cache
                .manifests
                .insert(ArcLocalChildManifest::File(manifest));
        });

        Ok(())
    }

    pub fn close(self, store: &super::WorkspaceStore) {
        store.data.with_current_view_cache(|cache| {
            cache.lock_update_manifests.release(self.update_guard);
        });
    }
}
