// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::{AccessToken, ShamirRecoveryShareData};

use crate::{ClientStartShamirRecoveryInvitationGreetError, ShamirRecoveryGreetInitialCtx};

use super::utils::client_factory;

fn get_alice_token(env: &TestbedEnv) -> AccessToken {
    let alice = env.local_device("alice@dev1");
    env.template
        .events
        .iter()
        .rev()
        .find_map(|e| match e {
            TestbedEvent::NewShamirRecoveryInvitation(event) if event.claimer == alice.user_id => {
                Some(event.token)
            }
            _ => None,
        })
        .unwrap()
}

#[parsec_test(testbed = "shamir", with_server)]
async fn setup_all_valid_while_never_deleted(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, bob).await;
    let token = get_alice_token(env);

    p_assert_matches!(
        client.start_shamir_recovery_invitation_greet(token).await.unwrap(),
        ShamirRecoveryGreetInitialCtx{share_data: ShamirRecoveryShareData { author, .. }, ..} if author == alice.device_id
    );
}

#[parsec_test(testbed = "shamir", with_server)]
async fn setup_all_valid_while_previously_deleted(
    #[values("prefetch", "no_prefetch")] kind: &str,
    env: &TestbedEnv,
) {
    let token = env
        .customize(|builder| {
            builder
                .new_shamir_recovery(
                    "bob",
                    3,
                    [
                        ("alice".parse().unwrap(), 2.try_into().unwrap()),
                        ("mike".parse().unwrap(), 1.try_into().unwrap()),
                    ],
                    "bob@dev2",
                )
                .map(|e| e.timestamp);
            match kind {
                "prefetch" => {
                    builder.certificates_storage_fetch_certificates("bob@dev1");
                }
                "no_prefetch" => {}
                _ => unreachable!(),
            }
            let new_invitation = builder.new_shamir_recovery_invitation("bob");
            new_invitation.get_event().token
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    p_assert_matches!(
        client.start_shamir_recovery_invitation_greet(token).await.unwrap(),
        ShamirRecoveryGreetInitialCtx{share_data: ShamirRecoveryShareData { author, .. }, ..} if author == bob.device_id
    );
}

#[parsec_test(testbed = "shamir", with_server)]
async fn setup_with_revoked_recipients(
    #[values("prefetch", "no_prefetch")] kind: &str,
    env: &TestbedEnv,
) {
    env.customize(|builder| {
        builder.revoke_user("bob"); // Bob has 2 shares
                                    // Threshold is 2, and Mallory & Mike remain with 1 share each
        match kind {
            "prefetch" => {
                builder.certificates_storage_fetch_certificates("mike@dev1");
            }
            "no_prefetch" => {}
            _ => unreachable!(),
        }
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let mike = env.local_device("mike@dev1");
    let client = client_factory(&env.discriminant_dir, mike).await;
    let token = get_alice_token(env);

    p_assert_matches!(
        client.start_shamir_recovery_invitation_greet(token).await.unwrap(),
        ShamirRecoveryGreetInitialCtx{share_data: ShamirRecoveryShareData { author, .. }, ..} if author == alice.device_id
    );
}

#[parsec_test(testbed = "shamir", with_server)]
async fn setup_but_unusable(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.revoke_user("bob"); // Bob has 2 shares
        builder.revoke_user("mallory"); // Mallory has 1 share
                                        // Threshold is 2, and only Mike with 1 share remains
        builder.certificates_storage_fetch_certificates("mike@dev1");
    })
    .await;

    let mike = env.local_device("mike@dev1");
    let client = client_factory(&env.discriminant_dir, mike).await;
    let token = get_alice_token(env);

    p_assert_matches!(
        client
            .start_shamir_recovery_invitation_greet(token)
            .await
            .unwrap_err(),
        ClientStartShamirRecoveryInvitationGreetError::ShamirRecoveryUnusable
    );
}

#[parsec_test(testbed = "shamir", with_server)]
async fn setup_deleted(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.delete_shamir_recovery("alice");
        builder.certificates_storage_fetch_certificates("bob@dev1");
    })
    .await;

    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, bob).await;
    let token = get_alice_token(env);

    p_assert_matches!(
        client
            .start_shamir_recovery_invitation_greet(token)
            .await
            .unwrap_err(),
        ClientStartShamirRecoveryInvitationGreetError::ShamirRecoveryDeleted
    );
}

#[parsec_test(testbed = "shamir", with_server)]
async fn invitation_not_found(env: &TestbedEnv) {
    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, bob).await;

    p_assert_matches!(
        client
            .start_shamir_recovery_invitation_greet(AccessToken::default())
            .await
            .unwrap_err(),
        ClientStartShamirRecoveryInvitationGreetError::InvitationNotFound
    );
}

#[parsec_test(testbed = "shamir", with_server)]
async fn stopped(env: &TestbedEnv) {
    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, bob).await;
    let token = get_alice_token(env);

    client.certificates_ops.stop().await.unwrap();

    p_assert_matches!(
        client
            .start_shamir_recovery_invitation_greet(token)
            .await
            .unwrap_err(),
        ClientStartShamirRecoveryInvitationGreetError::Stopped
    );
}
