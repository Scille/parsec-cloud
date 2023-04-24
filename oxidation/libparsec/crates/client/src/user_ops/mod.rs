// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{path::PathBuf, sync::Arc};

use libparsec_client_connection::AuthenticatedCmds;
use libparsec_platform_async::Mutex as AsyncMutex;
use libparsec_platform_storage2::{workspace_storage_non_speculative_init, UserStorage};
use libparsec_types::prelude::*;

use crate::event_bus::EventBus;

pub type DynError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, thiserror::Error)]
pub enum UserOpsError {
    #[error("Unknown workspace `{0}`")]
    UnknownWorkspace(EntryID),
    #[error("Internal error: {0}")]
    Internal(DynError),
}

#[derive(Debug, thiserror::Error)]
pub enum UserOpsWorkspaceShareError {
    #[error("Unknown workspace `{0}`")]
    UnknownWorkspace(EntryID),
    #[error("Cannot share with oneself")]
    ShareToSelf,
    #[error("Internal error: {0}")]
    Internal(DynError),
}

#[derive(Debug)]
pub struct UserOps {
    data_base_dir: PathBuf, // TODO: use Arc ?
    device: Arc<LocalDevice>,
    storage: UserStorage,
    update_user_manifest_lock: AsyncMutex<()>,
}

#[derive(Debug)]
pub struct ReencryptionJob {}

impl UserOps {
    pub async fn start(
        data_base_dir: PathBuf,
        device: Arc<LocalDevice>,
        _cmds: Arc<AuthenticatedCmds>,
        // remote_device_manager,
        _event_bus: Arc<EventBus>,
        // prevent_sync_pattern,
        // preferred_language,
        // workspace_storage_cache_size,
    ) -> Result<Self, DynError> {
        // TODO: handle errors
        let storage = UserStorage::start(&data_base_dir, device.clone()).await?;
        Ok(Self {
            data_base_dir,
            device,
            storage,
            update_user_manifest_lock: AsyncMutex::new(()),
        })
    }
    pub async fn stop(&self) {
        self.storage.stop().await;
    }

    pub async fn workspace_create(&self, name: EntryName) -> Result<EntryID, UserOpsError> {
        let guard = self.update_user_manifest_lock.lock().await;

        let timestamp = self.device.time_provider.now();
        let workspace_entry = WorkspaceEntry::generate(name, timestamp);
        let workspace_id = workspace_entry.id;
        let user_manifest = self.storage.get_user_manifest();
        let user_manifest = Arc::new(
            (*user_manifest)
                .clone()
                .evolve_workspaces_and_mark_updated(workspace_entry, timestamp),
        );
        // Given *we* are the creator of the workspace, our placeholder is
        // the only non-speculative one.
        //
        // Note the save order is important given there is no atomicity
        // between saving the non-speculative workspace manifest placeholder
        // and the save of the user manifest containing the workspace entry.
        // Indeed, if we would save the user manifest first and a crash
        // occurred before saving the placeholder, we would end-up in the same
        // situation as if the workspace has been created by someone else
        // (i.e. a workspace entry but no local data about this workspace)
        // so we would fallback to a local speculative workspace manifest.
        // However a speculative manifest means the workspace have been
        // created by somebody else, and hence we shouldn't try to create
        // it corresponding realm in the backend !
        workspace_storage_non_speculative_init(&self.data_base_dir, &self.device, workspace_id)
            .await
            .map_err(UserOpsError::Internal)?;
        self.storage
            .set_user_manifest(user_manifest)
            .await
            .map_err(UserOpsError::Internal)?;
        // TODO: handle events
        // self.event_bus.send(CoreEvent.FS_ENTRY_UPDATED, id=self.user_manifest_id)
        // self.event_bus.send(CoreEvent.FS_WORKSPACE_CREATED, new_entry=workspace_entry)

        drop(guard);
        Ok(workspace_id)
    }

    pub async fn workspace_rename(
        &self,
        workspace_id: &EntryID,
        new_name: EntryName,
    ) -> Result<(), UserOpsError> {
        let guard = self.update_user_manifest_lock.lock().await;

        let user_manifest = self.storage.get_user_manifest();

        if let Some(workspace_entry) = user_manifest.get_workspace_entry(workspace_id) {
            let mut updated_workspace_entry = workspace_entry.to_owned();
            updated_workspace_entry.name = new_name;
            let timestamp = self.device.time_provider.now();
            let updated_user_manifest = Arc::new(
                (*user_manifest)
                    .clone()
                    .evolve_workspaces_and_mark_updated(updated_workspace_entry, timestamp),
            );
            self.storage
                .set_user_manifest(updated_user_manifest)
                .await
                .map_err(UserOpsError::Internal)?;
            // TODO: handle events
            // self.event_bus.send(CoreEvent.FS_ENTRY_UPDATED, id=self.user_manifest_id)
        } else {
            return Err(UserOpsError::UnknownWorkspace(workspace_id.to_owned()));
        }

        drop(guard);
        Ok(())
    }

    pub async fn sync(&self) -> Result<(), DynError> {
        todo!()
    }

    pub async fn workspace_share(
        &self,
        _workspace_id: &EntryID,
        _recipient: UserID,
        _role: Option<RealmRole>,
        _timestamp_greater_than: Option<DateTime>,
    ) -> Result<(), UserOpsWorkspaceShareError> {
        todo!()
    }

    pub async fn process_last_messages(&self) -> Vec<Result<IndexInt, DynError>> {
        todo!()
    }

    pub async fn workspace_start_reencryption(
        &self,
        _workspace_id: &EntryID,
    ) -> Result<ReencryptionJob, DynError> {
        todo!()
    }

    pub async fn workspace_continue_reencryption(
        &self,
        _workspace_id: &EntryID,
    ) -> Result<ReencryptionJob, DynError> {
        todo!()
    }
}
