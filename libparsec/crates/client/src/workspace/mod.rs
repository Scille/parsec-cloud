// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod addr;
mod merge;
mod store;
mod transactions;

use std::{
    collections::HashMap,
    ops::DerefMut,
    sync::{Arc, Mutex},
};

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_platform_async::lock::Mutex as AsyncMutex;
use libparsec_types::prelude::*;

use crate::{certif::CertificateOps, event_bus::EventBus, ClientConfig};
pub use addr::{WorkspaceDecryptPathAddrError, WorkspaceGeneratePathAddrError};
use store::WorkspaceStore;
use transactions::RemoveEntryExpect;
pub use transactions::{
    EntryStat, FileStat, FolderReader, FolderReaderStatEntryError, FolderReaderStatNextOutcome,
    InboundSyncOutcome, MoveEntryMode, OpenOptions, OutboundSyncOutcome, WorkspaceCreateFileError,
    WorkspaceCreateFolderError, WorkspaceFdCloseError, WorkspaceFdFlushError, WorkspaceFdReadError,
    WorkspaceFdResizeError, WorkspaceFdStatError, WorkspaceFdWriteError,
    WorkspaceGetNeedInboundSyncEntriesError, WorkspaceGetNeedOutboundSyncEntriesError,
    WorkspaceMoveEntryError, WorkspaceOpenFileError, WorkspaceOpenFolderReaderError,
    WorkspaceRemoveEntryError, WorkspaceStatEntryError, WorkspaceStatFolderChildrenError,
    WorkspaceSyncError, WorkspaceWatchEntryOneShotError,
};

use self::{store::FileUpdater, transactions::FdWriteStrategy};

#[derive(Debug)]
enum ReadMode {
    Allowed,
    Denied,
}

#[derive(Debug)]
enum WriteMode {
    Allowed,
    Denied,
}

#[derive(Debug)]
struct OpenedFileCursor {
    file_descriptor: FileDescriptor,
    read_mode: ReadMode,
    write_mode: WriteMode,
}

#[derive(Debug)]
struct OpenedFile {
    /// If no cursor is opened in write mode, then the updater may be `None`.
    updater: Option<FileUpdater>,
    manifest: Arc<LocalFileManifest>,
    /// The file can be opened multiple time, each one of them having it own cursor.
    cursors: Vec<OpenedFileCursor>,
    /// Track the number of bytes written since the last flush, this is used as
    /// a simple heuristic to determine when to do a chunk reshape & flush.
    ///
    /// When a file is opened for write, the most common case is to have it entirely
    /// overwritten. From our point of view, this correspond to multiple writes
    /// operations (typically 4ko each on Linux) in sequential order.
    /// Hence we trigger the reshape once we have written enough bytes to create
    /// a block (depending on manifest's blocksize).
    bytes_written_since_last_flush: u64,
    /// Chunks no longer used since the last flush. They should be removed from
    /// the database  on the next flush.
    removed_chunks: Vec<ChunkID>,
    /// Chunks created since the last flush. They currently live in memory and should
    /// be written in the database on the next flush.
    new_chunks: Vec<(ChunkID, Vec<u8>)>,
    /// In theory we could deduce from `removed_chunks`&`new_chunks` if the flush
    /// is needed, but (at least for now) we instead value the simplicity of setting
    /// a flag when a change occurred.
    flush_needed: bool,
    /// Track the fact any modification occurred on the file since it has been opened.
    /// This is useful on close to determine if we should broadcast a
    /// `WorkspaceOpsOutboundSyncNeeded` event.
    modified_since_opened: bool,
}

#[derive(Debug)]
struct OpenedFiles {
    /// Opening new file is not allowed during `WorkspaceOps::stop`
    new_open_allowed: bool,
    next_file_descriptor: FileDescriptor,
    // TODO: use rustc-hash instead of std hashmap ? (or even vec ?)
    // TODO: use optimistic locking ?
    file_descriptors: HashMap<FileDescriptor, VlobID>,
    opened_files: HashMap<VlobID, Arc<AsyncMutex<OpenedFile>>>,
}

