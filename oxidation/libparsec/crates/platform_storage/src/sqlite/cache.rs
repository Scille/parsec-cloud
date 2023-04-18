// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{
    collections::HashSet,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use diesel::{
    dsl::{count_star, not},
    sql_query, table, BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl,
    RunQueryDsl,
};

use libparsec_types::prelude::*;

use crate::{BlockStorage, Closable, StorageError};

use super::{
    chunk_storage_auto_impl::ChunkStorageAutoImpl,
    error::{self, ConfigurationError},
    tables::{chunks, remanence, NewChunk},
    LocalDatabase, VacuumMode,
};

pub struct SqliteCacheStorage {
    conn: LocalDatabase,
    device: Arc<LocalDevice>,
    cache_size: u64,
    /// Flag that enable/disable the block remanence.
    block_remanent_enabled: AtomicBool,
}

impl SqliteCacheStorage {
    /// Create a new cache storage using the sqlite driver.
    ///
    /// # Errors
    ///
    /// Will return an error if it fails to open the sqlite database or it can't initialize the database.
    pub async fn from_path(
        base_dir: &Path,
        relative_db_path: &Path,
        vacuum_mode: VacuumMode,
        device: Arc<LocalDevice>,
        cache_size: u64,
    ) -> Result<Self, ConfigurationError> {
        let conn = LocalDatabase::from_path(base_dir, relative_db_path, vacuum_mode).await?;

        Self::new(conn, device, cache_size).await
    }

    /// Create a new cache storage using the sqlite driver.
    ///
    /// # Errors
    ///
    /// Will return an error if it can initialize the database.
    pub async fn new(
        conn: LocalDatabase,
        device: Arc<LocalDevice>,
        cache_size: u64,
    ) -> Result<Self, ConfigurationError> {
        let instance = Self {
            conn,
            device,
            cache_size,
            block_remanent_enabled: AtomicBool::new(false),
        };

        instance.create_db().await?;
        instance.load_remanence_flag().await?;
        Ok(instance)
    }

    async fn create_db(&self) -> Result<(), ConfigurationError> {
        self.conn()
            .exec(|conn| {
                conn.exclusive_transaction(|conn| {
                    sql_query(std::include_str!("sql/create-chunks-table.sql")).execute(conn)?;
                    sql_query(std::include_str!("sql/create-remanence-table.sql")).execute(conn)
                })
            })
            .await?;
        Ok(())
    }

    async fn load_remanence_flag(&self) -> super::db::DatabaseResult<()> {
        let remanence_flag = self
            .conn
            .exec(move |conn| {
                remanence::table
                    .select(remanence::block_remanent)
                    .first(conn)
                    .optional()
            })
            .await?;

        self.block_remanent_enabled
            .store(remanence_flag.unwrap_or(false), Ordering::SeqCst);
        Ok(())
    }

    async fn set_block_remanence(&self, toggle: bool) -> super::db::DatabaseResult<()> {
        self.conn
            .exec(move |conn| {
                conn.exclusive_transaction(|conn| {
                    diesel::sql_query(
                        "INSERT OR REPLACE INTO remanence(_id, block_remanent) VALUES (0, ?)",
                    )
                    .bind::<diesel::sql_types::Bool, _>(toggle)
                    .execute(conn)
                })
            })
            .await?;
        self.block_remanent_enabled.store(toggle, Ordering::SeqCst);
        Ok(())
    }
}

impl ChunkStorageAutoImpl for SqliteCacheStorage {
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
impl BlockStorage for SqliteCacheStorage {
    fn cache_size(&self) -> u64 {
        self.cache_size
    }

    fn block_limit(&self) -> u64 {
        self.cache_size() / *DEFAULT_BLOCK_SIZE
    }

    async fn clear_all_blocks(&self) -> crate::Result<()> {
        self.conn
            .exec(|conn| diesel::delete(chunks::table).execute(conn))
            .await
            .map_err(error::Error::DatabaseError)?;
        Ok(())
    }

    async fn set_clean_block(
        &self,
        block_id: BlockID,
        raw: &[u8],
    ) -> crate::Result<HashSet<BlockID>> {
        let ciphered = self.local_symkey().encrypt(raw);
        let accessed_on = self.time_provider().now();

        // Insert the chunk
        self.conn
            .exec(move |conn| {
                let new_chunk = NewChunk {
                    chunk_id: (*block_id).as_ref(),
                    size: ciphered.len() as i64,
                    offline: false,
                    accessed_on: Some(accessed_on.into()),
                    data: ciphered.as_ref(),
                };

                conn.exclusive_transaction(|conn| {
                    diesel::insert_into(chunks::table)
                        .values(&new_chunk)
                        .on_conflict(chunks::chunk_id)
                        .do_update()
                        .set(&new_chunk)
                        .execute(conn)
                })
            })
            .await
            .map_err(error::Error::DatabaseError)?;

        // Perform cleanup if necessary
        self.cleanup().await
    }

