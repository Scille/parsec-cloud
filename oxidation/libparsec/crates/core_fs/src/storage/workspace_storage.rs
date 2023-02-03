// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    path::Path,
    sync::{Arc, Mutex, MutexGuard, RwLock},
};

use libparsec_client_types::{
    LocalDevice, LocalFileManifest, LocalManifest, LocalWorkspaceManifest,
};
use libparsec_types::{BlockID, ChunkID, DateTime, EntryID, FileDescriptor, Regex};
use local_db::LocalDatabase;
use platform_async::future;

use super::{
    chunk_storage::ChunkStorage,
    manifest_storage::{ChunkOrBlockID, ManifestStorage},
};
use crate::{
    error::{FSError, FSResult},
    storage::{
        chunk_storage::{BlockStorage, BlockStorageTrait, ChunkStorageTrait},
        version::{get_workspace_cache_storage_db_path, get_workspace_data_storage_db_path},
    },
};

pub const DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE: u64 = 512 * 1024 * 1024;

lazy_static::lazy_static! {
    pub static ref FAILSAFE_PATTERN_FILTER: Regex = {
        Regex::from_regex_str("^\\b$").expect("Must be a valid regex")
    };
}

#[derive(Default, Clone)]
struct Locker<T: Eq + Hash>(Arc<Mutex<HashMap<T, Arc<Mutex<()>>>>>);

enum Status {
    Locked,
    Released,
}

impl<T: Eq + Hash + Copy> Locker<T> {
    fn acquire(&self, id: T) -> Arc<Mutex<()>> {
        let mut map = self.0.lock().expect("Mutex is poisoned");
        map.entry(id)
            .or_insert_with(|| Arc::new(Mutex::new(())))
            .clone()
    }

    fn release(&self, id: &T, guard: MutexGuard<'_, ()>) {
        drop(guard);
        self.0.lock().expect("Mutex is poisoned").remove(id);
    }

    fn check(&self, id: &T) -> Status {
        if let Some(mutex) = self.0.lock().expect("Mutex is poisoned").get(id) {
            if mutex.try_lock().is_err() {
                return Status::Locked;
            }
        }

        Status::Released
    }
}

/// WorkspaceStorage is implemented with interior mutability because
/// we want some parallelism between its fields (e.g entry_locks)
pub struct WorkspaceStorage {
    pub device: LocalDevice,
    pub workspace_id: EntryID,
    open_fds: Mutex<HashMap<FileDescriptor, EntryID>>,
    fd_counter: Mutex<u32>,
    block_storage: BlockStorage,
    chunk_storage: ChunkStorage,
    manifest_storage: ManifestStorage,
    prevent_sync_pattern: Mutex<Regex>,
    prevent_sync_pattern_fully_applied: Mutex<bool>,
    entry_locks: Locker<EntryID>,
    workspace_storage_manifest_copy: RwLock<Option<LocalWorkspaceManifest>>,
}

impl WorkspaceStorage {
    pub async fn new(
        // Allow different type as Path, PathBuf, String, &str
        data_base_dir: impl AsRef<Path> + Send,
        device: LocalDevice,
        workspace_id: EntryID,
        prevent_sync_pattern: Regex,
        cache_size: u64,
    ) -> FSResult<Self> {
        let data_path =
            get_workspace_data_storage_db_path(data_base_dir.as_ref(), &device, workspace_id);
        let cache_path =
            get_workspace_cache_storage_db_path(data_base_dir.as_ref(), &device, workspace_id);

        let data_conn = LocalDatabase::from_path(
            data_path
                .to_str()
                .expect("Non-Utf-8 character found in data_path"),
        )
        .await?;
        let data_conn = Arc::new(data_conn);
        let cache_conn = LocalDatabase::from_path(
            cache_path
                .to_str()
                .expect("Non-Utf-8 character found in cache_path"),
        )
        .await?;
        let cache_conn = Arc::new(cache_conn);

        let block_storage = BlockStorage::new(
            device.local_symkey.clone(),
            cache_conn,
            cache_size,
            device.time_provider.clone(),
        )
        .await?;

        let manifest_storage =
            ManifestStorage::new(device.local_symkey.clone(), workspace_id, data_conn.clone())
                .await?;

        let chunk_storage = ChunkStorage::new(
            device.local_symkey.clone(),
            data_conn,
            device.time_provider.clone(),
        )
        .await?;

        // Instantiate workspace storage
        let instance = Self {
            device,
            workspace_id,
            // File descriptors
            open_fds: Mutex::new(HashMap::new()),
            fd_counter: Mutex::new(0),
            // Manifest and block storage
            block_storage,
            chunk_storage,
            manifest_storage,
            // Pattern attributes
            prevent_sync_pattern: Mutex::new(prevent_sync_pattern.clone()),
            prevent_sync_pattern_fully_applied: Mutex::new(false),
            // Locking structure
            entry_locks: Locker::default(),

            workspace_storage_manifest_copy: RwLock::new(None),
        };

        instance
            .set_prevent_sync_pattern(&prevent_sync_pattern)
            .await?;

        instance.block_storage.cleanup().await?;
        instance.block_storage.run_vacuum().await?;
        // Populate the cache with the workspace manifest to be able to
        // access it synchronously at all time
        instance.load_workspace_manifest().await?;

        Ok(instance)
    }

    pub fn lock_entry_id(&self, entry_id: EntryID) -> Arc<Mutex<()>> {
        self.entry_locks.acquire(entry_id)
    }

    pub fn release_entry_id(&self, entry_id: &EntryID, guard: MutexGuard<'_, ()>) {
        self.entry_locks.release(entry_id, guard)
    }

    /// Check if an `entry_id` is locked.
    ///
    /// # Errors
    ///
    /// Will return an error if the given `entry_id` is not locked.
    fn check_lock_status(&self, entry_id: &EntryID) -> FSResult<()> {
        if let Status::Released = self.entry_locks.check(entry_id) {
            return Err(FSError::Runtime(*entry_id));
        }
        Ok(())
    }

