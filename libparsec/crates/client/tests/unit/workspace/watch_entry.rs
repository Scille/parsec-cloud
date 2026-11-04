// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::{
    test_register_sequence_of_send_hooks, test_send_hook_realm_get_keys_bundle,
    test_send_hook_vlob_read_batch,
};
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;
use crate::{
    EventWorkspaceOpsInboundSyncDone, EventWorkspaceOpsOutboundSyncNeeded,
    EventWorkspaceWatchedEntryChanged,
    workspace::{EntryStat, MoveEntryMode, OpenOptions, WorkspaceStatEntryError},
};

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok_local_change(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_bar_txt_id: VlobID = *env.template.get_stuff("wksp1_bar_txt_id");

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let mut spy = ops.event_bus.spy.start_expecting();

    // 1) Watch multiple things at the same time

    p_assert_eq!(
        ops.watch_entry_oneshot(&"/".parse().unwrap())
            .await
            .unwrap(),
        wksp1_id,
    );
    p_assert_eq!(
        ops.watch_entry_oneshot(&"/foo".parse().unwrap())
            .await
            .unwrap(),
        wksp1_foo_id,
    );
    p_assert_eq!(
        ops.watch_entry_oneshot(&"/bar.txt".parse().unwrap())
            .await
            .unwrap(),
        wksp1_bar_txt_id,
    );

    spy.assert_no_events();

    // 2) Change in workspace

    ops.create_file("/new.txt".parse().unwrap()).await.unwrap();
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
    });
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, wksp1_id);
    });
    spy.assert_next(|e: &EventWorkspaceWatchedEntryChanged| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, wksp1_id);
    });

    // Subsequent change is ignored

    ops.create_file("/new2.txt".parse().unwrap()).await.unwrap();
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
    });
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, wksp1_id);
    });
    spy.assert_no_events();

    // 3) Change in folder

    ops.move_entry(
        "/foo/spam".parse().unwrap(),
        "/foo/spam2".parse().unwrap(),
        MoveEntryMode::NoReplace,
    )
    .await
    .unwrap();
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, wksp1_foo_id);
    });
    spy.assert_next(|e: &EventWorkspaceWatchedEntryChanged| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, wksp1_foo_id);
    });

    // Subsequent change is ignored

    ops.move_entry(
        "/foo/spam2".parse().unwrap(),
        "/foo/spam3".parse().unwrap(),
        MoveEntryMode::NoReplace,
    )
    .await
    .unwrap();
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, wksp1_foo_id);
    });
    spy.assert_no_events();

    // 4) Change in file

    let fd = ops
        .open_file("/bar.txt".parse().unwrap(), OpenOptions::read_write())
        .await
        .unwrap();
    spy.assert_no_events();

    ops.fd_resize(fd, 0, false).await.unwrap();
    spy.assert_no_events();

    ops.fd_close(fd).await.unwrap();
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, wksp1_bar_txt_id);
    });
    spy.assert_next(|e: &EventWorkspaceWatchedEntryChanged| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, wksp1_bar_txt_id);
    });

    // Subsequent change is ignored
    let fd = ops
        .open_file("/bar.txt".parse().unwrap(), OpenOptions::read_write())
        .await
        .unwrap();
    ops.fd_write(fd, 0, b"dummy").await.unwrap();
    ops.fd_close(fd).await.unwrap();
    spy.assert_next(|e: &EventWorkspaceOpsOutboundSyncNeeded| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, wksp1_bar_txt_id);
    });
    spy.assert_no_events();
}

