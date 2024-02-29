// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex},
};

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_platform_async::{
    event::{Event, EventListener},
    lock::{Mutex as AsyncMutex, MutexGuard as AsyncMutexGuard},
};
use libparsec_platform_storage::workspace::{UpdateManifestData, WorkspaceStorage};
use libparsec_types::prelude::*;

use crate::{
    certif::{CertifOps, InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError},
    workspace::fetch::FetchRemoteManifestError,
};

enum FetchMode {
    LocalOnly,
    LocalAndRemote,
}

pub(super) struct FsPathResolution {
    pub entry_id: VlobID,
    /// The confinement point corresponds to the entry id of the folderish manifest
    /// (i.e. file or workspace manifest) that contains a child with a confined name
    /// in the corresponding path.
    ///
    /// If the entry is not confined, the confinement point is `None`.
    pub confinement_point: Option<VlobID>,
}

pub(super) enum FsPathResolutionAndManifest {
    Workspace {
        manifest: Arc<LocalWorkspaceManifest>,
    },
    Folder {
        manifest: Arc<LocalFolderManifest>,
        confinement_point: Option<VlobID>,
    },
    File {
        manifest: Arc<LocalFileManifest>,
        confinement_point: Option<VlobID>,
    },
}

pub(super) enum FolderishManifestAndUpdater<'a> {
    Root {
        manifest: Arc<LocalWorkspaceManifest>,
        updater: RootUpdater<'a>,
    },
    Folder {
        manifest: Arc<LocalFolderManifest>,
        updater: FolderUpdater<'a>,
        // TODO: confinement support not implemented yet
        #[allow(unused)]
        confinement_point: Option<VlobID>,
    },
}

#[derive(Debug, thiserror::Error)]
pub(super) enum GetEntryError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("Path doesn't exist")]
    EntryNotFound,
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

#[derive(Debug, thiserror::Error)]
pub(super) enum FetchChildManifestAndPopulateCacheError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("Path doesn't exist")]
    EntryNotFound,
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

