// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;

use crate::certif::CertifEncryptForSequesterServicesError;

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "empty")]
async fn ok(env: &TestbedEnv) {
    let (env, (service_id, sequester_pubkey)) = env.customize_with_map(|builder| {
        builder
            .bootstrap_organization("alice")
            .and_set_sequestered_organization();
        builder
            .new_sequester_service()
            .map(|event| (event.id, event.encryption_private_key.clone()))
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    ops.add_certificates_batch(
        &env.get_common_certificates_signed(),
        &env.get_sequester_certificates_signed(),
        &[],
        &Default::default(),
    )
    .await
    .unwrap();

    let res = ops
        .encrypt_for_sequester_services(b"data")
        .await
        .unwrap()
        .unwrap();

    p_assert_eq!(res.len(), 1);

    p_assert_eq!(res[0].0, service_id);
    p_assert_eq!(sequester_pubkey.decrypt(&res[0].1).unwrap(), b"data");
}

#[parsec_test(testbed = "empty")]
async fn multiple(env: &TestbedEnv) {
    let (env, (id0, pubkey0, id1, pubkey1)) = env.customize_with_map(|builder| {
        builder
            .bootstrap_organization("alice")
            .and_set_sequestered_organization();
        let (id0, pubkey0) = builder
            .new_sequester_service()
            .map(|event| (event.id, event.encryption_private_key.clone()));

        let (id1, pubkey1) = builder
            .new_sequester_service()
            .map(|event| (event.id, event.encryption_private_key.clone()));

        (id0, pubkey0, id1, pubkey1)
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    ops.add_certificates_batch(
        &env.get_common_certificates_signed(),
        &env.get_sequester_certificates_signed(),
        &[],
        &Default::default(),
    )
    .await
    .unwrap();

    let res = ops
        .encrypt_for_sequester_services(b"data")
        .await
        .unwrap()
        .unwrap();

    p_assert_eq!(res.len(), 2);

    p_assert_eq!(res[0].0, id0);
    p_assert_eq!(pubkey0.decrypt(&res[0].1).unwrap(), b"data");
    p_assert_eq!(res[1].0, id1);
    p_assert_eq!(pubkey1.decrypt(&res[1].1).unwrap(), b"data");
}

#[parsec_test(testbed = "minimal")]
async fn empty(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let res = ops.encrypt_for_sequester_services(b"data").await.unwrap();

    p_assert_eq!(res, None);
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    ops.stop().await.unwrap();

    let err = ops
        .encrypt_for_sequester_services(b"data")
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifEncryptForSequesterServicesError::Stopped);
}
