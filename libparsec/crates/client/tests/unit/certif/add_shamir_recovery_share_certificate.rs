// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_tests_fixtures::prelude::*;

use crate::certif::{
    CertifAddCertificatesBatchError, InvalidCertificateError, MaybeRedactedSwitch,
};

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.new_user("bob");

        builder.new_shamir_recovery(
            "bob",
            1,
            [("alice".parse().unwrap(), 1.try_into().unwrap())],
        );
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

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
    env.customize(|builder| {
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
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

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
    let timestamp = env
        .customize(|builder| {
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
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

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
    env.customize(|builder| {
        builder.new_user("bob");

        builder.new_shamir_recovery(
            "bob",
            1,
            [("alice".parse().unwrap(), 1.try_into().unwrap())],
        );
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

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

#[parsec_test(testbed = "minimal")]
async fn invalid_user_id(env: &TestbedEnv) {
    let bob_user_id = env
        .customize(|builder| {
            let bob_user_id = builder.new_user("bob").map(|u| u.user_id);

            builder.new_shamir_recovery(
                "bob",
                1,
                [("alice".parse().unwrap(), 1.try_into().unwrap())],
            );

            bob_user_id
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    // Patch the certificate to have an invalid user id

    let shamir_recovery_certificates = match env.template.events.last().unwrap() {
        TestbedEvent::NewShamirRecovery(e) => {
            let mut rev_certifs = e
                .certificates(&env.template)
                .collect::<Vec<_>>()
                .into_iter()
                .rev();
            let share = rev_certifs.next().unwrap();
            let brief = rev_certifs.next().unwrap();
            let share_certificate = match share.certificate {
                libparsec_types::AnyArcCertificate::ShamirRecoveryShare(mut c) => {
                    let c_mut = Arc::make_mut(&mut c);
                    c_mut.user_id = alice.user_id;
                    c_mut.dump_and_sign(&alice.signing_key)
                }
                _ => unreachable!(),
            };
            vec![brief.signed, share_certificate.into()]
        }
        _ => unreachable!(),
    };

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
            InvalidCertificateError::ShamirRecoveryNotAboutSelf { user_id, author_user_id, .. }
            if user_id == alice.user_id && author_user_id == bob_user_id
        )
    );
}
