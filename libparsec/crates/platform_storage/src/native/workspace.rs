// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Certificate storage relies on the upper layer to do the actual certification
// validation work and takes care of handling concurrency issues.
// Hence no unique violation should occur under normal circumstances here.

use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous},
    ConnectOptions, Connection, Row, SqliteConnection,
};
use std::path::Path;

use libparsec_types::prelude::*;

use crate::workspace::UpdateManifestData;

use super::model::{
    get_workspace_cache_storage_db_relative_path, get_workspace_storage_db_relative_path,
};

#[derive(Debug)]
pub(crate) struct PlatformWorkspaceStorage {
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
            None => {
                if let Some(parent) = db_path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }

                SqliteConnectOptions::new()
                    .filename(&db_path)
                    .create_if_missing(true)
                    .journal_mode(SqliteJournalMode::Wal)
                    .synchronous(SqliteSynchronous::Normal)
                    .connect()
                    .await?
            }
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
            None => {
                if let Some(parent) = cache_db_path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }

                SqliteConnectOptions::new()
                    .filename(&cache_db_path)
                    .create_if_missing(true)
                    .journal_mode(SqliteJournalMode::Wal)
                    .synchronous(SqliteSynchronous::Normal)
                    .connect()
                    .await?
            }
        };

        // 2) Initialize the database (if needed)

        super::model::sqlx_initialize_model_if_needed(&mut conn).await?;
        super::model::sqlx_initialize_model_if_needed(&mut cache_conn).await?;

        // 3) All done !

        let storage = Self {
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

    pub async fn get_manifest(&mut self, entry_id: VlobID) -> anyhow::Result<Option<Vec<u8>>> {
        db_get_manifest(&mut self.conn, entry_id).await
    }

    pub async fn get_chunk(&mut self, chunk_id: ChunkID) -> anyhow::Result<Option<Vec<u8>>> {
        match db_get_chunk(&mut self.conn, chunk_id).await? {
            Some(data) => Ok(Some(data)),
            None => db_get_chunk(&mut self.cache_conn, chunk_id).await,
        }
    }

    pub async fn get_block(&mut self, block_id: BlockID) -> anyhow::Result<Option<Vec<u8>>> {
        let chunk_id = ChunkID::from(block_id);
        db_get_chunk(&mut self.cache_conn, chunk_id).await
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
        new_chunks: impl Iterator<Item = (ChunkID, Vec<u8>)>,
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

    pub async fn set_block(&mut self, block_id: BlockID, encrypted: &[u8]) -> anyhow::Result<()> {
        let chunk_id = ChunkID::from(block_id);
        db_insert_chunk(&mut self.cache_conn, chunk_id, encrypted).await
    }

    /// Only used for debugging tests
    #[allow(unused)]
    pub async fn debug_dump(&mut self) -> anyhow::Result<String> {
        let checkpoint = self.get_realm_checkpoint().await?;
        let mut output = format!("checkpoint: {checkpoint}\n");

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

        output += "vlobs: [\n";
        for row in rows {
            let vlob_id =
                VlobID::try_from(row.try_get::<&[u8], _>(0)?).map_err(|e| anyhow::anyhow!(e))?;
            let need_sync = row.try_get::<bool, _>(1)?;
            let base_version = row.try_get::<u32, _>(2)?;
            let remote_version = row.try_get::<u32, _>(3)?;
            output += &format!(
                "{{\n\
                \tvlob_id: {vlob_id}\n\
                \tneed_sync: {need_sync}\n\
                \tbase_version: {base_version}\n\
                \tremote_version: {remote_version}\n\
            }},\n",
            );
        }
        output += "]\n";

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

        output += "chunks: [\n";
        for row in rows {
            let chunk_id =
                ChunkID::try_from(row.try_get::<&[u8], _>(0)?).map_err(|e| anyhow::anyhow!(e))?;
            let size = row.try_get::<u32, _>(1)?;
            let offline = row.try_get::<bool, _>(2)?;
            output += &format!(
                "{{\n\
                \tchunk_id: {chunk_id}\n\
                \tsize: {size}\n\
                \toffline: {offline}\n\
            }},\n",
            );
        }
        output += "]\n";

        Ok(output)
    }
}

pub async fn workspace_storage_non_speculative_init(
    data_base_dir: &Path,
    device: &LocalDevice,
    realm_id: VlobID,
) -> anyhow::Result<()> {
    // 1) Open & initialize the database

    let mut storage = PlatformWorkspaceStorage::no_populate_start(data_base_dir, device, realm_id)
        .await
        .map_err(|err| err.context("cannot initialize database"))?;

    // 2) Populate the database with the workspace manifest

    let timestamp = device.now();
    let manifest =
        LocalWorkspaceManifest::new(device.device_id.clone(), timestamp, Some(realm_id), false);

    storage
        .update_manifest(&UpdateManifestData {
            entry_id: manifest.base.id,
            encrypted: manifest.dump_and_encrypt(&device.local_symkey),
            need_sync: manifest.need_sync,
            base_version: manifest.base.version,
        })
        .await
        .map_err(|err| err.context("cannot store workspace manifest"))?;

    // 4) All done ! Don't forget to close the database before exiting ;-)

    storage.stop().await?;

    Ok(())
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
) -> anyhow::Result<Option<Vec<u8>>> {
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
            let blob = row.try_get::<Vec<u8>, _>(0)?;
            Ok(Some(blob))
        }
    }
}

pub async fn db_get_chunk(
    executor: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    id: ChunkID,
) -> anyhow::Result<Option<Vec<u8>>> {
    let row = sqlx::query(
        "SELECT data \
        FROM chunks \
        WHERE chunk_id = ?1 \
        ",
    )
    .bind(id.as_bytes())
    .fetch_optional(executor)
    .await?;

    match row {
        Some(row) => {
            let blob = row.try_get::<Vec<u8>, _>(0)?;
            Ok(Some(blob))
        }
        None => Ok(None),
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

pub async fn db_get_inbound_need_sync(
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

pub async fn db_get_outbound_need_sync(
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
