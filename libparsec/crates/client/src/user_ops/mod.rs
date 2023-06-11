// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod create;
mod merge;
mod message;
// mod reencryption;
mod share;
mod sync;

pub use message::*;
// pub use reencryption::*;
pub use share::*;
pub use sync::*;

use std::{path::PathBuf, sync::Arc};

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_platform_async::Mutex as AsyncMutex;
use libparsec_platform_storage::user::UserStorage;
use libparsec_types::prelude::*;

use crate::{certificates_ops::CertificatesOps, event_bus::EventBus};

#[derive(Debug)]
pub struct UserOps {
    data_base_dir: PathBuf, // TODO: use Arc ?
    device: Arc<LocalDevice>,
    storage: UserStorage,
    cmds: Arc<AuthenticatedCmds>,
    certificates_ops: Arc<CertificatesOps>,
    event_bus: EventBus,
    // Message processing is done in-order, hence it is pointless to do
    // it concurrently
    process_messages_lock: AsyncMutex<()>,
}

#[derive(Debug, thiserror::Error)]
pub enum UserOpsError {
    #[error("Unknown workspace `{0}`")]
    UnknownWorkspace(EntryID),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct ReencryptionJob {}

impl UserOps {
    pub async fn start(
        data_base_dir: PathBuf,
        device: Arc<LocalDevice>,
        cmds: Arc<AuthenticatedCmds>,
        certificates_ops: Arc<CertificatesOps>,
        event_bus: EventBus,
        // prevent_sync_pattern,
        // preferred_language,
        // workspace_storage_cache_size,
    ) -> Result<Self, anyhow::Error> {
        // TODO: handle errors
        let storage = UserStorage::start(&data_base_dir, device.clone()).await?;
        Ok(Self {
            data_base_dir,
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

    // For readability, we define the public interface here and let the actual
    // implementation in separated submodules

    pub fn list_workspaces(&self) -> Vec<(EntryID, EntryName)> {
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

    pub async fn workspace_create(&self, name: EntryName) -> Result<EntryID, anyhow::Error> {
        create::workspace_create(self, name).await
    }

    pub async fn workspace_rename(
        &self,
        workspace_id: EntryID,
        new_name: EntryName,
    ) -> Result<(), UserOpsError> {
        create::workspace_rename(self, workspace_id, new_name).await
    }

    pub async fn workspace_share(
        &self,
        workspace_id: EntryID,
        recipient: &UserID,
        role: Option<RealmRole>,
    ) -> Result<(), UserOpsWorkspaceShareError> {
        share::workspace_share(self, workspace_id, recipient, role).await
    }

    pub async fn workspace_start_reencryption(
        &self,
        _workspace_id: &EntryID,
    ) -> Result<ReencryptionJob, anyhow::Error> {
        todo!()
    }

    pub async fn workspace_continue_reencryption(
        &self,
        _workspace_id: &EntryID,
    ) -> Result<ReencryptionJob, anyhow::Error> {
        todo!()
    }
}
