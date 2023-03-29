// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{
    collections::{HashMap, HashSet},
    path::Path,
    sync::{Arc, Mutex},
};

use diesel::{sql_query, BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};

use libparsec_crypto::{CryptoError, SecretKey};
use libparsec_platform_async::futures::TryFutureExt;
use libparsec_types::{ChunkID, EntryID, LocalDevice, LocalManifest, Regex, TimeProvider};

use crate::{Closable, ManifestStorage, NeedSyncEntries, StorageError};

#[cfg(any(test, feature = "test-utils"))]
use super::tables::chunks;
use super::{
    chunk_storage_auto_impl::{remove_chunks, ChunkStorageAutoImpl},
    db::{DatabaseError, LocalDatabase, VacuumMode},
    error::{self, ConfigurationError},
    tables::{
        prevent_sync_pattern, realm_checkpoint, vlobs, NewPreventSyncPattern, NewRealmCheckpoint,
    },
};

/// Do not match anything (https://stackoverflow.com/a/2302992/2846140)
const EMPTY_PATTERN: &str = r"^\b$";

pub struct SqliteDataStorage {
    conn: LocalDatabase,
    device: Arc<LocalDevice>,
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
    removed_ids: Option<HashSet<ChunkID>>,
}

impl CacheEntry {
    #[cfg(any(test, feature = "test-utils"))]
    /// Return `true` if we have chunk that need to be push to the local database.
    pub fn has_chunk_to_be_flush(&self) -> bool {
        self.removed_ids.is_some()
    }
}

impl SqliteDataStorage {
    /// Create a new data storage using the sqlite driver.
    ///
    /// # Errors
    ///
    /// Will return an error if it fails to open the sqlite database or it can't initialize it.
    pub async fn from_path(
        base_dir: &Path,
        db_path: &Path,
        vacuum_mode: VacuumMode,
        device: Arc<LocalDevice>,
    ) -> Result<Self, ConfigurationError> {
        let conn = LocalDatabase::from_path(base_dir, db_path, vacuum_mode).await?;
        Self::new(conn, device).await
    }

    /// Create a new data storage using the sqlite driver.
    ///
    /// # Errors
    ///
    /// Will return an error if it can't initialize it.
    pub async fn new(
        conn: LocalDatabase,
        device: Arc<LocalDevice>,
    ) -> Result<Self, ConfigurationError> {
        let instance = Self {
            conn,
            device,
            caches: Mutex::default(),
        };

        instance.create_db().await?;
        instance.basic_init().await?;
        Ok(instance)
    }

    async fn create_db(&self) -> Result<(), ConfigurationError> {
        self.conn
            .exec_with_error_handler(
                |conn| {
                    conn.exclusive_transaction(|conn| {
                        sql_query(std::include_str!("sql/create-vlobs-table.sql")).execute(conn)?;
                        sql_query(std::include_str!("sql/create-realm-checkpoint-table.sql"))
                            .execute(conn)?;
                        sql_query(std::include_str!(
                            "sql/create-prevent-sync-pattern-table.sql"
                        ))
                        .execute(conn)?;
                        sql_query(std::include_str!("sql/create-chunks-table.sql"))
                            .execute(conn)?;
                        sql_query(std::include_str!("sql/create-remanence-table.sql")).execute(conn)
                    })
                },
                ConfigurationError::CreateTables,
            )
            .await?;

        Ok(())
    }

    async fn basic_init(&self) -> Result<(), ConfigurationError> {
        let pattern = NewPreventSyncPattern {
            _id: 0,
            pattern: EMPTY_PATTERN,
            fully_applied: false,
        };
        // Set the default "prevent sync" pattern if it doesn't exist
        self.conn
            .exec(|conn| {
                conn.exclusive_transaction(|conn| {
                    diesel::insert_or_ignore_into(prevent_sync_pattern::table)
                        .values(pattern)
                        .execute(conn)
                })
            })
            .await?;

        Ok(())
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

    async fn ensure_manifest_persistent_lock(
        &self,
        entry_id: EntryID,
        entry: Arc<Mutex<CacheEntry>>,
    ) -> crate::Result<()> {
        // We retrieve some data that are required for the next step.
        let stuff = {
            let cache = entry.lock().unwrap();

            cache
                .manifest
                .as_ref()
                .and_then(|manifest| {
                    cache
                        .removed_ids
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

                    remove_chunks(conn, chunk_ids.iter().map(ChunkID::as_bytes))
                })
            }).await.map_err(error::Error::DatabaseError)?;

            entry.lock().expect("Mutex is poisoned").removed_ids = None;
        }
        Ok(())
    }

    async fn set_prevent_sync_pattern_internal(&self, pattern: String) -> crate::Result<bool> {
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
                |e| error::Error::Query {
                    table_name: "prevent_sync_pattern",
                    error: e,
                },
            )
            .and_then(|_| async { Ok(get_prevent_sync_pattern_raw(&self.conn).await?.1) })
            .await
            .map_err(StorageError::from)
    }
}

