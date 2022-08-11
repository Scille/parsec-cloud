// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use diesel::dsl::count_star;
use diesel::{sql_query, table, AsChangeset, ExpressionMethods, Insertable, QueryDsl, RunQueryDsl};
use std::sync::Mutex;

use libparsec_crypto::SecretKey;
use libparsec_types::{ChunkID, DateTime, DEFAULT_BLOCK_SIZE};

use super::local_database::{SqliteConn, SQLITE_MAX_VARIABLE_NUMBER};
use crate::error::{FSError, FSResult};
use crate::extensions::CoalesceTotalSize;

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

pub(crate) trait ChunkStorageTrait {
    fn conn(&self) -> &Mutex<SqliteConn>;
    fn local_symkey(&self) -> &SecretKey;

    fn create_db(&self) -> FSResult<()> {
        let conn = &mut *self.conn().lock().expect("Mutex is poisoned");
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
        .map_err(|e| FSError::CreateTable(format!("chunks {e}")))?;
        Ok(())
    }

    #[cfg(test)]
    fn drop_db(&self) -> FSResult<()> {
        let conn = &mut *self.conn().lock().expect("Mutex is poisoned");
        sql_query("DROP TABLE IF EXISTS chunks;")
            .execute(conn)
            .map_err(|e| FSError::DropTable(format!("chunks {e}")))?;
        Ok(())
    }

    // Size and chunks

    fn get_nb_blocks(&self) -> FSResult<i64> {
        let conn = &mut *self.conn().lock().expect("Mutex is poisoned");
        chunks::table
            .select(count_star())
            .first(conn)
            .map_err(|e| FSError::QueryTable(format!("chunks: get_nb_blocks {e}")))
    }

    fn get_total_size(&self) -> FSResult<i64> {
        let conn = &mut *self.conn().lock().expect("Mutex is poisoned");
        chunks::table
            .select(CoalesceTotalSize::default())
            .first(conn)
            .map_err(|e| FSError::QueryTable(format!("chunks: get_total_size {e}")))
    }

    fn is_chunk(&self, chunk_id: ChunkID) -> FSResult<bool> {
        let conn = &mut *self.conn().lock().expect("Mutex is poisoned");
        chunks::table
            .select(count_star())
            .filter(chunks::chunk_id.eq((*chunk_id).as_ref()))
            .first::<i64>(conn)
            .map_err(|e| FSError::QueryTable(format!("chunks: is_chunk {e}")))
            .map(|res| res > 0)
    }

