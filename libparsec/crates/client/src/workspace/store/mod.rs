// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod cache;
mod file_updater;
mod folder_updater;
mod manifest_access;
mod per_manifest_update_lock;
mod prevent_sync_pattern;
mod reparent_updater;
mod resolve_path;
mod sync_updater;

use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_client_connection::ConnectionError;
use libparsec_platform_async::lock::Mutex as AsyncMutex;
use libparsec_platform_storage::workspace::{UpdateManifestData, WorkspaceStorage};
use libparsec_types::prelude::*;

use crate::{
    certif::CertificateOps,
    server_fetch::{server_fetch_block, ServerFetchBlockError},
    InvalidBlockAccessError, InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError,
};

use cache::CurrentViewCache;
pub(crate) use file_updater::{FileUpdater, ForUpdateFileError};
pub(crate) use folder_updater::{FolderUpdater, ForUpdateFolderError};
pub(crate) use manifest_access::GetManifestError;
pub(crate) use reparent_updater::{ForUpdateReparentingError, ReparentingUpdater};
pub(crate) use resolve_path::{
    PathConfinementPoint, ResolvePathError, RetrievePathFromIDEntry, RetrievePathFromIDError,
};
pub(crate) use sync_updater::{
    ForUpdateSyncError, ForUpdateSyncLocalOnlyError, IntoSyncConflictUpdaterError, SyncUpdater,
};

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceStoreOperationError {
    #[error("Component has stopped")]
    Stopped,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) type GetNeedSyncEntriesError = WorkspaceStoreOperationError;
pub(super) type GetRealmCheckpointError = WorkspaceStoreOperationError;
pub(super) type UpdateRealmCheckpointError = WorkspaceStoreOperationError;
pub(super) type UpdateFolderManifestError = WorkspaceStoreOperationError;
pub(super) type UpdateFileManifestAndContinueError = WorkspaceStoreOperationError;
pub(super) type PromoteLocalOnlyChunkToUploadedBlockError = WorkspaceStoreOperationError;
pub(super) type GetNotUploadedChunkError = WorkspaceStoreOperationError;