#[async_trait::async_trait]
impl ManifestStorage for SqliteDataStorage {
    async fn commit_deferred_manifest(&self) -> crate::Result<()> {
        let drained_items = {
            let mut lock = self.caches.lock().expect("Mutex is poisoned");
            let drained = lock.drain();

            drained.collect::<Vec<(EntryID, Arc<Mutex<CacheEntry>>)>>()
        };
        for (entry_id, lock) in drained_items {
            self.ensure_manifest_persistent_lock(entry_id, lock).await?;
        }
        Ok(())
    }

    #[cfg(any(test, feature = "test-utils"))]
    async fn drop_deferred_commit_manifest(&self) -> crate::Result<()> {
        self.caches.lock().expect("Mutex is poisoned").clear();

        Ok(())
    }

    #[cfg(test)]
    async fn get_prevent_sync_pattern(&self) -> crate::Result<(Regex, bool)> {
        let (re, fully_applied) = get_prevent_sync_pattern_raw(&self.conn).await?;

        let re =
            Regex::from_regex_str(&re).map_err(|e| StorageError::InvalidRegexPattern(re, e))?;

        Ok((re, fully_applied))
    }

    async fn set_prevent_sync_pattern(&self, pattern: &Regex) -> crate::Result<bool> {
        let pattern = pattern.to_string();

        self.set_prevent_sync_pattern_internal(pattern).await
    }

    #[cfg(any(test, feature = "test-utils"))]
    async fn set_raw_prevent_sync_pattern(&self, pattern: &str) -> crate::Result<bool> {
        self.set_prevent_sync_pattern_internal(pattern.to_string())
            .await
    }

