// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;

use libparsec_tests_fixtures::prelude::*;

use crate::certif::CertifForgetAllCertificatesError;

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.certificates_storage_fetch_certificates("alice@dev1");
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let add_first_certificate = async || {
        let first_certificate = env.get_certificates_signed().next().unwrap();
        ops.add_certificates_batch(&[first_certificate], &[], &[], &HashMap::new())
            .await
    };

    // Sanity check: make sure the certificate ops already contains a certificate
    p_assert_matches!(add_first_certificate().await, Err(_));

    ops.forget_all_certificates().await.unwrap();

    add_first_certificate().await.unwrap();
}

#[parsec_test(testbed = "minimal")]
async fn empty(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    ops.forget_all_certificates().await.unwrap();
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    ops.stop().await.unwrap();

    let err = ops.forget_all_certificates().await.unwrap_err();

    p_assert_matches!(err, CertifForgetAllCertificatesError::Stopped);
}
