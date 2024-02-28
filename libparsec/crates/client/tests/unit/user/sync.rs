// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    protocol::authenticated_cmds, test_register_send_hook, test_register_sequence_of_send_hooks,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::user_ops_factory;

#[parsec_test(testbed = "minimal_client_ready")]
async fn sync_non_placeholder(
    #[values(true, false)] with_local_workspace_entry: bool,
    env: &TestbedEnv,
) {
    let alice = env.local_device("alice@dev1");
    let user_ops = user_ops_factory(env, &alice).await;

    let user_manifest = user_ops.get_user_manifest();
    p_assert_eq!(user_manifest.need_sync, false);

    // Tweak local user manifest's `local_workspaces` field
    // (this field should not be impacted in any way by the synchronization)
    let expected_local_workspaces = {
        let (updater, mut user_manifest) = user_ops.store.for_update().await;
        // `Arc::make_mut` clones user manifest before we modify it
        let user_manifest_mut = std::sync::Arc::make_mut(&mut user_manifest);

        if with_local_workspace_entry {
            user_manifest_mut.local_workspaces = vec![LocalUserManifestWorkspaceEntry {
                id: VlobID::default(),
                name: "wksp1".parse().unwrap(),
                name_origin: CertificateBasedInfoOrigin::Placeholder,
                role: RealmRole::Owner,
                role_origin: CertificateBasedInfoOrigin::Placeholder,
            }];
        } else {
            user_manifest_mut.local_workspaces.clear();
        };

        let expected_workspaces = user_manifest.local_workspaces.clone();
        updater.set_user_manifest(user_manifest).await.unwrap();
        expected_workspaces
    };

    // Sanity check
    let user_manifest = user_ops.get_user_manifest();
    p_assert_eq!(user_manifest.need_sync, false);
    p_assert_eq!(user_manifest.speculative, false);
    p_assert_eq!(user_manifest.base.version, 1);
    p_assert_eq!(user_manifest.local_workspaces, expected_local_workspaces);

    // Do a change requiring a sync
    user_ops.test_bump_updated_and_need_sync().await.unwrap();

    // Check the user manifest is need sync
    let user_manifest = user_ops.get_user_manifest();
    p_assert_eq!(user_manifest.need_sync, true);
    p_assert_eq!(user_manifest.speculative, false);
    p_assert_eq!(user_manifest.base.version, 1);
    p_assert_eq!(user_manifest.local_workspaces, expected_local_workspaces);

    // Mock server command `vlob_update`
    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::vlob_update::Req| {
            p_assert_eq!(req.vlob_id, alice.user_realm_id);
            p_assert_eq!(req.version, 2);
            assert!(req.sequester_blob.is_none());

            UserManifest::decrypt_verify_and_load(
                &req.blob,
                &alice.user_realm_key,
                &alice.verify_key(),
                &alice.device_id,
                req.timestamp,
                Some(alice.user_realm_id),
                Some(2),
            )
            .unwrap();

            authenticated_cmds::latest::vlob_update::Rep::Ok {}
        },
    );

    user_ops.sync().await.unwrap();

    // Check the user manifest is not longer need sync
    let user_manifest = user_ops.get_user_manifest();
    p_assert_eq!(user_manifest.need_sync, false);
    p_assert_eq!(user_manifest.speculative, false);
    p_assert_eq!(user_manifest.base.version, 2);
    p_assert_eq!(user_manifest.local_workspaces, expected_local_workspaces);
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
    let user_manifest = user_ops.get_user_manifest();
    p_assert_eq!(user_manifest.need_sync, true);
    p_assert_eq!(user_manifest.speculative, is_speculative);
    p_assert_eq!(user_manifest.base.version, 0);
    assert!(user_manifest.local_workspaces.is_empty());

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) `realm_create`
        {
            let alice = alice.clone();
            move |req: authenticated_cmds::latest::realm_create::Req| {
                RealmRoleCertificate::verify_and_load(
                    &req.realm_role_certificate,
                    &alice.verify_key(),
                    &alice.device_id,
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

            UserManifest::decrypt_verify_and_load(
                &req.blob,
                &alice.user_realm_key,
                &alice.verify_key(),
                &alice.device_id,
                req.timestamp,
                Some(alice.user_realm_id),
                Some(1),
            )
            .unwrap();

            authenticated_cmds::latest::vlob_create::Rep::Ok {}
        },
    );

    user_ops.sync().await.unwrap();

    // Check the user manifest is no longer need sync
    let user_manifest = user_ops.get_user_manifest();
    p_assert_eq!(user_manifest.need_sync, false);
    p_assert_eq!(user_manifest.speculative, false);
    p_assert_eq!(user_manifest.base.version, 1);
    assert!(user_manifest.local_workspaces.is_empty());
}

// TODO: test upload with sequester services
// TODO: test upload with revoked sequester services (but first we need to
// actually implement `SequesterRevokedServiceCertificate` !)