    async fn mark_prevent_sync_pattern_fully_applied(
        &self,
        pattern: &Regex,
    ) -> crate::Result<bool> {
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
                |e| error::Error::Query {
                    table_name: "prevent_sync_pattern",
                    error: e,
                },
            )
            .and_then(|_| async { Ok(get_prevent_sync_pattern_raw(&self.conn).await?.1) })
            .await
            .map_err(StorageError::from)
    }

    async fn get_realm_checkpoint(&self) -> i64 {
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

    async fn update_realm_checkpoint(
        &self,
        new_checkpoint: i64,
        changed_vlobs: Vec<(EntryID, i64)>,
    ) -> crate::Result<()> {
        let new_realm_checkpoint = NewRealmCheckpoint {
            _id: 0,
            checkpoint: new_checkpoint,
        };

        // https://github.com/diesel-rs/diesel/issues/1517
        // TODO: How to improve ?
        // It is difficult to build a raw sql query with bind in a for loop
        // Another solution is to query all data then insert

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
                                diesel::update(vlobs::table.filter(vlobs::vlob_id.eq(id)))
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
                |e| error::Error::Query {
                    table_name: "vlobs",
                    error: e,
                },
            )
            .await?;

        Ok(())
    }

    async fn get_need_sync_entries(&self) -> crate::Result<NeedSyncEntries> {
        let mut remote_changes = HashSet::new();
        let caches = self.caches.lock().expect("Mutex is poisoned").clone();
        let mut local_changes = caches
            .iter()
            .filter_map(|(id, cache)| {
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
            })
            .collect::<HashSet<_>>();

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
                |e| error::Error::Query {
                    table_name: "vlobs",
                    error: e,
                },
            )
            .await?;

        for (manifest_id, need_sync, bv, rv) in vlobs_that_need_sync {
            let manifest_id = EntryID::try_from(manifest_id.as_slice()).map_err(|e| {
                StorageError::InvalidEntryID {
                    used_as: "manifest id",
                    error: e,
                }
            })?;

            if need_sync {
                local_changes.insert(manifest_id);
            }

            if bv != rv {
                remote_changes.insert(manifest_id);
            }
        }

        Ok(NeedSyncEntries {
            local_changes,
            remote_changes,
        })
    }

    fn get_manifest_in_cache(&self, entry_id: EntryID) -> Option<LocalManifest> {
        let caches = self.caches.lock().expect("Mutex is poisoned");

        caches
            .get(&entry_id)
            .and_then(|entry| entry.lock().expect("Mutex is poisoned").manifest.clone())
    }

    async fn get_manifest(&self, entry_id: EntryID) -> crate::Result<LocalManifest> {
        // Look in cache first
        let cache_entry = self.get_or_insert_default_cache_entry(entry_id);
        if let Some(manifest) = &cache_entry.lock().expect("Mutex is poisoned").manifest {
            return Ok(manifest.clone());
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
            .map_err(|e| match e {
                DatabaseError::Diesel(diesel::result::Error::NotFound) => {
                    StorageError::LocalEntryIDMiss(entry_id)
                }
                _ => StorageError::from(error::Error::from(e)),
            })?;

        let manifest = LocalManifest::decrypt_and_load(&manifest, self.local_symkey())
            .map_err(|_| StorageError::Crypto(CryptoError::Decryption))?;

        cache_entry.lock().expect("Mutex is poisoned").manifest = Some(manifest.clone());

        Ok(manifest)
    }

    fn set_manifest_deferred_commit(
        &self,
        entry_id: EntryID,
        manifest: LocalManifest,
        removed_ids: Option<HashSet<ChunkID>>,
    ) {
        let cache_entry = self.get_or_insert_default_cache_entry(entry_id);

        let mut cache_unlock = cache_entry.lock().expect("Mutex is poisoned");

        cache_unlock.manifest = Some(manifest);
        let pending_chunk_ids = cache_unlock.removed_ids.get_or_insert(HashSet::default());

        if let Some(removed_ids) = removed_ids {
            pending_chunk_ids.extend(removed_ids);
        }
    }

    async fn set_manifest(
        &self,
        entry_id: EntryID,
        manifest: LocalManifest,
        removed_ids: Option<HashSet<ChunkID>>,
    ) -> crate::Result<()> {
        self.set_manifest_deferred_commit(entry_id, manifest, removed_ids);

        self.ensure_manifest_persistent(entry_id).await?;

        Ok(())
    }

    async fn ensure_manifest_persistent(&self, entry_id: EntryID) -> crate::Result<()> {
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

    #[cfg(any(test, feature = "test-utils"))]
    async fn clear_manifest(&self, entry_id: EntryID) -> crate::Result<()> {
        // Remove from cache
        let cache_entry = self.get_or_insert_default_cache_entry(entry_id);
        let (previous_manifest, previous_pending_chunk_ids) = {
            let mut cache_unlock = cache_entry.lock().expect("Mutex is poisoned");

            (
                cache_unlock.manifest.take(),
                cache_unlock.removed_ids.take(),
            )
        };

        let res = self
            .conn
            .exec(move |conn| {
                conn.exclusive_transaction(|conn| {
                    let vlob_id = entry_id.as_bytes();
                    let deleted = diesel::delete(vlobs::table.filter(vlobs::vlob_id.eq(vlob_id)))
                        .execute(conn)?
                        > 0;

                    if let Some(pending_chunk_ids) = previous_pending_chunk_ids {
                        let pending_chunk_ids = pending_chunk_ids.into_iter().collect::<Vec<_>>();

                        pending_chunk_ids
                            .chunks(super::db::LOCAL_DATABASE_MAX_VARIABLE_NUMBER)
                            .try_for_each(|chunked| {
                                let ids = chunked.iter().map(ChunkID::as_bytes);
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
            return Err(StorageError::LocalEntryIDMiss(entry_id));
        }
        Ok(())
    }

    #[cfg(any(test, feature = "test-utils"))]
    fn is_manifest_cache_ahead_of_persistance(&self, entry_id: EntryID) -> bool {
        let cache_entry = {
            let caches_guard = self.caches.lock().expect("Mutex is poisoned");
            caches_guard.get(&entry_id).cloned()
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
) -> Result<(String, bool), error::Error> {
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
            |e| error::Error::Query {
                table_name: "prevent_sync_pattern",
                error: e,
            },
        )
        .await
}

impl ChunkStorageAutoImpl for SqliteDataStorage {
    fn local_symkey(&self) -> &SecretKey {
        &self.device.local_symkey
    }

    fn time_provider(&self) -> &TimeProvider {
        &self.device.time_provider
    }

    fn conn(&self) -> &LocalDatabase {
        &self.conn
    }
}

#[async_trait::async_trait]
impl Closable for SqliteDataStorage {
    async fn close(&self) {
        self.conn.close().await
    }
}