#[derive(Debug, thiserror::Error)]
pub(super) enum ReadChunkOrBlockLocalOnlyError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Chunk doesn't exist")]
    ChunkNotFound,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub(super) enum ReadChunkOrBlockError {
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("Block access is temporary unavailable on the server")]
    ServerBlockstoreUnavailable,
    #[error("Component has stopped")]
    Stopped,
    #[error("Chunk doesn't exist")]
    ChunkNotFound,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error(transparent)]
    InvalidBlockAccess(#[from] Box<InvalidBlockAccessError>),
    #[error(transparent)]
    InvalidKeysBundle(#[from] Box<InvalidKeysBundleError>),
    #[error(transparent)]
    InvalidCertificate(#[from] Box<InvalidCertificateError>),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub(super) enum EnsureManifestExistsWithParentError {
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

/*
 * WorkspaceStore & friends
 */

mod data {
    //! This module keeps private the internals of `WorkspaceStoreData` in order to reduce
    //! the risk of deadlocks.
    //!
    //! Indeed, the current view cache and storage (i.e. local database) are both protected
    //! by a mutex and used in conjunction, so deadlock may occur due to the wrong order of
    //! acquisition or if a lock it kept too long (especially since locks are not reentrant).
    //!
    //! So to solve this issue we only give access to store and storage through a callback
    //! based API:
    //! - It better limits and makes explicit the scope of the lock.
    //! - It is not possible to acquire the storage lock when the view cache lock is acquired
    //!   (since the former is protected by an async lock and the latter by a sync lock).
    //! - In debug build a deadlock detection mechanism is in place to ensure only a single
    //!   lock is acquired at a time for any given task.

    use super::*;
    #[cfg(debug_assertions)]
    use libparsec_platform_async::{try_task_id, TaskID};

    /*
     * WorkspaceStoreData
     */

    #[derive(Debug)]
    pub(super) struct WorkspaceStoreData {
        /// Note cache also contains the update locks.
        current_view_cache: Mutex<CurrentViewCache>,
        /// Given accessing `storage` requires exclusive access, it is better to have it
        /// under its own lock so that all cache hit operations can occur concurrently.
        storage: AsyncMutex<Option<WorkspaceStorage>>,
        #[cfg(debug_assertions)]
        lock_tracking: Mutex<Vec<(RunningFutureID, DataLock)>>,
    }

    impl WorkspaceStoreData {
        pub fn new(storage: WorkspaceStorage, root_manifest: LocalWorkspaceManifest) -> Self {
            Self {
                current_view_cache: Mutex::new(CurrentViewCache::new(Arc::new(
                    root_manifest.into(),
                ))),
                storage: AsyncMutex::new(Some(storage)),
                #[cfg(debug_assertions)]
                lock_tracking: Default::default(),
            }
        }

        /// Acquire the lock on the current view cache.
        ///
        /// There is two rules when acquiring data lock:
        /// - A task should never lock both current view cache and storage at the same time.
        /// - You should do the minimum amount of work inside the closure (e.g. avoid
        ///   doing encryption).
        pub fn with_current_view_cache<T>(&self, cb: impl FnOnce(&mut CurrentViewCache) -> T) -> T {
            #[cfg(debug_assertions)]
            let _lock_tracker_guard =
                DataLockTrackerGuard::register(&self.lock_tracking, DataLock::Cache);

            let mut guard = self.current_view_cache.lock().expect("Mutex is poisoned !");
            cb(&mut guard)
        }

        /// Acquire the lock on the storage.
        ///
        /// There is two rules when acquiring data lock:
        /// - A task should never lock both current view cache and storage at the same time.
        /// - You should do the minimum amount of work inside the closure (e.g. avoid
        ///   doing encryption).
        pub async fn with_storage<T, Fut>(
            &self,
            cb: impl FnOnce(&'static mut Option<WorkspaceStorage>) -> Fut,
        ) -> T
        where
            Fut: std::future::Future<Output = T>,
        {
            let mut guard = self.storage.lock().await;
            let storage_mut_ref = &mut *guard;

            unsafe fn pretend_static(
                src: &mut Option<WorkspaceStorage>,
            ) -> &'static mut Option<WorkspaceStorage> {
                std::mem::transmute(src)
            }
            // SAFETY: It is not currently possible to express the fact the lifetime
            // of a Future returned by a closure depends on the closure parameter if
            // they are references.
            // Here things are even worst because we have references coming from
            // `for_write` body and from `cb` closure (so workarounds as boxed future
            // don't work).
            // However in practice all our references have a lifetime bound to the
            // parent (i.e. `for_write`) or the grand-parent (i.e.
            // `CertificateOps::add_certificates_batch`) which are going to poll this
            // future directly, so the references' lifetimes *are* long enough.
            // TODO: Remove this once async closure are available
            let static_storage_mut_ref = unsafe { pretend_static(storage_mut_ref) };

            #[cfg(debug_assertions)]
            let _lock_tracker_guard =
                DataLockTrackerGuard::register(&self.lock_tracking, DataLock::Storage);

            let fut = cb(static_storage_mut_ref);
            fut.await
        }
    }

    /*
     * Deadlock detection stuff
     */

    #[cfg(debug_assertions)]
    #[derive(Debug)]
    enum DataLock {
        Cache,
        Storage,
    }

    #[cfg(debug_assertions)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum RunningFutureID {
        /// The future is being executed as a task (i.e. `tokio::spawn(<my_future>)`).
        AsTask(TaskID),
        /// The future is being executed as-is by a thread (i.e. `block_on(<my_future>)`).
        FromBlockOn(std::thread::ThreadId),
    }

    #[cfg(debug_assertions)]
    struct DataLockTrackerGuard<'a> {
        running_future_id: RunningFutureID,
        lock_tracking: &'a Mutex<Vec<(RunningFutureID, DataLock)>>,
    }

    #[cfg(debug_assertions)]
    impl<'a> DataLockTrackerGuard<'a> {
        fn register(
            lock_tracking: &'a Mutex<Vec<(RunningFutureID, DataLock)>>,
            lock: DataLock,
        ) -> Self {
            // Task ID is only available when within a task (i.e. a spawned future) !
            //
            // Put it another way, `try_task_id()` will return `None` if:
            // a) It is called from a synchronous context (e.g. the main function).
            // b) It is called from an asynchronous context that is not a task.
            //
            // In practice a) is not a problem since this code is deep within asynchronous
            // land, however b) is very common since it occurs whenever Tokio's `block_on`
            // is used, i.e.:
            // b1) Functions decorated by `tokio::main` (using `tokio::Runtime::block_on` under the hood).
            // b2) Functions decorated by `tokio::test` (using `tokio::Runtime::block_on` under the hood).
            // b3) Non Tokio threads calling `tokio::Handle::block_on`.
            //
            // So, in a nutshell, we are going to end up with `None` as task ID in any
            // code coming from:
            // - Our CLI main functions (due to b1).
            // - Our test functions (due to b2).
            // - Mountpoint operations on Windows (due to b3). This is because WinFSP
            //   schedules them on a pool of threads with a synchronous API.
            //
            // And now the final twist: whenever task ID is `None`, it means we are in a
            // thread doing a `block_on` on a future. This means we can simply use the ID
            // of the thread to identify this running future \o/
            let id = {
                match try_task_id() {
                    Some(task_id) => RunningFutureID::AsTask(task_id),
                    None => RunningFutureID::FromBlockOn(std::thread::current().id()),
                }
            };

            let mut guard = lock_tracking.lock().expect("Mutex is poisoned !");
            for (candidate_id, candidate_lock) in guard.iter() {
                if *candidate_id == id {
                    panic!("Running future {:?} is trying to acquire {:?} lock while holding another {:?} lock !", id, lock, *candidate_lock);
                }
            }
            guard.push((id, lock));

            Self {
                running_future_id: id,
                lock_tracking,
            }
        }
    }

    #[cfg(debug_assertions)]
    impl Drop for DataLockTrackerGuard<'_> {
        fn drop(&mut self) {
            let mut guard = self.lock_tracking.lock().expect("Mutex is poisoned !");
            let index = guard
                .iter()
                .position(|(running_future_id, _)| *running_future_id == self.running_future_id)
                .expect("Running future not found !");
            guard.swap_remove(index);
        }
    }
}

