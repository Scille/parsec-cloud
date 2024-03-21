// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::collections::{HashMap, HashSet};

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::workspace::UpdateManifestData;

use super::{workspace_storage_non_speculative_init, WorkspaceStorage};

#[allow(clippy::enum_variant_names)]
enum FetchStrategy {
    No,
    Single,
    Multiple,
}

#[parsec_test(testbed = "minimal")]
#[case::no_fetch(FetchStrategy::No)]
#[case::single_fetch(FetchStrategy::Single)]
#[case::multiple_fetch(FetchStrategy::Multiple)]
async fn testbed_support(#[case] fetch_strategy: FetchStrategy, env: &TestbedEnv) {
    let mut expected_version: VersionInt = 0;
    let mut expected_checkpoint: IndexInt = 0;

    let (env, (realm_id, block_id, chunk_id, file_id, folder_id)) =
        env.customize_with_map(|builder| {
            let realm_id = builder.new_realm("alice").map(|e| e.realm_id);
            builder.rotate_key_realm(realm_id);
            let block_id = builder
                .create_block("alice@dev1", realm_id, b"<block1>".as_ref())
                .map(|e| e.block_id);
            let chunk_id = builder
                .workspace_data_storage_chunk_create("alice@dev1", realm_id, b"<chunk1>".as_ref())
                .map(|e| e.chunk_id);

            let (file_id, folder_id) = if matches!(fetch_strategy, FetchStrategy::No) {
                builder
                    .workspace_data_storage_local_workspace_manifest_update("alice@dev1", realm_id);
                (None, None)
            } else {
                builder.create_or_update_workspace_manifest_vlob("alice@dev1", realm_id);
                let file_id = builder
                    .create_or_update_file_manifest_vlob("alice@dev1", realm_id, None)
                    .map(|e| e.manifest.id);
                let folder_id = builder
                    .create_or_update_folder_manifest_vlob("alice@dev1", realm_id, None)
                    .map(|e| e.manifest.id);
                expected_version = 1;
                expected_checkpoint = 3;
                builder.workspace_data_storage_fetch_realm_checkpoint("alice@dev1", realm_id);
                builder.workspace_data_storage_fetch_workspace_vlob("alice@dev1", realm_id, None);
                builder.workspace_data_storage_fetch_file_vlob("alice@dev1", realm_id, file_id);
                builder.workspace_data_storage_fetch_folder_vlob(
                    "alice@dev1",
                    realm_id,
                    folder_id,
                    None,
                );
                builder.workspace_cache_storage_fetch_block("alice@dev1", realm_id, block_id);

                if matches!(fetch_strategy, FetchStrategy::Multiple) {
                    builder.create_or_update_workspace_manifest_vlob("alice@dev1", realm_id);
                    builder.create_or_update_file_manifest_vlob(
                        "alice@dev1",
                        realm_id,
                        Some(file_id),
                    );
                    builder.create_or_update_folder_manifest_vlob(
                        "alice@dev1",
                        realm_id,
                        Some(folder_id),
                    );
                    expected_version = 2;
                    expected_checkpoint = 6;
                    builder.workspace_data_storage_fetch_realm_checkpoint("alice@dev1", realm_id);
                    builder.workspace_data_storage_fetch_workspace_vlob(
                        "alice@dev1",
                        realm_id,
                        None,
                    );
                    builder.workspace_data_storage_fetch_file_vlob("alice@dev1", realm_id, file_id);
                    builder.workspace_data_storage_fetch_folder_vlob(
                        "alice@dev1",
                        realm_id,
                        folder_id,
                        None,
                    );
                }

                (Some(file_id), Some(folder_id))
            };

            // Stuff our storage is not aware of

            let actual_version = builder
                .create_or_update_workspace_manifest_vlob("alice@dev1", realm_id)
                .map(|e| e.manifest.version);
            // Sanity check to ensure additional (and to be ignored) manifest have been added
            p_assert_ne!(expected_version, actual_version);
            if let Some(file_id) = &file_id {
                let actual_version = builder
                    .create_or_update_file_manifest_vlob("alice@dev1", realm_id, Some(*file_id))
                    .map(|e| e.manifest.version);
                p_assert_ne!(expected_version, actual_version);
            }
            if let Some(folder_id) = &folder_id {
                let actual_version = builder
                    .create_or_update_folder_manifest_vlob("alice@dev1", realm_id, Some(*folder_id))
                    .map(|e| e.manifest.version);
                p_assert_ne!(expected_version, actual_version);
            }

            (realm_id, block_id, chunk_id, file_id, folder_id)
        });

    let alice = env.local_device("alice@dev1");

    let mut workspace_storage =
        WorkspaceStorage::start(&env.discriminant_dir, &alice, realm_id, u64::MAX)
            .await
            .unwrap();

    // Checkpoint

    p_assert_eq!(
        workspace_storage.get_realm_checkpoint().await.unwrap(),
        expected_checkpoint
    );

    // Block

    let maybe_block_encrypted = workspace_storage
        .get_block(block_id, alice.now())
        .await
        .unwrap();
    if matches!(fetch_strategy, FetchStrategy::No) {
        p_assert_eq!(maybe_block_encrypted, None);
    } else {
        let block = alice
            .local_symkey
            .decrypt(&maybe_block_encrypted.unwrap())
            .unwrap();
        p_assert_eq!(block, b"<block1>");
    }

    // Chunk

    let chunk_encrypted = workspace_storage.get_chunk(chunk_id).await.unwrap();
    let chunk = alice
        .local_symkey
        .decrypt(&chunk_encrypted.unwrap())
        .unwrap();
    p_assert_eq!(chunk, b"<chunk1>");

    // Workspace manifest

    let encrypted = workspace_storage
        .get_manifest(realm_id)
        .await
        .unwrap()
        .unwrap();
    let workspace_manifest =
        LocalWorkspaceManifest::decrypt_and_load(&encrypted, &alice.local_symkey).unwrap();
    p_assert_eq!(workspace_manifest.base.version, expected_version);
    let expected_need_sync = match fetch_strategy {
        FetchStrategy::No => true,
        FetchStrategy::Single | FetchStrategy::Multiple => false,
    };
    p_assert_eq!(workspace_manifest.need_sync, expected_need_sync);

    // Folder manifest

    if let Some(file_id) = file_id {
        let encrypted = workspace_storage
            .get_manifest(file_id)
            .await
            .unwrap()
            .unwrap();
        let file_manifest =
            LocalFileManifest::decrypt_and_load(&encrypted, &alice.local_symkey).unwrap();
        p_assert_eq!(file_manifest.base.version, expected_version);
        let expected_need_sync = match fetch_strategy {
            FetchStrategy::No => true,
            FetchStrategy::Single | FetchStrategy::Multiple => false,
        };
        p_assert_eq!(file_manifest.need_sync, expected_need_sync);
    }

    // Folder manifest

    if let Some(folder_id) = folder_id {
        let encrypted = workspace_storage
            .get_manifest(folder_id)
            .await
            .unwrap()
            .unwrap();
        let folder_manifest =
            LocalFolderManifest::decrypt_and_load(&encrypted, &alice.local_symkey).unwrap();
        p_assert_eq!(folder_manifest.base.version, expected_version);
        let expected_need_sync = match fetch_strategy {
            FetchStrategy::No => true,
            FetchStrategy::Single | FetchStrategy::Multiple => false,
        };
        p_assert_eq!(folder_manifest.need_sync, expected_need_sync);
    }
}

