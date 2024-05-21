// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;
use crate::workspace::{MoveEntryMode, OutboundSyncOutcome};

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
    let wksp1_ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    let get_folder_manifest = |entry_id| {
        let wksp1_ops = &wksp1_ops;
        async move {
            let child_manifest = wksp1_ops.store.get_manifest(entry_id).await.unwrap();
            match child_manifest {
                ArcLocalChildManifest::File(m) => panic!("Expected folder, got {:?}", m),
                ArcLocalChildManifest::Folder(m) => m,
            }
        }
    };

    // Sanity checks
    let foo_manifest = get_folder_manifest(wksp1_foo_id).await;
    p_assert_eq!(foo_manifest.need_sync, false);
    p_assert_eq!(foo_manifest.base.version, 1);
    p_assert_eq!(foo_manifest.children, foo_manifest.base.children);
    let base_children = foo_manifest.children.to_owned();

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
    let foo_manifest = get_folder_manifest(wksp1_foo_id).await;
    p_assert_eq!(foo_manifest.need_sync, true);
    p_assert_eq!(foo_manifest.base.version, 1);
    p_assert_eq!(foo_manifest.base.children, base_children);
    p_assert_eq!(foo_manifest.children, expected_children);

    // Mock server command `vlob_update`
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
        // 2) `vlob_update` succeed on first try !
        move |req: authenticated_cmds::latest::vlob_update::Req| {
            p_assert_eq!(req.key_index, 1);
            p_assert_eq!(req.vlob_id, wksp1_foo_id);
            p_assert_eq!(req.version, 2);
            assert!(req.sequester_blob.is_none());
            authenticated_cmds::latest::vlob_update::Rep::Ok {}
        },
    );

    let outcome = wksp1_ops.outbound_sync(wksp1_foo_id).await.unwrap();
    p_assert_matches!(outcome, OutboundSyncOutcome::Done);

    // Check the user manifest is not longer need sync
    let foo_manifest = get_folder_manifest(wksp1_foo_id).await;
    p_assert_eq!(foo_manifest.need_sync, false);
    p_assert_eq!(foo_manifest.base.version, 2);
    p_assert_eq!(foo_manifest.children, expected_children);
    p_assert_eq!(foo_manifest.base.children, expected_children);

    // Subsequent sync is an idempotent noop

    let outcome = wksp1_ops.outbound_sync(wksp1_foo_id).await.unwrap();
    p_assert_matches!(outcome, OutboundSyncOutcome::Done);

    wksp1_ops.stop().await.unwrap();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn inbound_sync_needed(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_foo_id: VlobID = *env.template.get_stuff("wksp1_foo_id");

    let env = env.customize(|builder| {
        // New version of the file that we our client doesn't know about
        builder.create_or_update_folder_manifest_vlob("alice@dev1", wksp1_id, wksp1_foo_id, None);
    });

    let alice = env.local_device("alice@dev1");
    let wksp1_ops = workspace_ops_factory(&env.discriminant_dir, &alice, wksp1_id).await;

    let get_folder_manifest = |entry_id| {
        let wksp1_ops = &wksp1_ops;
        async move {
            let child_manifest = wksp1_ops.store.get_manifest(entry_id).await.unwrap();
            match child_manifest {
                ArcLocalChildManifest::File(m) => panic!("Expected folder, got {:?}", m),
                ArcLocalChildManifest::Folder(m) => m,
            }
        }
    };

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
        // 2) `vlob_update`
        move |req: authenticated_cmds::latest::vlob_update::Req| {
            p_assert_eq!(req.key_index, 1);
            p_assert_eq!(req.vlob_id, wksp1_foo_id);
            p_assert_eq!(req.version, 2);
            assert!(req.sequester_blob.is_none());
            authenticated_cmds::latest::vlob_update::Rep::BadVlobVersion
        },
    );

    let before_sync_manifest = get_folder_manifest(wksp1_foo_id).await;
    p_assert_eq!(before_sync_manifest.need_sync, true); // Sanity check

    let outcome = wksp1_ops.outbound_sync(wksp1_foo_id).await.unwrap();
    p_assert_matches!(outcome, OutboundSyncOutcome::InboundSyncNeeded);

    // Check the user manifest still need sync
    let after_failed_sync_manifest = get_folder_manifest(wksp1_foo_id).await;
    p_assert_eq!(after_failed_sync_manifest, before_sync_manifest);

    wksp1_ops.stop().await.unwrap();
}

// TODO: test with placeholder folder manifest
// TODO: test `OutboundSyncOutcome::EntryIsBusy`
// TODO: test sync with parent field changing and conflict
