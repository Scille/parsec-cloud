// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use libparsec_client_connection::{test_register_send_hook, test_register_sequence_of_send_hooks};
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::client_factory;
use crate::ClientRevokeUserError;

#[parsec_test(testbed = "coolorg")]
async fn ok(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    // Mock requests to server
    let new_common_certificates: Arc<Mutex<Vec<Bytes>>> = Arc::default();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        // 1) Revoke
        {
            let new_common_certificates = new_common_certificates.clone();
            move |req: authenticated_cmds::latest::user_revoke::Req| {
                new_common_certificates
                    .lock()
                    .unwrap()
                    .push(req.revoked_user_certificate);
                authenticated_cmds::latest::user_revoke::Rep::Ok
            }
        },
        // 2) Fetch new certificates
        {
            let new_common_certificates = new_common_certificates.clone();
            move |_req: authenticated_cmds::latest::certificate_get::Req| {
                authenticated_cmds::latest::certificate_get::Rep::Ok {
                    common_certificates: new_common_certificates.lock().unwrap().clone(),
                    realm_certificates: HashMap::new(),
                    sequester_certificates: vec![],
                    shamir_recovery_certificates: vec![],
                }
            }
        },
    );

    // let mut spy = client.event_bus.spy.start_expecting();

    let bob_user_id: UserID = "bob".parse().unwrap();
    client.revoke_user(bob_user_id.clone()).await.unwrap();

    // TODO: check event !
    // spy.assert_next(|event: &EventUserRevoked| {
    // });

    let users = client.list_users(false, None, None).await.unwrap();
    let bob = users.iter().find(|user| user.id == bob_user_id).unwrap();
    p_assert_eq!(bob.revoked_by, Some("alice@dev1".parse().unwrap()));
}

#[parsec_test(testbed = "coolorg")]
async fn not_allowed(env: &TestbedEnv) {
    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, bob).await;

    test_register_send_hook(
        &env.discriminant_dir,
        move |_req: authenticated_cmds::latest::user_revoke::Req| {
            authenticated_cmds::latest::user_revoke::Rep::AuthorNotAllowed
        },
    );

    let spy = client.event_bus.spy.start_expecting();

    let err = client
        .revoke_user("alice".parse().unwrap())
        .await
        .unwrap_err();

    p_assert_matches!(err, ClientRevokeUserError::AuthorNotAllowed);
    spy.assert_no_events();
}

#[parsec_test(testbed = "minimal_client_ready")]
async fn not_found(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        move |_req: authenticated_cmds::latest::user_revoke::Req| {
            authenticated_cmds::latest::user_revoke::Rep::UserNotFound
        },
    );

    let spy = client.event_bus.spy.start_expecting();

    let err = client
        .revoke_user("dummy".parse().unwrap())
        .await
        .unwrap_err();

    p_assert_matches!(err, ClientRevokeUserError::UserNotFound);
    spy.assert_no_events();
}

#[parsec_test(testbed = "coolorg")]
async fn offline(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let spy = client.event_bus.spy.start_expecting();

    let err = client
        .revoke_user("bob".parse().unwrap())
        .await
        .unwrap_err();

    p_assert_matches!(err, ClientRevokeUserError::Offline);
    spy.assert_no_events();
}

#[parsec_test(testbed = "coolorg")]
async fn stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    client.certificates_ops.stop().await.unwrap();

    // Certificate ops can still send command while it is stopped, so the error
    // will occur after it (while we try to integrate the new certificates)
    test_register_send_hook(
        &env.discriminant_dir,
        move |_req: authenticated_cmds::latest::user_revoke::Req| {
            authenticated_cmds::latest::user_revoke::Rep::Ok
        },
    );

    let spy = client.event_bus.spy.start_expecting();

    let err = client
        .revoke_user("bob".parse().unwrap())
        .await
        .unwrap_err();

    p_assert_matches!(err, ClientRevokeUserError::Stopped);
    spy.assert_no_events();
}
