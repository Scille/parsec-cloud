// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::client_factory;
use crate::WorkspaceInfo;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let (env, (wksp1_id, wksp2_id, wksp3_id, _, wksp5_id, wksp6_id)) =
        env.customize_with_map(|builder| {
            builder.new_user("bob");

            // wksp1 is a bootstrapped workspace
            let wksp1_id = builder.new_realm("alice").map(|e| e.realm_id);
            builder.rotate_key_realm(wksp1_id);
            builder.rename_realm(wksp1_id, "wksp1");

            // wksp2 is a bootstrapped workspace shared with us
            let wksp2_id = builder.new_realm("bob").map(|e| e.realm_id);
            builder.rotate_key_realm(wksp2_id);
            builder.rename_realm(wksp2_id, "wksp2");
            builder.share_realm(wksp2_id, "alice", Some(RealmRole::Contributor));

            // wksp3 is only locally created (hence no need to call the builder for it !)
            let wksp3_id = VlobID::default();
            let wksp3_name = "wksp3".parse().unwrap();

            // wksp4 hasn't been removed from local cache yet, but we are no longer part of it.
            // Hence it should not be listed.
            let wksp4_id = builder.new_realm("bob").map(|e| e.realm_id);
            builder.rotate_key_realm(wksp4_id);
            builder.rename_realm(wksp4_id, "wksp4");
            builder.share_realm(wksp4_id, "alice", Some(RealmRole::Contributor));
            builder.share_realm(wksp4_id, "alice", None);

            // wksp5 exists on server side but it is not fully bootstrapped (missing name certificate).
            let wksp5_id = builder.new_realm("alice").map(|e| e.realm_id);
            builder.rotate_key_realm(wksp5_id);
            let wksp5_name = "wksp5".parse().unwrap();

            // wksp6 is a not fully bootstrapped workspace shared with us.
            let wksp6_id = builder.new_realm("bob").map(|e| e.realm_id);
            builder.rotate_key_realm(wksp6_id);
            builder.share_realm(wksp6_id, "alice", Some(RealmRole::Contributor));
            builder.share_realm(wksp6_id, "alice", Some(RealmRole::Reader));
            let wksp6_name = "wksp6".parse().unwrap();

            builder.certificates_storage_fetch_certificates("alice@dev1");
            builder
                .user_storage_local_update("alice@dev1")
                .update_local_workspaces_with_fetched_certificates()
                .customize(|e| {
                    let local_manifest = std::sync::Arc::make_mut(&mut e.local_manifest);

                    // wksp3 is only locally created
                    local_manifest
                        .local_workspaces
                        .push(LocalUserManifestWorkspaceEntry {
                            id: wksp3_id,
                            name: wksp3_name,
                            name_origin: CertificateBasedInfoOrigin::Placeholder,
                            role: RealmRole::Owner,
                            role_origin: CertificateBasedInfoOrigin::Placeholder,
                        });

                    // Correct placeholder name for wksp5
                    let wksp5_entry = local_manifest
                        .local_workspaces
                        .iter_mut()
                        .find(|e| e.id == wksp5_id)
                        .unwrap();
                    wksp5_entry.name = wksp5_name;
                    wksp5_entry.name_origin = CertificateBasedInfoOrigin::Placeholder;

                    // Correct placeholder name for wksp6
                    let wksp6_entry = local_manifest
                        .local_workspaces
                        .iter_mut()
                        .find(|e| e.id == wksp6_id)
                        .unwrap();
                    wksp6_entry.name = wksp6_name;
                    wksp6_entry.name_origin = CertificateBasedInfoOrigin::Placeholder;
                });

            (wksp1_id, wksp2_id, wksp3_id, wksp4_id, wksp5_id, wksp6_id)
        });

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let workspaces = client.list_workspaces().await;
    p_assert_eq!(workspaces.len(), 5, "{:?}", workspaces);

    let wksp1_info = &workspaces[0];
    let wksp2_info = &workspaces[1];
    let wksp3_info = &workspaces[2];
    let wksp5_info = &workspaces[3];
    let wksp6_info = &workspaces[4];

    // wksp1
    {
        let WorkspaceInfo {
            id,
            name,
            self_current_role,
            is_started,
            is_bootstrapped,
        } = wksp1_info;
        p_assert_eq!(*id, wksp1_id);
        p_assert_eq!(*name, "wksp1".parse().unwrap());
        p_assert_eq!(*self_current_role, RealmRole::Owner);
        p_assert_eq!(*is_bootstrapped, true);
        p_assert_eq!(*is_started, false);
    }

    // wksp2
    {
        let WorkspaceInfo {
            id,
            name,
            self_current_role,
            is_started,
            is_bootstrapped,
        } = wksp2_info;
        p_assert_eq!(*id, wksp2_id, "{:?}", wksp2_info);
        p_assert_eq!(*name, "wksp2".parse().unwrap());
        p_assert_eq!(*self_current_role, RealmRole::Contributor);
        p_assert_eq!(*is_bootstrapped, true);
        p_assert_eq!(*is_started, false);
    }

    // wksp3
    {
        let WorkspaceInfo {
            id,
            name,
            self_current_role,
            is_started,
            is_bootstrapped,
        } = wksp3_info;
        p_assert_eq!(*id, wksp3_id);
        p_assert_eq!(*name, "wksp3".parse().unwrap());
        p_assert_eq!(*self_current_role, RealmRole::Owner);
        p_assert_eq!(*is_bootstrapped, false);
        p_assert_eq!(*is_started, false);
    }

    // wksp4 is never listed as we are no longer part of it

    // wksp5
    {
        let WorkspaceInfo {
            id,
            name,
            self_current_role,
            is_started,
            is_bootstrapped,
        } = wksp5_info;
        p_assert_eq!(*id, wksp5_id);
        p_assert_eq!(*name, "wksp5".parse().unwrap());
        p_assert_eq!(*self_current_role, RealmRole::Owner);
        p_assert_eq!(*is_bootstrapped, false);
        p_assert_eq!(*is_started, false);
    }

    // wksp6
    {
        let WorkspaceInfo {
            id,
            name,
            self_current_role,
            is_started,
            is_bootstrapped,
        } = wksp6_info;
        p_assert_eq!(*id, wksp6_id);
        p_assert_eq!(*name, "wksp6".parse().unwrap());
        p_assert_eq!(*self_current_role, RealmRole::Reader);
        p_assert_eq!(*is_bootstrapped, false);
        p_assert_eq!(*is_started, false);
    }
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn started_workspace(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    macro_rules! assert_is_started {
        ($expected_is_started:expr) => {
            async {
                let workspaces = client.list_workspaces().await;
                p_assert_eq!(workspaces.len(), 1, "{:?}", workspaces);
                p_assert_eq!(workspaces[0].is_started, $expected_is_started);
            }
        };
    }

    assert_is_started!(false).await;

    client.start_workspace(wksp1_id).await.unwrap();

    assert_is_started!(true).await;

    client.stop_workspace(wksp1_id).await;

    assert_is_started!(false).await;
}