#[derive(Debug, thiserror::Error)]
pub(super) enum ForUpdateFolderError {
    #[error("Cannot reach the server")]
    Offline,
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

#[derive(Debug, thiserror::Error)]
pub(super) enum ForUpdateFileError {
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

#[derive(Debug, thiserror::Error)]
pub(super) enum ForUpdateChildLocalOnlyError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Entry is already being updated")]
    WouldBlock,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub(super) enum GetFolderishEntryError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("Path doesn't exist")]
    EntryNotFound,
    #[error("Path points to an entry that is a file")]
    EntryIsFile,
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

#[derive(Debug, thiserror::Error)]
pub(super) enum ReadChunkLocalOnlyError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Chunk doesn't exist")]
    ChunkNotFound,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub(super) enum ReadChunkError {
    #[error("Cannot reach the server")]
    Offline,
    #[error("Component has stopped")]
    Stopped,
    #[error("Chunk doesn't exist")]
    ChunkNotFound,
    #[error("Not allowed to access this realm")]
    NoRealmAccess,
    #[error("Block access is temporary unavailable on the server")]
    StoreUnavailable,
    #[error("Block cannot be decrypted")]
    BadDecryption,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

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
pub(super) type UpdateWorkspaceManifestError = WorkspaceStoreOperationError;
pub(super) type UpdateFolderManifestError = WorkspaceStoreOperationError;
pub(super) type UpdateFileManifestAndContinueError = WorkspaceStoreOperationError;

/*
 * Child manifests lock
 */

mod child_manifests_lock {
    use super::*;

    /// This structure provide a way to lock independently each child manifest for update.
    /// Once taken, a `ChildManifestLockGuard` is returned to the caller, which must
    /// be in turn provided to `release` once the lock should be released.
    #[derive(Debug)]
    pub(super) struct ChildManifestsLock {
        per_manifest_lock: Vec<(VlobID, EntryLockState)>,
    }

    // Unlike for workspace manifest, we cannot just use a simple async mutex to
    // protect update of child manifests.
    //
    // This is because the child manifests cache is itself behind a sync mutex (which
    // must be released before waiting on the per child manifest async mutex !).
    // The natural solution to this is to wrap the per child manifest async mutex
    // in an Arc, but this creates a new issue: the `ChildManifestLockGuard`
    // would then have to keep both the Arc<AsyncMutex> and the corresponding `AsyncMutexGuard`.
    // However this is not possible given the guard takes a reference on the mutex !
    //
    // Hence we basically have to re-implement a custom async mutex using the lower-level
    // event system.
    #[derive(Debug)]
    enum EntryLockState {
        /// A single coroutine has locked the manifest
        Taken,
        /// Multiple coroutines want to lock the manifest: the first has the lock
        /// and the subsequent ones are listening on the event (which will be fired
        /// when the first coroutine releases the manifest).
        /// Note this means all coroutines waiting for the event will fight with no
        /// fairness to take the lock (it's basically up to the tokio scheduler).
        TakenWithConcurrency(Event),
    }

    pub(super) enum ChildManifestLockTakeOutcome {
        // The updater object is responsible to release the update lock on drop.
        // Hence this object must be created as soon as possible otherwise a deadlock
        // could occur (typically in case of an early return due to error handling)
        Taken(ChildManifestLockGuard),
        NeedWait(EventListener),
    }

    /// Note this should not be made `Clone` !
    #[derive(Debug)]
    pub(super) struct ChildManifestLockGuard {
        entry_id: VlobID,
        #[allow(unused)]
        is_released: bool,
    }

    // TODO: Lock is useful to detect misuse of the guard, but it is also very
    // annoying given it is triggered any time another error in the tests makes
    // us skip the release part. Worst, in this case the only outputted error
    // is the panic message, which is not very helpful.

    // impl Drop for ChildManifestLockGuard {
    //     fn drop(&mut self) {
    //         if !self.is_released {

    //             panic!("Child manifest `{}` guard dropped without being released !", self.entry_id);
    //         }
    //         assert!(
    //             self.is_released,
    //             "Child manifest `{}` guard dropped without being released !",
    //             self.entry_id,
    //         );
    //     }
    // }

    impl ChildManifestsLock {
        pub(super) fn new() -> Self {
            Self {
                per_manifest_lock: Vec::new(),
            }
        }

        pub(super) fn is_taken(&self, entry_id: VlobID) -> bool {
            self.per_manifest_lock.iter().any(|(id, _)| *id == entry_id)
        }

        pub(super) fn take(&mut self, entry_id: VlobID) -> ChildManifestLockTakeOutcome {
            let found = self
                .per_manifest_lock
                .iter_mut()
                .find(|(id, _)| *id == entry_id);

            match found {
                // Entry is missing: the manifest is not currently under update
                None => {
                    self.per_manifest_lock
                        .push((entry_id, EntryLockState::Taken));
                    let guard = ChildManifestLockGuard {
                        entry_id,
                        is_released: false,
                    };
                    // It's official: we are now the one and only updating this manifest !
                    ChildManifestLockTakeOutcome::Taken(guard)
                }

                // The entry is present: the manifest is already taken for update !
                Some((_, state)) => match state {
                    // Appart from the coroutine currently updating the manifest, nobody
                    // else is waiting for it... except us !
                    // So it's our job to setup the event to get notified when the manifest
                    // is again available for update.
                    EntryLockState::Taken => {
                        let event = Event::new();
                        let listener = event.listen();
                        *state = EntryLockState::TakenWithConcurrency(event);
                        ChildManifestLockTakeOutcome::NeedWait(listener)
                    }
                    // Multiple coroutines are already waiting to get their hands on
                    // this manifest, we are just the n+1 :)
                    EntryLockState::TakenWithConcurrency(event) => {
                        ChildManifestLockTakeOutcome::NeedWait(event.listen())
                    }
                },
            }
        }

        pub(super) fn try_take(&mut self, entry_id: VlobID) -> Option<ChildManifestLockGuard> {
            let found = self
                .per_manifest_lock
                .iter_mut()
                .find(|(id, _)| *id == entry_id);

            match found {
                // Entry is missing: the manifest is not currently under update
                None => {
                    self.per_manifest_lock
                        .push((entry_id, EntryLockState::Taken));
                    let guard = ChildManifestLockGuard {
                        entry_id,
                        is_released: false,
                    };
                    // It's official: we are now the one and only updating this manifest !
                    Some(guard)
                }

                // The entry is present: the manifest is already taken for update !
                Some(_) => None,
            }
        }

        pub(super) fn release(&mut self, mut guard: ChildManifestLockGuard) {
            let (index, (_, state)) = self
                .per_manifest_lock
                .iter()
                .enumerate()
                .find(|(_, (id, _))| *id == guard.entry_id)
                // `ChildManifestGuard` constructor is private, so it can only be created
                // in `take`, and is only removed once here (since we take ownership).
                .expect("Must be present");

            // If other coroutines are waiting for the manifest, now is the time to notify them !
            if let EntryLockState::TakenWithConcurrency(event) = state {
                event.notify(usize::MAX);
            }

            // We remove the entry altogether, this is the way to signify it is free to take now
            self.per_manifest_lock.swap_remove(index);

            // Finally mark the guard as released to pass the sanity check in it drop
            guard.is_released = true;
        }

        // See `WorkspaceDataStorageChildManifestUpdater`'s drop for the release part
    }
}
use child_manifests_lock::{
    ChildManifestLockGuard, ChildManifestLockTakeOutcome, ChildManifestsLock,
};

use super::fetch::FetchRemoteBlockError;

/*
 * WorkspaceStore & friends
 */

/// Very simple cache for chunks: we consider a chunk is most of the time much bigger
/// than a typical kernel read (512ko vs 4ko), so it's a big win to just keep the
/// chunks currently being read in memory.
/// To approximate that, we just keep the last 16 chunks read in memory.
#[derive(Debug)]
struct ChunksCache {
    items: [Option<(ChunkID, Bytes)>; 16],
    round_robin: usize,
}

impl ChunksCache {
    fn new() -> Self {
        Self {
            items: Default::default(),
            round_robin: 0,
        }
    }
    fn push(&mut self, id: ChunkID, data: Bytes) {
        self.items[self.round_robin] = Some((id, data));
        self.round_robin = (self.round_robin + 1) % self.items.len();
    }
    fn get(&self, id: &ChunkID) -> Option<Bytes> {
        self.items
            .iter()
            .find_map(|item| item.as_ref().filter(|(chunk_id, _)| chunk_id == id))
            .map(|(_, data)| data.to_owned())
    }
}

#[derive(Debug)]
struct CurrentViewCache {
    workspace_manifest: Arc<LocalWorkspaceManifest>,
    // `child_manifests` contains a cache on the database:
    // - the cache may be cleaned at any given time (i.e. inserting an entry in the cache
    //   doesn't guarantee it will be available later on)
    // - if the cache is present, it always correspond to the latest value (so the cache
    //   should always be preferred over data coming from the database)
    // - each cache entry can be "taken" for write access. In this mode
    child_manifests: HashMap<VlobID, ArcLocalChildManifest>,
    // Just like for workspace manifest, each child manifest has a dedicated async lock
    // to prevent concurrent update (ensuring consistency between cache and database).
    lock_update_child_manifests: ChildManifestsLock,
    chunks: ChunksCache,
}

#[derive(Debug)]
pub(super) struct WorkspaceStore {
    realm_id: VlobID,
    device: Arc<LocalDevice>,
    cmds: Arc<AuthenticatedCmds>,
    certificates_ops: Arc<CertifOps>,

    // A lock that will be used to prevent concurrent update on the workspace manifest.
    // This is needed to ensure the manifest in cache stays in sync with the content of the database.
    lock_update_workspace_manifest: AsyncMutex<()>,
    current_view_cache: Mutex<CurrentViewCache>,
    // Given accessing `storage` requires exclusive access, it is better to have it
    // under its own lock so that all cache hit operations can occur concurrently.
    storage: AsyncMutex<Option<WorkspaceStorage>>,
}

impl WorkspaceStore {
    pub(crate) async fn start(
        data_base_dir: &Path,
        device: Arc<LocalDevice>,
        cmds: Arc<AuthenticatedCmds>,
        certificates_ops: Arc<CertifOps>,
        cache_size: u64,
        realm_id: VlobID,
    ) -> Result<Self, anyhow::Error> {
        // 1) Open the database

        let mut storage =
            WorkspaceStorage::start(data_base_dir, &device, realm_id, cache_size).await?;

        // 2) Load the workspace manifest (as it must always be synchronously available)

        let maybe_workspace_manifest = storage.get_manifest(realm_id).await?;
        let workspace_manifest = match maybe_workspace_manifest {
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
                LocalWorkspaceManifest::new(
                    device.device_id.clone(),
                    timestamp,
                    Some(realm_id),
                    true,
                )
            }
        };

        // 3) All set !

        Ok(Self {
            realm_id,
            device,
            cmds,
            certificates_ops,
            lock_update_workspace_manifest: AsyncMutex::new(()),
            current_view_cache: Mutex::new(CurrentViewCache {
                workspace_manifest: Arc::new(workspace_manifest),
                child_manifests: HashMap::new(),
                lock_update_child_manifests: ChildManifestsLock::new(),
                chunks: ChunksCache::new(),
            }),
            storage: AsyncMutex::new(Some(storage)),
        })
    }

    pub(crate) async fn stop(&self) -> anyhow::Result<()> {
        let maybe_storage = self.storage.lock().await.take();
        if let Some(storage) = maybe_storage {
            storage.stop().await?;
        }
        Ok(())
    }

    pub(crate) async fn resolve_path(
        &self,
        path: &FsPath,
    ) -> Result<FsPathResolution, GetEntryError> {
        enum Parent {
            Root,
            /// The parent is itself the child of someone else
            Child(FsPathResolution),
        }
        let mut parent = Parent::Root;
        let path_parts = path.parts();
        let mut parts_index = 0;

        loop {
            // Most of the time we should have each entry in the path already in the cache,
            // so we want to lock the cache once and only release it in the unlikely case
            // we need to fetch from the local storage or server.
            let cache_miss_entry_id = {
                let cache = self.current_view_cache.lock().expect("Mutex is poisoned");
                loop {
                    let child_name = match path_parts.get(parts_index) {
                        Some(part) => part,
                        // The path is entirely resolved !
                        None => {
                            return Ok(match parent {
                                Parent::Root => FsPathResolution {
                                    entry_id: self.realm_id,
                                    confinement_point: None,
                                },
                                Parent::Child(resolution) => resolution,
                            });
                        }
                    };

                    let resolution = match &parent {
                        Parent::Root => {
                            let workspace_manifest = &cache.workspace_manifest;

                            let child_entry_id = workspace_manifest
                                .children
                                .get(child_name)
                                .ok_or(GetEntryError::EntryNotFound)?;

                            let confinement_point = workspace_manifest
                                .local_confinement_points
                                .contains(child_entry_id)
                                .then_some(workspace_manifest.base.id);

                            FsPathResolution {
                                entry_id: *child_entry_id,
                                confinement_point,
                            }
                        }

                        Parent::Child(parent) => {
                            let manifest = match cache.child_manifests.get(&parent.entry_id) {
                                Some(manifest) => manifest.to_owned(),
                                // Cache miss !
                                // `part_index` is not incremented here, so we are going to
                                // leave the second loop, populate the cache, loop into first
                                // loop and finally re-enter the second loop with the same
                                // part in the path to resolve.
                                None => break parent.entry_id,
                            };

                            let (children, local_confinement_points) = match &manifest {
                                ArcLocalChildManifest::File(_) => {
                                    // Cannot continue to resolve the path !
                                    return Err(GetEntryError::EntryNotFound);
                                }
                                ArcLocalChildManifest::Folder(manifest) => {
                                    (&manifest.children, &manifest.local_confinement_points)
                                }
                            };

                            let child_entry_id = children
                                .get(child_name)
                                .ok_or(GetEntryError::EntryNotFound)?;

                            // Top-most confinement point shadows child ones if any
                            let confinement_point = match parent.confinement_point {
                                confinement_point @ Some(_) => confinement_point,
                                None => local_confinement_points
                                    .contains(child_entry_id)
                                    .then_some(parent.entry_id),
                            };

                            FsPathResolution {
                                entry_id: *child_entry_id,
                                confinement_point,
                            }
                        }
                    };

                    parent = Parent::Child(resolution);
                    parts_index += 1;
                }
            };

            // We got a cache miss
            self.fetch_child_manifest_and_populate_cache(
                cache_miss_entry_id,
                FetchMode::LocalAndRemote,
            )
            .await
            .map_err(|err| match err {
                FetchChildManifestAndPopulateCacheError::Offline => GetEntryError::Offline,
                FetchChildManifestAndPopulateCacheError::Stopped => GetEntryError::Stopped,
                FetchChildManifestAndPopulateCacheError::EntryNotFound => {
                    GetEntryError::EntryNotFound
                }
                FetchChildManifestAndPopulateCacheError::NoRealmAccess => {
                    GetEntryError::NoRealmAccess
                }
                FetchChildManifestAndPopulateCacheError::InvalidKeysBundle(err) => {
                    GetEntryError::InvalidKeysBundle(err)
                }
                FetchChildManifestAndPopulateCacheError::InvalidCertificate(err) => {
                    GetEntryError::InvalidCertificate(err)
                }
                FetchChildManifestAndPopulateCacheError::InvalidManifest(err) => {
                    GetEntryError::InvalidManifest(err)
                }
                FetchChildManifestAndPopulateCacheError::Internal(err) => {
                    err.context("cannot fetch manifest").into()
                }
            })?;
        }
    }

    pub(crate) async fn resolve_path_and_get_manifest(
        &self,
        path: &FsPath,
    ) -> Result<FsPathResolutionAndManifest, GetEntryError> {
        if path.is_root() {
            let cache = self.current_view_cache.lock().expect("Mutex is poisoned");
            return Ok(FsPathResolutionAndManifest::Workspace {
                manifest: cache.workspace_manifest.clone(),
            });
        }

        let resolution = self.resolve_path(path).await?;
        let child = self.get_child_manifest(resolution.entry_id).await?;
        match child {
            ArcLocalChildManifest::File(manifest) => Ok(FsPathResolutionAndManifest::File {
                manifest,
                confinement_point: resolution.confinement_point,
            }),
            ArcLocalChildManifest::Folder(manifest) => Ok(FsPathResolutionAndManifest::Folder {
                manifest,
                confinement_point: resolution.confinement_point,
            }),
        }
    }

    pub(crate) async fn resolve_path_for_update_folderish_manifest(
        &self,
        path: &FsPath,
    ) -> Result<FolderishManifestAndUpdater, GetFolderishEntryError> {
        if path.is_root() {
            let (updater, manifest) = self.for_update_root().await;
            return Ok(FolderishManifestAndUpdater::Root { manifest, updater });
        }

        let resolution = self.resolve_path(path).await.map_err(|err| match err {
            GetEntryError::Offline => GetFolderishEntryError::Offline,
            GetEntryError::Stopped => GetFolderishEntryError::Stopped,
            GetEntryError::EntryNotFound => GetFolderishEntryError::EntryNotFound,
            GetEntryError::NoRealmAccess => GetFolderishEntryError::NoRealmAccess,
            GetEntryError::InvalidKeysBundle(err) => GetFolderishEntryError::InvalidKeysBundle(err),
            GetEntryError::InvalidCertificate(err) => {
                GetFolderishEntryError::InvalidCertificate(err)
            }
            GetEntryError::InvalidManifest(err) => GetFolderishEntryError::InvalidManifest(err),
            GetEntryError::Internal(err) => err.context("cannot resolve path").into(),
        })?;

        let (updater, manifest) =
            self.for_update_folder(resolution.entry_id)
                .await
                .map_err(|err| match err {
                    ForUpdateFolderError::Offline => GetFolderishEntryError::Offline,
                    ForUpdateFolderError::Stopped => GetFolderishEntryError::Stopped,
                    ForUpdateFolderError::EntryNotFound => GetFolderishEntryError::EntryNotFound,
                    ForUpdateFolderError::EntryNotAFolder => GetFolderishEntryError::EntryIsFile,
                    ForUpdateFolderError::NoRealmAccess => GetFolderishEntryError::NoRealmAccess,
                    ForUpdateFolderError::InvalidKeysBundle(err) => {
                        GetFolderishEntryError::InvalidKeysBundle(err)
                    }
                    ForUpdateFolderError::InvalidCertificate(err) => {
                        GetFolderishEntryError::InvalidCertificate(err)
                    }
                    ForUpdateFolderError::InvalidManifest(err) => {
                        GetFolderishEntryError::InvalidManifest(err)
                    }
                    ForUpdateFolderError::Internal(err) => {
                        err.context("cannot lock for update").into()
                    }
                })?;

        Ok(FolderishManifestAndUpdater::Folder {
            manifest,
            confinement_point: resolution.confinement_point,
            updater,
        })
    }

    pub(crate) fn get_workspace_manifest(&self) -> Arc<LocalWorkspaceManifest> {
        self.current_view_cache
            .lock()
            .expect("Mutex is poisoned")
            .workspace_manifest
            .clone()
    }

    pub(crate) async fn get_child_manifest(
        &self,
        entry_id: VlobID,
    ) -> Result<ArcLocalChildManifest, GetEntryError> {
        // Fast path: cache lookup
        {
            let cache = self.current_view_cache.lock().expect("Mutex is poisoned");
            if let Some(manifest) = cache.child_manifests.get(&entry_id) {
                return Ok(manifest.clone());
            }
        }

        // Entry not in the cache, try to fetch it from the local storage...
        let manifest = self
            .fetch_child_manifest_and_populate_cache(entry_id, FetchMode::LocalAndRemote)
            .await
            .map_err(|err| match err {
                FetchChildManifestAndPopulateCacheError::Offline => GetEntryError::Offline,
                FetchChildManifestAndPopulateCacheError::Stopped => GetEntryError::Stopped,
                FetchChildManifestAndPopulateCacheError::EntryNotFound => {
                    GetEntryError::EntryNotFound
                }
                FetchChildManifestAndPopulateCacheError::NoRealmAccess => {
                    GetEntryError::NoRealmAccess
                }
                FetchChildManifestAndPopulateCacheError::InvalidKeysBundle(err) => {
                    GetEntryError::InvalidKeysBundle(err)
                }
                FetchChildManifestAndPopulateCacheError::InvalidCertificate(err) => {
                    GetEntryError::InvalidCertificate(err)
                }
                FetchChildManifestAndPopulateCacheError::InvalidManifest(err) => {
                    GetEntryError::InvalidManifest(err)
                }
                FetchChildManifestAndPopulateCacheError::Internal(err) => err.into(),
            })?;

        Ok(manifest)
    }

    async fn fetch_child_manifest_and_populate_cache(
        &self,
        entry_id: VlobID,
        mode: FetchMode,
    ) -> Result<ArcLocalChildManifest, FetchChildManifestAndPopulateCacheError> {
        // Before any slower lookup, handle the special case of the root:
        // this function is aimed at fetching child entry only, so it should
        // never be called with the realm ID (which referenced the workspace
        // manifest).
        // However we don't want to crash if this happens (given a malicious user
        // could craft a regular folder manifest with the realm ID as a child).
        if entry_id == self.realm_id {
            return Err(FetchChildManifestAndPopulateCacheError::EntryNotFound);
        }

        let mut maybe_storage = self.storage.lock().await;
        let storage = match &mut *maybe_storage {
            None => return Err(FetchChildManifestAndPopulateCacheError::Stopped),
            Some(storage) => storage,
        };

        let found = storage.get_manifest(entry_id).await.map_err(|err| {
            FetchChildManifestAndPopulateCacheError::Internal(
                err.context("cannot access local storage"),
            )
        })?;
        let manifest = match (found, mode) {
            // Entry found in the local storage !
            (Some(encrypted), _) => {
                let outcome =
                    LocalChildManifest::decrypt_and_load(&encrypted, &self.device.local_symkey);
                match outcome {
                    Ok(LocalChildManifest::File(manifest)) => {
                        ArcLocalChildManifest::File(Arc::new(manifest))
                    }
                    Ok(LocalChildManifest::Folder(manifest)) => {
                        ArcLocalChildManifest::Folder(Arc::new(manifest))
                    }
                    Err(err) => {
                        return Err(FetchChildManifestAndPopulateCacheError::Internal(
                            anyhow::anyhow!("Local database contains invalid data: {}", err),
                        ))
                    }
                }
            }

            (None, FetchMode::LocalOnly) => {
                return Err(FetchChildManifestAndPopulateCacheError::EntryNotFound)
            }

            // Entry not in the local storage, last chance is to fetch from the server
            (None, FetchMode::LocalAndRemote) => {
                let outcome = super::fetch::fetch_remote_child_manifest(
                    &self.cmds,
                    &self.certificates_ops,
                    self.realm_id,
                    entry_id,
                )
                .await;
                match outcome {
                    Ok(ChildManifest::File(manifest)) => ArcLocalChildManifest::File(Arc::new(
                        LocalFileManifest::from_remote(manifest),
                    )),
                    Ok(ChildManifest::Folder(manifest)) => ArcLocalChildManifest::Folder(Arc::new(
                        LocalFolderManifest::from_remote(manifest, None),
                    )),
                    Err(err) => {
                        return Err(match err {
                            FetchRemoteManifestError::Stopped => {
                                FetchChildManifestAndPopulateCacheError::Stopped
                            }
                            FetchRemoteManifestError::Offline => {
                                FetchChildManifestAndPopulateCacheError::Offline
                            }
                            FetchRemoteManifestError::VlobNotFound => {
                                // This is unexpected: we got an entry ID from a parent folder/workspace
                                // manifest, but this ID points to nothing according to the server :/
                                //
                                // That could means two things:
                                // - the server is lying to us
                                // - the client that have uploaded the parent folder/workspace manifest
                                //   was buggy and include the ID of a not-yet-synchronized entry
                                //
                                // In theory it would be good to do a self-healing here (e.g. remove
                                // the entry from the parent), but this is cumbersome and only possible
                                // if the user has write access.
                                // So instead we just pretend the entry doesn't exist.

                                // TODO: add warning log !
                                FetchChildManifestAndPopulateCacheError::EntryNotFound
                            }
                            // The realm doesn't exist on server side, hence we are it creator and
                            // it data only live on our local storage, which we have already checked.
                            FetchRemoteManifestError::RealmNotFound => {
                                FetchChildManifestAndPopulateCacheError::EntryNotFound
                            }
                            FetchRemoteManifestError::NoRealmAccess => {
                                FetchChildManifestAndPopulateCacheError::NoRealmAccess
                            }
                            FetchRemoteManifestError::InvalidKeysBundle(err) => {
                                FetchChildManifestAndPopulateCacheError::InvalidKeysBundle(err)
                            }
                            FetchRemoteManifestError::InvalidCertificate(err) => {
                                FetchChildManifestAndPopulateCacheError::InvalidCertificate(err)
                            }
                            FetchRemoteManifestError::InvalidManifest(err) => {
                                FetchChildManifestAndPopulateCacheError::InvalidManifest(err)
                            }
                            FetchRemoteManifestError::Internal(err) => {
                                err.context("cannot fetch from server").into()
                            }
                        });
                    }
                }
            }
        };

        // We got our manifest, don't forget to update the cache before returning it
        let mut cache = self.current_view_cache.lock().expect("Mutex is poisoned");
        let manifest = match cache.child_manifests.entry(entry_id) {
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(manifest.clone());
                manifest
            }
            // Plot twist: a concurrent operation has updated the cache !
            // So we discard the data we've fetched and pretend we got a cache hit in
            // the first place.
            std::collections::hash_map::Entry::Occupied(entry) => entry.get().to_owned(),
        };

        Ok(manifest)
    }

