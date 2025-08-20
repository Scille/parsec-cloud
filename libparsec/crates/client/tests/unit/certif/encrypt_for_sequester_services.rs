// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;

use crate::certif::CertifEncryptForSequesterServicesError;

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "empty")]
async fn empty(#[values("always_empty", "with_revoked_service")] kind: &str, env: &TestbedEnv) {
    env.customize(|builder| {
        builder
            .bootstrap_organization("alice")
            .and_set_sequestered_organization();
        match kind {
            "always_empty" => (),
            "with_revoked_service" => {
                let sequester_service_id = builder.new_sequester_service().map(|e| e.id);
                builder.revoke_sequester_service(sequester_service_id);
            }
            unknown => panic!("Unknown kind: {unknown}"),
        }
        builder.certificates_storage_fetch_certificates("alice@dev1");
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let res = ops.encrypt_for_sequester_services(b"data").await.unwrap();

    p_assert_eq!(res, Some(vec![]));
}

#[parsec_test(testbed = "empty")]
async fn ok(env: &TestbedEnv) {
    let (service_id, sequester_pubkey) = env
        .customize(|builder| {
            builder
                .bootstrap_organization("alice")
                .and_set_sequestered_organization();
            let (service_id, sequester_pubkey) = builder
                .new_sequester_service()
                .map(|event| (event.id, event.encryption_private_key.clone()));

            builder.certificates_storage_fetch_certificates("alice@dev1");

            (service_id, sequester_pubkey)
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

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
    let (id0, privkey0, id2, privkey2) = env
        .customize(|builder| {
            builder
                .bootstrap_organization("alice")
                .and_set_sequestered_organization();
            let (id0, privkey0) = builder
                .new_sequester_service()
                .map(|event| (event.id, event.encryption_private_key.clone()));

            let id1 = builder.new_sequester_service().map(|event| event.id);

            let (id2, privkey2) = builder
                .new_sequester_service()
                .map(|event| (event.id, event.encryption_private_key.clone()));

            // Revoked service should be ignored during encryption
            builder.revoke_sequester_service(id1);

            builder.certificates_storage_fetch_certificates("alice@dev1");

            (id0, privkey0, id2, privkey2)
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let res = ops
        .encrypt_for_sequester_services(b"data")
        .await
        .unwrap()
        .unwrap();

    p_assert_eq!(res.len(), 2);

    p_assert_eq!(res[0].0, id0);
    p_assert_eq!(privkey0.decrypt(&res[0].1).unwrap(), b"data");
    p_assert_eq!(res[1].0, id2);
    p_assert_eq!(privkey2.decrypt(&res[1].1).unwrap(), b"data");
}

#[parsec_test(testbed = "minimal")]
async fn not_sequestered(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let res = ops.encrypt_for_sequester_services(b"data").await.unwrap();

    p_assert_eq!(res, None);
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    ops.stop().await.unwrap();

    let err = ops
        .encrypt_for_sequester_services(b"data")
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifEncryptForSequesterServicesError::Stopped);
}
