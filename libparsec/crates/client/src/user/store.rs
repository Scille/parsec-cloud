// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use libparsec_platform_async::lock::{Mutex as AsyncMutex, MutexGuard as AsyncMutexGuard};
use libparsec_platform_storage::user::UserStorage;
use libparsec_types::prelude::*;

#[derive(Debug)]
pub(super) struct UserStore {
    device: Arc<LocalDevice>,
    storage: AsyncMutex<Option<UserStorage>>,
    /// A lock that will be used to prevent concurrent update on the user manifest.
    /// This is needed to ensure `user_manifest` stays in sync with the content of the database.
    lock_update_user_manifest: AsyncMutex<()>,
    /// Keep the user manifest currently in database here for fast access
    user_manifest: Mutex<Arc<LocalUserManifest>>,
}

impl UserStore {
    pub(crate) async fn start(
        data_base_dir: &Path,
        device: Arc<LocalDevice>,
    ) -> Result<Self, anyhow::Error> {
        // 1) Open the database

        let mut storage = UserStorage::start(data_base_dir, &device).await?;

        // 2) Load the user manifest (as it must always be synchronously available)

        let maybe_user_manifest = storage.get_user_manifest().await?;
        let user_manifest = match maybe_user_manifest {
            Some(encrypted) => {
                // TODO: if we cannot load this user manifest, should we fallback on
                //       a new speculative manifest ?
                LocalUserManifest::decrypt_and_load(&encrypted, &device.local_symkey)
                    .context("Cannot load user manifest from local storage")?
            }
            // It is possible to lack the user manifest in local if our
            // device hasn't tried to access it yet (and we are not the
            // initial device of our user, in which case the user local db is
            // initialized with a non-speculative local manifest placeholder).
            // In such case it is easy to fall back on an empty manifest
            // which is a good enough approximation of the very first version
            // of the manifest (field `created` is invalid, but it will be
            // corrected by the merge during sync).
            None => {
                let timestamp = device.now();
                LocalUserManifest::new(
                    device.device_id.clone(),
                    timestamp,
                    Some(device.user_realm_id),
                    true,
                )
            }
        };

        // 3) All set !

        Ok(Self {
            device,
            storage: AsyncMutex::new(Some(storage)),
            lock_update_user_manifest: AsyncMutex::new(()),
            user_manifest: Mutex::new(Arc::new(user_manifest)),
        })
    }

    pub(crate) async fn stop(&self) -> anyhow::Result<()> {
        let maybe_storage = self.storage.lock().await.take();
        if let Some(storage) = maybe_storage {
            storage.stop().await?;
        }
        Ok(())
    }

    pub(crate) fn get_user_manifest(&self) -> Arc<LocalUserManifest> {
        self.user_manifest
            .lock()
            .expect("Mutex is poisoned !")
            .clone()
    }

    /// Updating the manifest is error prone:
    /// 1) the lock must be held
    /// 2) the user manifest must be fetched *after* the lock is held
    /// This method (and the related updater structure) make sure both requirements
    /// are met before providing the method to actually update the manifest.
    pub async fn for_update(&self) -> (UserStoreUpdater, Arc<LocalUserManifest>) {
        let guard = self.lock_update_user_manifest.lock().await;

        let manifest = self.get_user_manifest();
        let updater = UserStoreUpdater {
            store: self,
            _update_guard: guard,
        };

        (updater, manifest)
    }

    /// As its name implies, local cache is never synchronized. Hence this method that
    /// won't update the `need_sync` and `updated` fields of the local user manifest.
    pub async fn for_update_local_workspaces(&self) -> UserForUpdateLocalWorkspacesUpdater {
        let guard = self.lock_update_user_manifest.lock().await;

        let manifest = self.get_user_manifest();
        let updater = UserForUpdateLocalWorkspacesUpdater {
            store: self,
            _update_guard: guard,
            manifest,
        };

        updater
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserStoreUpdateError {
    #[error("Component has stopped")]
    Stopped,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct UserForUpdateLocalWorkspacesUpdater<'a> {
    store: &'a UserStore,
    _update_guard: AsyncMutexGuard<'a, ()>,
    manifest: Arc<LocalUserManifest>,
}

impl<'a> UserForUpdateLocalWorkspacesUpdater<'a> {
    pub fn workspaces(&self) -> &[LocalUserManifestWorkspaceEntry] {
        &self.manifest.local_workspaces
    }

    pub async fn set_workspaces_and_keep_lock(
        &mut self,
        local_workspaces: Vec<LocalUserManifestWorkspaceEntry>,
    ) -> Result<(), UserStoreUpdateError> {
        let mut guard = self.store.storage.lock().await;
        let storage = guard
            .as_mut()
            .ok_or_else(|| UserStoreUpdateError::Stopped)?;

        Arc::make_mut(&mut self.manifest).local_workspaces = local_workspaces;
        let encrypted = self
            .manifest
            .dump_and_encrypt(&self.store.device.local_symkey);

        // Update database before cache to avoid cache corruption on database error
        storage
            .update_user_manifest(
                &encrypted,
                self.manifest.need_sync,
                self.manifest.base.version,
            )
            .await?;
        *self.store.user_manifest.lock().expect("Mutex is poisoned") = self.manifest.clone();

        Ok(())
    }

    pub async fn set_workspaces(
        mut self,
        local_workspaces: Vec<LocalUserManifestWorkspaceEntry>,
    ) -> Result<(), UserStoreUpdateError> {
        self.set_workspaces_and_keep_lock(local_workspaces).await
    }
}

#[derive(Debug)]
pub struct UserStoreUpdater<'a> {
    store: &'a UserStore,
    _update_guard: AsyncMutexGuard<'a, ()>,
}

impl<'a> UserStoreUpdater<'a> {
    pub async fn set_user_manifest(
        mut self,
        manifest: Arc<LocalUserManifest>,
    ) -> Result<(), UserStoreUpdateError> {
        self.set_user_manifest_and_keep_lock(manifest).await
    }

    pub async fn set_user_manifest_and_keep_lock(
        &mut self,
        manifest: Arc<LocalUserManifest>,
    ) -> Result<(), UserStoreUpdateError> {
        let mut guard = self.store.storage.lock().await;
        let storage = guard
            .as_mut()
            .ok_or_else(|| UserStoreUpdateError::Stopped)?;

        let encrypted = manifest.dump_and_encrypt(&self.store.device.local_symkey);

        // Update database before cache to avoid cache corruption on database error
        storage
            .update_user_manifest(&encrypted, manifest.need_sync, manifest.base.version)
            .await?;
        *self.store.user_manifest.lock().expect("Mutex is poisoned") = manifest;

        Ok(())
    }
}
