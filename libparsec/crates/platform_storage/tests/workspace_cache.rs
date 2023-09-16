// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use libparsec_platform_storage::workspace::WorkspaceCacheStorage;

#[parsec_test(testbed = "minimal")]
async fn testbed_support(env: &TestbedEnv) {
    let mut wksp1_realm_id = None;
    let mut present_block_id = None;
    let mut missing_block_id = None;

    let env = env.customize(|builder| {
        let realm_id = builder.new_realm("alice").map(|e| e.realm_id);
        wksp1_realm_id = Some(realm_id);

        let block_id = builder
            .create_block("alice@dev1", realm_id, Bytes::from_static(b"aaa"))
            .map(|e| e.block_id);
        present_block_id = Some(block_id);
        builder.workspace_cache_storage_fetch_block("alice@dev1", realm_id, block_id);

        // Stuff the our storage is not aware of
        missing_block_id = Some(
            builder
                .create_block("alice@dev1", realm_id, Bytes::from_static(b"zzz"))
                .map(|e| e.block_id),
        );
    });

    let wksp1_realm_id = wksp1_realm_id.unwrap();
    let present_block_id = present_block_id.unwrap();
    let missing_block_id = missing_block_id.unwrap();

    let alice = env.local_device("alice@dev1");

    let storage = WorkspaceCacheStorage::start(
        &env.discriminant_dir,
        u64::MAX,
        alice.clone(),
        wksp1_realm_id,
    )
    .await
    .unwrap();

    p_assert_eq!(
        storage.get_block(present_block_id).await.unwrap(),
        Some(b"aaa".to_vec())
    );
    p_assert_eq!(storage.get_block(missing_block_id).await.unwrap(), None,);
}

#[parsec_test(testbed = "minimal")]
async fn operations(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let realm_id = VlobID::default();

    let cache_storage =
        WorkspaceCacheStorage::start(&env.discriminant_dir, 100, alice.clone(), realm_id)
            .await
            .unwrap();

    // See unit tests for bad start

    // 1) Set block

    let block_id = BlockID::default();
    cache_storage
        .set_block(block_id, b"<block content>")
        .await
        .unwrap();

    // 2) Get block

    let block = cache_storage.get_block(block_id).await.unwrap();
    p_assert_eq!(block, Some(b"<block content>".to_vec()));

    // 3) Clear block

    cache_storage.clear_block(block_id).await.unwrap();
    p_assert_matches!(cache_storage.get_block(block_id).await, Ok(None));
}

#[parsec_test(testbed = "minimal")]
async fn unknown_id(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let realm_id = VlobID::default();

    let cache_storage =
        WorkspaceCacheStorage::start(&env.discriminant_dir, 100, alice.clone(), realm_id)
            .await
            .unwrap();

    let block_id = BlockID::default();

    // 1) Get block

    p_assert_matches!(cache_storage.get_block(block_id).await, Ok(None));

    // 2) Clear block

    p_assert_matches!(cache_storage.clear_block(block_id).await, Ok(()));
}

#[parsec_test(testbed = "minimal")]
async fn cleanup(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let realm_id = VlobID::default();
    // Cache size is enough for 2 blocks, but not for 3
    let cache_size = *DEFAULT_BLOCK_SIZE * 2 + 1;

    let cache_storage =
        WorkspaceCacheStorage::start(&env.discriminant_dir, cache_size, alice.clone(), realm_id)
            .await
            .unwrap();

    let b1 = BlockID::default();
    let b2 = BlockID::default();
    let b3 = BlockID::default();

    let block_data = vec![0u8; *DEFAULT_BLOCK_SIZE as usize];

    // Store 3 * 512ko of data
    cache_storage.set_block(b1, &block_data).await.unwrap();
    cache_storage.set_block(b2, &block_data).await.unwrap();
    cache_storage.set_block(b3, &block_data).await.unwrap();

    // Access the blocks
    let _ = cache_storage.get_block(b2).await.unwrap();
    let _ = cache_storage.get_block(b3).await.unwrap();
    let _ = cache_storage.get_block(b1).await.unwrap();

    // Block 2 is the least recent, cleanup should remove it
    p_assert_matches!(cache_storage.cleanup().await, Ok(()));

    p_assert_matches!(cache_storage.get_block(b2).await, Ok(None));
    p_assert_matches!(cache_storage.get_block(b3).await, Ok(Some(b)) if b == block_data);
    p_assert_matches!(cache_storage.get_block(b1).await, Ok(Some(b)) if b == block_data);
}
