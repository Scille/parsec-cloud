// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{
    collections::HashSet,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use diesel::{
    dsl::{count_star, not},
    sql_query, table, AsChangeset, BoolExpressionMethods, ExpressionMethods, Insertable,
    OptionalExtension, QueryDsl, RunQueryDsl,
};

use libparsec_crypto::SecretKey;
use libparsec_platform_async::future;
use libparsec_platform_local_db::{LocalDatabase, LOCAL_DATABASE_MAX_VARIABLE_NUMBER};
use libparsec_types::{BlockID, ChunkID, LocalDevice, TimeProvider, DEFAULT_BLOCK_SIZE};

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

table! {
    remanence(_id) {
        _id -> BigInt,
        block_remanent -> Bool,
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
                conn.exclusive_transaction(|conn| {
                    sql_query(std::include_str!("sql/create-chunks-table.sql")).execute(conn)?;
                    sql_query(std::include_str!("sql/create-remanence-table.sql")).execute(conn)
                })
            })
            .await?;
        Ok(())
    }

    async fn close_connection(&self) {
        self.conn().close().await;
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
                        ChunkID::try_from(chunk_id.as_slice()).map_err(|_| {
                            FSError::QueryTable(format!("chunks: corrupted chunk_id {chunk_id:?}"))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
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
                    conn.exclusive_transaction(|conn| {
                        diesel::update(
                            chunks::table.filter(chunks::chunk_id.eq((*chunk_id).as_ref())),
                        )
                        .set(chunks::accessed_on.eq(accessed_on))
                        .execute(conn)
                    })
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
                conn.exclusive_transaction(|conn| {
                    diesel::insert_into(chunks::table)
                        .values(&new_chunk)
                        .on_conflict(chunks::chunk_id)
                        .do_update()
                        .set(&new_chunk)
                        .execute(conn)
                })
            })
            .await?;
        Ok(())
    }

    async fn clear_chunk(&self, chunk_id: ChunkID) -> FSResult<()> {
        let changes = self
            .conn()
            .exec(move |conn| {
                conn.exclusive_transaction(|conn| {
                    diesel::delete(chunks::table.filter(chunks::chunk_id.eq((*chunk_id).as_ref())))
                        .execute(conn)
                })
            })
            .await?
            > 0;

        if !changes {
            return Err(FSError::LocalMiss(*chunk_id));
        }

        Ok(())
    }

    /// Clear the chunks identified by the chunk ids list `chunk_ids`.
    async fn clear_chunks(&self, chunk_ids: &[ChunkID]) -> FSResult<()> {
        let chunk_ids = chunk_ids
            .iter()
            .map(|id| id.as_bytes().to_vec())
            .collect::<Vec<_>>();
        self.conn()
            .exec(move |conn| {
                conn.exclusive_transaction(|conn| {
                    chunk_ids
                        .chunks(LOCAL_DATABASE_MAX_VARIABLE_NUMBER)
                        .try_for_each(|chunked_ids| {
                            diesel::delete(
                                chunks::table.filter(chunks::chunk_id.eq_any(chunked_ids.iter())),
                            )
                            .execute(conn)
                            .map(|_| ())
                        })
                })
            })
            .await?;
        Ok(())
    }

    async fn run_vacuum(&self) -> FSResult<()> {
        self.conn()
            .vacuum()
            .await
            .map_err(|e| FSError::Vacuum(e.to_string()))
    }
}

#[async_trait::async_trait]
pub(crate) trait BlockStorageTrait: ChunkStorageTrait + Remanence {
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

    async fn set_clean_block(&self, block_id: BlockID, raw: &[u8]) -> FSResult<HashSet<BlockID>> {
        let ciphered = self.local_symkey().encrypt(raw);
        let accessed_on = self.time_provider().now();

        // Insert the chunk
        self.conn()
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
            .await?;

        // Perform cleanup if necessary
        self.cleanup().await
    }

