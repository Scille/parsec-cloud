// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{
    collections::HashSet,
    path::Path,
    sync::{Arc, RwLock},
};

use libparsec_client_types::{LocalDevice, LocalManifest, LocalUserManifest};
use libparsec_types::EntryID;
use local_db::LocalDatabase;

use super::{manifest_storage::ManifestStorage, version::get_user_data_storage_db_path};
use crate::error::{FSError, FSResult};

pub struct UserStorage {
    pub device: LocalDevice,
    pub user_manifest_id: EntryID,
    manifest_storage: ManifestStorage,
    data_conn: Arc<LocalDatabase>,
    user_manifest_copy: Arc<RwLock<Option<LocalUserManifest>>>,
}

impl UserStorage {
    pub async fn from_db_dir(
        device: LocalDevice,
        user_manifest_id: EntryID,
        data_base_dir: &Path,
    ) -> FSResult<Self> {
        let data_path = get_user_data_storage_db_path(data_base_dir, &device);
        let conn = LocalDatabase::from_path(
            data_path
                .to_str()
                .expect("Non-Utf-8 character found in data_path"),
        )
        .await?;
        let conn = Arc::new(conn);
        let manifest_storage =
            ManifestStorage::new(device.local_symkey.clone(), user_manifest_id, conn.clone())
                .await?;
        Ok(Self {
            device,
            user_manifest_id,
            manifest_storage,
            data_conn: conn,
            user_manifest_copy: Arc::new(RwLock::new(None)),
        })
    }

    /// Close the connections to the databases.
    /// Provide a way to manually close those connections.
    /// Event tho they will be closes when [UserStorage] is dropped.
    pub async fn close_connections(&self) {
        self.data_conn.close()
    }

    // Checkpoint Interface

    pub async fn get_realm_checkpoint(&self) -> i64 {
        self.manifest_storage.get_realm_checkpoint().await
    }

    pub async fn update_realm_checkpoint(
        &self,
        new_checkpoint: i64,
        changed_vlobs: &[(EntryID, i64)],
    ) -> FSResult<()> {
        self.manifest_storage
            .update_realm_checkpoint(new_checkpoint, changed_vlobs)
            .await
    }

    pub async fn get_need_sync_entries(&self) -> FSResult<(HashSet<EntryID>, HashSet<EntryID>)> {
        self.manifest_storage.get_need_sync_entries().await
    }

    // User manifest

    pub fn get_user_manifest(&self) -> FSResult<LocalUserManifest> {
        self.user_manifest_copy
            .read()
            .expect("RwLock is poisoned")
            .as_ref()
            .cloned()
            .ok_or(FSError::UserManifestMissing)
    }

    pub async fn load_user_manifest(&self) -> FSResult<()> {
        match self
            .manifest_storage
            .get_manifest(self.user_manifest_id)
            .await
        {
            Ok(LocalManifest::User(manifest)) => {
                self.user_manifest_copy
                    .write()
                    .expect("RwLock is poisoned")
                    .replace(manifest);
                Ok(())
            }
            Ok(_) => panic!("User manifest id is used for something other than a user manifest"),
            Err(_) => {
                let timestamp = self.device.now();
                let manifest = LocalUserManifest::new(
                    self.device.device_id.clone(),
                    timestamp,
                    Some(self.device.user_manifest_id),
                    true,
                );
                self.set_user_manifest(manifest).await
            }
        }
    }

    pub async fn set_user_manifest(&self, user_manifest: LocalUserManifest) -> FSResult<()> {
        assert_eq!(
            self.user_manifest_id, user_manifest.base.id,
            "UserManifest should have the same EntryID as registered in UserStorage"
        );
        self.manifest_storage
            .set_manifest(
                self.user_manifest_id,
                LocalManifest::User(user_manifest.clone()),
                false,
                None,
            )
            .await?;
        self.user_manifest_copy
            .write()
            .expect("RwLock is poisoned")
            .replace(user_manifest);
        Ok(())
    }
}

pub async fn user_storage_non_speculative_init(
    data_base_dir: &Path,
    device: LocalDevice,
) -> FSResult<()> {
    let data_path = get_user_data_storage_db_path(data_base_dir, &device);
    let conn = LocalDatabase::from_path(
        data_path
            .to_str()
            .expect("Non Utf-8 character found in data_path"),
    )
    .await?;
    let conn = Arc::new(conn);
    let manifest_storage =
        ManifestStorage::new(device.local_symkey.clone(), device.user_manifest_id, conn).await?;

    let timestamp = device.now();
    let manifest = LocalUserManifest::new(
        device.device_id,
        timestamp,
        Some(device.user_manifest_id),
        false,
    );

    manifest_storage
        .set_manifest(
            device.user_manifest_id,
            LocalManifest::User(manifest),
            false,
            None,
        )
        .await?;
    manifest_storage.clear_memory_cache(true).await?;
    manifest_storage.close_connection().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use libparsec_types::{DateTime, UserManifest};

    use rstest::rstest;
    use tests_fixtures::{alice, timestamp, tmp_path, Device, TmpPath};

    use super::*;

    #[rstest]
    #[tokio::test]
    async fn user_storage(alice: &Device, timestamp: DateTime, tmp_path: TmpPath) {
        let db_path = tmp_path.join("user_storage.sqlite");
        let user_manifest_id = alice.user_manifest_id;

        let user_storage =
            UserStorage::from_db_dir(alice.local_device(), user_manifest_id, &db_path)
                .await
                .unwrap();

        user_storage.get_realm_checkpoint().await;
        user_storage.update_realm_checkpoint(64, &[]).await.unwrap();
        user_storage.get_need_sync_entries().await.unwrap();
        user_storage.load_user_manifest().await.unwrap();

        let user_manifest = LocalUserManifest {
            base: UserManifest {
                author: alice.device_id.clone(),
                timestamp,
                id: user_manifest_id,
                version: 0,
                created: timestamp,
                updated: timestamp,
                last_processed_message: 0,
                workspaces: vec![],
            },
            need_sync: false,
            updated: timestamp,
            last_processed_message: 0,
            workspaces: vec![],
            speculative: false,
        };

        user_storage
            .set_user_manifest(user_manifest.clone())
            .await
            .unwrap();

        assert_eq!(user_storage.get_user_manifest().unwrap(), user_manifest);
    }
}
