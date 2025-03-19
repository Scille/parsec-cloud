// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Certificate storage relies on the upper layer to do the actual certification
// validation work and takes care of handling concurrency issues.
// Hence no unique violation should occur under normal circumstances here.

use libparsec_platform_async::future::TryFutureExt;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous},
    ConnectOptions, Connection, Row, SqliteConnection,
};
use std::{ops::DerefMut, path::Path};

use libparsec_types::prelude::*;

#[cfg(any(test, feature = "expose-test-methods"))]
use crate::workspace::{DebugBlock, DebugChunk, DebugDump, DebugVlob};
use crate::workspace::{
    MarkPreventSyncPatternFullyAppliedError, PopulateManifestOutcome, RawEncryptedBlock,
    RawEncryptedChunk, RawEncryptedManifest, UpdateManifestData,
};

use super::model::{
    get_workspace_cache_storage_db_relative_path, get_workspace_storage_db_relative_path,
};

async fn create_connection(db_path: &Path) -> anyhow::Result<SqliteConnection> {
    // Create parent directories if needed
    if let Some(parent) = db_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    // Create connection
    let mut conn = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .connect()
        .await?;

    // Manage auto-vacuum
    const FULL: u8 = 1;
    let auto_vacuum = sqlx::query("PRAGMA auto_vacuum")
        .fetch_one(&mut conn)
        .await?
        .try_get::<u8, _>(0)?;
    if auto_vacuum != FULL {
        sqlx::query("PRAGMA auto_vacuum = FULL")
            .execute(&mut conn)
            .await?;
        sqlx::query("VACUUM").execute(&mut conn).await?;
    }
    Ok(conn)
}

#[derive(Debug)]
pub(crate) struct PlatformWorkspaceStorage {
    cache_max_blocks: u64,
    conn: SqliteConnection,
    #[cfg(feature = "test-with-testbed")]
    path_info: super::testbed::DBPathInfo,
    // TODO: merge cache and data storage (simplify the code and improve performances)
    cache_conn: SqliteConnection,
    #[cfg(feature = "test-with-testbed")]
    cache_path_info: super::testbed::DBPathInfo,
}

impl PlatformWorkspaceStorage {
    pub async fn no_populate_start(
        data_base_dir: &Path,
        device: &LocalDevice,
        realm_id: VlobID,
        cache_max_blocks: u64,
    ) -> anyhow::Result<Self> {
        // 1) Open the database

        let db_relative_path = get_workspace_storage_db_relative_path(device, realm_id);
        let db_path = data_base_dir.join(&db_relative_path);

        #[cfg(feature = "test-with-testbed")]
        let path_info = super::testbed::DBPathInfo {
            data_base_dir: data_base_dir.to_owned(),
            db_relative_path: db_relative_path.to_owned(),
        };

        #[cfg(feature = "test-with-testbed")]
        let conn = super::testbed::maybe_open_sqlite_in_memory(&path_info).await;

        #[cfg(not(feature = "test-with-testbed"))]
        let conn: Option<SqliteConnection> = None;

        let mut conn = match conn {
            // In-memory database for testing
            Some(conn) => conn,
            // Actual production code: open the connection on disk
            None => create_connection(&db_path).await?,
        };

        // 1bis) Open the cache database

        let cache_db_relative_path = get_workspace_cache_storage_db_relative_path(device, realm_id);
        let cache_db_path = data_base_dir.join(&cache_db_relative_path);

        #[cfg(feature = "test-with-testbed")]
        let cache_path_info = super::testbed::DBPathInfo {
            data_base_dir: data_base_dir.to_owned(),
            db_relative_path: cache_db_relative_path.to_owned(),
        };

        #[cfg(feature = "test-with-testbed")]
        let cache_conn = super::testbed::maybe_open_sqlite_in_memory(&cache_path_info).await;

        #[cfg(not(feature = "test-with-testbed"))]
        let cache_conn: Option<SqliteConnection> = None;

        let mut cache_conn = match cache_conn {
            // In-memory database for testing
            Some(cache_conn) => cache_conn,
            // Actual production code: open the connection on disk
            None => create_connection(&cache_db_path).await?,
        };

        // 2) Initialize the database (if needed)

        super::model::sqlx_initialize_model_if_needed(&mut conn).await?;
        super::model::sqlx_initialize_model_if_needed(&mut cache_conn).await?;

        // 3) All done !

        let storage = Self {
            cache_max_blocks,
            conn,
            #[cfg(feature = "test-with-testbed")]
            path_info,
            cache_conn,
            #[cfg(feature = "test-with-testbed")]
            cache_path_info,
        };
        Ok(storage)
    }

