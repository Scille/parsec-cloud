// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_send_hook, test_register_sequence_of_send_hooks,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::utils::user_ops_factory;

#[parsec_test(testbed = "minimal_client_ready")]
async fn sync_non_placeholder(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let user_ops = user_ops_factory(env, &alice).await;

    let wid = user_ops
        .list_workspaces()
        .first()
        .expect("Template contains one workspace")
        .0;

    // Do a change requiring a sync

    user_ops
        .workspace_rename(wid, "new_name".parse().unwrap())
        .await
        .unwrap();

    // Check the user manifest is need sync
    let user_manifest = user_ops.test_get_user_manifest();
    p_assert_eq!(user_manifest.need_sync, true);
    p_assert_eq!(user_manifest.speculative, false);
    p_assert_eq!(user_manifest.base.version, 1);
    p_assert_eq!(
        user_manifest.base.workspaces[0].name,
        "wksp1".parse().unwrap()
    );
    p_assert_eq!(
        user_manifest.workspaces[0].name,
        "new_name".parse().unwrap()
    );

    // Mock server command `vlob_update`
    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::vlob_update::Req| {
            p_assert_eq!(req.encryption_revision, 1);
            p_assert_eq!(req.vlob_id, alice.user_realm_id);
            p_assert_eq!(req.version, 2);
            assert!(req.sequester_blob.is_none());

            authenticated_cmds::latest::vlob_update::Rep::Ok {}
        },
    );

    user_ops.sync().await.unwrap();

    // Check the user manifest is not longer need sync
    let user_manifest = user_ops.test_get_user_manifest();
    p_assert_eq!(user_manifest.need_sync, false);
    p_assert_eq!(user_manifest.speculative, false);
    p_assert_eq!(user_manifest.base.version, 2);
    p_assert_eq!(
        user_manifest.workspaces[0].name,
        "new_name".parse().unwrap()
    );
    p_assert_eq!(
        user_manifest.base.workspaces[0].name,
        "new_name".parse().unwrap()
    );

    user_ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
#[case::not_speculative(false)]
#[case::speculative(true)]
async fn sync_placeholder(#[case] is_speculative: bool, env: &TestbedEnv) {
    env.customize(|builder| {
        builder
            .user_storage_local_update("alice@dev1")
            .customize(|e| {
                std::sync::Arc::make_mut(&mut e.local_manifest).speculative = is_speculative;
            });
    });
    let alice = env.local_device("alice@dev1");
    let user_ops = user_ops_factory(env, &alice).await;

    // Check the user manifest is need sync
    let user_manifest = user_ops.test_get_user_manifest();
    p_assert_eq!(user_manifest.need_sync, true);
    p_assert_eq!(user_manifest.speculative, is_speculative);
    p_assert_eq!(user_manifest.base.version, 0);
    assert!(user_manifest.workspaces.is_empty());
    assert!(user_manifest.base.workspaces.is_empty());

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
                    Some(alice.user_realm_id),
                    Some(alice.user_id()),
                )
                .unwrap();
                authenticated_cmds::latest::realm_create::Rep::Ok {}
            }
        },
        // 2) `vlob_create`
        move |req: authenticated_cmds::latest::vlob_create::Req| {
            p_assert_eq!(req.realm_id, alice.user_realm_id);
            p_assert_eq!(req.vlob_id, alice.user_realm_id);
            authenticated_cmds::latest::vlob_create::Rep::Ok {}
        },
    );

    user_ops.sync().await.unwrap();

    // Check the user manifest is no longer need sync
    let user_manifest = user_ops.test_get_user_manifest();
    p_assert_eq!(user_manifest.need_sync, false);
    p_assert_eq!(user_manifest.speculative, false);
    p_assert_eq!(user_manifest.base.version, 1);
    assert!(user_manifest.workspaces.is_empty());
    assert!(user_manifest.base.workspaces.is_empty());

    user_ops.stop().await;
}
