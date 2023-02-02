// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::Arc;

use diesel::{
    dsl::count_star, sql_query, table, AsChangeset, ExpressionMethods, Insertable, QueryDsl,
    RunQueryDsl,
};

use libparsec_crypto::SecretKey;
use libparsec_types::{ChunkID, TimeProvider, DEFAULT_BLOCK_SIZE};
use local_db::{LocalDatabase, LOCAL_DATABASE_MAX_VARIABLE_NUMBER};
use platform_async::future;

use crate::{
    error::{FSError, FSResult},
    extensions::CoalesceTotalSize,
};

table! {
    chunks (chunk_id) {
        chunk_id -> Binary, // UUID
        size -> BigInt,
        offline -> Bool,
        accessed_on -> Nullable<Double>, // Timestamp
        data -> Binary,
    }
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = chunks)]
struct NewChunk<'a> {
    pub chunk_id: &'a [u8],
    pub size: i64,
    pub offline: bool,
    pub accessed_on: Option<super::sql_types::DateTime>,
    pub data: &'a [u8],
}

#[async_trait::async_trait]
pub(crate) trait ChunkStorageTrait {
    fn conn(&self) -> &LocalDatabase;
    fn local_symkey(&self) -> &SecretKey;
    fn time_provider(&self) -> &TimeProvider;