    pub async fn stop(self) -> anyhow::Result<()> {
        #[cfg(feature = "test-with-testbed")]
        {
            if let Err(conn) =
                super::testbed::maybe_return_sqlite_in_memory_conn(&self.path_info, self.conn).await
            {
                // Testbed don't want to keep this connection, so we should close it
                conn.close().await.map_err(|e| anyhow::anyhow!(e))?;
            }

            if let Err(conn) = super::testbed::maybe_return_sqlite_in_memory_conn(
                &self.cache_path_info,
                self.cache_conn,
            )
            .await
            {
                // Testbed don't want to keep this connection, so we should close it
                conn.close().await.map_err(|e| anyhow::anyhow!(e))?;
            }
        }

        #[cfg(not(feature = "test-with-testbed"))]
        {
            self.conn.close().await.map_err(|e| anyhow::anyhow!(e))?;
            self.cache_conn
                .close()
                .await
                .map_err(|e| anyhow::anyhow!(e))?;
        }

        Ok(())
    }

    pub async fn get_realm_checkpoint(&mut self) -> anyhow::Result<IndexInt> {
        db_get_realm_checkpoint(&mut self.conn).await
    }

    pub async fn update_realm_checkpoint(
        &mut self,
        new_checkpoint: IndexInt,
        changed_vlobs: &[(VlobID, VersionInt)],
    ) -> anyhow::Result<()> {
        let mut transaction = self.conn.begin().await?;

        db_update_realm_checkpoint(&mut *transaction, new_checkpoint).await?;
        for (vlob_id, remote_version) in changed_vlobs {
            db_update_manifest_remote_version(&mut *transaction, *vlob_id, *remote_version).await?;
        }

        transaction.commit().await?;

        Ok(())
    }

    pub async fn get_inbound_need_sync(&mut self, limit: u32) -> anyhow::Result<Vec<VlobID>> {
        db_get_inbound_need_sync(&mut self.conn, limit).await
    }

    pub async fn get_outbound_need_sync(&mut self, limit: u32) -> anyhow::Result<Vec<VlobID>> {
        db_get_outbound_need_sync(&mut self.conn, limit).await
    }

    pub async fn get_manifest(
        &mut self,
        entry_id: VlobID,
    ) -> anyhow::Result<Option<RawEncryptedManifest>> {
        db_get_manifest(&mut self.conn, entry_id).await
    }

    pub async fn list_manifests(
        &mut self,
        offset: u32,
        limit: u32,
    ) -> anyhow::Result<Vec<RawEncryptedManifest>> {
        db_list_manifests(&mut self.conn, offset, limit).await
    }

    pub async fn get_chunk(
        &mut self,
        chunk_id: ChunkID,
    ) -> anyhow::Result<Option<RawEncryptedChunk>> {
        db_get_chunk(&mut self.conn, chunk_id).await
    }

    pub async fn get_block(
        &mut self,
        block_id: BlockID,
        now: DateTime,
    ) -> anyhow::Result<Option<RawEncryptedBlock>> {
        db_get_block_and_update_accessed_on(&mut self.cache_conn, block_id, now).await
    }

    pub async fn populate_manifest(
        &mut self,
        manifest: &UpdateManifestData,
    ) -> anyhow::Result<PopulateManifestOutcome> {
        db_populate_manifest(&mut self.conn, manifest).await
    }

