// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use diesel::{
    sql_query, BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl,
};
use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex},
};

use libparsec_platform_async::{
    event::{Event, EventListener},
    lock::{Mutex as AsyncMutex, MutexGuard as AsyncMutexGuard},
};
use libparsec_types::prelude::*;

use super::super::db::{DatabaseError, DatabaseResult, LocalDatabase, VacuumMode};
use super::super::model::get_workspace_data_storage_db_relative_path;

/*
 * Workspace manifest updater
 */

#[derive(Debug)]
pub struct WorkspaceDataStorageWorkspaceManifestUpdater<'a> {
    storage: &'a WorkspaceDataStorage,
    _update_guard: AsyncMutexGuard<'a, ()>,
}

impl<'a> WorkspaceDataStorageWorkspaceManifestUpdater<'a> {
    pub async fn set_workspace_manifest(
        self,
        manifest: Arc<LocalWorkspaceManifest>,
    ) -> anyhow::Result<()> {
        db_set_workspace_manifest(&self.storage.db, &self.storage.device, &manifest).await?;

        self.storage
            .cache
            .lock()
            .expect("Mutex is poisoned")
            .workspace_manifest = manifest;

        Ok(())
    }
}

/*
 * Child manifests updater & lock
 */

mod child_manifests {
    use super::*;

    // Unlike for workspace manifest, we cannot just use a simple async mutex to
    // protect update of child manifests.
    //
    // This is because the child manifests cache is itself behind a sync mutex (which
    // must be released before waiting on the update child manifest async mutex !).
    // The natural solution to this is to wrap the update child manifest async mutex
    // in a Arc, but this creates a new issue: the `WorkspaceDataStorageChildManifestUpdater`
    // would then have to keep both the Arc<AsyncMutex> and the corresponding `AsyncMutexGuard`.
    // However this is not possible given the guard takes a reference on the mutex !
    //
    // Hence we basically have to re-implement a custom async mutex using the lower-level
    // event system.
    #[derive(Debug)]
    enum EntryLockState {
        /// A single coroutine has locked the manifest
        Taken,
        /// Multiple coroutines want to lock the manifest: the first has the lock
        /// and the subsequent ones are listening on the event (which will be fired
        /// when the first coroutine releases the manifest).
        /// Note this means all coroutines waiting for the event will fight with no
        /// fairness to take the lock (it's basically up to the tokio scheduler).
        TakenWithConcurrency(Event),
    }

    #[derive(Debug)]
    pub(super) struct ChildManifestsLock {
        per_manifest_lock: Vec<(VlobID, EntryLockState)>,
    }

    pub(super) enum ChildManifestLockTakeOutcome<'a> {
        // The updater object is responsible to release the update lock on drop.
        // Hence this object must be created as soon as possible otherwise a deadlock
        // could occur (typically in case of an early return due to error handling)
        Taken(WorkspaceDataStorageChildManifestUpdater<'a>),
        NeedWait(EventListener),
    }

    impl ChildManifestsLock {
        pub(super) fn new() -> Self {
            Self {
                per_manifest_lock: Vec::new(),
            }
        }

