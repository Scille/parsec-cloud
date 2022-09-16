// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::collections::HashSet;

use libparsec_client_types::{LocalDevice, LocalManifest, LocalUserManifest};
use libparsec_types::EntryID;

use super::manifest_storage::ManifestStorage;
use crate::error::{FSError, FSResult};

pub struct UserStorage {
    device: LocalDevice,
    user_manifest_id: EntryID,
    manifest_storage: ManifestStorage,
}

impl UserStorage {
    pub fn new(
        device: LocalDevice,
        user_manifest_id: EntryID,
        manifest_storage: ManifestStorage,
    ) -> Self {
        Self {
            device,
            user_manifest_id,
            manifest_storage,
        }
    }

    // Checkpoint Interface

    pub fn get_realm_checkpoint(&self) -> i64 {
        self.manifest_storage.get_realm_checkpoint()
    }

    pub fn update_realm_checkpoint(
        &self,
        new_checkpoint: i64,
        changed_vlobs: &[(EntryID, i64)],
    ) -> FSResult<()> {
        self.manifest_storage
            .update_realm_checkpoint(new_checkpoint, changed_vlobs)
    }

    pub fn get_need_sync_entries(&self) -> FSResult<(HashSet<EntryID>, HashSet<EntryID>)> {
        self.manifest_storage.get_need_sync_entries()
    }

    // User manifest

    pub fn get_user_manifest(&self) -> FSResult<LocalUserManifest> {
        self.manifest_storage
            .get_cache_entry(self.user_manifest_id)
            .lock()
            .expect("Mutex is poisoned")
            .manifest
            .as_ref()
            .and_then(|manifest| {
                if let LocalManifest::User(manifest) = manifest {
                    Some(manifest.clone())
                } else {
                    None
                }
            })
            .ok_or(FSError::UserManifestMissing)
    }

    fn load_user_manifest(&self) -> FSResult<()> {
        if self
            .manifest_storage
            .get_manifest(self.user_manifest_id)
            .is_err()
        {
            let timestamp = self.device.now();
            let manifest = LocalUserManifest::new(
                self.device.device_id.clone(),
                timestamp,
                Some(self.device.user_manifest_id),
                true,
            );
            self.manifest_storage.set_manifest(
                self.user_manifest_id,
                LocalManifest::User(manifest),
                false,
                None,
            )?;
        }
        Ok(())
    }

    pub fn set_user_manifest(&self, user_manifest: LocalUserManifest) -> FSResult<()> {
        self.manifest_storage.set_manifest(
            self.user_manifest_id,
            LocalManifest::User(user_manifest),
            false,
            None,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use libparsec_crypto::SecretKey;
    use libparsec_types::{DateTime, UserManifest};

    use rstest::rstest;
    use tests_fixtures::{alice, timestamp, tmp_path, Device, TmpPath};

    use super::super::local_database::SqlitePool;
    use super::*;

    #[rstest]
    fn user_storage(alice: &Device, timestamp: DateTime, tmp_path: TmpPath) {
        let db_path = tmp_path.join("user_storage.sqlite");
        let pool = SqlitePool::new(db_path.to_str().unwrap()).unwrap();
        let conn = Arc::new(Mutex::new(pool.conn().unwrap()));
        let local_symkey = SecretKey::generate();
        let realm_id = EntryID::default();
        let user_manifest_id = alice.user_manifest_id;

        let manifest_storage = ManifestStorage::new(local_symkey, conn, realm_id).unwrap();
        let user_storage =
            UserStorage::new(alice.local_device(), user_manifest_id, manifest_storage);

        user_storage.get_realm_checkpoint();
        user_storage.update_realm_checkpoint(64, &[]).unwrap();
        user_storage.get_need_sync_entries().unwrap();
        user_storage.load_user_manifest().unwrap();

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
            .unwrap();

        assert_eq!(user_storage.get_user_manifest().unwrap(), user_manifest);
    }
}