    async fn cleanup(&self) -> FSResult<HashSet<BlockID>> {
        // No cleanup for remanent storage.
        if self.is_block_remanent() {
            return Ok(HashSet::default());
        }

        // Count the chunks.
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
        .map_err(FSError::from)
        .and_then(|res: Vec<Vec<u8>>| {
            res.into_iter()
                .map(|raw_chunk| {
                    BlockID::try_from(&raw_chunk[..]).map_err(|_| {
                        FSError::QueryTable(format!("Chunks: corrupted block_id {raw_chunk:?}"))
                    })
                })
                .collect::<FSResult<HashSet<_>>>()
        })
    }
}

/// Interface to access the local chunks of data
// TODO: To improve perf, we would want to have the operations made in chunkstorage only commited when needed.
// i.e. We want to only commit chunks that are referenced by a manifest, so until they're referenced they may be not needed or changed.
pub(crate) struct ChunkStorage {
    conn: Arc<LocalDatabase>,
    device: Arc<LocalDevice>,
}

impl ChunkStorageTrait for ChunkStorage {
    fn conn(&self) -> &LocalDatabase {
        &self.conn
    }
    fn local_symkey(&self) -> &SecretKey {
        &self.device.local_symkey
    }
    fn time_provider(&self) -> &TimeProvider {
        &self.device.time_provider
    }
}

impl ChunkStorage {
    pub async fn new(conn: Arc<LocalDatabase>, device: Arc<LocalDevice>) -> FSResult<Self> {
        let instance = Self { device, conn };
        instance.create_db().await?;
        Ok(instance)
    }
}

// Interface for caching the data blocks.
pub(crate) struct BlockStorage {
    conn: LocalDatabase,
    device: Arc<LocalDevice>,
    cache_size: u64,
    /// Flag that enable/disable the block remanence.
    block_remanent_enabled: AtomicBool,
}

impl ChunkStorageTrait for BlockStorage {
    fn conn(&self) -> &LocalDatabase {
        &self.conn
    }
    fn local_symkey(&self) -> &SecretKey {
        &self.device.local_symkey
    }
    fn time_provider(&self) -> &TimeProvider {
        &self.device.time_provider
    }
}

impl BlockStorageTrait for BlockStorage {
    fn cache_size(&self) -> u64 {
        self.cache_size
    }
}

