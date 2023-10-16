// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_send_hook, test_register_sequence_of_send_hooks,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::workspace_ops_factory;

#[parsec_test(testbed = "minimal_client_ready")]
async fn sync_root_non_placeholder(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_key: &SecretKey = env.template.get_stuff("wksp1_key");
    let wksp1_ops = workspace_ops_factory(
        &env.discriminant_dir,
        &alice,
        wksp1_id,
        wksp1_key.to_owned(),
    )
    .await;

    // Sanity checks
    let workspace_manifest = wksp1_ops.data_storage.get_workspace_manifest();
    p_assert_eq!(workspace_manifest.need_sync, false);
    p_assert_eq!(workspace_manifest.speculative, false);
    p_assert_eq!(workspace_manifest.base.version, 1);
    p_assert_eq!(
        workspace_manifest.children,
        workspace_manifest.base.children
    );
    let base_children = workspace_manifest.children.to_owned();

    // Do a change requiring a sync

    wksp1_ops
        .create_folder(&"/new_folder".parse().unwrap())
        .await
        .unwrap();

    // Check the workspace manifest is need sync
    let workspace_manifest = wksp1_ops.data_storage.get_workspace_manifest();
    p_assert_eq!(workspace_manifest.need_sync, true);
    p_assert_eq!(workspace_manifest.speculative, false);
    p_assert_eq!(workspace_manifest.base.version, 1);
    p_assert_eq!(workspace_manifest.base.children, base_children);
    p_assert_eq!(workspace_manifest.children.len(), base_children.len() + 1,);
    assert!(workspace_manifest
        .children
        .contains_key(&"new_folder".parse().unwrap()));
    let new_folder_id = *workspace_manifest
        .children
        .get(&"new_folder".parse().unwrap())
        .unwrap();

    // Mock server command `vlob_update`
    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::vlob_update::Req| {
            p_assert_eq!(req.encryption_revision, 1);
            p_assert_eq!(req.vlob_id, wksp1_id);
            p_assert_eq!(req.version, 2);
            assert!(req.sequester_blob.is_none());
            authenticated_cmds::latest::vlob_update::Rep::Ok {}
        },
    );

    wksp1_ops.sync().await.unwrap();

    // Check the user manifest is not longer need sync
    let expected_children = {
        let mut children = base_children;
        children.insert("new_folder".parse().unwrap(), new_folder_id);
        children
    };

    let workspace_manifest = wksp1_ops.data_storage.get_workspace_manifest();
    p_assert_eq!(workspace_manifest.need_sync, false);
    p_assert_eq!(workspace_manifest.speculative, false);
    p_assert_eq!(workspace_manifest.base.version, 2);
    p_assert_eq!(workspace_manifest.children, expected_children);
    p_assert_eq!(workspace_manifest.base.children, expected_children);

    wksp1_ops.stop().await.unwrap();
}

#[parsec_test(testbed = "minimal")]
#[case::not_speculative(false)]
#[case::speculative(true)]
async fn sync_root_placeholder(#[case] is_speculative: bool, env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user_realm("alice");

        let new_realm_event = builder.new_realm("alice");
        let (wksp1_id, wksp1_key) = new_realm_event.map(|e| (e.realm_id, e.realm_key.clone()));
        new_realm_event.then_add_workspace_entry_to_user_manifest_vlob();
        builder.store_stuff("wksp1_id", &wksp1_id);
        builder.store_stuff("wksp1_key", &wksp1_key);

        builder.user_storage_fetch_user_vlob("alice@dev1");
        builder
            .workspace_data_storage_local_workspace_manifest_update("alice@dev1", wksp1_id)
            .customize(|e| {
                let manifest = std::sync::Arc::make_mut(&mut e.local_manifest);
                manifest.speculative = is_speculative;
            });
    });
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let wksp1_key: &SecretKey = env.template.get_stuff("wksp1_key");

    let alice = env.local_device("alice@dev1");
    let wksp1_ops = workspace_ops_factory(
        &env.discriminant_dir,
        &alice,
        wksp1_id,
        wksp1_key.to_owned(),
    )
    .await;

    // Check the workspace manifest is need sync
    let workspace_manifest = wksp1_ops.data_storage.get_workspace_manifest();
    p_assert_eq!(workspace_manifest.need_sync, true);
    p_assert_eq!(workspace_manifest.speculative, is_speculative);
    p_assert_eq!(workspace_manifest.base.version, 0);
    assert!(workspace_manifest.children.is_empty());
    assert!(workspace_manifest.base.children.is_empty());

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) `realm_create`
        {
            let alice = alice.clone();
            move |req: authenticated_cmds::latest::realm_create::Req| {
                RealmRoleCertificate::verify_and_load(
                    &req.role_certificate,
                    &alice.verify_key(),
                    CertificateSignerRef::User(&alice.device_id),
                    Some(wksp1_id),
                    Some(alice.user_id()),
                )
                .unwrap();
                authenticated_cmds::latest::realm_create::Rep::Ok {}
            }
        },
        // 2) `vlob_create`
        move |req: authenticated_cmds::latest::vlob_create::Req| {
            p_assert_eq!(req.realm_id, wksp1_id);
            p_assert_eq!(req.vlob_id, wksp1_id);
            assert!(req.sequester_blob.is_none());
            authenticated_cmds::latest::vlob_create::Rep::Ok {}
        },
    );

    wksp1_ops.sync().await.unwrap();

    // Check the workspace manifest is no longer need sync
    let workspace_manifest = wksp1_ops.data_storage.get_workspace_manifest();
    p_assert_eq!(workspace_manifest.need_sync, false);
    p_assert_eq!(workspace_manifest.speculative, false);
    p_assert_eq!(workspace_manifest.base.version, 1);
    assert!(workspace_manifest.children.is_empty());
    assert!(workspace_manifest.base.children.is_empty());

    wksp1_ops.stop().await.unwrap();
}
