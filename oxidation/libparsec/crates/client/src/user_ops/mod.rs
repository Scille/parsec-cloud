// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod merge;

use std::{path::PathBuf, sync::Arc};

use libparsec_client_connection::{protocol::authenticated_cmds, AuthenticatedCmds};
use libparsec_platform_async::Mutex as AsyncMutex;
use libparsec_platform_storage2::{
    user::UserStorage, workspace::workspace_storage_non_speculative_init,
};
use libparsec_types::prelude::*;

use crate::event_bus::EventBus;

pub type DynError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, thiserror::Error)]
pub enum UserOpsError {
    #[error("Unknown workspace `{0}`")]
    UnknownWorkspace(EntryID),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum UserOpsWorkspaceShareError {
    #[error("Unknown workspace `{0}`")]
    UnknownWorkspace(EntryID),
    #[error("Cannot share with oneself")]
    ShareToSelf,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct UserOps {
    data_base_dir: PathBuf, // TODO: use Arc ?
    device: Arc<LocalDevice>,
    storage: UserStorage,
    cmds: Arc<AuthenticatedCmds>,
    #[allow(dead_code)]
    event_bus: EventBus,
    // // Message processing is done in-order, hence it is pointless to do
    // // it concurrently
    // process_messages_lock: Mutex<()>,
    update_user_manifest_lock: AsyncMutex<()>,
}

#[derive(Debug)]
pub struct ReencryptionJob {}

impl UserOps {
    pub async fn start(
        data_base_dir: PathBuf,
        device: Arc<LocalDevice>,
        cmds: Arc<AuthenticatedCmds>,
        // remote_device_manager,
        event_bus: EventBus,
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
            cmds,
            event_bus,
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
        let user_manifest = self.storage.get_user_manifest();
        if user_manifest.need_sync {
            self._outbound_sync().await
        } else {
            self._inbound_sync().await
        }
    }

    async fn _outbound_sync(&self) -> Result<(), DynError> {
        todo!()
    }

    async fn _inbound_sync(&self) -> Result<(), DynError> {
        // Retrieve remote
        let _target_um = self._fetch_remote_user_manifest(None).await;
        // diverged_um = self.get_user_manifest()
        // if target_um.version == diverged_um.base_version:
        //     # Nothing new
        //     return

        // # New things in remote, merge is needed
        // async with self._update_user_manifest_lock:
        //     diverged_um = self.get_user_manifest()
        //     if target_um.version <= diverged_um.base_version:
        //         # Sync already achieved by a concurrent operation
        //         return
        //     merged_um = merge_local_user_manifests(diverged_um, target_um)
        //     await self.set_user_manifest(merged_um)
        //     # In case we weren't online when the sharing message arrived,
        //     # we will learn about the change in the sharing only now.
        //     # Hence send the corresponding events !
        //     self._detect_and_send_shared_events(diverged_um, merged_um)
        //     # TODO: deprecated event ?
        //     self.event_bus.send(
        //         CoreEvent.FS_ENTRY_REMOTE_CHANGED, path="/", id=self.user_manifest_id
        //     )
        //     return
        todo!()
    }

    async fn _fetch_remote_user_manifest(
        &self,
        version: Option<VersionInt>,
    ) -> Result<UserManifest, DynError> {
        let request = authenticated_cmds::v3::vlob_read::Req {
            // `encryption_revision` is always 1 given we never re-encrypt the user manifest's realm
            encryption_revision: 1,
            timestamp: None,
            version,
            vlob_id: VlobID::from(self.device.user_manifest_id.as_ref().to_owned()),
        };
        // TODO: handle errors !
        let _response = self.cmds.send(request).await?;

        // try:

        // except BackendNotAvailable as exc:
        //     raise FSBackendOfflineError(str(exc)) from exc

        // if isinstance(rep, VlobReadRepInMaintenance):
        //     raise FSWorkspaceInMaintenance(
        //         "Cannot access workspace data while it is in maintenance"
        //     )
        // elif not isinstance(rep, VlobReadRepOk):
        //     raise FSError(f"Cannot fetch user manifest from backend: {rep}")

        // expected_author = rep.author
        // expected_timestamp = rep.timestamp
        // expected_version = rep.version
        // blob = rep.blob

        // author = await self.remote_loader.get_device(expected_author)

        // try:
        //     manifest = UserManifest.decrypt_verify_and_load(
        //         blob,
        //         key=self.device.user_manifest_key,
        //         author_verify_key=author.verify_key,
        //         expected_id=self.device.user_manifest_id,
        //         expected_author=expected_author,
        //         expected_timestamp=expected_timestamp,
        //         expected_version=version if version is not None else expected_version,
        //     )

        // except DataError as exc:
        //     raise FSError(f"Invalid user manifest: {exc}") from exc

        // return manifest
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