        pub(super) fn take<'a>(
            &mut self,
            entry_id: VlobID,
            storage: &'a WorkspaceDataStorage,
        ) -> ChildManifestLockTakeOutcome<'a> {
            match self
                .per_manifest_lock
                .iter_mut()
                .find(|(id, _)| *id == entry_id)
            {
                // Entry is missing: the manifest is currently under update
                None => {
                    self.per_manifest_lock
                        .push((entry_id, EntryLockState::Taken));
                    let updater = WorkspaceDataStorageChildManifestUpdater { entry_id, storage };
                    // It's official: we are now the one and only updating this manifest !
                    ChildManifestLockTakeOutcome::Taken(updater)
                }
                // The entry is present: the manifest is already taken for update !
                Some((_, state)) => match state {
                    // Appart from the coroutine currently updating the manifest, nobody
                    // else is waiting for it... except us !
                    // So it's our job to setup the event to get notified when the manifest
                    // is again available for update.
                    EntryLockState::Taken => {
                        let event = Event::new();
                        let listener = event.listen();
                        *state = EntryLockState::TakenWithConcurrency(event);
                        ChildManifestLockTakeOutcome::NeedWait(listener)
                    }
                    // Multiple coroutines are already waiting to get their hands on
                    // this manifest, we are just the n+1 :)
                    EntryLockState::TakenWithConcurrency(event) => {
                        ChildManifestLockTakeOutcome::NeedWait(event.listen())
                    }
                },
            }
        }

        // See `WorkspaceDataStorageChildManifestUpdater`'s drop for the release part
    }

    #[derive(Debug)]
    pub struct WorkspaceDataStorageChildManifestUpdater<'a> {
        storage: &'a WorkspaceDataStorage,
        entry_id: VlobID,
    }

    impl<'a> Drop for WorkspaceDataStorageChildManifestUpdater<'a> {
        fn drop(&mut self) {
            let mut guard = self.storage.cache.lock().expect("Mutex is poisoned");

            let (index, (_, state)) = guard
                .lock_update_child_manifests
                .per_manifest_lock
                .iter()
                .enumerate()
                .find(|(_, (id, _))| *id == self.entry_id)
                // Entry has been inserted when *we* have been created, and it is only removed
                // when *we* are dropped (i.e. now)
                .expect("Must be present");

            // If other coroutines are waiting for the manifest, now is the time to notify them !
            if let EntryLockState::TakenWithConcurrency(event) = state {
                event.notify(usize::MAX);
            }

            // Finally we remove the entry altogether, this is the way to signify
            // it is free to take now
            guard
                .lock_update_child_manifests
                .per_manifest_lock
                .swap_remove(index);
        }
    }

    impl<'a> WorkspaceDataStorageChildManifestUpdater<'a> {
        /// `delay_flush` is to be used when the file is opened (given then the `flush`
        /// syscall should be used to guarantee the data are persistent)
        pub async fn set_file_manifest(
            self,
            manifest: Arc<LocalFileManifest>,
            delay_flush: bool,
            to_remove: impl Iterator<Item = ChunkID>,
        ) -> Result<(), anyhow::Error> {
            {
                let mut guard = self.storage.cache.lock().expect("Mutex is poisoned");

                guard.child_manifests.insert(
                    manifest.base.id,
                    ArcLocalChildManifest::File(manifest.clone()),
                );

                // Operation is O(n), but n is expected to be small: only the opened files
                // should have their manifest part of the list.
                // On top of that we consider recently opened files are more likely to be
                // accessed and hence crawl the list in reverse order.
                let already_present = guard
                    .work_ahead_of_db_to_commit
                    .iter()
                    .rev()
                    .any(|x| *x == manifest.base.id);
                if !already_present {
                    guard.work_ahead_of_db_to_commit.push(manifest.base.id);
                }
                // We don't try to do deduplication here, this is because the provided IDs should
                // already be unique (given a chunk is only referenced by a single file manifest,
                // and hence is removed once)
                guard.work_ahead_of_db_to_delete.extend(to_remove);
            }

            if !delay_flush {
                self.storage.flush_work_ahead_of_db().await
            } else {
                Ok(())
            }
        }

        pub async fn set_folder_manifest(
            self,
            manifest: Arc<LocalFolderManifest>,
        ) -> anyhow::Result<()> {
            // We store the new manifest first in database then in cache: doing the other
            // way around would mean a concurrent read could take the new manifest value
            // from the cache, only to have the database update fail (hence most likely
            // cancelling the update, hence leaving the concurrent read with a now invalid
            // manifest !)

            db_set_folder_manifest(&self.storage.db, &self.storage.device, &manifest).await?;

            let mut guard = self.storage.cache.lock().expect("Mutex is poisoned");
            guard
                .child_manifests
                .insert(self.entry_id, ArcLocalChildManifest::Folder(manifest));

            Ok(())
        }
    }
}
pub use child_manifests::WorkspaceDataStorageChildManifestUpdater;
use child_manifests::{ChildManifestLockTakeOutcome, ChildManifestsLock};

/*
 * WorkspaceDataStorage & friends
 */

#[derive(Debug)]
struct ManifestsCache {
    workspace_manifest: Arc<LocalWorkspaceManifest>,
    // `child_manifests` contains a cache on the database:
    // - the cache may be cleaned at any given time (i.e. inserting an entry in the cache
    //   doesn't guarantee it will be available later on)
    // - if the cache is present, it always correspond to the latest value (so the cache
    //   should always be preferred over data coming from the database)
    // - each cache entry can be "taken" for write access. In this mode
    child_manifests: HashMap<VlobID, ArcLocalChildManifest>,
    // Just like for workspace manifest, each child manifest has a dedicated async lock
    // to prevent concurrent update (ensuring consistency between cache and database).
    lock_update_child_manifests: ChildManifestsLock,
    work_ahead_of_db_to_commit: Vec<VlobID>,
    work_ahead_of_db_to_delete: Vec<ChunkID>,
}