#[derive(Debug)]
pub(super) struct WorkspaceStore {
    realm_id: VlobID,
    device: Arc<LocalDevice>,
    cmds: Arc<AuthenticatedCmds>,
    certificates_ops: Arc<CertificateOps>,

    data: data::WorkspaceStoreData,
    prevent_sync_pattern: PreventSyncPattern,
}

impl std::panic::UnwindSafe for WorkspaceStore {}

impl WorkspaceStore {
    pub async fn start(
        data_base_dir: &Path,
        device: Arc<LocalDevice>,
        cmds: Arc<AuthenticatedCmds>,
        certificates_ops: Arc<CertificateOps>,
        cache_size: u64,
        realm_id: VlobID,
        prevent_sync_pattern: &PreventSyncPattern,
    ) -> Result<Self, anyhow::Error> {
        // 1) Open the database

        let mut storage =
            WorkspaceStorage::start(data_base_dir, &device, realm_id, cache_size).await?;

        // 2) Load the workspace manifest (as it must always be synchronously available)

        let maybe_root_manifest = storage.get_manifest(realm_id).await?;
        let root_manifest = match maybe_root_manifest {
            Some(encrypted) => {
                // TODO: if we cannot load this user manifest, should we fallback on
                //       a new speculative manifest ?
                LocalWorkspaceManifest::decrypt_and_load(&encrypted, &device.local_symkey)
                    .context("Cannot load workspace manifest from local storage")?
            }
            // It is possible to lack the workspace manifest in local if our
            // device hasn't tried to access it yet (and we are not the device
            // that created this workspace, in which case the workspace local db
            // is initialized with a non-speculative local manifest placeholder).
            // In such case it is easy to fall back on an empty manifest
            // which is a good enough approximation of the very first version
            // of the manifest (field `created` is invalid, but it will be
            // corrected by the merge during sync).
            None => {
                let timestamp = device.now();
                let manifest =
                    LocalWorkspaceManifest::new(device.device_id, realm_id, timestamp, true);

                // We must store the speculative manifest in local storage, otherwise there
                // is no way for the outbound sync monitors to realize a synchronization is
                // needed here !
                let update_data = UpdateManifestData {
                    entry_id: manifest.0.base.id,
                    base_version: manifest.0.base.version,
                    need_sync: manifest.0.need_sync,
                    encrypted: manifest.0.dump_and_encrypt(&device.local_symkey),
                };
                storage.update_manifest(&update_data).await?;

                manifest
            }
        };

        // 3) Ensure the prevent sync pattern is applied to the workspace
        prevent_sync_pattern::ensure_prevent_sync_pattern_applied_to_wksp(
            &mut storage,
            device.clone(),
            prevent_sync_pattern,
        )
        .await?;

        // 4) All set !

        Ok(Self {
            realm_id,
            device,
            cmds,
            certificates_ops,
            data: data::WorkspaceStoreData::new(storage, root_manifest),
            prevent_sync_pattern: prevent_sync_pattern.clone(),
        })
    }

