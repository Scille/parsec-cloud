// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use diesel::{
    sql_query, table, AsChangeset, BoolExpressionMethods, ExpressionMethods, Insertable, QueryDsl,
    RunQueryDsl,
};
use std::{
    collections::{hash_map::RandomState, HashMap, HashSet},
    sync::{Arc, Mutex},
};

use libparsec_client_types::{LocalDevice, LocalManifest};
use libparsec_crypto::{CryptoError, SecretKey};
use libparsec_platform_async::future::TryFutureExt;
use libparsec_types::{BlockID, ChunkID, EntryID, Regex};

use crate::{
    error::{FSError, FSResult},
    storage::chunk_storage::chunks,
};
use libparsec_platform_local_db::{LocalDatabase, LOCAL_DATABASE_MAX_VARIABLE_NUMBER};

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
    conn: Arc<LocalDatabase>,
    device: Arc<LocalDevice>,
    pub realm_id: EntryID,
    /// This cache contains all the manifests that have been set or accessed
    /// since the last call to `clear_memory_cache`
    caches: Mutex<HashMap<EntryID, Arc<Mutex<CacheEntry>>>>,
}

/// A cache entry that may contain a manifest and its pending chunk.
#[derive(Default)]
struct CacheEntry {
    /// The current manifest for the cache.
    // TODO: manifest are immutable, so have an `Arc` here would simplify thing
    manifest: Option<LocalManifest>,
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

impl ManifestStorage {
    pub async fn new(
        conn: Arc<LocalDatabase>,
        device: Arc<LocalDevice>,
        realm_id: EntryID,
    ) -> FSResult<Self> {
        let instance = Self {
            conn,
            device,
            realm_id,
            caches: Mutex::new(HashMap::default()),
        };
        instance.create_db().await?;
        Ok(instance)
    }

    fn local_symkey(&self) -> &SecretKey {
        &self.device.local_symkey
    }

    /// Close the connection to the database.
    /// Before that it will try to flush data the are stored in cache
    // TODO: Rework the close to take ownership over the `ManifestStorage`
    // Currently this is only used in Python's `WorkspaceStorage.run` teardown
    // routine that has been carefully crafted to ensure `clear_memory_cache`
    // and `close_connection` are done without concurrency call of other manifest
    // storage methods.
    pub async fn close_connection(&self) {
        let res = {
            let cache_guard = self.caches.lock().expect("Mutex is poisoned");
            cache_guard.iter().try_for_each(|(id, lock)| {
                let id = *id;
                let has_pending_chunks = {
                    lock.lock()
                        .expect("Mutex is poisoned")
                        .pending_chunk_ids
                        .is_some()
                };
                if has_pending_chunks {
                    Err(format!("Manifest {id} has pending chunk not saved"))
                } else {
                    Ok(())
                }
            })
        };
        if let Err(e) = res {
            log::warn!("Invalid state when closing the connection {e}")
        }
        self.conn.close().await;
    }

    /// Database initialization
    pub async fn create_db(&self) -> FSResult<()> {
        let conn = &self.conn;

        conn.exec_with_error_handler(
            |conn| {
                conn.exclusive_transaction(|conn| {
                    sql_query(std::include_str!("sql/create-vlobs-table.sql")).execute(conn)?;
                    sql_query(std::include_str!("sql/create-realm-checkpoint-table.sql"))
                        .execute(conn)?;
                    sql_query(std::include_str!(
                        "sql/create-prevent-sync-pattern-table.sql"
                    ))
                    .execute(conn)
                })
            },
            |e| {
                FSError::CreateTable(format!(
                    "Failed to create tables required for ManifesStorage {e}"
                ))
            },
        )
        .await?;

        let pattern = NewPreventSyncPattern {
            _id: 0,
            pattern: EMPTY_PATTERN,
            fully_applied: false,
        };
        // Set the default "prevent sync" pattern if it doesn't exist
        conn.exec(|conn| {
            conn.exclusive_transaction(|conn| {
                diesel::insert_or_ignore_into(prevent_sync_pattern::table)
                    .values(pattern)
                    .execute(conn)
            })
        })
        .await?;

        Ok(())
    }