    pub(crate) async fn for_update_root(&self) -> (RootUpdater<'_>, Arc<LocalWorkspaceManifest>) {
        let guard = self.lock_update_workspace_manifest.lock().await;

        let manifest = {
            let cache = self.current_view_cache.lock().expect("Mutex is poisoned");
            cache.workspace_manifest.clone()
        };

        let updater = RootUpdater {
            store: self,
            _update_guard: guard,
        };

        (updater, manifest)
    }

    pub(crate) async fn for_update_folder(
        &self,
        entry_id: VlobID,
    ) -> Result<(FolderUpdater<'_>, Arc<LocalFolderManifest>), ForUpdateFolderError> {
        // Guard's drop will panic if the lock is not released
        macro_rules! release_guard_on_error {
            ($entry_guard:expr) => {
                let mut cache_guard = self.current_view_cache.lock().expect("Mutex is poisoned");
                cache_guard
                    .lock_update_child_manifests
                    .release($entry_guard);
            };
        }

        let mut maybe_need_wait = None;
        let (entry_guard, manifest) = loop {
            if let Some(listener) = maybe_need_wait {
                listener.await;
            }

            let entry_guard = {
                // 1) Cache lookup

                let mut cache_guard = self.current_view_cache.lock().expect("Mutex is poisoned");
                let outcome = cache_guard.lock_update_child_manifests.take(entry_id);

                match outcome {
                    ChildManifestLockTakeOutcome::Taken(entry_guard) => {
                        let found = cache_guard.child_manifests.get(&entry_id);
                        if let Some(manifest) = found {
                            // Cache hit ! We go to step 3.
                            break (entry_guard, manifest.clone());
                        }
                        // The entry is not in cache, from there we release the cache
                        // lock and jump to step 2.
                        entry_guard
                    }

                    ChildManifestLockTakeOutcome::NeedWait(listener) => {
                        maybe_need_wait = Some(listener);
                        continue;
                    }
                }
            };

            // 2) Fetch from local storage or server

            let outcome = self
                .fetch_child_manifest_and_populate_cache(entry_id, FetchMode::LocalAndRemote)
                .await
                .map_err(|err| match err {
                    FetchChildManifestAndPopulateCacheError::Offline => {
                        ForUpdateFolderError::Offline
                    }
                    FetchChildManifestAndPopulateCacheError::Stopped => {
                        ForUpdateFolderError::Stopped
                    }
                    FetchChildManifestAndPopulateCacheError::EntryNotFound => {
                        ForUpdateFolderError::EntryNotFound
                    }
                    FetchChildManifestAndPopulateCacheError::NoRealmAccess => {
                        ForUpdateFolderError::NoRealmAccess
                    }
                    FetchChildManifestAndPopulateCacheError::InvalidKeysBundle(err) => {
                        ForUpdateFolderError::InvalidKeysBundle(err)
                    }
                    FetchChildManifestAndPopulateCacheError::InvalidCertificate(err) => {
                        ForUpdateFolderError::InvalidCertificate(err)
                    }
                    FetchChildManifestAndPopulateCacheError::InvalidManifest(err) => {
                        ForUpdateFolderError::InvalidManifest(err)
                    }
                    FetchChildManifestAndPopulateCacheError::Internal(err) => err.into(),
                });

            match outcome {
                Ok(manifest) => break (entry_guard, manifest),
                Err(err) => {
                    release_guard_on_error!(entry_guard);
                    return Err(err);
                }
            }
        };

        // 3) We have lock the entry and got the corresponding manifest !

        let manifest = match manifest {
            ArcLocalChildManifest::Folder(manifest) => manifest,
            ArcLocalChildManifest::File(_) => {
                release_guard_on_error!(entry_guard);
                return Err(ForUpdateFolderError::EntryNotAFolder);
            }
        };

        let updater = FolderUpdater {
            store: self,
            _update_guard: Some(entry_guard),
        };

        Ok((updater, manifest))
    }