pub struct WorkspaceExternalInfo {
    /// Workspace entry as stored in the local user manifest.
    pub entry: LocalUserManifestWorkspaceEntry,
    /// Total number of workspace the user has access to (currently only used to
    /// determine the drive letter on WinFSP).
    pub total_workspaces: usize,
    /// Arbitrary index for this workspace among the user's workspaces (currently
    /// only used to determine the drive letter on WinFSP).
    pub workspace_index: usize,
}

pub struct WorkspaceOps {
    #[allow(unused)]
    config: Arc<ClientConfig>,
    device: Arc<LocalDevice>,
    cmds: Arc<AuthenticatedCmds>,
    certificates_ops: Arc<CertificateOps>,
    event_bus: EventBus,
    store: WorkspaceStore,
    opened_files: Mutex<OpenedFiles>,
    realm_id: VlobID,
    /// This contains the workspaces info that can change by uploading new
    /// certificates, and hence can be updated at any time.
    workspace_external_info: Mutex<WorkspaceExternalInfo>,
    entry_watchers: Arc<Mutex<transactions::EntryWatchers>>,
}

impl std::panic::UnwindSafe for WorkspaceOps {}

impl std::fmt::Debug for WorkspaceOps {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("WorkspaceOps")
            .field("device", &self.device)
            .field("realm_id", &self.realm_id)
            .finish_non_exhaustive()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WorkspaceOpsError {
    #[error("Unknown workspace `{0}`")]
    UnknownWorkspace(VlobID),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

// For readability, we define the public interface here and let the actual
// implementation in separated submodules
impl WorkspaceOps {
    /*
     * Crate-only interface (used by client, opses and monitors)
     */

    pub(crate) async fn start(
        config: Arc<ClientConfig>,
        device: Arc<LocalDevice>,
        cmds: Arc<AuthenticatedCmds>,
        certificates_ops: Arc<CertificateOps>,
        event_bus: EventBus,
        realm_id: VlobID,
        workspace_external_info: WorkspaceExternalInfo,
    ) -> Result<Self, anyhow::Error> {
        // Sanity check (note in practice `workspace_entry.id` is never used)
        assert_eq!(workspace_external_info.entry.id, realm_id);

        let store = WorkspaceStore::start(
            &config.data_base_dir,
            device.clone(),
            cmds.clone(),
            certificates_ops.clone(),
            config.workspace_storage_cache_size.cache_size(),
            realm_id,
            &config.prevent_sync_pattern,
        )
        .await?;

        Ok(Self {
            config,
            device,
            store,
            cmds,
            certificates_ops,
            event_bus,
            realm_id,
            workspace_external_info: Mutex::new(workspace_external_info),
            opened_files: Mutex::new(OpenedFiles {
                new_open_allowed: true,
                // Avoid using 0 as file descriptor, as it is error-prone
                next_file_descriptor: FileDescriptor(1),
                file_descriptors: HashMap::new(),
                opened_files: HashMap::new(),
            }),
            entry_watchers: Default::default(),
        })
    }

    /// Stop the underlying storage (and flush whatever data is not yet on disk)
    ///
    /// Once stopped, it can still theoretically be used (i.e. `stop` doesn't
    /// consume `self`), but will do nothing but return stopped error.
    pub(crate) async fn stop(&self) -> anyhow::Result<()> {
        // Forbid new file open to avoid concurrent open while we are closing
        // the already opened ones.
        {
            let mut opened_files = self.opened_files.lock().expect("Mutex is poisoned");
            opened_files.new_open_allowed = false;
        }

        // If we are already closed, there is not file descriptors to close so this
        // operation is a noop.
        let close_outcome = transactions::close_all_fds(self)
            .await
            .context("cannot close opened file");

        #[cfg(test)]
        libparsec_tests_fixtures::moment_define_inject_point(
            libparsec_tests_fixtures::Moment::WorkspaceOpsStopAllFdsClosed,
        )
        .await;

        // Continue even if the close has failed: no matter what we still want to
        // close the store.

        self.store
            .stop()
            .await
            .context("cannot stop data storage")?;

        close_outcome
    }

    /// Workspace entry contains information related to the workspace that are
    /// represented (and updated) as certificate.
    ///
    /// However the workspace ops doesn't bother to interrogate the certificate
    /// ops whenever it needs those information (this is partly because a newly
    /// created workspace's informations are not yet available as certificate).
    ///
    /// Hence this workspace entry that bundles all the informations, but which
    /// needs to be update whenever the corresponding certificates are updated.
    pub(crate) fn update_workspace_external_info(
        &self,
        updater: impl FnOnce(&mut WorkspaceExternalInfo),
    ) {
        let mut guard = self
            .workspace_external_info
            .lock()
            .expect("Mutex is poisoned");
        updater(guard.deref_mut());
        assert_eq!(guard.entry.id, self.realm_id); // Sanity check
        assert!(guard.workspace_index < guard.total_workspaces); // Sanity check
    }

    /// Download and merge remote changes from the server.
    ///
    /// If the client contains local changes, an outbound sync is still needed to
    /// have the client fully synchronized with the server.
    pub async fn inbound_sync(
        &self,
        entry_id: VlobID,
    ) -> Result<InboundSyncOutcome, WorkspaceSyncError> {
        transactions::inbound_sync(self, entry_id).await
    }

    /// Query the server for changes in the workspace since the last checkpoint
    /// we know about.
    pub async fn refresh_realm_checkpoint(&self) -> Result<(), WorkspaceSyncError> {
        transactions::refresh_realm_checkpoint(self).await
    }

    pub async fn get_need_inbound_sync(
        &self,
        limit: u32,
    ) -> Result<Vec<VlobID>, WorkspaceGetNeedInboundSyncEntriesError> {
        transactions::get_need_inbound_sync(self, limit).await
    }

    /// Upload local changes to the server.
    ///
    /// This also requires to download and merge any remote changes. Hence the
    /// client is fully synchronized with the server once this function returns
    /// (unless a concurrent local change occurred during the sync).
    pub async fn outbound_sync(
        &self,
        entry_id: VlobID,
    ) -> Result<OutboundSyncOutcome, WorkspaceSyncError> {
        transactions::outbound_sync(self, entry_id).await
    }

    pub async fn get_need_outbound_sync(
        &self,
        limit: u32,
    ) -> Result<Vec<VlobID>, WorkspaceGetNeedOutboundSyncEntriesError> {
        transactions::get_need_outbound_sync(self, limit).await
    }

    /*
     * Public interface
     */

    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    pub fn realm_id(&self) -> VlobID {
        self.realm_id
    }

    pub fn get_current_name_and_self_role(&self) -> (EntryName, RealmRole) {
        let guard = self
            .workspace_external_info
            .lock()
            .expect("Mutex is poisoned");

        (guard.entry.name.clone(), guard.entry.role)
    }

    /// Only needed for WinFSP
    pub fn get_workspace_index_and_total_workspaces(&self) -> (usize, usize) {
        let guard = self
            .workspace_external_info
            .lock()
            .expect("Mutex is poisoned");

        (guard.workspace_index, guard.total_workspaces)
    }

    pub async fn stat_entry(&self, path: &FsPath) -> Result<EntryStat, WorkspaceStatEntryError> {
        transactions::stat_entry(self, path).await
    }

    pub async fn stat_entry_by_id(
        &self,
        entry_id: VlobID,
    ) -> Result<EntryStat, WorkspaceStatEntryError> {
        transactions::stat_entry_by_id(self, entry_id, None).await
    }

    pub(crate) async fn stat_entry_by_id_with_known_confinement_point(
        &self,
        entry_id: VlobID,
        precomputed_confinement_point: store::PathConfinementPoint,
    ) -> Result<EntryStat, WorkspaceStatEntryError> {
        transactions::stat_entry_by_id(self, entry_id, Some(precomputed_confinement_point)).await
    }

    pub async fn stat_entry_by_id_ignore_confinement_point(
        &self,
        entry_id: VlobID,
    ) -> Result<EntryStat, WorkspaceStatEntryError> {
        transactions::stat_entry_by_id(
            self,
            entry_id,
            Some(store::PathConfinementPoint::NotConfined),
        )
        .await
    }

    pub async fn watch_entry_oneshot(
        &self,
        path: &FsPath,
    ) -> Result<VlobID, WorkspaceWatchEntryOneShotError> {
        transactions::watch_entry_oneshot(self, path).await
    }

    pub async fn open_folder_reader(
        &self,
        path: &FsPath,
    ) -> Result<FolderReader, WorkspaceOpenFolderReaderError> {
        transactions::open_folder_reader(self, path).await
    }

    pub async fn open_folder_reader_by_id(
        &self,
        entry_id: VlobID,
    ) -> Result<FolderReader, WorkspaceOpenFolderReaderError> {
        transactions::open_folder_reader_by_id(self, entry_id).await
    }

    /// Note children are listed in arbitrary order, and there is no '.' and '..'  special entries.
    pub async fn stat_folder_children(
        &self,
        path: &FsPath,
    ) -> Result<Vec<(EntryName, EntryStat)>, WorkspaceStatFolderChildrenError> {
        transactions::stat_folder_children(self, path).await
    }

    pub async fn stat_folder_children_by_id(
        &self,
        entry_id: VlobID,
    ) -> Result<Vec<(EntryName, EntryStat)>, WorkspaceStatFolderChildrenError> {
        transactions::stat_folder_children_by_id(self, entry_id).await
    }

    pub async fn move_entry(
        &self,
        src: FsPath,
        dst: FsPath,
        mode: MoveEntryMode,
    ) -> Result<(), WorkspaceMoveEntryError> {
        transactions::move_entry(self, src, dst, mode).await
    }

    pub async fn rename_entry_by_id(
        &self,
        src_parent_id: VlobID,
        src_name: EntryName,
        dst_name: EntryName,
        mode: MoveEntryMode,
    ) -> Result<(), WorkspaceMoveEntryError> {
        transactions::rename_entry_by_id(self, src_parent_id, src_name, dst_name, mode).await
    }

    // // TODO
    // pub async fn copy_entry(
    //     &self,
    //     src: FsPath,
    //     dst: FsPath,
    //     overwrite: bool,
    // ) -> Result<(), WorkspaceCopyEntryError> {
    //     todo!()
    // }

    pub async fn create_folder(&self, path: FsPath) -> Result<VlobID, WorkspaceCreateFolderError> {
        transactions::create_folder(self, path).await
    }

    /// Create the folder and any missing parent (equivalent to `mkdir -p` in Unix).
    ///
    /// This is a high level helper, and hence is non-atomic. This typically means
    /// the operation can fail with a `WorkspaceFsOperationError::EntryNotFound` error
    /// if a concurrent operation removes a parent folder at the wrong time.
    pub async fn create_folder_all(
        &self,
        path: FsPath,
    ) -> Result<VlobID, WorkspaceCreateFolderError> {
        transactions::create_folder_all(self, path).await
    }

    pub async fn create_file(&self, path: FsPath) -> Result<VlobID, WorkspaceCreateFileError> {
        transactions::create_file(self, path).await
    }

    pub async fn remove_entry(&self, path: FsPath) -> Result<(), WorkspaceRemoveEntryError> {
        transactions::remove_entry(self, path, RemoveEntryExpect::Anything).await
    }

    pub async fn remove_file(&self, path: FsPath) -> Result<(), WorkspaceRemoveEntryError> {
        transactions::remove_entry(self, path, RemoveEntryExpect::File).await
    }

    pub async fn remove_folder(&self, path: FsPath) -> Result<(), WorkspaceRemoveEntryError> {
        transactions::remove_entry(self, path, RemoveEntryExpect::EmptyFolder).await
    }

    pub async fn remove_folder_all(&self, path: FsPath) -> Result<(), WorkspaceRemoveEntryError> {
        transactions::remove_entry(self, path, RemoveEntryExpect::Folder).await
    }

    pub async fn open_file(
        &self,
        path: FsPath,
        options: OpenOptions,
    ) -> Result<FileDescriptor, WorkspaceOpenFileError> {
        transactions::open_file(self, path, options)
            .await
            .map(|(fd, _)| fd)
    }

    pub async fn open_file_by_id(
        &self,
        entry_id: VlobID,
        options: OpenOptions,
    ) -> Result<FileDescriptor, WorkspaceOpenFileError> {
        transactions::open_file_by_id(self, entry_id, options)
            .await
            .map(|(fd, _)| fd)
    }

    pub async fn open_file_and_get_id(
        &self,
        path: FsPath,
        options: OpenOptions,
    ) -> Result<(FileDescriptor, VlobID), WorkspaceOpenFileError> {
        transactions::open_file(self, path, options).await
    }

    pub async fn fd_close(&self, fd: FileDescriptor) -> Result<(), WorkspaceFdCloseError> {
        transactions::fd_close(self, fd).await
    }

    pub async fn fd_stat(&self, fd: FileDescriptor) -> Result<FileStat, WorkspaceFdStatError> {
        transactions::fd_stat(self, fd).await
    }

    pub async fn fd_flush(&self, fd: FileDescriptor) -> Result<(), WorkspaceFdFlushError> {
        transactions::fd_flush(self, fd).await
    }

    pub async fn fd_read(
        &self,
        fd: FileDescriptor,
        offset: u64,
        size: u64,
        buf: &mut impl std::io::Write,
    ) -> Result<u64, WorkspaceFdReadError> {
        transactions::fd_read(self, fd, offset, size, buf).await
    }

    pub async fn fd_resize(
        &self,
        fd: FileDescriptor,
        length: u64,
        truncate_only: bool,
    ) -> Result<(), WorkspaceFdResizeError> {
        transactions::fd_resize(self, fd, length, truncate_only).await
    }

    pub async fn fd_write(
        &self,
        fd: FileDescriptor,
        offset: u64,
        data: &[u8],
    ) -> Result<u64, WorkspaceFdWriteError> {
        transactions::fd_write(self, fd, data, FdWriteStrategy::Normal { offset }).await
    }

    // TODO: add `rename_entry` `move_entry` `create_folder` `create_file` `remove_entry` & `open_file`
    //       versions that work with parent entry_id instead of path (to match more closely how FUSE works)

    // TODO: a `fd_write_buff()` taking a `Vec<u8>` instead of `&[u8]` would be useful
    //       to avoid extra copy in FUSE

    // TODO: a `fd_copy_file_range` would be useful to avoid extra copy in FUSE

    // TODO: a `fd_allocate` would be useful to set the blocksize according to the
    //       expected final file size

    pub async fn fd_write_constrained_io(
        &self,
        fd: FileDescriptor,
        offset: u64,
        data: &[u8],
    ) -> Result<u64, WorkspaceFdWriteError> {
        transactions::fd_write(self, fd, data, FdWriteStrategy::ConstrainedIO { offset }).await
    }

    pub async fn fd_write_start_eof(
        &self,
        fd: FileDescriptor,
        data: &[u8],
    ) -> Result<u64, WorkspaceFdWriteError> {
        transactions::fd_write(self, fd, data, FdWriteStrategy::StartEOF).await
    }

    pub async fn generate_path_addr(
        &self,
        path: &FsPath,
    ) -> Result<ParsecWorkspacePathAddr, WorkspaceGeneratePathAddrError> {
        addr::generate_path_addr(self, path).await
    }

    pub async fn decrypt_path_addr(
        &self,
        link: &ParsecWorkspacePathAddr,
    ) -> Result<FsPath, WorkspaceDecryptPathAddrError> {
        addr::decrypt_path_addr(self, link).await
    }
}

#[cfg(test)]
#[path = "../../tests/unit/workspace/mod.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
