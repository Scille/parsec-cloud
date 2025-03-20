// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::collections::{HashMap, HashSet};

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{
    workspace::{
        DebugBlock, DebugChunk, DebugDump, DebugVlob, MarkPreventSyncPatternFullyAppliedError,
        UpdateManifestData,
    },
    PREVENT_SYNC_PATTERN_EMPTY_PATTERN,
};

use super::{workspace_storage_non_speculative_init, WorkspaceStorage};

#[cfg(target_arch = "wasm32")]
libparsec_tests_lite::platform::wasm_bindgen_test_configure!(run_in_browser);

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

    let (realm_id, block_id, chunk_id, file_id, folder_id) = env
        .customize(|builder| {
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
                    .create_or_update_file_manifest_vlob("alice@dev1", realm_id, None, realm_id)
                    .map(|e| e.manifest.id);
                let folder_id = builder
                    .create_or_update_folder_manifest_vlob("alice@dev1", realm_id, None, realm_id)
                    .map(|e| e.manifest.id);
                expected_version = 1;
                expected_checkpoint = 3;
                builder.workspace_data_storage_fetch_realm_checkpoint("alice@dev1", realm_id);
                builder.workspace_data_storage_fetch_workspace_vlob(
                    "alice@dev1",
                    realm_id,
                    libparsec_types::PreventSyncPattern::empty(),
                );
                builder.workspace_data_storage_fetch_file_vlob("alice@dev1", realm_id, file_id);
                builder.workspace_data_storage_fetch_folder_vlob(
                    "alice@dev1",
                    realm_id,
                    folder_id,
                    libparsec_types::PreventSyncPattern::empty(),
                );
                builder.workspace_cache_storage_fetch_block("alice@dev1", realm_id, block_id);

                if matches!(fetch_strategy, FetchStrategy::Multiple) {
                    builder.create_or_update_workspace_manifest_vlob("alice@dev1", realm_id);
                    builder.create_or_update_file_manifest_vlob(
                        "alice@dev1",
                        realm_id,
                        file_id,
                        None,
                    );
                    builder.create_or_update_folder_manifest_vlob(
                        "alice@dev1",
                        realm_id,
                        folder_id,
                        None,
                    );
                    expected_version = 2;
                    expected_checkpoint = 6;
                    builder.workspace_data_storage_fetch_realm_checkpoint("alice@dev1", realm_id);
                    builder.workspace_data_storage_fetch_workspace_vlob(
                        "alice@dev1",
                        realm_id,
                        libparsec_types::PreventSyncPattern::empty(),
                    );
                    builder.workspace_data_storage_fetch_file_vlob("alice@dev1", realm_id, file_id);
                    builder.workspace_data_storage_fetch_folder_vlob(
                        "alice@dev1",
                        realm_id,
                        folder_id,
                        libparsec_types::PreventSyncPattern::empty(),
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
                    .create_or_update_file_manifest_vlob(
                        "alice@dev1",
                        realm_id,
                        Some(*file_id),
                        None,
                    )
                    .map(|e| e.manifest.version);
                p_assert_ne!(expected_version, actual_version);
            }
            if let Some(folder_id) = &folder_id {
                let actual_version = builder
                    .create_or_update_folder_manifest_vlob(
                        "alice@dev1",
                        realm_id,
                        Some(*folder_id),
                        None,
                    )
                    .map(|e| e.manifest.version);
                p_assert_ne!(expected_version, actual_version);
            }

            (realm_id, block_id, chunk_id, file_id, folder_id)
        })
        .await;

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

    // File manifest

    if let Some(file_id) = file_id {
        let encrypted = workspace_storage
            .get_manifest(file_id)
            .await
            .unwrap()
            .unwrap();
        let file_manifest: LocalFileManifest =
            LocalChildManifest::decrypt_and_load(&encrypted, &alice.local_symkey)
                .unwrap()
                .try_into()
                .unwrap();
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
        let folder_manifest: LocalFolderManifest =
            LocalChildManifest::decrypt_and_load(&encrypted, &alice.local_symkey)
                .unwrap()
                .try_into()
                .unwrap();
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
    p_assert_eq!(dump, DebugDump::default());
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
        DebugDump {
            vlobs: vec![DebugVlob {
                id: entry_id,
                need_sync: false,
                base_version: 1,
                remote_version: 1
            }],
            ..Default::default()
        }
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
        DebugDump {
            vlobs: vec![DebugVlob {
                id: entry_id,
                need_sync: true,
                base_version: 2,
                remote_version: 2
            }],
            ..Default::default()
        }
    );
}