    pub async fn stop(&self) -> anyhow::Result<()> {
        self.data
            .with_storage(|maybe_storage| async move {
                if let Some(storage) = maybe_storage.take() {
                    storage.stop().await
                } else {
                    Ok(())
                }
            })
            .await?;
        Ok(())
    }

    pub async fn resolve_path(
        &self,
        path: &FsPath,
    ) -> Result<(ArcLocalChildManifest, PathConfinementPoint), ResolvePathError> {
        resolve_path::resolve_path(self, path).await
    }

    pub async fn retrieve_path_from_id(
        &self,
        entry_id: VlobID,
    ) -> Result<RetrievePathFromIDEntry, RetrievePathFromIDError> {
        resolve_path::retrieve_path_from_id(self, entry_id).await
    }

    pub fn get_root_manifest(&self) -> Arc<LocalFolderManifest> {
        self.data
            .with_current_view_cache(|cache| cache.manifests.root_manifest().clone())
    }

    pub async fn get_manifest(
        &self,
        entry_id: VlobID,
    ) -> Result<ArcLocalChildManifest, GetManifestError> {
        manifest_access::get_manifest(self, entry_id).await
    }

    /// Don't blindly trust folder manifest's `children` field !
    ///
    /// It may contain invalid data (i.e. referencing a non existing child ID, or a child
    /// which `parent` field not matching).
    ///
    /// Hence this method that ensure the potential child exists and *is* a child of the parent.
    pub async fn ensure_manifest_exists_with_parent(
        &self,
        child_id: VlobID,
        expected_parent_id: VlobID,
    ) -> Result<Option<ArcLocalChildManifest>, EnsureManifestExistsWithParentError> {
        let child_manifest = match self.get_manifest(child_id).await {
            Ok(manifest) => manifest,
            Err(err) => {
                return match err {
                    GetManifestError::EntryNotFound => Ok(None),
                    GetManifestError::Offline(e) => {
                        Err(EnsureManifestExistsWithParentError::Offline(e))
                    }
                    GetManifestError::Stopped => Err(EnsureManifestExistsWithParentError::Stopped),
                    GetManifestError::NoRealmAccess => {
                        Err(EnsureManifestExistsWithParentError::NoRealmAccess)
                    }
                    GetManifestError::InvalidKeysBundle(err) => {
                        Err(EnsureManifestExistsWithParentError::InvalidKeysBundle(err))
                    }
                    GetManifestError::InvalidCertificate(err) => {
                        Err(EnsureManifestExistsWithParentError::InvalidCertificate(err))
                    }
                    GetManifestError::InvalidManifest(err) => {
                        Err(EnsureManifestExistsWithParentError::InvalidManifest(err))
                    }
                    GetManifestError::Internal(err) => Err(err.into()),
                }
            }
        };

        if child_manifest.parent() == expected_parent_id {
            Ok(Some(child_manifest))
        } else {
            Ok(None)
        }
    }

    /// Lock the folder entry for update.
    /// Notes:
    /// - Server access may occur to fetch the entry manifest if it is missing.
    /// - If the entry is already locked for update, this method will wait until it
    ///   can take the lock.
    ///  - The returned `FolderUpdater` is designed to have a short lifetime (as it is
    ///    not possible to open a folder), hence the fact it releases the update lock
    ///    on drop.
    pub async fn for_update_folder(
        &self,
        entry_id: VlobID,
    ) -> Result<(FolderUpdater<'_>, Arc<LocalFolderManifest>), ForUpdateFolderError> {
        folder_updater::for_update_folder(self, entry_id).await
    }