    pub async fn update_manifest(&mut self, manifest: &UpdateManifestData) -> anyhow::Result<()> {
        db_update_manifest(&mut self.conn, manifest).await
    }

    pub async fn update_manifests(
        &mut self,
        manifests: impl Iterator<Item = UpdateManifestData>,
    ) -> anyhow::Result<()> {
        // Note transaction automatically rollbacks on drop
        let mut transaction = self.conn.begin().await?;

        for manifest in manifests {
            db_update_manifest(&mut *transaction, &manifest).await?;
        }

        transaction.commit().await?;
        Ok(())
    }

    pub async fn update_manifest_and_chunks(
        &mut self,
        manifest: &UpdateManifestData,
        new_chunks: impl Iterator<Item = (ChunkID, RawEncryptedChunk)>,
        removed_chunks: impl Iterator<Item = ChunkID>,
    ) -> anyhow::Result<()> {
        // Note transaction automatically rollbacks on drop
        let mut transaction = self.conn.begin().await?;

        db_update_manifest(&mut *transaction, manifest).await?;
        for (new_chunk_id, new_chunk_data) in new_chunks {
            db_insert_chunk(&mut *transaction, new_chunk_id, &new_chunk_data).await?;
        }
        for removed_chunk_id in removed_chunks {
            db_remove_chunk(&mut *transaction, removed_chunk_id).await?;
        }

        transaction.commit().await?;
        Ok(())
    }

    pub async fn set_chunk(&mut self, chunk_id: ChunkID, encrypted: &[u8]) -> anyhow::Result<()> {
        db_insert_chunk(&mut self.conn, chunk_id, encrypted).await
    }

    pub async fn set_block(
        &mut self,
        block_id: BlockID,
        encrypted: &[u8],
        now: DateTime,
    ) -> anyhow::Result<()> {
        let mut transaction = self.cache_conn.begin().await?;

        db_insert_block(&mut *transaction, block_id, encrypted, now).await?;
        may_cleanup_blocks(&mut transaction, self.cache_max_blocks).await?;

        transaction.commit().await?;
        Ok(())
    }

    pub async fn promote_chunk_to_block(
        &mut self,
        chunk_id: ChunkID,
        now: DateTime,
    ) -> anyhow::Result<()> {
        // TODO: have a single base is much better for this !
        let mut transaction = self.conn.begin().await?;
        let mut cache_transaction = self.cache_conn.begin().await?;

        let encrypted = match db_get_chunk(&mut *transaction, chunk_id).await? {
            Some(encrypted) => encrypted,
            // Nothing to promote, this should not occur under normal circumstances
            None => return Ok(()),
        };
        db_insert_block(&mut *cache_transaction, chunk_id.into(), &encrypted, now).await?;
        may_cleanup_blocks(&mut cache_transaction, self.cache_max_blocks).await?;
        db_remove_chunk(&mut *transaction, chunk_id).await?;

        cache_transaction.commit().await?;
        transaction.commit().await?;
        Ok(())
    }

