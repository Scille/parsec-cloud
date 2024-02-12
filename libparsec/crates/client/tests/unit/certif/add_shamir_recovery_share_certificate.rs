// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;

use crate::certif::{
    CertifAddCertificatesBatchError, InvalidCertificateError, MaybeRedactedSwitch,
};

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");

        builder.new_shamir_recovery(
            "bob",
            1,
            [("alice".parse().unwrap(), 1.try_into().unwrap())],
        );
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let switch = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &env.get_shamir_recovery_certificates_signed(),
            &Default::default(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch)
}

#[parsec_test(testbed = "minimal")]
async fn content_already_exists(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");

        builder.new_shamir_recovery(
            "bob",
            1,
            [("alice".parse().unwrap(), 1.try_into().unwrap())],
        );

        builder.new_shamir_recovery(
            "bob",
            1,
            [("alice".parse().unwrap(), 1.try_into().unwrap())],
        );
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &env.get_shamir_recovery_certificates_signed(),
            &Default::default(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(*boxed, InvalidCertificateError::ContentAlreadyExists { .. })
    )
}

#[parsec_test(testbed = "minimal")]
async fn timestamp_mismatch_with_brief(env: &TestbedEnv) {
    let (env, timestamp) = env.customize_with_map(|builder| {
        builder.new_user("bob");

        let timestamp = builder
            .new_shamir_recovery(
                "bob",
                1,
                [("alice".parse().unwrap(), 1.try_into().unwrap())],
            )
            .map(|event| event.timestamp);

        builder.new_shamir_recovery(
            "bob",
            1,
            [("alice".parse().unwrap(), 1.try_into().unwrap())],
        );

        timestamp
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let mut shamir_recovery_certificates = env.get_shamir_recovery_certificates_signed();
    shamir_recovery_certificates.remove(shamir_recovery_certificates.len() - 2);
    shamir_recovery_certificates.remove(shamir_recovery_certificates.len() - 2);

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &shamir_recovery_certificates,
            &Default::default(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(
            *boxed,
            InvalidCertificateError::InvalidTimestamp {
                last_certificate_timestamp,
                ..
            }
            if last_certificate_timestamp == timestamp
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn missing_brief(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");

        builder.new_shamir_recovery(
            "bob",
            1,
            [("alice".parse().unwrap(), 1.try_into().unwrap())],
        );
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let mut shamir_recovery_certificates = env.get_shamir_recovery_certificates_signed();
    shamir_recovery_certificates.remove(shamir_recovery_certificates.len() - 2);

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &shamir_recovery_certificates,
            &Default::default(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(
            *boxed,
            InvalidCertificateError::ShamirRecoveryMissingBriefCertificate { .. }
        )
    );
}