    pub(crate) async fn for_update_file(
        &self,
        entry_id: VlobID,
        wait: bool,
    ) -> Result<(FileUpdater, Arc<LocalFileManifest>), ForUpdateFileError> {
        // Guard's drop will panic if the lock is not released
        macro_rules! release_guard_on_error {
            ($entry_guard:expr) => {
                let mut cache_guard = self.current_view_cache.lock().expect("Mutex is poisoned");
                cache_guard
                    .lock_update_child_manifests
                    .release($entry_guard);
            };
        }

        let mut maybe_need_wait = None;
        let (entry_guard, manifest) = loop {
            if let Some(listener) = maybe_need_wait {
                listener.await;
            }

            let entry_guard = {
                // 1) Cache lookup

                let mut cache_guard = self.current_view_cache.lock().expect("Mutex is poisoned");
                let outcome = cache_guard.lock_update_child_manifests.take(entry_id);

                match outcome {
                    ChildManifestLockTakeOutcome::Taken(entry_guard) => {
                        let found = cache_guard.child_manifests.get(&entry_id);
                        if let Some(manifest) = found {
                            // Cache hit ! We go to step 3.
                            break (entry_guard, manifest.clone());
                        }
                        // The entry is not in cache, from there we release the cache
                        // lock and jump to step 2.
                        entry_guard
                    }

                    ChildManifestLockTakeOutcome::NeedWait(listener) => {
                        if !wait {
                            return Err(ForUpdateFileError::WouldBlock);
                        }
                        maybe_need_wait = Some(listener);
                        continue;
                    }
                }
            };

            // 2) Fetch from local storage or server

            let outcome = self
                .fetch_child_manifest_and_populate_cache(entry_id, FetchMode::LocalAndRemote)
                .await
                .map_err(|err| match err {
                    FetchChildManifestAndPopulateCacheError::Offline => ForUpdateFileError::Offline,
                    FetchChildManifestAndPopulateCacheError::Stopped => ForUpdateFileError::Stopped,
                    FetchChildManifestAndPopulateCacheError::EntryNotFound => {
                        ForUpdateFileError::EntryNotFound
                    }
                    FetchChildManifestAndPopulateCacheError::NoRealmAccess => {
                        ForUpdateFileError::NoRealmAccess
                    }
                    FetchChildManifestAndPopulateCacheError::InvalidKeysBundle(err) => {
                        ForUpdateFileError::InvalidKeysBundle(err)
                    }
                    FetchChildManifestAndPopulateCacheError::InvalidCertificate(err) => {
                        ForUpdateFileError::InvalidCertificate(err)
                    }
                    FetchChildManifestAndPopulateCacheError::InvalidManifest(err) => {
                        ForUpdateFileError::InvalidManifest(err)
                    }
                    FetchChildManifestAndPopulateCacheError::Internal(err) => err.into(),
                });

            match outcome {
                Ok(manifest) => break (entry_guard, manifest),
                Err(err) => {
                    release_guard_on_error!(entry_guard);
                    return Err(err);
                }
            }
        };

        // 3) We have lock the entry and got the corresponding manifest !

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

    pub(crate) async fn is_child_entry_locked(&self, entry_id: VlobID) -> bool {
        let cache_guard = self.current_view_cache.lock().expect("Mutex is poisoned");
        cache_guard.lock_update_child_manifests.is_taken(entry_id)
    }

    pub(crate) async fn for_update_child_local_only(
        &self,
        entry_id: VlobID,
    ) -> Result<(ChildUpdater<'_>, Option<ArcLocalChildManifest>), ForUpdateChildLocalOnlyError>
    {
        // Guard's drop will panic if the lock is not released
        macro_rules! release_guard_on_error {
            ($entry_guard:expr) => {
                let mut cache_guard = self.current_view_cache.lock().expect("Mutex is poisoned");
                cache_guard
                    .lock_update_child_manifests
                    .release($entry_guard);
            };
        }

        let (entry_guard, manifest) = 'cache_guard_block: {
            let entry_guard = {
                // 1) Cache lookup

                let mut cache_guard = self.current_view_cache.lock().expect("Mutex is poisoned");
                let outcome = cache_guard.lock_update_child_manifests.try_take(entry_id);

                match outcome {
                    Some(entry_guard) => {
                        let found = cache_guard.child_manifests.get(&entry_id);
                        if let Some(manifest) = found {
                            // Cache hit ! We go to step 3.
                            break 'cache_guard_block (entry_guard, Some(manifest.clone()));
                        }
                        // The entry is not in cache, from there we release the cache
                        // lock and jump to step 2.
                        entry_guard
                    }

                    None => {
                        return Err(ForUpdateChildLocalOnlyError::WouldBlock);
                    }
                }
            };

            // 2) Fetch from local storage or server

            let outcome = self
                .fetch_child_manifest_and_populate_cache(entry_id, FetchMode::LocalOnly)
                .await
                .map(Some)
                .or_else(|err| match err {
                    FetchChildManifestAndPopulateCacheError::EntryNotFound => Ok(None),
                    FetchChildManifestAndPopulateCacheError::Stopped => {
                        Err(ForUpdateChildLocalOnlyError::Stopped)
                    }
                    FetchChildManifestAndPopulateCacheError::Internal(err) => Err(err.into()),
                    err @ (FetchChildManifestAndPopulateCacheError::Offline
                    | FetchChildManifestAndPopulateCacheError::NoRealmAccess
                    | FetchChildManifestAndPopulateCacheError::InvalidKeysBundle(_)
                    | FetchChildManifestAndPopulateCacheError::InvalidCertificate(_)
                    | FetchChildManifestAndPopulateCacheError::InvalidManifest(_)) => {
                        unreachable!("unused error for local-only operation: {}", err);
                    }
                });

            match outcome {
                Ok(manifest) => break 'cache_guard_block (entry_guard, manifest),
                Err(err) => {
                    release_guard_on_error!(entry_guard);
                    return Err(err);
                }
            }
        };

        // 3) We have lock the entry and got the corresponding manifest (if any) !

        let updater = ChildUpdater {
            store: self,
            _update_guard: Some(entry_guard),
        };

        Ok((updater, manifest))
    }

    pub(crate) async fn get_chunk_local_only(
        &self,
        chunk: &Chunk,
    ) -> Result<Bytes, ReadChunkLocalOnlyError> {
        {
            let cache = self.current_view_cache.lock().expect("Mutex is poisoned");
            let found = cache.chunks.get(&chunk.id);
            if let Some(data) = found {
                return Ok(data);
            }
        }

        // Cache miss ! Try to fetch from the local storage

        let mut maybe_storage = self.storage.lock().await;
        let storage = match &mut *maybe_storage {
            None => return Err(ReadChunkLocalOnlyError::Stopped),
            Some(storage) => storage,
        };

        let mut maybe_encrypted = storage.get_chunk(chunk.id).await?;

        if maybe_encrypted.is_none() {
            maybe_encrypted = storage
                .get_block(chunk.id.into(), self.device.now())
                .await?;
        }

        if let Some(encrypted) = maybe_encrypted {
            let data: Bytes = self
                .device
                .local_symkey
                .decrypt(&encrypted)
                .map_err(|err| anyhow::anyhow!("Cannot decrypt block from local storage: {}", err))?
                .into();

            // Don't forget to update the cache !

            let mut cache = self.current_view_cache.lock().expect("Mutex is poisoned");
            cache.chunks.push(chunk.id, data.clone());

            return Ok(data);
        }

        Err(ReadChunkLocalOnlyError::ChunkNotFound)
    }

    pub(crate) async fn get_chunk(&self, chunk: &Chunk) -> Result<Bytes, ReadChunkError> {
        let outcome = self.get_chunk_local_only(chunk).await;
        match outcome {
            Ok(chunk_data) => return Ok(chunk_data),
            Err(err) => match err {
                ReadChunkLocalOnlyError::ChunkNotFound => (),
                ReadChunkLocalOnlyError::Stopped => return Err(ReadChunkError::Stopped),
                ReadChunkLocalOnlyError::Internal(err) => return Err(err.into()),
            },
        }

        // Chunk not in local, time to ask the server

        let data: Bytes = match &chunk.access {
            None => return Err(ReadChunkError::ChunkNotFound),
            Some(access) => super::fetch::fetch_block(&self.cmds, access.id, &access.key)
                .await
                .map_err(|err| match err {
                    FetchRemoteBlockError::Offline => ReadChunkError::Offline,
                    FetchRemoteBlockError::BlockNotFound => ReadChunkError::ChunkNotFound,
                    FetchRemoteBlockError::NoRealmAccess => ReadChunkError::NoRealmAccess,
                    FetchRemoteBlockError::StoreUnavailable => ReadChunkError::StoreUnavailable,
                    FetchRemoteBlockError::BadDecryption => ReadChunkError::BadDecryption,
                    FetchRemoteBlockError::Internal(err) => {
                        err.context("cannot fetch block from server").into()
                    }
                })?,
        }
        .into();

        // Should both store the data in local storage...

        let mut maybe_storage = self.storage.lock().await;
        let storage = match &mut *maybe_storage {
            None => return Err(ReadChunkError::Stopped),
            Some(storage) => storage,
        };
        storage.set_chunk(chunk.id, &data).await?;

        // ...and update the cache !

        let mut cache = self.current_view_cache.lock().expect("Mutex is poisoned");
        cache.chunks.push(chunk.id, data.clone());

        Ok(data)
    }

    pub(crate) async fn get_inbound_need_sync_entries(
        &self,
        limit: u32,
    ) -> Result<Vec<VlobID>, GetNeedSyncEntriesError> {
        let mut storage_guard = self.storage.lock().await;
        let storage = storage_guard
            .as_mut()
            .ok_or_else(|| GetNeedSyncEntriesError::Stopped)?;
        storage
            .get_inbound_need_sync(limit)
            .await
            .map_err(GetNeedSyncEntriesError::Internal)
    }

    pub(crate) async fn get_outbound_need_sync_entries(
        &self,
        limit: u32,
    ) -> Result<Vec<VlobID>, GetNeedSyncEntriesError> {
        let mut storage_guard = self.storage.lock().await;
        let storage = storage_guard
            .as_mut()
            .ok_or_else(|| GetNeedSyncEntriesError::Stopped)?;
        storage
            .get_outbound_need_sync(limit)
            .await
            .map_err(GetNeedSyncEntriesError::Internal)
    }

    pub(crate) async fn get_realm_checkpoint(&self) -> Result<IndexInt, GetRealmCheckpointError> {
        let mut storage_guard = self.storage.lock().await;
        let storage = storage_guard
            .as_mut()
            .ok_or_else(|| GetRealmCheckpointError::Stopped)?;
        storage
            .get_realm_checkpoint()
            .await
            .map_err(GetRealmCheckpointError::Internal)
    }

    pub(crate) async fn update_realm_checkpoint(
        &self,
        current_checkpoint: IndexInt,
        changes: &[(VlobID, VersionInt)],
    ) -> Result<(), UpdateRealmCheckpointError> {
        let mut storage_guard = self.storage.lock().await;
        let storage = storage_guard
            .as_mut()
            .ok_or_else(|| UpdateRealmCheckpointError::Stopped)?;
        storage
            .update_realm_checkpoint(current_checkpoint, changes)
            .await
            .map_err(UpdateRealmCheckpointError::Internal)
    }
}

pub(super) struct RootUpdater<'a> {
    store: &'a WorkspaceStore,
    _update_guard: AsyncMutexGuard<'a, ()>,
}