    fn get_local_chunk_ids(&self, chunk_ids: &[ChunkID]) -> FSResult<Vec<ChunkID>> {
        let bytes_id_list = chunk_ids
            .iter()
            .map(|chunk_id| (**chunk_id).as_ref())
            .collect::<Vec<_>>();

        let mut res = Vec::with_capacity(chunk_ids.len());

        let conn = &mut *self.conn().lock().expect("Mutex is poisoned");
        // The number of loop iteration is expected to be rather low:
        // typically considering 4ko per chunk (i.e. the size of the buffer the
        // Linux Kernel send us through Fuse), each query could perform ~4mo of data.

        // If performance ever becomes an issue, we could further optimize this by
        // having the caller filter the chunks: a block containing multiple chunk
        // means those correspond to local modification and hence can be filtered out.
        // With this we would only provide the chunks corresponding to a synced block
        // which are 512ko big, so each query performs up to 512Mo !
        for bytes_id_list_chunk in bytes_id_list.chunks(SQLITE_MAX_VARIABLE_NUMBER) {
            res.append(
                &mut chunks::table
                    .select(chunks::chunk_id)
                    .filter(chunks::chunk_id.eq_any(bytes_id_list_chunk))
                    .load::<Vec<u8>>(conn)
                    .map_err(|e| FSError::QueryTable(format!("chunks: get_local_chunk_ids {e}")))?
                    .into_iter()
                    .map(|chunk_id| {
                        <[u8; 16]>::try_from(&chunk_id[..]).map_err(|_| {
                            FSError::QueryTable(format!("chunks: corrupted chunk_id {chunk_id:?}"))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .map(ChunkID::from)
                    .collect(),
            )
        }

        Ok(res)
    }

    fn get_chunk(&self, chunk_id: ChunkID) -> FSResult<Vec<u8>> {
        let accessed_on = DateTime::now().get_f64_with_us_precision();

        let conn = &mut *self.conn().lock().expect("Mutex is poisoned");
        let changes =
            diesel::update(chunks::table.filter(chunks::chunk_id.eq((*chunk_id).as_ref())))
                .set(chunks::accessed_on.eq(accessed_on))
                .execute(conn)
                .map_err(|e| FSError::UpdateTable(format!("chunks: get_chunk {e}")))?
                > 0;

        if !changes {
            return Err(FSError::LocalMiss(*chunk_id));
        }

        let ciphered = chunks::table
            .select(chunks::data)
            .filter(chunks::chunk_id.eq((*chunk_id).as_ref()))
            .first::<Vec<u8>>(conn)
            .map_err(|e| FSError::QueryTable(format!("chunks: get_chunk {e}")))?;

        Ok(self.local_symkey().decrypt(&ciphered)?)
    }

    fn set_chunk(&self, chunk_id: ChunkID, raw: &[u8]) -> FSResult<()> {
        let ciphered = self.local_symkey().encrypt(raw);
        let accessed_on = DateTime::now();

        let new_chunk = NewChunk {
            chunk_id: (*chunk_id).as_ref(),
            size: ciphered.len() as i64,
            offline: false,
            accessed_on: Some(accessed_on.into()),
            data: &ciphered,
        };

        let conn = &mut *self.conn().lock().expect("Mutex is poisoned");
        diesel::insert_into(chunks::table)
            .values(&new_chunk)
            .on_conflict(chunks::chunk_id)
            .do_update()
            .set(&new_chunk)
            .execute(conn)
            .map_err(|e| FSError::InsertTable(format!("chunks: set_chunk {e}")))?;
        Ok(())
    }

    fn clear_chunk(&self, chunk_id: ChunkID) -> FSResult<()> {
        let conn = &mut *self.conn().lock().expect("Mutex is poisoned");
        let changes =
            diesel::delete(chunks::table.filter(chunks::chunk_id.eq((*chunk_id).as_ref())))
                .execute(conn)
                .map_err(|e| FSError::DeleteTable(format!("chunks: clear_chunk {e}")))?
                > 0;

        if !changes {
            return Err(FSError::LocalMiss(*chunk_id));
        }

        Ok(())
    }

    fn run_vacuum(&self) -> FSResult<()> {
        let conn = &mut *self.conn().lock().expect("Mutex is poisoned");
        sql_query("VACUUM;")
            .execute(conn)
            .map_err(|e| FSError::Vacuum(e.to_string()))?;
        Ok(())
    }
}

pub(crate) trait BlockStorageTrait: ChunkStorageTrait {
    fn cache_size(&self) -> u64;

    // Garbage collection

    fn block_limit(&self) -> u64 {
        self.cache_size() / *DEFAULT_BLOCK_SIZE
    }

    fn clear_all_blocks(&self) -> FSResult<()> {
        let conn = &mut *self.conn().lock().expect("Mutex is poisoned");
        diesel::delete(chunks::table)
            .execute(conn)
            .map_err(|e| FSError::DeleteTable(format!("chunks: clear_all_blocks {e}")))?;
        Ok(())
    }

    // Upgraded set method

    fn set_chunk_upgraded(&self, chunk_id: ChunkID, raw: &[u8]) -> FSResult<()> {
        let ciphered = self.local_symkey().encrypt(raw);
        let accessed_on = DateTime::now();

        let new_chunk = NewChunk {
            chunk_id: (*chunk_id).as_ref(),
            size: ciphered.len() as i64,
            offline: false,
            accessed_on: Some(accessed_on.into()),
            data: &ciphered,
        };

        // Insert the chunk
        diesel::insert_into(chunks::table)
            .values(&new_chunk)
            .on_conflict(chunks::chunk_id)
            .do_update()
            .set(&new_chunk)
            .execute(&mut *self.conn().lock().expect("Mutex is poisoned"))
            .map_err(|e| FSError::InsertTable(format!("chunks: set_chunk {e}")))?;

        // Perform cleanup if necessary
        self.cleanup()
    }

    fn cleanup(&self) -> FSResult<()> {
        // Count the chunks
        let conn = &mut *self.conn().lock().expect("Mutex is poisoned");
        let nb_blocks = chunks::table
            .select(count_star())
            .first::<i64>(conn)
            .map_err(|e| FSError::QueryTable(format!("chunks: cleanup {e}")))?;

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

        diesel::delete(chunks::table.filter(chunks::chunk_id.eq_any(sub_query)))
            .execute(conn)
            .map_err(|e| FSError::DeleteTable(format!("chunks: cleanup {e}")))?;

        Ok(())
    }
}

// Interface to access the local chunks of data
pub(crate) struct ChunkStorage {
    conn: Mutex<SqliteConn>,
    local_symkey: SecretKey,
}

impl ChunkStorageTrait for ChunkStorage {
    fn conn(&self) -> &Mutex<SqliteConn> {
        &self.conn
    }
    fn local_symkey(&self) -> &SecretKey {
        &self.local_symkey
    }
}

impl ChunkStorage {
    pub fn new(local_symkey: SecretKey, conn: Mutex<SqliteConn>) -> FSResult<Self> {
        let instance = Self { conn, local_symkey };
        instance.create_db()?;
        Ok(instance)
    }
}

// Interface for caching the data blocks.
pub(crate) struct BlockStorage {
    conn: Mutex<SqliteConn>,
    local_symkey: SecretKey,
    cache_size: u64,
}

impl ChunkStorageTrait for BlockStorage {
    fn conn(&self) -> &Mutex<SqliteConn> {
        &self.conn
    }
    fn local_symkey(&self) -> &SecretKey {
        &self.local_symkey
    }
}

impl BlockStorageTrait for BlockStorage {
    fn cache_size(&self) -> u64 {
        self.cache_size
    }
}

impl BlockStorage {
    pub fn new(
        local_symkey: SecretKey,
        conn: Mutex<SqliteConn>,
        cache_size: u64,
    ) -> FSResult<Self> {
        let instance = Self {
            conn,
            local_symkey,
            cache_size,
        };
        instance.create_db()?;
        Ok(instance)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::RandomState;
    use std::collections::HashSet;
    use uuid::Uuid;

    use rstest::rstest;
    use tests_fixtures::{tmp_path, TmpPath};

    use super::*;
    use crate::storage::local_database::SqlitePool;

    #[rstest]
    fn chunk_storage(tmp_path: TmpPath) {
        let db_path = tmp_path.join("chunk_storage.sqlite");
        let pool = SqlitePool::new(db_path.to_str().unwrap()).unwrap();
        let conn = Mutex::new(pool.conn().unwrap());
        let local_symkey = SecretKey::generate();

        let chunk_storage = ChunkStorage::new(local_symkey, conn).unwrap();

        // Initialization
        chunk_storage.drop_db().unwrap();
        chunk_storage.create_db().unwrap();

        assert_eq!(chunk_storage.get_nb_blocks().unwrap(), 0);
        assert_eq!(chunk_storage.get_total_size().unwrap(), 0);

        // Generate chunks
        let chunk_id = ChunkID::from(Uuid::new_v4());
        chunk_storage.set_chunk(chunk_id, &[1, 2, 3, 4]).unwrap();

        const N: usize = 2000;

        let mut chunk_ids = Vec::with_capacity(N);

        for _ in 0..N {
            let chunk_id = ChunkID::from(Uuid::new_v4());
            chunk_ids.push(chunk_id);

            chunk_storage.set_chunk(chunk_id, &[1, 2, 3, 4]).unwrap();
        }

        assert_eq!(chunk_storage.get_nb_blocks().unwrap(), N as i64 + 1);
        assert_eq!(chunk_storage.get_total_size().unwrap(), (N as i64 + 1) * 44);

        // Retrieve chunks
        assert!(chunk_storage.is_chunk(chunk_id).unwrap());
        assert_eq!(&chunk_storage.get_chunk(chunk_id).unwrap(), &[1, 2, 3, 4]);

        let local_chunk_ids = chunk_storage.get_local_chunk_ids(&chunk_ids).unwrap();
        let set0: HashSet<_, RandomState> = HashSet::from_iter(local_chunk_ids.iter());
        let set1: HashSet<_, RandomState> = HashSet::from_iter(chunk_ids.iter());
        assert_eq!(set0.len(), N);
        assert_eq!(set1.len(), N);
        assert_eq!(set0.intersection(&set1).count(), N);

        chunk_storage.clear_chunk(chunk_id).unwrap();

        assert_eq!(chunk_storage.get_nb_blocks().unwrap(), N as i64);
        assert_eq!(chunk_storage.get_total_size().unwrap(), N as i64 * 44);

        let new_conn = Mutex::new(pool.conn().unwrap());
        let local_symkey = SecretKey::generate();
        let cache_size = *DEFAULT_BLOCK_SIZE * 1024;

        let block_storage = BlockStorage::new(local_symkey, new_conn, cache_size).unwrap();

        assert_eq!(block_storage.block_limit(), 1024);

        // Generate chunks
        let chunk_id = ChunkID::from(Uuid::new_v4());
        block_storage
            .set_chunk_upgraded(chunk_id, &[1, 2, 3, 4])
            .unwrap();

        let blocks_left = std::cmp::min(N as i64 + 1, 1024 - 1024 / 10);
        assert_eq!(chunk_storage.get_nb_blocks().unwrap(), blocks_left);
        assert_eq!(chunk_storage.get_total_size().unwrap(), blocks_left * 44);

        block_storage.clear_all_blocks().unwrap();

        assert_eq!(chunk_storage.get_nb_blocks().unwrap(), 0);
        assert_eq!(chunk_storage.get_total_size().unwrap(), 0);
    }
}
