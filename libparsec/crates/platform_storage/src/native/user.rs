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

use super::model::get_user_data_storage_db_relative_path;

#[derive(Debug)]
pub(crate) struct PlatformUserStorage {
    conn: SqliteConnection,
    realm_id: VlobID,
    #[cfg(feature = "test-with-testbed")]
    path_info: super::testbed::DBPathInfo,
}

impl PlatformUserStorage {
    pub async fn no_populate_start(
        data_base_dir: &Path,
        device: &LocalDevice,
    ) -> anyhow::Result<Self> {
        // 1) Open the database

        let db_relative_path = get_user_data_storage_db_relative_path(device);
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
                // TODO: configure the database with pragma stuff
                let sqlite_url = db_path.to_str().ok_or_else(|| {
                    anyhow::anyhow!(
                        "Invalid DB path (contains non-utf8 characters): {:?}",
                        db_path
                    )
                })?;

                let path = Path::new(sqlite_url);
                if let Some(parent) = path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }

                SqliteConnectOptions::new()
                    .filename(sqlite_url)
                    .create_if_missing(true)
                    .journal_mode(SqliteJournalMode::Wal)
                    .synchronous(SqliteSynchronous::Normal)
                    .connect()
                    .await?
            }
        };

        // 2) Initialize the database (if needed)

        super::model::sqlx_initialize_model_if_needed(&mut conn).await?;

        // 3) All done !

        let storage = Self {
            conn,
            realm_id: device.user_realm_id,
            #[cfg(feature = "test-with-testbed")]
            path_info,
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
                conn.close().await.map_err(|e| e.into())
            } else {
                Ok(())
            }
        }

        #[cfg(not(feature = "test-with-testbed"))]
        self.conn.close().await.map_err(|e| e.into())
    }

    pub async fn get_realm_checkpoint(&mut self) -> anyhow::Result<IndexInt> {
        let row = sqlx::query(
            "SELECT checkpoint \
            FROM realm_checkpoint \
            WHERE _id = 0 \
            ",
        )
        .fetch_optional(&mut self.conn)
        .await?;
        match row {
            None => Ok(0),
            Some(row) => {
                let checkpoint = row.try_get::<u32, _>(0)?;
                Ok(checkpoint as IndexInt)
            }
        }
    }

    pub async fn update_realm_checkpoint(
        &mut self,
        new_checkpoint: IndexInt,
        remote_user_manifest_version: Option<VersionInt>,
    ) -> anyhow::Result<()> {
        let mut transaction = self.conn.begin().await?;

        if let Some(remote_user_manifest_version) = remote_user_manifest_version {
            sqlx::query(
                "UPDATE vlobs \
                SET remote_version = ?1 \
                WHERE vlob_id = ?2 \
                ",
            )
            .bind(remote_user_manifest_version)
            .bind(self.realm_id.as_bytes())
            .execute(&mut *transaction)
            .await?;
        }

        sqlx::query(
            " \
            INSERT OR REPLACE INTO realm_checkpoint(_id, checkpoint) \
            VALUES (0, ?1) \
            ",
        )
        .bind(new_checkpoint as u32)
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(())
    }

    pub async fn get_user_manifest(&mut self) -> anyhow::Result<Option<Vec<u8>>> {
        let row = sqlx::query(
            "SELECT blob \
            FROM vlobs \
            WHERE vlob_id = ?1 \
            ",
        )
        .bind(self.realm_id.as_bytes())
        .fetch_optional(&mut self.conn)
        .await?;
        match row {
            None => Ok(None),
            Some(row) => {
                let blob = row.try_get::<Vec<u8>, _>(0)?;
                Ok(Some(blob))
            }
        }
    }

    pub async fn update_user_manifest(
        &mut self,
        encrypted: &[u8],
        need_sync: bool,
        base_version: VersionInt,
    ) -> anyhow::Result<()> {
        sqlx::query(
            " \
            INSERT INTO vlobs(vlob_id, blob, need_sync, base_version, remote_version) \
            VALUES ( \
                ?1, \
                ?2, \
                ?3, \
                ?4, \
                ?5 \
            ) \
            ON CONFLICT DO UPDATE SET \
                blob = excluded.blob, \
                need_sync = excluded.need_sync, \
                base_version = excluded.base_version, \
                remote_version = ( \
                    CASE WHEN remote_version > excluded.remote_version \
                    THEN remote_version \
                    ELSE excluded.remote_version \
                    END \
                ) \
            ",
        )
        .bind(self.realm_id.as_bytes())
        .bind(encrypted)
        .bind(need_sync)
        .bind(base_version)
        .bind(base_version)
        .execute(&mut self.conn)
        .await?;
        Ok(())
    }

    /// Only used for debugging tests
    #[allow(unused)]
    pub async fn debug_dump(&mut self) -> anyhow::Result<String> {
        let checkpoint = self.get_realm_checkpoint().await?;

        let rows = sqlx::query(
            "SELECT \
                vlob_id, \
                need_sync, \
                base_version, \
                remote_version, \
            FROM vlobs \
            ",
        )
        .fetch_all(&mut self.conn)
        .await?;

        let mut output = format!("checkpoint: {checkpoint}\nvlobs: [\n");
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

        Ok(output)
    }
}

pub async fn user_storage_non_speculative_init(
    data_base_dir: &Path,
    device: &LocalDevice,
) -> anyhow::Result<()> {
    // 1) Open & initialize the database

    let mut storage = PlatformUserStorage::no_populate_start(data_base_dir, device).await?;

    // 2) Populate the database with the user manifest

    let timestamp = device.now();
    let manifest = LocalUserManifest::new(
        device.device_id.clone(),
        timestamp,
        Some(device.user_realm_id),
        false,
    );
    let encrypted = manifest.dump_and_encrypt(&device.local_symkey);

    storage
        .update_user_manifest(&encrypted, manifest.need_sync, manifest.base.version)
        .await?;

    // 4) All done ! Don't forget to close the database before exiting ;-)

    storage.stop().await?;

    Ok(())
}
