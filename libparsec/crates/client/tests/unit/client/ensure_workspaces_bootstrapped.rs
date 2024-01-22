// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use libparsec_client_connection::test_register_sequence_of_send_hooks;
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::client_factory;
use crate::{ClientEnsureWorkspacesBootstrappedError, WorkspaceInfo};

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let wksp1_id = VlobID::default();
    let env = env.customize(|builder| {
        builder.certificates_storage_fetch_certificates("alice@dev1");
        builder
            .user_storage_local_update("alice@dev1")
            .add_or_update_placeholder(wksp1_id, "wksp1".parse().unwrap());
    });

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    // Mock requests to server

    let new_realm_certificates: Arc<Mutex<Vec<Bytes>>> = Arc::default();
    let new_realm_initial_keys_bundle: Arc<Mutex<Option<Bytes>>> = Arc::default();
    let new_realm_initial_keys_bundle_access: Arc<Mutex<Option<Bytes>>> = Arc::default();
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Realm creation
        {
            let alice = alice.clone();
            let new_realm_certificates = new_realm_certificates.clone();
            move |req: authenticated_cmds::latest::realm_create::Req| {
                RealmRoleCertificate::verify_and_load(
                    &req.realm_role_certificate,
                    &alice.verify_key(),
                    CertificateSignerRef::User(&alice.device_id),
                    Some(wksp1_id),
                    Some(alice.user_id()),
                )
                .unwrap();
                new_realm_certificates
                    .lock()
                    .unwrap()
                    .push(req.realm_role_certificate);
                authenticated_cmds::latest::realm_create::Rep::Ok
            }
        },
        // 2) Initial key rotation
        {
            let alice = alice.clone();
            let new_realm_certificates = new_realm_certificates.clone();
            let new_realm_initial_keys_bundle = new_realm_initial_keys_bundle.clone();
            let new_realm_initial_keys_bundle_access = new_realm_initial_keys_bundle_access.clone();
            move |req: authenticated_cmds::latest::realm_rotate_key::Req| {
                RealmKeyRotationCertificate::verify_and_load(
                    &req.realm_key_rotation_certificate,
                    &alice.verify_key(),
                    &alice.device_id,
                    Some(wksp1_id),
                )
                .unwrap();
                new_realm_certificates
                    .lock()
                    .unwrap()
                    .push(req.realm_key_rotation_certificate);
                *new_realm_initial_keys_bundle.lock().unwrap() = Some(req.keys_bundle);
                let access = req
                    .per_participant_keys_bundle_access
                    .get(&"alice".parse().unwrap())
                    .unwrap()
                    .to_owned();
                *new_realm_initial_keys_bundle_access.lock().unwrap() = Some(access);
                authenticated_cmds::latest::realm_rotate_key::Rep::Ok
            }
        },
        // 3) Fetch new realm certificates (required for initial rename)
        {
            let new_realm_certificates = new_realm_certificates.clone();
            move |_req: authenticated_cmds::latest::certificate_get::Req| {
                authenticated_cmds::latest::certificate_get::Rep::Ok {
                    common_certificates: vec![],
                    realm_certificates: HashMap::from_iter([(
                        wksp1_id,
                        new_realm_certificates.lock().unwrap().clone(),
                    )]),
                    sequester_certificates: vec![],
                    shamir_recovery_certificates: vec![],
                }
            }
        },
        // 4) Fetch keys bundle (required for initial rename)
        {
            let new_realm_initial_keys_bundle = new_realm_initial_keys_bundle.clone();
            let new_realm_initial_keys_bundle_access = new_realm_initial_keys_bundle_access.clone();
            move |_req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                    key_index: 1,
                    keys_bundle: new_realm_initial_keys_bundle
                        .lock()
                        .unwrap()
                        .as_ref()
                        .unwrap()
                        .clone(),
                    keys_bundle_access: new_realm_initial_keys_bundle_access
                        .lock()
                        .unwrap()
                        .as_ref()
                        .unwrap()
                        .clone(),
                }
            }
        },
        // 5) Actual rename
        {
            let alice = alice.clone();
            let new_realm_certificates = new_realm_certificates.clone();
            move |req: authenticated_cmds::latest::realm_rename::Req| {
                RealmNameCertificate::verify_and_load(
                    &req.realm_name_certificate,
                    &alice.verify_key(),
                    &alice.device_id,
                    Some(wksp1_id),
                )
                .unwrap();
                new_realm_certificates
                    .lock()
                    .unwrap()
                    .push(req.realm_name_certificate);
                authenticated_cmds::latest::realm_rename::Rep::Ok
            }
        }
    );

    // Actual operation

    client.ensure_workspaces_bootstrapped().await.unwrap();

    // The workspace is synchronized, but local user manifest still reflect the
    // previous state (as it gets updated by a separated operation)

    let workspaces = client.list_workspaces().await;
    p_assert_eq!(workspaces.len(), 1, "{:?}", workspaces);

    let wksp1_info = &workspaces[0];
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
        p_assert_eq!(*is_bootstrapped, false); // Weird but expected !
        p_assert_eq!(*is_started, false);
    }
}