#[parsec_test(testbed = "minimal")]
async fn get_and_update_manifest(env: &TestbedEnv) {
    let realm_id = VlobID::from_hex("aa0000000000000000000000000000ee").unwrap();
    let entry_id = VlobID::from_hex("aa0000000000000000000000000000ff").unwrap();
    let alice = env.local_device("alice@dev1");

    let mut workspace_storage =
        WorkspaceStorage::start(&env.discriminant_dir, &alice, realm_id, u64::MAX)
            .await
            .unwrap();

    // 1) Storage starts empty

    let dump = workspace_storage.debug_dump().await.unwrap();
    p_assert_eq!(
        dump,
        "\
checkpoint: 0
vlobs: [
]
chunks: [
]
blocks: [
]
"
    );
    p_assert_eq!(
        workspace_storage.get_manifest(entry_id).await.unwrap(),
        None
    );

    // 2) Update

    workspace_storage
        .update_manifest(&UpdateManifestData {
            entry_id,
            encrypted: b"<manifest_v1>".to_vec(),
            need_sync: false,
            base_version: 1,
        })
        .await
        .unwrap();
    p_assert_eq!(
        workspace_storage
            .get_manifest(entry_id)
            .await
            .unwrap()
            .unwrap(),
        b"<manifest_v1>"
    );

    // 3) Re-starting the database and check data are still there

    workspace_storage.stop().await.unwrap();
    let mut workspace_storage =
        WorkspaceStorage::start(&env.discriminant_dir, &alice, realm_id, u64::MAX)
            .await
            .unwrap();

    p_assert_eq!(
        workspace_storage
            .get_manifest(entry_id)
            .await
            .unwrap()
            .unwrap(),
        b"<manifest_v1>"
    );

    let dump = workspace_storage.debug_dump().await.unwrap();
    p_assert_eq!(
        dump,
        "\
checkpoint: 0
vlobs: [
{
	vlob_id: aa000000-0000-0000-0000-0000000000ff
	need_sync: false
	base_version: 1
	remote_version: 1
},
]
chunks: [
]
blocks: [
]
"
    );

    // 4) Overwrite

    workspace_storage
        .update_manifest(&UpdateManifestData {
            entry_id,
            encrypted: b"<manifest_v2>".to_vec(),
            need_sync: true,
            base_version: 2,
        })
        .await
        .unwrap();
    p_assert_eq!(
        workspace_storage
            .get_manifest(entry_id)
            .await
            .unwrap()
            .unwrap(),
        b"<manifest_v2>"
    );

    let dump = workspace_storage.debug_dump().await.unwrap();
    p_assert_eq!(
        dump,
        "\
checkpoint: 0
vlobs: [
{
	vlob_id: aa000000-0000-0000-0000-0000000000ff
	need_sync: true
	base_version: 2
	remote_version: 2
},
]
chunks: [
]
blocks: [
]
"
    );
}

