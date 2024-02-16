// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_sequence_of_send_hooks,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;

enum Modification {
    Create,
    Remove,
    Rename,
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn base(
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
            let child_manifest = wksp1_ops
                .data_storage
                .get_child_manifest(entry_id)
                .await
                .unwrap();
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
                .create_folder(&"/foo/new_folder".parse().unwrap())
                .await
                .unwrap();
            let mut expected_children = base_children.clone();
            expected_children.insert("new_folder".parse().unwrap(), new_folder_id);
            expected_children
        }
        Modification::Remove => {
            wksp1_ops
                .remove_entry(&"/foo/spam".parse().unwrap())
                .await
                .unwrap();
            let mut expected_children = base_children.clone();
            expected_children.remove(&"spam".parse().unwrap());
            expected_children
        }
        Modification::Rename => {
            wksp1_ops
                .rename_entry(
                    &"/foo/spam".parse().unwrap(),
                    "spam_renamed".parse().unwrap(),
                    false,
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

    wksp1_ops.outbound_sync(wksp1_foo_id).await.unwrap();

    // Check the user manifest is not longer need sync
    let foo_manifest = get_folder_manifest(wksp1_foo_id).await;
    p_assert_eq!(foo_manifest.need_sync, false);
    p_assert_eq!(foo_manifest.base.version, 2);
    p_assert_eq!(foo_manifest.children, expected_children);
    p_assert_eq!(foo_manifest.base.children, expected_children);

    wksp1_ops.stop().await.unwrap();
}
