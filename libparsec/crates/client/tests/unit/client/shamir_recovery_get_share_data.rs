// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::ShamirRecoveryShareData;

use crate::ClientGetShamirRecoveryShareDataError;

use super::utils::client_factory;

#[parsec_test(testbed = "coolorg")]
async fn never_setup(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, bob).await;

    p_assert_matches!(
        client.get_shamir_recovery_share_data(alice.user_id).await.unwrap_err(),
        ClientGetShamirRecoveryShareDataError::ShamirRecoveryNotFound { user_id } if user_id == alice.user_id
    );
}

#[parsec_test(testbed = "shamir")]
async fn setup_all_valid_while_never_deleted(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, bob).await;

    p_assert_matches!(
        client.get_shamir_recovery_share_data(alice.user_id).await.unwrap(),
        ShamirRecoveryShareData { author, .. } if author == alice.device_id
    );
}

#[parsec_test(testbed = "shamir")]
async fn setup_all_valid_while_previously_deleted(env: &TestbedEnv) {
    env.customize(|builder| {
        let setup_timestamp = builder
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
        builder.certificates_storage_fetch_certificates("bob@dev1");
        setup_timestamp
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    p_assert_matches!(
        client.get_shamir_recovery_share_data(bob.user_id).await.unwrap(),
        ShamirRecoveryShareData { author, .. } if author == bob.device_id
    );
}

#[parsec_test(testbed = "shamir")]
async fn setup_with_revoked_recipients(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.revoke_user("bob"); // Bob has 2 shares
                                    // Threshold is 2, and Mallory & Mike remain with 1 share each
        builder.certificates_storage_fetch_certificates("mike@dev1");
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let mike = env.local_device("mike@dev1");
    let client = client_factory(&env.discriminant_dir, mike).await;

    p_assert_matches!(
        client.get_shamir_recovery_share_data(alice.user_id).await.unwrap(),
        ShamirRecoveryShareData { author, .. } if author == alice.device_id
    );
}

#[parsec_test(testbed = "shamir")]
async fn setup_but_unusable(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.revoke_user("bob"); // Bob has 2 shares
        builder.revoke_user("mallory"); // Mallory has 1 share
                                        // Threshold is 2, and only Mike with 1 share remains
        builder.certificates_storage_fetch_certificates("mike@dev1");
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let mike = env.local_device("mike@dev1");
    let client = client_factory(&env.discriminant_dir, mike).await;

    p_assert_matches!(
        client.get_shamir_recovery_share_data(alice.user_id).await.unwrap_err(),
        ClientGetShamirRecoveryShareDataError::ShamirRecoveryUnusable { user_id } if user_id == alice.user_id
    );
}

#[parsec_test(testbed = "shamir")]
async fn stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    client.certificates_ops.stop().await.unwrap();

    p_assert_matches!(
        client
            .get_shamir_recovery_share_data(bob.user_id)
            .await
            .unwrap_err(),
        ClientGetShamirRecoveryShareDataError::Stopped
    );
}