#[parsec_test(testbed = "minimal")]
async fn update_manifests(env: &TestbedEnv) {
    let realm_id = VlobID::from_hex("aa0000000000000000000000000000ee").unwrap();
    let entry1_id = VlobID::from_hex("aa0000000000000000000000000000f1").unwrap();
    let entry2_id = VlobID::from_hex("aa0000000000000000000000000000f2").unwrap();
    let alice = env.local_device("alice@dev1");

    let mut workspace_storage =
        WorkspaceStorage::start(&env.discriminant_dir, &alice, realm_id, u64::MAX)
            .await
            .unwrap();

    workspace_storage
        .update_manifests(
            [
                UpdateManifestData {
                    entry_id: entry1_id,
                    encrypted: b"<manifest1_v1>".to_vec(),
                    need_sync: true,
                    base_version: 1,
                },
                UpdateManifestData {
                    entry_id: entry2_id,
                    encrypted: b"<manifest2_v2>".to_vec(),
                    need_sync: false,
                    base_version: 2,
                },
            ]
            .into_iter(),
        )
        .await
        .unwrap();

    p_assert_eq!(
        workspace_storage
            .get_manifest(entry1_id)
            .await
            .unwrap()
            .unwrap(),
        b"<manifest1_v1>"
    );
    p_assert_eq!(
        workspace_storage
            .get_manifest(entry2_id)
            .await
            .unwrap()
            .unwrap(),
        b"<manifest2_v2>"
    );

    let dump = workspace_storage.debug_dump().await.unwrap();
    p_assert_eq!(
        dump,
        "\
checkpoint: 0
vlobs: [
{
	vlob_id: aa000000-0000-0000-0000-0000000000f1
	need_sync: true
	base_version: 1
	remote_version: 1
},
{
	vlob_id: aa000000-0000-0000-0000-0000000000f2
	need_sync: false
	base_version: 2
	remote_version: 2
},
]
chunks: [
]
blocks: [
]
"
    );
}

