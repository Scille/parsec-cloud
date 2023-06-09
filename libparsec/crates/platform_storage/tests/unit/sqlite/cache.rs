// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use libparsec_testbed::TestbedEnv;
use libparsec_tests_fixtures::parsec_test;
use libparsec_types::prelude::*;

use crate::{
    sqlite::{LocalDatabase, SqliteCacheStorage, VacuumMode},
    BlockStorage, ChunkStorage,
};

use super::test_chunk_interface;

async fn cache_storage_with_defaults(
    discriminant_dir: &Path,
    device: Arc<LocalDevice>,
) -> SqliteCacheStorage {
    let db_relative_path = PathBuf::from("block_storage.sqlite");
    let conn = LocalDatabase::from_path(discriminant_dir, &db_relative_path, VacuumMode::default())
        .await
        .unwrap();
    const CACHE_SIZE: u64 = DEFAULT_BLOCK_SIZE.inner() * 1024;
    SqliteCacheStorage::new(conn, device, CACHE_SIZE)
        .await
        .unwrap()
}

#[parsec_test(testbed = "minimal")]
async fn test_block_storage(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1".parse().unwrap());
    let cache_storage = cache_storage_with_defaults(&env.discriminant_dir, alice.clone()).await;

    const CACHE_SIZE: u64 = DEFAULT_BLOCK_SIZE.inner() * 1024;
    const CHUNK_TO_INSERT: usize = 2000;

    test_block_interface::<SqliteCacheStorage, CACHE_SIZE>(cache_storage, CHUNK_TO_INSERT).await;
}

async fn test_block_interface<
    S: BlockStorage + ChunkStorage + Send + Sync,
    const CACHE_SIZE: u64,
>(
    storage: S,
    chunk_to_insert: usize,
) {
    test_chunk_interface::<S>(&storage, chunk_to_insert).await;
    let block_limit = storage.block_limit() as usize;
    assert_eq!(block_limit as u64, (CACHE_SIZE / *DEFAULT_BLOCK_SIZE));

    let block_id = BlockID::default();
    const RAW_BLOCK_DATA: [u8; 4] = [5, 6, 7, 8];
    storage
        .set_clean_block(block_id, &RAW_BLOCK_DATA)
        .await
        .unwrap();

    let blocks_left = std::cmp::min(chunk_to_insert + 1, block_limit - block_limit / 10);

    assert_eq!(storage.get_nb_chunks().await.unwrap(), blocks_left);
    /// `test_chunk_interface` & `test_block_interface` both use DATA that is 4 bytes wide.
    /// The value below correspond to the size of that data but after encryption.
    const CHUNK_SIZE: usize = 44;
    assert_eq!(
        storage.get_total_size().await.unwrap(),
        blocks_left * CHUNK_SIZE
    );

    storage.clear_all_blocks().await.unwrap();

    assert_eq!(storage.get_nb_chunks().await.unwrap(), 0);
    assert_eq!(storage.get_total_size().await.unwrap(), 0);
}