#[parsec_test(testbed = "minimal")]
async fn list_manifests(env: &TestbedEnv) {
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
        workspace_storage.list_manifests(0, 1).await.unwrap(),
        [b"<manifest1_v1>"]
    );
    p_assert_eq!(
        workspace_storage.list_manifests(1, 1).await.unwrap(),
        [b"<manifest2_v2>"]
    );

    p_assert_eq!(
        workspace_storage.list_manifests(0, 10).await.unwrap(),
        [b"<manifest1_v1>", b"<manifest2_v2>"]
    );
    p_assert_eq!(
        workspace_storage.list_manifests(2, 10).await.unwrap(),
        [] as [&[u8]; 0]
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
        DebugDump {
            vlobs: vec![
                DebugVlob {
                    id: entry1_id,
                    need_sync: true,
                    base_version: 1,
                    remote_version: 1
                },
                DebugVlob {
                    id: entry2_id,
                    need_sync: false,
                    base_version: 2,
                    remote_version: 2
                }
            ],
            ..Default::default()
        }
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
        DebugDump {
            vlobs: vec![DebugVlob {
                id: entry_id,
                need_sync: true,
                base_version: 1,
                remote_version: 1
            }],
            chunks: vec![
                DebugChunk {
                    id: chunk1_id,
                    size: 8,
                    offline: false
                },
                DebugChunk {
                    id: chunk2_id,
                    size: 14,
                    offline: false
                },
            ],
            ..Default::default()
        }
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
        DebugDump {
            vlobs: vec![DebugVlob {
                id: entry_id,
                need_sync: true,
                base_version: 2,
                remote_version: 2
            }],
            chunks: vec![DebugChunk {
                id: chunk1_id,
                size: 8,
                offline: false
            }],
            ..Default::default()
        }
    );
}

