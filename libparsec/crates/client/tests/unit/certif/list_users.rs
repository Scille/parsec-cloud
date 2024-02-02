// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;

use crate::certif::CertifListUsersError;

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    ops.add_certificates_batch(
        &env.get_common_certificates_signed(),
        &[],
        &[],
        &Default::default(),
    )
    .await
    .unwrap();

    let res = ops
        .list_users(false, None, None)
        .await
        .unwrap()
        .into_iter()
        .map(|x| x.id)
        .collect::<Vec<_>>();

    p_assert_eq!(res, ["alice".parse().unwrap()]);
}

#[parsec_test(testbed = "minimal")]
async fn multiple(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    ops.add_certificates_batch(
        &env.get_common_certificates_signed(),
        &[],
        &[],
        &Default::default(),
    )
    .await
    .unwrap();

    let res = ops
        .list_users(false, None, None)
        .await
        .unwrap()
        .into_iter()
        .map(|x| x.id)
        .collect::<Vec<_>>();

    p_assert_eq!(res, ["alice".parse().unwrap(), "bob".parse().unwrap()]);
}

#[parsec_test(testbed = "minimal")]
async fn revoked(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.revoke_user("bob");
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    ops.add_certificates_batch(
        &env.get_common_certificates_signed(),
        &[],
        &[],
        &Default::default(),
    )
    .await
    .unwrap();

    let res = ops
        .list_users(false, None, None)
        .await
        .unwrap()
        .into_iter()
        .map(|x| x.id)
        .collect::<Vec<_>>();

    p_assert_eq!(res, ["alice".parse().unwrap(), "bob".parse().unwrap()]);
}

#[parsec_test(testbed = "minimal")]
async fn revoked_filtered(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.revoke_user("bob");
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    ops.add_certificates_batch(
        &env.get_common_certificates_signed(),
        &[],
        &[],
        &Default::default(),
    )
    .await
    .unwrap();

    let res = ops
        .list_users(true, None, None)
        .await
        .unwrap()
        .into_iter()
        .map(|x| x.id)
        .collect::<Vec<_>>();

    p_assert_eq!(res, ["alice".parse().unwrap()]);
}

#[parsec_test(testbed = "minimal")]
async fn empty(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let res = ops.list_users(false, None, None).await.unwrap();

    assert!(res.is_empty());
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    ops.stop().await.unwrap();

    let err = ops.list_users(false, None, None).await.unwrap_err();

    p_assert_matches!(err, CertifListUsersError::Stopped);
}
