// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use std::{path::Path, sync::Arc};

use libparsec_types::prelude::*;

use super::super::db::{DatabaseError, DatabaseResult, LocalDatabase, VacuumMode};
use super::super::model::get_workspace_cache_storage_db_relative_path;

#[derive(Debug)]
pub struct WorkspaceCacheStorage {
    pub realm_id: VlobID,
    pub device: Arc<LocalDevice>,
    db: LocalDatabase,
    cache_size: u64,
}

impl WorkspaceCacheStorage {
    pub async fn start(
        data_base_dir: &Path,
        cache_size: u64,
        device: Arc<LocalDevice>,
        realm_id: VlobID,
    ) -> anyhow::Result<Self> {
        // `maybe_populate_workspace_cache_storage` needs to start a `WorkspaceCacheStorage`,
        // leading to a recursive call which is not supported for async functions.
        // Hence `no_populate_start` which breaks the recursion.
        //
        // Also note we don't try to return the `WorkspaceCacheStorage` that has been
        // used during the populate as it would change the internal state of the
        // storage (typically caches) depending of if populate has been needed or not.

        #[cfg(feature = "test-with-testbed")]
        crate::testbed::maybe_populate_workspace_cache_storage(
            data_base_dir,
            device.clone(),
            realm_id,
        )
        .await;

        Self::no_populate_start(data_base_dir, cache_size, device, realm_id).await
    }

    fn cache_size(&self) -> u64 {
        self.cache_size
    }

    fn block_limit(&self) -> u64 {
        self.cache_size() / *DEFAULT_BLOCK_SIZE
    }

    pub(crate) async fn no_populate_start(
        data_base_dir: &Path,
        cache_size: u64,
        device: Arc<LocalDevice>,
        realm_id: VlobID,
    ) -> anyhow::Result<Self> {
        // 1) Open the database

        let db_relative_path = get_workspace_cache_storage_db_relative_path(&device, &realm_id);
        let db = LocalDatabase::from_path(data_base_dir, &db_relative_path, VacuumMode::default())
            .await?;

        // 2) Initialize the database (if needed)

        super::super::model::initialize_model_if_needed(&db).await?;

        // 3) All done !

        let storage = Self {
            realm_id,
            device,
            db,
            cache_size,
        };
        Ok(storage)
    }

    pub async fn stop(&self) {
        self.db.close().await
    }

    /// Returns the block data in cleartext or None if not present
    pub async fn get_block(&self, id: BlockID) -> Result<Option<Vec<u8>>, anyhow::Error> {
        match db_get_block(&self.db, &self.device, id).await {
            Ok(ok) => Ok(Some(ok)),
            Err(DatabaseError::Diesel(diesel::NotFound)) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    pub async fn set_block(
        &self,
        id: BlockID,
        cleartext_block: &[u8],
    ) -> Result<(), anyhow::Error> {
        db_set_block(&self.db, &self.device, id, cleartext_block)
            .await
            .map_err(|err| err.into())
    }

    pub async fn clear_block(&self, id: BlockID) -> Result<(), anyhow::Error> {
        db_clear_block(&self.db, id).await.map_err(|err| err.into())
    }

    // TODO: do we really need this one ?
    pub async fn get_local_block_ids(
        &self,
        _ids: Vec<BlockID>,
    ) -> Result<Vec<BlockID>, anyhow::Error> {
        todo!()
    }

    pub async fn cleanup(&self) -> Result<(), anyhow::Error> {
        db_cleanup(&self.db, self.block_limit())
            .await
            .map_err(|err| err.into())
    }

    pub async fn vacuum(&self) -> Result<(), anyhow::Error> {
        self.db.vacuum().await.map_err(|err| err.into())
    }
}

async fn db_get_block(
    db: &LocalDatabase,
    device: &LocalDevice,
    id: BlockID,
) -> DatabaseResult<Vec<u8>> {
    let id = *id;
    let accessed_on = device.time_provider.now().get_f64_with_us_precision();

    let ciphered: Vec<u8> = db
        .exec(move |conn| {
            // TODO: find a way to avoid having to write `accessed_on` on every read !
            conn.immediate_transaction(|conn| {
                use super::super::model::chunks;

                let updated_count =
                    diesel::update(chunks::table.filter(chunks::chunk_id.eq(id.as_ref())))
                        .set(chunks::accessed_on.eq(accessed_on))
                        .execute(conn)?;

                if updated_count == 0 {
                    // Block not found
                    return Err(diesel::NotFound);
                }

                chunks::table
                    .select(chunks::data)
                    .filter(chunks::chunk_id.eq(id.as_ref()))
                    .first(conn)
            })
        })
        .await?;

    device
        .local_symkey
        .decrypt(&ciphered)
        .map_err(DatabaseError::from)
}

async fn db_set_block(
    db: &LocalDatabase,
    device: &LocalDevice,
    id: BlockID,
    cleartext_block: &[u8],
) -> DatabaseResult<()> {
    let id = *id;
    let ciphered = device.local_symkey.encrypt(cleartext_block);
    let accessed_on = device.time_provider.now();

    db.exec(move |conn| {
        conn.immediate_transaction(|conn| {
            use super::super::model::{chunks, NewChunk};

            let new_chunk = NewChunk {
                chunk_id: id.as_ref(),
                size: ciphered.len() as i64,
                offline: false,
                accessed_on: Some(accessed_on.into()),
                data: ciphered.as_ref(),
            };

            diesel::insert_into(chunks::table)
                .values(&new_chunk)
                .on_conflict(chunks::chunk_id)
                .do_update()
                .set(&new_chunk)
                .execute(conn)
                .map(|_| ())
        })
    })
    .await
}

async fn db_clear_block(db: &LocalDatabase, id: BlockID) -> DatabaseResult<()> {
    let id = *id;

    db.exec(move |conn| {
        conn.immediate_transaction(|conn| {
            use super::super::model::chunks;

            diesel::delete(chunks::table.filter(chunks::chunk_id.eq(id.as_ref())))
                .execute(conn)
                .map(|_| ())
        })
    })
    .await
}

async fn db_cleanup(db: &LocalDatabase, block_limit: u64) -> DatabaseResult<()> {
    let block_limit = block_limit as i64;

    db.exec(move |conn| {
        use super::super::model::chunks;
        use diesel::dsl::count_star;

        // Count the chunks.
        let nb_blocks = chunks::table.select(count_star()).first::<i64>(conn)?;

        let extra_blocks = nb_blocks - block_limit;

        // No cleanup is needed
        if extra_blocks <= 0 {
            return Ok(());
        }

        // Remove the extra block plus 10% of the cache size, i.e. 100 blocks
        let limit = extra_blocks + block_limit / 10;

        // Given we open the write transaction only now, `nb_blocks` might be
        // wrong (i.e. a concurrent operation made the cleanup no longer needed).
        // However this is no big deal given the cleanup operation is idempotent.
        conn.immediate_transaction(|conn| {
            let sub_query = chunks::table
                .select(chunks::chunk_id)
                .order_by(chunks::accessed_on.asc())
                .limit(limit)
                .into_boxed();

            diesel::delete(chunks::table.filter(chunks::chunk_id.eq_any(sub_query)))
                .execute(conn)
                .map(|_| ())
        })
    })
    .await
}

#[cfg(test)]
#[path = "../../../tests/unit/native_workspace_cache_storage.rs"]
mod tests;