#[derive(Debug)]
pub struct WorkspaceDataStorage {
    pub realm_id: VlobID,
    pub device: Arc<LocalDevice>,
    db: LocalDatabase,
    cache: Mutex<ManifestsCache>,
    /// A lock that will be used to prevent concurrent update on the workspace manifest.
    /// This is needed to ensure the manifest in cache stays in sync with the content of the database.
    lock_update_workspace_manifest: AsyncMutex<()>,
    /// A lock held to prevent concurrent update on the changed not-yet-flushed-to-database
    /// Flush operation first take the "work ahead of db" and clear it from the cache
    /// structure. This means we really don't want concurrent flush, especially given
    /// database operation may fail (in such case the work is stored back in cache, but
    /// the concurrent flush may have save to database changes that are dependant on the
    /// one that failed !)
    lock_flush_work_ahead_of_db: AsyncMutex<()>,
}

#[derive(Debug, thiserror::Error)]
pub enum GetChildManifestError {
    #[error("Manifest not present in data storage")]
    NotFound,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct NeedSyncEntries {
    pub local: Vec<VlobID>,
    pub remote: Vec<VlobID>,
}

impl WorkspaceDataStorage {
    pub async fn start(
        data_base_dir: &Path,
        device: Arc<LocalDevice>,
        realm_id: VlobID,
    ) -> anyhow::Result<Self> {
        // `maybe_populate_workspace_data_storage` needs to start a `WorkspaceDataStorage`,
        // leading to a recursive call which is not support for async functions.
        // Hence `no_populate_start` which breaks the recursion.
        //
        // Also note we don't try to return the `WorkspaceDataStorage` that has been
        // use during the populate as it would change the internal state of the
        // storage (typically caches) depending of if populate has been needed or not.

        #[cfg(feature = "test-with-testbed")]
        crate::testbed::maybe_populate_workspace_data_storage(
            data_base_dir,
            device.clone(),
            realm_id,
        )
        .await;

        Self::no_populate_start(data_base_dir, device, realm_id).await
    }

    pub(crate) async fn no_populate_start(
        data_base_dir: &Path,
        device: Arc<LocalDevice>,
        realm_id: VlobID,
    ) -> anyhow::Result<Self> {
        // 1) Open the database

        let db_relative_path = get_workspace_data_storage_db_relative_path(&device, &realm_id);
        let db = LocalDatabase::from_path(data_base_dir, &db_relative_path, VacuumMode::default())
            .await?;

        // 2) Initialize the database (if needed)

        super::super::model::initialize_model_if_needed(&db).await?;

        // 3) Retrieve the workspace manifest

        let workspace_manifest =
            WorkspaceDataStorage::load_workspace_manifest(&db, &device, realm_id).await?;

        // 4) All done !

        let storage = Self {
            realm_id,
            device,
            db,
            cache: Mutex::new(ManifestsCache {
                workspace_manifest,
                child_manifests: HashMap::new(),
                lock_update_child_manifests: ChildManifestsLock::new(),
                work_ahead_of_db_to_commit: Vec::new(),
                work_ahead_of_db_to_delete: Vec::new(),
            }),
            lock_update_workspace_manifest: AsyncMutex::new(()),
            lock_flush_work_ahead_of_db: AsyncMutex::new(()),
        };
        Ok(storage)
    }

    pub async fn stop(&self) -> anyhow::Result<()> {
        self.flush_work_ahead_of_db()
            .await
            .context("Cannot flush work ahead of DB")?;
        self.db.close().await;
        Ok(())
    }

    async fn load_workspace_manifest(
        db: &LocalDatabase,
        device: &LocalDevice,
        realm_id: VlobID,
    ) -> DatabaseResult<Arc<LocalWorkspaceManifest>> {
        let ret = db_get_workspace_manifest(db, device, realm_id).await;

        // It is possible to lack the workspace manifest in local if our
        // device hasn't tried to access it yet (and we are not the initial
        // creator of this workspace, in which case the local db is initialized
        // with a non-speculative local manifest placeholder).
        // In such case it is easy to fall back on an empty manifest which is
        // a good enough approximation of the very first version of the
        // manifest (field `created` is invalid, but it will be correction by
        // the merge during sync).
        if let Err(DatabaseError::Diesel(diesel::NotFound)) = ret {
            let timestamp = device.now();
            let manifest = Arc::new(LocalWorkspaceManifest::new(
                device.device_id.clone(),
                timestamp,
                Some(realm_id),
                true,
            ));

            db_set_workspace_manifest(db, device, &manifest).await?;

            Ok(manifest)
        } else {
            ret
        }
    }

