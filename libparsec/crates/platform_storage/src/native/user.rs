// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use diesel::{result::Error::NotFound, ExpressionMethods, QueryDsl, RunQueryDsl};
use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use libparsec_platform_async::{Mutex as AsyncMutex, MutexGuard as AsyncMutexGuard};
use libparsec_types::prelude::*;

use super::db::{DatabaseError, DatabaseResult, LocalDatabase, VacuumMode};
use super::model::get_user_data_storage_db_relative_path;

pub struct NeedSyncEntries {
    pub remote: Vec<EntryID>,
    pub local: Vec<EntryID>,
}

#[derive(Debug)]
pub struct UserStorage {
    pub device: Arc<LocalDevice>,
    db: LocalDatabase,
    /// A lock that will be used to prevent concurrent update in [UserStorage::set_user_manifest].
    /// This is needed to ensure `user_manifest` stay in sync with the content of the database.
    lock_update_user_manifest: AsyncMutex<()>,
    /// Keep the user manifest currently in database here for fast access
    user_manifest: Mutex<Arc<LocalUserManifest>>,
}

#[derive(Debug)]
pub struct UserStorageUpdater<'a> {
    storage: &'a UserStorage,
    _update_guard: AsyncMutexGuard<'a, ()>,
}

impl<'a> UserStorageUpdater<'a> {
    pub async fn set_user_manifest(
        &self,
        user_manifest: Arc<LocalUserManifest>,
    ) -> anyhow::Result<()> {
        db_set_user_manifest(
            &self.storage.db,
            &self.storage.device,
            user_manifest.clone(),
        )
        .await?;

        *self
            .storage
            .user_manifest
            .lock()
            .expect("Mutex is poisoned") = user_manifest;

        Ok(())
    }
}

impl UserStorage {
    pub async fn start(data_base_dir: &Path, device: Arc<LocalDevice>) -> anyhow::Result<Self> {
        // `maybe_populate_user_storage` needs to start a `UserStorage`,
        // leading to a recursive call which is not support for async functions.
        // Hence `no_populate_start` which breaks the recursion.
        //
        // Also note we don't try to return the `UserStorage` that has been
        // use during the populate as it would change the internal state of the
        // storage (typically caches) depending of if populate has been needed or not.

        #[cfg(feature = "test-with-testbed")]
        crate::testbed::maybe_populate_user_storage(data_base_dir, device.clone()).await;

        Self::no_populate_start(data_base_dir, device).await
    }

    pub(crate) async fn no_populate_start(
        data_base_dir: &Path,
        device: Arc<LocalDevice>,
    ) -> anyhow::Result<Self> {
        // 1) Open the database

        let db_relative_path = get_user_data_storage_db_relative_path(&device);
        let db = LocalDatabase::from_path(data_base_dir, &db_relative_path, VacuumMode::default())
            .await?;

        // TODO: What should be our strategy when the database contains invalid data ?
        //
        // Currently we are conservative: we fail and stop if the data are invalid,
        // however we could also go the violent way by overwritting the invalid database.
        //
        // This has the advantage of automatically "fixing" buggy database (in case of
        // bug in previous Parsec version or if the user played fool and modified the
        // database by hand...).
        //
        // The drawback is of course the fact we may lose data. However 1) there is not
        // much to lose (worst case is if the user manifest hasn't been synced since
        // a workspace has been created) and 2) we can attempt to retrieve the user manifest
        // blob (if it is readable !) before overwritting the database.
        //
        // The worst case is if we are reading a newer version of the database: we will
        // overwrite a valid database :'(
        //
        // The alternative of this "automatic database fix" is to return a dedicated
        // error used by GUI/CLI to notify the user and provide a command to overwrite
        // it if the user ask for it.

        // 2) Initialize the database (if needed)

        super::model::initialize_model_if_needed(&db).await?;

        // 3) Retrieve the user manifest

        let user_manifest = UserStorage::load_user_manifest(&db, &device).await?;

        // 4) All done !

        let user_storage = Self {
            device,
            db,
            lock_update_user_manifest: AsyncMutex::new(()),
            user_manifest: Mutex::new(user_manifest),
        };
        Ok(user_storage)
    }