    async fn create_db(&self) -> FSResult<()> {
        self.conn()
            .exec(|conn| {
                sql_query(
                    "CREATE TABLE IF NOT EXISTS chunks (
                    chunk_id BLOB PRIMARY KEY NOT NULL, -- UUID
                    size INTEGER NOT NULL,
                    offline BOOLEAN NOT NULL,
                    accessed_on REAL, -- Timestamp
                    data BLOB NOT NULL
                );",
                )
                .execute(conn)
            })
            .await?;
        Ok(())
    }

    async fn close_connection(&self) -> FSResult<()> {
        self.conn().close();
        Ok(())
    }

    #[cfg(test)]
    async fn drop_db(&self) -> FSResult<()> {
        self.conn()
            .exec(|conn| sql_query("DROP TABLE IF EXISTS chunks;").execute(conn))
            .await?;
        Ok(())
    }

    // Size and chunks

    async fn get_nb_blocks(&self) -> FSResult<i64> {
        self.conn()
            .exec_with_error_handler(
                |conn| chunks::table.select(count_star()).first(conn),
                |e| FSError::QueryTable(format!("chunks: get_nb_blocks {e}")),
            )
            .await
    }

    async fn get_total_size(&self) -> FSResult<i64> {
        let conn = self.conn();
        conn.exec_with_error_handler(
            |conn| {
                chunks::table
                    .select(CoalesceTotalSize::default())
                    .first(conn)
            },
            |e| FSError::QueryTable(format!("chunks: get_total_size {e}")),
        )
        .await
    }

    async fn is_chunk(&self, chunk_id: ChunkID) -> FSResult<bool> {
        self.conn()
            .exec_with_error_handler(
                move |conn| {
                    chunks::table
                        .select(count_star())
                        .filter(chunks::chunk_id.eq((*chunk_id).as_ref()))
                        .first::<i64>(conn)
                        .map(|res| res > 0)
                },
                |e| FSError::QueryTable(format!("chunks: is_chunk {e}")),
            )
            .await
    }

    async fn get_local_chunk_ids(&self, chunk_ids: &[ChunkID]) -> FSResult<Vec<ChunkID>> {
        let bytes_id_list = chunk_ids
            .iter()
            .map(|chunk_id| (**chunk_id).as_ref().to_vec())
            .collect::<Vec<_>>();

        let mut res = Vec::with_capacity(chunk_ids.len());

        let conn = self.conn();
        // The number of loop iteration is expected to be rather low:
        // typically considering 4ko per chunk (i.e. the size of the buffer the
        // Linux Kernel send us through Fuse), each query could perform ~4mo of data.

        // If performance ever becomes an issue, we could further optimize this by
        // having the caller filter the chunks: a block containing multiple chunk
        // means those correspond to local modification and hence can be filtered out.
        // With this we would only provide the chunks corresponding to a synced block
        // which are 512ko big, so each query performs up to 512Mo !
        let futures = bytes_id_list
            .chunks(LOCAL_DATABASE_MAX_VARIABLE_NUMBER)
            .map(|bytes_id_list_chunk| {
                let query = chunks::table
                    .select(chunks::chunk_id)
                    .filter(chunks::chunk_id.eq_any(bytes_id_list_chunk.to_vec()));

                conn.exec_with_error_handler(
                    move |conn| query.load::<Vec<u8>>(conn),
                    |e| FSError::QueryTable(format!("chunks: get_local_chunk_ids {e}")),
                )
            });
        for chunks in future::join_all(futures).await {
            let mut chunks = chunks.and_then(|chunk| {
                chunk
                    .into_iter()
                    .map(|chunk_id| {
                        <[u8; 16]>::try_from(&chunk_id[..]).map_err(|_| {
                            FSError::QueryTable(format!("chunks: corrupted chunk_id {chunk_id:?}"))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .map(|chunk| {
                        chunk
                            .into_iter()
                            .map(ChunkID::from)
                            .collect::<Vec<ChunkID>>()
                    })
            })?;

            res.append(&mut chunks)
        }

        Ok(res)
    }

    async fn get_chunk(&self, chunk_id: ChunkID) -> FSResult<Vec<u8>> {
        let accessed_on = self.time_provider().now().get_f64_with_us_precision();

        let conn = self.conn();

        let changes = conn
            .exec_with_error_handler(
                move |conn| {
                    diesel::update(chunks::table.filter(chunks::chunk_id.eq((*chunk_id).as_ref())))
                        .set(chunks::accessed_on.eq(accessed_on))
                        .execute(conn)
                },
                |e| FSError::UpdateTable(format!("chunks: get_chunk {e}")),
            )
            .await?
            > 0;

        if !changes {
            return Err(FSError::LocalMiss(*chunk_id));
        }

        let ciphered = conn
            .exec_with_error_handler(
                move |conn| {
                    chunks::table
                        .select(chunks::data)
                        .filter(chunks::chunk_id.eq((*chunk_id).as_ref()))
                        .first::<Vec<u8>>(conn)
                },
                |e| FSError::QueryTable(format!("chunks: get_chunk {e}")),
            )
            .await?;

        Ok(self.local_symkey().decrypt(&ciphered)?)
    }

    async fn set_chunk(&self, chunk_id: ChunkID, raw: &[u8]) -> FSResult<()> {
        let ciphered = self.local_symkey().encrypt(raw);
        let accessed_on = self.time_provider().now();

        self.conn()
            .exec(move |conn| {
                let new_chunk = NewChunk {
                    chunk_id: (*chunk_id).as_ref(),
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
            })
            .await?;
        Ok(())
    }

    async fn clear_chunk(&self, chunk_id: ChunkID) -> FSResult<()> {
        let changes = self
            .conn()
            .exec(move |conn| {
                diesel::delete(chunks::table.filter(chunks::chunk_id.eq((*chunk_id).as_ref())))
                    .execute(conn)
            })
            .await?
            > 0;

        if !changes {
            return Err(FSError::LocalMiss(*chunk_id));
        }

        Ok(())
    }

    async fn run_vacuum(&self) -> FSResult<()> {
        self.conn()
            .exec_with_error_handler(
                |conn| sql_query("VACUUM;").execute(conn),
                |e| FSError::Vacuum(e.to_string()),
            )
            .await?;
        Ok(())
    }
}

#[async_trait::async_trait]
pub(crate) trait BlockStorageTrait: ChunkStorageTrait {
    fn cache_size(&self) -> u64;

    // Garbage collection

    fn block_limit(&self) -> u64 {
        self.cache_size() / *DEFAULT_BLOCK_SIZE
    }

    async fn clear_all_blocks(&self) -> FSResult<()> {
        self.conn()
            .exec(|conn| diesel::delete(chunks::table).execute(conn))
            .await?;
        Ok(())
    }

    // Upgraded set method

    async fn set_chunk_upgraded(&self, chunk_id: ChunkID, raw: &[u8]) -> FSResult<()> {
        let ciphered = self.local_symkey().encrypt(raw);
        let accessed_on = self.time_provider().now();

        // Insert the chunk
        self.conn()
            .exec(move |conn| {
                let new_chunk = NewChunk {
                    chunk_id: (*chunk_id).as_ref(),
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
            })
            .await?;

        // Perform cleanup if necessary
        self.cleanup().await
    }

    async fn cleanup(&self) -> FSResult<()> {
        // Count the chunks
        let conn = self.conn();
        let nb_blocks = conn
            .exec_with_error_handler(
                |conn| chunks::table.select(count_star()).first::<i64>(conn),
                |e| FSError::QueryTable(format!("chunks: cleanup {e}")),
            )
            .await?;

        let block_limit = self.block_limit() as i64;

        let extra_blocks = nb_blocks - block_limit;

        // No cleanup is needed
        if extra_blocks <= 0 {
            return Ok(());
        }

        // Remove the extra block plus 10% of the cache size, i.e 100 blocks
        let limit = extra_blocks + block_limit / 10;

        let sub_query = chunks::table
            .select(chunks::chunk_id)
            .order_by(chunks::accessed_on.asc())
            .limit(limit)
            .into_boxed();

        conn.exec(|conn| {
            diesel::delete(chunks::table.filter(chunks::chunk_id.eq_any(sub_query))).execute(conn)
        })
        .await?;

        Ok(())
    }
}

// Interface to access the local chunks of data
#[derive(Clone)]
pub(crate) struct ChunkStorage {
    conn: Arc<LocalDatabase>,
    local_symkey: SecretKey,
    time_provider: TimeProvider,
}

impl ChunkStorageTrait for ChunkStorage {
    fn conn(&self) -> &LocalDatabase {
        &self.conn
    }
    fn local_symkey(&self) -> &SecretKey {
        &self.local_symkey
    }
    fn time_provider(&self) -> &TimeProvider {
        &self.time_provider
    }
}

impl ChunkStorage {
    pub async fn new(
        local_symkey: SecretKey,
        conn: Arc<LocalDatabase>,
        time_provider: TimeProvider,
    ) -> FSResult<Self> {
        let instance = Self {
            conn,
            local_symkey,
            time_provider,
        };
        instance.create_db().await?;
        Ok(instance)
    }
}

// Interface for caching the data blocks.
#[derive(Clone)]
pub(crate) struct BlockStorage {
    conn: Arc<LocalDatabase>,
    local_symkey: SecretKey,
    cache_size: u64,
    time_provider: TimeProvider,
}

impl ChunkStorageTrait for BlockStorage {
    fn conn(&self) -> &LocalDatabase {
        &self.conn
    }
    fn local_symkey(&self) -> &SecretKey {
        &self.local_symkey
    }
    fn time_provider(&self) -> &TimeProvider {
        &self.time_provider
    }
}

impl BlockStorageTrait for BlockStorage {
    fn cache_size(&self) -> u64 {
        self.cache_size
    }
}

impl BlockStorage {
    pub async fn new(
        local_symkey: SecretKey,
        conn: Arc<LocalDatabase>,
        cache_size: u64,
        time_provider: TimeProvider,
    ) -> FSResult<Self> {
        let instance = Self {
            conn,
            local_symkey,
            cache_size,
            time_provider,
        };
        instance.create_db().await?;
        Ok(instance)
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, sync::Arc};
    use uuid::Uuid;

    use rstest::rstest;
    use tests_fixtures::{tmp_path, TmpPath};

    use super::*;

    #[rstest]
    #[tokio::test]
    async fn chunk_storage(tmp_path: TmpPath) {
        let db_path = tmp_path.join("chunk_storage.sqlite");
        let conn = LocalDatabase::from_path(db_path.to_str().unwrap())
            .await
            .unwrap();
        let conn = Arc::new(conn);
        let local_symkey = SecretKey::generate();

        let chunk_storage = ChunkStorage::new(local_symkey, conn.clone(), TimeProvider::default())
            .await
            .unwrap();

        // Initialization
        chunk_storage.drop_db().await.unwrap();
        chunk_storage.create_db().await.unwrap();

        assert_eq!(chunk_storage.get_nb_blocks().await.unwrap(), 0);
        assert_eq!(chunk_storage.get_total_size().await.unwrap(), 0);

        // Generate chunks
        let chunk_id = ChunkID::from(Uuid::new_v4());
        chunk_storage
            .set_chunk(chunk_id, &[1, 2, 3, 4])
            .await
            .unwrap();

        const N: usize = 2000;

        let mut chunk_ids = Vec::with_capacity(N);

        for _ in 0..N {
            let chunk_id = ChunkID::from(Uuid::new_v4());
            chunk_ids.push(chunk_id);

            chunk_storage
                .set_chunk(chunk_id, &[1, 2, 3, 4])
                .await
                .unwrap();
        }

        assert_eq!(chunk_storage.get_nb_blocks().await.unwrap(), N as i64 + 1);
        assert_eq!(
            chunk_storage.get_total_size().await.unwrap(),
            (N as i64 + 1) * 44
        );

        // Retrieve chunks
        assert!(chunk_storage.is_chunk(chunk_id).await.unwrap());
        assert_eq!(
            &chunk_storage.get_chunk(chunk_id).await.unwrap(),
            &[1, 2, 3, 4]
        );

        let local_chunk_ids = chunk_storage.get_local_chunk_ids(&chunk_ids).await.unwrap();
        let set0 = local_chunk_ids.iter().collect::<HashSet<_>>();
        let set1 = chunk_ids.iter().collect::<HashSet<_>>();
        assert_eq!(set0.len(), N);
        assert_eq!(set1.len(), N);
        assert_eq!(set0.intersection(&set1).count(), N);

        chunk_storage.clear_chunk(chunk_id).await.unwrap();

        assert_eq!(chunk_storage.get_nb_blocks().await.unwrap(), N as i64);
        assert_eq!(chunk_storage.get_total_size().await.unwrap(), N as i64 * 44);

        let local_symkey = SecretKey::generate();
        let cache_size = *DEFAULT_BLOCK_SIZE * 1024;

        let block_storage =
            BlockStorage::new(local_symkey, conn, cache_size, TimeProvider::default())
                .await
                .unwrap();

        assert_eq!(block_storage.block_limit(), 1024);

        // Generate chunks
        let chunk_id = ChunkID::from(Uuid::new_v4());
        block_storage
            .set_chunk_upgraded(chunk_id, &[1, 2, 3, 4])
            .await
            .unwrap();

        let blocks_left = std::cmp::min(N as i64 + 1, 1024 - 1024 / 10);
        assert_eq!(chunk_storage.get_nb_blocks().await.unwrap(), blocks_left);
        assert_eq!(
            chunk_storage.get_total_size().await.unwrap(),
            blocks_left * 44
        );

        block_storage.clear_all_blocks().await.unwrap();

        assert_eq!(chunk_storage.get_nb_blocks().await.unwrap(), 0);
        assert_eq!(chunk_storage.get_total_size().await.unwrap(), 0);
    }
}