    pub fn get_prevent_sync_pattern(&self) -> libparsec_types::Regex {
        self.prevent_sync_pattern
            .lock()
            .expect("Mutex is poisoned")
            .clone()
    }

    pub fn get_prevent_sync_pattern_fully_applied(&self) -> bool {
        *self
            .prevent_sync_pattern_fully_applied
            .lock()
            .expect("Mutex is poisoned")
    }

    fn get_next_fd(&self) -> FileDescriptor {
        let mut fd_counter = self.fd_counter.lock().expect("Mutex is poisoned");
        *fd_counter += 1;
        FileDescriptor(*fd_counter)
    }

    // File management interface

    pub fn create_file_descriptor(&self, manifest: LocalFileManifest) -> FileDescriptor {
        let fd = self.get_next_fd();
        self.open_fds
            .lock()
            .expect("Mutex is poisoned")
            .insert(fd, manifest.base.id);
        fd
    }

    pub fn load_file_descriptor_in_cache(&self, fd: FileDescriptor) -> FSResult<LocalFileManifest> {
        let entry_id = self
            .open_fds
            .lock()
            .expect("Mutex is poisoned")
            .get(&fd)
            .cloned()
            .ok_or(FSError::InvalidFileDescriptor(fd))?;

        match self.get_manifest_in_cache(&entry_id) {
            Some(LocalManifest::File(manifest)) => Ok(manifest),
            _ => Err(FSError::LocalMiss(*entry_id)),
        }
    }

    pub async fn load_file_descriptor(&self, fd: FileDescriptor) -> FSResult<LocalFileManifest> {
        let entry_id = self
            .open_fds
            .lock()
            .expect("Mutex is poisoned")
            .get(&fd)
            .cloned()
            .ok_or(FSError::InvalidFileDescriptor(fd))?;

        match self.get_manifest(entry_id).await {
            Ok(LocalManifest::File(manifest)) => Ok(manifest),
            _ => Err(FSError::LocalMiss(*entry_id)),
        }
    }

    pub fn remove_file_descriptor(&self, fd: FileDescriptor) -> Option<EntryID> {
        self.open_fds.lock().expect("Mutex is poisoned").remove(&fd)
    }

    // Block interface

    pub async fn set_clean_block(&self, block_id: BlockID, block: &[u8]) -> FSResult<()> {
        self.block_storage
            .set_chunk_upgraded(ChunkID::from(*block_id), block)
            .await
    }

    pub async fn clear_clean_block(&self, block_id: BlockID) {
        let _: FSResult<()> = self
            .block_storage
            .clear_chunk(ChunkID::from(*block_id))
            .await;
    }

    pub async fn get_dirty_block(&self, block_id: BlockID) -> FSResult<Vec<u8>> {
        self.chunk_storage.get_chunk(ChunkID::from(*block_id)).await
    }

    // Chunk interface

    pub async fn get_chunk(&self, chunk_id: ChunkID) -> FSResult<Vec<u8>> {
        if let Ok(raw) = self.chunk_storage.get_chunk(chunk_id).await {
            Ok(raw)
        } else if let Ok(raw) = self.block_storage.get_chunk(chunk_id).await {
            Ok(raw)
        } else {
            Err(FSError::LocalMiss(*chunk_id))
        }
    }

    pub async fn set_chunk(&self, chunk_id: ChunkID, block: &[u8]) -> FSResult<()> {
        self.chunk_storage.set_chunk(chunk_id, block).await
    }

    pub async fn clear_chunk(&self, chunk_id: ChunkID, miss_ok: bool) -> FSResult<()> {
        let res = self.chunk_storage.clear_chunk(chunk_id).await;
        if !miss_ok {
            return res;
        }
        Ok(())
    }

    // Helpers

    pub async fn clear_memory_cache(&self, flush: bool) -> FSResult<()> {
        self.manifest_storage.clear_memory_cache(flush).await
    }

    // Checkpoint interface

    pub async fn get_realm_checkpoint(&self) -> i64 {
        self.manifest_storage.get_realm_checkpoint().await
    }

    pub async fn update_realm_checkpoint(
        &self,
        new_checkpoint: i64,
        changed_vlobs: &[(EntryID, i64)],
    ) -> FSResult<()> {
        self.manifest_storage
            .update_realm_checkpoint(new_checkpoint, changed_vlobs)
            .await
    }

    pub async fn get_need_sync_entries(&self) -> FSResult<(HashSet<EntryID>, HashSet<EntryID>)> {
        self.manifest_storage.get_need_sync_entries().await
    }

    // Manifest interface

    async fn load_workspace_manifest(&self) -> FSResult<()> {
        match self.manifest_storage.get_manifest(self.workspace_id).await {
            Ok(LocalManifest::Workspace(manifest)) => {
                self.workspace_storage_manifest_copy
                    .write()
                    .expect("RwLock is poisoned")
                    .replace(manifest);
                Ok(())
            }
            Ok(_) => panic!(
                "Workspace manifest id is used for something other than a workspace manifest"
            ),
            Err(_) => {
                let timestamp = self.device.now();
                let manifest = LocalWorkspaceManifest::new(
                    self.device.device_id.clone(),
                    timestamp,
                    Some(self.workspace_id),
                    true,
                );
                self.set_manifest(
                    self.workspace_id,
                    LocalManifest::Workspace(manifest),
                    false,
                    false,
                    None,
                )
                .await
            }
        }
    }

    pub fn get_workspace_manifest(&self) -> FSResult<LocalWorkspaceManifest> {
        self.workspace_storage_manifest_copy
            .read()
            .expect("RwLock is poisoned")
            .as_ref()
            .cloned()
            .ok_or(FSError::LocalMiss(*self.workspace_id))
    }

    pub async fn get_manifest(&self, entry_id: EntryID) -> FSResult<LocalManifest> {
        self.manifest_storage.get_manifest(entry_id).await
    }

