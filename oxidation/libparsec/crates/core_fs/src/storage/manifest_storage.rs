// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use diesel::{
    sql_query, table, AsChangeset, BoolExpressionMethods, ExpressionMethods, Insertable, QueryDsl,
    RunQueryDsl,
};
use fancy_regex::Regex;
use libparsec_crypto::{CryptoError, SecretKey};
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

use libparsec_client_types::LocalManifest;
use libparsec_types::{BlockID, ChunkID, EntryID};

use super::local_database::{SqliteConn, SQLITE_MAX_VARIABLE_NUMBER};
use crate::error::{FSError, FSResult};
use crate::storage::chunk_storage::chunks;

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

pub struct ManifestStorage {
    local_symkey: SecretKey,
    conn: Mutex<SqliteConn>,
    pub realm_id: EntryID,
    /// This cache contains all the manifests that have been set or accessed
    /// since the last call to `clear_memory_cache`
    pub(crate) cache: Mutex<HashMap<EntryID, LocalManifest>>,
    /// This dictionnary keeps track of all the entry ids of the manifests
    /// that have been added to the cache but still needs to be written to
    /// the conn. The corresponding value is a set with the ids of all
    /// the chunks that needs to be removed from the conn after the
    /// manifest is written. Note: this set might be empty but the manifest
    /// still requires to be flushed.
    cache_ahead_of_localdb: Mutex<HashMap<EntryID, HashSet<ChunkOrBlockID>>>,
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
        conn: Mutex<SqliteConn>,
        realm_id: EntryID,
    ) -> FSResult<Self> {
        let instance = Self {
            local_symkey,
            conn,
            realm_id,
            cache: Mutex::new(HashMap::new()),
            cache_ahead_of_localdb: Mutex::new(HashMap::new()),
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
        if flush {
            self.flush_cache_ahead_of_persistance()?;
        }
        self.cache_ahead_of_localdb
            .lock()
            .expect("Mutex is poisoned")
            .clear();
        self.cache.lock().expect("Mutex is poisoned").clear();

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
        let mut local_changes = HashSet::<_, RandomState>::from_iter(
            self.cache
                .lock()
                .expect("Mutex is poisoned")
                .iter()
                .filter_map(|(id, manifest)| {
                    if manifest.need_sync() {
                        Some(*id)
                    } else {
                        None
                    }
                }),
        );

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

    // Manifest operations

    pub fn get_manifest(&self, entry_id: EntryID) -> FSResult<LocalManifest> {
        // Look in cache first
        if let Some(manifest) = self.cache.lock().expect("Mutex is poisoned").get(&entry_id) {
            return Ok(manifest.clone());
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

        // Fill the cache
        self.cache
            .lock()
            .expect("Mutex is poisoned")
            .insert(entry_id, manifest.clone());

        Ok(manifest)
    }

    pub fn set_manifest(
        &self,
        entry_id: EntryID,
        manifest: LocalManifest,
        cache_only: bool,
        removed_ids: Option<HashSet<ChunkOrBlockID>>,
    ) -> FSResult<()> {
        // Set the cache first
        self.cache
            .lock()
            .expect("Mutex is poisoned")
            .insert(entry_id, manifest);
        // Tag the entry as ahead of localdb
        if self
            .cache_ahead_of_localdb
            .lock()
            .expect("Mutex is poisoned")
            .get(&entry_id)
            .is_none()
        {
            self.cache_ahead_of_localdb
                .lock()
                .expect("Mutex is poisoned")
                .insert(entry_id, HashSet::new());
        }

        // Cleanup
        if let Some(removed_ids) = &removed_ids {
            let value = self
                .cache_ahead_of_localdb
                .lock()
                .expect("Mutex is poisoned")
                .get(&entry_id)
                .unwrap()
                | removed_ids;

            self.cache_ahead_of_localdb
                .lock()
                .expect("Mutex is poisoned")
                .insert(entry_id, value);
        }

        // Flush the cached value to the localdb
        if !cache_only {
            self.ensure_manifest_persistent(entry_id)?;
        }
        Ok(())
    }

    pub fn ensure_manifest_persistent(&self, entry_id: EntryID) -> FSResult<()> {
        match (
            self.cache_ahead_of_localdb
                .lock()
                .expect("Mutex is poisoned")
                .get(&entry_id),
            self.cache.lock().expect("Mutex is poisoned").get(&entry_id),
        ) {
            (Some(pending_chunk_ids), Some(manifest)) => {
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

                for pending_chunk_ids_chunk in pending_chunk_ids.chunks(SQLITE_MAX_VARIABLE_NUMBER)
                {
                    diesel::delete(
                        chunks::table.filter(chunks::chunk_id.eq_any(pending_chunk_ids_chunk)),
                    )
                    .execute(conn)
                    .map_err(|e| FSError::DeleteTable(format!("chunks: clear_manifest {e}")))?;
                }

                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn flush_cache_ahead_of_persistance(&self) -> FSResult<()> {
        let keys = self
            .cache_ahead_of_localdb
            .lock()
            .expect("Mutex is poisoned")
            .keys()
            .copied()
            .collect::<Vec<_>>();
        for entry_id in keys {
            self.ensure_manifest_persistent(entry_id)?;
        }

        Ok(())
    }

    // This method is not used in the code base but it is still tested
    // as it might come handy in a cleanup routine later

    #[deprecated]
    pub fn clear_manifest(&self, entry_id: EntryID) -> FSResult<()> {
        // Remove from cache
        let in_cache = self
            .cache
            .lock()
            .expect("Mutex is poisoned")
            .remove(&entry_id)
            .is_some();

        // Remove from local database
        let conn = &mut *self.conn.lock().expect("Mutex is poisoned");
        let deleted = diesel::delete(vlobs::table.filter(vlobs::vlob_id.eq((*entry_id).as_ref())))
            .execute(conn)
            .map_err(|e| FSError::DeleteTable(format!("vlobs: clear_manifest {e}")))?
            > 0;

        if let Some(pending_chunk_ids) = self
            .cache_ahead_of_localdb
            .lock()
            .expect("Mutex is poisoned")
            .remove(&entry_id)
        {
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

        if !deleted && !in_cache {
            return Err(FSError::LocalMiss(*entry_id));
        }

        Ok(())
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
        let t2 = t1 + 1;
        let db_path = tmp_path.join("manifest_storage.sqlite");
        let pool = SqlitePool::new(db_path.to_str().unwrap()).unwrap();
        let conn = Mutex::new(pool.conn().unwrap());
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
