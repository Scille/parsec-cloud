// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::test_register_send_hook;
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{Client, ClientUserUpdateProfileError};

use super::utils::client_factory;

async fn update_profile_and_check(profile: UserProfile, user_id: UserID, client: &Arc<Client>) {
    client.update_user_profile(user_id, profile).await.unwrap();

    let users = client.list_users(false, None, None).await.unwrap();
    let user = users.iter().find(|user| user.id == user_id).unwrap();
    p_assert_eq!(user.current_profile, profile);
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn ok(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let bob_user_id: UserID = "bob".parse().unwrap();

    // 0. check bob is standard
    let users = client.list_users(false, None, None).await.unwrap();
    let bob = users.iter().find(|user| user.id == bob_user_id).unwrap();
    p_assert_eq!(bob.current_profile, UserProfile::Standard);

    // 0.5 standard to standard
    update_profile_and_check(UserProfile::Standard, bob_user_id, &client).await;

    // 1. standard to admin
    update_profile_and_check(UserProfile::Admin, bob_user_id, &client).await;

    // 1.5 admin to admin
    update_profile_and_check(UserProfile::Admin, bob_user_id, &client).await;

    // 2. admin to outsider
    update_profile_and_check(UserProfile::Outsider, bob_user_id, &client).await;

    // 2.5 outsider to outsider
    update_profile_and_check(UserProfile::Outsider, bob_user_id, &client).await;

    // 3. outsider to standard
    update_profile_and_check(UserProfile::Standard, bob_user_id, &client).await;

    // 4. standard to outsider
    update_profile_and_check(UserProfile::Outsider, bob_user_id, &client).await;

    // 5. outsider to admin
    update_profile_and_check(UserProfile::Admin, bob_user_id, &client).await;

    // 6. admin to standard
    update_profile_and_check(UserProfile::Standard, bob_user_id, &client).await;
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn user_revoked(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;
    let bob_user_id: UserID = "bob".parse().unwrap();

    client.revoke_user(bob_user_id).await.unwrap();
    let err = client
        .update_user_profile("bob".parse().unwrap(), UserProfile::Outsider)
        .await
        .unwrap_err();

    p_assert_matches!(err, ClientUserUpdateProfileError::UserRevoked);
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn self_change_admin(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let err = client
        .update_user_profile("alice".parse().unwrap(), UserProfile::Admin)
        .await
        .unwrap_err();

    p_assert_matches!(err, ClientUserUpdateProfileError::UserIsSelf);
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn not_allowed_standard(env: &TestbedEnv) {
    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, bob).await;

    let err = client
        .update_user_profile("alice".parse().unwrap(), UserProfile::Outsider)
        .await
        .unwrap_err();

    p_assert_matches!(err, ClientUserUpdateProfileError::AuthorNotAllowed);
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn not_allowed_outsider(env: &TestbedEnv) {
    let mallory = env.local_device("mallory@dev1");
    let client = client_factory(&env.discriminant_dir, mallory).await;

    let err = client
        .update_user_profile("bob".parse().unwrap(), UserProfile::Outsider)
        .await
        .unwrap_err();

    p_assert_matches!(err, ClientUserUpdateProfileError::AuthorNotAllowed);
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn self_downgrade_outsider(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let err = client
        .update_user_profile("alice".parse().unwrap(), UserProfile::Outsider)
        .await
        .unwrap_err();

    p_assert_matches!(err, ClientUserUpdateProfileError::UserIsSelf);
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn self_downgrade_standard(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let err = client
        .update_user_profile("alice".parse().unwrap(), UserProfile::Standard)
        .await
        .unwrap_err();

    p_assert_matches!(err, ClientUserUpdateProfileError::UserIsSelf);
}

#[parsec_test(testbed = "minimal_client_ready", with_server)]
async fn not_found(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let err = client
        .update_user_profile(UserID::default(), UserProfile::Standard)
        .await
        .unwrap_err();

    p_assert_matches!(err, ClientUserUpdateProfileError::UserNotFound);
}

#[parsec_test(testbed = "coolorg")]
async fn offline(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let spy = client.event_bus.spy.start_expecting();

    let err = client
        .update_user_profile("bob".parse().unwrap(), UserProfile::Admin)
        .await
        .unwrap_err();

    p_assert_matches!(err, ClientUserUpdateProfileError::Offline);
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
        move |_req: authenticated_cmds::latest::user_update::Req| {
            authenticated_cmds::latest::user_update::Rep::Ok
        },
    );

    let spy = client.event_bus.spy.start_expecting();

    let err = client
        .update_user_profile("bob".parse().unwrap(), UserProfile::Admin)
        .await
        .unwrap_err();

    p_assert_matches!(err, ClientUserUpdateProfileError::Stopped);
    spy.assert_no_events();
}

#[parsec_test(testbed = "minimal")]
#[case::unknown_status(
    authenticated_cmds::latest::user_update::Rep::InvalidCertificate {  },
    |err| p_assert_matches!(err, ClientUserUpdateProfileError::Internal(_)),
)]
#[case::unknown_status(
    authenticated_cmds::latest::user_update::Rep::TimestampOutOfBallpark {
        ballpark_client_early_offset: 300.,
        ballpark_client_late_offset: 320.,
        client_timestamp: "2000-01-02T00:00:00Z".parse().unwrap(),
        server_timestamp: "2000-01-02T00:00:00Z".parse().unwrap(),
    },
    |err| p_assert_matches!(
        err,
        ClientUserUpdateProfileError::TimestampOutOfBallpark {
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            client_timestamp,
            server_timestamp,
        }
        if ballpark_client_early_offset == 300.
            && ballpark_client_late_offset == 320.
            && client_timestamp == "2000-01-02T00:00:00Z".parse().unwrap()
            && server_timestamp == "2000-01-02T00:00:00Z".parse().unwrap(),
    ),
)]
async fn server_error(
    #[case] rep: authenticated_cmds::latest::user_update::Rep,
    #[case] assert: impl FnOnce(ClientUserUpdateProfileError),
    env: &TestbedEnv,
) {
    use libparsec_client_connection::test_register_send_hook;

    let alice = env.local_device("alice@dev1");

    let client = client_factory(&env.discriminant_dir, alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        move |_: authenticated_cmds::latest::user_update::Req| rep,
    );

    let err = client
        .update_user_profile("bob".parse().unwrap(), UserProfile::Admin)
        .await
        .unwrap_err();

    assert(err);
}
