// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Arc,
};

use libparsec_crypto::{prelude::*, HashDigest};
use libparsec_types::{
    BlockAccess, BlockID, Blocksize, Chunk, ChunkID, DateTime, DeviceID, EntryID, FileManifest,
    LocalDevice, LocalFileManifest, LocalManifest, Regex,
};

use libparsec_testbed::TestbedEnv;
use libparsec_tests_fixtures::{parsec_test, timestamp};

use crate::{
    sqlite::{LocalDatabase, SqliteDataStorage, VacuumMode},
    Closable, ManifestStorage, NeedSyncEntries,
};

use super::test_chunk_interface;

const EMPTY_PATTERN: &str = r"^\b$";

fn get_db_relative_path() -> PathBuf {
    PathBuf::from("data_storage.sqlite")
}

async fn data_storage_with_defaults(
    discriminant_dir: &Path,
    device: Arc<LocalDevice>,
) -> SqliteDataStorage {
    let db_relative_path = get_db_relative_path();
    let conn = LocalDatabase::from_path(discriminant_dir, &db_relative_path, VacuumMode::default())
        .await
        .unwrap();
    SqliteDataStorage::new(conn, device).await.unwrap()
}

#[parsec_test(testbed = "minimal")]
async fn prevent_sync_pattern(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1".parse().unwrap());
    let data_storage = data_storage_with_defaults(dbg!(&env.discriminant_dir), alice.clone()).await;

    let (re, fully_applied) = data_storage.get_prevent_sync_pattern().await.unwrap();

    assert_eq!(re.to_string(), EMPTY_PATTERN);
    assert!(!fully_applied);

    let regex = Regex::from_regex_str(r"\z").unwrap();
    data_storage.set_prevent_sync_pattern(&regex).await.unwrap();

    let (re, fully_applied) = data_storage.get_prevent_sync_pattern().await.unwrap();

    assert_eq!(re.to_string(), r"\z");
    assert!(!fully_applied);

    // Passing fully applied on a random pattern is a noop...

    data_storage
        .mark_prevent_sync_pattern_fully_applied(&Regex::from_regex_str(EMPTY_PATTERN).unwrap())
        .await
        .unwrap();

    let (re, fully_applied) = data_storage.get_prevent_sync_pattern().await.unwrap();

    assert_eq!(re.to_string(), r"\z");
    assert!(!fully_applied);

    // ...unlike passing fully applied on the currently registered pattern

    data_storage
        .mark_prevent_sync_pattern_fully_applied(&regex)
        .await
        .unwrap();

    let (re, fully_applied) = data_storage.get_prevent_sync_pattern().await.unwrap();

    assert_eq!(re.to_string(), r"\z");
    assert!(fully_applied);
}

#[parsec_test(testbed = "minimal")]
async fn invalid_prevent_sync_pattern(env: &TestbedEnv) {
    let alice: Arc<LocalDevice> = env.local_device("alice@dev1".parse().unwrap());
    let data_storage = data_storage_with_defaults(&env.discriminant_dir, alice.clone()).await;

    let initial_prevent_sync_pattern = data_storage.get_prevent_sync_pattern().await.unwrap();

    const INVALID_REGEX: &str = "[";
    let valid_regex = Regex::from_regex_str("ok").unwrap();

    data_storage
        .set_prevent_sync_pattern(&valid_regex)
        .await
        .unwrap();

    // Close the connection to avoid `OperationalError: database is locked`
    // error due to concurrency operations on the SQLite database
    data_storage.close().await;
    drop(data_storage);

    let data_storage = data_storage_with_defaults(&env.discriminant_dir, alice.clone()).await;

    // Corrupt the db with an invalid regex
    data_storage
        .set_raw_prevent_sync_pattern(INVALID_REGEX)
        .await
        .unwrap();

    data_storage.close().await;
    drop(data_storage);

    let data_storage = data_storage_with_defaults(&env.discriminant_dir, alice.clone()).await;

    data_storage
        .set_prevent_sync_pattern(&initial_prevent_sync_pattern.0)
        .await
        .unwrap();

    assert_eq!(
        data_storage.get_prevent_sync_pattern().await.unwrap(),
        initial_prevent_sync_pattern
    );
}

#[parsec_test(testbed = "minimal")]
async fn realm_checkpoint(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1".parse().unwrap());
    let data_storage = data_storage_with_defaults(&env.discriminant_dir, alice.clone()).await;

    let entry_id = EntryID::default();

    data_storage
        .update_realm_checkpoint(64, vec![(entry_id, 2)])
        .await
        .unwrap();

    assert_eq!(data_storage.get_realm_checkpoint().await, 64);
}

#[parsec_test(testbed = "minimal")]
async fn set_manifest(timestamp: DateTime, env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1".parse().unwrap());
    let data_storage = data_storage_with_defaults(&env.discriminant_dir, alice.clone()).await;

    let t1 = timestamp;
    let t2 = t1.add_us(1);

    let entry_id = EntryID::default();

    let local_file_manifest = LocalManifest::File(LocalFileManifest {
        base: FileManifest {
            author: DeviceID::default(),
            timestamp: t1,
            id: EntryID::default(),
            parent: EntryID::default(),
            version: 1,
            created: t1,
            updated: t1,
            size: 8,
            blocksize: Blocksize::try_from(8).unwrap(),
            blocks: vec![BlockAccess {
                id: BlockID::default(),
                key: SecretKey::generate(),
                offset: 0,
                size: std::num::NonZeroU64::try_from(8).unwrap(),
                digest: HashDigest::from_data(&[]),
            }],
        },
        need_sync: false,
        updated: t2,
        size: 8,
        blocksize: Blocksize::try_from(8).unwrap(),
        blocks: vec![vec![Chunk {
            id: ChunkID::default(),
            start: 0,
            stop: std::num::NonZeroU64::try_from(8).unwrap(),
            raw_offset: 0,
            raw_size: std::num::NonZeroU64::try_from(8).unwrap(),
            access: None,
        }]],
    });

    assert!(data_storage.get_manifest_in_cache(entry_id).is_none());
    assert_eq!(
        data_storage.get_manifest(entry_id).await,
        Err(crate::StorageError::LocalEntryIDMiss(entry_id))
    );

    data_storage
        .set_manifest(entry_id, local_file_manifest.clone(), None)
        .await
        .unwrap();

    assert_eq!(
        data_storage.get_manifest(entry_id).await.unwrap(),
        local_file_manifest
    );

    let NeedSyncEntries {
        local_changes,
        remote_changes,
    } = data_storage.get_need_sync_entries().await.unwrap();

    assert_eq!(local_changes, HashSet::new());
    assert_eq!(remote_changes, HashSet::new());
}

#[parsec_test(testbed = "minimal")]
async fn chunk_storage(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1".parse().unwrap());
    let data_storage = data_storage_with_defaults(&env.discriminant_dir, alice.clone()).await;

    const CHUNK_TO_INSERT: usize = 2000;
    test_chunk_interface::<SqliteDataStorage>(&data_storage, CHUNK_TO_INSERT).await;
}
