// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod create;
mod error;
mod merge;
mod message;
// mod reencryption;
mod share;
mod sync;

pub use create::*;
pub use error::*;
pub use message::*;
// pub use reencryption::*;
pub use share::*;
pub use sync::*;

use std::sync::Arc;

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_platform_async::lock::Mutex as AsyncMutex;
use libparsec_platform_storage::user::UserStorage;
use libparsec_types::prelude::*;

use crate::{certificates_ops::CertificatesOps, event_bus::EventBus, ClientConfig};

#[derive(Debug)]
pub struct UserOps {
    config: Arc<ClientConfig>,
    device: Arc<LocalDevice>,
    storage: UserStorage,
    cmds: Arc<AuthenticatedCmds>,
    certificates_ops: Arc<CertificatesOps>,
    event_bus: EventBus,
    // Message processing is done in-order, hence it is pointless to do
    // it concurrently
    process_messages_lock: AsyncMutex<()>,
}

#[derive(Debug)]
pub struct ReencryptionJob {}

// For readability, we define the public interface here and let the actual
// implementation in separated submodules
impl UserOps {
    /*
     * Crate-only interface (used by client, opses and monitors)
     */

    pub(crate) async fn start(
        config: Arc<ClientConfig>,
        device: Arc<LocalDevice>,
        cmds: Arc<AuthenticatedCmds>,
        certificates_ops: Arc<CertificatesOps>,
        event_bus: EventBus,
    ) -> Result<Self, anyhow::Error> {
        // TODO: handle errors
        let storage = UserStorage::start(&config.data_base_dir, device.clone()).await?;
        Ok(Self {
            config,
            device,
            storage,
            cmds,
            certificates_ops,
            event_bus,
            process_messages_lock: AsyncMutex::new(()),
        })
    }

    pub(crate) async fn stop(&self) {
        self.storage.stop().await;
    }

    /// Low-level access, should be only needed for tests
    #[cfg(test)]
    pub(crate) fn test_get_user_manifest(&self) -> Arc<LocalUserManifest> {
        self.storage.get_user_manifest()
    }

    #[allow(unused)]
    pub(crate) async fn process_last_messages(
        &self,
        latest_known_index: Option<IndexInt>,
    ) -> Result<(), ProcessLastMessagesError> {
        message::process_last_messages(self, latest_known_index).await
    }

    #[allow(unused)]
    pub(crate) async fn sync(&self) -> Result<(), SyncError> {
        sync::sync(self).await
    }

    /*
     * Public interface
     */

    pub fn list_workspaces(&self) -> Vec<(VlobID, EntryName)> {
        let user_manifest = self.storage.get_user_manifest();
        user_manifest
            .workspaces
            .iter()
            .map(|entry| (entry.id, entry.name.clone()))
            .collect()
    }

    pub async fn create_workspace(&self, name: EntryName) -> Result<VlobID, anyhow::Error> {
        create::create_workspace(self, name).await
    }

    pub async fn rename_workspace(
        &self,
        realm_id: VlobID,
        new_name: EntryName,
    ) -> Result<(), RenameWorkspaceError> {
        create::rename_workspace(self, realm_id, new_name).await
    }

    pub async fn share_workspace(
        &self,
        realm_id: VlobID,
        recipient: &UserID,
        role: Option<RealmRole>,
    ) -> Result<(), ShareWorkspaceError> {
        share::share_workspace(self, realm_id, recipient, role).await
    }

    pub async fn workspace_start_reencryption(
        &self,
        _realm_id: &VlobID,
    ) -> Result<ReencryptionJob, anyhow::Error> {
        todo!()
    }

    pub async fn workspace_continue_reencryption(
        &self,
        _realm_id: &VlobID,
    ) -> Result<ReencryptionJob, anyhow::Error> {
        todo!()
    }
}

#[cfg(test)]
#[path = "../../tests/unit/user_ops/mod.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
