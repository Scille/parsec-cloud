// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use diesel::{sql_query, BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
use fancy_regex::Regex;
use parsec_api_crypto::{CryptoError, SecretKey};
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};

use parsec_api_types::{BlockID, ChunkID, EntryID};
use parsec_client_types::LocalManifest;

use super::local_database::{SqliteConn, SQLITE_MAX_VARIABLE_NUMBER};
use crate::error::{FSError, FSResult};
use crate::schema::{chunks, prevent_sync_pattern, realm_checkpoint, vlobs};

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
    base_version: i32,
    remote_version: i32,
    need_sync: bool,
    blob: &'a [u8],
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = realm_checkpoint)]
struct NewRealmCheckpoint {
    _id: i32,
    checkpoint: i32,
}

#[derive(Insertable)]
#[diesel(table_name = prevent_sync_pattern)]
struct NewPreventSyncPattern<'a> {
    _id: i32,
    pattern: &'a str,
    fully_applied: bool,
}

pub struct ManifestStorage {
    local_symkey: SecretKey,
    conn: SqliteConn,
    pub realm_id: EntryID,
    // This cache contains all the manifests that have been set or accessed
    // since the last call to `clear_memory_cache`
    pub(crate) cache: HashMap<EntryID, LocalManifest>,
    // This dictionnary keeps track of all the entry ids of the manifests
    // that have been added to the cache but still needs to be written to
    // the conn. The corresponding value is a set with the ids of all
    // the chunks that needs to be removed from the conn after the
    // manifest is written. Note: this set might be empty but the manifest
    // still requires to be flushed.
    cache_ahead_of_localdb: HashMap<EntryID, HashSet<ChunkOrBlockID>>,
}

impl Drop for ManifestStorage {
    fn drop(&mut self) {
        self.flush_cache_ahead_of_persistance()
            .expect("Cannot flush cache when ManifestStorage dropped");
    }
}

impl ManifestStorage {
    pub fn new(local_symkey: SecretKey, conn: SqliteConn, realm_id: EntryID) -> FSResult<Self> {
        let mut instance = Self {
            local_symkey,
            conn,
            realm_id,
            cache: HashMap::new(),
            cache_ahead_of_localdb: HashMap::new(),
        };
        instance.create_db()?;
        Ok(instance)
    }

    // Database initialization