#[parsec_test(testbed = "minimal")]
async fn update_manifest_and_chunks(env: &TestbedEnv) {
    let realm_id = VlobID::from_hex("aa0000000000000000000000000000ee").unwrap();
    let entry_id = VlobID::from_hex("aa0000000000000000000000000000f1").unwrap();
    let alice = env.local_device("alice@dev1");

    let mut workspace_storage =
        WorkspaceStorage::start(&env.discriminant_dir, &alice, realm_id, u64::MAX)
            .await
            .unwrap();

    let chunk1_id = ChunkID::from_hex("aa0000000000000000000000000000c1").unwrap();
    let chunk2_id = ChunkID::from_hex("aa0000000000000000000000000000c2").unwrap();
    let chunk1_promoted_as_block_id =
        BlockID::from_hex("aa0000000000000000000000000000f1").unwrap();

    // 1) Add chunks

    workspace_storage
        .update_manifest_and_chunks(
            &UpdateManifestData {
                entry_id,
                encrypted: b"<manifest_v1>".to_vec(),
                need_sync: true,
                base_version: 1,
            },
            [
                (chunk1_id, b"<chunk1>".to_vec()),
                (chunk2_id, b"<chunk2chunk2>".to_vec()),
            ]
            .into_iter(),
            [].into_iter(),
            [].into_iter(),
        )
        .await
        .unwrap();

    p_assert_eq!(
        workspace_storage
            .get_manifest(entry_id)
            .await
            .unwrap()
            .unwrap(),
        b"<manifest_v1>"
    );

    p_assert_eq!(
        workspace_storage
            .get_chunk(chunk1_id)
            .await
            .unwrap()
            .unwrap(),
        b"<chunk1>"
    );

    p_assert_eq!(
        workspace_storage
            .get_chunk(chunk2_id)
            .await
            .unwrap()
            .unwrap(),
        b"<chunk2chunk2>"
    );

    let dump = workspace_storage.debug_dump().await.unwrap();
    p_assert_eq!(
        dump,
        "\
checkpoint: 0
vlobs: [
{
	vlob_id: aa000000-0000-0000-0000-0000000000f1
	need_sync: true
	base_version: 1
	remote_version: 1
},
]
chunks: [
{
	chunk_id: aa000000-0000-0000-0000-0000000000c1
	size: 8
	offline: false
},
{
	chunk_id: aa000000-0000-0000-0000-0000000000c2
	size: 14
	offline: false
},
]
blocks: [
]
"
    );

    // 2) Remove chunks

    workspace_storage
        .update_manifest_and_chunks(
            &UpdateManifestData {
                entry_id,
                encrypted: b"<manifest_v2>".to_vec(),
                need_sync: true,
                base_version: 2,
            },
            [].into_iter(),
            [chunk2_id].into_iter(),
            [].into_iter(),
        )
        .await
        .unwrap();

    p_assert_eq!(
        workspace_storage
            .get_chunk(chunk1_id)
            .await
            .unwrap()
            .unwrap(),
        b"<chunk1>"
    );

    p_assert_eq!(workspace_storage.get_chunk(chunk2_id).await.unwrap(), None);

    let dump = workspace_storage.debug_dump().await.unwrap();
    p_assert_eq!(
        dump,
        "\
checkpoint: 0
vlobs: [
{
	vlob_id: aa000000-0000-0000-0000-0000000000f1
	need_sync: true
	base_version: 2
	remote_version: 2
},
]
chunks: [
{
	chunk_id: aa000000-0000-0000-0000-0000000000c1
	size: 8
	offline: false
},
]
blocks: [
]
"
    );

    // 3) Promote chunks

    workspace_storage
        .update_manifest_and_chunks(
            &UpdateManifestData {
                entry_id,
                encrypted: b"<manifest_v3>".to_vec(),
                need_sync: false,
                base_version: 2,
            },
            [].into_iter(),
            [].into_iter(),
            [(
                chunk1_id,
                chunk1_promoted_as_block_id,
                "2000-01-01T00:00:00Z".parse().unwrap(),
            )]
            .into_iter(),
        )
        .await
        .unwrap();

    p_assert_eq!(workspace_storage.get_chunk(chunk1_id).await.unwrap(), None);

    let dump = workspace_storage.debug_dump().await.unwrap();
    p_assert_eq!(
        dump,
        "\
checkpoint: 0
vlobs: [
{
	vlob_id: aa000000-0000-0000-0000-0000000000f1
	need_sync: false
	base_version: 2
	remote_version: 2
},
]
chunks: [
]
blocks: [
{
	block_id: aa000000-0000-0000-0000-0000000000f1
	size: 8
	offline: false
	accessed_on: 2000-01-01T00:00:00Z
},
]
"
    );
}