    /// Only used for debugging tests
    #[cfg(any(test, feature = "expose-test-methods"))]
    pub async fn debug_dump(&mut self) -> anyhow::Result<DebugDump> {
        let checkpoint = self.get_realm_checkpoint().await?;

        // Vlobs

        let rows = sqlx::query(
            "SELECT \
                vlob_id, \
                need_sync, \
                base_version, \
                remote_version \
            FROM vlobs \
            ",
        )
        .fetch_all(&mut self.conn)
        .await?;

        let vlobs = rows
            .iter()
            .map(|row| {
                let id = VlobID::try_from(row.try_get::<&[u8], _>(0)?)
                    .map_err(|e| anyhow::anyhow!(e))?;
                let need_sync = row.try_get::<bool, _>(1)?;
                let base_version = row.try_get::<u32, _>(2)?;
                let remote_version = row.try_get::<u32, _>(3)?;

                Ok(DebugVlob {
                    id,
                    need_sync,
                    base_version,
                    remote_version,
                })
            })
            .collect::<anyhow::Result<_>>()?;

        // Chunks

        let rows = sqlx::query(
            "SELECT \
                chunk_id, \
                size, \
                offline \
            FROM chunks \
            ",
        )
        .fetch_all(&mut self.conn)
        .await?;

        let chunks = rows
            .iter()
            .map(|row| {
                let id = ChunkID::try_from(row.try_get::<&[u8], _>(0)?)
                    .map_err(|e| anyhow::anyhow!(e))?;
                let size = row.try_get::<u32, _>(1)?;
                let offline = row.try_get::<bool, _>(2)?;

                Ok(DebugChunk { id, size, offline })
            })
            .collect::<anyhow::Result<_>>()?;

        // Blocks

        let rows = sqlx::query(
            "SELECT \
                chunk_id, \
                size, \
                offline, \
                accessed_on
            FROM chunks \
            ",
        )
        .fetch_all(&mut self.cache_conn)
        .await?;

        let blocks = rows
            .iter()
            .map(|row| {
                let id = BlockID::try_from(row.try_get::<&[u8], _>(0)?)
                    .map_err(|e| anyhow::anyhow!(e))?;
                let size = row.try_get::<u32, _>(1)?;
                let offline = row.try_get::<bool, _>(2)?;
                let accessed_on =
                    DateTime::from_timestamp_micros(row.try_get::<i64, _>(3)?)?.to_rfc3339();

                Ok(DebugBlock {
                    id,
                    size,
                    offline,
                    accessed_on,
                })
            })
            .collect::<anyhow::Result<_>>()?;

        Ok(DebugDump {
            checkpoint,
            vlobs,
            chunks,
            blocks,
        })
    }

    pub async fn set_prevent_sync_pattern(
        &mut self,
        pattern: &PreventSyncPattern,
    ) -> anyhow::Result<bool> {
        let pattern = pattern.to_string();

        let mut transaction = self.conn.begin().await?;

        let fully_applied = db_update_prevent_sync_pattern(&mut transaction, &pattern).await?;

        transaction.commit().await?;

        Ok(fully_applied)
    }

    pub async fn get_prevent_sync_pattern(&mut self) -> anyhow::Result<(PreventSyncPattern, bool)> {
        db_get_prevent_sync_pattern(&mut self.conn)
            .await
            .and_then(|(pattern, fully_applied)| {
                PreventSyncPattern::from_regex(&pattern)
                    .map(|re| (re, fully_applied))
                    .map_err(anyhow::Error::from)
            })
    }

    pub async fn mark_prevent_sync_pattern_fully_applied(
        &mut self,
        pattern: &PreventSyncPattern,
    ) -> Result<(), MarkPreventSyncPatternFullyAppliedError> {
        let pattern = pattern.to_string();

        let mut transaction = self.conn.begin().await.map_err(anyhow::Error::from)?;

        db_set_prevent_sync_pattern_as_fully_applied(&mut transaction, &pattern).await?;

        transaction.commit().await.map_err(anyhow::Error::from)?;

        Ok(())
    }
}

async fn db_update_prevent_sync_pattern<T, E>(mut trans: T, pattern: &str) -> anyhow::Result<bool>
where
    T: AsMut<E>,
    for<'c> &'c mut E: sqlx::Executor<'c, Database = sqlx::Sqlite>,
{
    let res = sqlx::query(concat!(
        "UPDATE prevent_sync_pattern",
        " SET pattern = ?1, fully_applied = false",
        " WHERE _id = 0 AND pattern != ?1",
    ))
    .bind(pattern)
    .execute(trans.as_mut())
    .await?;

    if res.rows_affected() == 1 {
        Ok(false)
    } else {
        // No row was updated, the pattern was already set.
        db_get_fully_applied(trans.as_mut()).await
    }
}