    pub fn set_manifest_in_cache(
        &self,
        entry_id: EntryID,
        manifest: LocalManifest,
        check_lock_status: bool,
        removed_ids: Option<HashSet<ChunkOrBlockID>>,
    ) -> FSResult<()> {
        if check_lock_status {
            self.check_lock_status(&entry_id)?;
        }
        self.manifest_storage
            .set_manifest_cache_only(entry_id, manifest.clone(), removed_ids);
        if self.workspace_id == entry_id {
            let manifest = if let LocalManifest::Workspace(manifest) = manifest {
                manifest
            } else {
                panic!("We updated the workspace manifest with a manifest of the wrong type");
            };
            self.workspace_storage_manifest_copy
                .write()
                .expect("RwLock is poisoned")
                .replace(manifest);
        }
        Ok(())
    }

    pub async fn set_manifest(
        &self,
        entry_id: EntryID,
        manifest: LocalManifest,
        cache_only: bool,
        check_lock_status: bool,
        removed_ids: Option<HashSet<ChunkOrBlockID>>,
    ) -> FSResult<()> {
        if check_lock_status {
            self.check_lock_status(&entry_id)?;
        }
        self.manifest_storage
            .set_manifest(entry_id, manifest.clone(), cache_only, removed_ids)
            .await?;
        if self.workspace_id == entry_id {
            let manifest = if let LocalManifest::Workspace(manifest) = manifest {
                manifest
            } else {
                panic!("We updated the workspace manifest with a manifest of the wrong type");
            };
            self.workspace_storage_manifest_copy
                .write()
                .expect("RwLock is poisoned")
                .replace(manifest);
        }
        Ok(())
    }

    pub async fn ensure_manifest_persistent(
        &self,
        entry_id: EntryID,
        check_lock_status: bool,
    ) -> FSResult<()> {
        if check_lock_status {
            self.check_lock_status(&entry_id)?;
        }
        self.manifest_storage
            .ensure_manifest_persistent(entry_id)
            .await
    }

    pub async fn clear_manifest(
        &self,
        entry_id: &EntryID,
        check_lock_status: bool,
    ) -> FSResult<()> {
        if check_lock_status {
            self.check_lock_status(entry_id)?;
        }
        self.manifest_storage.clear_manifest(*entry_id).await
    }

    /// Close the connections to the databases.
    /// Provide a way to manually close those connections.
    /// Event tho they will be closes when [WorkspaceStorage] is dropped.
    pub async fn close_connections(&self) -> FSResult<()> {
        let (r1, r2, r3) = future::join3(
            self.manifest_storage.close_connection(),
            self.chunk_storage.close_connection(),
            self.block_storage.close_connection(),
        )
        .await;

        r1.and(r2).and(r3)
    }

    // Prevent sync pattern interface

    async fn load_prevent_sync_pattern(&self, re: &Regex, fully_applied: bool) {
        *self.prevent_sync_pattern.lock().expect("Mutex is poisoned") = re.clone();
        *self
            .prevent_sync_pattern_fully_applied
            .lock()
            .expect("Mutex is poisoned") = fully_applied;
    }

    pub async fn set_prevent_sync_pattern(&self, pattern: &Regex) -> FSResult<()> {
        let fully_applied = self
            .manifest_storage
            .set_prevent_sync_pattern(pattern)
            .await?;
        self.load_prevent_sync_pattern(pattern, fully_applied).await;
        Ok(())
    }

    pub async fn mark_prevent_sync_pattern_fully_applied(&self, pattern: &Regex) -> FSResult<()> {
        let fully_applied = self
            .manifest_storage
            .mark_prevent_sync_pattern_fully_applied(pattern)
            .await?;
        self.load_prevent_sync_pattern(pattern, fully_applied).await;
        Ok(())
    }

    pub async fn get_local_block_ids(&self, chunk_ids: &[ChunkID]) -> FSResult<Vec<ChunkID>> {
        self.block_storage.get_local_chunk_ids(chunk_ids).await
    }

    pub async fn get_local_chunk_ids(&self, chunk_ids: &[ChunkID]) -> FSResult<Vec<ChunkID>> {
        self.chunk_storage.get_local_chunk_ids(chunk_ids).await
    }

    pub async fn run_vacuum(&self) -> FSResult<()> {
        // TODO: Add some condition
        self.chunk_storage.run_vacuum().await
    }

    /// Return `true` when the given manifest identified by `entry_id` is cached.
    pub async fn is_manifest_cache_ahead_of_persistance(&self, entry_id: &EntryID) -> bool {
        self.manifest_storage
            .is_manifest_cache_ahead_of_persistance(entry_id)
            .await
    }

    pub fn get_manifest_in_cache(&self, entry_id: &EntryID) -> Option<LocalManifest> {
        self.manifest_storage.get_manifest_in_cache(entry_id)
    }

    /// Take a snapshot of the current [WorkspaceStorage]
    pub fn to_timestamp(this: Arc<Self>) -> WorkspaceStorageSnapshot {
        this.into()
    }
}

/// Snapshot of a [WorkspaceStorage] at a given time.
#[derive(Clone)]
pub struct WorkspaceStorageSnapshot {
    cache: Arc<RwLock<HashMap<EntryID, LocalManifest>>>,
    open_fds: Arc<Mutex<HashMap<FileDescriptor, EntryID>>>,
    fd_counter: Arc<Mutex<u32>>,
    pub workspace_storage: Arc<WorkspaceStorage>,
}

impl From<Arc<WorkspaceStorage>> for WorkspaceStorageSnapshot {
    fn from(workspace_storage: Arc<WorkspaceStorage>) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::default())),
            workspace_storage,
            fd_counter: Arc::new(Mutex::new(0)),
            open_fds: Arc::new(Mutex::new(HashMap::default())),
        }
    }
}

// File management interface.

impl WorkspaceStorageSnapshot {
    fn get_next_fd(&self) -> FileDescriptor {
        let mut counter = self.fd_counter.lock().expect("Mutex is poisoned");
        *counter += 1;
        FileDescriptor(*counter)
    }

