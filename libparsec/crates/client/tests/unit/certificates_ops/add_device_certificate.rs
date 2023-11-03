// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certificates_ops::{AddCertificateError, InvalidCertificateError, MaybeRedactedSwitch};

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let store = ops.store.for_write().await;
    let certificates = env.get_certificates_signed();

    let switch = ops
        .add_certificates_batch(&store, 0, certificates)
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch);
}

#[parsec_test(testbed = "minimal")]
async fn content_already_exists(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, alice_dev1_signed) = env.get_device_certificate("alice@dev1");

    let err = ops
        .add_certificates_batch(
            &store,
            0,
            [alice_signed, alice_dev1_signed.clone(), alice_dev1_signed].into_iter(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        AddCertificateError::InvalidCertificate(
            InvalidCertificateError::ContentAlreadyExists { .. }
        )
    )
}

#[parsec_test(testbed = "minimal")]
async fn index_already_exists(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, alice_dev1_signed) = env.get_device_certificate("alice@dev1");

    ops.add_certificates_batch(&store, 0, [alice_signed].into_iter())
        .await
        .unwrap();

    let err = ops
        .add_certificates_batch(&store, 0, [alice_dev1_signed].into_iter())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        AddCertificateError::InvalidCertificate(InvalidCertificateError::IndexAlreadyExists { .. })
    );
}

#[parsec_test(testbed = "minimal")]
async fn invalid_timestamp(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_device("alice").customize(|event| {
            event.timestamp = DateTime::from_ymd_hms_us(1999, 1, 1, 0, 0, 0, 0).unwrap()
        });
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let certificates = env.get_certificates_signed();

    let err = ops
        .add_certificates_batch(&store, 0, certificates)
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        AddCertificateError::InvalidCertificate(InvalidCertificateError::InvalidTimestamp {
            last_certificate_timestamp,
            ..
        })
        if last_certificate_timestamp == DateTime::from_ymd_hms_us(2000, 1, 2, 0, 0, 0, 0).unwrap()
    );
}

#[parsec_test(testbed = "minimal")]
async fn non_existing_related_user(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_dev1_signed) = env.get_device_certificate("alice@dev1");

    let err = ops
        .add_certificates_batch(&store, 0, [alice_dev1_signed].into_iter())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        AddCertificateError::InvalidCertificate(
            InvalidCertificateError::NonExistingRelatedUser { .. }
        )
    )
}

#[parsec_test(testbed = "minimal")]
async fn ok_non_root(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let certificates = env.get_certificates_signed();

    let switch = ops
        .add_certificates_batch(&store, 0, certificates)
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch)
}

#[parsec_test(testbed = "minimal")]
async fn revoked(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder
            .new_user("bob")
            .with_initial_profile(UserProfile::Admin);
        builder.new_user("mallory");
        builder.revoke_user("bob");
        builder
            .new_device("mallory")
            .with_author("bob@dev1".try_into().unwrap());
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let certificates = env.get_certificates_signed();

    let err = ops
        .add_certificates_batch(&store, 0, certificates)
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        AddCertificateError::InvalidCertificate(InvalidCertificateError::RevokedAuthor { author_revoked_on, .. })
        if author_revoked_on == DateTime::from_ymd_hms_us(2000, 1, 5, 0, 0, 0, 0).unwrap()
    );
}

#[parsec_test(testbed = "minimal")]
#[case(UserProfile::Standard)]
#[case(UserProfile::Outsider)]
async fn not_admin(#[case] profile: UserProfile, env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob").with_initial_profile(profile);
        builder.new_user("mallory");
        builder
            .new_device("mallory")
            .with_author("bob@dev1".try_into().unwrap());
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let certificates = env.get_certificates_signed();

    let err = ops
        .add_certificates_batch(&store, 0, certificates)
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        AddCertificateError::InvalidCertificate(InvalidCertificateError::AuthorNotAdmin { author_profile, .. })
        if author_profile == profile
    );
}

#[parsec_test(testbed = "minimal")]
async fn self_author(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder
            .new_device("alice")
            .with_author("alice@dev1".try_into().unwrap());
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let certificates = env.get_certificates_signed();

    let switch = ops
        .add_certificates_batch(&store, 0, certificates)
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch);
}
