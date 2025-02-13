// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use libparsec_client_connection::{
    test_register_sequence_of_send_hooks, test_send_hook_realm_get_keys_bundle,
};
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::client_factory;
use crate::{
    ClientRenameWorkspaceError, EventNewCertificates, EventWorkspacesSelfListChanged, WorkspaceInfo,
};

#[parsec_test(testbed = "coolorg")]
async fn ok(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let alice_user_id = alice.user_id;
    let client = client_factory(&env.discriminant_dir, alice).await;

    // Mock requests to server
    let new_realm_certificates: Arc<Mutex<Vec<Bytes>>> = Arc::default();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Fetch keys bundle (required for rename)
        test_send_hook_realm_get_keys_bundle!(env, alice_user_id, wksp1_id),
        // 2) Rename
        {
            let new_realm_certificates = new_realm_certificates.clone();
            move |req: authenticated_cmds::latest::realm_rename::Req| {
                new_realm_certificates
                    .lock()
                    .unwrap()
                    .push(req.realm_name_certificate);
                authenticated_cmds::latest::realm_rename::Rep::Ok
            }
        },
        // 3) Fetch new certificates
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
    );

    let mut spy = client.event_bus.spy.start_expecting();

    client
        .rename_workspace(wksp1_id, "wksp1'".parse().unwrap())
        .await
        .unwrap();

    // Rename also trigger a local workspace list refresh
    spy.assert_next(|_event: &EventNewCertificates| {});
    spy.assert_next(|_event: &EventWorkspacesSelfListChanged| {});

    let mut workspaces = client.list_workspaces().await;
    p_assert_eq!(workspaces.len(), 1, "{:?}", workspaces);

    let wksp1_info = workspaces.pop().unwrap();

    // wksp1
    {
        let WorkspaceInfo {
            id,
            current_name,
            current_self_role,
            is_started,
            is_bootstrapped,
        } = wksp1_info;
        p_assert_eq!(id, wksp1_id);
        p_assert_eq!(current_name, "wksp1'".parse().unwrap());
        p_assert_eq!(current_self_role, RealmRole::Owner);
        p_assert_eq!(is_bootstrapped, true);
        p_assert_eq!(is_started, false);
    }
}

#[parsec_test(testbed = "minimal")]
async fn realm_not_bootstrapped_missing_initial_rename(env: &TestbedEnv) {
    let wksp1_id = env
        .customize(|builder| {
            // Realm created but not bootstrapped
            let wksp1_id = builder.new_realm("alice").map(|e| e.realm_id);
            builder.rotate_key_realm(wksp1_id);
            // No initial rename, hence the realm is not fully bootstrapped
            builder.certificates_storage_fetch_certificates("alice@dev1");
            builder
                .user_storage_local_update("alice@dev1")
                .update_local_workspaces_with_fetched_certificates();
            wksp1_id
        })
        .await;

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    // Mock requests to server

    let new_realm_certificates: Arc<Mutex<Vec<Bytes>>> = Arc::default();
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Finish the bootstrap: fetch keys bundle (required for initial rename)
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, wksp1_id),
        // 2) Finish the bootstrap: initial realm rename
        {
            let new_realm_certificates = new_realm_certificates.clone();
            move |req: authenticated_cmds::latest::realm_rename::Req| {
                new_realm_certificates
                    .lock()
                    .unwrap()
                    .push(req.realm_name_certificate);
                authenticated_cmds::latest::realm_rename::Rep::Ok
            }
        },
        // 3) Fetch new certificates
        {
            let new_realm_certificates = new_realm_certificates.clone();
            move |_req: authenticated_cmds::latest::certificate_get::Req| {
                authenticated_cmds::latest::certificate_get::Rep::Ok {
                    common_certificates: vec![],
                    realm_certificates: HashMap::from_iter([(
                        wksp1_id,
                        new_realm_certificates.lock().unwrap().drain(..).collect(),
                    )]),
                    sequester_certificates: vec![],
                    shamir_recovery_certificates: vec![],
                }
            }
        },
    );

    let mut spy = client.event_bus.spy.start_expecting();

    client
        .rename_workspace(wksp1_id, "wksp1'".parse().unwrap())
        .await
        .unwrap();

    // Rename also trigger a local workspace list refresh
    spy.assert_next(|_event: &EventNewCertificates| {});
    spy.assert_next(|_event: &EventWorkspacesSelfListChanged| {});

    let mut workspaces = client.list_workspaces().await;
    p_assert_eq!(workspaces.len(), 1, "{:?}", workspaces);

    let wksp1_info = workspaces.pop().unwrap();

    // wksp1
    {
        let WorkspaceInfo {
            id,
            current_name,
            current_self_role,
            is_started,
            is_bootstrapped,
        } = wksp1_info;
        p_assert_eq!(id, wksp1_id);
        p_assert_eq!(current_name, "wksp1'".parse().unwrap());
        p_assert_eq!(current_self_role, RealmRole::Owner);
        p_assert_eq!(is_bootstrapped, true);
        p_assert_eq!(is_started, false);
    }
}