#[parsec_test(testbed = "minimal")]
async fn get_and_set_chunk(env: &TestbedEnv) {
    let realm_id = VlobID::from_hex("aa0000000000000000000000000000ee").unwrap();
    let chunk_id = ChunkID::from_hex("aa0000000000000000000000000000f1").unwrap();
    let alice = env.local_device("alice@dev1");

    let mut workspace_storage =
        WorkspaceStorage::start(&env.discriminant_dir, &alice, realm_id, u64::MAX)
            .await
            .unwrap();

    // 1) Insert chunk

    workspace_storage
        .set_chunk(chunk_id, b"<chunk1>".as_ref())
        .await
        .unwrap();

    // 2) Get back chunk

    p_assert_eq!(
        workspace_storage
            .get_chunk(chunk_id)
            .await
            .unwrap()
            .as_deref(),
        Some(b"<chunk1>".as_ref())
    );

    // 3) Test chunk or block access

    p_assert_eq!(
        workspace_storage
            .get_chunk_or_block(chunk_id, "2000-01-03T00:00:00Z".parse().unwrap())
            .await
            .unwrap()
            .as_deref(),
        Some(b"<chunk1>".as_ref())
    );

    // 4) Make sure chunk and blocks don't get mixed !

    p_assert_eq!(
        workspace_storage
            .get_block(BlockID::from(*chunk_id), alice.now())
            .await
            .unwrap(),
        None
    );

    let dump = workspace_storage.debug_dump().await.unwrap();
    p_assert_eq!(
        dump,
        "\
checkpoint: 0
vlobs: [
]
chunks: [
{
	chunk_id: aa000000-0000-0000-0000-0000000000f1
	size: 8
	offline: false
},
]
blocks: [
]
"
    );
}

