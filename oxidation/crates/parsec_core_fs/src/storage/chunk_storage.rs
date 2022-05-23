// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use diesel::dsl::count_star;
use diesel::{sql_query, ExpressionMethods, Insertable, QueryDsl, RunQueryDsl};
use std::sync::Mutex;

use parsec_api_crypto::SecretKey;
use parsec_api_types::{ChunkID, DateTime, DEFAULT_BLOCK_SIZE};

use super::local_database::{SqliteConn, SQLITE_MAX_VARIABLE_NUMBER};
use crate::error::{FSError, FSResult};
use crate::extensions::coalesce_total_size;
use crate::schema::chunks;

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = chunks)]
struct NewChunk<'a> {
    pub chunk_id: &'a [u8],
    pub size: i32,
    pub offline: bool,
    pub accessed_on: Option<f32>,
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
        .map_err(|_| FSError::CreateTable("chunks"))?;
        Ok(())
    }

    #[cfg(test)]
    fn drop_db(&self) -> FSResult<()> {
        let conn = &mut *self.conn().lock().expect("Mutex is poisoned");
        sql_query("DROP TABLE IF EXISTS chunks;")
            .execute(conn)
            .map_err(|_| FSError::DropTable("chunks"))?;
        Ok(())
    }

    // Size and chunks

    fn get_nb_blocks(&self) -> FSResult<i64> {
        let conn = &mut *self.conn().lock().expect("Mutex is poisoned");
        chunks::table
            .select(count_star())
            .first(conn)
            .map_err(|_| FSError::QueryTable("chunks: get_nb_blocks"))
    }

    fn get_total_size(&self) -> FSResult<i64> {
        let conn = &mut *self.conn().lock().expect("Mutex is poisoned");
        chunks::table
            .select(coalesce_total_size())
            .first(conn)
            .map_err(|_| FSError::QueryTable("chunks: get_total_size"))
    }

    fn is_chunk(&self, chunk_id: ChunkID) -> FSResult<bool> {
        let conn = &mut *self.conn().lock().expect("Mutex is poisoned");
        chunks::table
            .select(count_star())
            .filter(chunks::chunk_id.eq((*chunk_id).as_ref()))
            .first::<i64>(conn)
            .map_err(|_| FSError::QueryTable("chunks: is_chunk"))
            .map(|res| res > 0)
    }

    fn get_local_chunk_ids(&self, chunk_ids: &[ChunkID]) -> FSResult<Vec<ChunkID>> {
        let bytes_id_list = chunk_ids
            .iter()
            .map(|chunk_id| (**chunk_id).as_ref())
            .collect::<Vec<_>>();

        let mut res = Vec::with_capacity(chunk_ids.len());

        let conn = &mut *self.conn().lock().expect("Mutex is poisoned");
        for bytes_id_list_chunk in bytes_id_list.chunks(SQLITE_MAX_VARIABLE_NUMBER) {
            res.append(
                &mut chunks::table
                    .select(chunks::chunk_id)
                    .filter(chunks::chunk_id.eq_any(bytes_id_list_chunk))
                    .load::<Vec<u8>>(conn)
                    .map_err(|_| FSError::QueryTable("chunks: get_local_chunk_ids"))?
                    .into_iter()
                    .map(|chunk_id| {
                        <[u8; 16]>::try_from(&chunk_id[..])
                            .map_err(|_| FSError::QueryTable("chunks: corrupted chunk_id"))
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
        let accessed_on = DateTime::now().get_f64_with_us_precision() as f32;

        let conn = &mut *self.conn().lock().expect("Mutex is poisoned");
        let changes =
            diesel::update(chunks::table.filter(chunks::chunk_id.eq((*chunk_id).as_ref())))
                .set(chunks::accessed_on.eq(accessed_on))
                .execute(conn)
                .map_err(|_| FSError::UpdateTable("chunks: get_chunk"))?
                > 0;

        if !changes {
            return Err(FSError::LocalMiss(*chunk_id));
        }

        let ciphered = chunks::table
            .select(chunks::data)
            .filter(chunks::chunk_id.eq((*chunk_id).as_ref()))
            .first::<Vec<u8>>(conn)
            .map_err(|_| FSError::QueryTable("chunks: get_chunk"))?;

        Ok(self.local_symkey().decrypt(&ciphered)?)
    }

    fn set_chunk(&self, chunk_id: ChunkID, raw: &[u8]) -> FSResult<()> {
        let ciphered = self.local_symkey().encrypt(raw);
        let accessed_on = DateTime::now().get_f64_with_us_precision() as f32;

        let new_chunk = NewChunk {
            chunk_id: (*chunk_id).as_ref(),
            size: ciphered.len() as i32,
            offline: false,
            accessed_on: Some(accessed_on),
            data: &ciphered,
        };

        let conn = &mut *self.conn().lock().expect("Mutex is poisoned");
        diesel::insert_into(chunks::table)
            .values(&new_chunk)
            .on_conflict(chunks::chunk_id)
            .do_update()
            .set(&new_chunk)
            .execute(conn)
            .map_err(|_| FSError::InsertTable("chunks: set_chunk"))?;
        Ok(())
    }

    fn clear_chunk(&self, chunk_id: ChunkID) -> FSResult<()> {
        let conn = &mut *self.conn().lock().expect("Mutex is poisoned");
        let changes =
            diesel::delete(chunks::table.filter(chunks::chunk_id.eq((*chunk_id).as_ref())))
                .execute(conn)
                .map_err(|_| FSError::DeleteTable("chunks: clear_chunk"))?
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
            .map_err(|_| FSError::Vacuum)?;
        Ok(())
    }
}

pub(crate) trait BlockStorageTrait: ChunkStorageTrait {
    fn cache_size(&self) -> u64;

    // Garbage collection

    fn block_limit(&self) -> u64 {
        self.cache_size() / DEFAULT_BLOCK_SIZE
    }

    fn clear_all_blocks(&self) -> FSResult<()> {
        let conn = &mut *self.conn().lock().expect("Mutex is poisoned");
        diesel::delete(chunks::table)
            .execute(conn)
            .map_err(|_| FSError::DeleteTable("chunks: clear_all_blocks"))?;
        Ok(())
    }

    // Upgraded set method

    fn set_chunk_upgraded(&self, chunk_id: ChunkID, raw: &[u8]) -> FSResult<()> {
        let ciphered = self.local_symkey().encrypt(raw);
        let accessed_on = DateTime::now().get_f64_with_us_precision() as f32;

        let new_chunk = NewChunk {
            chunk_id: (*chunk_id).as_ref(),
            size: ciphered.len() as i32,
            offline: false,
            accessed_on: Some(accessed_on),
            data: &ciphered,
        };

        // Insert the chunk
        diesel::insert_into(chunks::table)
            .values(&new_chunk)
            .on_conflict(chunks::chunk_id)
            .do_update()
            .set(&new_chunk)
            .execute(&mut *self.conn().lock().expect("Mutex is poisoned"))
            .map_err(|_| FSError::InsertTable("chunks: set_chunk"))?;

        // Perform cleanup if necessary
        self.cleanup()
    }

    fn cleanup(&self) -> FSResult<()> {
        // Count the chunks
        let conn = &mut *self.conn().lock().expect("Mutex is poisoned");
        let nb_blocks = chunks::table
            .select(count_star())
            .first::<i64>(conn)
            .map_err(|_| FSError::QueryTable("chunks: cleanup"))?;

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
            .map_err(|_| FSError::DeleteTable("chunks: cleanup"))?;

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

    use super::super::local_database::SqlitePool;
    use super::*;

    #[test]
    fn chunk_storage() {
        let pool = SqlitePool::new("/tmp/chunk_storage.sqlite").unwrap();
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
        assert_eq!(chunk_storage.is_chunk(chunk_id).unwrap(), true);
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
        let cache_size = DEFAULT_BLOCK_SIZE * 1024;

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
