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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

    pub async fn create_folder_all(
        &self,
        _path: &FsPath,
    ) -> Result<VlobID, WorkspaceFsOperationError> {
        // TODO: this is high level (non-atomic) stuff: should implement it here
        todo!()
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