    pub fn create_file_descriptor(&self, manifest: LocalFileManifest) -> FileDescriptor {
        let fd = self.get_next_fd();
        self.open_fds
            .lock()
            .expect("Mutex is poisoned")
            .insert(fd, manifest.base.id);
        fd
    }

    pub fn load_file_descriptor(&self, fd: FileDescriptor) -> FSResult<LocalFileManifest> {
        let entry_id = self
            .open_fds
            .lock()
            .expect("Mutex is poisoned")
            .get(&fd)
            .copied()
            .ok_or(FSError::InvalidFileDescriptor(fd))?;

        match self.get_manifest(entry_id) {
            Ok(LocalManifest::File(manifest)) => Ok(manifest),
            _ => Err(FSError::LocalMiss(*entry_id)),
        }
    }

    pub fn remove_file_descriptor(&self, fd: FileDescriptor) -> Option<EntryID> {
        self.open_fds.lock().expect("Mutex is poisoned").remove(&fd)
    }
}

impl WorkspaceStorageSnapshot {
    /// Return a chunk identified by `chunk_id`.
    /// Will look for the chunk in the [ChunkStorage] & [BlockStorage].
    pub async fn get_chunk(&self, chunk_id: ChunkID) -> FSResult<Vec<u8>> {
        self.workspace_storage.get_chunk(chunk_id).await
    }

    /// Return the [LocalWorkspaceManifest] of the current [WorkspaceStorageTimestamped].
    pub fn get_workspace_manifest(&self) -> FSResult<LocalWorkspaceManifest> {
        self.workspace_storage.get_workspace_manifest()
    }

    pub fn get_manifest(&self, entry_id: EntryID) -> FSResult<LocalManifest> {
        self.cache
            .read()
            .expect("RwLock is poisoned")
            .get(&entry_id)
            .cloned()
            .ok_or(FSError::LocalMiss(*entry_id))
    }

    pub fn set_manifest(
        &self,
        entry_id: EntryID,
        manifest: LocalManifest,
        check_lock_status: bool,
    ) -> FSResult<()> {
        if manifest.need_sync() {
            Err(FSError::WorkspaceStorageTimestamped)
        } else {
            if check_lock_status {
                self.check_lock_status(&entry_id)?;
            }
            self.cache
                .write()
                .expect("RwLock is poisoned")
                .insert(entry_id, manifest);
            Ok(())
        }
    }

    /// Check if an `entry_id` is locked.
    ///
    /// # Errors
    ///
    /// Will return an error if the given `entry_id` is not locked.
    pub fn check_lock_status(&self, entry_id: &EntryID) -> FSResult<()> {
        self.workspace_storage.check_lock_status(entry_id)
    }

    pub fn get_prevent_sync_pattern(&self) -> libparsec_types::Regex {
        self.workspace_storage.get_prevent_sync_pattern()
    }

    pub fn get_prevent_sync_pattern_fully_applied(&self) -> bool {
        self.workspace_storage
            .get_prevent_sync_pattern_fully_applied()
    }

    // Block interface.

    pub async fn set_clean_block(&self, block_id: BlockID, block: &[u8]) -> FSResult<()> {
        self.workspace_storage
            .set_clean_block(block_id, block)
            .await
    }

    pub async fn clear_clean_block(&self, block_id: BlockID) {
        self.workspace_storage.clear_clean_block(block_id).await
    }

    pub async fn get_dirty_block(&self, block_id: BlockID) -> FSResult<Vec<u8>> {
        self.workspace_storage.get_dirty_block(block_id).await
    }

    pub fn lock_entry_id(&self, entry_id: EntryID) -> Arc<Mutex<()>> {
        self.workspace_storage.lock_entry_id(entry_id)
    }

    pub async fn release_entry_id(&self, entry_id: &EntryID, guard: MutexGuard<'_, ()>) {
        self.workspace_storage.release_entry_id(entry_id, guard)
    }

    /// Clear the local manifest cache
    pub fn clear_local_cache(&self) {
        self.cache.write().expect("RwLock is poisoned").clear()
    }

    /// Take a snapshot of the current `workspace storage`
    pub fn to_timestamp(&self) -> Self {
        Self::from(self.workspace_storage.clone())
    }
}

