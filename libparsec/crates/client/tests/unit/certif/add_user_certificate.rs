// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certif::{
    CertifAddCertificatesBatchError, InvalidCertificateError, MaybeRedactedSwitch,
};

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let switch = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &Default::default(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch { .. });
}

#[parsec_test(testbed = "minimal")]
async fn content_already_exists(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.new_user("bob").customize(|event| {
            event.first_device_id = "alice@dev1".parse().unwrap();
        });
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &Default::default(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed) if matches!(*boxed,
            InvalidCertificateError::ContentAlreadyExists { .. }
        )
    )
}

#[parsec_test(testbed = "minimal")]
async fn older_than_author(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.new_user("bob").customize(|event| {
            event.timestamp = DateTime::from_ymd_hms_us(1999, 1, 1, 0, 0, 0, 0).unwrap();
        });
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
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
            if author_created_on == DateTime::from_ymd_hms_us(2000, 1, 2, 0, 0, 0, 0).unwrap()
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn invalid_timestamp(env: &TestbedEnv) {
    let timestamp = env
        .customize(|builder| {
            let timestamp = builder.new_user("mike").map(|e| e.timestamp);
            builder.new_user("bob").customize(|event| {
                event.timestamp = timestamp;
            });
            timestamp
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
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
async fn non_existing_author(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.new_user("bob");
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let (_, bob_signed) = env.get_user_certificate("bob");

    let err = ops
        .add_certificates_batch(&[bob_signed], &[], &[], &Default::default())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(
            *boxed,
            InvalidCertificateError::NonExistingAuthor { .. }
        )
    )
}

#[parsec_test(testbed = "minimal")]
async fn ok_non_root(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.new_user("bob");
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let switch = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &Default::default(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch { .. });
}

#[parsec_test(testbed = "minimal")]
async fn revoked(env: &TestbedEnv) {
    env.customize(|builder| {
        builder
            .new_user("bob")
            .with_initial_profile(UserProfile::Admin);
        builder.revoke_user("bob");
        builder
            .new_user("mallory")
            .with_author("bob@dev1".try_into().unwrap());
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
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
            if author_revoked_on == DateTime::from_ymd_hms_us(2000, 1, 4, 0, 0, 0, 0).unwrap()
        )
    );
}

#[parsec_test(testbed = "minimal")]
#[case(UserProfile::Standard)]
#[case(UserProfile::Outsider)]
async fn not_admin(#[case] profile: UserProfile, env: &TestbedEnv) {
    env.customize(|builder| {
        builder.new_user("bob").with_initial_profile(profile);
        builder
            .new_user("mallory")
            .with_author("bob@dev1".try_into().unwrap());
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &Default::default(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(
            *boxed,
            InvalidCertificateError::AuthorNotAdmin { author_profile, .. }
            if author_profile == profile
        )
    );
}
