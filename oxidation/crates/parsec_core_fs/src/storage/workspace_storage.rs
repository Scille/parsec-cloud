// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use fancy_regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::Mutex;

use parsec_api_types::{BlockID, ChunkID, EntryID, FileDescriptor};
use parsec_client_types::{LocalDevice, LocalFileManifest, LocalManifest, LocalWorkspaceManifest};

use super::chunk_storage::ChunkStorage;
use super::manifest_storage::{ChunkOrBlockID, ManifestStorage};
use crate::error::{FSError, FSResult};
use crate::storage::chunk_storage::{BlockStorage, BlockStorageTrait, ChunkStorageTrait};
use crate::storage::local_database::SqlitePool;
use crate::storage::version::{
    get_workspace_cache_storage_db_path, get_workspace_data_storage_db_path,
};

pub const DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE: u64 = 512 * 1024 * 1024;

pub struct WorkspaceStorage {
    pub device: LocalDevice,
    pub workspace_id: EntryID,
    open_fds: Mutex<HashMap<FileDescriptor, EntryID>>,
    fd_counter: Mutex<i32>,
    block_storage: BlockStorage,
    chunk_storage: ChunkStorage,
    manifest_storage: ManifestStorage,
    prevent_sync_pattern: Mutex<Regex>,
    prevent_sync_pattern_fully_applied: Mutex<bool>,
}

impl WorkspaceStorage {
    pub fn new(
        data_base_dir: &Path,
        device: LocalDevice,
        workspace_id: EntryID,
        cache_size: u64,
    ) -> FSResult<Self> {
        let data_path = get_workspace_data_storage_db_path(data_base_dir, &device, workspace_id);
        let cache_path = get_workspace_cache_storage_db_path(data_base_dir, &device, workspace_id);

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
        )?;

        let manifest_storage = ManifestStorage::new(
            device.local_symkey.clone(),
            Mutex::new(data_pool.conn()?),
            workspace_id,
        )?;

        let chunk_storage =
            ChunkStorage::new(device.local_symkey.clone(), Mutex::new(data_pool.conn()?))?;

        let (prevent_sync_pattern, prevent_sync_pattern_fully_applied) =
            manifest_storage.get_prevent_sync_pattern()?;

        // Instanciate workspace storage
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
        };

        instance.block_storage.cleanup()?;
        instance.block_storage.run_vacuum()?;
        // Populate the cache with the workspace manifest to be able to
        // access it synchronously at all time
        instance.load_workspace_manifest()?;

        Ok(instance)
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

    pub fn get_realm_checkpoint(&self) -> i32 {
        self.manifest_storage.get_realm_checkpoint()
    }

    pub fn update_realm_checkpoint(
        &self,
        new_checkpoint: i32,
        changed_vlobs: &[(EntryID, i32)],
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
            let timestamp = self.device.timestamp();
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
        removed_ids: Option<HashSet<ChunkOrBlockID>>,
    ) -> FSResult<()> {
        self.manifest_storage
            .set_manifest(entry_id, manifest, cache_only, removed_ids)
    }

    pub fn ensure_manifest_persistent(&self, entry_id: EntryID) -> FSResult<()> {
        self.manifest_storage.ensure_manifest_persistent(entry_id)
    }

    pub fn clear_manifest(&self, entry_id: EntryID) -> FSResult<()> {
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

    pub fn block_storage_get_local_chunk_ids(
        &self,
        chunk_ids: &[ChunkID],
    ) -> FSResult<Vec<ChunkID>> {
        self.block_storage.get_local_chunk_ids(chunk_ids)
    }

    pub fn chunk_storage_get_local_chunk_ids(
        &self,
        chunk_ids: &[ChunkID],
    ) -> FSResult<Vec<ChunkID>> {
        self.chunk_storage.get_local_chunk_ids(chunk_ids)
    }

    pub fn run_vacuum(&self) -> FSResult<()> {
        // TODO: Add some condition
        self.chunk_storage.run_vacuum()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use tests_fixtures::{alice, Device};

    use super::*;

    #[rstest]
    fn workspace_storage(alice: &Device) {
        let _workspace_storage = WorkspaceStorage::new(
            &Path::new("/tmp/workspace_storage/"),
            alice.local_device(),
            EntryID::default(),
            DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE,
        )
        .unwrap();
    }
}
