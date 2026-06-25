// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::{
    test_register_sequence_of_send_hooks, test_send_hook_realm_get_keys_bundle,
    test_send_hook_vlob_read_batch,
};
use libparsec_platform_async::prelude::*;
use libparsec_platform_storage::workspace::WorkspaceOutboundSyncBacklog;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::workspace::{OpenOptions, WorkspaceOps};

use super::utils::{restart_workspace_ops, workspace_ops_factory};

async fn mutate_file_locally(ops: &WorkspaceOps, entry_id: VlobID) {
    let fd = ops
        .open_file_by_id(entry_id, OpenOptions::read_write())
        .await
        .unwrap();
    ops.fd_write(fd, 0, b"backlog-test").await.unwrap();
    ops.fd_flush(fd).await.unwrap();
    ops.fd_close(fd).await.unwrap();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn outbound_sync_backlog_cache_survives_restart_after_local_write(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");

    let wksp1_ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    p_assert_eq!(
        wksp1_ops.get_outbound_sync_backlog().await.unwrap(),
        WorkspaceOutboundSyncBacklog {
            pending_entries: 0,
            pending_bytes: 0,
        }
    );

    mutate_file_locally(&wksp1_ops, wksp1_bar_txt_id).await;

    let backlog_before_restart = wksp1_ops.get_outbound_sync_backlog().await.unwrap();
    p_assert_eq!(backlog_before_restart.pending_entries, 1);
    assert!(backlog_before_restart.pending_bytes > 0);

    let wksp1_ops = restart_workspace_ops(wksp1_ops).await;

    p_assert_eq!(
        wksp1_ops.get_outbound_sync_backlog().await.unwrap(),
        backlog_before_restart
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn outbound_sync_backlog_cache_decreases_after_simulated_sync(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");

    let wksp1_ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;
    mutate_file_locally(&wksp1_ops, wksp1_bar_txt_id).await;

    let (updater, manifest) = wksp1_ops
        .store
        .for_update_sync_local_only(wksp1_bar_txt_id)
        .await
        .unwrap();
    let manifest = match manifest.unwrap() {
        ArcLocalChildManifest::File(manifest) => manifest,
        ArcLocalChildManifest::Folder(_) => panic!("expected a file manifest"),
    };
    let chunk_views: Vec<_> = manifest
        .blocks
        .iter()
        .flat_map(|block| block.iter().map(|chunk| (chunk.id, chunk.raw_size.get())))
        .collect();
    let mut synced_manifest = (*manifest).clone();
    synced_manifest.need_sync = false;

    updater
        .update_file_manifest_and_chunks(Arc::new(synced_manifest), [].into_iter(), [].into_iter())
        .await
        .unwrap();
    for (chunk_id, _) in chunk_views {
        wksp1_ops
            .store
            .promote_local_only_chunk_to_uploaded_block(chunk_id)
            .await
            .unwrap();
    }

    p_assert_eq!(
        wksp1_ops.get_outbound_sync_backlog().await.unwrap(),
        WorkspaceOutboundSyncBacklog {
            pending_entries: 0,
            pending_bytes: 0,
        }
    );

    let wksp1_ops = restart_workspace_ops(wksp1_ops).await;
    p_assert_eq!(
        wksp1_ops.get_outbound_sync_backlog().await.unwrap(),
        WorkspaceOutboundSyncBacklog {
            pending_entries: 0,
            pending_bytes: 0,
        }
    );
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn populate_with_concurrency(
    #[values(
        "concurrent_populates_current_version",
        "concurrent_populates_old_version",
        "concurrent_populates_locally_modified_version"
    )]
    kind: &str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    // Customize the testbed so that alice@dev1's local storage contains a workspace
    // manifest referencing `new.txt`, but `new.txt`'s manifest is missing in local.
    let (
        wksp1_new_txt_id,
        concurrent_vlob_read_batch_server_up_to,
        concurrent_local_modification,
        concurrent_populated_version,
    ) = env
        .customize(|builder| {
            let wksp1_new_txt_id = builder.counters.next_entry_id();

            builder
                .create_or_update_workspace_manifest_vlob("alice@dev1", wksp1_id)
                .customize_children([("new.txt", Some(wksp1_new_txt_id))].into_iter());
            builder.workspace_data_storage_fetch_workspace_vlob(
                "alice@dev1",
                wksp1_id,
                libparsec_types::PreventSyncPattern::empty(),
            );

            // `new.txt` version 1
            let wksp1_new_txt_v1_timestamp = builder
                .create_or_update_file_manifest_vlob(
                    "alice@dev1",
                    wksp1_id,
                    Some(wksp1_new_txt_id),
                    Some(wksp1_id),
                )
                .map(|e| e.manifest.timestamp);
            // `new.txt` version 2
            let wksp1_new_txt_v2_timestamp = builder
                .create_or_update_file_manifest_vlob(
                    "alice@dev1",
                    wksp1_id,
                    Some(wksp1_new_txt_id),
                    None,
                )
                .map(|e| e.manifest.timestamp);

            match kind {
                "concurrent_populates_old_version" => {
                    (wksp1_new_txt_id, wksp1_new_txt_v1_timestamp, false, 1)
                }
                "concurrent_populates_current_version" => {
                    (wksp1_new_txt_id, wksp1_new_txt_v2_timestamp, false, 2)
                }
                "concurrent_populates_locally_modified_version" => {
                    (wksp1_new_txt_id, wksp1_new_txt_v2_timestamp, true, 2)
                }
                unknown => panic!("Unknown kind: {unknown}"),
            }
        })
        .await;

    let alice = env.local_device("alice@dev1");
    let wksp1_ops =
        Arc::new(workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await);

    // 1) Configure the hook so that `new.txt` gets populated concurrently

    let concurrent_populate_requested = event::Event::<()>::new();
    let (concurrent_handle, concurrent_populated) = {
        let populate_requested = concurrent_populate_requested.listen();
        let populated = event::Event::<()>::new();
        let on_populated = populated.listen();
        let wksp1_ops = wksp1_ops.clone();

        let handle = spawn(async move {
            populate_requested.await;

            wksp1_ops
                .store
                .get_manifest(wksp1_new_txt_id)
                .await
                .unwrap();

            if concurrent_local_modification {
                let (updater, mut manifest) = wksp1_ops
                    .store
                    .for_update_file(wksp1_new_txt_id, true)
                    .await
                    .unwrap();

                {
                    let manifest = Arc::make_mut(&mut manifest);
                    manifest.need_sync = true;
                    manifest.size = 10;
                }

                updater
                    .update_file_manifest_and_continue(
                        &wksp1_ops.store,
                        manifest,
                        [].into_iter(),
                        [].into_iter(),
                    )
                    .await
                    .unwrap();
            }

            populated.notify(1);
        });

        (handle, on_populated)
    };

    libparsec_tests_fixtures::moment_inject_hook(
        Moment::WorkspaceStorePopulateCacheFetchRemote,
        async move {
            concurrent_populate_requested.notify(1);
            concurrent_populated.await;
        },
    );

    // 2) Configure server request

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // First vlob read is from the concurrent populate
        test_send_hook_vlob_read_batch!(env, server_up_to: concurrent_vlob_read_batch_server_up_to, wksp1_id, wksp1_new_txt_id),
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
        // Second vlob read is the main populate that got interrupted by the moment hook
        test_send_hook_vlob_read_batch!(env, wksp1_id, wksp1_new_txt_id),
        // Client has already loaded the realm keys bundle, so no additional server request
    );

    // 3) All ready ! Do the actual test !

    let (main_result, concurrent_result) = future::join(
        wksp1_ops.store.get_manifest(wksp1_new_txt_id),
        concurrent_handle,
    )
    .await;

    concurrent_result.unwrap();
    let manifest = main_result.unwrap();

    // Ensure the populate has kept the last version of the manifest
    let (expected_need_sync, expected_size) = if concurrent_local_modification {
        (true, 10)
    } else {
        (false, 0)
    };

    p_assert_matches!(
        manifest,
        ArcLocalChildManifest::File(manifest)
        if manifest.base.id == wksp1_new_txt_id && manifest.base.version == concurrent_populated_version
        && manifest.size == expected_size && manifest.need_sync == expected_need_sync
    );
}
