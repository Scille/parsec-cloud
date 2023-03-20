// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{
    collections::{HashMap, HashSet},
    path::Path,
    sync::{Arc, Mutex},
};

use libparsec_client_types::{
    LocalDevice, LocalFileManifest, LocalFolderManifest, LocalManifest, LocalUserManifest,
    LocalWorkspaceManifest,
};
use libparsec_platform_async::Mutex as AsyncMutex;
use libparsec_platform_local_db::{AutoVacuum, LocalDatabase, VacuumMode};
use libparsec_types::{BlockID, ChunkID, EntryID, FileDescriptor, Regex};

use super::{
    chunk_storage::{ChunkStorage, Remanence},
    manifest_storage::{ChunkOrBlockID, ManifestStorage},
};
use crate::{
    error::{FSError, FSResult},
    storage::{
        chunk_storage::{BlockStorage, BlockStorageTrait, ChunkStorageTrait},
        version::{
            get_workspace_cache_storage_db_relative_path,
            get_workspace_data_storage_db_relative_path,
        },
    },
};

/// The default threshold at which when vacuuming the chunk storage, will start to do things.
pub const DEFAULT_CHUNK_VACUUM_THRESHOLD: usize = 512 * 1024 * 1024;

/// The default cache size to store block
pub const DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE: u64 = 512 * 1024 * 1024;

