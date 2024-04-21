// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, sync::Arc};

use libparsec_platform_storage::workspace::{PopulateManifestOutcome, UpdateManifestData};
use libparsec_types::prelude::*;

use crate::{
    certif::{InvalidCertificateError, InvalidKeysBundleError, InvalidManifestError},
    workspace::fetch::FetchRemoteManifestError,
};

use super::per_manifest_update_lock::PerManifestUpdateLock;

/// Very simple cache for chunks: we consider a chunk is most of the time much bigger
/// than a typical kernel read (512ko vs 4ko), so it's a big win to just keep the
/// chunks currently being read in memory.
/// To approximate that, we just keep the last 16 chunks read in memory.
#[derive(Debug)]
pub(super) struct ChunksCache {
    items: [Option<(ChunkID, Bytes)>; 16],
    round_robin: usize,
}

impl ChunksCache {
    pub fn new() -> Self {
        Self {
            items: Default::default(),
            round_robin: 0,
        }
    }
    pub fn push(&mut self, id: ChunkID, data: Bytes) {
        self.items[self.round_robin] = Some((id, data));
        self.round_robin = (self.round_robin + 1) % self.items.len();
    }
    pub fn get(&self, id: &ChunkID) -> Option<Bytes> {
        self.items
            .iter()
            .find_map(|item| item.as_ref().filter(|(chunk_id, _)| chunk_id == id))
            .map(|(_, data)| data.to_owned())
    }
}

#[derive(Debug)]
pub(super) struct CurrentViewCache {
    /// Workspace's root folder manifest is special in that we always have access to it
    /// (given we know it must be a folder manifest, we create an empty speculative folder
    /// manifest if we don't have it yet). For this reason, it is kept apart.
    pub root_manifest: Arc<LocalFolderManifest>,
    /// `child_manifests` contains a cache on the database:
    /// - the cache may be cleaned at any given time (i.e. inserting an entry in the cache
    ///   doesn't guarantee it will be available later on)
    /// - if the cache is present, it always correspond to the latest value (so the cache
    ///   should always be preferred over data coming from the database)
    /// - each cache entry can be "taken" for write access. In this mode
    pub child_manifests: HashMap<VlobID, ArcLocalChildManifest>,
    /// Each manifest has a dedicated async lock to prevent concurrent update (ensuring
    /// consistency between cache and database).
    pub lock_update_manifests: PerManifestUpdateLock,
    pub chunks: ChunksCache,
}

