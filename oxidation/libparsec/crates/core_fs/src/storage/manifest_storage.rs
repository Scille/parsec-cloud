// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use diesel::{
    sql_query, table, AsChangeset, BoolExpressionMethods, ExpressionMethods, Insertable, QueryDsl,
    RunQueryDsl,
};
use fancy_regex::Regex;
use libparsec_crypto::{CryptoError, SecretKey};
use std::{
    collections::{hash_map::RandomState, HashMap, HashSet},
    sync::{Arc, Mutex},
};

use libparsec_client_types::LocalManifest;
use libparsec_types::{BlockID, ChunkID, EntryID};

use super::local_database::{SqliteConn, SQLITE_MAX_VARIABLE_NUMBER};
use crate::{
    error::{FSError, FSResult},
    storage::chunk_storage::chunks,
};

table! {
    prevent_sync_pattern (_id) {
        _id -> BigInt,
        pattern -> Text,
        fully_applied -> Bool,
    }
}

table! {
    realm_checkpoint (_id) {
        _id -> BigInt,
        checkpoint -> BigInt,
    }
}

table! {
    vlobs (vlob_id) {
        vlob_id -> Binary, // UUID
        base_version -> BigInt,
        remote_version -> BigInt,
        need_sync -> Bool,
        blob -> Binary,
    }
}

const EMPTY_PATTERN: &str = r"^\b$"; // Do not match anything (https://stackoverflow.com/a/2302992/2846140)

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ChunkOrBlockID {
    ChunkID(ChunkID),
    BlockID(BlockID),
}

impl std::convert::AsRef<[u8]> for ChunkOrBlockID {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::ChunkID(id) => (**id).as_ref(),
            Self::BlockID(id) => (**id).as_ref(),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = vlobs)]
struct NewVlob<'a> {
    vlob_id: &'a [u8],
    base_version: i64,
    remote_version: i64,
    need_sync: bool,
    blob: &'a [u8],
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = realm_checkpoint)]
struct NewRealmCheckpoint {
    _id: i64,
    checkpoint: i64,
}

#[derive(Insertable)]
#[diesel(table_name = prevent_sync_pattern)]
struct NewPreventSyncPattern<'a> {
    _id: i64,
    pattern: &'a str,
    fully_applied: bool,
}

#[derive(Clone)]
pub struct ManifestStorage {
    local_symkey: SecretKey,
    conn: Arc<Mutex<SqliteConn>>,
    pub realm_id: EntryID,
    /// This cache contains all the manifests that have been set or accessed
    /// since the last call to `clear_memory_cache`
    caches: Arc<Mutex<HashMap<EntryID, Arc<Mutex<CacheEntry>>>>>,
}

/// A cache entry that may contain a manifest and its pending chunk.
#[derive(Default)]
pub(crate) struct CacheEntry {
    /// The current manifest for the cache.
    pub(crate) manifest: Option<LocalManifest>,
    /// The set of entries ids that are pending to be written.
    ///
    /// > Note: this set might be empty but the manifest
    /// > still requires to be flushed.
    pending_chunk_ids: Option<HashSet<ChunkOrBlockID>>,
}

impl CacheEntry {
    /// Return `true` if we have chunk that need to be push to the local database.
    pub fn has_chunk_to_be_flush(&self) -> bool {
        self.pending_chunk_ids.is_some()
    }
}

impl Drop for ManifestStorage {
    fn drop(&mut self) {
        self.flush_cache_ahead_of_persistance()
            .expect("Cannot flush cache when ManifestStorage dropped");
    }
}

impl ManifestStorage {
    pub fn new(
        local_symkey: SecretKey,
        conn: Arc<Mutex<SqliteConn>>,
        realm_id: EntryID,
    ) -> FSResult<Self> {
        let instance = Self {
            local_symkey,
            conn,
            realm_id,
            caches: Arc::new(Mutex::new(HashMap::default())),
        };
        instance.create_db()?;
        Ok(instance)
    }

