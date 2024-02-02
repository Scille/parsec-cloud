// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;

use crate::certif::CertifGetUserDeviceError;

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.certificates_storage_fetch_certificates("alice@dev1");
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let res = ops
        .get_user_device("alice@dev1".parse().unwrap())
        .await
        .unwrap();

    p_assert_eq!(res.0.id, "alice".parse().unwrap());
    p_assert_eq!(res.1.id, "alice@dev1".parse().unwrap());
}

#[parsec_test(testbed = "minimal")]
async fn non_existing(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .get_user_device("alice@dev1".parse().unwrap())
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifGetUserDeviceError::NonExisting);
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    ops.stop().await.unwrap();

    let err = ops
        .get_user_device("alice@dev1".parse().unwrap())
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifGetUserDeviceError::Stopped);
}