pub async fn workspace_storage_non_speculative_init(
    data_base_dir: &Path,
    device: LocalDevice,
    workspace_id: EntryID,
    timestamp: Option<DateTime>,
) -> FSResult<()> {
    let data_path = get_workspace_data_storage_db_path(data_base_dir, &device, workspace_id);
    let conn = LocalDatabase::from_path(
        data_path
            .to_str()
            .expect("Non-Utf-8 character found in data_path"),
    )
    .await?;
    let conn = Arc::new(conn);
    let manifest_storage =
        ManifestStorage::new(device.local_symkey.clone(), workspace_id, conn).await?;
    let timestamp = timestamp.unwrap_or_else(|| device.now());
    let manifest =
        LocalWorkspaceManifest::new(device.device_id, timestamp, Some(workspace_id), false);

    manifest_storage
        .set_manifest(
            workspace_id,
            LocalManifest::Workspace(manifest),
            false,
            None,
        )
        .await?;
    manifest_storage.clear_memory_cache(true).await?;
    manifest_storage.close_connection().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::await_holding_lock)]

    use crate::conftest::{alice_workspace_storage, TmpWorkspaceStorage};
    use libparsec_client_types::Chunk;
    use libparsec_types::{Blocksize, Regex, DEFAULT_BLOCK_SIZE};
    use rstest::rstest;
    use std::num::NonZeroU64;
    use tests_fixtures::{alice, tmp_path, Device, TmpPath};

    use super::*;

    fn create_workspace_manifest(device: &LocalDevice) -> LocalWorkspaceManifest {
        let author = device.device_id.clone();
        let timestamp = device.now();
        LocalWorkspaceManifest::new(author, timestamp, None, false)
    }

    fn create_file_manifest(device: &LocalDevice) -> LocalFileManifest {
        let author = device.device_id.clone();
        let timestamp = device.now();
        LocalFileManifest::new(author, EntryID::default(), timestamp, DEFAULT_BLOCK_SIZE)
    }

    async fn clear_cache(storage: &WorkspaceStorage) {
        storage
            .manifest_storage
            .clear_memory_cache(false)
            .await
            .expect("Failed to flush cache");
    }

    #[rstest]
    #[tokio::test]
    async fn test_lock_required(#[future] alice_workspace_storage: TmpWorkspaceStorage) {
        let aws = alice_workspace_storage.await;
        let manifest = create_workspace_manifest(&aws.device);
        let manifest = LocalManifest::Workspace(manifest);
        let manifest_id = manifest.id();

        assert_eq!(
            aws.set_manifest(manifest_id, manifest, false, true, None)
                .await
                .unwrap_err(),
            FSError::Runtime(manifest_id)
        );

        assert_eq!(
            aws.ensure_manifest_persistent(manifest_id, true)
                .await
                .unwrap_err(),
            FSError::Runtime(manifest_id)
        );

        assert_eq!(
            aws.clear_manifest(&manifest_id, true).await.unwrap_err(),
            FSError::Runtime(manifest_id)
        );
    }

    #[rstest]
    #[tokio::test]
    async fn test_basic_set_get_clear(#[future] alice_workspace_storage: TmpWorkspaceStorage) {
        let aws = alice_workspace_storage.await;
        let manifest = create_workspace_manifest(&aws.device);
        let manifest = LocalManifest::Workspace(manifest);
        let manifest_id = manifest.id();

        let mutex = aws.lock_entry_id(manifest_id);
        let guard = mutex.lock().unwrap();

        // 1) No data
        assert_eq!(
            aws.get_manifest(manifest_id).await.unwrap_err(),
            FSError::LocalMiss(*manifest_id)
        );

        // 2) Set data
        aws.set_manifest(manifest_id, manifest.clone(), false, true, None)
            .await
            .unwrap();
        assert_eq!(aws.get_manifest(manifest_id).await.unwrap(), manifest);

        // Make sure data are not only stored in cache
        clear_cache(&aws).await;
        assert_eq!(aws.get_manifest(manifest_id).await.unwrap(), manifest);

        // 3) Clear data
        aws.clear_manifest(&manifest_id, true).await.unwrap();

        assert_eq!(
            aws.get_manifest(manifest_id).await.unwrap_err(),
            FSError::LocalMiss(*manifest_id)
        );

        assert_eq!(
            aws.clear_manifest(&manifest_id, true).await.unwrap_err(),
            FSError::LocalMiss(*manifest_id)
        );

        aws.release_entry_id(&manifest_id, guard);
    }

    #[rstest]
    #[tokio::test]
    async fn test_cache_set_get(#[future] alice_workspace_storage: TmpWorkspaceStorage) {
        let aws = alice_workspace_storage.await;
        let manifest = create_workspace_manifest(&aws.device);
        let manifest = LocalManifest::Workspace(manifest);
        let manifest_id = manifest.id();

        let mutex = aws.lock_entry_id(manifest_id);
        let guard = mutex.lock().unwrap();

        // 1) Set data
        aws.set_manifest(manifest_id, manifest.clone(), true, true, None)
            .await
            .unwrap();
        assert_eq!(aws.get_manifest(manifest_id).await.unwrap(), manifest);

        // Data should be set only in the cache
        clear_cache(&aws).await;
        assert_eq!(
            aws.get_manifest(manifest_id).await.unwrap_err(),
            FSError::LocalMiss(*manifest_id)
        );

        // Re-set data
        aws.set_manifest(manifest_id, manifest.clone(), true, true, None)
            .await
            .unwrap();

        // 2) Clear should work as expected
        aws.clear_manifest(&manifest_id, true).await.unwrap();
        assert_eq!(
            aws.get_manifest(manifest_id).await.unwrap_err(),
            FSError::LocalMiss(*manifest_id)
        );

        // Re-set data
        aws.set_manifest(manifest_id, manifest.clone(), true, true, None)
            .await
            .unwrap();

        // 3) Flush data
        aws.ensure_manifest_persistent(manifest_id, true)
            .await
            .unwrap();
        assert_eq!(aws.get_manifest(manifest_id).await.unwrap(), manifest);

        // Data should be persistent in real database
        clear_cache(&aws).await;
        assert_eq!(aws.get_manifest(manifest_id).await.unwrap(), manifest);

        // 4) Idempotency
        aws.ensure_manifest_persistent(manifest_id, true)
            .await
            .unwrap();

        aws.release_entry_id(&manifest_id, guard);
    }

    #[rstest]
    #[case(false, false)]
    #[case(false, true)]
    #[case(true, false)]
    #[case(true, true)]
    #[tokio::test]
    async fn test_chunk_clearing(
        #[future] alice_workspace_storage: TmpWorkspaceStorage,
        #[case] cache_only: bool,
        #[case] clear_manifest: bool,
    ) {
        let aws = alice_workspace_storage.await;
        let mut _manifest = create_file_manifest(&aws.device);
        let data1 = b"abc";
        let chunk1 = Chunk::new(0, NonZeroU64::new(3).unwrap());
        let data2 = b"def";
        let chunk2 = Chunk::new(3, NonZeroU64::new(6).unwrap());
        _manifest.blocks = vec![vec![chunk1.clone(), chunk2.clone()]];
        _manifest.size = 6;
        let manifest = LocalManifest::File(_manifest.clone());
        let manifest_id = manifest.id();

        let mutex = aws.lock_entry_id(manifest_id);
        let guard = mutex.lock().unwrap();

        // Set chunks and manifest
        aws.set_chunk(chunk1.id, data1).await.unwrap();
        aws.set_chunk(chunk2.id, data2).await.unwrap();
        aws.set_manifest(manifest_id, manifest, false, true, None)
            .await
            .unwrap();

        // Set a new version of the manifest without the chunks
        let removed_ids = HashSet::from([
            ChunkOrBlockID::ChunkID(chunk1.id),
            ChunkOrBlockID::ChunkID(chunk2.id),
        ]);
        _manifest.blocks.clear();
        let new_manifest = LocalManifest::File(_manifest.clone());

        aws.set_manifest(
            manifest_id,
            new_manifest,
            cache_only,
            true,
            Some(removed_ids),
        )
        .await
        .unwrap();

        if cache_only {
            // The chunks are still accessible
            assert_eq!(aws.get_chunk(chunk1.id).await.unwrap(), b"abc");
            assert_eq!(aws.get_chunk(chunk2.id).await.unwrap(), b"def");
        } else {
            // The chunks are gone
            assert_eq!(
                aws.get_chunk(chunk1.id).await.unwrap_err(),
                FSError::LocalMiss(*chunk1.id)
            );
            assert_eq!(
                aws.get_chunk(chunk2.id).await.unwrap_err(),
                FSError::LocalMiss(*chunk2.id)
            );
        }

        // Now flush the manifest
        if clear_manifest {
            aws.clear_manifest(&manifest_id, true).await.unwrap();
        } else {
            aws.ensure_manifest_persistent(manifest_id, true)
                .await
                .unwrap();
        }

        // The chunks are gone
        assert_eq!(
            aws.get_chunk(chunk1.id).await.unwrap_err(),
            FSError::LocalMiss(*chunk1.id)
        );
        assert_eq!(
            aws.get_chunk(chunk2.id).await.unwrap_err(),
            FSError::LocalMiss(*chunk2.id)
        );

        // Idempotency
        aws.ensure_manifest_persistent(manifest_id, true)
            .await
            .unwrap();

        aws.release_entry_id(&manifest_id, guard);
    }

    #[rstest]
    #[tokio::test]
    async fn test_cache_flushed_on_exit(alice: &Device, tmp_path: TmpPath) {
        let db_path = tmp_path.join("workspace_storage.sqlite");
        let manifest = create_workspace_manifest(&alice.local_device());
        let manifest = LocalManifest::Workspace(manifest);
        let manifest_id = manifest.id();
        let workspace_id = EntryID::default();

        let aws = WorkspaceStorage::new(
            Path::new(&db_path),
            alice.local_device(),
            workspace_id,
            FAILSAFE_PATTERN_FILTER.clone(),
            DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE,
        )
        .await
        .unwrap();

        let mutex = aws.lock_entry_id(manifest_id);
        let guard = mutex.lock().unwrap();

        aws.set_manifest(manifest_id, manifest.clone(), true, true, None)
            .await
            .unwrap();

        aws.release_entry_id(&manifest_id, guard);
        aws.clear_memory_cache(true).await.unwrap();
        aws.close_connections().await.unwrap();

        let aws = WorkspaceStorage::new(
            Path::new(&db_path),
            alice.local_device(),
            workspace_id,
            FAILSAFE_PATTERN_FILTER.clone(),
            DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE,
        )
        .await
        .unwrap();

        assert_eq!(aws.get_manifest(manifest_id).await.unwrap(), manifest);
    }

    #[rstest]
    #[tokio::test]
    async fn test_clear_cache(#[future] alice_workspace_storage: TmpWorkspaceStorage) {
        let aws = alice_workspace_storage.await;
        let manifest1 = create_workspace_manifest(&aws.device);
        let manifest1 = LocalManifest::Workspace(manifest1);
        let manifest2 = create_workspace_manifest(&aws.device);
        let manifest2 = LocalManifest::Workspace(manifest2);
        let manifest1_id = manifest1.id();
        let manifest2_id = manifest2.id();

        let mutex1 = aws.lock_entry_id(manifest1_id);
        let mutex2 = aws.lock_entry_id(manifest2_id);
        let guard1 = mutex1.lock().unwrap();
        let guard2 = mutex2.lock().unwrap();

        // Set manifest 1 and manifest 2, cache only
        aws.set_manifest(manifest1_id, manifest1.clone(), false, true, None)
            .await
            .unwrap();
        aws.set_manifest(manifest2_id, manifest2.clone(), true, true, None)
            .await
            .unwrap();

        // Clear without flushing
        aws.clear_memory_cache(false).await.unwrap();

        // Manifest 1 is present but manifest2 got lost
        assert_eq!(aws.get_manifest(manifest1_id).await.unwrap(), manifest1);
        assert_eq!(
            aws.get_manifest(manifest2_id).await.unwrap_err(),
            FSError::LocalMiss(*manifest2_id)
        );

        // Set Manifest 2, cache only
        aws.set_manifest(manifest2_id, manifest2.clone(), true, true, None)
            .await
            .unwrap();

        // Clear with flushing
        aws.clear_memory_cache(true).await.unwrap();
        assert_eq!(aws.get_manifest(manifest2_id).await.unwrap(), manifest2);

        aws.release_entry_id(&manifest1_id, guard1);
        aws.release_entry_id(&manifest2_id, guard2);
    }

    #[rstest]
    #[tokio::test]
    async fn test_serialize_non_empty_local_file_manifest(
        #[future] alice_workspace_storage: TmpWorkspaceStorage,
    ) {
        let aws = alice_workspace_storage.await;
        let mut manifest = create_file_manifest(&aws.device);
        let chunk1 = Chunk::new(0, NonZeroU64::try_from(7).unwrap())
            .evolve_as_block(b"0123456")
            .unwrap();
        let chunk2 = Chunk::new(7, NonZeroU64::try_from(8).unwrap());
        let chunk3 = Chunk::new(8, NonZeroU64::try_from(10).unwrap());
        let blocks = vec![vec![chunk1, chunk2], vec![chunk3]];
        manifest.size = 10;
        manifest.blocks = blocks;
        manifest.blocksize = Blocksize::try_from(8).unwrap();
        manifest.assert_integrity();
        let manifest = LocalManifest::File(manifest);
        let manifest_id = manifest.id();

        let mutex = aws.lock_entry_id(manifest_id);
        let guard = mutex.lock().unwrap();

        aws.set_manifest(manifest_id, manifest.clone(), false, true, None)
            .await
            .unwrap();
        assert_eq!(aws.get_manifest(manifest_id).await.unwrap(), manifest);

        aws.release_entry_id(&manifest_id, guard);
    }

    #[rstest]
    #[tokio::test]
    async fn test_realm_checkpoint(#[future] alice_workspace_storage: TmpWorkspaceStorage) {
        let aws = alice_workspace_storage.await;
        let mut manifest = create_file_manifest(&aws.device);
        let manifest_id = manifest.base.id;

        assert_eq!(aws.get_realm_checkpoint().await, 0);
        // Workspace storage starts with a speculative workspace manifest placeholder
        assert_eq!(
            aws.get_need_sync_entries().await.unwrap(),
            (HashSet::from([aws.workspace_id]), HashSet::new())
        );

        let mut workspace_manifest = create_workspace_manifest(&aws.device);
        let base = workspace_manifest.to_remote(aws.device.device_id.clone(), aws.device.now());
        workspace_manifest.base = base;
        workspace_manifest.need_sync = false;
        let workspace_manifest = LocalManifest::Workspace(workspace_manifest);
        aws.set_manifest(aws.workspace_id, workspace_manifest, false, false, None)
            .await
            .unwrap();

        assert_eq!(aws.get_realm_checkpoint().await, 0);
        assert_eq!(
            aws.get_need_sync_entries().await.unwrap(),
            (HashSet::new(), HashSet::new())
        );

        aws.update_realm_checkpoint(11, &[(manifest_id, 22), (EntryID::default(), 33)])
            .await
            .unwrap();

        assert_eq!(aws.get_realm_checkpoint().await, 11);
        assert_eq!(
            aws.get_need_sync_entries().await.unwrap(),
            (HashSet::new(), HashSet::new())
        );

        aws.set_manifest(
            manifest_id,
            LocalManifest::File(manifest.clone()),
            false,
            false,
            None,
        )
        .await
        .unwrap();

        assert_eq!(aws.get_realm_checkpoint().await, 11);
        assert_eq!(
            aws.get_need_sync_entries().await.unwrap(),
            (HashSet::from([manifest_id]), HashSet::new())
        );

        manifest.need_sync = false;
        aws.set_manifest(
            manifest_id,
            LocalManifest::File(manifest),
            false,
            false,
            None,
        )
        .await
        .unwrap();

        assert_eq!(aws.get_realm_checkpoint().await, 11);
        assert_eq!(
            aws.get_need_sync_entries().await.unwrap(),
            (HashSet::new(), HashSet::new())
        );

        aws.update_realm_checkpoint(44, &[(manifest_id, 55), (EntryID::default(), 66)])
            .await
            .unwrap();

        assert_eq!(aws.get_realm_checkpoint().await, 44);
        assert_eq!(
            aws.get_need_sync_entries().await.unwrap(),
            (HashSet::new(), HashSet::from([manifest_id]))
        );
    }

    #[rstest]
    #[tokio::test]
    async fn test_block_interface(#[future] alice_workspace_storage: TmpWorkspaceStorage) {
        let aws = alice_workspace_storage.await;
        let data = b"0123456";
        let chunk = Chunk::new(0, NonZeroU64::try_from(7).unwrap())
            .evolve_as_block(data)
            .unwrap();
        let block_id = chunk.access.unwrap().id;

        aws.clear_clean_block(block_id).await;

        assert_eq!(
            aws.get_chunk(chunk.id).await.unwrap_err(),
            FSError::LocalMiss(*chunk.id)
        );
        assert!(!aws.block_storage.is_chunk(chunk.id).await.unwrap());
        assert_eq!(aws.block_storage.get_total_size().await.unwrap(), 0);

        aws.set_clean_block(block_id, data).await.unwrap();
        assert_eq!(aws.get_chunk(chunk.id).await.unwrap(), data);
        assert!(aws.block_storage.is_chunk(chunk.id).await.unwrap());
        assert!(aws.block_storage.get_total_size().await.unwrap() >= 7);

        aws.clear_clean_block(block_id).await;
        assert_eq!(
            aws.get_chunk(chunk.id).await.unwrap_err(),
            FSError::LocalMiss(*chunk.id)
        );
        assert!(!aws.block_storage.is_chunk(chunk.id).await.unwrap());
        assert_eq!(aws.block_storage.get_total_size().await.unwrap(), 0);

        aws.set_chunk(chunk.id, data).await.unwrap();
        assert_eq!(aws.get_dirty_block(block_id).await.unwrap(), data);
    }

    #[rstest]
    #[tokio::test]
    async fn test_chunk_interface(#[future] alice_workspace_storage: TmpWorkspaceStorage) {
        let aws = alice_workspace_storage.await;
        let data = b"0123456";
        let chunk = Chunk::new(0, NonZeroU64::try_from(7).unwrap());

        assert_eq!(
            aws.get_chunk(chunk.id).await.unwrap_err(),
            FSError::LocalMiss(*chunk.id)
        );
        assert_eq!(
            aws.clear_chunk(chunk.id, false).await.unwrap_err(),
            FSError::LocalMiss(*chunk.id)
        );
        aws.clear_chunk(chunk.id, true).await.unwrap();
        assert!(!aws.chunk_storage.is_chunk(chunk.id).await.unwrap());
        assert_eq!(aws.chunk_storage.get_total_size().await.unwrap(), 0);

        aws.set_chunk(chunk.id, data).await.unwrap();
        assert_eq!(aws.get_chunk(chunk.id).await.unwrap(), data);
        assert!(aws.chunk_storage.is_chunk(chunk.id).await.unwrap());
        assert!(aws.chunk_storage.get_total_size().await.unwrap() >= 7);

        aws.clear_chunk(chunk.id, false).await.unwrap();
        assert_eq!(
            aws.get_chunk(chunk.id).await.unwrap_err(),
            FSError::LocalMiss(*chunk.id)
        );
        assert_eq!(
            aws.clear_chunk(chunk.id, false).await.unwrap_err(),
            FSError::LocalMiss(*chunk.id)
        );
        assert!(!aws.chunk_storage.is_chunk(chunk.id).await.unwrap());
        assert_eq!(aws.chunk_storage.get_total_size().await.unwrap(), 0);
        aws.clear_chunk(chunk.id, true).await.unwrap();
    }

    #[rstest]
    #[tokio::test]
    async fn test_chunk_many(#[future] alice_workspace_storage: TmpWorkspaceStorage) {
        let aws = alice_workspace_storage.await;
        let data = b"0123456";

        // More than the sqlite max argument limit to prevent regression
        let chunks_number = 2000;
        let mut chunks = Vec::with_capacity(chunks_number);

        for _ in 0..chunks_number {
            let c = Chunk::new(0, NonZeroU64::try_from(7).unwrap());
            chunks.push(c.id);
            aws.chunk_storage.set_chunk(c.id, data).await.unwrap();
        }

        assert_eq!(chunks.len(), chunks_number);
        let ret = aws.get_local_chunk_ids(&chunks).await.unwrap();
        assert_eq!(ret.len(), chunks_number);
    }

    #[rstest]
    #[tokio::test]
    async fn test_file_descriptor(#[future] alice_workspace_storage: TmpWorkspaceStorage) {
        let aws = alice_workspace_storage.await;
        let manifest = create_file_manifest(&aws.device);
        let manifest_id = manifest.base.id;

        aws.set_manifest(
            manifest_id,
            LocalManifest::File(manifest.clone()),
            false,
            false,
            None,
        )
        .await
        .unwrap();
        let fd = aws.create_file_descriptor(manifest.clone());
        assert_eq!(fd, FileDescriptor(1));

        assert_eq!(aws.load_file_descriptor(fd).await.unwrap(), manifest);

        aws.remove_file_descriptor(fd);
        assert_eq!(
            aws.load_file_descriptor(fd).await.unwrap_err(),
            FSError::InvalidFileDescriptor(fd)
        );
        assert_eq!(aws.remove_file_descriptor(fd), None);
    }

    #[rstest]
    #[tokio::test]
    async fn test_run_vacuum(#[future] alice_workspace_storage: TmpWorkspaceStorage) {
        alice_workspace_storage.await.run_vacuum().await.unwrap();
    }

    #[rstest]
    #[tokio::test]
    async fn test_garbage_collection(alice: &Device, tmp_path: TmpPath) {
        let db_path = tmp_path.join("workspace_storage.sqlite");
        let aws = WorkspaceStorage::new(
            Path::new(&db_path),
            alice.local_device(),
            EntryID::default(),
            FAILSAFE_PATTERN_FILTER.clone(),
            *DEFAULT_BLOCK_SIZE,
        )
        .await
        .unwrap();

        let block_size = NonZeroU64::try_from(*DEFAULT_BLOCK_SIZE).unwrap();
        let data = vec![0; *DEFAULT_BLOCK_SIZE as usize];
        let chunk1 = Chunk::new(0, block_size).evolve_as_block(&data).unwrap();
        let chunk2 = Chunk::new(0, block_size).evolve_as_block(&data).unwrap();
        let chunk3 = Chunk::new(0, block_size).evolve_as_block(&data).unwrap();

        assert_eq!(aws.block_storage.get_nb_blocks().await.unwrap(), 0);
        aws.set_clean_block(chunk1.access.unwrap().id, &data)
            .await
            .unwrap();
        assert_eq!(aws.block_storage.get_nb_blocks().await.unwrap(), 1);
        aws.set_clean_block(chunk2.access.unwrap().id, &data)
            .await
            .unwrap();
        assert_eq!(aws.block_storage.get_nb_blocks().await.unwrap(), 1);
        aws.set_clean_block(chunk3.access.unwrap().id, &data)
            .await
            .unwrap();
        assert_eq!(aws.block_storage.get_nb_blocks().await.unwrap(), 1);
        aws.block_storage.clear_all_blocks().await.unwrap();
        assert_eq!(aws.block_storage.get_nb_blocks().await.unwrap(), 0);
    }

    #[rstest]
    #[tokio::test]
    async fn test_invalid_regex(tmp_path: TmpPath, alice: &Device) {
        use crate::storage::manifest_storage::prevent_sync_pattern::dsl::*;
        use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};

        const INVALID_REGEX: &str = "[";
        let wid = EntryID::default();
        let valid_regex = Regex::from_regex_str("ok").unwrap();

        let db_dir = tmp_path.join("invalid-regex");
        let db_path = get_workspace_data_storage_db_path(&db_dir, &alice.local_device(), wid);
        let conn = LocalDatabase::from_path(db_path.to_str().unwrap())
            .await
            .unwrap();

        let workspace_storage = WorkspaceStorage::new(
            db_dir.clone(),
            alice.local_device(),
            wid,
            FAILSAFE_PATTERN_FILTER.clone(),
            *DEFAULT_BLOCK_SIZE,
        )
        .await
        .unwrap();

        // Ensure the entry is present.
        workspace_storage
            .set_prevent_sync_pattern(&valid_regex)
            .await
            .unwrap();

        // Corrupt the db with an invalid regex.
        conn.exec(|conn| {
            diesel::update(prevent_sync_pattern.filter(_id.eq(0).and(pattern.ne(INVALID_REGEX))))
                .set((pattern.eq(INVALID_REGEX), fully_applied.eq(false)))
                .execute(conn)
        })
        .await
        .unwrap();

        drop(workspace_storage);
        drop(conn);

        let workspace_storage = WorkspaceStorage::new(
            db_dir,
            alice.local_device(),
            wid,
            FAILSAFE_PATTERN_FILTER.clone(),
            *DEFAULT_BLOCK_SIZE,
        )
        .await
        .unwrap();
        workspace_storage.get_prevent_sync_pattern();
    }
}
