// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::collections::HashSet;

use libparsec_types::prelude::*;

use crate::{ChunkStorage, StorageError};

pub async fn test_chunk_interface<S: ChunkStorage + Send + Sync>(
    storage: &S,
    chunk_to_insert: usize,
) {
    assert_eq!(storage.get_nb_chunks().await.unwrap(), 0);
    assert_eq!(storage.get_total_size().await.unwrap(), 0);

    // Generate chunks
    const RAW_CHUNK_DATA: [u8; 4] = [1, 2, 3, 4];

    let chunk_ids =
        insert_random_chunk::<S>(storage, RAW_CHUNK_DATA.as_slice(), chunk_to_insert).await;

    assert_eq!(storage.get_nb_chunks().await.unwrap(), chunk_to_insert);
    assert_eq!(
        storage.get_total_size().await.unwrap(),
        chunk_to_insert * 44
    );

    // Retrieve chunks
    let random_index = 42;
    assert!(random_index < chunk_to_insert);
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

    assert_eq!(set0.len(), chunk_to_insert);
    assert_eq!(set1.len(), chunk_to_insert);

    assert_eq!(set0.intersection(&set1).count(), chunk_to_insert);

    remove_chunk_from_storage(storage, chunk_id).await;

    assert_eq!(storage.get_nb_chunks().await.unwrap(), chunk_to_insert - 1);
    assert_eq!(
        storage.get_total_size().await.unwrap(),
        (chunk_to_insert - 1) * 44
    );
}

async fn insert_random_chunk<S: ChunkStorage + Send + Sync>(
    storage: &S,
    chunk_data: &[u8],
    chunk_to_insert: usize,
) -> Vec<ChunkID> {
    let mut chunk_ids = Vec::with_capacity(chunk_to_insert);

    for _ in 0..chunk_to_insert {
        let chunk_id = ChunkID::default();
        chunk_ids.push(chunk_id);

        storage.set_chunk(chunk_id, chunk_data).await.unwrap();
    }

    chunk_ids
}

async fn remove_chunk_from_storage<S: ChunkStorage + Send + Sync>(storage: &S, chunk_id: ChunkID) {
    storage.clear_chunk(chunk_id).await.unwrap();
    operation_on_missing_chunk(storage, chunk_id).await
}

async fn operation_on_missing_chunk<S: ChunkStorage + Send + Sync>(storage: &S, chunk_id: ChunkID) {
    assert!(!storage.is_chunk(chunk_id).await.unwrap());
    assert_eq!(
        storage.get_chunk(chunk_id).await,
        Err(StorageError::LocalChunkIDMiss(chunk_id))
    );

    assert_eq!(
        storage.clear_chunk(chunk_id).await,
        Err(StorageError::LocalChunkIDMiss(chunk_id))
    );
}