async fn db_get_fully_applied(
    executor: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
) -> anyhow::Result<bool> {
    sqlx::query(concat!(
        "SELECT fully_applied",
        " FROM prevent_sync_pattern",
        " WHERE _id = 0",
    ))
    .fetch_one(executor)
    .await?
    .try_get(0)
    .map_err(anyhow::Error::from)
}

async fn db_get_prevent_sync_pattern(
    executor: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
) -> anyhow::Result<(String, bool)> {
    let row = sqlx::query(concat!(
        "SELECT pattern, fully_applied",
        " FROM prevent_sync_pattern",
        " WHERE _id = 0",
    ))
    .fetch_one(executor)
    .await?;

    let pattern = row.try_get::<String, _>(0)?;
    let fully_applied = row.try_get::<bool, _>(1)?;

    Ok((pattern, fully_applied))
}

async fn db_set_prevent_sync_pattern_as_fully_applied<T, E>(
    mut trans: T,
    pattern: &str,
) -> Result<(), MarkPreventSyncPatternFullyAppliedError>
where
    T: AsMut<E>,
    for<'c> &'c mut E: sqlx::Executor<'c, Database = sqlx::Sqlite>,
{
    let res = sqlx::query(concat!(
        "UPDATE prevent_sync_pattern",
        " SET fully_applied = true",
        " WHERE _id = 0 AND pattern = ?1",
    ))
    .bind(pattern)
    .execute(trans.as_mut())
    .await
    .map_err(anyhow::Error::from)?;

    if res.rows_affected() == 1 {
        Ok(())
    } else {
        // No row was updated, a different pattern is set in the database.
        Err(MarkPreventSyncPatternFullyAppliedError::PatternMismatch)
    }
}

pub async fn db_get_realm_checkpoint(
    executor: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
) -> anyhow::Result<IndexInt> {
    let row = sqlx::query(
        "SELECT checkpoint \
        FROM realm_checkpoint \
        WHERE _id = 0 \
        ",
    )
    .fetch_optional(executor)
    .await?;
    match row {
        None => Ok(0),
        Some(row) => {
            let checkpoint = row.try_get::<u32, _>(0)?;
            Ok(checkpoint as IndexInt)
        }
    }
}

