// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, sync::Arc};

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks,
    test_send_hook_realm_get_keys_bundle,
};
use libparsec_platform_async::prelude::*;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;
use crate::{
    EventWorkspaceOpsInboundSyncDone, EventWorkspaceOpsOutboundSyncAborted,
    EventWorkspaceOpsOutboundSyncDone, EventWorkspaceOpsOutboundSyncNeeded,
    EventWorkspaceOpsOutboundSyncStarted, WorkspaceOps,
    workspace::{MoveEntryMode, OutboundSyncOutcome},
};

async fn assert_folder(
    ops: &WorkspaceOps,
    entry_id: VlobID,
    expected_need_sync: bool,
    expected_version: VersionInt,
    expected_children: &HashMap<EntryName, VlobID>,
) -> Arc<LocalFolderManifest> {
    let manifest = match ops.store.get_manifest(entry_id).await.unwrap() {
        ArcLocalChildManifest::Folder(m) => m,
        ArcLocalChildManifest::File(m) => panic!("Expected folder, got {:?}", m),
    };

    p_assert_eq!(manifest.need_sync, expected_need_sync);
    p_assert_eq!(manifest.base.version, expected_version);
    p_assert_eq!(manifest.children, *expected_children);
    if !expected_need_sync {
        p_assert_eq!(manifest.children, manifest.base.children);
    }

    manifest
}

enum Modification {
    Create,
    Remove,
    Rename,
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn non_placeholder(
    #[values(Modification::Create, Modification::Remove, Modification::Rename)]
    modification: Modification,
    env: &TestbedEnv,
) {
    let alice = env.local_device("alice@dev1");
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");
    let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");
    let wksp1_ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    // Sanity checks
    let base_children = HashMap::from_iter([
        ("spam".parse().unwrap(), wksp1_foo_spam_id),
        ("egg.txt".parse().unwrap(), wksp1_foo_egg_txt_id),
    ]);
    assert_folder(&wksp1_ops, wksp1_foo_id, false, 1, &base_children).await;

    // Do a change requiring a sync
    let expected_children = match modification {
        Modification::Create => {
            let new_folder_id = wksp1_ops
                .create_folder("/foo/new_folder".parse().unwrap())
                .await
                .unwrap();
            let mut expected_children = base_children.clone();
            expected_children.insert("new_folder".parse().unwrap(), new_folder_id);
            expected_children
        }
        Modification::Remove => {
            wksp1_ops
                .remove_entry("/foo/spam".parse().unwrap())
                .await
                .unwrap();
            let mut expected_children = base_children.clone();
            expected_children.remove(&"spam".parse().unwrap());
            expected_children
        }
        Modification::Rename => {
            wksp1_ops
                .move_entry(
                    "/foo/spam".parse().unwrap(),
                    "/foo/spam_renamed".parse().unwrap(),
                    MoveEntryMode::NoReplace,
                )
                .await
                .unwrap();
            let mut expected_children = base_children.clone();
            let spam_id = expected_children.remove(&"spam".parse().unwrap()).unwrap();
            expected_children.insert("spam_renamed".parse().unwrap(), spam_id);
            expected_children
        }
    };
    // Check the workspace manifest is need sync
    assert_folder(&wksp1_ops, wksp1_foo_id, true, 1, &expected_children).await;

    // Mock server command `vlob_update`
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Fetch last workspace keys bundle to encrypt the new manifest
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
        // 2) `vlob_update` succeed on first try !
        move |req: authenticated_cmds::latest::vlob_update::Req| {
            p_assert_eq!(req.key_index, 1);
            p_assert_eq!(req.vlob_id, wksp1_foo_id);
            p_assert_eq!(req.version, 2);
            authenticated_cmds::latest::vlob_update::Rep::Ok {}
        },
    );

    let mut spy = wksp1_ops.event_bus.spy.start_expecting();

