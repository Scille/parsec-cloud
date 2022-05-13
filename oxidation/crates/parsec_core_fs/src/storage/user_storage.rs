// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use std::collections::HashSet;

use parsec_api_types::EntryID;
use parsec_client_types::{LocalDevice, LocalManifest, LocalUserManifest};

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

    pub fn get_realm_checkpoint(&mut self) -> i32 {
        self.manifest_storage.get_realm_checkpoint()
    }

    pub fn update_realm_checkpoint(
        &mut self,
        new_checkpoint: i32,
        changed_vlobs: &[(EntryID, i32)],
    ) -> FSResult<()> {
        self.manifest_storage
            .update_realm_checkpoint(new_checkpoint, changed_vlobs)
    }

    pub fn get_need_sync_entries(&mut self) -> FSResult<(HashSet<EntryID>, HashSet<EntryID>)> {
        self.manifest_storage.get_need_sync_entries()
    }

    // User manifest

    pub fn get_user_manifest(&self) -> FSResult<&LocalUserManifest> {
        match self.manifest_storage.cache.get(&self.user_manifest_id) {
            Some(LocalManifest::User(manifest)) => Ok(manifest),
            _ => Err(FSError::UserManifestMissing),
        }
    }

    fn load_user_manifest(&mut self) -> FSResult<()> {
        if self
            .manifest_storage
            .get_manifest(self.user_manifest_id)
            .is_err()
        {
            let timestamp = self.device.timestamp();
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

    pub fn set_user_manifest(&mut self, user_manifest: LocalUserManifest) -> FSResult<()> {
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
    use parsec_api_crypto::SecretKey;
    use parsec_api_types::{DateTime, UserManifest};
    use rstest::rstest;
    use tests_fixtures::{alice, Device};

    use super::super::local_database::SqlitePool;
    use super::*;

    #[rstest]
    fn user_storage(alice: &Device) {
        let now = DateTime::now();
        let pool = SqlitePool::new("/tmp/manifest_storage.sqlite").unwrap();
        let conn = pool.conn().unwrap();
        let local_symkey = SecretKey::generate();
        let realm_id = EntryID::default();
        let user_manifest_id = alice.user_manifest_id;

        let manifest_storage = ManifestStorage::new(local_symkey, conn, realm_id).unwrap();
        let mut user_storage =
            UserStorage::new(alice.local_device(), user_manifest_id, manifest_storage);

        user_storage.get_realm_checkpoint();
        user_storage.update_realm_checkpoint(64, &[]).unwrap();
        user_storage.get_need_sync_entries().unwrap();
        user_storage.load_user_manifest().unwrap();

        let user_manifest = LocalUserManifest {
            base: UserManifest {
                author: alice.device_id.clone(),
                timestamp: now,

                id: user_manifest_id,
                version: 0,
                created: now,
                updated: now,
                last_processed_message: 0,
                workspaces: vec![],
            },
            need_sync: false,
            updated: now,
            last_processed_message: 0,
            workspaces: vec![],
            speculative: false,
        };

        user_storage
            .set_user_manifest(user_manifest.clone())
            .unwrap();

        assert_eq!(user_storage.get_user_manifest().unwrap(), &user_manifest);
    }
}
