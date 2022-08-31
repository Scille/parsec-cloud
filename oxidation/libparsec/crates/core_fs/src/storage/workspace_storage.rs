// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use fancy_regex::Regex;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard, TryLockError};

use libparsec_client_types::{
    LocalDevice, LocalFileManifest, LocalManifest, LocalWorkspaceManifest,
};
use libparsec_types::{BlockID, ChunkID, EntryID, FileDescriptor};

use super::chunk_storage::ChunkStorage;
use super::manifest_storage::{ChunkOrBlockID, ManifestStorage};
use crate::error::{FSError, FSResult};
use crate::storage::chunk_storage::{BlockStorage, BlockStorageTrait, ChunkStorageTrait};
use crate::storage::local_database::SqlitePool;
use crate::storage::version::{
    get_workspace_cache_storage_db_path, get_workspace_data_storage_db_path,
};

pub const DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE: u64 = 512 * 1024 * 1024;

#[derive(Default)]
struct Locker<T: Eq + Hash>(Mutex<HashMap<T, Arc<Mutex<()>>>>);

enum Status {
    Locked,
    Released,
}

impl<T: Eq + Hash + Copy> Locker<T> {
    fn acquire(&self, id: T) -> Arc<Mutex<()>> {
        let mut map = self.0.lock().expect("Mutex is poisoned");
        map.entry(id).or_insert_with(|| Arc::new(Mutex::new(())));
        map.get(&id).unwrap_or_else(|| unreachable!()).clone()
    }
    fn release(&self, id: T, guard: MutexGuard<()>) {
        drop(guard);
        self.0.lock().expect("Mutex is poisoned").remove(&id);
    }
    fn check(&self, id: T) -> Status {
        if let Some(mutex) = self.0.lock().expect("Mutex is poisoned").get(&id) {
            if let Err(TryLockError::WouldBlock) = mutex.try_lock() {
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
}

impl WorkspaceStorage {
    pub fn new(
        // Allow different type as Path, PathBuf, String, &str
        data_base_dir: impl AsRef<Path>,
        device: LocalDevice,
        workspace_id: EntryID,
        cache_size: u64,
    ) -> FSResult<Self> {
        let data_path =
            get_workspace_data_storage_db_path(data_base_dir.as_ref(), &device, workspace_id);
        let cache_path =
            get_workspace_cache_storage_db_path(data_base_dir.as_ref(), &device, workspace_id);

        let data_pool = SqlitePool::new(
            data_path
                .to_str()
                .expect("Non-Utf-8 character found in data_path"),
        )?;
        let cache_pool = SqlitePool::new(
            cache_path
                .to_str()
                .expect("Non-Utf-8 character found in cache_path"),
        )?;

        let block_storage = BlockStorage::new(
            device.local_symkey.clone(),
            Mutex::new(cache_pool.conn()?),
            cache_size,
            device.time_provider.clone(),
        )?;

        let manifest_storage = ManifestStorage::new(
            device.local_symkey.clone(),
            Mutex::new(data_pool.conn()?),
            workspace_id,
        )?;

        let chunk_storage = ChunkStorage::new(
            device.local_symkey.clone(),
            Mutex::new(data_pool.conn()?),
            device.time_provider.clone(),
        )?;

        let (prevent_sync_pattern, prevent_sync_pattern_fully_applied) =
            manifest_storage.get_prevent_sync_pattern()?;

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
            prevent_sync_pattern: Mutex::new(prevent_sync_pattern),
            prevent_sync_pattern_fully_applied: Mutex::new(prevent_sync_pattern_fully_applied),
            // Locking structure
            entry_locks: Locker::default(),
        };

        instance.block_storage.cleanup()?;
        instance.block_storage.run_vacuum()?;
        // Populate the cache with the workspace manifest to be able to
        // access it synchronously at all time
        instance.load_workspace_manifest()?;

        Ok(instance)
    }

    pub fn lock_entry_id(&self, entry_id: EntryID) -> Arc<Mutex<()>> {
        self.entry_locks.acquire(entry_id)
    }

    pub fn release_entry_id(&self, entry_id: EntryID, guard: MutexGuard<()>) {
        self.entry_locks.release(entry_id, guard)
    }

    fn check_lock_status(&self, entry_id: EntryID) -> FSResult<()> {
        if let Status::Released = self.entry_locks.check(entry_id) {
            return Err(FSError::Runtime(entry_id));
        }
        Ok(())
    }

    pub fn get_prevent_sync_pattern(&self) -> Regex {
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

    pub fn load_file_descriptor(&self, fd: FileDescriptor) -> FSResult<LocalFileManifest> {
        match self.open_fds.lock().expect("Mutex is poisoned").get(&fd) {
            Some(&entry_id) => match self.get_manifest(entry_id) {
                Ok(LocalManifest::File(manifest)) => Ok(manifest),
                _ => Err(FSError::LocalMiss(*entry_id)),
            },
            None => Err(FSError::InvalidFileDescriptor(fd)),
        }
    }

    pub fn remove_file_descriptor(&self, fd: FileDescriptor) -> Option<EntryID> {
        self.open_fds.lock().expect("Mutex is poisoned").remove(&fd)
    }

    // Block interface

    pub fn set_clean_block(&self, block_id: BlockID, block: &[u8]) -> FSResult<()> {
        self.block_storage
            .set_chunk_upgraded(ChunkID::from(*block_id), block)
    }

    pub fn clear_clean_block(&self, block_id: BlockID) {
        let _ = self.block_storage.clear_chunk(ChunkID::from(*block_id));
    }

    pub fn get_dirty_block(&self, block_id: BlockID) -> FSResult<Vec<u8>> {
        self.chunk_storage.get_chunk(ChunkID::from(*block_id))
    }

    // Chunk interface

    pub fn get_chunk(&self, chunk_id: ChunkID) -> FSResult<Vec<u8>> {
        if let Ok(raw) = self.chunk_storage.get_chunk(chunk_id) {
            Ok(raw)
        } else if let Ok(raw) = self.block_storage.get_chunk(chunk_id) {
            Ok(raw)
        } else {
            Err(FSError::LocalMiss(*chunk_id))
        }
    }

    pub fn set_chunk(&self, chunk_id: ChunkID, block: &[u8]) -> FSResult<()> {
        self.chunk_storage.set_chunk(chunk_id, block)
    }

    pub fn clear_chunk(&self, chunk_id: ChunkID, miss_ok: bool) -> FSResult<()> {
        let res = self.chunk_storage.clear_chunk(chunk_id);
        if !miss_ok {
            return res;
        }
        Ok(())
    }

    // Helpers

    pub fn clear_memory_cache(&self, flush: bool) -> FSResult<()> {
        self.manifest_storage.clear_memory_cache(flush)
    }

    // Checkpoint interface

    pub fn get_realm_checkpoint(&self) -> i64 {
        self.manifest_storage.get_realm_checkpoint()
    }

    pub fn update_realm_checkpoint(
        &self,
        new_checkpoint: i64,
        changed_vlobs: &[(EntryID, i64)],
    ) -> FSResult<()> {
        self.manifest_storage
            .update_realm_checkpoint(new_checkpoint, changed_vlobs)
    }

    pub fn get_need_sync_entries(&self) -> FSResult<(HashSet<EntryID>, HashSet<EntryID>)> {
        self.manifest_storage.get_need_sync_entries()
    }

    // Manifest interface

    fn load_workspace_manifest(&self) -> FSResult<()> {
        if self
            .manifest_storage
            .get_manifest(self.workspace_id)
            .is_err()
        {
            let timestamp = self.device.now();
            let manifest = LocalWorkspaceManifest::new(
                self.device.device_id.clone(),
                timestamp,
                Some(self.workspace_id),
                true,
            );
            self.manifest_storage.set_manifest(
                self.workspace_id,
                LocalManifest::Workspace(manifest),
                false,
                None,
            )?;
        }

        Ok(())
    }

    pub fn get_workspace_manifest(&self) -> FSResult<LocalWorkspaceManifest> {
        let cache = self
            .manifest_storage
            .cache
            .lock()
            .expect("Mutex is poisoned");
        match cache.get(&self.workspace_id) {
            Some(LocalManifest::Workspace(manifest)) => Ok(manifest.clone()),
            _ => Err(FSError::LocalMiss(*self.workspace_id)),
        }
    }

    pub fn get_manifest(&self, entry_id: EntryID) -> FSResult<LocalManifest> {
        self.manifest_storage.get_manifest(entry_id)
    }

    pub fn set_manifest(
        &self,
        entry_id: EntryID,
        manifest: LocalManifest,
        cache_only: bool,
        check_lock_status: bool,
        removed_ids: Option<HashSet<ChunkOrBlockID>>,
    ) -> FSResult<()> {
        if check_lock_status {
            self.check_lock_status(entry_id)?;
        }
        self.manifest_storage
            .set_manifest(entry_id, manifest, cache_only, removed_ids)
    }

    pub fn ensure_manifest_persistent(
        &self,
        entry_id: EntryID,
        check_lock_status: bool,
    ) -> FSResult<()> {
        if check_lock_status {
            self.check_lock_status(entry_id)?;
        }
        self.manifest_storage.ensure_manifest_persistent(entry_id)
    }

    #[allow(deprecated)]
    pub fn clear_manifest(&self, entry_id: EntryID, check_lock_status: bool) -> FSResult<()> {
        if check_lock_status {
            self.check_lock_status(entry_id)?;
        }
        self.manifest_storage.clear_manifest(entry_id)
    }

    // Prevent sync pattern interface

    fn load_prevent_sync_pattern(&self) -> FSResult<()> {
        (
            *self.prevent_sync_pattern.lock().expect("Mutex is poisoned"),
            *self
                .prevent_sync_pattern_fully_applied
                .lock()
                .expect("Mutex is poisoned"),
        ) = self.manifest_storage.get_prevent_sync_pattern()?;
        Ok(())
    }

    pub fn set_prevent_sync_pattern(&self, pattern: &Regex) -> FSResult<()> {
        self.manifest_storage.set_prevent_sync_pattern(pattern)?;
        self.load_prevent_sync_pattern()
    }

    pub fn mark_prevent_sync_pattern_fully_applied(&self, pattern: &Regex) -> FSResult<()> {
        self.manifest_storage
            .mark_prevent_sync_pattern_fully_applied(pattern)?;
        self.load_prevent_sync_pattern()
    }

    pub fn get_local_block_ids(&self, chunk_ids: &[ChunkID]) -> FSResult<Vec<ChunkID>> {
        self.block_storage.get_local_chunk_ids(chunk_ids)
    }

    pub fn get_local_chunk_ids(&self, chunk_ids: &[ChunkID]) -> FSResult<Vec<ChunkID>> {
        self.chunk_storage.get_local_chunk_ids(chunk_ids)
    }

    pub fn run_vacuum(&self) -> FSResult<()> {
        // TODO: Add some condition
        self.chunk_storage.run_vacuum()
    }
}

#[cfg(test)]
mod tests {
    use crate::conftest::{alice_workspace_storage, TmpWorkspaceStorage};
    use libparsec_client_types::Chunk;
    use libparsec_types::{Blocksize, DEFAULT_BLOCK_SIZE};
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

    fn clear_cache(storage: &WorkspaceStorage) {
        storage.manifest_storage.cache.lock().unwrap().clear();
    }

    #[rstest]
    fn test_lock_required(alice_workspace_storage: TmpWorkspaceStorage) {
        let aws = alice_workspace_storage;
        let manifest = create_workspace_manifest(&aws.device);
        let manifest = LocalManifest::Workspace(manifest);
        let manifest_id = manifest.id();

        assert_eq!(
            aws.set_manifest(manifest_id, manifest, false, true, None)
                .unwrap_err(),
            FSError::Runtime(manifest_id)
        );

        assert_eq!(
            aws.ensure_manifest_persistent(manifest_id, true)
                .unwrap_err(),
            FSError::Runtime(manifest_id)
        );

        assert_eq!(
            aws.clear_manifest(manifest_id, true).unwrap_err(),
            FSError::Runtime(manifest_id)
        );
    }

    #[rstest]
    fn test_basic_set_get_clear(alice_workspace_storage: TmpWorkspaceStorage) {
        let aws = alice_workspace_storage;
        let manifest = create_workspace_manifest(&aws.device);
        let manifest = LocalManifest::Workspace(manifest);
        let manifest_id = manifest.id();

        let mutex = aws.lock_entry_id(manifest_id);
        let guard = mutex.lock().unwrap();

        // 1) No data
        assert_eq!(
            aws.get_manifest(manifest_id).unwrap_err(),
            FSError::LocalMiss(*manifest_id)
        );

        // 2) Set data
        aws.set_manifest(manifest_id, manifest.clone(), false, true, None)
            .unwrap();
        assert_eq!(aws.get_manifest(manifest_id).unwrap(), manifest);

        // Make sure data are not only stored in cache
        clear_cache(&aws);
        assert_eq!(aws.get_manifest(manifest_id).unwrap(), manifest);

        // 3) Clear data
        aws.clear_manifest(manifest_id, true).unwrap();

        assert_eq!(
            aws.get_manifest(manifest_id).unwrap_err(),
            FSError::LocalMiss(*manifest_id)
        );

        assert_eq!(
            aws.clear_manifest(manifest_id, true).unwrap_err(),
            FSError::LocalMiss(*manifest_id)
        );

        aws.release_entry_id(manifest_id, guard);
    }

    #[rstest]
    fn test_cache_set_get(alice_workspace_storage: TmpWorkspaceStorage) {
        let aws = alice_workspace_storage;
        let manifest = create_workspace_manifest(&aws.device);
        let manifest = LocalManifest::Workspace(manifest);
        let manifest_id = manifest.id();

        let mutex = aws.lock_entry_id(manifest_id);
        let guard = mutex.lock().unwrap();

        // 1) Set data
        aws.set_manifest(manifest_id, manifest.clone(), true, true, None)
            .unwrap();
        assert_eq!(aws.get_manifest(manifest_id).unwrap(), manifest);

        // Data should be set only in the cache
        clear_cache(&aws);
        assert_eq!(
            aws.get_manifest(manifest_id).unwrap_err(),
            FSError::LocalMiss(*manifest_id)
        );

        // Re-set data
        aws.set_manifest(manifest_id, manifest.clone(), true, true, None)
            .unwrap();

        // 2) Clear should work as expected
        aws.clear_manifest(manifest_id, true).unwrap();
        assert_eq!(
            aws.get_manifest(manifest_id).unwrap_err(),
            FSError::LocalMiss(*manifest_id)
        );

        // Re-set data
        aws.set_manifest(manifest_id, manifest.clone(), true, true, None)
            .unwrap();

        // 3) Flush data
        aws.ensure_manifest_persistent(manifest_id, true).unwrap();
        assert_eq!(aws.get_manifest(manifest_id).unwrap(), manifest);

        // Data should be persistent in real database
        clear_cache(&aws);
        assert_eq!(aws.get_manifest(manifest_id).unwrap(), manifest);

        // 4) Idempotency
        aws.ensure_manifest_persistent(manifest_id, true).unwrap();

        aws.release_entry_id(manifest_id, guard);
    }

    #[rstest]
    #[case(false, false)]
    #[case(false, true)]
    #[case(true, false)]
    #[case(true, true)]
    fn test_chunk_clearing(
        alice_workspace_storage: TmpWorkspaceStorage,
        #[case] cache_only: bool,
        #[case] clear_manifest: bool,
    ) {
        let aws = alice_workspace_storage;
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
        aws.set_chunk(chunk1.id, data1).unwrap();
        aws.set_chunk(chunk2.id, data2).unwrap();
        aws.set_manifest(manifest_id, manifest, false, true, None)
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
        .unwrap();

        if cache_only {
            // The chunks are still accessible
            assert_eq!(aws.get_chunk(chunk1.id).unwrap(), b"abc");
            assert_eq!(aws.get_chunk(chunk2.id).unwrap(), b"def");
        } else {
            // The chunks are gone
            assert_eq!(
                aws.get_chunk(chunk1.id).unwrap_err(),
                FSError::LocalMiss(*chunk1.id)
            );
            assert_eq!(
                aws.get_chunk(chunk2.id).unwrap_err(),
                FSError::LocalMiss(*chunk2.id)
            );
        }

        // Now flush the manifest
        if clear_manifest {
            aws.clear_manifest(manifest_id, true).unwrap();
        } else {
            aws.ensure_manifest_persistent(manifest_id, true).unwrap();
        }

        // The chunks are gone
        assert_eq!(
            aws.get_chunk(chunk1.id).unwrap_err(),
            FSError::LocalMiss(*chunk1.id)
        );
        assert_eq!(
            aws.get_chunk(chunk2.id).unwrap_err(),
            FSError::LocalMiss(*chunk2.id)
        );

        // Idempotency
        aws.ensure_manifest_persistent(manifest_id, true).unwrap();

        aws.release_entry_id(manifest_id, guard);
    }

    #[rstest]
    fn test_cache_flushed_on_exit(alice: &Device, tmp_path: TmpPath) {
        let db_path = tmp_path.join("workspace_storage.sqlite");
        let manifest = create_workspace_manifest(&alice.local_device());
        let manifest = LocalManifest::Workspace(manifest);
        let manifest_id = manifest.id();
        let workspace_id = EntryID::default();

        let aws = WorkspaceStorage::new(
            Path::new(&db_path),
            alice.local_device(),
            workspace_id,
            DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE,
        )
        .unwrap();

        let mutex = aws.lock_entry_id(manifest_id);
        let guard = mutex.lock().unwrap();

        aws.set_manifest(manifest_id, manifest.clone(), true, true, None)
            .unwrap();

        aws.release_entry_id(manifest_id, guard);

        drop(aws);

        let aws = WorkspaceStorage::new(
            Path::new(&db_path),
            alice.local_device(),
            workspace_id,
            DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE,
        )
        .unwrap();

        assert_eq!(aws.get_manifest(manifest_id).unwrap(), manifest);
    }

    #[rstest]
    fn test_clear_cache(alice_workspace_storage: TmpWorkspaceStorage) {
        let aws = alice_workspace_storage;
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
            .unwrap();
        aws.set_manifest(manifest2_id, manifest2.clone(), true, true, None)
            .unwrap();

        // Clear without flushing
        aws.clear_memory_cache(false).unwrap();

        // Manifest 1 is present but manifest2 got lost
        assert_eq!(aws.get_manifest(manifest1_id).unwrap(), manifest1);
        assert_eq!(
            aws.get_manifest(manifest2_id).unwrap_err(),
            FSError::LocalMiss(*manifest2_id)
        );

        // Set Manifest 2, cache only
        aws.set_manifest(manifest2_id, manifest2.clone(), true, true, None)
            .unwrap();

        // Clear with flushing
        aws.clear_memory_cache(true).unwrap();
        assert_eq!(aws.get_manifest(manifest2_id).unwrap(), manifest2);

        aws.release_entry_id(manifest1_id, guard1);
        aws.release_entry_id(manifest2_id, guard2);
    }

    #[rstest]
    fn test_serialize_non_empty_local_file_manifest(alice_workspace_storage: TmpWorkspaceStorage) {
        let aws = alice_workspace_storage;
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
            .unwrap();
        assert_eq!(aws.get_manifest(manifest_id).unwrap(), manifest);

        aws.release_entry_id(manifest_id, guard);
    }

    #[rstest]
    fn test_realm_checkpoint(alice_workspace_storage: TmpWorkspaceStorage) {
        let aws = alice_workspace_storage;
        let mut manifest = create_file_manifest(&aws.device);
        let manifest_id = manifest.base.id;

        assert_eq!(aws.get_realm_checkpoint(), 0);
        // Workspace storage starts with a speculative workspace manifest placeholder
        assert_eq!(
            aws.get_need_sync_entries().unwrap(),
            (HashSet::from([aws.workspace_id]), HashSet::new())
        );

        let mut workspace_manifest = create_workspace_manifest(&aws.device);
        let base = workspace_manifest.to_remote(aws.device.device_id.clone(), aws.device.now());
        workspace_manifest.base = base;
        workspace_manifest.need_sync = false;
        let workspace_manifest = LocalManifest::Workspace(workspace_manifest);
        aws.set_manifest(aws.workspace_id, workspace_manifest, false, false, None)
            .unwrap();

        assert_eq!(aws.get_realm_checkpoint(), 0);
        assert_eq!(
            aws.get_need_sync_entries().unwrap(),
            (HashSet::new(), HashSet::new())
        );

        aws.update_realm_checkpoint(11, &[(manifest_id, 22), (EntryID::default(), 33)])
            .unwrap();

        assert_eq!(aws.get_realm_checkpoint(), 11);
        assert_eq!(
            aws.get_need_sync_entries().unwrap(),
            (HashSet::new(), HashSet::new())
        );

        aws.set_manifest(
            manifest_id,
            LocalManifest::File(manifest.clone()),
            false,
            false,
            None,
        )
        .unwrap();

        assert_eq!(aws.get_realm_checkpoint(), 11);
        assert_eq!(
            aws.get_need_sync_entries().unwrap(),
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
        .unwrap();

        assert_eq!(aws.get_realm_checkpoint(), 11);
        assert_eq!(
            aws.get_need_sync_entries().unwrap(),
            (HashSet::new(), HashSet::new())
        );

        aws.update_realm_checkpoint(44, &[(manifest_id, 55), (EntryID::default(), 66)])
            .unwrap();

        assert_eq!(aws.get_realm_checkpoint(), 44);
        assert_eq!(
            aws.get_need_sync_entries().unwrap(),
            (HashSet::new(), HashSet::from([manifest_id]))
        );
    }

    #[rstest]
    fn test_block_interface(alice_workspace_storage: TmpWorkspaceStorage) {
        let aws = alice_workspace_storage;
        let data = b"0123456";
        let chunk = Chunk::new(0, NonZeroU64::try_from(7).unwrap())
            .evolve_as_block(data)
            .unwrap();
        let block_id = chunk.access.unwrap().id;

        aws.clear_clean_block(block_id);

        assert_eq!(
            aws.get_chunk(chunk.id).unwrap_err(),
            FSError::LocalMiss(*chunk.id)
        );
        assert!(!aws.block_storage.is_chunk(chunk.id).unwrap());
        assert_eq!(aws.block_storage.get_total_size().unwrap(), 0);

        aws.set_clean_block(block_id, data).unwrap();
        assert_eq!(aws.get_chunk(chunk.id).unwrap(), data);
        assert!(aws.block_storage.is_chunk(chunk.id).unwrap());
        assert!(aws.block_storage.get_total_size().unwrap() >= 7);

        aws.clear_clean_block(block_id);
        assert_eq!(
            aws.get_chunk(chunk.id).unwrap_err(),
            FSError::LocalMiss(*chunk.id)
        );
        assert!(!aws.block_storage.is_chunk(chunk.id).unwrap());
        assert_eq!(aws.block_storage.get_total_size().unwrap(), 0);

        aws.set_chunk(chunk.id, data).unwrap();
        assert_eq!(aws.get_dirty_block(block_id).unwrap(), data);
    }

    #[rstest]
    fn test_chunk_interface(alice_workspace_storage: TmpWorkspaceStorage) {
        let aws = alice_workspace_storage;
        let data = b"0123456";
        let chunk = Chunk::new(0, NonZeroU64::try_from(7).unwrap());

        assert_eq!(
            aws.get_chunk(chunk.id).unwrap_err(),
            FSError::LocalMiss(*chunk.id)
        );
        assert_eq!(
            aws.clear_chunk(chunk.id, false).unwrap_err(),
            FSError::LocalMiss(*chunk.id)
        );
        aws.clear_chunk(chunk.id, true).unwrap();
        assert!(!aws.chunk_storage.is_chunk(chunk.id).unwrap());
        assert_eq!(aws.chunk_storage.get_total_size().unwrap(), 0);

        aws.set_chunk(chunk.id, data).unwrap();
        assert_eq!(aws.get_chunk(chunk.id).unwrap(), data);
        assert!(aws.chunk_storage.is_chunk(chunk.id).unwrap());
        assert!(aws.chunk_storage.get_total_size().unwrap() >= 7);

        aws.clear_chunk(chunk.id, false).unwrap();
        assert_eq!(
            aws.get_chunk(chunk.id).unwrap_err(),
            FSError::LocalMiss(*chunk.id)
        );
        assert_eq!(
            aws.clear_chunk(chunk.id, false).unwrap_err(),
            FSError::LocalMiss(*chunk.id)
        );
        assert!(!aws.chunk_storage.is_chunk(chunk.id).unwrap());
        assert_eq!(aws.chunk_storage.get_total_size().unwrap(), 0);
        aws.clear_chunk(chunk.id, true).unwrap();
    }

    #[rstest]
    fn test_chunk_many(alice_workspace_storage: TmpWorkspaceStorage) {
        let aws = alice_workspace_storage;
        let data = b"0123456";

        // More than the sqlite max argument limit to prevent regression
        let chunks_number = 2000;
        let mut chunks = Vec::with_capacity(chunks_number);

        for _ in 0..chunks_number {
            let c = Chunk::new(0, NonZeroU64::try_from(7).unwrap());
            chunks.push(c.id);
            aws.chunk_storage.set_chunk(c.id, data).unwrap();
        }

        assert_eq!(chunks.len(), chunks_number);
        let ret = aws.get_local_chunk_ids(&chunks).unwrap();
        assert_eq!(ret.len(), chunks_number);
    }

    #[rstest]
    fn test_file_descriptor(alice_workspace_storage: TmpWorkspaceStorage) {
        let aws = alice_workspace_storage;
        let manifest = create_file_manifest(&aws.device);
        let manifest_id = manifest.base.id;

        aws.set_manifest(
            manifest_id,
            LocalManifest::File(manifest.clone()),
            false,
            false,
            None,
        )
        .unwrap();
        let fd = aws.create_file_descriptor(manifest.clone());
        assert_eq!(fd, FileDescriptor(1));

        assert_eq!(aws.load_file_descriptor(fd).unwrap(), manifest);

        aws.remove_file_descriptor(fd);
        assert_eq!(
            aws.load_file_descriptor(fd).unwrap_err(),
            FSError::InvalidFileDescriptor(fd)
        );
        assert_eq!(aws.remove_file_descriptor(fd), None);
    }

    #[rstest]
    fn test_run_vacuum(alice_workspace_storage: TmpWorkspaceStorage) {
        alice_workspace_storage.run_vacuum().unwrap();
    }

    #[rstest]
    fn test_garbage_collection(alice: &Device, tmp_path: TmpPath) {
        let db_path = tmp_path.join("workspace_storage.sqlite");
        let aws = WorkspaceStorage::new(
            Path::new(&db_path),
            alice.local_device(),
            EntryID::default(),
            *DEFAULT_BLOCK_SIZE,
        )
        .unwrap();

        let block_size = NonZeroU64::try_from(*DEFAULT_BLOCK_SIZE).unwrap();
        let data = vec![0; *DEFAULT_BLOCK_SIZE as usize];
        let chunk1 = Chunk::new(0, block_size).evolve_as_block(&data).unwrap();
        let chunk2 = Chunk::new(0, block_size).evolve_as_block(&data).unwrap();
        let chunk3 = Chunk::new(0, block_size).evolve_as_block(&data).unwrap();

        assert_eq!(aws.block_storage.get_nb_blocks().unwrap(), 0);
        aws.set_clean_block(chunk1.access.unwrap().id, &data)
            .unwrap();
        assert_eq!(aws.block_storage.get_nb_blocks().unwrap(), 1);
        aws.set_clean_block(chunk2.access.unwrap().id, &data)
            .unwrap();
        assert_eq!(aws.block_storage.get_nb_blocks().unwrap(), 1);
        aws.set_clean_block(chunk3.access.unwrap().id, &data)
            .unwrap();
        assert_eq!(aws.block_storage.get_nb_blocks().unwrap(), 1);
        aws.block_storage.clear_all_blocks().unwrap();
        assert_eq!(aws.block_storage.get_nb_blocks().unwrap(), 0);
    }
}