    let outcome = wksp1_ops.outbound_sync(wksp1_foo_id).await.unwrap();
    p_assert_matches!(outcome, OutboundSyncOutcome::Done);
    spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncStarted| {
        p_assert_eq!(event.realm_id, wksp1_id);
        p_assert_eq!(event.entry_id, wksp1_foo_id);
    });
    spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncDone| {
        p_assert_eq!(event.realm_id, wksp1_id);
        p_assert_eq!(event.entry_id, wksp1_foo_id);
    });

    // Check the user manifest is not longer need sync
    assert_folder(&wksp1_ops, wksp1_foo_id, false, 2, &expected_children).await;

    // Subsequent sync is an idempotent noop
    let outcome = wksp1_ops.outbound_sync(wksp1_foo_id).await.unwrap();
    p_assert_matches!(outcome, OutboundSyncOutcome::Done);
    spy.assert_no_events();

    wksp1_ops.stop().await.unwrap();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn inbound_sync_needed(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");
    let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");

    env.customize(|builder| {
        // New version of the file that we our client doesn't know about
        builder.create_or_update_folder_manifest_vlob("alice@dev1", wksp1_id, wksp1_foo_id, None);
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let wksp1_ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    // Modify the folder to require a sync

    wksp1_ops
        .move_entry(
            "/foo/spam".parse().unwrap(),
            "/foo/spam_renamed".parse().unwrap(),
            MoveEntryMode::NoReplace,
        )
        .await
        .unwrap();

    // Mock server commands and do the sync

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Fetch last workspace keys bundle to encrypt the new manifest
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
        // 2) `vlob_update`
        move |req: authenticated_cmds::latest::vlob_update::Req| {
            p_assert_eq!(req.key_index, 1);
            p_assert_eq!(req.vlob_id, wksp1_foo_id);
            p_assert_eq!(req.version, 2);
            authenticated_cmds::latest::vlob_update::Rep::BadVlobVersion
        },
    );

    let expected_children = HashMap::from_iter([
        ("spam_renamed".parse().unwrap(), wksp1_foo_spam_id),
        ("egg.txt".parse().unwrap(), wksp1_foo_egg_txt_id),
    ]);
    let before_sync_manifest =
        assert_folder(&wksp1_ops, wksp1_foo_id, true, 1, &expected_children).await;

    let mut spy = wksp1_ops.event_bus.spy.start_expecting();

    let outcome = wksp1_ops.outbound_sync(wksp1_foo_id).await.unwrap();
    p_assert_matches!(outcome, OutboundSyncOutcome::InboundSyncNeeded);
    spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncStarted| {
        p_assert_eq!(event.realm_id, wksp1_id);
        p_assert_eq!(event.entry_id, wksp1_foo_id);
    });
    spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncAborted| {
        p_assert_eq!(event.realm_id, wksp1_id);
        p_assert_eq!(event.entry_id, wksp1_foo_id);
    });

    // Check the user manifest still need sync
    let after_failed_sync_manifest =
        assert_folder(&wksp1_ops, wksp1_foo_id, true, 1, &expected_children).await;
    p_assert_eq!(after_failed_sync_manifest, before_sync_manifest);

    wksp1_ops.stop().await.unwrap();
}