#[parsec_test(testbed = "minimal")]
async fn legacy_shared_before_bootstrapped(env: &TestbedEnv) {
    let (env, wksp1_id) = env.customize_with_map(|builder| {
        builder.new_user("bob");
        builder.new_user("mallory");
        let wksp1_id = builder.new_realm("bob").map(|e| e.realm_id);
        // Must disable check consistency to build legacy-style workspace
        builder.with_check_consistency_disabled(|builder| {
            builder.share_realm(wksp1_id, "alice", Some(RealmRole::Owner));
            builder.share_realm(wksp1_id, "mallory", Some(RealmRole::Reader));
            builder.share_realm(wksp1_id, "bob", None);
        });
        builder.certificates_storage_fetch_certificates("alice@dev1");
        builder
            .user_storage_local_update("alice@dev1")
            .update_local_workspaces_with_fetched_certificates()
            .add_or_update_placeholder(wksp1_id, "wksp1".parse().unwrap());
        wksp1_id
    });

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    // Mock requests to server

    let new_realm_certificates: Arc<Mutex<Vec<Bytes>>> = Arc::default();
    let new_realm_initial_keys_bundle: Arc<Mutex<Option<Bytes>>> = Arc::default();
    let new_realm_initial_keys_bundle_accesses: Arc<Mutex<Option<HashMap<UserID, Bytes>>>> =
        Arc::default();
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // No step 1: realm already created

        // 2) Initial key rotation
        {
            let new_realm_certificates = new_realm_certificates.clone();
            let new_realm_initial_keys_bundle = new_realm_initial_keys_bundle.clone();
            let new_realm_initial_keys_bundle_accesses =
                new_realm_initial_keys_bundle_accesses.clone();
            move |req: authenticated_cmds::latest::realm_rotate_key::Req| {
                p_assert_eq!(
                    req.per_participant_keys_bundle_access.len(),
                    2,
                    "{:?}",
                    req.per_participant_keys_bundle_access
                );
                assert!(req
                    .per_participant_keys_bundle_access
                    .contains_key(&"alice".parse().unwrap()));
                assert!(req
                    .per_participant_keys_bundle_access
                    .contains_key(&"mallory".parse().unwrap()));
                new_realm_certificates
                    .lock()
                    .unwrap()
                    .push(req.realm_key_rotation_certificate);
                *new_realm_initial_keys_bundle.lock().unwrap() = Some(req.keys_bundle);
                *new_realm_initial_keys_bundle_accesses.lock().unwrap() =
                    Some(req.per_participant_keys_bundle_access);
                authenticated_cmds::latest::realm_rotate_key::Rep::Ok
            }
        },
        // 3) Fetch new realm certificates (required for initial rename)
        {
            let new_realm_certificates = new_realm_certificates.clone();
            move |_req: authenticated_cmds::latest::certificate_get::Req| {
                authenticated_cmds::latest::certificate_get::Rep::Ok {
                    common_certificates: vec![],
                    realm_certificates: HashMap::from_iter([(
                        wksp1_id,
                        new_realm_certificates.lock().unwrap().clone(),
                    )]),
                    sequester_certificates: vec![],
                    shamir_recovery_certificates: vec![],
                }
            }
        },
        // 4) Fetch keys bundle (required for initial rename)
        {
            let new_realm_initial_keys_bundle = new_realm_initial_keys_bundle.clone();
            let new_realm_initial_keys_bundle_accesses =
                new_realm_initial_keys_bundle_accesses.clone();
            move |_req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                let alice_access = new_realm_initial_keys_bundle_accesses
                    .lock()
                    .unwrap()
                    .as_ref()
                    .unwrap()
                    .get(&"alice".parse().unwrap())
                    .unwrap()
                    .clone();
                authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                    key_index: 1,
                    keys_bundle: new_realm_initial_keys_bundle
                        .lock()
                        .unwrap()
                        .as_ref()
                        .unwrap()
                        .clone(),
                    keys_bundle_access: alice_access,
                }
            }
        },
        // 5) Actual rename
        {
            let new_realm_certificates = new_realm_certificates.clone();
            move |req: authenticated_cmds::latest::realm_rename::Req| {
                new_realm_certificates
                    .lock()
                    .unwrap()
                    .push(req.realm_name_certificate);
                authenticated_cmds::latest::realm_rename::Rep::Ok
            }
        }
    );

    // Actual operation

    client.ensure_workspaces_bootstrapped().await.unwrap();

    // TODO: ensure the initial key rotation has been done with the legacy key
}

