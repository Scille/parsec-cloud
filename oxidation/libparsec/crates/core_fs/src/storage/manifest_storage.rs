// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use diesel::{
    sql_query, table, AsChangeset, BoolExpressionMethods, ExpressionMethods, Insertable, QueryDsl,
    RunQueryDsl,
};
use std::{
    collections::{hash_map::RandomState, HashMap, HashSet},
    sync::{Arc, Mutex, RwLock},
};

use libparsec_client_types::LocalManifest;
use libparsec_crypto::{CryptoError, SecretKey};
use libparsec_types::{BlockID, ChunkID, EntryID, Regex};
use platform_async::{future::TryFutureExt, futures::executor::block_on};

use crate::{
    error::{FSError, FSResult},
    storage::chunk_storage::chunks,
};
use local_db::{LocalDatabase, LOCAL_DATABASE_MAX_VARIABLE_NUMBER};

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
    conn: LocalDatabase,
    pub realm_id: EntryID,
    /// This cache contains all the manifests that have been set or accessed
    /// since the last call to `clear_memory_cache`
    caches: Arc<Mutex<HashMap<EntryID, Arc<RwLock<CacheEntry>>>>>,
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
        block_on(async { self.flush_cache_ahead_of_persistance().await })
            .expect("Cannot flush cache when ManifestStorage dropped");
    }
}

impl ManifestStorage {
    pub async fn new(
        local_symkey: SecretKey,
        realm_id: EntryID,
        conn: LocalDatabase,
    ) -> FSResult<Self> {
        let instance = Self {
            local_symkey,
            conn,
            realm_id,
            caches: Arc::new(Mutex::new(HashMap::default())),
        };
        instance.create_db().await?;
        Ok(instance)
    }