    // TODO: `clear_memory_cache` should always flush it's local cache into the sqlite one.
    // We use `flush=false` only during test.
    // Can rename to `flush_and_clear_memory_cache` & `drop_memory_cache`
    // - flush_and_clear_memory_cache: `and_clear` part only useful for tests, but doesn't
    //   hurt prod code (given it is only used during teardown)
    // - drop_memory_cache: only available with test-utils feature
    pub async fn clear_memory_cache(&self, flush: bool) -> FSResult<()> {
        let drained_items = {
            let mut lock = self.caches.lock().expect("Mutex is poisoned");
            let drained = lock.drain();

            if flush {
                Some(drained.collect::<Vec<(EntryID, Arc<Mutex<CacheEntry>>)>>())
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

    /// Prevent sync pattern is a global configuration, hence at each startup `WorkspaceStorage`
    /// has to make sure this configuration is consistent with what is stored in the DB (see
    /// `set_prevent_sync_pattern`). Long story short, we never need to retrieve the pattern
    /// from the DB when in production. However this is still convenient for testing ;-)
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
    /// Return whether the pattern is fully applied
    pub async fn set_prevent_sync_pattern(&self, pattern: &Regex) -> FSResult<bool> {
        let pattern = pattern.to_string();

        self.conn
            .exec_with_error_handler(
                move |conn| {
                    conn.exclusive_transaction(|conn| {
                        // TODO: use returning close to remove the need for `get_prevent_sync_pattern_raw` ?
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
                    })
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
    /// Return whether the pattern is fully applied
    pub async fn mark_prevent_sync_pattern_fully_applied(&self, pattern: &Regex) -> FSResult<bool> {
        let pattern = pattern.to_string();

        self.conn
            .exec_with_error_handler(
                move |conn| {
                    conn.exclusive_transaction(|conn| {
                        diesel::update(
                            prevent_sync_pattern::table.filter(
                                prevent_sync_pattern::_id
                                    .eq(0)
                                    .and(prevent_sync_pattern::pattern.eq(pattern)),
                            ),
                        )
                        .set(prevent_sync_pattern::fully_applied.eq(true))
                        .execute(conn)
                    })
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

        let changed_vlobs = changed_vlobs.to_vec();
        self.conn
            .exec_with_error_handler(
                move |conn| {
                    conn.immediate_transaction(|conn| {
                        // Update version on vlobs.
                        // TODO: Can update the vlobs in batch => `UPDATE vlobs SET remote_version = :version WHERE vlob_id in [changed_vlobs[].id];`
                        changed_vlobs
                            .into_iter()
                            .map(|(id, version)| {
                                let id = id.as_bytes();
                                diesel::update(
                                    vlobs::table.filter(vlobs::vlob_id.eq(id.as_slice())),
                                )
                                .set(vlobs::remote_version.eq(version))
                                .execute(conn)
                            })
                            .collect::<diesel::result::QueryResult<Vec<usize>>>()?;
                        // Update realm checkpoint value.
                        diesel::insert_into(realm_checkpoint::table)
                            .values(&new_realm_checkpoint)
                            .on_conflict(realm_checkpoint::_id)
                            .do_update()
                            .set(&new_realm_checkpoint)
                            .execute(conn)
                            .and(Ok(()))
                    })
                },
                |e| FSError::UpdateTable(format!("Failed to udpate vlobs table: {e}")),
            )
            .await?;

        Ok(())
    }

    pub async fn get_need_sync_entries(&self) -> FSResult<(HashSet<EntryID>, HashSet<EntryID>)> {
        let mut remote_changes = HashSet::new();
        let caches = self.caches.lock().expect("Mutex is poisoned").clone();
        let iter = caches.iter().filter_map(|(id, cache)| {
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
            let manifest_id = EntryID::try_from(manifest_id.as_slice())
                .map_err(|e| FSError::QueryTable(format!("vlobs: corrupted manifest_id {e}")))?;

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
    fn get_or_insert_default_cache_entry(&self, entry_id: EntryID) -> Arc<Mutex<CacheEntry>> {
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
            .and_then(|entry| entry.lock().expect("Mutex is poisoned").manifest.clone())
    }

    pub async fn get_manifest(&self, entry_id: EntryID) -> FSResult<LocalManifest> {
        // Look in cache first
        let cache_entry = self.get_or_insert_default_cache_entry(entry_id);
        if let Some(manifest) = {
            cache_entry
                .lock()
                .expect("Mutex is poisoned")
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

        let manifest = LocalManifest::decrypt_and_load(&manifest, self.local_symkey())
            .map_err(|_| FSError::Crypto(CryptoError::Decryption))?;

        cache_entry.lock().expect("Mutex is poisoned").manifest = Some(manifest.clone());

        Ok(manifest)
    }

    pub fn set_manifest_cache_only(
        &self,
        entry_id: EntryID,
        manifest: LocalManifest,
        removed_ids: Option<HashSet<ChunkOrBlockID>>,
    ) {
        let cache_entry = self.get_or_insert_default_cache_entry(entry_id);

        let mut cache_unlock = cache_entry.lock().expect("Mutex is poisoned");

        cache_unlock.manifest = Some(manifest);
        let pending_chunk_ids = cache_unlock
            .pending_chunk_ids
            .get_or_insert(HashSet::default());
        if let Some(removed_ids) = &removed_ids {
            *pending_chunk_ids = (pending_chunk_ids as &HashSet<ChunkOrBlockID>) | removed_ids;
        }
    }

    pub async fn set_manifest(
        &self,
        entry_id: EntryID,
        manifest: LocalManifest,
        cache_only: bool,
        removed_ids: Option<HashSet<ChunkOrBlockID>>,
    ) -> FSResult<()> {
        self.set_manifest_cache_only(entry_id, manifest, removed_ids);

        // Flush the cached value to the localdb
        if !cache_only {
            self.ensure_manifest_persistent(entry_id).await?;
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
        cache_lock: Arc<Mutex<CacheEntry>>,
    ) -> FSResult<()> {
        // We retrieve some data that are required for the next step.
        let stuff = {
            let cache = cache_lock.lock().unwrap();

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
                        manifest.dump_and_encrypt(self.local_symkey()),
                        manifest.need_sync(),
                        manifest.base_version() as i64,
                        Vec::from_iter(chunk_ids.into_iter()),
                    )
                })
        };

        if let Some((ciphered, need_sync, base_version, chunk_ids)) = stuff {
            self.conn.exec(move |conn| {
                conn.exclusive_transaction(|conn| {
                    let vlob_id = (*entry_id).as_ref();

                    // TODO: replace with [replace_into](https://docs.diesel.rs/2.0.x/diesel/fn.replace_into.html)
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
                            .execute(conn)?;
                    chunk_ids
                        .chunks(LOCAL_DATABASE_MAX_VARIABLE_NUMBER)
                        .try_for_each(|chunked| {
                            let ids = chunked.iter().map(ChunkOrBlockID::as_ref);
                            let query = chunks::table.filter(chunks::chunk_id.eq_any(ids));
                            diesel::delete(query).execute(conn).and(Ok(()))
                        })
                })
            }).await?;

            cache_lock
                .lock()
                .expect("Mutex is poisoned")
                .pending_chunk_ids = None;
        }
        Ok(())
    }

    #[cfg(any(test, feature = "test-utils"))]
    pub async fn clear_manifest(&self, entry_id: EntryID) -> FSResult<()> {
        // Remove from cache
        let cache_entry = self.get_or_insert_default_cache_entry(entry_id);
        let (previous_manifest, previous_pending_chunk_ids) = {
            let mut cache_unlock = cache_entry.lock().expect("Mutex is poisoned");

            (
                cache_unlock.manifest.take(),
                cache_unlock.pending_chunk_ids.take(),
            )
        };

        let res = self
            .conn
            .exec(move |conn| {
                conn.exclusive_transaction(|conn| {
                    let vlob_id = entry_id.as_bytes().as_slice();
                    let deleted = diesel::delete(vlobs::table.filter(vlobs::vlob_id.eq(vlob_id)))
                        .execute(conn)?
                        > 0;

                    if let Some(pending_chunk_ids) = previous_pending_chunk_ids {
                        let pending_chunk_ids = pending_chunk_ids.into_iter().collect::<Vec<_>>();

                        pending_chunk_ids
                            .chunks(LOCAL_DATABASE_MAX_VARIABLE_NUMBER)
                            .try_for_each(|chunked| {
                                let ids = chunked.iter().map(ChunkOrBlockID::as_ref);
                                let query = chunks::table.filter(chunks::chunk_id.eq_any(ids));
                                diesel::delete(query).execute(conn).and(Ok(()))
                            })?;
                    }

                    Ok(deleted)
                })
            })
            .await;
        // Something unexpected happened with the database, given we have modified the cache it is
        // safer to panic instead of returning an error that could be seen as recoverable from the
        // caller point of view
        let deleted =
            res.expect("Failed to remove the manifest & its associated data in the local database");

        let is_manifest_cached = previous_manifest.is_some();

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
        let cache_entry = {
            let caches_guard = self.caches.lock().expect("Mutex is poisoned");
            caches_guard.get(entry_id).cloned()
        };
        cache_entry
            .as_ref()
            .map(|entry| {
                entry
                    .lock()
                    .expect("Mutex is poisoned")
                    .has_chunk_to_be_flush()
            })
            .unwrap_or(false)
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
    use std::{
        path::{Path, PathBuf},
        sync::Arc,
    };

    use libparsec_client_types::{Chunk, LocalFileManifest};
    use libparsec_crypto::{prelude::*, HashDigest};
    use libparsec_platform_local_db::VacuumMode;
    use libparsec_types::{BlockAccess, Blocksize, DateTime, DeviceID, FileManifest, Regex};

    use libparsec_tests_fixtures::{timestamp, TestbedScope};
    use rstest::rstest;

    use super::*;

    async fn manifest_storage_with_defaults(
        discriminant_dir: &Path,
        device: Arc<LocalDevice>,
    ) -> ManifestStorage {
        let db_relative_path = PathBuf::from("manifest_storage.sqlite");
        let conn =
            LocalDatabase::from_path(discriminant_dir, &db_relative_path, VacuumMode::default())
                .await
                .unwrap();
        let conn = Arc::new(conn);
        let realm_id = EntryID::default();
        ManifestStorage::new(conn, device, realm_id).await.unwrap()
    }

    #[rstest]
    #[test_log::test(tokio::test)]
    async fn prevent_sync_pattern() {
        TestbedScope::run("minimal", |env| async move {
            let alice = env.local_device("alice@dev1".parse().unwrap());
            let manifest_storage =
                manifest_storage_with_defaults(&env.discriminant_dir, alice.clone()).await;

            let (re, fully_applied) = manifest_storage.get_prevent_sync_pattern().await.unwrap();

            assert_eq!(re.to_string(), EMPTY_PATTERN);
            assert!(!fully_applied);

            let regex = Regex::from_regex_str(r"\z").unwrap();
            manifest_storage
                .set_prevent_sync_pattern(&regex)
                .await
                .unwrap();

            let (re, fully_applied) = manifest_storage.get_prevent_sync_pattern().await.unwrap();

            assert_eq!(re.to_string(), r"\z");
            assert!(!fully_applied);

            // Passing fully applied on a random pattern is a noop...

            manifest_storage
                .mark_prevent_sync_pattern_fully_applied(
                    &Regex::from_regex_str(EMPTY_PATTERN).unwrap(),
                )
                .await
                .unwrap();

            let (re, fully_applied) = manifest_storage.get_prevent_sync_pattern().await.unwrap();

            assert_eq!(re.to_string(), r"\z");
            assert!(!fully_applied);

            // ...unlike passing fully applied on the currently registered pattern

            manifest_storage
                .mark_prevent_sync_pattern_fully_applied(&regex)
                .await
                .unwrap();

            let (re, fully_applied) = manifest_storage.get_prevent_sync_pattern().await.unwrap();

            assert_eq!(re.to_string(), r"\z");
            assert!(fully_applied);
        })
        .await
    }

    #[rstest]
    #[test_log::test(tokio::test)]
    async fn realm_checkpoint() {
        TestbedScope::run("minimal", |env| async move {
            let alice = env.local_device("alice@dev1".parse().unwrap());
            let manifest_storage =
                manifest_storage_with_defaults(&env.discriminant_dir, alice.clone()).await;

            let entry_id = EntryID::default();

            manifest_storage
                .update_realm_checkpoint(64, &[(entry_id, 2)])
                .await
                .unwrap();

            assert_eq!(manifest_storage.get_realm_checkpoint().await, 64);
        })
        .await
    }

    #[rstest]
    #[test_log::test(tokio::test)]
    async fn set_manifest(timestamp: DateTime) {
        TestbedScope::run("minimal", |env| async move {
            let alice = env.local_device("alice@dev1".parse().unwrap());
            let manifest_storage =
                manifest_storage_with_defaults(&env.discriminant_dir, alice.clone()).await;

            let t1 = timestamp;
            let t2 = t1.add_us(1);

            let entry_id = EntryID::default();

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

            assert!(manifest_storage.get_manifest_in_cache(&entry_id).is_none());
            assert_eq!(
                manifest_storage.get_manifest(entry_id).await,
                Err(FSError::LocalMiss(*entry_id))
            );

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
        })
        .await
    }
}