lazy_static::lazy_static! {
    pub static ref FAILSAFE_PATTERN_FILTER: Regex = {
        Regex::from_regex_str("^\\b$").expect("Must be a valid regex")
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// A File or Folder local manifest.
pub enum LocalFileOrFolderManifest {
    File(LocalFileManifest),
    Folder(LocalFolderManifest),
}

impl LocalFileOrFolderManifest {
    pub fn need_sync(&self) -> bool {
        match self {
            LocalFileOrFolderManifest::File(manifest) => manifest.need_sync,
            LocalFileOrFolderManifest::Folder(manifest) => manifest.need_sync,
        }
    }
}

impl TryFrom<LocalManifest> for LocalFileOrFolderManifest {
    type Error = LocalUserOrWorkspaceManifest;

    fn try_from(value: LocalManifest) -> Result<Self, Self::Error> {
        match value {
            LocalManifest::File(manifest) => Ok(Self::File(manifest)),
            LocalManifest::Folder(manifest) => Ok(Self::Folder(manifest)),
            LocalManifest::Workspace(manifest) => Err(manifest.into()),
            LocalManifest::User(manifest) => Err(manifest.into()),
        }
    }
}

impl From<LocalFileOrFolderManifest> for LocalManifest {
    fn from(value: LocalFileOrFolderManifest) -> Self {
        match value {
            LocalFileOrFolderManifest::File(manifest) => LocalManifest::File(manifest),
            LocalFileOrFolderManifest::Folder(manifest) => LocalManifest::Folder(manifest),
        }
    }
}

/// A User or Workspace manifest.
pub enum LocalUserOrWorkspaceManifest {
    User(LocalUserManifest),
    Workspace(LocalWorkspaceManifest),
}

impl From<LocalUserManifest> for LocalUserOrWorkspaceManifest {
    fn from(value: LocalUserManifest) -> Self {
        Self::User(value)
    }
}

impl From<LocalWorkspaceManifest> for LocalUserOrWorkspaceManifest {
    fn from(value: LocalWorkspaceManifest) -> Self {
        Self::Workspace(value)
    }
}

/// WorkspaceStorage is implemented with interior mutability because
/// we want some parallelism between its fields (e.g open_fds)
// TODO: Currently we handle EntryID lock in Python, should be implemented here instead
pub struct WorkspaceStorage {
    pub device: Arc<LocalDevice>,
    pub workspace_id: EntryID,
    open_fds: Mutex<HashMap<FileDescriptor, EntryID>>,
    fd_counter: Mutex<u32>,
    block_storage: BlockStorage,
    chunk_storage: ChunkStorage,
    manifest_storage: ManifestStorage,
    prevent_sync_pattern: Mutex<Regex>,
    prevent_sync_pattern_fully_applied: Mutex<bool>,
    lock_manifest_udpate: AsyncMutex<()>,
    /// Keep a copy of the workspace manifest to have it available at all time.
    /// (We don't rely on [ManifestStorage]'s cache since it can be cleared).
    workspace_manifest_copy: Mutex<LocalWorkspaceManifest>,
}

impl WorkspaceStorage {
    pub async fn new(
        data_base_dir: &Path,
        device: Arc<LocalDevice>,
        workspace_id: EntryID,
        prevent_sync_pattern: Regex,
        data_vacuum_threshold: usize,
        cache_size: u64,
    ) -> FSResult<Self> {
        let data_relative_path = get_workspace_data_storage_db_relative_path(&device, workspace_id);
        let cache_relative_path =
            get_workspace_cache_storage_db_relative_path(&device, workspace_id);

        // TODO: once the auto_vacuum approach has been validated for the cache storage,
        // we should investigate whether it is a good fit for the data storage.
        let data_conn = LocalDatabase::from_path(
            data_base_dir,
            &data_relative_path,
            VacuumMode::WithThreshold(data_vacuum_threshold),
        )
        .await?;
        let data_conn = Arc::new(data_conn);
        let cache_conn = LocalDatabase::from_path(
            data_base_dir,
            &cache_relative_path,
            VacuumMode::Automatic(AutoVacuum::Full),
        )
        .await?;

        let block_storage = BlockStorage::new(cache_conn, device.clone(), cache_size).await?;

        let manifest_storage =
            ManifestStorage::new(data_conn.clone(), device.clone(), workspace_id).await?;

        let chunk_storage = ChunkStorage::new(data_conn, device.clone()).await?;

        // Populate the cache with the workspace manifest to be able to
        // access it synchronously at all time
        let workspace_manifest =
            WorkspaceStorage::load_workspace_manifest(&manifest_storage, workspace_id, &device)
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

            lock_manifest_udpate: AsyncMutex::new(()),
            workspace_manifest_copy: Mutex::new(workspace_manifest),
        };

        instance
            .set_prevent_sync_pattern(&prevent_sync_pattern)
            .await?;

        instance.block_storage.cleanup().await?;
        instance.block_storage.run_vacuum().await?;

        Ok(instance)
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
            .copied()
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

    pub async fn set_clean_block(
        &self,
        block_id: BlockID,
        block: &[u8],
    ) -> FSResult<HashSet<BlockID>> {
        self.block_storage.set_clean_block(block_id, block).await
    }

    pub async fn is_clean_block(&self, block_id: BlockID) -> FSResult<bool> {
        self.block_storage.is_chunk(ChunkID::from(*block_id)).await
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
        if !miss_ok || res != Err(FSError::LocalMiss(*chunk_id)) {
            return res;
        }
        Ok(())
    }

    pub async fn clear_chunks(&self, chunk_ids: &[ChunkID]) -> FSResult<()> {
        self.chunk_storage.clear_chunks(chunk_ids).await
    }

    pub async fn remove_clean_blocks(&self, block_ids: &[BlockID]) -> FSResult<()> {
        let chunk_ids = block_ids
            .iter()
            .map(|id| ChunkID::from(*id.as_bytes()))
            .collect::<Vec<_>>();
        self.block_storage.clear_chunks(&chunk_ids).await
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

    async fn load_workspace_manifest(
        manifest_storage: &ManifestStorage,
        workspace_id: EntryID,
        device: &LocalDevice,
    ) -> FSResult<LocalWorkspaceManifest> {
        match manifest_storage.get_manifest(workspace_id).await {
            Ok(LocalManifest::Workspace(manifest)) => Ok(manifest),
            Ok(_) => panic!(
                "Workspace manifest id is used for something other than a workspace manifest"
            ),
            // It is possible to lack the workspace manifest in local if our
            // device hasn't tried to access it yet (and we are not the creator
            // of the workspace, in which case the workspacefs local db is
            // initialized with a non-speculative local manifest placeholder).
            // In such case it is easy to fall back on an empty manifest
            // which is a good enough approximation of the very first version
            // of the manifest (field `created` is invalid, but it will be
            // correction by the merge during sync).
            // This approach also guarantees the workspace root folder is always
            // consistent (ls/touch/mkdir always works on it), which is not the
            // case for the others files and folders (as their access may
            // require communication with the backend).
            // This is especially important when the workspace is accessed from
            // file system mountpoint given having a weird error popup when clicking
            // on the mountpoint from the file explorer really feel like a bug :/
            Err(_) => {
                let timestamp = device.now();
                let manifest = LocalWorkspaceManifest::new(
                    device.device_id.clone(),
                    timestamp,
                    Some(workspace_id),
                    true,
                );
                manifest_storage
                    .set_manifest(
                        workspace_id,
                        LocalManifest::Workspace(manifest.clone()),
                        false,
                        None,
                    )
                    .await
                    .and(Ok(manifest))
            }
        }
    }

    pub fn get_workspace_manifest(&self) -> LocalWorkspaceManifest {
        self.workspace_manifest_copy
            .lock()
            .expect("Mutex is poisoned")
            .clone()
    }

    pub async fn get_manifest(&self, entry_id: EntryID) -> FSResult<LocalManifest> {
        self.manifest_storage.get_manifest(entry_id).await
    }

    pub fn set_manifest_in_cache(
        &self,
        entry_id: EntryID,
        manifest: LocalFileOrFolderManifest,
        removed_ids: Option<HashSet<ChunkOrBlockID>>,
    ) -> FSResult<()> {
        self.manifest_storage
            .set_manifest_cache_only(entry_id, manifest.into(), removed_ids);
        Ok(())
    }

    pub async fn set_manifest(
        &self,
        entry_id: EntryID,
        manifest: LocalFileOrFolderManifest,
        cache_only: bool,
        removed_ids: Option<HashSet<ChunkOrBlockID>>,
    ) -> FSResult<()> {
        self.manifest_storage
            .set_manifest(entry_id, manifest.into(), cache_only, removed_ids)
            .await?;
        Ok(())
    }

    pub async fn set_workspace_manifest(&self, manifest: LocalWorkspaceManifest) -> FSResult<()> {
        if manifest.base.id != self.workspace_id {
            panic!("Trying to set a workspace manifest which id isn't the same as the WorkspaceStorage::id (manifest_id={}, workspace_id={})", manifest.base.id, self.workspace_id)
        }
        let guard = self.lock_manifest_udpate.lock().await;

        self.manifest_storage
            .set_manifest(self.workspace_id, manifest.clone().into(), false, None)
            .await?;

        *self
            .workspace_manifest_copy
            .lock()
            .expect("Mutex is poisoned") = manifest;

        drop(guard);
        Ok(())
    }

    pub async fn ensure_manifest_persistent(&self, entry_id: EntryID) -> FSResult<()> {
        self.manifest_storage
            .ensure_manifest_persistent(entry_id)
            .await
    }

    #[cfg(any(test, feature = "test-utils"))]
    pub async fn clear_manifest(&self, entry_id: &EntryID) -> FSResult<()> {
        self.manifest_storage.clear_manifest(*entry_id).await
    }

    /// Close the connections to the databases.
    /// Provide a way to manually close those connections.
    /// Event tho they will be closed when [WorkspaceStorage] is dropped.
    pub async fn close_connections(&self) {
        self.manifest_storage.close_connection().await;
        self.chunk_storage.close_connection().await;
        self.block_storage.close_connection().await;
    }

    // Prevent sync pattern interface

    fn load_prevent_sync_pattern(&self, re: &Regex, fully_applied: bool) {
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
        self.load_prevent_sync_pattern(pattern, fully_applied);
        Ok(())
    }

    pub async fn mark_prevent_sync_pattern_fully_applied(&self, pattern: &Regex) -> FSResult<()> {
        let fully_applied = self
            .manifest_storage
            .mark_prevent_sync_pattern_fully_applied(pattern)
            .await?;
        self.load_prevent_sync_pattern(pattern, fully_applied);
        Ok(())
    }

    pub async fn get_local_block_ids(&self, chunk_ids: &[ChunkID]) -> FSResult<Vec<ChunkID>> {
        self.block_storage.get_local_chunk_ids(chunk_ids).await
    }

    pub async fn get_local_chunk_ids(&self, chunk_ids: &[ChunkID]) -> FSResult<Vec<ChunkID>> {
        self.chunk_storage.get_local_chunk_ids(chunk_ids).await
    }

    pub async fn run_vacuum(&self) -> FSResult<()> {
        self.chunk_storage.run_vacuum().await
    }

    /// Return `true` when the given manifest identified by `entry_id` is cached.
    pub fn is_manifest_cache_ahead_of_persistance(&self, entry_id: &EntryID) -> bool {
        self.manifest_storage
            .is_manifest_cache_ahead_of_persistance(entry_id)
    }

    pub fn get_manifest_in_cache(&self, entry_id: &EntryID) -> Option<LocalManifest> {
        self.manifest_storage.get_manifest_in_cache(entry_id)
    }
}

#[async_trait::async_trait]
impl Remanence for WorkspaceStorage {
    fn is_block_remanent(&self) -> bool {
        self.block_storage.is_block_remanent()
    }

    async fn enable_block_remanence(&self) -> FSResult<bool> {
        self.block_storage.enable_block_remanence().await
    }

    async fn disable_block_remanence(&self) -> FSResult<Option<HashSet<BlockID>>> {
        self.block_storage.disable_block_remanence().await
    }

    async fn clear_unreferenced_chunks(
        &self,
        chunk_ids: &[ChunkID],
        not_accessed_after: libparsec_types::DateTime,
    ) -> FSResult<()> {
        self.block_storage
            .clear_unreferenced_chunks(chunk_ids, not_accessed_after)
            .await
    }
}

pub async fn workspace_storage_non_speculative_init(
    data_base_dir: &Path,
    device: Arc<LocalDevice>,
    workspace_id: EntryID,
) -> FSResult<()> {
    let data_relative_path = get_workspace_data_storage_db_relative_path(&device, workspace_id);
    let conn =
        LocalDatabase::from_path(data_base_dir, &data_relative_path, VacuumMode::default()).await?;
    let conn = Arc::new(conn);
    let manifest_storage = ManifestStorage::new(conn, device.clone(), workspace_id).await?;
    let timestamp = device.now();
    let manifest = LocalWorkspaceManifest::new(
        device.device_id.clone(),
        timestamp,
        Some(workspace_id),
        false,
    );

    manifest_storage
        .set_manifest(
            workspace_id,
            LocalManifest::Workspace(manifest),
            false,
            None,
        )
        .await?;
    manifest_storage.clear_memory_cache(true).await?;
    manifest_storage.close_connection().await;

    Ok(())
}

#[cfg(test)]
mod tests {
    // TODO: add tests for `workspace_storage_non_speculative_init` !

    use libparsec_client_types::Chunk;
    use libparsec_tests_fixtures::TestbedScope;
    use libparsec_types::{Blocksize, Regex, DEFAULT_BLOCK_SIZE};
    use rstest::rstest;
    use std::num::NonZeroU64;

    use super::*;

    async fn workspace_storage_with_defaults(
        data_base_dir: &Path,
        device: Arc<LocalDevice>,
        workspace_id: Option<&EntryID>,
    ) -> WorkspaceStorage {
        WorkspaceStorage::new(
            data_base_dir,
            device,
            workspace_id.cloned().unwrap_or_default(),
            FAILSAFE_PATTERN_FILTER.clone(),
            DEFAULT_CHUNK_VACUUM_THRESHOLD,
            DEFAULT_WORKSPACE_STORAGE_CACHE_SIZE,
        )
        .await
        .unwrap()
    }

    fn create_workspace_manifest(
        device: &LocalDevice,
        workspace_id: EntryID,
    ) -> LocalWorkspaceManifest {
        let author = device.device_id.clone();
        let timestamp = device.now();
        LocalWorkspaceManifest::new(author, timestamp, Some(workspace_id), false)
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
    #[test_log::test(tokio::test)]
    async fn test_basic_set_get_clear() {
        TestbedScope::run("minimal", |env| async move {
            let alice = env.local_device("alice@dev1".parse().unwrap());
            let aws =
                workspace_storage_with_defaults(&env.discriminant_dir, alice.clone(), None).await;

            let manifest = create_file_manifest(&alice);
            let manifest_id = manifest.base.id;
            let manifest = LocalFileOrFolderManifest::File(manifest);
            let gen_manifest = manifest.clone().into();

            // 1) No data
            assert_eq!(
                aws.get_manifest(manifest_id).await.unwrap_err(),
                FSError::LocalMiss(*manifest_id)
            );

            // 2) Set data
            aws.set_manifest(manifest_id, manifest.clone(), false, None)
                .await
                .unwrap();
            assert_eq!(aws.get_manifest(manifest_id).await.unwrap(), gen_manifest);

            // Make sure data are not only stored in cache
            clear_cache(&aws).await;
            assert_eq!(aws.get_manifest(manifest_id).await.unwrap(), gen_manifest);

            // 3) Clear data
            aws.clear_manifest(&manifest_id).await.unwrap();

            assert_eq!(
                aws.get_manifest(manifest_id).await.unwrap_err(),
                FSError::LocalMiss(*manifest_id)
            );

            assert_eq!(
                aws.clear_manifest(&manifest_id).await.unwrap_err(),
                FSError::LocalMiss(*manifest_id)
            );
        })
        .await
    }

    #[rstest]
    #[test_log::test(tokio::test)]
    async fn test_cache_set_get() {
        TestbedScope::run("minimal", |env| async move {
            let alice = env.local_device("alice@dev1".parse().unwrap());
            let aws =
                workspace_storage_with_defaults(&env.discriminant_dir, alice.clone(), None).await;

            let manifest = create_file_manifest(&alice);
            let manifest_id = manifest.base.id;
            let manifest = LocalFileOrFolderManifest::File(manifest);
            let gen_manifest = manifest.clone().into();

            // 1) Set data
            aws.set_manifest(manifest_id, manifest.clone(), true, None)
                .await
                .unwrap();
            assert_eq!(aws.get_manifest(manifest_id).await.unwrap(), gen_manifest);

            // Data should be set only in the cache
            clear_cache(&aws).await;
            assert_eq!(
                aws.get_manifest(manifest_id).await.unwrap_err(),
                FSError::LocalMiss(*manifest_id)
            );

            // Re-set data
            aws.set_manifest(manifest_id, manifest.clone(), true, None)
                .await
                .unwrap();

            // 2) Clear should work as expected
            aws.clear_manifest(&manifest_id).await.unwrap();
            assert_eq!(
                aws.get_manifest(manifest_id).await.unwrap_err(),
                FSError::LocalMiss(*manifest_id)
            );

            // Re-set data
            aws.set_manifest(manifest_id, manifest, true, None)
                .await
                .unwrap();

            // 3) Flush data
            aws.ensure_manifest_persistent(manifest_id).await.unwrap();
            assert_eq!(aws.get_manifest(manifest_id).await.unwrap(), gen_manifest);

            // Data should be persistent in real database
            clear_cache(&aws).await;
            assert_eq!(aws.get_manifest(manifest_id).await.unwrap(), gen_manifest);

            // 4) Idempotency
            aws.ensure_manifest_persistent(manifest_id).await.unwrap();
        })
        .await
    }

    #[rstest]
    #[case(false, false)]
    #[case(false, true)]
    #[case(true, false)]
    #[case(true, true)]
    #[test_log::test(tokio::test)]
    async fn test_chunk_clearing(#[case] cache_only: bool, #[case] clear_manifest: bool) {
        TestbedScope::run("minimal", |env| async move {
            let alice = env.local_device("alice@dev1".parse().unwrap());
            let aws =
                workspace_storage_with_defaults(&env.discriminant_dir, alice.clone(), None).await;

            let mut file_manifest = create_file_manifest(&alice);
            let data1 = b"abc";
            let chunk1 = Chunk::new(0, NonZeroU64::new(3).unwrap());
            let data2 = b"def";
            let chunk2 = Chunk::new(3, NonZeroU64::new(6).unwrap());
            file_manifest.blocks = vec![vec![chunk1.clone(), chunk2.clone()]];
            file_manifest.size = 6;
            let manifest_id = file_manifest.base.id;
            let manifest = LocalFileOrFolderManifest::File(file_manifest.clone());

            // Set chunks and manifest
            aws.set_chunk(chunk1.id, data1).await.unwrap();
            aws.set_chunk(chunk2.id, data2).await.unwrap();
            aws.set_manifest(manifest_id, manifest, false, None)
                .await
                .unwrap();

            // Set a new version of the manifest without the chunks
            let removed_ids = HashSet::from([
                ChunkOrBlockID::ChunkID(chunk1.id),
                ChunkOrBlockID::ChunkID(chunk2.id),
            ]);
            file_manifest.blocks.clear();
            let new_manifest = LocalFileOrFolderManifest::File(file_manifest.clone());

            aws.set_manifest(manifest_id, new_manifest, cache_only, Some(removed_ids))
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
                aws.clear_manifest(&manifest_id).await.unwrap();
            } else {
                aws.ensure_manifest_persistent(manifest_id).await.unwrap();
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
            aws.ensure_manifest_persistent(manifest_id).await.unwrap();
        })
        .await
    }

    #[rstest]
    #[test_log::test(tokio::test)]
    async fn test_cache_flushed_on_exit() {
        TestbedScope::run("minimal", |env| async move {
            let alice = env.local_device("alice@dev1".parse().unwrap());
            let aws =
                workspace_storage_with_defaults(&env.discriminant_dir, alice.clone(), None).await;

            let manifest = create_file_manifest(&alice);
            let manifest_id = manifest.base.id;
            let manifest = LocalFileOrFolderManifest::File(manifest);

            aws.set_manifest(manifest_id, manifest.clone(), true, None)
                .await
                .unwrap();

            aws.clear_memory_cache(true).await.unwrap();
            aws.close_connections().await;

            let aws2 = workspace_storage_with_defaults(
                &env.discriminant_dir,
                alice,
                Some(&aws.workspace_id),
            )
            .await;
            assert_eq!(
                aws2.get_manifest(manifest_id).await.unwrap(),
                manifest.into()
            );
        })
        .await
    }

    #[rstest]
    #[test_log::test(tokio::test)]
    async fn test_clear_cache() {
        TestbedScope::run("minimal", |env| async move {
            let alice = env.local_device("alice@dev1".parse().unwrap());
            let aws =
                workspace_storage_with_defaults(&env.discriminant_dir, alice.clone(), None).await;

            let manifest1 = create_file_manifest(&alice);
            let manifest1_id = manifest1.base.id;
            let manifest1 = LocalFileOrFolderManifest::File(manifest1);
            let gen_manifest1 = manifest1.clone().into();
            let manifest2 = create_file_manifest(&alice);
            let manifest2_id = manifest2.base.id;
            let manifest2 = LocalFileOrFolderManifest::File(manifest2);
            let gen_manifest2 = manifest2.clone().into();

            // Set manifest 1 and manifest 2, cache only
            aws.set_manifest(manifest1_id, manifest1, false, None)
                .await
                .unwrap();
            aws.set_manifest(manifest2_id, manifest2.clone(), true, None)
                .await
                .unwrap();

            // Clear without flushing
            aws.clear_memory_cache(false).await.unwrap();

            // Manifest 1 is present but manifest2 got lost
            assert_eq!(aws.get_manifest(manifest1_id).await.unwrap(), gen_manifest1);
            assert_eq!(
                aws.get_manifest(manifest2_id).await.unwrap_err(),
                FSError::LocalMiss(*manifest2_id)
            );

            // Set Manifest 2, cache only
            aws.set_manifest(manifest2_id, manifest2, true, None)
                .await
                .unwrap();

            // Clear with flushing
            aws.clear_memory_cache(true).await.unwrap();
            assert_eq!(aws.get_manifest(manifest2_id).await.unwrap(), gen_manifest2);
        })
        .await
    }

    #[rstest]
    #[test_log::test(tokio::test)]
    async fn test_serialize_non_empty_local_file_manifest() {
        TestbedScope::run("minimal", |env| async move {
            let alice = env.local_device("alice@dev1".parse().unwrap());
            let aws =
                workspace_storage_with_defaults(&env.discriminant_dir, alice.clone(), None).await;

            let mut file_manifest = create_file_manifest(&alice);
            let chunk1 = Chunk::new(0, NonZeroU64::try_from(7).unwrap())
                .evolve_as_block(b"0123456")
                .unwrap();
            let chunk2 = Chunk::new(7, NonZeroU64::try_from(8).unwrap());
            let chunk3 = Chunk::new(8, NonZeroU64::try_from(10).unwrap());
            let blocks = vec![vec![chunk1, chunk2], vec![chunk3]];
            file_manifest.size = 10;
            file_manifest.blocks = blocks;
            file_manifest.blocksize = Blocksize::try_from(8).unwrap();
            file_manifest.assert_integrity();
            let manifest_id = file_manifest.base.id;
            let manifest = LocalFileOrFolderManifest::File(file_manifest);
            let gen_manifest = manifest.clone().into();

            aws.set_manifest(manifest_id, manifest, false, None)
                .await
                .unwrap();
            assert_eq!(aws.get_manifest(manifest_id).await.unwrap(), gen_manifest);
        })
        .await;
    }

    #[rstest]
    #[test_log::test(tokio::test)]
    async fn test_realm_checkpoint() {
        TestbedScope::run("minimal", |env| async move {
            let alice = env.local_device("alice@dev1".parse().unwrap());
            let aws =
                workspace_storage_with_defaults(&env.discriminant_dir, alice.clone(), None).await;

            let mut manifest = create_file_manifest(&alice);
            let manifest_id = manifest.base.id;

            assert_eq!(aws.get_realm_checkpoint().await, 0);
            // Workspace storage starts with a speculative workspace manifest placeholder
            assert_eq!(
                aws.get_need_sync_entries().await.unwrap(),
                (HashSet::from([aws.workspace_id]), HashSet::new())
            );

            let mut workspace_manifest = create_workspace_manifest(&aws.device, aws.workspace_id);
            let base = workspace_manifest.to_remote(aws.device.device_id.clone(), aws.device.now());
            workspace_manifest.base = base;
            workspace_manifest.need_sync = false;
            aws.set_workspace_manifest(workspace_manifest)
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
                LocalFileOrFolderManifest::File(manifest.clone()),
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
                LocalFileOrFolderManifest::File(manifest),
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
        })
        .await
    }

    #[rstest]
    #[test_log::test(tokio::test)]
    async fn test_block_interface() {
        TestbedScope::run("minimal", |env| async move {
            let alice = env.local_device("alice@dev1".parse().unwrap());
            let aws = workspace_storage_with_defaults(&env.discriminant_dir, alice, None).await;

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
        })
        .await;
    }

    #[rstest]
    #[test_log::test(tokio::test)]
    async fn test_chunk_interface() {
        TestbedScope::run("minimal", |env| async move {
            let alice = env.local_device("alice@dev1".parse().unwrap());
            let aws = workspace_storage_with_defaults(&env.discriminant_dir, alice, None).await;

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
        })
        .await;
    }

    #[rstest]
    #[test_log::test(tokio::test)]
    async fn test_chunk_many() {
        TestbedScope::run("minimal", |env| async move {
            let alice = env.local_device("alice@dev1".parse().unwrap());
            let aws = workspace_storage_with_defaults(&env.discriminant_dir, alice, None).await;

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
        })
        .await;
    }

    #[rstest]
    #[test_log::test(tokio::test)]
    async fn test_file_descriptor() {
        TestbedScope::run("minimal", |env| async move {
            let alice = env.local_device("alice@dev1".parse().unwrap());
            let aws =
                workspace_storage_with_defaults(&env.discriminant_dir, alice.clone(), None).await;

            let manifest = create_file_manifest(&alice);
            let manifest_id = manifest.base.id;

            aws.set_manifest(
                manifest_id,
                LocalFileOrFolderManifest::File(manifest.clone()),
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
        })
        .await;
    }

    #[rstest]
    #[test_log::test(tokio::test)]
    async fn test_run_vacuum() {
        TestbedScope::run("minimal", |env| async move {
            let alice = env.local_device("alice@dev1".parse().unwrap());
            let aws = workspace_storage_with_defaults(&env.discriminant_dir, alice, None).await;
            aws.run_vacuum().await.unwrap();
        })
        .await;
    }

    #[rstest]
    #[test_log::test(tokio::test)]
    async fn test_garbage_collection() {
        TestbedScope::run("minimal", |env| async move {
            let alice = env.local_device("alice@dev1".parse().unwrap());
            let aws = WorkspaceStorage::new(
                &env.discriminant_dir,
                alice.clone(),
                EntryID::default(),
                FAILSAFE_PATTERN_FILTER.clone(),
                DEFAULT_CHUNK_VACUUM_THRESHOLD,
                // Here is the trick: We set the cache to contain at most 2 blocks
                *DEFAULT_BLOCK_SIZE * 2,
            )
            .await
            .unwrap();

            let block_size = NonZeroU64::try_from(*DEFAULT_BLOCK_SIZE).unwrap();
            let data = vec![0; *DEFAULT_BLOCK_SIZE as usize];
            let chunk1 = Chunk::new(0, block_size).evolve_as_block(&data).unwrap();
            let chunk2 = Chunk::new(0, block_size).evolve_as_block(&data).unwrap();
            let chunk3 = Chunk::new(0, block_size).evolve_as_block(&data).unwrap();

            // Store the first block
            assert_eq!(aws.block_storage.get_nb_blocks().await.unwrap(), 0);
            aws.set_clean_block(chunk1.access.unwrap().id, &data)
                .await
                .unwrap();
            assert_eq!(aws.block_storage.get_nb_blocks().await.unwrap(), 1);
            // Store the second block
            aws.set_clean_block(chunk2.access.unwrap().id, &data)
                .await
                .unwrap();
            assert_eq!(aws.block_storage.get_nb_blocks().await.unwrap(), 2);
            // Store the third block, the first one gets cleared
            aws.set_clean_block(chunk3.access.unwrap().id, &data)
                .await
                .unwrap();
            assert_eq!(aws.block_storage.get_nb_blocks().await.unwrap(), 2);
            // Force clear, everything gets cleared
            aws.block_storage.clear_all_blocks().await.unwrap();
            assert_eq!(aws.block_storage.get_nb_blocks().await.unwrap(), 0);
        })
        .await;
    }

    #[rstest]
    #[test_log::test(tokio::test)]
    async fn test_invalid_regex() {
        TestbedScope::run("minimal", |env| async move {
            use crate::storage::manifest_storage::prevent_sync_pattern::dsl::*;
            use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};

            let alice = env.local_device("alice@dev1".parse().unwrap());
            let aws =
                workspace_storage_with_defaults(&env.discriminant_dir, alice.clone(), None).await;

            let initial_prevent_sync_pattern = aws.get_prevent_sync_pattern();

            const INVALID_REGEX: &str = "[";
            let valid_regex = Regex::from_regex_str("ok").unwrap();

            // Ensure the entry is present.
            aws.set_prevent_sync_pattern(&valid_regex).await.unwrap();

            // Close the connection to avoid `OperationalError: database is locked`
            // error due to concurrency operations on the SQLite database
            aws.close_connections().await;

            // Corrupt the db with an invalid regex
            let db_relative_path =
                get_workspace_data_storage_db_relative_path(&alice, aws.workspace_id);
            let conn = LocalDatabase::from_path(
                &env.discriminant_dir,
                &db_relative_path,
                VacuumMode::default(),
            )
            .await
            .unwrap();
            conn.exec(|conn| {
                diesel::update(
                    prevent_sync_pattern.filter(_id.eq(0).and(pattern.ne(INVALID_REGEX))),
                )
                .set((pattern.eq(INVALID_REGEX), fully_applied.eq(false)))
                .execute(conn)
            })
            .await
            .unwrap();
            conn.close().await;

            // Now re-open an see what happen !
            let aws2 = workspace_storage_with_defaults(
                &env.discriminant_dir,
                alice.clone(),
                Some(&aws.workspace_id),
            )
            .await;
            assert_eq!(
                aws2.get_prevent_sync_pattern(),
                initial_prevent_sync_pattern
            );
        })
        .await;
    }

    #[rstest]
    #[test_log::test(tokio::test)]
    #[should_panic]
    async fn inserting_different_workspace_manifest() {
        TestbedScope::run("minimal", |env| async move {
            let alice = env.local_device("alice@dev1".parse().unwrap());
            let aws =
                workspace_storage_with_defaults(&env.discriminant_dir, alice.clone(), None).await;
            let workspace_manifest = create_workspace_manifest(&alice, EntryID::default());

            // Should panic because we insert a workspace manifest which id is different than `aws.workspace_id`
            let _ = aws.set_workspace_manifest(workspace_manifest).await;
        })
        .await
    }
}