#[parsec_test(testbed = "minimal")]
async fn realm_not_bootstrapped_missing_initial_key_rotation(env: &TestbedEnv) {
    let wksp1_id = env
        .customize(|builder| {
            // Realm created but not bootstrapped
            let wksp1_id = builder.new_realm("alice").map(|e| e.realm_id);
            // No initial key rotation and rename, hence the realm is not fully bootstrapped
            builder.certificates_storage_fetch_certificates("alice@dev1");
            builder
                .user_storage_local_update("alice@dev1")
                .update_local_workspaces_with_fetched_certificates();
            wksp1_id
        })
        .await;

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    // Mock requests to server

    let new_realm_certificates: Arc<Mutex<Vec<Bytes>>> = Arc::default();
    let new_realm_initial_keys_bundle: Arc<Mutex<Option<Bytes>>> = Arc::default();
    let new_realm_initial_keys_bundle_access: Arc<Mutex<Option<Bytes>>> = Arc::default();
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Finish the bootstrap: initial key rotation
        {
            let new_realm_certificates = new_realm_certificates.clone();
            let new_realm_initial_keys_bundle = new_realm_initial_keys_bundle.clone();
            let new_realm_initial_keys_bundle_access = new_realm_initial_keys_bundle_access.clone();
            move |req: authenticated_cmds::latest::realm_rotate_key::Req| {
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
        // 2) Finish the bootstrap: fetch new realm certificates (required for initial rename)
        {
            let new_realm_certificates = new_realm_certificates.clone();
            move |_req: authenticated_cmds::latest::certificate_get::Req| {
                authenticated_cmds::latest::certificate_get::Rep::Ok {
                    common_certificates: vec![],
                    realm_certificates: HashMap::from_iter([(
                        wksp1_id,
                        // Note the drain: we clear `new_realm_certificates` given
                        // new certificates will be added and polled later on
                        new_realm_certificates.lock().unwrap().drain(..).collect(),
                    )]),
                    sequester_certificates: vec![],
                    shamir_recovery_certificates: vec![],
                }
            }
        },
        // 3) Finish the bootstrap: fetch keys bundle (required for initial rename)
        // Cannot use `test_send_hook_realm_get_keys_bundle` helper here since we are
        // referring to a key rotation that didn't occur in the testbed template.
        {
            let new_realm_initial_keys_bundle = new_realm_initial_keys_bundle.clone();
            let new_realm_initial_keys_bundle_access = new_realm_initial_keys_bundle_access.clone();
            move |_req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
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
        // 4) Finish the bootstrap: initial realm rename
        {
            let new_realm_certificates = new_realm_certificates.clone();
            move |req: authenticated_cmds::latest::realm_rename::Req| {
                new_realm_certificates
                    .lock()
                    .unwrap()
                    .push(req.realm_name_certificate);
                authenticated_cmds::latest::realm_rename::Rep::Ok
            }
        },
        // 5) Fetch new certificates
        {
            let new_realm_certificates = new_realm_certificates.clone();
            move |_req: authenticated_cmds::latest::certificate_get::Req| {
                authenticated_cmds::latest::certificate_get::Rep::Ok {
                    common_certificates: vec![],
                    realm_certificates: HashMap::from_iter([(
                        wksp1_id,
                        new_realm_certificates.lock().unwrap().drain(..).collect(),
                    )]),
                    sequester_certificates: vec![],
                    shamir_recovery_certificates: vec![],
                }
            }
        },
    );

    let mut spy = client.event_bus.spy.start_expecting();

    client
        .rename_workspace(wksp1_id, "wksp1'".parse().unwrap())
        .await
        .unwrap();

    // Some certificates were missing and had to be fetched first...
    spy.assert_next(|_event: &EventNewCertificates| {});
    // ..then the rename also trigger a local workspace list refresh
    spy.assert_next(|_event: &EventNewCertificates| {});
    spy.assert_next(|_event: &EventWorkspacesSelfListChanged| {});

    let mut workspaces = client.list_workspaces().await;
    p_assert_eq!(workspaces.len(), 1, "{:?}", workspaces);

    let wksp1_info = workspaces.pop().unwrap();

    // wksp1
    {
        let WorkspaceInfo {
            id,
            current_name,
            current_self_role,
            is_started,
            is_bootstrapped,
        } = wksp1_info;
        p_assert_eq!(id, wksp1_id);
        p_assert_eq!(current_name, "wksp1'".parse().unwrap());
        p_assert_eq!(current_self_role, RealmRole::Owner);
        p_assert_eq!(is_bootstrapped, true);
        p_assert_eq!(is_started, false);
    }
}

#[parsec_test(testbed = "minimal")]
async fn legacy_realm_shared_before_initial_key_rotation(env: &TestbedEnv) {
    let wksp1_id = env
        .customize(|builder| {
            // Realm created but not bootstrapped
            builder.new_user("bob");
            let wksp1_id = builder.new_realm("bob").map(|e| e.realm_id);
            // Disable check to allow simulate Parsec < v3 behavior were key rotation didn't exist
            builder.with_check_consistency_disabled(|builder| {
                // Given Alice is not OWNER, she cannot bootstrap the realm (this also means)
                builder.share_realm(wksp1_id, "alice", Some(RealmRole::Manager));
            });
            builder.certificates_storage_fetch_certificates("alice@dev1");
            builder
                .user_storage_local_update("alice@dev1")
                .update_local_workspaces_with_fetched_certificates();
            wksp1_id
        })
        .await;

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let spy = client.event_bus.spy.start_expecting();

    let err = client
        .rename_workspace(wksp1_id, "wksp1'".parse().unwrap())
        .await
        .unwrap_err();

    p_assert_matches!(err, ClientRenameWorkspaceError::AuthorNotAllowed);
    spy.assert_no_events();
}

#[parsec_test(testbed = "minimal")]
async fn not_allowed(env: &TestbedEnv) {
    let wksp1_id = env
        .customize(|builder| {
            builder.new_user("bob");
            let wksp1_id = builder.new_realm("bob").map(|e| e.realm_id);
            builder.rotate_key_realm(wksp1_id);
            builder.rename_realm(wksp1_id, "wksp1");
            // Given Alice is not OWNER, she is not allowed to do a rename
            builder.share_realm(wksp1_id, "alice", Some(RealmRole::Manager));
            builder.certificates_storage_fetch_certificates("alice@dev1");
            builder
                .user_storage_local_update("alice@dev1")
                .update_local_workspaces_with_fetched_certificates();
            wksp1_id
        })
        .await;

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let spy = client.event_bus.spy.start_expecting();

    let err = client
        .rename_workspace(wksp1_id, "wksp'".parse().unwrap())
        .await
        .unwrap_err();

    p_assert_matches!(err, ClientRenameWorkspaceError::AuthorNotAllowed);
    spy.assert_no_events();
}

// TODO: not allowed when then local workspace has an outdated OWNER role
//       (hence it's the server that respond a not allowed error)

#[parsec_test(testbed = "minimal")]
async fn not_found(env: &TestbedEnv) {
    let dummy_id = VlobID::default();

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let spy = client.event_bus.spy.start_expecting();

    let err = client
        .rename_workspace(dummy_id, "wksp".parse().unwrap())
        .await
        .unwrap_err();

    p_assert_matches!(err, ClientRenameWorkspaceError::WorkspaceNotFound);
    spy.assert_no_events();
}

#[parsec_test(testbed = "coolorg")]
async fn offline(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let spy = client.event_bus.spy.start_expecting();

    let err = client
        .rename_workspace(wksp1_id, "wksp1'".parse().unwrap())
        .await
        .unwrap_err();

    p_assert_matches!(err, ClientRenameWorkspaceError::Offline(_));
    spy.assert_no_events();
}

#[parsec_test(testbed = "coolorg")]
async fn stopped(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    client.certificates_ops.stop().await.unwrap();

    let spy = client.event_bus.spy.start_expecting();

    let err = client
        .rename_workspace(wksp1_id, "wksp1'".parse().unwrap())
        .await
        .unwrap_err();

    p_assert_matches!(err, ClientRenameWorkspaceError::Stopped);
    spy.assert_no_events();
}