#[parsec_test(testbed = "minimal_client_ready", with_server)]
async fn ok_remote_change_self(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    env.customize(|builder| {
        builder
            .create_or_update_workspace_manifest_vlob("alice@dev1", wksp1_id)
            .customize(|e| {
                let manifest = Arc::make_mut(&mut e.manifest);
                let bar_txt_id = manifest
                    .children
                    .remove(&"bar.txt".parse().unwrap())
                    .unwrap();
                manifest
                    .children
                    .insert("bar2.txt".parse().unwrap(), bar_txt_id);
            });
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let mut spy = ops.event_bus.spy.start_expecting();

    // 1) Start watching

    p_assert_eq!(
        ops.watch_entry_oneshot(&"/".parse().unwrap())
            .await
            .unwrap(),
        wksp1_id,
    );
    spy.assert_no_events();

    // 2) Fetch the remote change

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        test_send_hook_vlob_read_batch!(env, wksp1_id, wksp1_id),
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
    );

    ops.inbound_sync(wksp1_id).await.unwrap();

    spy.assert_next(|e: &EventWorkspaceOpsInboundSyncDone| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, wksp1_id);
    });
    spy.assert_next(|e: &EventWorkspaceWatchedEntryChanged| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, wksp1_id);
    });

    spy.assert_no_events();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok_remote_change_newly_created_child(env: &TestbedEnv) {
    // A folder manifest can contain children with invalid entry ID.
    // This can be due to two things:
    // - The entry is invalid (e.g. uploaded by some malicious client, or a user
    //   was revoked while synchronizing data)
    // - The entry will eventually become valid.
    //
    // This test is focusing on the second case, so the idea is:
    // - Client contains a root manifest referencing an invalid entry.
    // - Client starts watch the root folder.
    // - Client does an inbound sync to fetch the child. At this point the root
    //   has changed event if its manifest hasn't (since listing the root folder
    //   produce a different outcome).

    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let new_child_txt_id = VlobID::default();

    env.customize(|builder| {
        // We add a new entry `new_child.txt` to the root folder...
        builder
            .create_or_update_workspace_manifest_vlob("alice@dev1", wksp1_id)
            .customize(|e| {
                let manifest = Arc::make_mut(&mut e.manifest);
                manifest
                    .children
                    .insert("new_child.txt".parse().unwrap(), new_child_txt_id);
            });
        builder.create_or_update_file_manifest_vlob(
            "alice@dev1",
            wksp1_id,
            Some(new_child_txt_id),
            wksp1_id,
        );

        // ...however only the root folder manifest is up to date on the client !
        builder.workspace_data_storage_fetch_workspace_vlob(
            "alice@dev1",
            wksp1_id,
            libparsec_types::PreventSyncPattern::empty(),
        );
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id.to_owned()).await;

    let mut spy = ops.event_bus.spy.start_expecting();

    // 1) Start watching

    p_assert_eq!(
        ops.watch_entry_oneshot(&"/".parse().unwrap())
            .await
            .unwrap(),
        wksp1_id,
    );
    spy.assert_no_events();

    let last_common_certificate_timestamp = env.get_last_common_certificate_timestamp();
    let last_realm_certificate_timestamp = env.get_last_realm_certificate_timestamp(wksp1_id);
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // First time `new_child.txt` is tried to be accessed is during the sanity
        // check. At that time we pretend the vlob hasn't been uploaded yet.
        {
            move |req: authenticated_cmds::latest::vlob_read_batch::Req| {
                p_assert_eq!(req.realm_id, wksp1_id);
                p_assert_eq!(req.vlobs, vec![new_child_txt_id]);

                authenticated_cmds::latest::vlob_read_batch::Rep::Ok {
                    items: vec![],
                    needed_common_certificate_timestamp: last_common_certificate_timestamp,
                    needed_realm_certificate_timestamp: last_realm_certificate_timestamp,
                }
            }
        },
        // Now should be during the actual inbound sync, at this point we
        // can provide the `new_child.txt`'s vlob.
        test_send_hook_vlob_read_batch!(env, wksp1_id, new_child_txt_id),
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
    );

    // Sanity check: root folder is up to date, child is not present yet
    p_assert_matches!(
        ops.stat_entry(&"/".parse().unwrap()).await,
        Ok(EntryStat::Folder { base_version, need_sync, .. }) if base_version == 2 && !need_sync
    );
    p_assert_matches!(
        ops.stat_entry(&"/new_child.txt".parse().unwrap()).await,
        Err(WorkspaceStatEntryError::EntryNotFound)
    );

    // 2) Fetch the remote change

    ops.inbound_sync(new_child_txt_id).await.unwrap();

    spy.assert_next(|e: &EventWorkspaceOpsInboundSyncDone| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, new_child_txt_id);
    });
    spy.assert_next(|e: &EventWorkspaceWatchedEntryChanged| {
        p_assert_eq!(e.realm_id, wksp1_id);
        p_assert_eq!(e.entry_id, wksp1_id);
    });

    spy.assert_no_events();
}

// Note there is no `ok_local_change_newly_created_child` test, this is because
// local changes modify child and parent manifests atomically.
// Hence it is not possible to have a local modification leading to a folder
// referencing a non-existing entry.