#[parsec_test(testbed = "minimal")]
async fn promote_chunk_to_block(env: &TestbedEnv) {
    let realm_id = VlobID::from_hex("aa0000000000000000000000000000ee").unwrap();
    let entry_id = VlobID::from_hex("aa0000000000000000000000000000f1").unwrap();
    let alice = env.local_device("alice@dev1");

    let mut workspace_storage =
        WorkspaceStorage::start(&env.discriminant_dir, &alice, realm_id, u64::MAX)
            .await
            .unwrap();

    let chunk1_id = ChunkID::from_hex("aa0000000000000000000000000000c1").unwrap();
    let chunk2_id = ChunkID::from_hex("aa0000000000000000000000000000c2").unwrap();

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
        )
        .await
        .unwrap();

    let dump = workspace_storage.debug_dump().await.unwrap();
    p_assert_eq!(
        dump,
        DebugDump {
            checkpoint: 0,
            vlobs: vec![DebugVlob {
                id: entry_id,
                need_sync: true,
                base_version: 1,
                remote_version: 1
            }],
            chunks: vec![
                DebugChunk {
                    id: chunk1_id,
                    size: 8,
                    offline: false
                },
                DebugChunk {
                    id: chunk2_id,
                    size: 14,
                    offline: false
                },
            ],
            ..Default::default()
        }
    );

    // 2) Promote chunk

    workspace_storage
        .promote_chunk_to_block(chunk1_id, "2000-01-31T00:00:00Z".parse().unwrap())
        .await
        .unwrap();

    p_assert_eq!(workspace_storage.get_chunk(chunk1_id).await.unwrap(), None);

    let dump = workspace_storage.debug_dump().await.unwrap();
    p_assert_eq!(
        dump,
        DebugDump {
            checkpoint: 0,
            vlobs: vec![DebugVlob {
                id: entry_id,
                need_sync: true,
                base_version: 1,
                remote_version: 1
            }],
            chunks: vec![DebugChunk {
                id: chunk2_id,
                size: 14,
                offline: false
            },],
            blocks: vec![DebugBlock {
                id: chunk1_id.into(),
                size: 8,
                offline: false,
                accessed_on: "2000-01-31T00:00:00Z".into()
            }]
        }
    );

    p_assert_eq!(
        workspace_storage
            .get_block(chunk1_id.into(), alice.now())
            .await
            .unwrap(),
        Some(b"<chunk1>".to_vec())
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
        DebugDump {
            chunks: vec![DebugChunk {
                id: chunk_id,
                size: 8,
                offline: false
            }],
            ..Default::default()
        }
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
        DebugDump {
            blocks: vec![DebugBlock {
                id: block_id,
                size: 8,
                offline: false,
                accessed_on: "2000-01-01T00:00:00Z".into()
            }],
            ..Default::default()
        }
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
        DebugDump {
            blocks: vec![DebugBlock {
                id: block_id,
                size: 8,
                offline: false,
                // The access date should have changed.
                accessed_on: "2000-01-02T00:00:00Z".into()
            }],
            ..Default::default()
        }
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
    let block3_id = insert_block!(
        "aa0000000000000000000000000000f3",
        block_size,
        "2000-01-03T00:00:00Z"
    );

    // 2) Add one more block that should trigger the cleanup and remove block 1

    let block4_id = insert_block!(
        "aa0000000000000000000000000000f4",
        block_size,
        "2000-01-04T00:00:00Z"
    );

    let dump = workspace_storage.debug_dump().await.unwrap();
    p_assert_eq!(
        dump,
        DebugDump {
            blocks: vec![
                DebugBlock {
                    id: block2_id,
                    size: 524288,
                    offline: false,
                    accessed_on: "2000-01-02T00:00:00Z".into()
                },
                DebugBlock {
                    id: block3_id,
                    size: 524288,
                    offline: false,
                    accessed_on: "2000-01-03T00:00:00Z".into()
                },
                DebugBlock {
                    id: block4_id,
                    size: 524288,
                    offline: false,
                    accessed_on: "2000-01-04T00:00:00Z".into()
                },
            ],
            ..Default::default()
        }
    );

    // 3) Accessing a block should prevent it from being the one to be removed

    workspace_storage
        .get_block(block2_id, "2000-01-05T00:00:00Z".parse().unwrap())
        .await
        .unwrap();

    let block5_id = insert_block!(
        "aa0000000000000000000000000000f5",
        block_size,
        "2000-01-06T00:00:00Z"
    );

    let dump = workspace_storage.debug_dump().await.unwrap();
    p_assert_eq!(
        dump,
        DebugDump {
            blocks: vec![
                DebugBlock {
                    id: block2_id,
                    size: 524288,
                    offline: false,
                    accessed_on: "2000-01-05T00:00:00Z".into()
                },
                DebugBlock {
                    id: block4_id,
                    size: 524288,
                    offline: false,
                    accessed_on: "2000-01-04T00:00:00Z".into()
                },
                DebugBlock {
                    id: block5_id,
                    size: 524288,
                    offline: false,
                    accessed_on: "2000-01-06T00:00:00Z".into()
                },
            ],
            ..Default::default()
        }
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

    let mut storage = WorkspaceStorage::start(&env.discriminant_dir, &alice, realm_id, u64::MAX)
        .await
        .unwrap();

    p_assert_eq!(storage.get_realm_checkpoint().await.unwrap(), 0);

    let encrypted = storage.get_manifest(realm_id).await.unwrap().unwrap();
    let workspace_manifest: LocalFolderManifest =
        LocalWorkspaceManifest::decrypt_and_load(&encrypted, &alice.local_symkey)
            .unwrap()
            .into();

    let expected = LocalFolderManifest {
        base: FolderManifest {
            author: alice.device_id,
            timestamp: workspace_manifest.updated,
            id: realm_id,
            parent: realm_id,
            version: 0,
            created: workspace_manifest.updated,
            updated: workspace_manifest.updated,
            children: HashMap::new(),
        },
        parent: realm_id,
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

#[cfg(not(target_arch = "wasm32"))]
#[parsec_test]
async fn start_with_on_disk_db(tmp_path: TmpPath, alice: &Device) {
    let realm_id = VlobID::from_hex("aa0000000000000000000000000000ff").unwrap();

    // Start when the db file does not exist
    let storage = WorkspaceStorage::start(&tmp_path, &alice.local_device(), realm_id, u64::MAX)
        .await
        .unwrap();
    storage.stop().await.unwrap();

    // Check the db files have been created
    assert!(tmp_path.join("de10a11cec0010000000000000000000/aa0000000000000000000000000000ff/workspace_data-v1.sqlite").exists());
    assert!(tmp_path.join("de10a11cec0010000000000000000000/aa0000000000000000000000000000ff/workspace_cache-v1.sqlite").exists());

    // Start when the db file already exists
    let storage = WorkspaceStorage::start(&tmp_path, &alice.local_device(), realm_id, u64::MAX)
        .await
        .unwrap();
    storage.stop().await.unwrap();
}

async fn start_workspace(env: &TestbedEnv) -> WorkspaceStorage {
    let realm_id = VlobID::from_hex("aa0000000000000000000000000000ee").unwrap();
    let alice = env.local_device("alice@dev1");

    WorkspaceStorage::start(&env.discriminant_dir, &alice, realm_id, u64::MAX)
        .await
        .unwrap()
}

#[parsec_test(testbed = "minimal")]
async fn check_prevent_sync_pattern_initialized_with_empty_pattern(env: &TestbedEnv) {
    let mut workspace = start_workspace(env).await;

    let (regex, bool) = workspace.get_prevent_sync_pattern().await.unwrap();

    p_assert_eq!(
        regex,
        PreventSyncPattern::from_regex(PREVENT_SYNC_PATTERN_EMPTY_PATTERN).unwrap()
    );
    assert!(!bool);
}

#[parsec_test(testbed = "minimal")]
async fn mark_empty_pattern_as_fully_applied(env: &TestbedEnv) {
    let mut workspace = start_workspace(env).await;

    let empty_pattern = PreventSyncPattern::from_regex(PREVENT_SYNC_PATTERN_EMPTY_PATTERN).unwrap();

    let res = workspace
        .mark_prevent_sync_pattern_fully_applied(&empty_pattern)
        .await;

    p_assert_matches!(res, Ok(()));
}

/// Using `set_prevent_sync_pattern` should not change the `fully applied` status if the pattern is the same.
#[parsec_test(testbed = "minimal")]
async fn check_set_pattern_is_idempotent(env: &TestbedEnv) {
    let mut workspace = start_workspace(env).await;

    // 1st, mark the empty pattern as fully applied.
    let empty_pattern = PreventSyncPattern::from_regex(PREVENT_SYNC_PATTERN_EMPTY_PATTERN).unwrap();

    let res = workspace
        .mark_prevent_sync_pattern_fully_applied(&empty_pattern)
        .await;

    p_assert_matches!(res, Ok(()));

    // 2nd, set the empty pattern again.
    let res = workspace
        .set_prevent_sync_pattern(&empty_pattern)
        .await
        .unwrap();

    assert!(res);
}

#[parsec_test(testbed = "minimal")]
async fn set_prevent_sync_pattern(env: &TestbedEnv) {
    let mut workspace = start_workspace(env).await;

    let regex = PreventSyncPattern::from_regex(r".*\.tmp$").unwrap();

    let res = workspace.set_prevent_sync_pattern(&regex).await.unwrap();

    assert!(!res);
    let (got_regex, bool) = workspace.get_prevent_sync_pattern().await.unwrap();

    assert_eq!(got_regex, regex);
    assert!(!bool);
}

#[parsec_test(testbed = "minimal")]
async fn nop_mark_prevent_sync_pattern_with_different_pat(env: &TestbedEnv) {
    let mut workspace = start_workspace(env).await;

    let regex = PreventSyncPattern::from_regex(r".*\.tmp$").unwrap();

    let res = workspace
        .mark_prevent_sync_pattern_fully_applied(&regex)
        .await;

    p_assert_matches!(
        res,
        Err(MarkPreventSyncPatternFullyAppliedError::PatternMismatch)
    );
}

// We test inbound and outbound together to ensure the two are not stepping on each other
#[parsec_test(testbed = "minimal")]
async fn get_inbound_outbound_need_sync(env: &TestbedEnv) {
    let realm_id = VlobID::from_hex("aa0000000000000000000000000000ee").unwrap();
    let vlob1_id = VlobID::from_hex("aa000000000000000000000000000011").unwrap();
    let vlob2_id = VlobID::from_hex("aa000000000000000000000000000022").unwrap();
    let vlob3_id = VlobID::from_hex("aa000000000000000000000000000033").unwrap();
    let vlob4_id = VlobID::from_hex("aa000000000000000000000000000044").unwrap();
    let vlob5_id = VlobID::from_hex("aa000000000000000000000000000055").unwrap();
    let alice = env.local_device("alice@dev1");

    let mut workspace_storage =
        WorkspaceStorage::start(&env.discriminant_dir, &alice, realm_id, u64::MAX)
            .await
            .unwrap();

    macro_rules! assert_outbound_need_sync {
        ($expected: expr) => {
            async {
                let expected: Vec<VlobID> = $expected;
                assert!(expected.len() <= 100); // Sanity check
                p_assert_eq!(
                    workspace_storage.get_outbound_need_sync(100).await.unwrap(),
                    expected
                );
                p_assert_eq!(
                    workspace_storage.get_outbound_need_sync(1).await.unwrap(),
                    match expected.first() {
                        None => vec![],
                        Some(&first) => vec![first],
                    }
                );
                p_assert_eq!(
                    workspace_storage.get_outbound_need_sync(0).await.unwrap(),
                    vec![]
                );
            }
        };
    }

    macro_rules! assert_inbound_need_sync {
        ($expected: expr) => {
            async {
                let expected: Vec<VlobID> = $expected;
                assert!(expected.len() <= 100); // Sanity check
                p_assert_eq!(
                    workspace_storage.get_inbound_need_sync(100).await.unwrap(),
                    expected
                );
                p_assert_eq!(
                    workspace_storage.get_inbound_need_sync(1).await.unwrap(),
                    match expected.first() {
                        None => vec![],
                        Some(&first) => vec![first],
                    }
                );
                p_assert_eq!(
                    workspace_storage.get_inbound_need_sync(0).await.unwrap(),
                    vec![]
                );
            }
        };
    }

    // 1) Storage starts empty

    assert_outbound_need_sync!(vec![]).await;
    assert_inbound_need_sync!(vec![]).await;

    // 2) Add items that doesn't need sync

    workspace_storage
        .update_manifest(&UpdateManifestData {
            entry_id: vlob1_id,
            encrypted: b"<vlob1_v1>".to_vec(),
            need_sync: false,
            base_version: 1,
        })
        .await
        .unwrap();

    workspace_storage
        .update_manifest(&UpdateManifestData {
            entry_id: vlob2_id,
            encrypted: b"<vlob2_v2>".to_vec(),
            need_sync: false,
            base_version: 2,
        })
        .await
        .unwrap();

    workspace_storage
        .update_manifest(&UpdateManifestData {
            entry_id: vlob3_id,
            encrypted: b"<vlob3_v1>".to_vec(),
            need_sync: false,
            base_version: 1,
        })
        .await
        .unwrap();

    workspace_storage
        .update_manifest(&UpdateManifestData {
            entry_id: vlob4_id,
            encrypted: b"<vlob4_v4>".to_vec(),
            need_sync: false,
            base_version: 4,
        })
        .await
        .unwrap();

    assert_outbound_need_sync!(vec![]).await;
    assert_inbound_need_sync!(vec![]).await;

    // 4) Add inbound need sync

    workspace_storage
        .update_realm_checkpoint(
            1,
            &[
                (vlob1_id, 1),  // Noop since version is the same
                (vlob2_id, 3),  // Need sync !
                (vlob3_id, 42), // Need sync !
                (vlob4_id, 3),  // Smaller version should also trigger inbound sync
            ],
        )
        .await
        .unwrap();

    assert_outbound_need_sync!(vec![]).await;
    assert_inbound_need_sync!(vec![vlob2_id, vlob3_id, vlob4_id]).await;

    // 3) Add outbound need sync

    workspace_storage
        .update_manifest(&UpdateManifestData {
            entry_id: vlob1_id,
            encrypted: b"<vlob1_v1 locally modified>".to_vec(),
            need_sync: true,
            base_version: 1,
        })
        .await
        .unwrap();

    workspace_storage
        .update_manifest(&UpdateManifestData {
            entry_id: vlob2_id,
            encrypted: b"<vlob2_v0 locally modified>".to_vec(),
            need_sync: true,
            base_version: 3, // Change in base version should impact inbound sync
        })
        .await
        .unwrap();

    workspace_storage
        .update_manifest(&UpdateManifestData {
            entry_id: vlob5_id,
            encrypted: b"<vlob5_v0 locally modified>".to_vec(),
            need_sync: true,
            base_version: 0,
        })
        .await
        .unwrap();

    assert_outbound_need_sync!(vec![vlob1_id, vlob2_id, vlob5_id]).await;
    assert_inbound_need_sync!(vec![vlob3_id, vlob4_id]).await;

    workspace_storage
        .update_manifest(&UpdateManifestData {
            entry_id: vlob5_id,
            encrypted: b"<vlob5_v1>".to_vec(),
            need_sync: false,
            base_version: 1,
        })
        .await
        .unwrap();

    assert_outbound_need_sync!(vec![vlob1_id, vlob2_id]).await;
    assert_inbound_need_sync!(vec![vlob3_id, vlob4_id]).await;
}
