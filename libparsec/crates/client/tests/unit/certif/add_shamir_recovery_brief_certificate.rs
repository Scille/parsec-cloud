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

    // To test `shamir_recovery_brief_certificate`, we remove the `shamir_recovery_share_certificate`
    // which is tested in another file.
    let mut shamir_recovery_certificates = env.get_shamir_recovery_certificates_signed();
    shamir_recovery_certificates.pop();

    let switch = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &shamir_recovery_certificates,
            &Default::default(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch)
}

#[parsec_test(testbed = "minimal")]
async fn multiple(env: &TestbedEnv) {
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

    // To test `shamir_recovery_brief_certificate`, we remove the `shamir_recovery_share_certificates`
    // which is tested in another file.
    let mut shamir_recovery_certificates = env.get_shamir_recovery_certificates_signed();
    shamir_recovery_certificates.pop();
    shamir_recovery_certificates.remove(shamir_recovery_certificates.len() - 1);

    let switch = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &shamir_recovery_certificates,
            &Default::default(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch)
}

#[parsec_test(testbed = "minimal")]
async fn older_than_author(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");

        builder
            .new_shamir_recovery(
                "bob",
                1,
                [("alice".parse().unwrap(), 1.try_into().unwrap())],
            )
            .customize(|event| {
                event.timestamp = DateTime::from_ymd_hms_us(1999, 1, 1, 0, 0, 0, 0).unwrap()
            });
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    // To test `shamir_recovery_brief_certificate`, we remove the `shamir_recovery_share_certificate`
    // which is tested in another file.
    let mut shamir_recovery_certificates = env.get_shamir_recovery_certificates_signed();
    shamir_recovery_certificates.pop();

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
            InvalidCertificateError::OlderThanAuthor {
                author_created_on,
                ..
            }
            if author_created_on == DateTime::from_ymd_hms_us(2000, 1, 3, 0, 0, 0, 0).unwrap()
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn invalid_timestamp(env: &TestbedEnv) {
    let (env, timestamp) = env.customize_with_map(|builder| {
        builder.new_user("bob");

        let timestamp = builder
            .new_shamir_recovery(
                "bob",
                1,
                [("alice".parse().unwrap(), 1.try_into().unwrap())],
            )
            .map(|event| event.timestamp);

        builder
            .new_shamir_recovery(
                "bob",
                1,
                [("alice".parse().unwrap(), 1.try_into().unwrap())],
            )
            .customize(|event| event.timestamp = timestamp);

        timestamp
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    // To test `shamir_recovery_brief_certificate`, we remove the `shamir_recovery_share_certificate`
    // which is tested in another file.
    let mut shamir_recovery_certificates = env.get_shamir_recovery_certificates_signed();
    shamir_recovery_certificates.pop();

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
async fn revoked(env: &TestbedEnv) {
    let (env, timestamp) = env.customize_with_map(|builder| {
        builder.new_user("bob");
        let timestamp = builder.revoke_user("bob").map(|event| event.timestamp);

        builder
            // Check can't be bypassed
            .new_shamir_recovery(
                "alice",
                1,
                [("alice".parse().unwrap(), 1.try_into().unwrap())],
            )
            .customize(|event| event.author = "bob@dev1".parse().unwrap());

        timestamp
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    // To test `shamir_recovery_brief_certificate`, we remove the `shamir_recovery_share_certificate`
    // which is tested in another file.
    let mut shamir_recovery_certificates = env.get_shamir_recovery_certificates_signed();
    shamir_recovery_certificates.pop();

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
            InvalidCertificateError::RevokedAuthor { author_revoked_on, .. }
            if author_revoked_on == timestamp
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn threshold_greater_than_share(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");

        builder.new_shamir_recovery(
            "bob",
            2,
            [("alice".parse().unwrap(), 1.try_into().unwrap())],
        );
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    // To test `shamir_recovery_brief_certificate`, we remove the `shamir_recovery_share_certificate`
    // which is tested in another file.
    let mut shamir_recovery_certificates = env.get_shamir_recovery_certificates_signed();
    shamir_recovery_certificates.pop();

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
        if matches!(*boxed, InvalidCertificateError::Corrupted { .. })
    )
}

#[parsec_test(testbed = "minimal")]
async fn threshold_equal_sum_of_shares(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.new_user("mallory");

        builder.new_shamir_recovery(
            "bob",
            2,
            [
                ("alice".parse().unwrap(), 1.try_into().unwrap()),
                ("mallory".parse().unwrap(), 1.try_into().unwrap()),
            ],
        );
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    // To test `shamir_recovery_brief_certificate`, we remove the `shamir_recovery_share_certificate`
    // which is tested in another file.
    let mut shamir_recovery_certificates = env.get_shamir_recovery_certificates_signed();
    shamir_recovery_certificates.pop();

    let switch = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &shamir_recovery_certificates,
            &Default::default(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch);
}

#[parsec_test(testbed = "minimal")]
async fn self_recipient(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_shamir_recovery(
            "alice",
            1,
            [("alice".parse().unwrap(), 1.try_into().unwrap())],
        );
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    // To test `shamir_recovery_brief_certificate`, we remove the `shamir_recovery_share_certificate`
    // which is tested in another file.
    let mut shamir_recovery_certificates = env.get_shamir_recovery_certificates_signed();
    shamir_recovery_certificates.pop();

    let switch = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &shamir_recovery_certificates,
            &Default::default(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch);
}

#[parsec_test(testbed = "minimal")]
async fn recipient_revoked(env: &TestbedEnv) {
    let (env, timestamp) = env.customize_with_map(|builder| {
        builder.new_user("bob");

        let timestamp = builder.revoke_user("bob").map(|event| event.timestamp);

        builder.new_shamir_recovery(
            "alice",
            1,
            [("bob".parse().unwrap(), 1.try_into().unwrap())],
        );

        timestamp
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    // To test `shamir_recovery_brief_certificate`, we remove the `shamir_recovery_share_certificate`
    // which is tested in another file.
    let mut shamir_recovery_certificates = env.get_shamir_recovery_certificates_signed();
    shamir_recovery_certificates.pop();

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
            InvalidCertificateError::RelatedUserAlreadyRevoked { user_revoked_on, .. }
            if user_revoked_on == timestamp
        )
    )
}
