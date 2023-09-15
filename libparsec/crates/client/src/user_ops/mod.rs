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

impl UserOps {
    pub async fn start(
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

    pub async fn stop(&self) {
        self.storage.stop().await;
    }

    /// Low-level access, should be only needed for tests
    pub fn test_get_user_manifest(&self) -> Arc<LocalUserManifest> {
        self.storage.get_user_manifest()
    }

    // For readability, we define the public interface here and let the actual
    // implementation in separated submodules

    pub fn list_workspaces(&self) -> Vec<(VlobID, EntryName)> {
        let user_manifest = self.storage.get_user_manifest();
        user_manifest
            .workspaces
            .iter()
            .map(|entry| (entry.id, entry.name.clone()))
            .collect()
    }

    pub async fn process_last_messages(
        &self,
        latest_known_index: Option<IndexInt>,
    ) -> Result<(), ProcessLastMessagesError> {
        message::process_last_messages(self, latest_known_index).await
    }

    pub async fn sync(&self) -> Result<(), SyncError> {
        sync::sync(self).await
    }

    pub async fn workspace_create(&self, name: EntryName) -> Result<VlobID, anyhow::Error> {
        create::workspace_create(self, name).await
    }

    pub async fn workspace_rename(
        &self,
        realm_id: VlobID,
        new_name: EntryName,
    ) -> Result<(), WorkspaceRenameError> {
        create::workspace_rename(self, realm_id, new_name).await
    }

    pub async fn workspace_share(
        &self,
        realm_id: VlobID,
        recipient: &UserID,
        role: Option<RealmRole>,
    ) -> Result<(), WorkspaceShareError> {
        share::workspace_share(self, realm_id, recipient, role).await
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