#[parsec_test(testbed = "minimal_client_ready", with_server)]
async fn entry_is_busy(#[values(false, true)] concurrent_write: bool, env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");
    let wksp1_foo_spam_id: VlobID = *env.template.get_stuff("wksp1_foo_spam_id");
    let wksp1_foo_egg_txt_id: VlobID = *env.template.get_stuff("wksp1_foo_egg_txt_id");
    let wksp1_ops = Arc::new(workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await);

    // Modify the folder to require a sync

    wksp1_ops
        .move_entry(
            "/foo/spam".parse().unwrap(),
            "/foo/spam_renamed".parse().unwrap(),
            MoveEntryMode::NoReplace,
        )
        .await
        .unwrap();

    // 1) Configure the hook so that the folder gets locked while synchronization
    //    has started (this simulates a concurrent operation on the folder).

    let concurrent_folder_lock_requested = event::Event::<()>::new();
    let concurrent_folder_unlock_requested = event::Event::<()>::new();
    let (concurrent_folder_locked, concurrent_folder_unlocked) = {
        let lock_requested = concurrent_folder_lock_requested.listen();
        let unlock_requested = concurrent_folder_unlock_requested.listen();
        let locked = event::Event::<()>::new();
        let on_locked = locked.listen();
        let unlocked = event::Event::<()>::new();
        let on_unlocked = unlocked.listen();
        let wksp1_ops = wksp1_ops.clone();

        spawn(async move {
            lock_requested.await;

            if concurrent_write {
                wksp1_ops
                    .move_entry(
                        "/foo/egg.txt".parse().unwrap(),
                        "/foo/egg_renamed.txt".parse().unwrap(),
                        MoveEntryMode::NoReplace,
                    )
                    .await
                    .unwrap();
            }
            // There is no equivalent to `open_file` for folder (as folder operations are
            // always short lived), hence we must use the internals to be able to lock the
            // folder for an arbitrary duration.
            let (updater, _) = wksp1_ops
                .store
                .for_update_folder(wksp1_foo_id)
                .await
                .unwrap();

            locked.notify(1);
            unlock_requested.await;

            drop(updater);

            unlocked.notify(1);
        });

        (on_locked, on_unlocked)
    };

    libparsec_tests_fixtures::moment_inject_hook(
        libparsec_tests_fixtures::Moment::OutboundSyncLocalRetrieved,
        async move {
            concurrent_folder_lock_requested.notify(1);
            concurrent_folder_locked.await;
        },
    );

    let egg_txt_current_name = if concurrent_write {
        "egg_renamed.txt"
    } else {
        "egg.txt"
    };
    let expected_children = HashMap::from_iter([
        ("spam_renamed".parse().unwrap(), wksp1_foo_spam_id),
        (egg_txt_current_name.parse().unwrap(), wksp1_foo_egg_txt_id),
    ]);

    // 2) The synchronization couldn't successfully finish

    {
        let mut spy = wksp1_ops.event_bus.spy.start_expecting();

        let outcome = wksp1_ops.outbound_sync(wksp1_foo_id).await.unwrap();
        p_assert_matches!(outcome, OutboundSyncOutcome::EntryIsBusy);

        if concurrent_write {
            spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncNeeded| {
                p_assert_eq!(event.realm_id, wksp1_id);
                p_assert_eq!(event.entry_id, wksp1_foo_id);
            });
        }
        spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncStarted| {
            p_assert_eq!(event.realm_id, wksp1_id);
            p_assert_eq!(event.entry_id, wksp1_foo_id);
        });
        spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncAborted| {
            p_assert_eq!(event.realm_id, wksp1_id);
            p_assert_eq!(event.entry_id, wksp1_foo_id);
        });
    }
    // The outbound sync had synced the data in the server, however it couldn't
    // acknowledge it, hence we still see the manifest as need sync with initial version !
    assert_folder(&wksp1_ops, wksp1_foo_id, true, 1, &expected_children).await;

    // 3) The folder is still opened, this time the sync cannot even start

    {
        let spy = wksp1_ops.event_bus.spy.start_expecting();
        let outcome = wksp1_ops.outbound_sync(wksp1_foo_id).await.unwrap();
        p_assert_matches!(outcome, OutboundSyncOutcome::EntryIsBusy);
        // The abort is so early there is no events !
        spy.assert_no_events();
    }

    // 4) Now close the folder, the next sync should proceed

    {
        let mut spy = wksp1_ops.event_bus.spy.start_expecting();

        concurrent_folder_unlock_requested.notify(1);
        concurrent_folder_unlocked.await;

        let outcome = wksp1_ops.outbound_sync(wksp1_foo_id).await.unwrap();
        // The first outbound had uploaded data on the server, but couldn't acknowledged
        // them, hence the need sync here !
        p_assert_matches!(outcome, OutboundSyncOutcome::InboundSyncNeeded);

        spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncStarted| {
            p_assert_eq!(event.realm_id, wksp1_id);
            p_assert_eq!(event.entry_id, wksp1_foo_id);
        });
        spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncAborted| {
            p_assert_eq!(event.realm_id, wksp1_id);
            p_assert_eq!(event.entry_id, wksp1_foo_id);
        });
    }

    // Check the manifest is still need sync
    assert_folder(&wksp1_ops, wksp1_foo_id, true, 1, &expected_children).await;

    // 5) Bonus check: ensure doing an inbound sync will correctly take into account
    //    the previously uploaded data

    {
        let mut spy = wksp1_ops.event_bus.spy.start_expecting();

        wksp1_ops.inbound_sync(wksp1_foo_id).await.unwrap();

        if concurrent_write {
            assert_folder(&wksp1_ops, wksp1_foo_id, true, 2, &expected_children).await;
        } else {
            // TODO: currently inbound sync's remote/local merge doesn't recompute
            // need sync when remote manifest author is self !
            assert_folder(&wksp1_ops, wksp1_foo_id, true, 2, &expected_children).await;
        }

        spy.assert_next(|event: &EventWorkspaceOpsInboundSyncDone| {
            p_assert_eq!(event.realm_id, wksp1_id);
            p_assert_eq!(event.entry_id, wksp1_foo_id);
        });
    }

    // 6) Bonus check: now the final outbound sync will succeed !

    {
        let mut spy = wksp1_ops.event_bus.spy.start_expecting();

        wksp1_ops.outbound_sync(wksp1_foo_id).await.unwrap();

        assert_folder(&wksp1_ops, wksp1_foo_id, false, 3, &expected_children).await;

        spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncStarted| {
            p_assert_eq!(event.realm_id, wksp1_id);
            p_assert_eq!(event.entry_id, wksp1_foo_id);
        });
        spy.assert_next(|event: &EventWorkspaceOpsOutboundSyncDone| {
            p_assert_eq!(event.realm_id, wksp1_id);
            p_assert_eq!(event.entry_id, wksp1_foo_id);
        });
    }

    wksp1_ops.stop().await.unwrap();
}

// TODO: test with placeholder folder manifest
// TODO: test sync with parent field changing and conflict