#[parsec_test(testbed = "minimal")]
async fn get_and_set_block(env: &TestbedEnv) {
    let realm_id = VlobID::from_hex("aa0000000000000000000000000000ee").unwrap();
    let block_id = BlockID::from_hex("aa0000000000000000000000000000f1").unwrap();
    let alice = env.local_device("alice@dev1");

    let mut workspace_storage =
        WorkspaceStorage::start(&env.discriminant_dir, &alice, realm_id, u64::MAX)
            .await
            .unwrap();

    // 1) Insert block

    workspace_storage
        .set_block(
            block_id,
            b"<block1>".as_ref(),
            "2000-01-01T00:00:00Z".parse().unwrap(),
        )
        .await
        .unwrap();

    let dump = workspace_storage.debug_dump().await.unwrap();
    p_assert_eq!(
        dump,
        "\
checkpoint: 0
vlobs: [
]
chunks: [
]
blocks: [
{
	block_id: aa000000-0000-0000-0000-0000000000f1
	size: 8
	offline: false
	accessed_on: 2000-01-01T00:00:00Z
},
]
"
    );

    // 2) Get back block

    p_assert_eq!(
        workspace_storage
            .get_block(block_id, "2000-01-02T00:00:00Z".parse().unwrap())
            .await
            .unwrap()
            .as_deref(),
        Some(b"<block1>".as_ref())
    );

    let dump = workspace_storage.debug_dump().await.unwrap();
    p_assert_eq!(
        dump,
        "\
checkpoint: 0
vlobs: [
]
chunks: [
]
blocks: [
{
	block_id: aa000000-0000-0000-0000-0000000000f1
	size: 8
	offline: false
	accessed_on: 2000-01-02T00:00:00Z
},
]
"
    );

    // 3) Test chunk or block access

    p_assert_eq!(
        workspace_storage
            .get_chunk_or_block(
                ChunkID::from(*block_id),
                "2000-01-03T00:00:00Z".parse().unwrap()
            )
            .await
            .unwrap()
            .as_deref(),
        Some(b"<block1>".as_ref())
    );

    // 4) Make sure block and blocks don't get mixed !

    p_assert_eq!(
        workspace_storage
            .get_chunk(ChunkID::from(*block_id))
            .await
            .unwrap(),
        None
    );
}

#[parsec_test(testbed = "minimal")]
async fn block_cache_cleanup(env: &TestbedEnv) {
    let realm_id = VlobID::from_hex("aa0000000000000000000000000000ee").unwrap();
    let alice = env.local_device("alice@dev1");
    let block_size = 512 * 1024; // 512Ko
    let cache_size = 2_000_000; // ~2Mo

    let mut workspace_storage =
        WorkspaceStorage::start(&env.discriminant_dir, &alice, realm_id, cache_size)
            .await
            .unwrap();

    macro_rules! insert_block {
        ($block_id:expr, $size:expr, $timestamp:expr) => {{
            let block_id = BlockID::from_hex($block_id).unwrap();
            let data = vec![0; $size];
            workspace_storage
                .set_block(block_id, &data, $timestamp.parse().unwrap())
                .await
                .unwrap();
            block_id
        }};
    }

    // 1) Insert block to reach the cache limit

    insert_block!(
        "aa0000000000000000000000000000f1",
        block_size,
        "2000-01-01T00:00:00Z"
    );
    let block2_id = insert_block!(
        "aa0000000000000000000000000000f2",
        block_size,
        "2000-01-02T00:00:00Z"
    );
    insert_block!(
        "aa0000000000000000000000000000f3",
        block_size,
        "2000-01-03T00:00:00Z"
    );

    // 2) Add one more block that should trigger the cleanup and remove block 1

    insert_block!(
        "aa0000000000000000000000000000f4",
        block_size,
        "2000-01-04T00:00:00Z"
    );

    let dump = workspace_storage.debug_dump().await.unwrap();
    p_assert_eq!(
        dump,
        "\
checkpoint: 0
vlobs: [
]
chunks: [
]
blocks: [
{
	block_id: aa000000-0000-0000-0000-0000000000f2
	size: 524288
	offline: false
	accessed_on: 2000-01-02T00:00:00Z
},
{
	block_id: aa000000-0000-0000-0000-0000000000f3
	size: 524288
	offline: false
	accessed_on: 2000-01-03T00:00:00Z
},
{
	block_id: aa000000-0000-0000-0000-0000000000f4
	size: 524288
	offline: false
	accessed_on: 2000-01-04T00:00:00Z
},
]
"
    );

    // 3) Accessing a block should prevent it from being the one to be removed

    workspace_storage
        .get_block(block2_id, "2000-01-05T00:00:00Z".parse().unwrap())
        .await
        .unwrap();

    insert_block!(
        "aa0000000000000000000000000000f5",
        block_size,
        "2000-01-06T00:00:00Z"
    );

    let dump = workspace_storage.debug_dump().await.unwrap();
    p_assert_eq!(
        dump,
        "\
checkpoint: 0
vlobs: [
]
chunks: [
]
blocks: [
{
	block_id: aa000000-0000-0000-0000-0000000000f2
	size: 524288
	offline: false
	accessed_on: 2000-01-05T00:00:00Z
},
{
	block_id: aa000000-0000-0000-0000-0000000000f4
	size: 524288
	offline: false
	accessed_on: 2000-01-04T00:00:00Z
},
{
	block_id: aa000000-0000-0000-0000-0000000000f5
	size: 524288
	offline: false
	accessed_on: 2000-01-06T00:00:00Z
},
]
"
    );
}

