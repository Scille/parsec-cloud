// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::{HashMap, HashSet};

use libparsec_tests_fixtures::prelude::*;

use crate::{CertifListShamirRecoveriesForOthersError, OtherShamirRecoveryInfo};

use super::utils::client_factory;

#[parsec_test(testbed = "shamir")]
async fn deleted(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    p_assert_eq!(
        client.list_shamir_recoveries_for_others().await.unwrap(),
        vec![OtherShamirRecoveryInfo::Deleted {
            user_id: "bob".parse().unwrap(),
            created_on: "2000-01-10T00:00:00Z".parse().unwrap(),
            created_by: "bob@dev1".parse().unwrap(),
            threshold: 1.try_into().unwrap(),
            per_recipient_shares: HashMap::from_iter([
                ("alice".parse().unwrap(), 1.try_into().unwrap()),
                ("mallory".parse().unwrap(), 1.try_into().unwrap()),
            ]),
            deleted_on: "2000-01-11T00:00:00Z".parse().unwrap(),
            deleted_by: "bob@dev1".parse().unwrap(),
        }]
    );
}

#[parsec_test(testbed = "shamir")]
async fn setup_all_valid_while_never_deleted(env: &TestbedEnv) {
    let mike = env.local_device("mike@dev1");
    let client = client_factory(&env.discriminant_dir, mike).await;

    p_assert_eq!(
        client.list_shamir_recoveries_for_others().await.unwrap(),
        vec![
            OtherShamirRecoveryInfo::SetupAllValid {
                user_id: "alice".parse().unwrap(),
                created_on: "2000-01-09T00:00:00Z".parse().unwrap(),
                created_by: "alice@dev1".parse().unwrap(),
                threshold: 2.try_into().unwrap(),
                per_recipient_shares: HashMap::from_iter([
                    ("bob".parse().unwrap(), 2.try_into().unwrap()),
                    ("mallory".parse().unwrap(), 1.try_into().unwrap()),
                    ("mike".parse().unwrap(), 1.try_into().unwrap()),
                ]),
            },
            OtherShamirRecoveryInfo::SetupAllValid {
                user_id: "mallory".parse().unwrap(),
                created_on: "2000-01-12T00:00:00Z".parse().unwrap(),
                created_by: "mallory@dev1".parse().unwrap(),
                threshold: 1.try_into().unwrap(),
                per_recipient_shares: HashMap::from_iter([(
                    "mike".parse().unwrap(),
                    1.try_into().unwrap()
                ),]),
            }
        ]
    );
}

#[parsec_test(testbed = "shamir")]
async fn setup_all_valid_while_previously_deleted(env: &TestbedEnv) {
    let setup_timestamp = env
        .customize(|builder| {
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
            builder.certificates_storage_fetch_certificates("alice@dev1");
            setup_timestamp
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    p_assert_eq!(
        client.list_shamir_recoveries_for_others().await.unwrap(),
        vec![OtherShamirRecoveryInfo::SetupAllValid {
            user_id: "bob".parse().unwrap(),
            created_on: setup_timestamp,
            created_by: "bob@dev1".parse().unwrap(),
            threshold: 3.try_into().unwrap(),
            per_recipient_shares: HashMap::from_iter([
                ("alice".parse().unwrap(), 2.try_into().unwrap()),
                ("mike".parse().unwrap(), 1.try_into().unwrap()),
            ]),
        }]
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

    let mike = env.local_device("mike@dev1");
    let client = client_factory(&env.discriminant_dir, mike).await;

    p_assert_eq!(
        client.list_shamir_recoveries_for_others().await.unwrap(),
        vec![
            OtherShamirRecoveryInfo::SetupWithRevokedRecipients {
                user_id: "alice".parse().unwrap(),
                created_on: "2000-01-09T00:00:00Z".parse().unwrap(),
                created_by: "alice@dev1".parse().unwrap(),
                threshold: 2.try_into().unwrap(),
                per_recipient_shares: HashMap::from_iter([
                    ("bob".parse().unwrap(), 2.try_into().unwrap()),
                    ("mallory".parse().unwrap(), 1.try_into().unwrap()),
                    ("mike".parse().unwrap(), 1.try_into().unwrap()),
                ]),
                revoked_recipients: HashSet::from_iter(["bob".parse().unwrap()]),
            },
            OtherShamirRecoveryInfo::SetupAllValid {
                user_id: "mallory".parse().unwrap(),
                created_on: "2000-01-12T00:00:00Z".parse().unwrap(),
                created_by: "mallory@dev1".parse().unwrap(),
                threshold: 1.try_into().unwrap(),
                per_recipient_shares: HashMap::from_iter([(
                    "mike".parse().unwrap(),
                    1.try_into().unwrap()
                ),]),
            }
        ]
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

    let mike = env.local_device("mike@dev1");
    let client = client_factory(&env.discriminant_dir, mike).await;

    p_assert_eq!(
        client.list_shamir_recoveries_for_others().await.unwrap(),
        vec![
            OtherShamirRecoveryInfo::SetupButUnusable {
                user_id: "alice".parse().unwrap(),
                created_on: "2000-01-09T00:00:00Z".parse().unwrap(),
                created_by: "alice@dev1".parse().unwrap(),
                threshold: 2.try_into().unwrap(),
                per_recipient_shares: HashMap::from_iter([
                    ("bob".parse().unwrap(), 2.try_into().unwrap()),
                    ("mallory".parse().unwrap(), 1.try_into().unwrap()),
                    ("mike".parse().unwrap(), 1.try_into().unwrap()),
                ]),
                revoked_recipients: HashSet::from_iter([
                    "bob".parse().unwrap(),
                    "mallory".parse().unwrap()
                ]),
            },
            OtherShamirRecoveryInfo::SetupAllValid {
                user_id: "mallory".parse().unwrap(),
                created_on: "2000-01-12T00:00:00Z".parse().unwrap(),
                created_by: "mallory@dev1".parse().unwrap(),
                threshold: 1.try_into().unwrap(),
                per_recipient_shares: HashMap::from_iter([(
                    "mike".parse().unwrap(),
                    1.try_into().unwrap()
                ),]),
            }
        ]
    );
}

#[parsec_test(testbed = "shamir")]
async fn stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    client.certificates_ops.stop().await.unwrap();

    p_assert_matches!(
        client
            .list_shamir_recoveries_for_others()
            .await
            .unwrap_err(),
        CertifListShamirRecoveriesForOthersError::Stopped
    );
}