impl BlockStorage {
    pub async fn new(
        conn: LocalDatabase,
        device: Arc<LocalDevice>,
        cache_size: u64,
    ) -> FSResult<Self> {
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
}

// TODO: Remove it !
// We would need to refactor the storage later.
// We would combine the `manifest + chunk` storage that both use the `data` database and let the `block storage` on it's own.
// At that moment we could more easily merge the trait directly into `block` storage.
#[async_trait::async_trait]
pub trait Remanence {
    /// Is block remanence enabled ?
    fn is_block_remanent(&self) -> bool;
    /// Enable block remanence.
    /// Return `true` if the value has changed from the previous state.
    async fn enable_block_remanence(&self) -> FSResult<bool>;
    /// Disable block remanence.
    /// If the block remanence was disabled by this call,
    /// It will return the list of block that where clean-up.
    async fn disable_block_remanence(&self) -> FSResult<Option<HashSet<BlockID>>>;
    /// Remove from the database, the chunk whose id is in `chunk_ids` and it's access time older than `not_accessed_after`.
    async fn clear_unreferenced_chunks(
        &self,
        chunk_ids: &[ChunkID],
        not_accessed_after: libparsec_types::DateTime,
    ) -> FSResult<()>;
}

impl BlockStorage {
    async fn load_remanence_flag(&self) -> FSResult<()> {
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

    async fn set_block_remanence(&self, toggle: bool) -> Result<(), FSError> {
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

#[async_trait::async_trait]
impl Remanence for BlockStorage {
    fn is_block_remanent(&self) -> bool {
        self.block_remanent_enabled.load(Ordering::SeqCst)
    }

    async fn enable_block_remanence(&self) -> FSResult<bool> {
        if self.is_block_remanent() {
            return Ok(false);
        }
        self.set_block_remanence(true).await?;
        Ok(true)
    }

    async fn disable_block_remanence(&self) -> FSResult<Option<HashSet<BlockID>>> {
        if !self.is_block_remanent() {
            return Ok(None);
        }

        self.set_block_remanence(false).await?;

        self.cleanup().await.map(Some)
    }

    /// Remove chunks that aren't in `chunk_ids` and are older than `not_accessed_after`.
    async fn clear_unreferenced_chunks(
        &self,
        chunk_ids: &[ChunkID],
        not_accessed_after: libparsec_types::DateTime,
    ) -> FSResult<()> {
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
                        .chunks(LOCAL_DATABASE_MAX_VARIABLE_NUMBER)
                        .try_for_each(|chunked| {
                            let entries = chunked
                                .iter()
                                .map(|v| unreferenced_chunks::chunk_id.eq(v.as_bytes().as_slice()))
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
            .map_err(FSError::from)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashSet,
        path::{Path, PathBuf},
        sync::Arc,
    };

    use libparsec_platform_local_db::VacuumMode;
    use libparsec_testbed::TestbedEnv;
    use libparsec_tests_fixtures::parsec_test;
    use libparsec_types::LocalDevice;

    use super::*;

    async fn chunk_storage_with_defaults(
        discriminant_dir: &Path,
        device: Arc<LocalDevice>,
    ) -> ChunkStorage {
        let db_relative_path = PathBuf::from("chunk_storage.sqlite");
        let conn =
            LocalDatabase::from_path(discriminant_dir, &db_relative_path, VacuumMode::default())
                .await
                .unwrap();
        let conn = Arc::new(conn);
        ChunkStorage::new(conn, device).await.unwrap()
    }

    async fn block_storage_with_defaults(
        discriminant_dir: &Path,
        device: Arc<LocalDevice>,
    ) -> BlockStorage {
        let db_relative_path = PathBuf::from("block_storage.sqlite");
        let conn =
            LocalDatabase::from_path(discriminant_dir, &db_relative_path, VacuumMode::default())
                .await
                .unwrap();
        const CACHE_SIZE: u64 = DEFAULT_BLOCK_SIZE.inner() * 1024;
        BlockStorage::new(conn, device, CACHE_SIZE).await.unwrap()
    }

    #[parsec_test(testbed = "minimal")]
    async fn chunk_storage(env: &TestbedEnv) {
        let alice = env.local_device("alice@dev1".parse().unwrap());
        let chunk_storage = chunk_storage_with_defaults(&env.discriminant_dir, alice.clone()).await;

        const CHUNK_TO_INSERT: usize = 2000;
        test_chunk_interface::<ChunkStorage, CHUNK_TO_INSERT>(&chunk_storage).await;
    }

    async fn test_chunk_interface<
        S: ChunkStorageTrait + Send + Sync,
        const CHUNK_TO_INSERT: usize,
    >(
        storage: &S,
    ) {
        assert_eq!(storage.get_nb_blocks().await.unwrap(), 0);
        assert_eq!(storage.get_total_size().await.unwrap(), 0);

        // Generate chunks
        const RAW_CHUNK_DATA: [u8; 4] = [1, 2, 3, 4];

        let chunk_ids =
            insert_random_chunk::<S, CHUNK_TO_INSERT>(storage, RAW_CHUNK_DATA.as_slice()).await;

        assert_eq!(
            storage.get_nb_blocks().await.unwrap(),
            CHUNK_TO_INSERT as i64
        );
        assert_eq!(
            storage.get_total_size().await.unwrap(),
            CHUNK_TO_INSERT as i64 * 44
        );

        // Retrieve chunks
        let random_index = 42;
        assert!(random_index < CHUNK_TO_INSERT);
        let chunk_id = chunk_ids[random_index];

        assert!(storage.is_chunk(chunk_id).await.unwrap());
        assert_eq!(storage.get_chunk(chunk_id).await.unwrap(), &RAW_CHUNK_DATA);

        // Retrieve missing chunks.
        let random_chunk_id = ChunkID::default();
        assert!(
            !chunk_ids.iter().any(|id| id == &random_chunk_id),
            "The random generated chunk_id MUST not be already inserted in the storage by the previous step"
        );
        operation_on_missing_chunk(storage, random_chunk_id).await;

        let local_chunk_ids = storage.get_local_chunk_ids(&chunk_ids).await.unwrap();
        let set0 = local_chunk_ids.iter().collect::<HashSet<_>>();
        let set1 = chunk_ids.iter().collect::<HashSet<_>>();

        assert_eq!(set0.len(), CHUNK_TO_INSERT);
        assert_eq!(set1.len(), CHUNK_TO_INSERT);

        assert_eq!(set0.intersection(&set1).count(), CHUNK_TO_INSERT);

        remove_chunk_from_storage(storage, chunk_id).await;

        assert_eq!(
            storage.get_nb_blocks().await.unwrap(),
            CHUNK_TO_INSERT as i64 - 1
        );
        assert_eq!(
            storage.get_total_size().await.unwrap(),
            (CHUNK_TO_INSERT as i64 - 1) * 44
        );
    }

    async fn insert_random_chunk<
        S: ChunkStorageTrait + Send + Sync,
        const CHUNK_TO_INSERT: usize,
    >(
        storage: &S,
        chunk_data: &[u8],
    ) -> Vec<ChunkID> {
        let mut chunk_ids = Vec::with_capacity(CHUNK_TO_INSERT);

        for _ in 0..CHUNK_TO_INSERT {
            let chunk_id = ChunkID::default();
            chunk_ids.push(chunk_id);

            storage.set_chunk(chunk_id, chunk_data).await.unwrap();
        }

        chunk_ids
    }

    async fn remove_chunk_from_storage<S: ChunkStorageTrait + Send + Sync>(
        storage: &S,
        chunk_id: ChunkID,
    ) {
        storage.clear_chunk(chunk_id).await.unwrap();
        operation_on_missing_chunk(storage, chunk_id).await
    }

    async fn operation_on_missing_chunk<S: ChunkStorageTrait + Send + Sync>(
        storage: &S,
        chunk_id: ChunkID,
    ) {
        assert!(!storage.is_chunk(chunk_id).await.unwrap());
        assert_eq!(
            storage.get_chunk(chunk_id).await,
            Err(FSError::LocalMiss(*chunk_id))
        );

        assert_eq!(
            storage.clear_chunk(chunk_id).await,
            Err(FSError::LocalMiss(*chunk_id))
        );
    }

    #[parsec_test(testbed = "minimal")]
    async fn test_block_storage(env: &TestbedEnv) {
        let alice = env.local_device("alice@dev1".parse().unwrap());
        let block_storage = block_storage_with_defaults(&env.discriminant_dir, alice.clone()).await;

        const CACHE_SIZE: u64 = DEFAULT_BLOCK_SIZE.inner() * 1024;
        const CHUNK_TO_INSERT: usize = 2000;

        test_block_interface::<BlockStorage, CACHE_SIZE, CHUNK_TO_INSERT>(block_storage).await;
    }

    async fn test_block_interface<
        S: BlockStorageTrait + Send + Sync,
        const CACHE_SIZE: u64,
        const CHUNK_TO_INSERT: usize,
    >(
        storage: S,
    ) {
        test_chunk_interface::<S, CHUNK_TO_INSERT>(&storage).await;
        let block_limit = storage.block_limit();
        assert_eq!(block_limit, (CACHE_SIZE / *DEFAULT_BLOCK_SIZE));

        let block_id = BlockID::default();
        const RAW_BLOCK_DATA: [u8; 4] = [5, 6, 7, 8];
        storage
            .set_clean_block(block_id, &RAW_BLOCK_DATA)
            .await
            .unwrap();

        let blocks_left = std::cmp::min(
            CHUNK_TO_INSERT as i64 + 1,
            block_limit as i64 - block_limit as i64 / 10,
        );

        assert_eq!(storage.get_nb_blocks().await.unwrap(), blocks_left);
        /// `test_chunk_interface` & `test_block_interface` both use DATA that is 4 bytes wide.
        /// The value below correspond to the size of that data but after encryption.
        const CHUNK_SIZE: usize = 44;
        assert_eq!(
            storage.get_total_size().await.unwrap(),
            blocks_left * CHUNK_SIZE as i64
        );

        storage.clear_all_blocks().await.unwrap();

        assert_eq!(storage.get_nb_blocks().await.unwrap(), 0);
        assert_eq!(storage.get_total_size().await.unwrap(), 0);
    }
}