    /// Updating the workspace manifest is error prone:
    /// 1) the workspace manifest update lock must be held
    /// 2) the workspace manifest must be fetched *after* the lock is held
    /// This method (and the related updater structure) make sure both requirements
    /// are met before providing the method to actually update the manifest.
    pub async fn for_update_workspace_manifest(
        &self,
    ) -> (
        WorkspaceDataStorageWorkspaceManifestUpdater,
        Arc<LocalWorkspaceManifest>,
    ) {
        let guard = self.lock_update_workspace_manifest.lock().await;

        let manifest = self
            .cache
            .lock()
            .expect("Mutex is poisoned")
            .workspace_manifest
            .clone();
        let updater = WorkspaceDataStorageWorkspaceManifestUpdater {
            storage: self,
            _update_guard: guard,
        };

        (updater, manifest)
    }

    pub async fn for_update_child_manifest(
        &self,
        entry_id: VlobID,
    ) -> Result<
        (
            WorkspaceDataStorageChildManifestUpdater,
            Option<ArcLocalChildManifest>,
        ),
        anyhow::Error,
    > {
        loop {
            let outcome = {
                let mut guard = self.cache.lock().expect("Mutex is poisoned");
                guard.lock_update_child_manifests.take(entry_id, self)
            };

            match outcome {
                // The manifest is already being updated by somebody else...
                ChildManifestLockTakeOutcome::NeedWait(listener) => {
                    listener.await;
                }
                // Finally it's our turn to update the manifest !
                ChildManifestLockTakeOutcome::Taken(updater) => {
                    let maybe_manifest = match self.get_child_manifest(entry_id).await {
                        Ok(manifest) => Some(manifest),
                        Err(GetChildManifestError::NotFound) => None,
                        // Here updater's drop will release the update lock automatically ;-)
                        Err(err) => return Err(err.into()),
                    };
                    return Ok((updater, maybe_manifest));
                }
            }
        }
    }

    // Sync pattern

    // TODO: Should we introduce a lock on the prevent sync pattern apply
    // operations ?
    // Typically this operation would be cancelled if another call to it
    // occurs.

    /// Set the "prevent sync" pattern for the corresponding workspace.
    ///
    /// This operation is idempotent,
    /// i.e. it does not reset the `fully_applied` flag if the pattern hasn't changed.
    pub async fn set_prevent_sync_pattern(&self, pattern: Regex) -> Result<bool, anyhow::Error> {
        db_set_prevent_sync_pattern(&self.db, pattern)
            .await
            .map_err(|err| err.into())
    }

    /// Mark the provided pattern as fully applied.
    ///
    /// This is meant to be called after one made sure that all the manifests in the
    /// workspace are compliant with the new pattern. The applied pattern is provided
    /// as an argument in order to avoid concurrency issues.
    pub async fn mark_prevent_sync_pattern_fully_applied(
        &self,
        pattern: Regex,
    ) -> Result<bool, anyhow::Error> {
        db_mark_prevent_sync_pattern_fully_applied(&self.db, pattern)
            .await
            .map_err(|err| err.into())
    }

    // Checkpoint operations

    // Each time a vlob is created/uploaded, this increment a counter in the
    // corresponding realm: this is the realm checkpoint.
    // The idea is the client (*us* !) can query the server with the last checkpoint
    // it knows of, and receive the new checkpoint along with a list of all the
    // vlob that have been created/updated.
    //
    // However this system comes with it own challenges: we only save on database
    // the remote version of the vlob we know about.
    // 1) Client knows about realm checkpoint 10
    // 1) Client fetches vlob 0x42 at version 1 from server
    // 2) Vlob 0x42 is updated at version 2 on server
    // 3) Client ask the server what has changed since the realm checkpoint 10.
    //    Server answer with realm checkpoint 11 with vlob 0x42 now at version 2
    // 4) Client doesn't have vlob 0x42 on database hence discard the related
    //    information but update the realm checkpoint in database to 11
    // 5) Client now update vlob 0x42 with information from step 1)
    // 6) Now the database contains realm checkpoint 11 but vlob 0x42 with realm
    //    version 1 ! The vlob won't be synced until it gets another update !
    //
    // The solution to this is to make "fetch vlob + update in database" operation
    // exclusive with "poll vlob changes + update in database". This should be
    // achieved by the caller, typically with read/write lock.

    pub async fn get_realm_checkpoint(&self) -> Result<IndexInt, anyhow::Error> {
        db_get_realm_checkpoint(&self.db)
            .await
            .map_err(|err| err.into())
    }