pub async fn db_update_realm_checkpoint(
    executor: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    new_checkpoint: IndexInt,
) -> anyhow::Result<()> {
    sqlx::query(
        " \
        INSERT OR REPLACE INTO realm_checkpoint(_id, checkpoint) \
        VALUES (0, ?1) \
        ",
    )
    .bind(new_checkpoint as u32)
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn db_update_manifest_remote_version(
    executor: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    entry_id: VlobID,
    remote_version: VersionInt,
) -> anyhow::Result<()> {
    sqlx::query(
        "UPDATE vlobs \
        SET remote_version = ?1 \
        WHERE vlob_id = ?2 \
        ",
    )
    .bind(remote_version)
    .bind(entry_id.as_bytes())
    .execute(executor)
    .await?;

    Ok(())
}

async fn db_get_manifest(
    executor: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    entry_id: VlobID,
) -> anyhow::Result<Option<RawEncryptedManifest>> {
    let row = sqlx::query(
        "SELECT blob \
        FROM vlobs \
        WHERE vlob_id = ?1 \
        ",
    )
    .bind(entry_id.as_bytes())
    .fetch_optional(executor)
    .await?;

    match row {
        None => Ok(None),
        Some(row) => {
            let blob = row.try_get::<RawEncryptedManifest, _>(0)?;
            Ok(Some(blob))
        }
    }
}

async fn db_list_manifests(
    executor: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    offset: u32,
    limit: u32,
) -> anyhow::Result<Vec<RawEncryptedManifest>> {
    sqlx::query("SELECT blob FROM vlobs LIMIT ?1 OFFSET ?2")
        .bind(limit)
        .bind(offset)
        .fetch_all(executor)
        .map_ok(|vec| {
            vec.iter()
                .filter_map(|row| row.try_get(0).ok())
                .collect::<Vec<_>>()
        })
        .err_into::<anyhow::Error>()
        .await
}

pub async fn db_get_chunk(
    executor: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    chunk_id: ChunkID,
) -> anyhow::Result<Option<RawEncryptedChunk>> {
    let row = sqlx::query(
        "SELECT data \
        FROM chunks \
        WHERE chunk_id = ?1 \
        ",
    )
    .bind(chunk_id.as_bytes())
    .fetch_optional(executor)
    .await?;

    match row {
        Some(row) => {
            let blob = row.try_get::<RawEncryptedChunk, _>(0)?;
            Ok(Some(blob))
        }
        None => Ok(None),
    }
}

pub async fn db_get_block_and_update_accessed_on(
    executor: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    block_id: BlockID,
    timestamp: DateTime,
) -> anyhow::Result<Option<RawEncryptedBlock>> {
    let row = sqlx::query(
        "UPDATE chunks \
        SET accessed_on = ?1 \
        WHERE chunk_id = ?2 \
        RETURNING data \
        ",
    )
    .bind(timestamp.as_timestamp_micros())
    .bind(block_id.as_bytes())
    .fetch_optional(executor)
    .await?;

    match row {
        Some(row) => {
            let blob = row.try_get::<RawEncryptedBlock, _>(0)?;
            Ok(Some(blob))
        }
        None => Ok(None),
    }
}

pub async fn db_populate_manifest(
    executor: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    manifest: &UpdateManifestData,
) -> anyhow::Result<PopulateManifestOutcome> {
    let result = sqlx::query(
        " \
        INSERT OR IGNORE INTO vlobs(vlob_id, need_sync, blob, base_version, remote_version) \
        VALUES ( \
            ?1, \
            ?2, \
            ?3, \
            ?4, \
            ?5 \
        ) \
        ",
    )
    .bind(manifest.entry_id.as_bytes())
    .bind(manifest.need_sync)
    .bind(&manifest.encrypted)
    .bind(manifest.base_version)
    .bind(manifest.base_version) // Use base version as default for remote version
    .execute(executor)
    .await?;

    if result.rows_affected() != 0 {
        Ok(PopulateManifestOutcome::Stored)
    } else {
        Ok(PopulateManifestOutcome::AlreadyPresent)
    }
}

async fn db_update_manifest(
    executor: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    manifest: &UpdateManifestData,
) -> anyhow::Result<()> {
    sqlx::query(
        " \
        INSERT INTO vlobs(vlob_id, need_sync, blob, base_version, remote_version) \
        VALUES ( \
            ?1, \
            ?2, \
            ?3, \
            ?4, \
            ?5 \
        ) \
        ON CONFLICT DO UPDATE SET \
            need_sync = excluded.need_sync, \
            blob = excluded.blob, \
            base_version = excluded.base_version, \
            remote_version = MAX(excluded.remote_version, remote_version) \
        ",
    )
    .bind(manifest.entry_id.as_bytes())
    .bind(manifest.need_sync)
    .bind(&manifest.encrypted)
    .bind(manifest.base_version)
    .bind(manifest.base_version) // Use base version as default for remote version
    .execute(executor)
    .await?;

    Ok(())
}

async fn db_insert_chunk(
    executor: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    chunk_id: ChunkID,
    encrypted: &[u8],
) -> anyhow::Result<()> {
    sqlx::query(
        " \
        INSERT INTO chunks(chunk_id, data, size, offline) \
        VALUES ( \
            ?1, \
            ?2, \
            ?3, \
            ?4 \
        ) \
        ON CONFLICT DO UPDATE SET \
            size = excluded.size, \
            offline = excluded.offline, \
            data = excluded.data \
        ",
    )
    .bind(chunk_id.as_bytes())
    .bind(encrypted)
    // SQLite's INTEGER type is at most an 8 bytes signed, so we must use `i64` here
    .bind(encrypted.len() as i64)
    .bind(false)
    .execute(executor)
    .await?;

    Ok(())
}

async fn db_insert_block(
    executor: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    block_id: BlockID,
    encrypted: &[u8],
    accessed_on: DateTime,
) -> anyhow::Result<()> {
    sqlx::query(
        " \
        INSERT INTO chunks(chunk_id, data, size, offline, accessed_on) \
        VALUES ( \
            ?1, \
            ?2, \
            ?3, \
            ?4, \
            ?5 \
        ) \
        ON CONFLICT DO UPDATE SET \
            size = excluded.size, \
            offline = excluded.offline, \
            data = excluded.data, \
            accessed_on = excluded.accessed_on \
        ",
    )
    .bind(block_id.as_bytes())
    .bind(encrypted)
    // SQLite's INTEGER type is at most an 8 bytes signed, so we must use `i64` here
    .bind(encrypted.len() as i64)
    .bind(false)
    .bind(accessed_on.as_timestamp_micros())
    .execute(executor)
    .await?;

    Ok(())
}

async fn may_cleanup_blocks(
    executor: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    max_blocks: u64,
) -> anyhow::Result<()> {
    let nb_blocks = db_get_blocks_count(executor.deref_mut()).await?;

    let extra_blocks = nb_blocks.saturating_sub(max_blocks);

    // Cleanup is needed
    if extra_blocks > 0 {
        // Remove the extra block plus 10% of the cache size, i.e 100 blocks
        let to_remove = extra_blocks + max_blocks / 10;
        db_cleanup_blocks(executor.deref_mut(), to_remove).await?;
    }

    Ok(())
}

async fn db_get_blocks_count(
    executor: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
) -> anyhow::Result<u64> {
    let row = sqlx::query(
        " \
        SELECT COUNT(*) \
        FROM chunks \
        ",
    )
    .fetch_one(executor)
    .await?;

    // SQLite's INTEGER type is at most an 8 bytes signed, so we must use `i64` here
    let nb_blocks = row.try_get::<i64, _>(0)?;

    Ok(nb_blocks as u64)
}

async fn db_cleanup_blocks(
    executor: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    to_remove: u64,
) -> anyhow::Result<()> {
    sqlx::query(
        " \
        DELETE FROM chunks \
        WHERE chunk_id IN ( \
            SELECT chunk_id \
            FROM chunks \
            ORDER BY accessed_on ASC \
            LIMIT ?1 \
        ) \
        ",
    )
    // SQLite's INTEGER type is at most an 8 bytes signed, so we must use `i64` here
    .bind(to_remove as i64)
    .execute(executor)
    .await?;

    Ok(())
}

async fn db_remove_chunk(
    executor: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    chunk_id: ChunkID,
) -> anyhow::Result<()> {
    sqlx::query(
        " \
        DELETE FROM chunks \
        WHERE chunk_id = ?1 \
        ",
    )
    .bind(chunk_id.as_bytes())
    .execute(executor)
    .await?;

    Ok(())
}

async fn db_get_outbound_need_sync(
    executor: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    limit: u32,
) -> anyhow::Result<Vec<VlobID>> {
    let rows = sqlx::query(
        " \
        SELECT \
            vlob_id \
        FROM vlobs \
        WHERE need_sync = TRUE \
        LIMIT ?1 \
        ",
    )
    .bind(limit)
    .fetch_all(executor)
    .await?;

    rows.into_iter()
        .map(|row| VlobID::try_from(row.try_get::<&[u8], _>(0)?).map_err(|e| anyhow::anyhow!(e)))
        .collect()
}

async fn db_get_inbound_need_sync(
    executor: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    limit: u32,
) -> anyhow::Result<Vec<VlobID>> {
    let rows = sqlx::query(
        " \
        SELECT \
            vlob_id \
        FROM vlobs \
        WHERE base_version != remote_version \
        LIMIT ?1 \
        ",
    )
    .bind(limit)
    .fetch_all(executor)
    .await?;

    rows.into_iter()
        .map(|row| VlobID::try_from(row.try_get::<&[u8], _>(0)?).map_err(|e| anyhow::anyhow!(e)))
        .collect()
}