#[parsec_test(testbed = "minimal")]
async fn partially_bootstrapped(
    #[values(true, false)] role_considered_placeholder: bool,
    env: &TestbedEnv,
) {
    let (env, wksp1_id) = env.customize_with_map(|builder| {
        let wksp1_id = builder.new_realm("alice").map(|e| e.realm_id);
        builder.rotate_key_realm(wksp1_id);
        // Initial rename is missing for bootstrap to be done
        builder.certificates_storage_fetch_certificates("alice@dev1");
        if role_considered_placeholder {
            builder
                .user_storage_local_update("alice@dev1")
                // Ignore certificates for local workspaces, hence role stays a placeholder
                .add_or_update_placeholder(wksp1_id, "wksp1".parse().unwrap());
        } else {
            builder
                .user_storage_local_update("alice@dev1")
                .update_local_workspaces_with_fetched_certificates()
                .add_or_update_placeholder(wksp1_id, "wksp1".parse().unwrap());
        }
        wksp1_id
    });

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    // Mock requests to server

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // No step 1: realm already created

        // No step 2: Initial key rotation already done

        // No step 3: certificates already fetched

        // 4) Fetch keys bundle (required for initial rename)
        {
            let keys_bundle = env.get_last_realm_keys_bundle(wksp1_id);
            let keys_bundle_access =
                env.get_last_realm_keys_bundle_access_for(wksp1_id, alice.user_id());
            move |_req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                    key_index: 1,
                    keys_bundle,
                    keys_bundle_access,
                }
            }
        },
        // 5) Actual rename
        {
            let access_key = env
                .get_last_realm_keys_bundle_access_key(wksp1_id)
                .to_owned();
            move |req: authenticated_cmds::latest::realm_rename::Req| {
                let certif = RealmNameCertificate::verify_and_load(
                    &req.realm_name_certificate,
                    &alice.verify_key(),
                    &alice.device_id,
                    Some(wksp1_id),
                )
                .unwrap();
                let name = access_key.decrypt(&certif.encrypted_name).unwrap();
                p_assert_eq!(name, b"wksp1");
                authenticated_cmds::latest::realm_rename::Rep::Ok
            }
        }
    );

    // Actual operation

    client.ensure_workspaces_bootstrapped().await.unwrap();
}

#[parsec_test(testbed = "minimal")]
async fn legacy_no_longer_access(env: &TestbedEnv) {
    // Workspace is not bootstrapped (doesn't have initial key rotation nor rename),
    // but alice cannot do anything about it given she no longer has access to the realm.
    let env = env.customize(|builder| {
        builder.new_user("bob");
        let wksp1_id = builder.new_realm("alice").map(|e| e.realm_id);

        builder.with_check_consistency_disabled(|builder| {
            builder.share_realm(wksp1_id, "bob", Some(RealmRole::Owner));
        });
        builder.certificates_storage_fetch_certificates("alice@dev1");
        builder
            .user_storage_local_update("alice@dev1")
            .update_local_workspaces_with_fetched_certificates()
            .add_or_update_placeholder(wksp1_id, "wksp1".parse().unwrap());
        // Alice is not aware the workspace is no longer shared with her
        builder.with_check_consistency_disabled(|builder| {
            builder.share_realm(wksp1_id, "alice", None);
        });
    });

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    // Mock requests to server

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // No step 1: realm already created

        // 2) Initial key rotation
        |req: authenticated_cmds::latest::realm_rotate_key::Req| {
            p_assert_eq!(
                req.per_participant_keys_bundle_access.len(),
                2,
                "{:?}",
                req.per_participant_keys_bundle_access
            );
            assert!(req
                .per_participant_keys_bundle_access
                .contains_key(&"alice".parse().unwrap()));
            assert!(req
                .per_participant_keys_bundle_access
                .contains_key(&"bob".parse().unwrap()));
            authenticated_cmds::latest::realm_rotate_key::Rep::AuthorNotAllowed
        },
        // No more steps
    );

    // Actual operation, note the error is not propagated

    client.ensure_workspaces_bootstrapped().await.unwrap();
}

#[parsec_test(testbed = "minimal")]
async fn already_bootstrapped(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        let wksp1_id = builder.new_realm("alice").map(|e| e.realm_id);
        builder.rotate_key_realm(wksp1_id);
        builder.rename_realm(wksp1_id, "alice");
        builder.certificates_storage_fetch_certificates("alice@dev1");
        builder
            .user_storage_local_update("alice@dev1")
            .update_local_workspaces_with_fetched_certificates();
    });

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    // No server request should occur, the operation is a no-op

    client.ensure_workspaces_bootstrapped().await.unwrap();
}

#[parsec_test(testbed = "minimal")]
async fn stopped(#[values(true, false)] something_to_bootstrap: bool, env: &TestbedEnv) {
    if something_to_bootstrap {
        env.customize(|builder| {
            builder
                .user_storage_local_update("alice@dev1")
                .add_or_update_placeholder(VlobID::default(), "wksp1".parse().unwrap());
        });
    }

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    client.certificates_ops.stop().await.unwrap();

    let spy = client.event_bus.spy.start_expecting();

    let outcome = client.ensure_workspaces_bootstrapped().await;

    spy.assert_no_events();

    // If there is nothing to bootstrap, then no work is done !
    if something_to_bootstrap {
        let err = outcome.unwrap_err();
        p_assert_matches!(err, ClientEnsureWorkspacesBootstrappedError::Stopped);
    } else {
        assert!(outcome.is_ok());
    }
}
