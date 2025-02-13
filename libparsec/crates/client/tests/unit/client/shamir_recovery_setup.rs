// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{CertifSetupShamirRecoveryError, SelfShamirRecoveryInfo};

use super::utils::client_factory;

#[parsec_test(testbed = "coolorg", with_server)]
async fn ok_first_setup(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    // Sanity check
    p_assert_matches!(
        client.get_self_shamir_recovery().await.unwrap(),
        SelfShamirRecoveryInfo::NeverSetup
    );

    client
        .setup_shamir_recovery(
            // Ask for the max allowed number of shares (i.e. 255)
            HashMap::from([
                ("bob".parse().unwrap(), 128.try_into().unwrap()),
                ("mallory".parse().unwrap(), 127.try_into().unwrap()),
            ]),
            255.try_into().unwrap(),
        )
        .await
        .unwrap();

    p_assert_matches!(
        client.get_self_shamir_recovery().await.unwrap(),
        SelfShamirRecoveryInfo::SetupAllValid{ created_by, .. }
        if created_by == alice.device_id
    );
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn ok_with_previously_deleted(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.new_shamir_recovery(
            "alice",
            1,
            [("bob".parse().unwrap(), 1.try_into().unwrap())],
            "alice@dev2",
        );
        builder.delete_shamir_recovery("alice");
        builder.certificates_storage_fetch_certificates("alice@dev1");
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    // Sanity check
    p_assert_matches!(
        client.get_self_shamir_recovery().await.unwrap(),
        SelfShamirRecoveryInfo::Deleted { .. }
    );

    let bob_user_id: UserID = "bob".parse().unwrap();
    let share_recipients = HashMap::from([(bob_user_id, 2.try_into().unwrap())]);
    client
        .setup_shamir_recovery(share_recipients, 1.try_into().unwrap())
        .await
        .unwrap();

    p_assert_matches!(
        client.get_self_shamir_recovery().await.unwrap(),
        SelfShamirRecoveryInfo::SetupAllValid{ created_by, .. }
        if created_by == alice.device_id
    );
}

#[parsec_test(testbed = "coolorg")]
async fn offline(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    let err = client
        .setup_shamir_recovery(
            HashMap::from([("bob".parse().unwrap(), 1.try_into().unwrap())]),
            1.try_into().unwrap(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifSetupShamirRecoveryError::Offline(_));
}

#[parsec_test(testbed = "coolorg")]
async fn stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    client.certificates_ops.stop().await.unwrap();

    let err = client
        .setup_shamir_recovery(
            HashMap::from([("bob".parse().unwrap(), 1.try_into().unwrap())]),
            1.try_into().unwrap(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifSetupShamirRecoveryError::Stopped);
}

#[parsec_test(testbed = "coolorg")]
async fn threshold_bigger_than_sum_of_shares(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    let err = client
        .setup_shamir_recovery(
            HashMap::from([("bob".parse().unwrap(), 1.try_into().unwrap())]),
            2.try_into().unwrap(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifSetupShamirRecoveryError::ThresholdBiggerThanSumOfShares
    );
}

#[parsec_test(testbed = "coolorg")]
async fn too_many_shares(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    let err = client
        .setup_shamir_recovery(
            HashMap::from([
                ("bob".parse().unwrap(), 129.try_into().unwrap()),
                ("mallory".parse().unwrap(), 128.try_into().unwrap()),
            ]),
            1.try_into().unwrap(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifSetupShamirRecoveryError::TooManyShares);
}

#[parsec_test(testbed = "coolorg")]
async fn author_among_recipients(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    let err = client
        .setup_shamir_recovery(
            HashMap::from([
                ("bob".parse().unwrap(), 1.try_into().unwrap()),
                (alice.user_id, 1.try_into().unwrap()),
            ]),
            2.try_into().unwrap(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifSetupShamirRecoveryError::AuthorAmongRecipients);
}

#[parsec_test(testbed = "coolorg")]
async fn recipient_not_found(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;
    let dummy_user_id = UserID::default();

    let err = client
        .setup_shamir_recovery(
            HashMap::from([(dummy_user_id, 1.try_into().unwrap())]),
            1.try_into().unwrap(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifSetupShamirRecoveryError::RecipientNotFound);
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn recipient_revoked(#[values("on_local", "on_server")] kind: &str, env: &TestbedEnv) {
    env.customize(|builder| {
        builder.revoke_user("bob");

        match kind {
            "on_local" => {
                builder.certificates_storage_fetch_certificates("alice@dev1");
            }
            "on_server" => (),
            unknown => panic!("Unknown kind: {}", unknown),
        }
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    let err = client
        .setup_shamir_recovery(
            HashMap::from([
                ("bob".parse().unwrap(), 1.try_into().unwrap()),
                ("mallory".parse().unwrap(), 1.try_into().unwrap()),
            ]),
            2.try_into().unwrap(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifSetupShamirRecoveryError::RecipientRevoked);
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn shamir_recovery_already_exists(
    #[values("on_local", "on_server")] kind: &str,
    env: &TestbedEnv,
) {
    env.customize(|builder| {
        builder.new_shamir_recovery(
            "alice",
            1,
            [("bob".parse().unwrap(), 1.try_into().unwrap())],
            "alice@dev2",
        );

        match kind {
            "on_local" => {
                builder.certificates_storage_fetch_certificates("alice@dev1");
            }
            "on_server" => (),
            unknown => panic!("Unknown kind: {}", unknown),
        }
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    let err = client
        .setup_shamir_recovery(
            HashMap::from([("bob".parse().unwrap(), 1.try_into().unwrap())]),
            1.try_into().unwrap(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifSetupShamirRecoveryError::ShamirRecoveryAlreadyExists
    );
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn timestamp_out_of_ballpark(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;
    // Mock Alice clock to be 1h too late, but not the one used to communicate with
    // the server (so that only the content is rejected, not the request itself)
    alice.time_provider.mock_time_shifted(-3_600_000_000);
    client.cmds.time_provider.mock_time_shifted(3_600_000_000);

    let err = client
        .setup_shamir_recovery(
            HashMap::from([("bob".parse().unwrap(), 1.try_into().unwrap())]),
            1.try_into().unwrap(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifSetupShamirRecoveryError::TimestampOutOfBallpark { .. }
    );
}