    /// Resolve the given path, then lock the folder entry for update.
    /// Notes:
    /// - Server access may occur to fetch missing manifest for the path resolution or
    ///   to get the entry manifest.
    /// - If the entry is already locked for update, this method will wait until it
    ///   can take the lock.
    pub async fn resolve_path_for_update_folder<'a>(
        &'a self,
        path: &FsPath,
    ) -> Result<
        (
            Arc<LocalFolderManifest>,
            PathConfinementPoint,
            FolderUpdater<'a>,
        ),
        ForUpdateFolderError,
    > {
        folder_updater::resolve_path_for_update_folder(self, path).await
    }

    /// Lock the folder entry for update.
    /// Notes:
    /// - Server access may occur to fetch the entry manifest if it is missing.
    /// - The returned `FileUpdater` is designed to have a long lifetime (typically
    ///   as long a the file is open) and hence must be manually closed once you are done.
    /// - If the entry is already locked for update, this method will return
    ///   without waiting. This is because in this case the caller is supposed to
    ///   already posses the `FileUpdater` responsible for the lock.
    pub async fn for_update_file(
        &self,
        entry_id: VlobID,
        wait: bool,
    ) -> Result<(FileUpdater, Arc<LocalFileManifest>), ForUpdateFileError> {
        file_updater::for_update_file(self, entry_id, wait).await
    }

    // Note, unlike for folder, there is no `resolve_path_for_update_file` method.
    // In theory this method should be used by `WorkspaceOps::open_file`, but it is
    // not the case due to how is implemented create-on-open.

    /// As its name suggest, this methods won't fetch missing entry from the server.
    ///
    /// The reason is pretty obvious: this method is meant to be used while synchronize
    /// data between client and server, which is precisely what is needed when fetching
    /// missing data from the server !
    pub async fn for_update_sync_local_only(
        &self,
        entry_id: VlobID,
    ) -> Result<(SyncUpdater, Option<ArcLocalChildManifest>), ForUpdateSyncLocalOnlyError> {
        sync_updater::for_update_sync_local_only(self, entry_id).await
    }

    /// Get back the local entry, resolve its path, then lock it for update.
    ///
    /// This method as a peculiar behavior:
    /// - Any missing manifest needed to resolve the path will be downloaded from the server.
    /// - The entry itself will only be fetched from local data.
    ///
    /// This is because this method is expected to be used to synchronize this very
    /// entry from/to the server.
    pub async fn for_update_sync(
        &self,
        entry_id: VlobID,
        wait: bool,
    ) -> Result<(SyncUpdater, RetrievePathFromIDEntry), ForUpdateSyncError> {
        sync_updater::for_update_sync(self, entry_id, wait).await
    }

    pub async fn resolve_path_for_update_reparenting<'a>(
        &'a self,
        src_parent_path: &FsPath,
        src_child_name: &EntryName,
        dst_parent_path: &FsPath,
        maybe_dst_child_name: Option<&EntryName>,
    ) -> Result<ReparentingUpdater<'a>, ForUpdateReparentingError> {
        reparent_updater::resolve_path_for_update_reparenting(
            self,
            src_parent_path,
            src_child_name,
            dst_parent_path,
            maybe_dst_child_name,
        )
        .await
    }

    pub async fn is_entry_locked(&self, entry_id: VlobID) -> bool {
        self.data
            .with_current_view_cache(|cache| cache.lock_update_manifests.is_taken(entry_id))
    }

    pub async fn get_chunk_or_block_local_only(
        &self,
        chunk_view: &ChunkView,
    ) -> Result<Bytes, ReadChunkOrBlockLocalOnlyError> {
        let found = self
            .data
            .with_current_view_cache(|cache| cache.chunks.get(&chunk_view.id).cloned());
        if let Some(data) = found {
            return Ok(data);
        }

        // Cache miss ! Try to fetch from the local storage

        let maybe_encrypted = self
            .data
            .with_storage(|maybe_storage| async move {
                let storage = maybe_storage
                    .as_mut()
                    .ok_or_else(|| ReadChunkOrBlockLocalOnlyError::Stopped)?;

                let mut maybe_encrypted = storage.get_chunk(chunk_view.id).await?;
                if maybe_encrypted.is_none() {
                    maybe_encrypted = storage
                        .get_block(chunk_view.id.into(), self.device.now())
                        .await?;
                }

                Result::<_, ReadChunkOrBlockLocalOnlyError>::Ok(maybe_encrypted)
            })
            .await?;

        if let Some(encrypted) = maybe_encrypted {
            let data: Bytes = self
                .device
                .local_symkey
                .decrypt(&encrypted)
                .map_err(|err| anyhow::anyhow!("Cannot decrypt block from local storage: {}", err))?
                .into();

            // Don't forget to update the cache !

            self.data.with_current_view_cache(|cache| {
                cache.chunks.push(chunk_view.id, data.clone());
            });

            return Ok(data);
        }

        Err(ReadChunkOrBlockLocalOnlyError::ChunkNotFound)
    }

    pub async fn get_chunk_or_block(
        &self,
        chunk_view: &ChunkView,
        remote_manifest: &FileManifest,
    ) -> Result<Bytes, ReadChunkOrBlockError> {
        let outcome = self.get_chunk_or_block_local_only(chunk_view).await;
        match outcome {
            Ok(chunk_data) => return Ok(chunk_data),
            Err(err) => match err {
                ReadChunkOrBlockLocalOnlyError::ChunkNotFound => (),
                ReadChunkOrBlockLocalOnlyError::Stopped => {
                    return Err(ReadChunkOrBlockError::Stopped)
                }
                ReadChunkOrBlockLocalOnlyError::Internal(err) => return Err(err.into()),
            },
        }

        // Chunk not in local, time to ask the server

        let access = match &chunk_view.access {
            Some(access) => access,
            None => return Err(ReadChunkOrBlockError::ChunkNotFound),
        };

        let data = server_fetch_block(
            &self.cmds,
            &self.certificates_ops,
            self.realm_id,
            remote_manifest,
            access,
        )
        .await
        .map_err(|err| match err {
            ServerFetchBlockError::Stopped => ReadChunkOrBlockError::Stopped,
            ServerFetchBlockError::Offline(e) => ReadChunkOrBlockError::Offline(e),
            ServerFetchBlockError::ServerBlockstoreUnavailable => {
                ReadChunkOrBlockError::ServerBlockstoreUnavailable
            }
            ServerFetchBlockError::BlockNotFound => {
                // This is unexpected: we got a block ID from a file manifest,
                // but this ID points to nothing according to the server :/
                //
                // That could means two things:
                // - The server is lying to us.
                // - The client that has uploaded the file manifest was buggy
                //   and included the ID of a not-yet-synchronized block.
                ReadChunkOrBlockError::ChunkNotFound
            }
            ServerFetchBlockError::RealmNotFound => {
                // The realm doesn't exist on server side, hence we are its creator and
                // its data only live on our local storage, which we have already checked.
                ReadChunkOrBlockError::ChunkNotFound
            }
            ServerFetchBlockError::NoRealmAccess => ReadChunkOrBlockError::NoRealmAccess,
            ServerFetchBlockError::InvalidBlockAccess(err) => {
                ReadChunkOrBlockError::InvalidBlockAccess(err)
            }
            ServerFetchBlockError::InvalidKeysBundle(err) => {
                ReadChunkOrBlockError::InvalidKeysBundle(err)
            }
            ServerFetchBlockError::InvalidCertificate(err) => {
                ReadChunkOrBlockError::InvalidCertificate(err)
            }
            ServerFetchBlockError::Internal(err) => {
                err.context("cannot fetch block from server").into()
            }
        })?;

        // Now that we have received the block data from the server, it's time to
        // ask ourself about concurrency. What if another task also got a cache miss
        // and is currently fetching the same block ?
        //
        // Long story short: not doing anything about it is fine.
        //
        // Long version:
        //
        // The risk here is to have two concurrent tasks wanting to populate a given
        // block with different data (in case the server is buggy/malicious).
        //
        // However, the block data has been validated against a block access containing
        // its hash. So to have a given block changing between two concurrent populates,
        // it must be two different manifests that are accessed concurrently and which
        // both reference the same block ID but with different block hash.
        //
        // This is a purely theoretical case since a block can only be uploaded once.
        // The most likely reason for ending up with this would be if we get a block ID
        // collision (given when a block upload is rejected because the server already
        // contains one with this ID, the client assumes the block data on the server
        // are the one it wanted to upload...).

        // Should both store the data in local storage...

        let encrypted = self.device.local_symkey.encrypt(&data);
        self.data
            .with_storage(|maybe_storage| async move {
                let storage = maybe_storage
                    .as_mut()
                    .ok_or_else(|| ReadChunkOrBlockError::Stopped)?;

                storage
                    .set_block(access.id, &encrypted, self.device.now())
                    .await
                    .map_err(ReadChunkOrBlockError::Internal)
            })
            .await?;

        // ...and update the cache !

        self.data.with_current_view_cache(|cache| {
            cache.chunks.push(chunk_view.id, data.clone());
        });

        Ok(data)
    }

    pub async fn get_not_uploaded_chunk(
        &self,
        chunk_id: ChunkID,
    ) -> Result<Option<Bytes>, GetNotUploadedChunkError> {
        // Don't use the in-memory cache given it doesn't tell if the data is from and
        // uploaded block or not.
        let maybe_encrypted = self
            .data
            .with_storage(|maybe_storage| async move {
                let storage = maybe_storage
                    .as_mut()
                    .ok_or_else(|| GetNotUploadedChunkError::Stopped)?;

                storage
                    .get_chunk(chunk_id)
                    .await
                    .map_err(GetNotUploadedChunkError::Internal)
            })
            .await?;

        match maybe_encrypted {
            None => Ok(None),
            Some(encrypted) => {
                let cleartext = self
                    .device
                    .local_symkey
                    .decrypt(&encrypted)
                    .map_err(|err| {
                        anyhow::anyhow!("Cannot decrypt block from local storage: {}", err)
                    })?;

                Ok(Some(cleartext.into()))
            }
        }
    }

    pub async fn promote_local_only_chunk_to_uploaded_block(
        &self,
        chunk_id: ChunkID,
    ) -> Result<(), PromoteLocalOnlyChunkToUploadedBlockError> {
        self.data
            .with_storage(|maybe_storage| async move {
                let storage = maybe_storage
                    .as_mut()
                    .ok_or_else(|| PromoteLocalOnlyChunkToUploadedBlockError::Stopped)?;

                storage
                    .promote_chunk_to_block(chunk_id, self.device.now())
                    .await
                    .map_err(PromoteLocalOnlyChunkToUploadedBlockError::Internal)
            })
            .await
    }

    pub async fn get_inbound_need_sync_entries(
        &self,
        limit: u32,
    ) -> Result<Vec<VlobID>, GetNeedSyncEntriesError> {
        self.data
            .with_storage(|maybe_storage| async move {
                let storage = maybe_storage
                    .as_mut()
                    .ok_or_else(|| GetNeedSyncEntriesError::Stopped)?;

                storage
                    .get_inbound_need_sync(limit)
                    .await
                    .map_err(GetNeedSyncEntriesError::Internal)
            })
            .await
    }

    pub async fn get_outbound_need_sync_entries(
        &self,
        limit: u32,
    ) -> Result<Vec<VlobID>, GetNeedSyncEntriesError> {
        self.data
            .with_storage(|maybe_storage| async move {
                let storage = maybe_storage
                    .as_mut()
                    .ok_or_else(|| GetNeedSyncEntriesError::Stopped)?;

                storage
                    .get_outbound_need_sync(limit)
                    .await
                    .map_err(GetNeedSyncEntriesError::Internal)
            })
            .await
    }

    pub async fn get_realm_checkpoint(&self) -> Result<IndexInt, GetRealmCheckpointError> {
        self.data
            .with_storage(|maybe_storage| async move {
                let storage = maybe_storage
                    .as_mut()
                    .ok_or_else(|| GetRealmCheckpointError::Stopped)?;

                storage
                    .get_realm_checkpoint()
                    .await
                    .map_err(GetRealmCheckpointError::Internal)
            })
            .await
    }

    pub async fn update_realm_checkpoint(
        &self,
        current_checkpoint: IndexInt,
        changes: &[(VlobID, VersionInt)],
    ) -> Result<(), UpdateRealmCheckpointError> {
        self.data
            .with_storage(|maybe_storage| async move {
                let storage = maybe_storage
                    .as_mut()
                    .ok_or_else(|| UpdateRealmCheckpointError::Stopped)?;

                storage
                    .update_realm_checkpoint(current_checkpoint, changes)
                    .await
                    .map_err(UpdateRealmCheckpointError::Internal)
            })
            .await
    }
}