    pub fn create_db(&mut self) -> FSResult<()> {
        sql_query(
            "CREATE TABLE IF NOT EXISTS vlobs (
              vlob_id BLOB PRIMARY KEY NOT NULL, -- UUID
              base_version INTEGER NOT NULL,
              remote_version INTEGER NOT NULL,
              need_sync BOOLEAN NOT NULL,
              blob BLOB NOT NULL
            );",
        )
        .execute(&mut self.conn)
        .map_err(|_| FSError::CreateTable("vlobs"))?;
        // Singleton storing the checkpoint
        sql_query(
            "CREATE TABLE IF NOT EXISTS realm_checkpoint (
              _id INTEGER PRIMARY KEY NOT NULL,
              checkpoint INTEGER NOT NULL
            );",
        )
        .execute(&mut self.conn)
        .map_err(|_| FSError::CreateTable("realm_checkpoint"))?;
        // Singleton storing the prevent_sync_pattern
        sql_query(
            "CREATE TABLE IF NOT EXISTS prevent_sync_pattern (
              _id INTEGER PRIMARY KEY NOT NULL,
              pattern TEXT NOT NULL,
              fully_applied BOOLEAN NOT NULL
            );",
        )
        .execute(&mut self.conn)
        .map_err(|_| FSError::CreateTable("prevent_sync_pattern"))?;

        let pattern = NewPreventSyncPattern {
            _id: 0,
            pattern: EMPTY_PATTERN,
            fully_applied: false,
        };
        // Set the default "prevent sync" pattern if it doesn't exist
        diesel::insert_or_ignore_into(prevent_sync_pattern::table)
            .values(pattern)
            .execute(&mut self.conn)
            .map_err(|_| FSError::InsertTable("prevent_sync_pattern: create_db"))?;

        Ok(())
    }

    pub fn clear_memory_cache(&mut self, flush: bool) -> FSResult<()> {
        if flush {
            self.flush_cache_ahead_of_persistance()?;
        }
        self.cache_ahead_of_localdb.clear();
        self.cache.clear();

        Ok(())
    }

    // "Prevent sync" pattern operations

    pub fn get_prevent_sync_pattern(&mut self) -> FSResult<(Regex, bool)> {
        let (re, fully_applied) = prevent_sync_pattern::table
            .select((
                prevent_sync_pattern::pattern,
                prevent_sync_pattern::fully_applied,
            ))
            .filter(prevent_sync_pattern::_id.eq(0))
            .first::<(String, _)>(&mut self.conn)
            .map_err(|_| FSError::QueryTable("prevent_sync_pattern: get_prevent_sync_pattern"))?;

        let re = Regex::new(&re)
            .map_err(|_| FSError::QueryTable("prevent_sync_pattern: corrupted pattern"))?;

        Ok((re, fully_applied))
    }

    pub fn set_prevent_sync_pattern(&mut self, pattern: &Regex) -> FSResult<()> {
        // Set the "prevent sync" pattern for the corresponding workspace

        // This operation is idempotent,
        // i.e it does not reset the `fully_applied` flag if the pattern hasn't changed.
        let pattern = pattern.as_str();
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
        .execute(&mut self.conn)
        .map_err(|_| FSError::UpdateTable("prevent_sync_pattern: set_prevent_sync_pattern"))?;

        Ok(())
    }

    pub fn mark_prevent_sync_pattern_fully_applied(&mut self, pattern: &Regex) -> FSResult<()> {
        // Mark the provided pattern as fully applied.

        // This is meant to be called after one made sure that all the manifests in the
        // workspace are compliant with the new pattern. The applied pattern is provided
        // as an argument in order to avoid concurrency issues.
        let pattern = pattern.as_str();
        diesel::update(
            prevent_sync_pattern::table.filter(
                prevent_sync_pattern::_id
                    .eq(0)
                    .and(prevent_sync_pattern::pattern.eq(pattern)),
            ),
        )
        .set(prevent_sync_pattern::fully_applied.eq(true))
        .execute(&mut self.conn)
        .map_err(|_| {
            FSError::UpdateTable("prevent_sync_pattern: mark_prevent_sync_pattern_fully_applied")
        })?;

        Ok(())
    }

    // Checkpoint operations

    pub fn get_realm_checkpoint(&mut self) -> i32 {
        realm_checkpoint::table
            .select(realm_checkpoint::checkpoint)
            .filter(realm_checkpoint::_id.eq(0))
            .first(&mut self.conn)
            .unwrap_or(0)
    }

    pub fn update_realm_checkpoint(
        &mut self,
        new_checkpoint: i32,
        changed_vlobs: &[(EntryID, i32)],
    ) -> FSResult<()> {
        let new_realm_checkpoint = NewRealmCheckpoint {
            _id: 0,
            checkpoint: new_checkpoint,
        };

        // https://github.com/diesel-rs/diesel/issues/1517
        // TODO: How to improve ?
        // It is difficult to build a raw sql query with bind in a for loop
        // Another solution is to query all data then insert
        for (id, version) in changed_vlobs {
            sql_query("UPDATE vlobs SET remote_version = ? WHERE vlob_id = ?;")
                .bind::<diesel::sql_types::Integer, _>(version)
                .bind::<diesel::sql_types::Binary, _>((**id).as_ref())
                .execute(&mut self.conn)
                .map_err(|_| FSError::UpdateTable("vlobs: update_realm_checkpoint"))?;
        }

        diesel::insert_into(realm_checkpoint::table)
            .values(&new_realm_checkpoint)
            .on_conflict(realm_checkpoint::_id)
            .do_update()
            .set(&new_realm_checkpoint)
            .execute(&mut self.conn)
            .map_err(|_| FSError::InsertTable("realm_checkpoint: update_realm_checkpoint"))?;

        Ok(())
    }

    pub fn get_need_sync_entries(&mut self) -> FSResult<(HashSet<EntryID>, HashSet<EntryID>)> {
        let mut remote_changes = HashSet::new();
        let mut local_changes =
            HashSet::<_, RandomState>::from_iter(self.cache.iter().filter_map(|(id, manifest)| {
                if manifest.need_sync() {
                    Some(*id)
                } else {
                    None
                }
            }));

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
            .load::<(Vec<u8>, bool, i32, i32)>(&mut self.conn)
            .map_err(|_| FSError::QueryTable("vlobs: get_need_sync_entries"))?
        {
            let manifest_id = EntryID::from(
                <[u8; 16]>::try_from(&manifest_id[..])
                    .map_err(|_| FSError::QueryTable("vlobs: corrupted manifest_id"))?,
            );

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

    pub fn get_manifest(&mut self, entry_id: EntryID) -> FSResult<LocalManifest> {
        // Look in cache first
        if let Some(manifest) = self.cache.get(&entry_id) {
            return Ok(manifest.clone());
        }

        // Look into the database
        let manifest = vlobs::table
            .select(vlobs::blob)
            .filter(vlobs::vlob_id.eq((*entry_id).as_ref()))
            .first::<Vec<u8>>(&mut self.conn)
            .map_err(|_| FSError::QueryTable("vlobs: get_manifest"))?;

        let manifest = LocalManifest::decrypt_and_load(&manifest, &self.local_symkey)
            .map_err(|_| FSError::Crypto(CryptoError::Decryption))?;

        // Fill the cache
        self.cache.insert(entry_id, manifest.clone());

        Ok(manifest)
    }

    pub fn set_manifest(
        &mut self,
        entry_id: EntryID,
        manifest: LocalManifest,
        cache_only: bool,
        removed_ids: Option<HashSet<ChunkOrBlockID>>,
    ) -> FSResult<()> {
        // Set the cache first
        self.cache.insert(entry_id, manifest);
        // Tag the entry as ahead of localdb
        if self.cache_ahead_of_localdb.get(&entry_id).is_none() {
            self.cache_ahead_of_localdb.insert(entry_id, HashSet::new());
        }

        // Cleanup
        if let Some(removed_ids) = &removed_ids {
            self.cache_ahead_of_localdb.insert(
                entry_id,
                self.cache_ahead_of_localdb.get(&entry_id).unwrap() | removed_ids,
            );
        }

        // Flush the cached value to the localdb
        if !cache_only {
            self.ensure_manifest_persistent(entry_id)?;
        }

        Ok(())
    }

    pub fn ensure_manifest_persistent(&mut self, entry_id: EntryID) -> FSResult<()> {
        match (
            self.cache_ahead_of_localdb.get(&entry_id),
            self.cache.get(&entry_id),
        ) {
            (Some(pending_chunk_ids), Some(manifest)) => {
                let ciphered = manifest.dump_and_encrypt(&self.local_symkey);
                let pending_chunk_ids = pending_chunk_ids
                    .iter()
                    .map(|chunk_id| chunk_id.as_ref())
                    .collect::<Vec<_>>();

                let vlob_id = (*entry_id).as_ref();

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
                    .bind::<diesel::sql_types::Integer, _>(manifest.base_version() as i32)
                    .bind::<diesel::sql_types::Integer, _>(manifest.base_version() as i32)
                    .bind::<diesel::sql_types::Binary, _>(vlob_id)
                    .execute(&mut self.conn)
                    .map_err(|_| FSError::InsertTable("vlobs: _ensure_manifest_persistent"))?;

                for pending_chunk_ids_chunk in pending_chunk_ids.chunks(SQLITE_MAX_VARIABLE_NUMBER)
                {
                    diesel::delete(
                        chunks::table.filter(chunks::chunk_id.eq_any(pending_chunk_ids_chunk)),
                    )
                    .execute(&mut self.conn)
                    .map_err(|_| FSError::DeleteTable("chunks: clear_manifest"))?;
                }

                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn flush_cache_ahead_of_persistance(&mut self) -> FSResult<()> {
        let keys = self
            .cache_ahead_of_localdb
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

    pub fn clear_manifest(&mut self, entry_id: EntryID) -> FSResult<()> {
        // Remove from cache
        let in_cache = self.cache.remove(&entry_id).is_some();

        // Remove from local database
        let deleted = diesel::delete(vlobs::table.filter(vlobs::vlob_id.eq((*entry_id).as_ref())))
            .execute(&mut self.conn)
            .map_err(|_| FSError::DeleteTable("vlobs: clear_manifest"))?
            > 0;

        if let Some(pending_chunk_ids) = self.cache_ahead_of_localdb.remove(&entry_id) {
            let pending_chunk_ids = pending_chunk_ids
                .iter()
                .map(ChunkOrBlockID::as_ref)
                .collect::<Vec<_>>();

            for pending_chunk_ids_chunk in pending_chunk_ids.chunks(SQLITE_MAX_VARIABLE_NUMBER) {
                diesel::delete(
                    chunks::table.filter(chunks::chunk_id.eq_any(pending_chunk_ids_chunk)),
                )
                .execute(&mut self.conn)
                .map_err(|_| FSError::DeleteTable("chunks: clear_manifest"))?;
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
    use parsec_api_crypto::HashDigest;
    use parsec_api_types::{BlockAccess, Blocksize, DateTime, DeviceID, FileManifest};
    use parsec_client_types::{Chunk, LocalFileManifest};

    use super::super::local_database::SqlitePool;
    use super::*;

    impl ManifestStorage {
        fn drop_db(&mut self) -> FSResult<()> {
            sql_query("DROP TABLE IF EXISTS vlobs;")
                .execute(&mut self.conn)
                .map_err(|_| FSError::DropTable("vlobs"))?;

            sql_query("DROP TABLE IF EXISTS realm_checkpoints;")
                .execute(&mut self.conn)
                .map_err(|_| FSError::DropTable("realm_checkpoints"))?;

            sql_query("DROP TABLE IF EXISTS prevent_sync_pattern;")
                .execute(&mut self.conn)
                .map_err(|_| FSError::DropTable("prevent_sync_pattern"))?;

            Ok(())
        }
    }

    #[test]
    fn manifest_storage() {
        let now = DateTime::now();
        let pool = SqlitePool::new("/tmp/manifest_storage.sqlite").unwrap();
        let conn = pool.conn().unwrap();
        let local_symkey = SecretKey::generate();
        let realm_id = EntryID::default();

        let mut manifest_storage = ManifestStorage::new(local_symkey, conn, realm_id).unwrap();
        manifest_storage.drop_db().unwrap();
        manifest_storage.create_db().unwrap();

        let (re, fully_applied) = manifest_storage.get_prevent_sync_pattern().unwrap();

        assert_eq!(re.as_str(), EMPTY_PATTERN);
        assert_eq!(fully_applied, false);

        manifest_storage
            .set_prevent_sync_pattern(&Regex::new(r"\z").unwrap())
            .unwrap();

        let (re, fully_applied) = manifest_storage.get_prevent_sync_pattern().unwrap();

        assert_eq!(re.as_str(), r"\z");
        assert_eq!(fully_applied, false);

        manifest_storage
            .mark_prevent_sync_pattern_fully_applied(&Regex::new(EMPTY_PATTERN).unwrap())
            .unwrap();

        let (re, fully_applied) = manifest_storage.get_prevent_sync_pattern().unwrap();

        assert_eq!(re.as_str(), r"\z");
        assert_eq!(fully_applied, false);

        let entry_id = EntryID::default();

        manifest_storage
            .update_realm_checkpoint(64, &[(entry_id, 2)])
            .unwrap();

        assert_eq!(manifest_storage.get_realm_checkpoint(), 64);

        let local_file_manifest = LocalManifest::File(LocalFileManifest {
            base: FileManifest {
                author: DeviceID::default(),
                timestamp: now,
                id: EntryID::default(),
                parent: EntryID::default(),
                version: 1,
                created: now,
                updated: now,
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
            updated: DateTime::now(),
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
