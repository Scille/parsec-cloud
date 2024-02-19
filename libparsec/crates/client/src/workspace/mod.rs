// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod fetch;
mod merge;
mod transactions;

use std::{
    ops::DerefMut,
    sync::{Arc, Mutex},
};

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_platform_storage::workspace::{WorkspaceCacheStorage, WorkspaceDataStorage};
use libparsec_types::prelude::*;

use crate::{certif::CertifOps, event_bus::EventBus, ClientConfig};
use transactions::RemoveEntryExpect;
pub use transactions::{
    EntryStat, FsOperationError as WorkspaceFsOperationError, InboundSyncOutcome, OpenedFile,
    WorkspaceSyncError,
};

pub struct WorkspaceOps {
    #[allow(unused)]
    config: Arc<ClientConfig>,
    #[allow(unused)]
    device: Arc<LocalDevice>,
    data_storage: WorkspaceDataStorage,
    cache_storage: WorkspaceCacheStorage,
    #[allow(unused)]
    cmds: Arc<AuthenticatedCmds>,
    #[allow(unused)]
    certificates_ops: Arc<CertifOps>,
    #[allow(unused)]
    event_bus: EventBus,
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
        // TODO: handle errors
        let data_storage =
            WorkspaceDataStorage::start(&config.data_base_dir, device.clone(), realm_id).await?;
        let cache_storage = WorkspaceCacheStorage::start(
            &config.data_base_dir,
            config.workspace_storage_cache_size.cache_size(),
            device.clone(),
            realm_id,
        )
        .await?;

        Ok(Self {
            config,
            device,
            data_storage,
            cache_storage,
            cmds,
            certificates_ops,
            event_bus,
            realm_id,
            workspace_entry: Mutex::new(workspace_entry),
        })
    }

    /// Stop the underlying storage (and flush whatever data is not yet on disk)
    ///
    /// Once stopped, it can still theoretically be used (i.e. `stop` doesn't
    /// consume `self`), but will do nothing but return stopped error.
    pub(crate) async fn stop(&self) -> anyhow::Result<()> {
        // TODO: is the storages teardown order important ?
        self.data_storage
            .stop()
            .await
            .context("Cannot stop data storage")?;
        self.cache_storage.stop().await;
        Ok(())
    }

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

    pub async fn get_need_inbound_sync(&self) -> anyhow::Result<Vec<VlobID>> {
        transactions::get_need_inbound_sync(self).await
    }

    /// Upload local changes to the server.
    ///
    /// This also requires to download and merge any remote changes. Hence the
    /// client is fully synchronized with the server once this function returns
    /// (unless a concurrent local change occured during the sync).
    pub async fn outbound_sync(&self, entry_id: VlobID) -> Result<(), WorkspaceSyncError> {
        transactions::outbound_sync(self, entry_id).await
    }

    pub async fn get_need_outbound_sync(&self) -> anyhow::Result<Vec<VlobID>> {
        transactions::get_need_outbound_sync(self).await
    }

    /*
     * Public interface
     */

    pub fn realm_id(&self) -> VlobID {
        self.realm_id
    }

    pub async fn stat_entry(&self, path: &FsPath) -> Result<EntryStat, WorkspaceFsOperationError> {
        transactions::stat_entry(self, path).await
    }

    pub async fn rename_entry(
        &self,
        path: &FsPath,
        new_name: EntryName,
        overwrite: bool,
    ) -> Result<(), WorkspaceFsOperationError> {
        transactions::rename_entry(self, path, new_name, overwrite).await
    }

    pub async fn create_folder(&self, path: &FsPath) -> Result<VlobID, WorkspaceFsOperationError> {
        transactions::create_folder(self, path).await
    }

    /// Create the folder and any missing parent (equivalent to `mkdir -p` in Unix).
    ///
    /// This is a high level helper, and hence is non-atomic. This typically means
    /// the operation can fail with a `WorkspaceFsOperationError::EntryNotFound` error
    /// if a concurrent operation removes a parent folder at the wrong time.
    pub async fn create_folder_all(
        &self,
        path: &FsPath,
    ) -> Result<VlobID, WorkspaceFsOperationError> {
        // Start by trying the most common case: the parent folder already exists
        let outcome = transactions::create_folder(self, path).await;
        if !matches!(outcome, Err(WorkspaceFsOperationError::EntryNotFound)) {
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
            let outcome = transactions::create_folder(self, &ancestor_path).await;
            let entry_id = match outcome {
                Ok(entry_id) => Result::<_, WorkspaceFsOperationError>::Ok(entry_id),
                Err(WorkspaceFsOperationError::EntryExists { entry_id }) => Ok(entry_id),
                Err(err) => return Err(err),
            }?;
            ancestor_path = match path_parts.next() {
                Some(part) => ancestor_path.join(part.to_owned()),
                // All done !
                None => return Ok(entry_id),
            };
        }
    }

    pub async fn create_file(&self, path: &FsPath) -> Result<VlobID, WorkspaceFsOperationError> {
        transactions::create_file(self, path).await
    }

    pub async fn remove_entry(&self, path: &FsPath) -> Result<(), WorkspaceFsOperationError> {
        transactions::remove_entry(self, path, RemoveEntryExpect::Anything).await
    }

    pub async fn remove_file(&self, path: &FsPath) -> Result<(), WorkspaceFsOperationError> {
        transactions::remove_entry(self, path, RemoveEntryExpect::File).await
    }

    pub async fn remove_folder(&self, path: &FsPath) -> Result<(), WorkspaceFsOperationError> {
        transactions::remove_entry(self, path, RemoveEntryExpect::EmptyFolder).await
    }

    pub async fn remove_folder_all(&self, path: &FsPath) -> Result<(), WorkspaceFsOperationError> {
        transactions::remove_entry(self, path, RemoveEntryExpect::Folder).await
    }

    pub async fn resize_file(
        &self,
        path: &FsPath,
        size: usize,
    ) -> Result<(), WorkspaceFsOperationError> {
        transactions::resize_file(self, path, size).await
    }

    pub async fn open_file(&self, path: &FsPath) -> Result<OpenedFile, WorkspaceFsOperationError> {
        transactions::open_file(self, path).await
    }
}

#[cfg(test)]
#[path = "../../tests/unit/workspace/mod.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