    /// Database initialization
    pub async fn create_db(&self) -> FSResult<()> {
        let conn = &self.conn;

        conn.exec_with_error_handler(
            |conn| {
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
            },
            |e| FSError::CreateTable(format!("vlobs {e}")),
        )
        .await?;
        // Singleton storing the checkpoint
        conn.exec_with_error_handler(
            |conn| {
                sql_query(
                    "CREATE TABLE IF NOT EXISTS realm_checkpoint (
                    _id INTEGER PRIMARY KEY NOT NULL,
                    checkpoint INTEGER NOT NULL
                    );",
                )
                .execute(conn)
            },
            |e| FSError::CreateTable(format!("realm_checkpoint {e}")),
        )
        .await?;
        // Singleton storing the prevent_sync_pattern
        conn.exec_with_error_handler(
            |conn| {
                sql_query(
                    "CREATE TABLE IF NOT EXISTS prevent_sync_pattern (
                    _id INTEGER PRIMARY KEY NOT NULL,
                    pattern TEXT NOT NULL,
                    fully_applied BOOLEAN NOT NULL
                    );",
                )
                .execute(conn)
            },
            |e| FSError::CreateTable(format!("prevent_sync_pattern {e}")),
        )
        .await?;

        let pattern = NewPreventSyncPattern {
            _id: 0,
            pattern: EMPTY_PATTERN,
            fully_applied: false,
        };
        // Set the default "prevent sync" pattern if it doesn't exist
        conn.exec(|conn| {
            diesel::insert_or_ignore_into(prevent_sync_pattern::table)
                .values(pattern)
                .execute(conn)
        })
        .await?;

        Ok(())
    }

    #[cfg(test)]
    async fn drop_db(&self) -> FSResult<()> {
        self.conn
            .exec(|conn| {
                sql_query("DROP TABLE IF EXISTS vlobs;").execute(conn)?;
                sql_query("DROP TABLE IF EXISTS realm_checkpoints;").execute(conn)?;
                sql_query("DROP TABLE IF EXISTS prevent_sync_pattern;").execute(conn)
            })
            .await?;

        Ok(())
    }

    pub async fn clear_memory_cache(&self, flush: bool) -> FSResult<()> {
        let drained_items = {
            let mut lock = self.caches.lock().expect("Mutex is poisoned");
            let drained = lock.drain();

            if flush {
                Some(drained.collect::<Vec<(EntryID, Arc<RwLock<CacheEntry>>)>>())
            } else {
                None
            }
        };
        if let Some(drained_items) = drained_items {
            for (entry_id, lock) in drained_items {
                self.ensure_manifest_persistent_lock(entry_id, lock).await?;
            }
        }
        Ok(())
    }

    // "Prevent sync" pattern operations

    #[cfg(test)]
    pub async fn get_prevent_sync_pattern(&self) -> FSResult<(Regex, bool)> {
        let (re, fully_applied) = get_prevent_sync_pattern_raw(&self.conn).await?;

        let re = Regex::from_regex_str(&re).map_err(|e| {
            FSError::QueryTable(format!("prevent_sync_pattern: corrupted pattern: {e}"))
        })?;

        Ok((re, fully_applied))
    }

    /// Set the "prevent sync" pattern for the corresponding workspace
    /// This operation is idempotent,
    /// i.e it does not reset the `fully_applied` flag if the pattern hasn't changed.
    pub async fn set_prevent_sync_pattern(&self, pattern: &Regex) -> FSResult<bool> {
        let pattern = pattern.to_string();

        self.conn
            .exec_with_error_handler(
                move |conn| {
                    diesel::update(
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
                    .execute(conn)
                },
                |e| {
                    FSError::UpdateTable(format!(
                        "prevent_sync_pattern: set_prevent_sync_pattern {e}"
                    ))
                },
            )
            .and_then(|_| async { Ok(get_prevent_sync_pattern_raw(&self.conn).await?.1) })
            .await
    }

    /// Mark the provided pattern as fully applied.
    /// This is meant to be called after one made sure that all the manifests in the
    /// workspace are compliant with the new pattern. The applied pattern is provided
    /// as an argument in order to avoid concurrency issues.
    pub async fn mark_prevent_sync_pattern_fully_applied(&self, pattern: &Regex) -> FSResult<bool> {
        let pattern = pattern.to_string();

        self.conn
            .exec_with_error_handler(
                move |conn| {
                    diesel::update(
                        prevent_sync_pattern::table.filter(
                            prevent_sync_pattern::_id
                                .eq(0)
                                .and(prevent_sync_pattern::pattern.eq(pattern)),
                        ),
                    )
                    .set(prevent_sync_pattern::fully_applied.eq(true))
                    .execute(conn)
                },
                |e| {
                    FSError::UpdateTable(format!(
                        "prevent_sync_pattern: mark_prevent_sync_pattern_fully_applied {e}"
                    ))
                },
            )
            .and_then(|_| async { Ok(get_prevent_sync_pattern_raw(&self.conn).await?.1) })
            .await
    }

    // Checkpoint operations

    pub async fn get_realm_checkpoint(&self) -> i64 {
        self.conn
            .exec(|conn| {
                realm_checkpoint::table
                    .select(realm_checkpoint::checkpoint)
                    .filter(realm_checkpoint::_id.eq(0))
                    .first(conn)
            })
            .await
            .unwrap_or(0)
    }

    pub async fn update_realm_checkpoint(
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

        for (id, version) in changed_vlobs.iter().copied() {
            self.conn
                .exec_with_error_handler(
                    move |conn| {
                        sql_query("UPDATE vlobs SET remote_version = ? WHERE vlob_id = ?;")
                            .bind::<diesel::sql_types::BigInt, _>(version)
                            .bind::<diesel::sql_types::Binary, _>((*id).as_ref())
                            .execute(conn)
                    },
                    |e| FSError::UpdateTable(format!("vlobs: update_realm_checkpoint {e}")),
                )
                .await?;
        }

        self.conn
            .exec(move |conn| {
                diesel::insert_into(realm_checkpoint::table)
                    .values(&new_realm_checkpoint)
                    .on_conflict(realm_checkpoint::_id)
                    .do_update()
                    .set(&new_realm_checkpoint)
                    .execute(conn)
            })
            .await?;

        Ok(())
    }

    pub async fn get_need_sync_entries(&self) -> FSResult<(HashSet<EntryID>, HashSet<EntryID>)> {
        let mut remote_changes = HashSet::new();
        let caches = self.caches.lock().expect("Mutex is poisoned").clone();
        let iter = caches.iter().filter_map(|(id, cache)| {
            if cache
                .read()
                .expect("RwLock is poisoned")
                .manifest
                .as_ref()
                .map(|manifest| manifest.need_sync())
                .unwrap_or_default()
            {
                Some(*id)
            } else {
                None
            }
        });
        let mut local_changes = HashSet::<_, RandomState>::from_iter(iter);

        let vlobs_that_need_sync = self
            .conn
            .exec_with_error_handler(
                |conn| {
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
                },
                |e| FSError::QueryTable(format!("vlobs: get_need_sync_entries {e}")),
            )
            .await?;

        for (manifest_id, need_sync, bv, rv) in vlobs_that_need_sync {
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
    ) -> Arc<RwLock<CacheEntry>> {
        self.caches
            .lock()
            .expect("Mutex is poisoned")
            .entry(entry_id)
            .or_default()
            .clone()
    }
    // Manifest operations

    pub fn get_manifest_in_cache(&self, entry_id: &EntryID) -> Option<LocalManifest> {
        let caches = self.caches.lock().expect("Mutex is poisoned");

        caches
            .get(entry_id)
            .and_then(|entry| entry.read().expect("RwLock is poisoned").manifest.clone())
    }

    pub async fn get_manifest(&self, entry_id: EntryID) -> FSResult<LocalManifest> {
        // Look in cache first
        let cache_entry = self.get_or_insert_default_cache_entry(entry_id);
        if let Some(manifest) = {
            cache_entry
                .read()
                .expect("RwLock is poisoned")
                .manifest
                .clone()
        } {
            return Ok(manifest);
        }

        // Look into the database
        let manifest = self
            .conn
            .exec(move |conn| {
                vlobs::table
                    .select(vlobs::blob)
                    .filter(vlobs::vlob_id.eq((*entry_id).as_ref()))
                    .first::<Vec<u8>>(conn)
            })
            .await
            .map_err(|_| FSError::LocalMiss(*entry_id))?;

        let manifest = LocalManifest::decrypt_and_load(&manifest, &self.local_symkey)
            .map_err(|_| FSError::Crypto(CryptoError::Decryption))?;

        let mut cache_unlock = cache_entry.write().expect("RwLock is poisoned");
        cache_unlock.manifest = Some(manifest.clone());

        Ok(manifest)
    }

    pub async fn set_manifest(
        &self,
        entry_id: EntryID,
        manifest: LocalManifest,
        cache_only: bool,
        removed_ids: Option<HashSet<ChunkOrBlockID>>,
    ) -> FSResult<()> {
        let cache_entry = self.get_or_insert_default_cache_entry(entry_id);

        {
            let mut cache_unlock = cache_entry.write().expect("RwLock is poisoned");

            cache_unlock.manifest = Some(manifest);
            let pending_chunk_ids = cache_unlock
                .pending_chunk_ids
                .get_or_insert(HashSet::default());
            if let Some(removed_ids) = &removed_ids {
                *pending_chunk_ids = (pending_chunk_ids as &HashSet<ChunkOrBlockID>) | removed_ids;
            }
        }

        // Flush the cached value to the localdb
        if !cache_only {
            self.ensure_manifest_persistent_lock(entry_id, cache_entry)
                .await?;
        }

        Ok(())
    }

    pub async fn ensure_manifest_persistent(&self, entry_id: EntryID) -> FSResult<()> {
        let cache_entry = {
            let cache_guard = self.caches.lock().expect("Mutex is poisoned");

            cache_guard.get(&entry_id).cloned()
        };

        if let Some(cache_entry) = cache_entry {
            self.ensure_manifest_persistent_lock(entry_id, cache_entry)
                .await
        } else {
            Ok(())
        }
    }

    async fn ensure_manifest_persistent_lock(
        &self,
        entry_id: EntryID,
        cache_lock: Arc<RwLock<CacheEntry>>,
    ) -> FSResult<()> {
        let stuff = {
            let cache = cache_lock.read().unwrap();

            cache
                .manifest
                .as_ref()
                .and_then(|manifest| {
                    cache
                        .pending_chunk_ids
                        .clone()
                        .map(|chunk_ids| (manifest, chunk_ids))
                })
                .map(|(manifest, chunk_ids)| {
                    (
                        manifest.dump_and_encrypt(&self.local_symkey),
                        manifest.need_sync(),
                        manifest.base_version() as i64,
                        Vec::from_iter(chunk_ids.into_iter()),
                    )
                })
        };

        if let Some((ciphered, need_sync, base_version, chunk_ids)) = stuff {
            self.conn.exec(move |conn| {
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
                        .bind::<diesel::sql_types::Bool, _>(need_sync)
                        .bind::<diesel::sql_types::BigInt, _>(base_version)
                        .bind::<diesel::sql_types::BigInt, _>(base_version)
                        .bind::<diesel::sql_types::Binary, _>(vlob_id)
                        .execute(conn)
            }).await?;

            for pending_ids in chunk_ids
                .chunks(LOCAL_DATABASE_MAX_VARIABLE_NUMBER)
                .map(|chunk| chunk.to_vec())
            {
                self.conn
                    .exec(move |conn| {
                        let ids = pending_ids.iter().map(ChunkOrBlockID::as_ref);
                        let query = chunks::table.filter(chunks::chunk_id.eq_any(ids));
                        let res: diesel::result::QueryResult<usize> =
                            diesel::delete(query).execute(conn);

                        res
                    })
                    .await?;
            }
            cache_lock
                .write()
                .expect("RwLock is poisoned")
                .pending_chunk_ids = None;
        }
        Ok(())
    }

    pub async fn flush_cache_ahead_of_persistance(&self) -> FSResult<()> {
        let items = self
            .caches
            .lock()
            .expect("Mutex is poisoned")
            .iter()
            .map(|(entry_id, entry)| (*entry_id, entry.clone()))
            .collect::<Vec<(EntryID, Arc<RwLock<CacheEntry>>)>>();
        for (entry_id, lock) in items {
            self.ensure_manifest_persistent_lock(entry_id, lock).await?;
        }
        Ok(())
    }

    /// This method is not used in the code base but it is still tested
    /// as it might come handy in a cleanup routine later
    pub async fn clear_manifest(&self, entry_id: EntryID) -> FSResult<()> {
        // Remove from cache
        let cache_entry = self.get_or_insert_default_cache_entry(entry_id);
        let (previous_manifest, previous_pending_chunk_ids) = {
            let mut cache_unlock = cache_entry.write().expect("RwLock is poisoned");

            (
                cache_unlock.manifest.take(),
                cache_unlock.pending_chunk_ids.take(),
            )
        };

        let res: FSResult<bool> = async {
            // Remove from local database

            let deleted = self
                .conn
                .exec(move |conn| {
                    let vlob_id = (*entry_id).as_ref();
                    diesel::delete(vlobs::table.filter(vlobs::vlob_id.eq(vlob_id))).execute(conn)
                })
                .await?
                > 0;

            if let Some(pending_chunk_ids) = previous_pending_chunk_ids.clone() {
                let pending_chunk_ids = pending_chunk_ids.into_iter().collect::<Vec<_>>();

                for chunked_ids in pending_chunk_ids
                    .chunks(LOCAL_DATABASE_MAX_VARIABLE_NUMBER)
                    .map(|chunks| chunks.to_vec())
                {
                    self.conn
                        .exec(move |conn| {
                            let ids = chunked_ids.iter().map(ChunkOrBlockID::as_ref);
                            let query = chunks::table.filter(chunks::chunk_id.eq_any(ids));
                            diesel::delete(query).execute(conn)
                        })
                        .await?;
                }
            }
            Ok(deleted)
        }
        .await;

        if let Ok(deleted) = res {
            let is_manifest_cached = previous_manifest.is_some();

            if !deleted && !is_manifest_cached {
                return Err(FSError::LocalMiss(*entry_id));
            }
        } else {
            // TODO: ANSWER WANTED: Do we want to rollback the cache entry when we fail to remove data from the database ?
            let mut cache_unlock = cache_entry.write().expect("RwLock is poisoned");

            cache_unlock.manifest = previous_manifest;
            cache_unlock.pending_chunk_ids = previous_pending_chunk_ids;
        }

        Ok(())
    }

    /// Return `true` is the given manifest identified by `entry_id` is present in the cache
    /// waiting to be written onto the database.
    ///
    /// For more information see [ManifestStorage::cache_ahead_of_localdb].
    pub async fn is_manifest_cache_ahead_of_persistance(&self, entry_id: &EntryID) -> bool {
        let cache_entry = {
            let caches_guard = self.caches.lock().expect("Mutex is poisoned");
            caches_guard.get(entry_id).cloned()
        };
        let result = cache_entry.as_ref().map(|entry| async {
            entry
                .read()
                .expect("RwLock is poisoned")
                .has_chunk_to_be_flush()
        });

        if let Some(fut) = result {
            fut.await
        } else {
            false
        }
    }
}

async fn get_prevent_sync_pattern_raw(
    connection: &LocalDatabase,
) -> Result<(String, bool), FSError> {
    connection
        .exec_with_error_handler(
            |conn| {
                prevent_sync_pattern::table
                    .select((
                        prevent_sync_pattern::pattern,
                        prevent_sync_pattern::fully_applied,
                    ))
                    .filter(prevent_sync_pattern::_id.eq(0))
                    .first::<(String, _)>(conn)
            },
            |e| {
                FSError::QueryTable(format!(
                    "prevent_sync_pattern: get_prevent_sync_pattern {e}"
                ))
            },
        )
        .await
}

#[cfg(test)]
mod tests {
    use libparsec_client_types::{Chunk, LocalFileManifest};
    use libparsec_crypto::{prelude::*, HashDigest};
    use libparsec_types::{BlockAccess, Blocksize, DateTime, DeviceID, FileManifest, Regex};

    use rstest::rstest;
    use tests_fixtures::{timestamp, tmp_path, TmpPath};

    use super::*;

    #[rstest]
    #[tokio::test]
    async fn manifest_storage(tmp_path: TmpPath, timestamp: DateTime) {
        let t1 = timestamp;
        let t2 = t1.add_us(1);
        let db_path = tmp_path.join("manifest_storage.sqlite");
        let conn = LocalDatabase::from_path(db_path.to_str().unwrap())
            .await
            .unwrap();
        let local_symkey = SecretKey::generate();
        let realm_id = EntryID::default();

        let manifest_storage = ManifestStorage::new(local_symkey, realm_id, conn)
            .await
            .unwrap();
        manifest_storage.drop_db().await.unwrap();
        manifest_storage.create_db().await.unwrap();

        let (re, fully_applied) = manifest_storage.get_prevent_sync_pattern().await.unwrap();

        assert_eq!(re.to_string(), EMPTY_PATTERN);
        assert!(!fully_applied);

        manifest_storage
            .set_prevent_sync_pattern(&Regex::from_regex_str(r"\z").unwrap())
            .await
            .unwrap();

        let (re, fully_applied) = manifest_storage.get_prevent_sync_pattern().await.unwrap();

        assert_eq!(re.to_string(), r"\z");
        assert!(!fully_applied);

        manifest_storage
            .mark_prevent_sync_pattern_fully_applied(&Regex::from_regex_str(EMPTY_PATTERN).unwrap())
            .await
            .unwrap();

        let (re, fully_applied) = manifest_storage.get_prevent_sync_pattern().await.unwrap();

        assert_eq!(re.to_string(), r"\z");
        assert!(!fully_applied);

        let entry_id = EntryID::default();

        manifest_storage
            .update_realm_checkpoint(64, &[(entry_id, 2)])
            .await
            .unwrap();

        assert_eq!(manifest_storage.get_realm_checkpoint().await, 64);

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
            .await
            .unwrap();

        assert_eq!(
            manifest_storage.get_manifest(entry_id).await.unwrap(),
            local_file_manifest
        );

        let (local_changes, remote_changes) =
            manifest_storage.get_need_sync_entries().await.unwrap();

        assert_eq!(local_changes, HashSet::new());
        assert_eq!(remote_changes, HashSet::new());
    }
}