    pub async fn stop(&self) {
        self.db.close().await
    }

    pub async fn get_realm_checkpoint(&self) -> anyhow::Result<IndexInt> {
        let checkpoint = db_get_realm_checkpoint(&self.db).await?;
        IndexInt::try_from(checkpoint).map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn update_realm_checkpoint(
        &self,
        new_checkpoint: IndexInt,
        remote_user_manifest_version: Option<VersionInt>,
    ) -> anyhow::Result<()> {
        let new_checkpoint = i64::try_from(new_checkpoint)?;

        db_update_realm_checkpoint(
            &self.db,
            &self.device,
            new_checkpoint,
            remote_user_manifest_version,
        )
        .await
        .map_err(|e| anyhow::anyhow!(e))
    }

    pub fn get_user_manifest(&self) -> Arc<LocalUserManifest> {
        self.user_manifest
            .lock()
            .expect("Mutex is poisoned")
            .clone()
    }

    /// Updating the manifest is error prone:
    /// 1) the lock must be held
    /// 2) the user manifest must be fetched *after* the lock is help
    /// This method (and the related updater structure) make sure both requirements
    /// are met before providing the method to actually update the manifest.
    pub async fn for_update(&self) -> (UserStorageUpdater, Arc<LocalUserManifest>) {
        let guard = self.lock_update_user_manifest.lock().await;

        let manifest = self.get_user_manifest();
        let updater = UserStorageUpdater {
            storage: self,
            _update_guard: guard,
        };

        (updater, manifest)
    }

    async fn load_user_manifest(
        db: &LocalDatabase,
        device: &LocalDevice,
    ) -> DatabaseResult<Arc<LocalUserManifest>> {
        let ret = db_get_user_manifest(db, device).await;

        // It is possible to lack the user manifest in local if our
        // device hasn't tried to access it yet (and we are not the
        // initial device of our user, in which case the user local db is
        // initialized with a non-speculative local manifest placeholder).
        // In such case it is easy to fall back on an empty manifest
        // which is a good enough approximation of the very first version
        // of the manifest (field `created` is invalid, but it will be
        // correction by the merge during sync).
        if let Err(DatabaseError::Diesel(diesel::NotFound)) = ret {
            let timestamp = device.now();
            let manifest = Arc::new(LocalUserManifest::new(
                device.device_id.clone(),
                timestamp,
                Some(device.user_manifest_id),
                true,
            ));

            db_set_user_manifest(db, device, manifest.clone()).await?;

            Ok(manifest)
        } else {
            ret
        }
    }
}

pub async fn user_storage_non_speculative_init(
    data_base_dir: &Path,
    device: &LocalDevice,
) -> anyhow::Result<()> {
    // 1) Open the database

    let db_relative_path = get_user_data_storage_db_relative_path(device);
    let db =
        LocalDatabase::from_path(data_base_dir, &db_relative_path, VacuumMode::default()).await?;

    // 2) Initialize the database

    super::model::initialize_model_if_needed(&db).await?;

    // 3) Populate the database with the user manifest

    let timestamp = device.now();
    let manifest = Arc::new(LocalUserManifest::new(
        device.device_id.clone(),
        timestamp,
        Some(device.user_manifest_id),
        false,
    ));

    db_set_user_manifest(&db, device, manifest).await?;

    // 4) All done ! Don't forget the close the database before exit ;-)

    db.close().await;
    Ok(())
}

async fn db_get_user_manifest(
    db: &LocalDatabase,
    device: &LocalDevice,
) -> DatabaseResult<Arc<LocalUserManifest>> {
    let user_manifest_id = *device.user_manifest_id;

    let ciphered = db
        .exec(move |conn| {
            use super::model::vlobs;
            vlobs::table
                .select(vlobs::blob)
                .filter(vlobs::vlob_id.eq(user_manifest_id.as_ref()))
                .first::<Vec<u8>>(conn)
        })
        .await?;

    LocalUserManifest::decrypt_and_load(&ciphered, &device.local_symkey)
        .map(Arc::new)
        .map_err(DatabaseError::from)
}

async fn db_set_user_manifest(
    db: &LocalDatabase,
    device: &LocalDevice,
    manifest: Arc<LocalUserManifest>,
) -> DatabaseResult<()> {
    let blob = manifest.dump_and_encrypt(&device.local_symkey);

    db.exec(move |conn| {
        conn.immediate_transaction(|conn| {
            let new_vlob = super::model::NewVlob {
                vlob_id: manifest.base.id.as_bytes(),
                base_version: manifest.base.version as i64,
                remote_version: manifest.base.version as i64,
                need_sync: manifest.need_sync,
                blob: &blob,
            };

            {
                use diesel::dsl::sql;
                use diesel::upsert::excluded;
                use super::model::vlobs::dsl::*;

                diesel::insert_into(vlobs)
                .values(new_vlob)
                .on_conflict(vlob_id)
                .do_update()
                .set(
                    (
                        base_version.eq(excluded(base_version)),
                        remote_version.eq(
                            sql("(CASE WHEN `remote_version` > `excluded`.`remote_version` THEN `remote_version` ELSE `excluded`.`remote_version` END)")
                        ),
                        need_sync.eq(excluded(need_sync)),
                        blob.eq(excluded(blob)),
                    )
                )
                .execute(conn)?;
            }

            Ok(())
        })
    }).await
}

async fn db_get_realm_checkpoint(db: &LocalDatabase) -> DatabaseResult<i64> {
    let ret = db
        .exec(move |conn| {
            use super::model::realm_checkpoint;

            realm_checkpoint::table
                .select(realm_checkpoint::checkpoint)
                .filter(realm_checkpoint::_id.eq(0))
                .first(conn)
        })
        .await;

    if let Err(DatabaseError::Diesel(NotFound)) = ret {
        Ok(0)
    } else {
        ret
    }
}

async fn db_update_realm_checkpoint(
    db: &LocalDatabase,
    device: &LocalDevice,
    new_checkpoint: i64,
    user_vlob_remote_version: Option<u32>,
) -> DatabaseResult<()> {
    use super::model::{realm_checkpoint, vlobs, NewRealmCheckpoint};

    let user_manifest_id = *device.user_manifest_id;
    let new_realm_checkpoint = NewRealmCheckpoint {
        _id: 0,
        checkpoint: new_checkpoint,
    };

    db.exec(move |conn| {
        conn.immediate_transaction(|conn| {
            if let Some(remote_version) = user_vlob_remote_version {
                diesel::update(vlobs::table.filter(vlobs::vlob_id.eq(user_manifest_id.as_ref())))
                    .set(vlobs::remote_version.eq(remote_version as i64))
                    .execute(conn)?;
            }

            // Update realm checkpoint value.
            diesel::insert_into(realm_checkpoint::table)
                .values(&new_realm_checkpoint)
                .on_conflict(realm_checkpoint::_id)
                // In case of conflict, we don't try to compare and keep the biggest checkpoint:
                // - in theory new checkpoint is always bigger than the one in database
                // - in case it's not the case it's no big deal: next time we ask the server
                //   about modifications we will be notified about changes that we already
                //   know about (which is fine given we are idempotent on that)
                .do_update()
                .set(&new_realm_checkpoint)
                .execute(conn)
                .and(Ok(()))
        })
    })
    .await

    // TODO: move this comment !
    // The checkpoint index is increased everytime a vlob is created or updated in the realm,
    // hence it is possible that a vlob different from the one storing the user manifest
    // is responsible for the checkpoint increase.
    // However this is purely theorical and, in the event it occurs, not a big deal:
    // - the user realm only contains a single user manifest
    // - the user realm is only accessible by the user, so
}

#[cfg(test)]
#[path = "../../tests/unit/native_user_storage.rs"]
mod tests;