    async fn cleanup(&self) -> crate::Result<HashSet<BlockID>> {
        // No cleanup for remanent storage.
        if self.is_block_remanent() {
            return Ok(HashSet::default());
        }

        // Count the chunks.
        let conn = &self.conn;
        let nb_blocks = conn
            .exec_with_error_handler(
                |conn| chunks::table.select(count_star()).first::<i64>(conn),
                |e| error::Error::Query {
                    table_name: "chunks",
                    error: e,
                },
            )
            .await?;

        let block_limit = self.block_limit() as i64;

        let extra_blocks = nb_blocks - block_limit;

        // No cleanup is needed
        if extra_blocks <= 0 {
            return Ok(HashSet::new());
        }

        // Remove the extra block plus 10% of the cache size, i.e 100 blocks
        let limit = extra_blocks + block_limit / 10;

        let sub_query = chunks::table
            .select(chunks::chunk_id)
            .order_by(chunks::accessed_on.asc())
            .limit(limit)
            .into_boxed();

        conn.exec(|conn| {
            conn.exclusive_transaction(|conn| {
                diesel::delete(chunks::table.filter(chunks::chunk_id.eq_any(sub_query)))
                    .returning(chunks::chunk_id)
                    .get_results(conn)
            })
        })
        .await
        .map_err(|e| StorageError::from(error::Error::DatabaseError(e)))
        .and_then(|res: Vec<Vec<u8>>| {
            res.into_iter()
                .map(|raw_chunk| {
                    BlockID::try_from(&raw_chunk[..]).map_err(|e| StorageError::InvalidEntryID {
                        used_as: "block_id",
                        error: e,
                    })
                })
                .collect::<crate::Result<HashSet<_>>>()
        })
    }

    fn is_block_remanent(&self) -> bool {
        self.block_remanent_enabled.load(Ordering::SeqCst)
    }

    async fn enable_block_remanence(&self) -> crate::Result<bool> {
        if self.is_block_remanent() {
            return Ok(false);
        }
        self.set_block_remanence(true)
            .await
            .map_err(error::Error::from)?;
        Ok(true)
    }

    async fn disable_block_remanence(&self) -> crate::Result<Option<HashSet<BlockID>>> {
        if !self.is_block_remanent() {
            return Ok(None);
        }
        self.set_block_remanence(false)
            .await
            .map_err(error::Error::from)?;

        self.cleanup().await.map(Some)
    }

    async fn clear_unreferenced_chunks(
        &self,
        chunk_ids: &[ChunkID],
        not_accessed_after: libparsec_types::DateTime,
    ) -> crate::Result<()> {
        table! {
            unreferenced_chunks(chunk_id) {
                chunk_id -> Binary,
            }
        }

        if chunk_ids.is_empty() {
            return Ok(());
        }

        let not_accessed_after = super::sql_types::DateTime::from(not_accessed_after);

        let raw_chunk_ids = chunk_ids.to_vec();
        self.conn
            .exec(move |conn| {
                conn.exclusive_transaction(|conn| {
                    // We have an exclusive access to the sqlite database, so nobody else is using/modifing the table `unreferenced_chunks`.
                    // We drop the table to prevent using an different table scheme or having garbage data from a previous call that failed to cleanup the table.
                    sql_query("DROP TABLE IF EXISTS unreferenced_chunks").execute(conn)?;
                    // This table only existe in this transaction.
                    // it will be drop at the end so we wont have collision problem.
                    // Even in the case of a failure since the transaction will rollback the change from before the creation of the table.
                    // Even tho the rollback fail, later call would be safe with the `Drop table ..` above.
                    sql_query(std::include_str!(
                        "sql/create-temp-unreferenced-chunks-table.sql"
                    ))
                    .execute(conn)?;

                    // We register `chunks` that need to be safe from the deletion at the later step.
                    raw_chunk_ids
                        .chunks(super::db::LOCAL_DATABASE_MAX_VARIABLE_NUMBER)
                        .try_for_each(|chunked| {
                            let entries = chunked
                                .iter()
                                .map(|v| unreferenced_chunks::chunk_id.eq(v.as_bytes()))
                                .collect::<Vec<_>>();

                            diesel::insert_or_ignore_into(unreferenced_chunks::table)
                                .values(&entries)
                                .execute(conn)
                                .and(Ok(()))
                        })?;

                    // Sub query to select all chunk_id associated with the current transaction.
                    let sub_query = unreferenced_chunks::table
                        .select(unreferenced_chunks::chunk_id)
                        .into_boxed();

                    // Remove unreferenced chunks from `chunks` table.
                    diesel::delete(
                        chunks::table.filter(
                            not(chunks::chunk_id.eq_any(sub_query))
                                .and(chunks::accessed_on.lt(not_accessed_after)),
                        ),
                    )
                    .execute(conn)?;

                    sql_query("DROP TABLE unreferenced_chunks").execute(conn)
                })
                .and(Ok(()))
            })
            .await
            .map_err(|e| StorageError::from(error::Error::DatabaseError(e)))
    }
}

#[async_trait::async_trait]
impl Closable for SqliteCacheStorage {
    async fn close(&self) {
        self.conn.close().await
    }
}