    /// Database initialization
    pub fn create_db(&self) -> FSResult<()> {
        let conn = &mut *self.conn.lock().expect("Mutex is poisoned");
        sql_query(
            "CREATE TABLE IF NOT EXISTS vlobs (
              vlob_id BLOB PRIMARY KEY NOT NULL, -- UUID
              base_version INTEGER NOT NULL,
              remote_version INTEGER NOT NULL,
              need_sync BOOLEAN NOT NULL,
              blob BLOB NOT NULL
            );",
        )
        .execute(conn)
        .map_err(|e| FSError::CreateTable(format!("vlobs {e}")))?;
        // Singleton storing the checkpoint
        sql_query(
            "CREATE TABLE IF NOT EXISTS realm_checkpoint (
              _id INTEGER PRIMARY KEY NOT NULL,
              checkpoint INTEGER NOT NULL
            );",
        )
        .execute(conn)
        .map_err(|e| FSError::CreateTable(format!("realm_checkpoint {e}")))?;
        // Singleton storing the prevent_sync_pattern
        sql_query(
            "CREATE TABLE IF NOT EXISTS prevent_sync_pattern (
              _id INTEGER PRIMARY KEY NOT NULL,
              pattern TEXT NOT NULL,
              fully_applied BOOLEAN NOT NULL
            );",
        )
        .execute(conn)
        .map_err(|e| FSError::CreateTable(format!("prevent_sync_pattern {e}")))?;

        let pattern = NewPreventSyncPattern {
            _id: 0,
            pattern: EMPTY_PATTERN,
            fully_applied: false,
        };
        // Set the default "prevent sync" pattern if it doesn't exist
        diesel::insert_or_ignore_into(prevent_sync_pattern::table)
            .values(pattern)
            .execute(conn)
            .map_err(|e| FSError::InsertTable(format!("prevent_sync_pattern: create_db {e}")))?;

        Ok(())
    }

    #[cfg(test)]
    fn drop_db(&self) -> FSResult<()> {
        let conn = &mut *self.conn.lock().expect("Mutex is poisoned");
        sql_query("DROP TABLE IF EXISTS vlobs;")
            .execute(conn)
            .map_err(|e| FSError::DropTable(format!("vlobs {e}")))?;

        sql_query("DROP TABLE IF EXISTS realm_checkpoints;")
            .execute(conn)
            .map_err(|e| FSError::DropTable(format!("realm_checkpoints {e}")))?;

        sql_query("DROP TABLE IF EXISTS prevent_sync_pattern;")
            .execute(conn)
            .map_err(|e| FSError::DropTable(format!("prevent_sync_pattern {e}")))?;

        Ok(())
    }

    pub fn clear_memory_cache(&self, flush: bool) -> FSResult<()> {
        let mut caches_lock = self.caches.lock().expect("Mutex is poisoned");
        let drained = caches_lock.drain();
        if flush {
            let items = drained
                .map(|(entry_id, entry)| (entry_id, entry))
                .collect::<Vec<(EntryID, Arc<Mutex<CacheEntry>>)>>();

            drop(caches_lock);
            for (entry_id, lock) in items {
                self.ensure_manifest_persistent_lock(entry_id, lock)?;
            }
        }
        Ok(())
    }

    // "Prevent sync" pattern operations

    pub fn get_prevent_sync_pattern(&self) -> FSResult<(Regex, bool)> {
        let conn = &mut *self.conn.lock().expect("Mutex is poisoned");
        let (re, fully_applied) = prevent_sync_pattern::table
            .select((
                prevent_sync_pattern::pattern,
                prevent_sync_pattern::fully_applied,
            ))
            .filter(prevent_sync_pattern::_id.eq(0))
            .first::<(String, _)>(conn)
            .map_err(|e| {
                FSError::QueryTable(format!(
                    "prevent_sync_pattern: get_prevent_sync_pattern {e}"
                ))
            })?;

        let re = Regex::new(&re).map_err(|e| {
            FSError::QueryTable(format!("prevent_sync_pattern: corrupted pattern {e}"))
        })?;

        Ok((re, fully_applied))
    }

    /// Set the "prevent sync" pattern for the corresponding workspace
    /// This operation is idempotent,
    /// i.e it does not reset the `fully_applied` flag if the pattern hasn't changed.
    pub fn set_prevent_sync_pattern(&self, pattern: &Regex) -> FSResult<()> {
        let pattern = pattern.as_str();
        let conn = &mut *self.conn.lock().expect("Mutex is poisoned");
        diesel::update(
            prevent_sync_pattern::table.filter(
                prevent_sync_pattern::_id
                    .eq(0)
                    .and(prevent_sync_pattern::pattern.ne(pattern)),
            ),
        )
        .set((
            prevent_sync_pattern::pattern.eq(pattern),
            prevent_sync_pattern::fully_applied.eq(false),
        ))
        .execute(conn)
        .map_err(|e| {
            FSError::UpdateTable(format!(
                "prevent_sync_pattern: set_prevent_sync_pattern {e}"
            ))
        })?;

        Ok(())
    }

    /// Mark the provided pattern as fully applied.
    /// This is meant to be called after one made sure that all the manifests in the
    /// workspace are compliant with the new pattern. The applied pattern is provided
    /// as an argument in order to avoid concurrency issues.
    pub fn mark_prevent_sync_pattern_fully_applied(&self, pattern: &Regex) -> FSResult<()> {
        let pattern = pattern.as_str();
        let conn = &mut *self.conn.lock().expect("Mutex is poisoned");
        diesel::update(
            prevent_sync_pattern::table.filter(
                prevent_sync_pattern::_id
                    .eq(0)
                    .and(prevent_sync_pattern::pattern.eq(pattern)),
            ),
        )
        .set(prevent_sync_pattern::fully_applied.eq(true))
        .execute(conn)
        .map_err(|e| {
            FSError::UpdateTable(format!(
                "prevent_sync_pattern: mark_prevent_sync_pattern_fully_applied {e}"
            ))
        })?;

        Ok(())
    }

    // Checkpoint operations

    pub fn get_realm_checkpoint(&self) -> i64 {
        let conn = &mut *self.conn.lock().expect("Mutex is poisoned");
        realm_checkpoint::table
            .select(realm_checkpoint::checkpoint)
            .filter(realm_checkpoint::_id.eq(0))
            .first(conn)
            .unwrap_or(0)
    }

    pub fn update_realm_checkpoint(
        &self,
        new_checkpoint: i64,
        changed_vlobs: &[(EntryID, i64)],
    ) -> FSResult<()> {
        let new_realm_checkpoint = NewRealmCheckpoint {
            _id: 0,
            checkpoint: new_checkpoint,
        };

        // https://github.com/diesel-rs/diesel/issues/1517
        // TODO: How to improve ?
        // It is difficult to build a raw sql query with bind in a for loop
        // Another solution is to query all data then insert
        let conn = &mut *self.conn.lock().expect("Mutex is poisoned");
        for (id, version) in changed_vlobs {
            sql_query("UPDATE vlobs SET remote_version = ? WHERE vlob_id = ?;")
                .bind::<diesel::sql_types::BigInt, _>(version)
                .bind::<diesel::sql_types::Binary, _>((**id).as_ref())
                .execute(conn)
                .map_err(|e| FSError::UpdateTable(format!("vlobs: update_realm_checkpoint {e}")))?;
        }

        diesel::insert_into(realm_checkpoint::table)
            .values(&new_realm_checkpoint)
            .on_conflict(realm_checkpoint::_id)
            .do_update()
            .set(&new_realm_checkpoint)
            .execute(conn)
            .map_err(|e| {
                FSError::InsertTable(format!("realm_checkpoint: update_realm_checkpoint {e}"))
            })?;

        Ok(())
    }

    pub fn get_need_sync_entries(&self) -> FSResult<(HashSet<EntryID>, HashSet<EntryID>)> {
        let mut remote_changes = HashSet::new();
        let caches = self.caches.lock().expect("Mutex is poisoned").clone();
        let mut local_changes =
            HashSet::<_, RandomState>::from_iter(caches.iter().filter_map(|(id, cache)| {
                if cache
                    .lock()
                    .expect("Mutex is poisoned")
                    .manifest
                    .as_ref()
                    .map(|manifest| manifest.need_sync())
                    .unwrap_or_default()
                {
                    Some(*id)
                } else {
                    None
                }
            }));

        let conn = &mut *self.conn.lock().expect("Mutex is poisoned");
        for (manifest_id, need_sync, bv, rv) in vlobs::table
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
            .map_err(|e| FSError::QueryTable(format!("vlobs: get_need_sync_entries {e}")))?
        {
            let manifest_id =
                EntryID::from(<[u8; 16]>::try_from(&manifest_id[..]).map_err(|e| {
                    FSError::QueryTable(format!("vlobs: corrupted manifest_id {e}"))
                })?);

            if need_sync {
                local_changes.insert(manifest_id);
            }

            if bv != rv {
                remote_changes.insert(manifest_id);
            }
        }

        Ok((local_changes, remote_changes))
    }

    /// Get a cache entry identified by `entry_id`.
    /// If the cache is not present it will insert it then return it (see `or_default`).
    pub(crate) fn get_or_insert_default_cache_entry(
        &self,
        entry_id: EntryID,
    ) -> Arc<Mutex<CacheEntry>> {
        self.caches
            .lock()
            .expect("Mutex is poisoned")
            .entry(entry_id)
            .or_default()
            .clone()
    }
    // Manifest operations

    pub fn get_manifest(&self, entry_id: EntryID) -> FSResult<LocalManifest> {
        // Look in cache first
        let cache_entry = self.get_or_insert_default_cache_entry(entry_id);
        let mut cache_unlock = cache_entry.lock().expect("Mutex is poisoned");
        if let Some(manifest) = cache_unlock.manifest.clone() {
            return Ok(manifest);
        }

        // Look into the database
        let conn = &mut *self.conn.lock().expect("Mutex is poisoned");
        let manifest = vlobs::table
            .select(vlobs::blob)
            .filter(vlobs::vlob_id.eq((*entry_id).as_ref()))
            .first::<Vec<u8>>(conn)
            .map_err(|_| FSError::LocalMiss(*entry_id))?;

        let manifest = LocalManifest::decrypt_and_load(&manifest, &self.local_symkey)
            .map_err(|_| FSError::Crypto(CryptoError::Decryption))?;

        cache_unlock.manifest = Some(manifest.clone());

        Ok(manifest)
    }

    /// Get a cached manifest.
    pub fn get_cached_manifest(&self, entry_id: EntryID) -> Option<LocalManifest> {
        let cache_entry = {
            self.caches
                .lock()
                .expect("Mutex is poisoned")
                .get(&entry_id)
                .cloned()
        };
        cache_entry.and_then(|entry| {
            let unlock = entry.lock().expect("Mutex is poisoned");
            unlock.manifest.clone()
        })
    }

    pub fn set_manifest(
        &self,
        entry_id: EntryID,
        manifest: LocalManifest,
        cache_only: bool,
        removed_ids: Option<HashSet<ChunkOrBlockID>>,
    ) -> FSResult<()> {
        let cache_entry = self.get_or_insert_default_cache_entry(entry_id);
        let mut cache_unlock = cache_entry.lock().expect("Mutex is poisoned");

        cache_unlock.manifest = Some(manifest);
        let pending_chunk_ids = cache_unlock
            .pending_chunk_ids
            .get_or_insert(HashSet::default());
        if let Some(removed_ids) = &removed_ids {
            *pending_chunk_ids = (pending_chunk_ids as &HashSet<ChunkOrBlockID>) | removed_ids;
        }
        // Flush the cached value to the localdb
        if !cache_only {
            self.ensure_manifest_persistent_unlock(entry_id, &mut cache_unlock)?;
        }

        Ok(())
    }

    pub fn ensure_manifest_persistent(&self, entry_id: EntryID) -> FSResult<()> {
        if let Some(cache_entry) = self
            .caches
            .lock()
            .expect("Mutex is poisoned")
            .get(&entry_id)
            .cloned()
        {
            self.ensure_manifest_persistent_lock(entry_id, cache_entry)
        } else {
            Ok(())
        }
    }

    fn ensure_manifest_persistent_lock(
        &self,
        entry_id: EntryID,
        cache_lock: Arc<Mutex<CacheEntry>>,
    ) -> FSResult<()> {
        self.ensure_manifest_persistent_unlock(
            entry_id,
            &mut cache_lock.lock().expect("Mutex is poisoned"),
        )
    }

    fn ensure_manifest_persistent_unlock(
        &self,
        entry_id: EntryID,
        cache: &mut CacheEntry,
    ) -> FSResult<()> {
        if let (Some(manifest), Some(ref pending_chunk_ids)) =
            (&mut cache.manifest, &mut cache.pending_chunk_ids)
        {
            let ciphered = manifest.dump_and_encrypt(&self.local_symkey);
            let pending_chunk_ids = pending_chunk_ids
                .iter()
                .map(|chunk_id| chunk_id.as_ref())
                .collect::<Vec<_>>();

            let vlob_id = (*entry_id).as_ref();

            let conn = &mut *self.conn.lock().expect("Mutex is poisoned");
            sql_query("INSERT OR REPLACE INTO vlobs (vlob_id, blob, need_sync, base_version, remote_version)
                        VALUES (
                        ?, ?, ?, ?,
                            max(
                                ?,
                                IFNULL((SELECT remote_version FROM vlobs WHERE vlob_id=?), 0)
                            )
                        )")
                        .bind::<diesel::sql_types::Binary, _>(vlob_id)
                        .bind::<diesel::sql_types::Binary, _>(ciphered)
                        .bind::<diesel::sql_types::Bool, _>(manifest.need_sync())
                        .bind::<diesel::sql_types::BigInt, _>(manifest.base_version() as i64)
                        .bind::<diesel::sql_types::BigInt, _>(manifest.base_version() as i64)
                        .bind::<diesel::sql_types::Binary, _>(vlob_id)
                        .execute(conn)
                        .map_err(|e| FSError::InsertTable(format!("vlobs: ensure_manifest_persistent {e}")))?;

            for pending_chunk_ids_chunk in pending_chunk_ids.chunks(SQLITE_MAX_VARIABLE_NUMBER) {
                diesel::delete(
                    chunks::table.filter(chunks::chunk_id.eq_any(pending_chunk_ids_chunk)),
                )
                .execute(conn)
                .map_err(|e| FSError::DeleteTable(format!("chunks: clear_manifest {e}")))?;
            }
            cache.pending_chunk_ids = None;
        }
        Ok(())
    }

    pub fn flush_cache_ahead_of_persistance(&self) -> FSResult<()> {
        let items = self
            .caches
            .lock()
            .expect("Mutex is poisoned")
            .iter()
            .map(|(entry_id, entry)| (*entry_id, entry.clone()))
            .collect::<Vec<(EntryID, Arc<Mutex<CacheEntry>>)>>();
        for (entry_id, lock) in items {
            self.ensure_manifest_persistent_lock(entry_id, lock)?;
        }
        Ok(())
    }

    /// This method is not used in the code base but it is still tested
    /// as it might come handy in a cleanup routine later
    #[deprecated]
    pub fn clear_manifest(&self, entry_id: EntryID) -> FSResult<()> {
        // Remove from cache
        let cache_entry = self.get_or_insert_default_cache_entry(entry_id);
        let mut cache_unlock = cache_entry.lock().expect("Mutex is poisoned");

        // Remove from local database
        let conn = &mut *self.conn.lock().expect("Mutex is poisoned");
        let deleted = diesel::delete(vlobs::table.filter(vlobs::vlob_id.eq((*entry_id).as_ref())))
            .execute(conn)
            .map_err(|e| FSError::DeleteTable(format!("vlobs: clear_manifest {e}")))?
            > 0;

        if let Some(pending_chunk_ids) = &cache_unlock.pending_chunk_ids {
            let pending_chunk_ids = pending_chunk_ids
                .iter()
                .map(ChunkOrBlockID::as_ref)
                .collect::<Vec<_>>();

            for pending_chunk_ids_chunk in pending_chunk_ids.chunks(SQLITE_MAX_VARIABLE_NUMBER) {
                diesel::delete(
                    chunks::table.filter(chunks::chunk_id.eq_any(pending_chunk_ids_chunk)),
                )
                .execute(conn)
                .map_err(|e| FSError::DeleteTable(format!("chunks: clear_manifest {e}")))?;
            }
        }
        let is_manifest_cached = cache_unlock.manifest.is_some();

        cache_unlock.manifest = None;
        cache_unlock.pending_chunk_ids = None;

        if !deleted && !is_manifest_cached {
            return Err(FSError::LocalMiss(*entry_id));
        }

        Ok(())
    }

    /// Return `true` is the given manifest identified by `entry_id` is present in the cache
    /// waiting to be written onto the database.
    ///
    /// For more information see [ManifestStorage::cache_ahead_of_localdb].
    pub fn is_manifest_cache_ahead_of_persistance(&self, entry_id: &EntryID) -> bool {
        self.caches
            .lock()
            .expect("Mutex is poisoned")
            .get(entry_id)
            .map(|entry| {
                entry
                    .lock()
                    .expect("Mutex is poisoned")
                    .has_chunk_to_be_flush()
            })
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use libparsec_client_types::{Chunk, LocalFileManifest};
    use libparsec_crypto::HashDigest;
    use libparsec_types::{BlockAccess, Blocksize, DateTime, DeviceID, FileManifest};

    use rstest::rstest;
    use tests_fixtures::{timestamp, tmp_path, TmpPath};

    use super::*;
    use crate::storage::local_database::SqlitePool;

    #[rstest]
    fn manifest_storage(tmp_path: TmpPath, timestamp: DateTime) {
        let t1 = timestamp;
        let t2 = t1.add_us(1);
        let db_path = tmp_path.join("manifest_storage.sqlite");
        let pool = SqlitePool::new(db_path.to_str().unwrap()).unwrap();
        let conn = Arc::new(Mutex::new(pool.conn().unwrap()));
        let local_symkey = SecretKey::generate();
        let realm_id = EntryID::default();

        let manifest_storage = ManifestStorage::new(local_symkey, conn, realm_id).unwrap();
        manifest_storage.drop_db().unwrap();
        manifest_storage.create_db().unwrap();

        let (re, fully_applied) = manifest_storage.get_prevent_sync_pattern().unwrap();

        assert_eq!(re.as_str(), EMPTY_PATTERN);
        assert!(!fully_applied);

        manifest_storage
            .set_prevent_sync_pattern(&Regex::new(r"\z").unwrap())
            .unwrap();

        let (re, fully_applied) = manifest_storage.get_prevent_sync_pattern().unwrap();

        assert_eq!(re.as_str(), r"\z");
        assert!(!fully_applied);

        manifest_storage
            .mark_prevent_sync_pattern_fully_applied(&Regex::new(EMPTY_PATTERN).unwrap())
            .unwrap();

        let (re, fully_applied) = manifest_storage.get_prevent_sync_pattern().unwrap();

        assert_eq!(re.as_str(), r"\z");
        assert!(!fully_applied);

        let entry_id = EntryID::default();

        manifest_storage
            .update_realm_checkpoint(64, &[(entry_id, 2)])
            .unwrap();

        assert_eq!(manifest_storage.get_realm_checkpoint(), 64);

        let local_file_manifest = LocalManifest::File(LocalFileManifest {
            base: FileManifest {
                author: DeviceID::default(),
                timestamp: t1,
                id: EntryID::default(),
                parent: EntryID::default(),
                version: 1,
                created: t1,
                updated: t1,
                size: 8,
                blocksize: Blocksize::try_from(8).unwrap(),
                blocks: vec![BlockAccess {
                    id: BlockID::default(),
                    key: SecretKey::generate(),
                    offset: 0,
                    size: std::num::NonZeroU64::try_from(8).unwrap(),
                    digest: HashDigest::from_data(&[]),
                }],
            },
            need_sync: false,
            updated: t2,
            size: 8,
            blocksize: Blocksize::try_from(8).unwrap(),
            blocks: vec![vec![Chunk {
                id: ChunkID::default(),
                start: 0,
                stop: std::num::NonZeroU64::try_from(8).unwrap(),
                raw_offset: 0,
                raw_size: std::num::NonZeroU64::try_from(8).unwrap(),
                access: None,
            }]],
        });

        manifest_storage
            .set_manifest(entry_id, local_file_manifest.clone(), false, None)
            .unwrap();

        assert_eq!(
            manifest_storage.get_manifest(entry_id).unwrap(),
            local_file_manifest
        );

        let (local_changes, remote_changes) = manifest_storage.get_need_sync_entries().unwrap();

        assert_eq!(local_changes, HashSet::new());
        assert_eq!(remote_changes, HashSet::new());
    }
}
