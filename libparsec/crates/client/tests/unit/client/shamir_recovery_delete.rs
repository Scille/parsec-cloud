// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;

use crate::{ClientDeleteShamirRecoveryError, SelfShamirRecoveryInfo};

use super::utils::client_factory;

#[parsec_test(testbed = "shamir", with_server)]
async fn ok(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    // Sanity check
    p_assert_matches!(
        client.get_self_shamir_recovery().await.unwrap(),
        SelfShamirRecoveryInfo::SetupAllValid { .. }
    );

    client.delete_shamir_recovery().await.unwrap();

    p_assert_matches!(
        client.get_self_shamir_recovery().await.unwrap(),
        SelfShamirRecoveryInfo::Deleted{ deleted_by, .. }
        if deleted_by == alice.device_id
    );
}

#[parsec_test(testbed = "shamir", with_server)]
async fn already_deleted_remote_idempotent(env: &TestbedEnv) {
    let (deletion_author, deletion_timestamp) = env
        .customize(|builder| {
            builder
                .delete_shamir_recovery("alice")
                .map(|e| (e.author, e.timestamp))
        })
        .await;

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    // Sanity check
    p_assert_matches!(
        client.get_self_shamir_recovery().await.unwrap(),
        SelfShamirRecoveryInfo::SetupAllValid { .. }
    );

    client.delete_shamir_recovery().await.unwrap();

    // Now the client has fetched the missing certificates
    p_assert_matches!(
        client.get_self_shamir_recovery().await.unwrap(),
        SelfShamirRecoveryInfo::Deleted{ deleted_by, deleted_on, .. }
        if deleted_by == deletion_author
        || deleted_on == deletion_timestamp
    );
}

#[parsec_test(testbed = "shamir")]
async fn already_deleted_local_idempotent(env: &TestbedEnv) {
    let bob = env.local_device("bob@dev1");
    let client = client_factory(&env.discriminant_dir, bob.clone()).await;

    let (deletion_author, deletion_timestamp) =
        match client.get_self_shamir_recovery().await.unwrap() {
            SelfShamirRecoveryInfo::Deleted {
                deleted_on,
                deleted_by,
                ..
            } => (deleted_by, deleted_on),
            _ => unreachable!(),
        };

    client.delete_shamir_recovery().await.unwrap();

    // Make sure nothing has changed
    p_assert_matches!(
        client.get_self_shamir_recovery().await.unwrap(),
        SelfShamirRecoveryInfo::Deleted{ deleted_by, deleted_on, .. }
        if deleted_by == deletion_author
        || deleted_on == deletion_timestamp
    );
}

#[parsec_test(testbed = "shamir", with_server)]
async fn timestamp_out_of_ballpark(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;
    // Mock Alice clock to be 1h too late, but not the one used to communicate with
    // the server (so that only the content is rejected, not the request itself)
    alice.time_provider.mock_time_shifted(-3_600_000_000);
    client.cmds.time_provider.mock_time_shifted(3_600_000_000);

    let err = client.delete_shamir_recovery().await.unwrap_err();

    p_assert_matches!(
        err,
        ClientDeleteShamirRecoveryError::TimestampOutOfBallpark { .. }
    );
}

#[parsec_test(testbed = "shamir")]
async fn shamir_recovery_stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    client.certificates_ops.stop().await.unwrap();

    let err = client.delete_shamir_recovery().await.unwrap_err();

    p_assert_matches!(err, ClientDeleteShamirRecoveryError::Stopped);
}

#[parsec_test(testbed = "shamir")]
async fn shamir_recovery_offline(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    let err = client.delete_shamir_recovery().await.unwrap_err();

    p_assert_matches!(err, ClientDeleteShamirRecoveryError::Offline(_));
}
