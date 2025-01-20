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
use libparsec_platform_async::lock::Mutex as AsyncMutex;
use libparsec_platform_storage::workspace::{UpdateManifestData, WorkspaceStorage};
use libparsec_types::prelude::*;

use crate::{
    certif::CertificateOps, InvalidBlockAccessError, InvalidCertificateError,
    InvalidKeysBundleError, InvalidManifestError,
};

use cache::CurrentViewCache;
pub(crate) use file_updater::{FileUpdater, ForUpdateFileError};
pub(crate) use folder_updater::{FolderUpdater, ForUpdateFolderError};
pub(crate) use manifest_access::GetManifestError;
pub(crate) use reparent_updater::{ForUpdateReparentingError, ReparentingUpdater};
pub(crate) use resolve_path::{PathConfinementPoint, ResolvePathError};
pub(crate) use sync_updater::{
    ForUpdateSyncLocalOnlyError, IntoSyncConflictUpdaterError, SyncUpdater,
};

use super::fetch::FetchRemoteBlockError;

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
    #[error("Cannot reach the server")]
    Offline,
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
    #[error("Cannot reach the server")]
    Offline,
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

#[derive(Debug)]
pub(super) struct WorkspaceStore {
    realm_id: VlobID,
    device: Arc<LocalDevice>,
    cmds: Arc<AuthenticatedCmds>,
    certificates_ops: Arc<CertificateOps>,

    /// Note cache also contains the update locks.
    current_view_cache: Mutex<CurrentViewCache>,
    /// Given accessing `storage` requires exclusive access, it is better to have it
    /// under its own lock so that all cache hit operations can occur concurrently.
    storage: AsyncMutex<Option<WorkspaceStorage>>,
    prevent_sync_pattern: Regex,
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
        pattern: &Regex,
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
            pattern,
        )
        .await?;

        // 4) All set !

        Ok(Self {
            realm_id,
            device,
            cmds,
            certificates_ops,
            current_view_cache: Mutex::new(CurrentViewCache::new(Arc::new(root_manifest.into()))),
            storage: AsyncMutex::new(Some(storage)),
            prevent_sync_pattern: pattern.clone(),
        })
    }

    pub async fn stop(&self) -> anyhow::Result<()> {
        let maybe_storage = self.storage.lock().await.take();
        if let Some(storage) = maybe_storage {
            storage.stop().await?;
        }
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
    ) -> Result<(ArcLocalChildManifest, FsPath, PathConfinementPoint), ResolvePathError> {
        resolve_path::retrieve_path_from_id(self, entry_id).await
    }

    pub fn get_root_manifest(&self) -> Arc<LocalFolderManifest> {
        self.current_view_cache
            .lock()
            .expect("Mutex is poisoned")
            .manifests
            .root_manifest()
            .clone()
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
                    GetManifestError::Offline => Err(EnsureManifestExistsWithParentError::Offline),
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
        let cache_guard = self.current_view_cache.lock().expect("Mutex is poisoned");
        cache_guard.lock_update_manifests.is_taken(entry_id)
    }

    pub async fn get_chunk_or_block_local_only(
        &self,
        chunk_view: &ChunkView,
    ) -> Result<Bytes, ReadChunkOrBlockLocalOnlyError> {
        {
            let cache = self.current_view_cache.lock().expect("Mutex is poisoned");
            let found = cache.chunks.get(&chunk_view.id);
            if let Some(data) = found {
                return Ok(data);
            }
        }

        // Cache miss ! Try to fetch from the local storage

        let mut maybe_storage = self.storage.lock().await;
        let storage = match &mut *maybe_storage {
            None => return Err(ReadChunkOrBlockLocalOnlyError::Stopped),
            Some(storage) => storage,
        };

        let mut maybe_encrypted = storage.get_chunk(chunk_view.id).await?;

        if maybe_encrypted.is_none() {
            maybe_encrypted = storage
                .get_block(chunk_view.id.into(), self.device.now())
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
            cache.chunks.push(chunk_view.id, data.clone());

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

        let data = super::fetch::fetch_block(
            &self.cmds,
            &self.certificates_ops,
            self.realm_id,
            remote_manifest,
            access,
        )
        .await
        .map_err(|err| match err {
            FetchRemoteBlockError::Stopped => ReadChunkOrBlockError::Stopped,
            FetchRemoteBlockError::Offline | FetchRemoteBlockError::StoreUnavailable => {
                ReadChunkOrBlockError::Offline
            }
            FetchRemoteBlockError::BlockNotFound => ReadChunkOrBlockError::ChunkNotFound,
            FetchRemoteBlockError::NoRealmAccess => ReadChunkOrBlockError::NoRealmAccess,
            FetchRemoteBlockError::InvalidBlockAccess(err) => {
                ReadChunkOrBlockError::InvalidBlockAccess(err)
            }
            FetchRemoteBlockError::InvalidKeysBundle(err) => {
                ReadChunkOrBlockError::InvalidKeysBundle(err)
            }
            FetchRemoteBlockError::InvalidCertificate(err) => {
                ReadChunkOrBlockError::InvalidCertificate(err)
            }
            FetchRemoteBlockError::Internal(err) => {
                err.context("cannot fetch block from server").into()
            }
        })?;

        // Should both store the data in local storage...

        let mut maybe_storage = self.storage.lock().await;
        let storage = match &mut *maybe_storage {
            None => return Err(ReadChunkOrBlockError::Stopped),
            Some(storage) => storage,
        };
        let encrypted = self.device.local_symkey.encrypt(&data);
        storage
            .set_block(access.id, &encrypted, self.device.now())
            .await?;

        // ...and update the cache !

        let mut cache = self.current_view_cache.lock().expect("Mutex is poisoned");
        cache.chunks.push(chunk_view.id, data.clone());

        Ok(data)
    }

    pub async fn get_not_uploaded_chunk(
        &self,
        chunk_id: ChunkID,
    ) -> Result<Option<Bytes>, GetNotUploadedChunkError> {
        // Don't use the in-memory cache given it doesn't tell if the data is from and
        // uploaded block or not.
        let mut maybe_storage = self.storage.lock().await;
        let storage = match &mut *maybe_storage {
            None => return Err(GetNotUploadedChunkError::Stopped),
            Some(storage) => storage,
        };

        let maybe_encrypted = storage
            .get_chunk(chunk_id)
            .await
            .map_err(GetNotUploadedChunkError::Internal)?;

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
        let mut storage_guard = self.storage.lock().await;
        let storage = storage_guard
            .as_mut()
            .ok_or_else(|| PromoteLocalOnlyChunkToUploadedBlockError::Stopped)?;
        storage
            .promote_chunk_to_block(chunk_id, self.device.now())
            .await
            .map_err(PromoteLocalOnlyChunkToUploadedBlockError::Internal)
    }

    pub async fn get_inbound_need_sync_entries(
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

    pub async fn get_outbound_need_sync_entries(
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

    pub async fn get_realm_checkpoint(&self) -> Result<IndexInt, GetRealmCheckpointError> {
        let mut storage_guard = self.storage.lock().await;
        let storage = storage_guard
            .as_mut()
            .ok_or_else(|| GetRealmCheckpointError::Stopped)?;
        storage
            .get_realm_checkpoint()
            .await
            .map_err(GetRealmCheckpointError::Internal)
    }

    pub async fn update_realm_checkpoint(
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