impl<'a> RootUpdater<'a> {
    pub async fn update_workspace_manifest(
        self,
        manifest: Arc<LocalWorkspaceManifest>,
        new_child: Option<ArcLocalChildManifest>,
    ) -> Result<(), UpdateWorkspaceManifestError> {
        let mut storage_guard = self.store.storage.lock().await;
        let storage = storage_guard
            .as_mut()
            .ok_or_else(|| UpdateWorkspaceManifestError::Stopped)?;

        let update_data = UpdateManifestData {
            entry_id: manifest.base.id,
            base_version: manifest.base.version,
            need_sync: manifest.need_sync,
            encrypted: manifest.dump_and_encrypt(&self.store.device.local_symkey),
        };

        match new_child {
            None => {
                storage.update_manifest(&update_data).await?;

                // Finally update cache
                let mut cache = self
                    .store
                    .current_view_cache
                    .lock()
                    .expect("Mutex is poisoned");
                cache.workspace_manifest = manifest;
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
                let new_child_id = new_child_update_data.entry_id;
                storage
                    .update_manifests([update_data, new_child_update_data].into_iter())
                    .await?;

                // Finally update cache
                let mut cache = self
                    .store
                    .current_view_cache
                    .lock()
                    .expect("Mutex is poisoned");
                cache.workspace_manifest = manifest;
                cache.child_manifests.insert(new_child_id, new_child);
            }
        }

        Ok(())
    }
}

