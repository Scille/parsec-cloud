// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;
use crate::workspace::{OpenOptions, OutboundSyncOutcome};

#[parsec_test(testbed = "minimal_client_ready")]
async fn non_placeholder(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");
    let wksp1_ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    let get_file_manifest = |entry_id| {
        let wksp1_ops = &wksp1_ops;
        async move {
            let child_manifest = wksp1_ops.store.get_child_manifest(entry_id).await.unwrap();
            match child_manifest {
                ArcLocalChildManifest::File(m) => m,
                ArcLocalChildManifest::Folder(m) => panic!("Expected file, got {:?}", m),
            }
        }
    };

    let assert_local_and_base_blocks_similar = |manifest: &LocalFileManifest| {
        p_assert_eq!(manifest.blocks.len(), manifest.base.blocks.len());
        for (block_chunks, base_block) in manifest.blocks.iter().zip(manifest.base.blocks.iter()) {
            p_assert_eq!(block_chunks.len(), 1);

            p_assert_eq!(block_chunks[0].id, base_block.id.into());
            p_assert_eq!(block_chunks[0].start, block_chunks[0].raw_offset);
            p_assert_eq!(
                block_chunks[0].stop,
                block_chunks[0]
                    .raw_size
                    .checked_add(block_chunks[0].raw_offset)
                    .unwrap()
            );
            p_assert_eq!(block_chunks[0].raw_offset, base_block.offset);
            p_assert_eq!(block_chunks[0].raw_size, base_block.size);
            p_assert_eq!(block_chunks[0].access.as_ref(), Some(base_block));
        }
    };

    // Sanity checks
    let foo_manifest = get_file_manifest(wksp1_bar_txt_id).await;
    p_assert_eq!(foo_manifest.need_sync, false);
    p_assert_eq!(foo_manifest.base.version, 1);
    p_assert_eq!(foo_manifest.size, foo_manifest.base.size);
    assert_local_and_base_blocks_similar(&foo_manifest);
    let base_blocks = foo_manifest.base.blocks.clone();

    const NEW_DATA: &[u8] = b"new data";

    // Do a change requiring a sync
    let expected_size = {
        let options = OpenOptions {
            read: false,
            write: true,
            truncate: true,
            create: false,
            create_new: false,
        };
        let fd = wksp1_ops
            .open_file_by_id(wksp1_bar_txt_id, options)
            .await
            .unwrap();
        wksp1_ops.fd_write(fd, 0, NEW_DATA).await.unwrap();
        wksp1_ops.fd_close(fd).await.unwrap();
        NEW_DATA.len() as u64
    };

    // Check the workspace manifest is need sync
    let foo_manifest = get_file_manifest(wksp1_bar_txt_id).await;
    p_assert_eq!(foo_manifest.need_sync, true);
    p_assert_eq!(foo_manifest.base.version, 1);
    p_assert_eq!(foo_manifest.base.blocks, base_blocks);
    p_assert_eq!(foo_manifest.size, expected_size);

    // Mock server commands and do the sync

    let expected_base_blocks = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Fetch last workspace keys bundle to encrypt the new manifest
        {
            let keys_bundle = env.get_last_realm_keys_bundle(wksp1_id);
            let keys_bundle_access =
                env.get_last_realm_keys_bundle_access_for(wksp1_id, alice.user_id());
            move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                p_assert_eq!(req.realm_id, wksp1_id);
                p_assert_eq!(req.key_index, 1);
                authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                    keys_bundle,
                    keys_bundle_access,
                }
            }
        },
        // 3) `block_create`
        {
            let expected_base_blocks = expected_base_blocks.clone();
            let (key, key_index) = env.get_last_realm_key(wksp1_id);
            let key = key.to_owned();
            move |req: authenticated_cmds::latest::block_create::Req| {
                p_assert_eq!(key.decrypt(&req.block).unwrap(), NEW_DATA);
                expected_base_blocks.lock().unwrap().push(BlockAccess {
                    id: req.block_id,
                    key: None,
                    key_index,
                    offset: 0,
                    size: (NEW_DATA.len() as u64).try_into().unwrap(),
                    digest: HashDigest::from_data(NEW_DATA),
                });
                authenticated_cmds::latest::block_create::Rep::Ok {}
            }
        },
        // 2) `vlob_update` succeed on first try !
        move |req: authenticated_cmds::latest::vlob_update::Req| {
            p_assert_eq!(req.key_index, 1);
            p_assert_eq!(req.vlob_id, wksp1_bar_txt_id);
            p_assert_eq!(req.version, 2);
            assert!(req.sequester_blob.is_none());
            authenticated_cmds::latest::vlob_update::Rep::Ok {}
        },
    );

    let outcome = wksp1_ops.outbound_sync(wksp1_bar_txt_id).await.unwrap();
    p_assert_matches!(outcome, OutboundSyncOutcome::Done);

    // Check the user manifest is not longer need sync
    let foo_manifest = get_file_manifest(wksp1_bar_txt_id).await;
    p_assert_eq!(foo_manifest.need_sync, false);
    p_assert_eq!(foo_manifest.base.version, 2);
    assert_local_and_base_blocks_similar(&foo_manifest);
    p_assert_eq!(
        foo_manifest.base.blocks,
        *expected_base_blocks.lock().unwrap()
    );
    p_assert_eq!(foo_manifest.size, expected_size);

    // Subsequent sync is an idempotent noop

    let outcome = wksp1_ops.outbound_sync(wksp1_bar_txt_id).await.unwrap();
    p_assert_matches!(outcome, OutboundSyncOutcome::Done);

    wksp1_ops.stop().await.unwrap();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn inbound_sync_needed(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");

    let env = env.customize(|builder| {
        // New version of the file that we our client doesn't know about
        builder.create_or_update_file_manifest_vlob("alice@dev1", wksp1_id, wksp1_bar_txt_id);
    });

    let alice = env.local_device("alice@dev1");
    let wksp1_ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    let get_file_manifest = |entry_id| {
        let wksp1_ops = &wksp1_ops;
        async move {
            let child_manifest = wksp1_ops.store.get_child_manifest(entry_id).await.unwrap();
            match child_manifest {
                ArcLocalChildManifest::File(m) => m,
                ArcLocalChildManifest::Folder(m) => panic!("Expected file, got {:?}", m),
            }
        }
    };

    // Truncate the file to require a sync

    let options = OpenOptions {
        read: false,
        write: true,
        truncate: true,
        create: false,
        create_new: false,
    };
    let fd = wksp1_ops
        .open_file_by_id(wksp1_bar_txt_id, options)
        .await
        .unwrap();
    wksp1_ops.fd_close(fd).await.unwrap();

    // Mock server commands and do the sync

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Fetch last workspace keys bundle to encrypt the new manifest
        {
            let keys_bundle = env.get_last_realm_keys_bundle(wksp1_id);
            let keys_bundle_access =
                env.get_last_realm_keys_bundle_access_for(wksp1_id, alice.user_id());
            move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                p_assert_eq!(req.realm_id, wksp1_id);
                p_assert_eq!(req.key_index, 1);
                authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                    keys_bundle,
                    keys_bundle_access,
                }
            }
        },
        // No `block_create` as we only truncated the file
        // 2) `vlob_update`
        move |req: authenticated_cmds::latest::vlob_update::Req| {
            p_assert_eq!(req.key_index, 1);
            p_assert_eq!(req.vlob_id, wksp1_bar_txt_id);
            p_assert_eq!(req.version, 2);
            assert!(req.sequester_blob.is_none());
            authenticated_cmds::latest::vlob_update::Rep::BadVlobVersion
        },
    );

    let before_sync_manifest = get_file_manifest(wksp1_bar_txt_id).await;
    p_assert_eq!(before_sync_manifest.need_sync, true); // Sanity check

    let outcome = wksp1_ops.outbound_sync(wksp1_bar_txt_id).await.unwrap();
    p_assert_matches!(outcome, OutboundSyncOutcome::InboundSyncNeeded);

    // Check the user manifest still need sync
    let after_failed_sync_manifest = get_file_manifest(wksp1_bar_txt_id).await;
    p_assert_eq!(after_failed_sync_manifest, before_sync_manifest);

    wksp1_ops.stop().await.unwrap();
}

// TODO: test with placeholder folder manifest
// TODO: test `OutboundSyncOutcome::EntryIsBusy`