    pub async fn update_realm_checkpoint(
        &self,
        new_checkpoint: IndexInt,
        changed_vlobs: Vec<(VlobID, VersionInt)>,
    ) -> Result<(), anyhow::Error> {
        // We need to take into account the not-yet-flushed manifests, (e.g. otherwise
        // an already fetched manifest will appear in the remote entry list if it has
        // not been flushed)
        //
        // To do that we take the simplest route (i.e. flush to database then do DB
        // queries).
        // There is no performance issue here given in practice this function is only
        // used when the server connection goes online.
        self.flush_work_ahead_of_db().await?;

        db_update_realm_checkpoint(&self.db, new_checkpoint, changed_vlobs)
            .await
            .map_err(|err| err.into())
    }

    pub async fn get_need_sync_entries(&self) -> Result<NeedSyncEntries, anyhow::Error> {
        // We need to take into account the not-yet-flushed manifests, (e.g. otherwise
        // an already fetched manifest will appear in the remote entry list if it has
        // not been flushed)
        //
        // To do that we take the simplest route (i.e. flush to database then do DB
        // queries).
        // There is no performance issue here given in practice this function is only
        // used when the WorkspaceOps is initialized (and on top of that at that time
        // the work ahead lists are most likely empty !).
        self.flush_work_ahead_of_db().await?;

        // The database contains all info now, just query it !
        db_get_need_sync_entries(&self.db)
            .await
            .map_err(anyhow::Error::from)
    }

    // Manifest operations

    pub fn get_workspace_manifest(&self) -> Arc<LocalWorkspaceManifest> {
        self.cache
            .lock()
            .expect("Mutex is poisoned")
            .workspace_manifest
            .clone()
    }

    pub async fn get_child_manifest(
        &self,
        entry_id: VlobID,
    ) -> Result<ArcLocalChildManifest, GetChildManifestError> {
        // First lookup in the cache
        let maybe_manifest = self
            .cache
            .lock()
            .expect("Mutex is poisoned")
            .child_manifests
            .get(&entry_id)
            .cloned();

        match maybe_manifest {
            // We got a cache hit, all done !
            Some(manifest) => Ok(manifest),
            // Cache miss, our last chance is to look into the database
            None => {
                let ret = db_get_child_manifest(&self.db, &self.device, entry_id).await;
                if let Err(DatabaseError::Diesel(diesel::NotFound)) = ret {
                    return Err(GetChildManifestError::NotFound);
                }
                let manifest = ret.map_err(anyhow::Error::from)?;
                // Before leaving, don't forget to update the cache !
                match self
                    .cache
                    .lock()
                    .expect("Mutex is poisoned")
                    .child_manifests
                    .entry(entry_id)
                {
                    // Cache is missing, we can populate it
                    std::collections::hash_map::Entry::Vacant(e) => {
                        e.insert(manifest.clone());
                        Ok(manifest)
                    }
                    // Plot twist: the cache entry already exists after all !
                    // Of course we shouldn't modify the cache (this would typically
                    // overwrite a concurrent manifest update !).
                    std::collections::hash_map::Entry::Occupied(e) => {
                        // The cache always contains the most recent version of the
                        // manifest, so better use it instead of what we got from
                        // the database
                        Ok(e.get().to_owned())
                    }
                }
            }
        }
    }

    // TODO: rename this given we flush everything at once ?
    pub async fn ensure_manifest_persistent(&self, _entry_id: VlobID) -> Result<(), anyhow::Error> {
        self.flush_work_ahead_of_db().await
    }