pub(super) struct FolderUpdater<'a> {
    store: &'a WorkspaceStore,
    _update_guard: Option<ChildManifestLockGuard>,
}

impl<'a> FolderUpdater<'a> {
    pub async fn update_folder_manifest(
        self,
        manifest: Arc<LocalFolderManifest>,
        new_child: Option<ArcLocalChildManifest>,
    ) -> Result<(), UpdateFolderManifestError> {
        let mut storage_guard = self.store.storage.lock().await;
        let storage = storage_guard
            .as_mut()
            .ok_or_else(|| UpdateFolderManifestError::Stopped)?;

        let update_data = UpdateManifestData {
            entry_id: manifest.base.id,
            base_version: manifest.base.version,
            need_sync: manifest.need_sync,
            encrypted: manifest.dump_and_encrypt(&self.store.device.local_symkey),
        };

        match new_child {
            None => {
                storage.update_manifest(&update_data).await?;

                // Finally update cache
                let mut cache = self
                    .store
                    .current_view_cache
                    .lock()
                    .expect("Mutex is poisoned");
                cache
                    .child_manifests
                    .insert(manifest.base.id, ArcLocalChildManifest::Folder(manifest));
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
                let new_child_id = new_child_update_data.entry_id;
                storage
                    .update_manifests([update_data, new_child_update_data].into_iter())
                    .await?;

                // Finally update cache
                let mut cache = self
                    .store
                    .current_view_cache
                    .lock()
                    .expect("Mutex is poisoned");
                cache
                    .child_manifests
                    .insert(manifest.base.id, ArcLocalChildManifest::Folder(manifest));
                cache.child_manifests.insert(new_child_id, new_child);
            }
        }

        Ok(())
    }
}

