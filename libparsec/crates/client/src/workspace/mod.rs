// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod fetch;
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

use crate::{certif::CertifOps, event_bus::EventBus, ClientConfig};
use store::WorkspaceStore;
use transactions::RemoveEntryExpect;
pub use transactions::{
    EntryStat, InboundSyncOutcome, OpenOptions, OutboundSyncOutcome, WorkspaceCreateFileError,
    WorkspaceCreateFolderError, WorkspaceFdCloseError, WorkspaceFdFlushError, WorkspaceFdReadError,
    WorkspaceFdResizeError, WorkspaceFdWriteError, WorkspaceGetNeedInboundSyncEntriesError,
    WorkspaceGetNeedOutboundSyncEntriesError, WorkspaceOpenFileError, WorkspaceRemoveEntryError,
    WorkspaceRenameEntryError, WorkspaceStatEntryError, WorkspaceSyncError,
};

use self::store::FileUpdater;

enum ReadMode {
    Allowed,
    Denied,
}

enum WriteMode {
    Allowed,
    Denied,
}

struct OpenedFileCursor {
    file_descriptor: FileDescriptor,
    read_mode: ReadMode,
    write_mode: WriteMode,
}

struct OpenedFile {
    updater: FileUpdater,
    manifest: Arc<LocalFileManifest>,
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
    removed_chunks: Vec<ChunkID>,
    new_chunks: Vec<(ChunkID, Vec<u8>)>,
    flush_needed: bool,
}

struct OpenedFiles {
    next_file_descriptor: FileDescriptor,
    // TODO: use rustc-hash instead of std hashmap ? (or even vec ?)
    // TODO: use optimistic locking ?
    file_descriptors: HashMap<FileDescriptor, VlobID>,
    opened_files: HashMap<VlobID, Arc<AsyncMutex<OpenedFile>>>,
}

pub struct WorkspaceOps {
    #[allow(unused)]
    config: Arc<ClientConfig>,
    device: Arc<LocalDevice>,
    cmds: Arc<AuthenticatedCmds>,
    certificates_ops: Arc<CertifOps>,
    event_bus: EventBus,
    store: WorkspaceStore,
    opened_files: Mutex<OpenedFiles>,
    realm_id: VlobID,
    /// Workspace entry as stored in the local user manifest.
    /// This contains the workspaces info that can change by uploading new
    /// certificates, and hence can be updated at any time.
    workspace_entry: Mutex<LocalUserManifestWorkspaceEntry>,
}

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
        certificates_ops: Arc<CertifOps>,
        event_bus: EventBus,
        realm_id: VlobID,
        workspace_entry: LocalUserManifestWorkspaceEntry,
    ) -> Result<Self, anyhow::Error> {
        // Sanity check (note in practice `workspace_entry.id` is never used)
        assert_eq!(workspace_entry.id, realm_id);

        let store = WorkspaceStore::start(
            &config.data_base_dir,
            device.clone(),
            cmds.clone(),
            certificates_ops.clone(),
            config.workspace_storage_cache_size.cache_size(),
            realm_id,
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
            workspace_entry: Mutex::new(workspace_entry),
            opened_files: Mutex::new(OpenedFiles {
                // Avoid using 0 as file descriptor as it is error-prone
                next_file_descriptor: FileDescriptor(1),
                file_descriptors: HashMap::new(),
                opened_files: HashMap::new(),
            }),
        })
    }

    /// Stop the underlying storage (and flush whatever data is not yet on disk)
    ///
    /// Once stopped, it can still theoretically be used (i.e. `stop` doesn't
    /// consume `self`), but will do nothing but return stopped error.
    pub(crate) async fn stop(&self) -> anyhow::Result<()> {
        // If we are already closed, there is not file descriptors to close so this
        // operation is a noop.
        let close_outcome = transactions::close_all_fds(self)
            .await
            .context("cannot close opened file");

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
    pub(crate) fn update_workspace_entry(
        &self,
        updater: impl FnOnce(&mut LocalUserManifestWorkspaceEntry),
    ) {
        let mut guard = self.workspace_entry.lock().expect("Mutex is poisoned");
        updater(guard.deref_mut());
        assert_eq!(guard.id, self.realm_id); // Sanity check
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
    /// (unless a concurrent local change occured during the sync).
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

    pub fn realm_id(&self) -> VlobID {
        self.realm_id
    }

    pub async fn stat_entry(&self, path: &FsPath) -> Result<EntryStat, WorkspaceStatEntryError> {
        transactions::stat_entry(self, path).await
    }

    pub async fn rename_entry(
        &self,
        path: FsPath,
        new_name: EntryName,
        overwrite: bool,
    ) -> Result<(), WorkspaceRenameEntryError> {
        transactions::rename_entry(self, path, new_name, overwrite).await
    }

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
        // Start by trying the most common case: the parent folder already exists
        let outcome = transactions::create_folder(self, path.clone()).await;
        if !matches!(outcome, Err(WorkspaceCreateFolderError::ParentNotFound)) {
            return outcome;
        }

        // Some parents are missing, recursively try to create them all.
        // Note it would probably be more efficient to do that in reverse order
        // (given most of the time only the last part of the path is missing)
        // but it is just simpler to do it this way.

        let mut path_parts = path.parts().iter();
        // If `path` is root, it has already been handled in `transactions::create_folder`
        let in_root_entry_name = path_parts.next().expect("path cannot be root");
        let mut ancestor_path = FsPath::from_parts(vec![in_root_entry_name.to_owned()]);
        loop {
            let outcome = transactions::create_folder(self, ancestor_path.clone()).await;
            let entry_id = match outcome {
                Ok(entry_id) => Result::<_, WorkspaceCreateFolderError>::Ok(entry_id),
                Err(WorkspaceCreateFolderError::EntryExists { entry_id }) => Ok(entry_id),
                Err(err) => return Err(err),
            }?;
            ancestor_path = match path_parts.next() {
                Some(part) => ancestor_path.join(part.to_owned()),
                // All done !
                None => return Ok(entry_id),
            };
        }
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
        transactions::open_file(self, path, options).await
    }

    pub async fn fd_close(&self, fd: FileDescriptor) -> Result<(), WorkspaceFdCloseError> {
        transactions::fd_close(self, fd).await
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
        transactions::fd_write(self, fd, offset, data, false).await
    }

    pub async fn fd_write_with_constrained_io(
        &self,
        fd: FileDescriptor,
        offset: u64,
        data: &[u8],
    ) -> Result<u64, WorkspaceFdWriteError> {
        transactions::fd_write(self, fd, offset, data, true).await
    }
}

#[cfg(test)]
#[path = "../../tests/unit/workspace/mod.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
