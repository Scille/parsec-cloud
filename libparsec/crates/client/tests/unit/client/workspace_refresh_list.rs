// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::{
    test_register_sequence_of_send_hooks, test_send_hook_realm_get_keys_bundle,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::client_factory;
use crate::{ClientRefreshWorkspacesListError, EventWorkspacesSelfListChanged, WorkspaceInfo};

#[parsec_test(testbed = "coolorg")]
async fn ok_with_changes(
    #[values("new_sharing", "unsharing", "self_role_change", "renamed")] kind: &str,
    env: &TestbedEnv,
) {
    use std::{collections::HashMap, sync::Mutex};

    use libparsec_protocol::authenticated_cmds;

    let expected_workspaces = env
        .customize(|builder| {
            let mut expected_workspaces = vec![];

            // The testbed env is coolorg, hence Alice already has acces to workspace wksp1...

            let wksp1_id: VlobID = *builder.get_stuff("wksp1_id");

            expected_workspaces.push(WorkspaceInfo {
                id: wksp1_id,
                current_name: "wksp1".parse().unwrap(),
                current_self_role: RealmRole::Owner,
                is_started: false,
                is_bootstrapped: true,
            });

            // ...provide Alice's client with an additional local-only workspace.

            let wksp2_id = VlobID::default();
            builder
                .user_storage_local_update("alice@dev1")
                .customize(|e| {
                    let local_manifest = Arc::make_mut(&mut e.local_manifest);
                    local_manifest
                        .local_workspaces
                        .push(LocalUserManifestWorkspaceEntry {
                            id: wksp2_id,
                            name: "wksp2".parse().unwrap(),
                            name_origin: CertificateBasedInfoOrigin::Placeholder,
                            role: RealmRole::Owner,
                            role_origin: CertificateBasedInfoOrigin::Placeholder,
                        });
                });

            expected_workspaces.push(WorkspaceInfo {
                id: wksp2_id,
                current_name: "wksp2".parse().unwrap(),
                current_self_role: RealmRole::Owner,
                is_started: false,
                is_bootstrapped: false,
            });

            builder.certificates_storage_fetch_certificates("alice@dev1");

            match kind {
                "new_sharing" => {
                    let wksp3_id = builder.new_realm("bob").map(|e| e.realm_id);
                    builder.rotate_key_realm(wksp3_id);
                    builder.rename_realm(wksp3_id, "wksp3");
                    builder.share_realm(wksp3_id, "alice", RealmRole::Manager);

                    expected_workspaces.push(WorkspaceInfo {
                        id: wksp3_id,
                        current_name: "wksp3".parse().unwrap(),
                        current_self_role: RealmRole::Manager,
                        is_started: false,
                        is_bootstrapped: true,
                    });
                }
                "unsharing" => {
                    // Must promote Bob first so that he can change Alice's role
                    builder.share_realm(wksp1_id, "bob", RealmRole::Owner);
                    builder.share_realm(wksp1_id, "alice", None);

                    expected_workspaces.remove(0);
                }
                "self_role_change" => {
                    // Must promote Bob first so that he can change Alice's role
                    builder.share_realm(wksp1_id, "bob", RealmRole::Owner);
                    builder.share_realm(wksp1_id, "alice", RealmRole::Reader);

                    expected_workspaces[0].current_self_role = RealmRole::Reader;
                }
                "renamed" => {
                    builder.rename_realm(wksp1_id, "wksp1-renamed");

                    expected_workspaces[0].current_name = "wksp1-renamed".parse().unwrap();
                }
                unknown => panic!("Unknown kind: {}", unknown),
            }
            builder.certificates_storage_fetch_certificates("alice@dev1");

            expected_workspaces
        })
        .await;

    let keys_bundles_to_fetch = expected_workspaces
        .iter()
        .filter(|x| x.is_bootstrapped)
        .count();
    match keys_bundles_to_fetch {
        0 => (),
        1 => {
            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                test_send_hook_realm_get_keys_bundle!(
                    env,
                    "alice".parse().unwrap(),
                    expected_workspaces[0].id
                ),
            );
        }

        2 => {
            // Cannot use `test_send_hook_realm_get_keys_bundle` helper here since
            // the client is going to fetch multiple key bundles in an arbitrary order.

            let keys_bundles = Arc::new(Mutex::new(HashMap::<VlobID, (Bytes, Bytes)>::from_iter(
                expected_workspaces.iter().filter_map(|wksp| {
                    if wksp.is_bootstrapped {
                        let keys_bundle = env.get_last_realm_keys_bundle(wksp.id);
                        let keys_bundle_access = env.get_last_realm_keys_bundle_access_for(
                            wksp.id,
                            "alice".parse().unwrap(),
                        );

                        let alice = env.local_device("alice@dev1");

                        let cleartext_keys_bundle_access = alice
                            .private_key
                            .decrypt_from_self(&keys_bundle_access)
                            .unwrap();
                        let access =
                            RealmKeysBundleAccess::load(&cleartext_keys_bundle_access).unwrap();

                        let cleartext_keys_bundle =
                            access.keys_bundle_key.decrypt(&keys_bundle).unwrap();
                        RealmKeysBundle::unsecure_load(Bytes::from(cleartext_keys_bundle)).unwrap();

                        Some((wksp.id, (keys_bundle, keys_bundle_access)))
                    } else {
                        None
                    }
                }),
            )));

            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                {
                    let keys_bundles = keys_bundles.clone();
                    move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                        keys_bundles
                            .lock()
                            .unwrap()
                            .remove(&req.realm_id)
                            .map(|(keys_bundle, keys_bundle_access)| {
                                authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                                    keys_bundle,
                                    keys_bundle_access,
                                }
                            })
                            .unwrap_or_else(|| panic!("Unexpected realm_id: {:?}", req.realm_id))
                    }
                },
                {
                    let keys_bundles = keys_bundles.clone();
                    move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                        keys_bundles
                            .lock()
                            .unwrap()
                            .remove(&req.realm_id)
                            .map(|(keys_bundle, keys_bundle_access)| {
                                authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                                    keys_bundle,
                                    keys_bundle_access,
                                }
                            })
                            .unwrap_or_else(|| panic!("Unexpected realm_id: {:?}", req.realm_id))
                    }
                },
            );
        }

        _ => unreachable!(),
    }

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let mut spy = client.event_bus.spy.start_expecting();

    client.refresh_workspaces_list().await.unwrap();

    spy.assert_next(|_: &EventWorkspacesSelfListChanged| {});

    let workspaces = client.list_workspaces().await;
    p_assert_eq!(workspaces, expected_workspaces);
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn ok_noop(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let expected_workspaces = client.list_workspaces().await;

    let spy = client.event_bus.spy.start_expecting();

    let err = client.refresh_workspaces_list().await.unwrap_err();

    p_assert_matches!(err, ClientRefreshWorkspacesListError::Offline(_));
    spy.assert_no_events();

    let workspaces = client.list_workspaces().await;
    p_assert_eq!(workspaces, expected_workspaces)
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn offline(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let spy = client.event_bus.spy.start_expecting();

    let err = client.refresh_workspaces_list().await.unwrap_err();

    p_assert_matches!(err, ClientRefreshWorkspacesListError::Offline(_));
    spy.assert_no_events();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    client.certificates_ops.stop().await.unwrap();

    let spy = client.event_bus.spy.start_expecting();

    let err = client.refresh_workspaces_list().await.unwrap_err();

    p_assert_matches!(err, ClientRefreshWorkspacesListError::Stopped);
    spy.assert_no_events();
}