impl Drop for FolderUpdater<'_> {
    fn drop(&mut self) {
        if let Some(update_guard) = self._update_guard.take() {
            self.store
                .current_view_cache
                .lock()
                .expect("Mutex is poisoned")
                .lock_update_child_manifests
                .release(update_guard);
        }
    }
}

/// File updater doesn't keep hold on the store.
/// This is because, unlike folder and root updater, it is expected to have
/// a long lifetime (i.e. time the file is open).
pub(super) struct FileUpdater {
    _update_guard: ChildManifestLockGuard,
}

impl FileUpdater {
    pub async fn update_file_manifest_and_continue(
        &self,
        store: &WorkspaceStore,
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

    pub fn close(self, store: &WorkspaceStore) {
        store
            .current_view_cache
            .lock()
            .expect("Mutex is poisoned")
            .lock_update_child_manifests
            .release(self._update_guard);
    }
}

pub(super) struct ChildUpdater<'a> {
    store: &'a WorkspaceStore,
    _update_guard: Option<ChildManifestLockGuard>,
}

impl<'a> ChildUpdater<'a> {
    pub async fn update_manifest(
        self,
        manifest: ArcLocalChildManifest,
    ) -> Result<(), UpdateFolderManifestError> {
        let mut storage_guard = self.store.storage.lock().await;
        let storage = storage_guard
            .as_mut()
            .ok_or_else(|| UpdateFolderManifestError::Stopped)?;

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

        storage.update_manifest(&update_data).await?;

        // Finally update cache
        let mut cache = self
            .store
            .current_view_cache
            .lock()
            .expect("Mutex is poisoned");
        cache.child_manifests.insert(update_data.entry_id, manifest);

        Ok(())
    }
}

impl Drop for ChildUpdater<'_> {
    fn drop(&mut self) {
        if let Some(update_guard) = self._update_guard.take() {
            self.store
                .current_view_cache
                .lock()
                .expect("Mutex is poisoned")
                .lock_update_child_manifests
                .release(update_guard);
        }
    }
}
