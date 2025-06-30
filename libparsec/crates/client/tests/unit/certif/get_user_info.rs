// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::UserProfile;

use crate::certif::CertifGetUserInfoError;

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.certificates_storage_fetch_certificates("alice@dev1");
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let res = ops.get_user_info(alice.user_id).await.unwrap();

    p_assert_eq!(res.id, alice.user_id);
    p_assert_eq!(res.human_handle, alice.human_handle);
    p_assert_eq!(res.current_profile, UserProfile::Admin);
    p_assert_eq!(res.created_on, "2000-01-02T00:00:00Z".parse().unwrap());
    p_assert_eq!(res.created_by, None);
    p_assert_eq!(res.revoked_on, None);
    p_assert_eq!(res.revoked_by, None);
}

#[parsec_test(testbed = "minimal")]
async fn revoked(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.new_user("bob");
        builder.revoke_user("bob");
        builder.certificates_storage_fetch_certificates("alice@dev1");
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let bob = env.local_device("bob@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let res = ops.get_user_info(bob.user_id).await.unwrap();

    p_assert_eq!(res.id, bob.user_id);
    p_assert_eq!(res.human_handle, bob.human_handle);
    p_assert_eq!(res.current_profile, UserProfile::Standard);
    p_assert_eq!(res.created_on, "2000-01-03T00:00:00Z".parse().unwrap());
    p_assert_eq!(res.created_by, Some(alice.device_id));
    p_assert_eq!(
        res.revoked_on,
        Some("2000-01-04T00:00:00Z".parse().unwrap())
    );
    p_assert_eq!(res.revoked_by, Some(alice.device_id));
}

#[parsec_test(testbed = "minimal")]
async fn non_existing(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops.get_user_info("bob".parse().unwrap()).await.unwrap_err();

    p_assert_matches!(err, CertifGetUserInfoError::NonExisting);
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    ops.stop().await.unwrap();

    let err = ops.get_user_info(alice.user_id).await.unwrap_err();

    p_assert_matches!(err, CertifGetUserInfoError::Stopped);
}
