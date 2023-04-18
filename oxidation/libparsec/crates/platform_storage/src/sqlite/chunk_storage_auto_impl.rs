// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use diesel::{dsl::count_star, ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
use itertools::Itertools;

use libparsec_platform_async::future;
use libparsec_types::prelude::*;

use crate::{ChunkStorage, StorageError};

use super::{
    error, sql_types,
    tables::{chunks, NewChunk},
    LocalDatabase,
};

pub(super) trait ChunkStorageAutoImpl {
    fn local_symkey(&self) -> &SecretKey;
    fn time_provider(&self) -> &TimeProvider;
    fn conn(&self) -> &LocalDatabase;
}

#[async_trait::async_trait]
impl<T> ChunkStorage for T
where
    T: ChunkStorageAutoImpl + Sync + Send,
{
    async fn get_nb_chunks(&self) -> crate::Result<usize> {
        self.conn()
            .exec_with_error_handler(
                |conn| chunks::table.select(count_star()).first(conn),
                |e| error::Error::Query {
                    table_name: "chunks",
                    error: e,
                },
            )
            .await
            .map(|v: i64| v as usize)
            .map_err(StorageError::from)
    }

    async fn get_total_size(&self) -> crate::Result<usize> {
        self.conn()
            .exec_with_error_handler(
                |conn| {
                    chunks::table
                        .select(sql_types::CoalesceTotalSize::default())
                        .first::<i64>(conn)
                },
                |e| error::Error::Query {
                    table_name: "chunks",
                    error: e,
                },
            )
            .await
            .map(|v| v as usize)
            .map_err(StorageError::from)
    }

    async fn is_chunk(&self, chunk_id: ChunkID) -> crate::Result<bool> {
        self.conn()
            .exec_with_error_handler(
                move |conn| {
                    chunks::table
                        .select(count_star())
                        .filter(chunks::chunk_id.eq((*chunk_id).as_ref()))
                        .first::<i64>(conn)
                        .map(|res| res > 0)
                },
                |e| error::Error::Query {
                    table_name: "chunks",
                    error: e,
                },
            )
            .await
            .map_err(StorageError::from)
    }

    async fn get_local_chunk_ids(&self, chunk_ids: &[ChunkID]) -> crate::Result<Vec<ChunkID>> {
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
            .chunks(super::db::LOCAL_DATABASE_MAX_VARIABLE_NUMBER)
            .map(|bytes_id_list_chunk| {
                let query = chunks::table
                    .select(chunks::chunk_id)
                    .filter(chunks::chunk_id.eq_any(bytes_id_list_chunk.to_vec()));

                conn.exec_with_error_handler(
                    move |conn| query.load::<Vec<u8>>(conn),
                    |e| error::Error::Query {
                        table_name: "chunks",
                        error: e,
                    },
                )
            });
        for chunks in future::join_all(futures).await {
            let mut chunks = chunks?
                .into_iter()
                .map(|chunk_id| {
                    ChunkID::try_from(chunk_id.as_slice()).map_err(|e| {
                        StorageError::InvalidEntryID {
                            used_as: "chunk_id",
                            error: e,
                        }
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;

            res.append(&mut chunks)
        }

        Ok(res)
    }

    async fn get_chunk(&self, chunk_id: ChunkID) -> crate::Result<Vec<u8>> {
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
                |e| error::Error::Query {
                    table_name: "chunks",
                    error: e,
                },
            )
            .await?
            > 0;

        if !changes {
            return Err(StorageError::LocalChunkIDMiss(chunk_id));
        }

        let ciphered = conn
            .exec_with_error_handler(
                move |conn| {
                    chunks::table
                        .select(chunks::data)
                        .filter(chunks::chunk_id.eq((*chunk_id).as_ref()))
                        .first::<Vec<u8>>(conn)
                },
                |e| error::Error::Query {
                    table_name: "chunks",
                    error: e,
                },
            )
            .await?;

        Ok(self.local_symkey().decrypt(&ciphered)?)
    }

    async fn set_chunk(&self, chunk_id: ChunkID, raw: &[u8]) -> crate::Result<()> {
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
            .await
            .map_err(error::Error::from)?;
        Ok(())
    }

    async fn clear_chunk(&self, chunk_id: ChunkID) -> crate::Result<()> {
        let changes = self
            .conn()
            .exec(move |conn| {
                conn.exclusive_transaction(|conn| {
                    diesel::delete(chunks::table.filter(chunks::chunk_id.eq((*chunk_id).as_ref())))
                        .execute(conn)
                })
            })
            .await
            .map_err(error::Error::from)?
            > 0;

        if !changes {
            return Err(StorageError::LocalChunkIDMiss(chunk_id));
        }

        Ok(())
    }

    async fn clear_chunks(&self, chunk_ids: &[ChunkID]) -> crate::Result<()> {
        let chunk_ids = chunk_ids
            .iter()
            .map(|id| id.as_bytes().to_vec())
            .collect::<Vec<_>>();

        self.conn()
            .exec(move |conn| {
                conn.exclusive_transaction(|conn| {
                    remove_chunks(conn, chunk_ids.iter().map(Vec::as_slice))
                })
            })
            .await
            .map_err(error::Error::from)?;
        Ok(())
    }

    async fn vacuum(&self) -> crate::Result<()> {
        self.conn()
            .vacuum()
            .await
            .map_err(|e| StorageError::Vacuum(Box::new(e)))
    }
}

pub(super) fn remove_chunks<'a>(
    conn: &mut SqliteConnection,
    chunk_ids: impl Iterator<Item = &'a [u8]>,
) -> diesel::QueryResult<()> {
    chunk_ids
        .chunks(super::db::LOCAL_DATABASE_MAX_VARIABLE_NUMBER)
        .into_iter()
        .try_for_each(|chunked_ids| {
            diesel::delete(chunks::table.filter(chunks::chunk_id.eq_any(chunked_ids)))
                .execute(conn)
                .and(Ok(()))
        })
}