#[parsec_test(testbed = "minimal")]
async fn checkpoint(env: &TestbedEnv) {
    let realm_id = VlobID::from_hex("aa0000000000000000000000000000ff").unwrap();
    let alice = env.local_device("alice@dev1");

    let mut user_storage =
        WorkspaceStorage::start(&env.discriminant_dir, &alice, realm_id, u64::MAX)
            .await
            .unwrap();

    // 1) Initial value

    p_assert_eq!(user_storage.get_realm_checkpoint().await.unwrap(), 0);

    // 2) Update

    user_storage.update_realm_checkpoint(1, &[]).await.unwrap();
    p_assert_eq!(user_storage.get_realm_checkpoint().await.unwrap(), 1);

    // 3) Re-starting the database and check data are still there

    user_storage.stop().await.unwrap();
    let mut user_storage =
        WorkspaceStorage::start(&env.discriminant_dir, &alice, realm_id, u64::MAX)
            .await
            .unwrap();

    p_assert_eq!(user_storage.get_realm_checkpoint().await.unwrap(), 1);
}

#[parsec_test(testbed = "minimal")]
async fn non_speculative_init(env: &TestbedEnv) {
    let realm_id = VlobID::from_hex("aa0000000000000000000000000000ff").unwrap();
    let alice = env.local_device("alice@dev1");

    // 1) Initialize the database

    workspace_storage_non_speculative_init(&env.discriminant_dir, &alice, realm_id)
        .await
        .unwrap();

    // 2) Check the database content

    let mut user_storage =
        WorkspaceStorage::start(&env.discriminant_dir, &alice, realm_id, u64::MAX)
            .await
            .unwrap();

    p_assert_eq!(user_storage.get_realm_checkpoint().await.unwrap(), 0);

    let encrypted = user_storage.get_manifest(realm_id).await.unwrap().unwrap();
    let workspace_manifest =
        LocalWorkspaceManifest::decrypt_and_load(&encrypted, &alice.local_symkey).unwrap();

    let expected = LocalWorkspaceManifest {
        base: WorkspaceManifest {
            author: alice.device_id.clone(),
            timestamp: workspace_manifest.updated,
            id: realm_id,
            version: 0,
            created: workspace_manifest.updated,
            updated: workspace_manifest.updated,
            children: HashMap::new(),
        },
        need_sync: true,
        updated: workspace_manifest.updated,
        children: HashMap::new(),
        local_confinement_points: HashSet::new(),
        remote_confinement_points: HashSet::new(),
        speculative: false,
    };
    p_assert_eq!(workspace_manifest, expected);
}

#[cfg(not(target_arch = "wasm32"))]
#[parsec_test]
async fn bad_start(tmp_path: TmpPath, alice: &Device) {
    let realm_id = VlobID::from_hex("aa0000000000000000000000000000ff").unwrap();

    // 1) Bad path

    let not_a_dir_path = tmp_path.join("foo.txt");
    std::fs::File::create(&not_a_dir_path).unwrap();

    p_assert_matches!(
        WorkspaceStorage::start(&not_a_dir_path, &alice.local_device(), realm_id, u64::MAX).await,
        Err(_)
    );

    // TODO: create a valid database, then modify workspace manifest's vlob to
    // turn it into something invalid:
    // - invalid schema
    // - invalid encryption

    // TODO: modify the database to make its schema invalid

    // TODO: drop the database so that it exists but it is empty, this shouldn't cause any issue

    // TODO: remove workspace manifest's vlob from the database, this shouldn't cause any issue
}

// TODO: test get/set blocks