impl CurrentViewCache {
    pub fn new(root_manifest: Arc<LocalFolderManifest>) -> Self {
        Self {
            root_manifest,
            child_manifests: HashMap::new(),
            lock_update_manifests: PerManifestUpdateLock::new(),
            chunks: ChunksCache::new(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(super) enum PopulateCacheFromLocalStorageError {
    #[error("Component has stopped")]
    Stopped,
    #[error("Path doesn't exist")]
    EntryNotFound,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn populate_cache_from_local_storage(
    store: &super::WorkspaceStore,
    entry_id: VlobID,
) -> Result<ArcLocalChildManifest, PopulateCacheFromLocalStorageError> {
    // 1) Local storage lookup

    let maybe_found = {
        let mut maybe_storage = store.storage.lock().await;
        let storage = match &mut *maybe_storage {
            None => return Err(PopulateCacheFromLocalStorageError::Stopped),
            Some(storage) => storage,
        };

        storage.get_manifest(entry_id).await.map_err(|err| {
            PopulateCacheFromLocalStorageError::Internal(err.context("cannot access local storage"))
        })?
    };

    let manifest = match maybe_found {
        Some(encrypted) => {
            let outcome =
                LocalChildManifest::decrypt_and_load(&encrypted, &store.device.local_symkey);
            match outcome {
                Ok(LocalChildManifest::File(manifest)) => {
                    ArcLocalChildManifest::File(Arc::new(manifest))
                }
                Ok(LocalChildManifest::Folder(manifest)) => {
                    ArcLocalChildManifest::Folder(Arc::new(manifest))
                }
                Err(err) => {
                    return Err(PopulateCacheFromLocalStorageError::Internal(
                        anyhow::anyhow!("Local database contains invalid data: {}", err),
                    ))
                }
            }
        }
        None => {
            return Err(PopulateCacheFromLocalStorageError::EntryNotFound);
        }
    };

    // 2) We got our manifest, don't forget to update the cache before returning it

    let mut cache = store.current_view_cache.lock().expect("Mutex is poisoned");
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

#[derive(Debug, thiserror::Error)]
pub(super) enum PopulateCacheFromLocalStorageOrServerError {
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

pub(super) async fn populate_cache_from_local_storage_or_server(
    store: &super::WorkspaceStore,
    entry_id: VlobID,
) -> Result<ArcLocalChildManifest, PopulateCacheFromLocalStorageOrServerError> {
    // 1) Local storage lookup

    match populate_cache_from_local_storage(store, entry_id).await {
        Ok(manifest) => return Ok(manifest),
        Err(err) => match err {
            // Entry not in the local storage, go to stop 2
            PopulateCacheFromLocalStorageError::EntryNotFound => (),
            // Actual errors
            PopulateCacheFromLocalStorageError::Stopped => {
                return Err(PopulateCacheFromLocalStorageOrServerError::Stopped)
            }
            PopulateCacheFromLocalStorageError::Internal(err) => {
                return Err(err
                    .context("cannot populate cache from local storage")
                    .into())
            }
        },
    }

    // 2) Entry not in the local storage, last chance is to fetch from the server

    let outcome = super::super::fetch::fetch_remote_child_manifest(
        &store.cmds,
        &store.certificates_ops,
        store.realm_id,
        entry_id,
    )
    .await;
    let manifest = match outcome {
        Ok(ChildManifest::File(manifest)) => {
            ArcLocalChildManifest::File(Arc::new(LocalFileManifest::from_remote(manifest)))
        }
        Ok(ChildManifest::Folder(manifest)) => ArcLocalChildManifest::Folder(Arc::new(
            LocalFolderManifest::from_remote(manifest, None),
        )),
        Err(err) => {
            return Err(match err {
                FetchRemoteManifestError::Stopped => {
                    PopulateCacheFromLocalStorageOrServerError::Stopped
                }
                FetchRemoteManifestError::Offline => {
                    PopulateCacheFromLocalStorageOrServerError::Offline
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
                    PopulateCacheFromLocalStorageOrServerError::EntryNotFound
                }
                // The realm doesn't exist on server side, hence we are it creator and
                // it data only live on our local storage, which we have already checked.
                FetchRemoteManifestError::RealmNotFound => {
                    PopulateCacheFromLocalStorageOrServerError::EntryNotFound
                }
                FetchRemoteManifestError::NoRealmAccess => {
                    PopulateCacheFromLocalStorageOrServerError::NoRealmAccess
                }
                FetchRemoteManifestError::InvalidKeysBundle(err) => {
                    PopulateCacheFromLocalStorageOrServerError::InvalidKeysBundle(err)
                }
                FetchRemoteManifestError::InvalidCertificate(err) => {
                    PopulateCacheFromLocalStorageOrServerError::InvalidCertificate(err)
                }
                FetchRemoteManifestError::InvalidManifest(err) => {
                    PopulateCacheFromLocalStorageOrServerError::InvalidManifest(err)
                }
                FetchRemoteManifestError::Internal(err) => {
                    err.context("cannot fetch from server").into()
                }
            });
        }
    };

    // 3) We got our manifest, update local storage with it

    let manifest = loop {
        let mut maybe_storage = store.storage.lock().await;
        let storage = match &mut *maybe_storage {
            None => return Err(PopulateCacheFromLocalStorageOrServerError::Stopped),
            Some(storage) => storage,
        };

        let update_data = match &manifest {
            ArcLocalChildManifest::File(manifest) => UpdateManifestData {
                entry_id: manifest.base.id,
                base_version: manifest.base.version,
                need_sync: manifest.need_sync,
                encrypted: manifest.dump_and_encrypt(&store.device.local_symkey),
            },
            ArcLocalChildManifest::Folder(manifest) => UpdateManifestData {
                entry_id: manifest.base.id,
                base_version: manifest.base.version,
                need_sync: manifest.need_sync,
                encrypted: manifest.dump_and_encrypt(&store.device.local_symkey),
            },
        };
        let outcome = storage
            .populate_manifest(&update_data)
            .await
            .map_err(|err| {
                PopulateCacheFromLocalStorageOrServerError::Internal(
                    err.context("cannot populate local storage with manifest"),
                )
            })?;
        match outcome {
            PopulateManifestOutcome::Stored => break manifest,
            // A concurrent operation has populated the local storage !
            PopulateManifestOutcome::AlreadyPresent => {
                let manifest = match populate_cache_from_local_storage(store, entry_id).await {
                    Ok(manifest) => manifest,
                    Err(err) => match err {
                        // The entry which has been concurrently added has now been
                        // concurrently removed from local storage !
                        // This is highly unlikely, but we can just redo step 3 from
                        // the start.
                        PopulateCacheFromLocalStorageError::EntryNotFound => continue,
                        // Actual errors
                        PopulateCacheFromLocalStorageError::Stopped => {
                            return Err(PopulateCacheFromLocalStorageOrServerError::Stopped)
                        }
                        PopulateCacheFromLocalStorageError::Internal(err) => {
                            return Err(err
                                .context("cannot populate cache from local storage")
                                .into())
                        }
                    },
                };
                return Ok(manifest);
            }
        }
    };

    // 4) Also update the cache

    let mut cache = store.current_view_cache.lock().expect("Mutex is poisoned");
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