    async fn flush_work_ahead_of_db(&self) -> Result<(), anyhow::Error> {
        let _flush_guard = self.lock_flush_work_ahead_of_db.lock().await;

        let (to_commit, to_delete_ids) = {
            let mut guard = self.cache.lock().expect("Mutex is poisoned");

            // Shortcut if there is nothing to flush
            if guard.work_ahead_of_db_to_commit.is_empty()
                && guard.work_ahead_of_db_to_delete.is_empty()
            {
                return Ok(());
            }

            let to_commit_ids = std::mem::take(&mut guard.work_ahead_of_db_to_commit);
            // Don't do any fancy processing here given the lock is held
            let to_commit: Vec<ArcLocalChildManifest> = to_commit_ids
                .iter()
                // TODO: log error if the `entry_id` is not part of `child_manifests` ?
                .map(|entry_id| {
                    guard
                        .child_manifests
                        .get(entry_id)
                        .cloned()
                        .expect("Child manifest need commit but not in cache !")
                })
                .collect();

            let to_delete_ids = std::mem::take(&mut guard.work_ahead_of_db_to_delete);

            (to_commit, to_delete_ids)
        };

        // TODO: would be better to find a way to avoid cloning `to_delete_ids` (given
        // this is only needed in the unlikely case the database flush fails)
        let outcome =
            db_flush_work_ahead(&self.db, &self.device, &to_commit, to_delete_ids.clone())
                .await
                .map_err(anyhow::Error::from);

        // If the database operation fails, we must insert back the changes in the work
        // ahead lists (otherwise we would lose those changes, and next successful flush
        // will make the database inconsistent !)
        if outcome.is_err() {
            let mut guard = self.cache.lock().expect("Mutex is poisoned");

            guard
                .work_ahead_of_db_to_commit
                .extend(to_commit.into_iter().map(|m| match m {
                    ArcLocalChildManifest::File(m) => m.base.id,
                    ArcLocalChildManifest::Folder(m) => m.base.id,
                }));

            guard.work_ahead_of_db_to_delete.extend(to_delete_ids);
        }

        outcome
    }
}

/*
 * Database operations
 */

async fn db_flush_work_ahead(
    db: &LocalDatabase,
    device: &LocalDevice,
    to_commit: &[ArcLocalChildManifest],
    to_delete: Vec<ChunkID>,
) -> Result<(), DatabaseError> {
    let to_commit: Vec<_> = to_commit
        .iter()
        .map(|manifest| match manifest {
            ArcLocalChildManifest::File(m) => {
                let ciphered = m.dump_and_encrypt(&device.local_symkey);
                (*m.base.id, m.base.version as i64, m.need_sync, ciphered)
            }
            ArcLocalChildManifest::Folder(m) => {
                let ciphered = m.dump_and_encrypt(&device.local_symkey);
                (*m.base.id, m.base.version as i64, m.need_sync, ciphered)
            }
        })
        .collect();

    db.exec(move |conn| {
        conn.immediate_transaction(|conn| {

            // 1) Commit the modified vlobs

            // TODO: replace with [replace_into](https://docs.diesel.rs/2.0.x/diesel/fn.replace_into.html)
            let query = sql_query("INSERT OR REPLACE INTO vlobs (vlob_id, blob, need_sync, base_version, remote_version)
                    VALUES (
                    ?, ?, ?, ?,
                        max(
                            ?,
                            IFNULL((SELECT remote_version FROM vlobs WHERE vlob_id=?), 0)
                        )
                    )");

            for (vlob_id, base_version, need_sync, ciphered) in to_commit {
                let vlob_id = vlob_id.as_ref();
                query
                    .clone()
                    .bind::<diesel::sql_types::Binary, _>(vlob_id)
                    .bind::<diesel::sql_types::Binary, _>(ciphered)
                    .bind::<diesel::sql_types::Bool, _>(need_sync)
                    .bind::<diesel::sql_types::BigInt, _>(base_version)
                    .bind::<diesel::sql_types::BigInt, _>(base_version)
                    .bind::<diesel::sql_types::Binary, _>(vlob_id)
                    .execute(conn)?;
            }

            // 2) Delete the no-longer-needed chunks

            to_delete
                .chunks(super::super::db::LOCAL_DATABASE_MAX_VARIABLE_NUMBER)
                .try_for_each(|chunked_ids| {
                    use super::super::model::chunks;
                    let chunked_ids: Vec<_> = chunked_ids.iter().map(|id| id.as_bytes()).collect();
                    diesel::delete(chunks::table.filter(chunks::chunk_id.eq_any(chunked_ids)))
                        .execute(conn)
                        .and(Ok(()))
                })?;

            Ok(())
        })
    }).await
}

pub async fn db_get_workspace_manifest(
    db: &LocalDatabase,
    device: &LocalDevice,
    realm_id: VlobID,
) -> DatabaseResult<Arc<LocalWorkspaceManifest>> {
    let ciphered = db_get_manifest(db, realm_id).await?;

    LocalWorkspaceManifest::decrypt_and_load(&ciphered, &device.local_symkey)
        .map(Arc::new)
        .map_err(DatabaseError::from)
}

pub async fn db_get_child_manifest(
    db: &LocalDatabase,
    device: &LocalDevice,
    id: VlobID,
) -> DatabaseResult<ArcLocalChildManifest> {
    let ciphered = db_get_manifest(db, id).await?;

    let manifest = LocalChildManifest::decrypt_and_load(&ciphered, &device.local_symkey)
        .map_err(DatabaseError::from)?;

    match manifest {
        LocalChildManifest::File(m) => Ok(ArcLocalChildManifest::File(Arc::new(m))),
        LocalChildManifest::Folder(m) => Ok(ArcLocalChildManifest::Folder(Arc::new(m))),
    }
}

async fn db_get_manifest(db: &LocalDatabase, id: VlobID) -> DatabaseResult<Vec<u8>> {
    let id = *id;
    db.exec(move |conn| {
        use super::super::model::vlobs;
        vlobs::table
            .select(vlobs::blob)
            .filter(vlobs::vlob_id.eq(id.as_ref()))
            .first::<Vec<u8>>(conn)
    })
    .await
}

pub(super) async fn db_set_workspace_manifest(
    db: &LocalDatabase,
    device: &LocalDevice,
    manifest: &LocalWorkspaceManifest,
) -> DatabaseResult<()> {
    let blob = manifest.dump_and_encrypt(&device.local_symkey);
    db_set_manifest(
        db,
        manifest.base.id,
        manifest.base.version,
        manifest.need_sync,
        blob,
    )
    .await
}

async fn db_set_folder_manifest(
    db: &LocalDatabase,
    device: &LocalDevice,
    manifest: &LocalFolderManifest,
) -> DatabaseResult<()> {
    let blob = manifest.dump_and_encrypt(&device.local_symkey);
    db_set_manifest(
        db,
        manifest.base.id,
        manifest.base.version,
        manifest.need_sync,
        blob,
    )
    .await
}

async fn db_set_manifest(
    db: &LocalDatabase,
    id: VlobID,
    base_version: VersionInt,
    need_sync: bool,
    blob: Vec<u8>,
) -> DatabaseResult<()> {
    db.exec(move |conn| {
        conn.immediate_transaction(|conn| {
            let new_vlob = super::super::model::NewVlob {
                vlob_id: id.as_bytes(),
                base_version: base_version as i64,
                remote_version: base_version as i64,
                need_sync,
                blob: &blob,
            };

            {
                use diesel::dsl::sql;
                use diesel::upsert::excluded;
                use super::super::model::vlobs::dsl::*;

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

// TODO: return a PreventSyncPatternState enum instead of a bool
async fn db_set_prevent_sync_pattern(db: &LocalDatabase, pattern: Regex) -> DatabaseResult<bool> {
    let pattern = pattern.to_string();

    db.exec(move |conn| {
        conn.immediate_transaction(|conn| {
            use super::super::model::prevent_sync_pattern;

            let maybe_update = diesel::update(
                prevent_sync_pattern::table.filter(
                    prevent_sync_pattern::_id
                        .eq(0)
                        .and(prevent_sync_pattern::pattern.ne(&pattern)),
                ),
            )
            .set((
                prevent_sync_pattern::pattern.eq(&pattern),
                prevent_sync_pattern::fully_applied.eq(false),
            ))
            .returning(prevent_sync_pattern::fully_applied)
            .get_result::<bool>(conn)
            .optional()?;

            match maybe_update {
                // The pattern was updated, `fully_applied` is always false there
                Some(fully_applied) => Ok(fully_applied),
                // The pattern was not updated, need to fetch `fully_applied` field
                None => prevent_sync_pattern::table
                    .select(prevent_sync_pattern::fully_applied)
                    .filter(prevent_sync_pattern::_id.eq(0))
                    .first::<bool>(conn),
            }
        })
    })
    .await
}

// TODO: return a PreventSyncPatternState enum instead of a bool
async fn db_mark_prevent_sync_pattern_fully_applied(
    db: &LocalDatabase,
    pattern: Regex,
) -> DatabaseResult<bool> {
    let pattern = pattern.to_string();

    db.exec(move |conn| {
        conn.immediate_transaction(|conn| {
            use super::super::model::prevent_sync_pattern;

            let maybe_update = diesel::update(
                prevent_sync_pattern::table.filter(
                    prevent_sync_pattern::_id
                        .eq(0)
                        .and(prevent_sync_pattern::pattern.eq(pattern)),
                ),
            )
            .set(prevent_sync_pattern::fully_applied.eq(true))
            .returning(prevent_sync_pattern::fully_applied)
            .get_result::<bool>(conn)
            .optional()?;

            match maybe_update {
                // The pattern was the one we expected, `fully_applied` has been set to true
                Some(fully_applied) => Ok(fully_applied),
                // The pattern was not expected, need to fetch `fully_applied` field.
                // TODO: We can end up in a weird state where we return full_applied, but
                // it is another pattern that has been applied ! This doesn't seems a big
                // deal in practice but it would be better to avoid this (by introducing
                // a lock on the prevent sync pattern apply operations ?)
                None => prevent_sync_pattern::table
                    .select(prevent_sync_pattern::fully_applied)
                    .filter(prevent_sync_pattern::_id.eq(0))
                    .first::<bool>(conn),
            }
        })
    })
    .await
}

// Checkpoint operations

async fn db_get_realm_checkpoint(db: &LocalDatabase) -> DatabaseResult<IndexInt> {
    db.exec(move |conn| {
        use super::super::model::realm_checkpoint;
        realm_checkpoint::table
            .select(realm_checkpoint::checkpoint)
            .filter(realm_checkpoint::_id.eq(0))
            .first::<i64>(conn)
            .optional()
            .map(|maybe_checkpoint| maybe_checkpoint.unwrap_or(0) as IndexInt)
    })
    .await
}

async fn db_update_realm_checkpoint(
    db: &LocalDatabase,
    new_checkpoint: IndexInt,
    changed_vlobs: Vec<(VlobID, VersionInt)>,
) -> DatabaseResult<()> {
    // There is little trick related to how we store `changed_vlobs` here:
    // we only store the remote version for vlobs already present in the database.
    //
    // The obvious benefit is we don't have to create empty vlob entries for every
    // vlob ever created in the workspace.
    //
    // However this also means we don't know the full state of the workspace at
    // the checkpoint we saved. Let's consider an example:
    // 1) client polls the server for changes since checkpoint 3, it receives
    // `  new_checkpoint=5, changed vlobs=[(0x42, 3]` (i.e. a vlob with id 0x42 has
    //    been updated to version 3 at checkpoint 4 or 5)
    // 2) client doesn't have vlob 0x42 in database, hence it only stores the new
    //    checkpoint value
    // 3) client downloads an old version 2 of vlob 0x42 (e.g. it is using history)
    // 4) client now has in database checkpoint 5 but vlob 0x42 with remote version 2
    //    (instead of the expected 3 !). Hence it won't try to download the newer
    //    version of this vlob as long as it is not yet again updated.
    //
    // To prevent from this, it is vitally important that we only store in the database
    // the latest version of a vlob according to the server ! This way we are guaranteed
    // that the provided vlob's version is at least equal to the value in `changed_vlobs`
    // we discarded when saving the checkpoint.

    // https://github.com/diesel-rs/diesel/issues/1517
    // TODO: How to improve ?
    // It is difficult to build a raw sql query with bind in a for loop
    // Another solution is to query all data then insert

    db.exec(move |conn| {
        conn.immediate_transaction(|conn| {
            use super::super::model::{realm_checkpoint, vlobs, NewRealmCheckpoint};

            // Update version on vlobs.
            // TODO: Can update the vlobs in batch => `UPDATE vlobs SET remote_version = :version WHERE vlob_id in [changed_vlobs[].id];`
            for (id, version) in changed_vlobs {
                let id = id.as_bytes();
                let version = version as i64;
                diesel::update(vlobs::table.filter(vlobs::vlob_id.eq(id)))
                    .set(vlobs::remote_version.eq(version))
                    .execute(conn)?;
            }

            // Update realm checkpoint value.
            let new_realm_checkpoint = NewRealmCheckpoint {
                _id: 0,
                checkpoint: new_checkpoint as i64,
            };
            diesel::insert_into(realm_checkpoint::table)
                .values(&new_realm_checkpoint)
                .on_conflict(realm_checkpoint::_id)
                .do_update()
                .set(&new_realm_checkpoint)
                .execute(conn)
                .and(Ok(()))
        })
    })
    .await
}

async fn db_get_need_sync_entries(db: &LocalDatabase) -> DatabaseResult<NeedSyncEntries> {
    let rows = db
        .exec(move |conn| {
            use super::super::model::vlobs;

            vlobs::table
                .select((
                    vlobs::vlob_id,
                    vlobs::need_sync,
                    vlobs::base_version,
                    vlobs::remote_version,
                ))
                .filter(
                    vlobs::need_sync
                        .eq(true)
                        .or(vlobs::base_version.ne(vlobs::remote_version)),
                )
                .load::<(Vec<u8>, bool, i64, i64)>(conn)
        })
        .await?;

    let mut remote_changes = vec![];
    let mut local_changes = vec![];
    for (manifest_id, need_sync, base_version, remote_version) in rows {
        let manifest_id = VlobID::try_from(manifest_id.as_slice())
            .map_err(|_| DatabaseError::InvalidData(DataError::Serialization))?;

        if need_sync {
            local_changes.push(manifest_id);
        }

        if base_version != remote_version {
            remote_changes.push(manifest_id);
        }
    }

    Ok(NeedSyncEntries {
        local: local_changes,
        remote: remote_changes,
    })
}
