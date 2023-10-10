// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certificates_ops::{AddCertificateError, InvalidCertificateError, MaybeRedactedSwitch};

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.revoke_user("bob");
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, alice_dev1_signed) = env.get_device_certificate("alice@dev1");
    let (_, bob_signed) = env.get_user_certificate("bob");
    let (_, bob_dev1_signed) = env.get_device_certificate("bob@dev1");
    let (_, bob_revoked_signed) = env.get_revoked_certificate("bob");

    let switch = ops
        .add_certificates_batch(
            &store,
            0,
            [
                alice_signed,
                alice_dev1_signed,
                bob_signed,
                bob_dev1_signed,
                bob_revoked_signed,
            ]
            .into_iter(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch);
}

#[parsec_test(testbed = "minimal")]
async fn related_user_already_revoked(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.revoke_user("bob");
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, alice_dev1_signed) = env.get_device_certificate("alice@dev1");
    let (_, bob_signed) = env.get_user_certificate("bob");
    let (_, bob_dev1_signed) = env.get_device_certificate("bob@dev1");
    let (_, bob_revoked_signed) = env.get_revoked_certificate("bob");

    let err = ops
        .add_certificates_batch(
            &store,
            0,
            [
                alice_signed,
                alice_dev1_signed,
                bob_signed,
                bob_dev1_signed,
                bob_revoked_signed.clone(),
                bob_revoked_signed,
            ]
            .into_iter(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        AddCertificateError::InvalidCertificate(
            InvalidCertificateError::RelatedUserAlreadyRevoked { user_revoked_on, .. }
        )
        if user_revoked_on == DateTime::from_ymd_hms_us(2000, 1, 4, 0, 0, 0, 0).unwrap()
    )
}

#[parsec_test(testbed = "minimal")]
async fn index_already_exists(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.revoke_user("bob");
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, bob_revoked_signed) = env.get_revoked_certificate("bob");

    ops.add_certificates_batch(&store, 0, [alice_signed].into_iter())
        .await
        .unwrap();

    let err = ops
        .add_certificates_batch(&store, 0, [bob_revoked_signed].into_iter())
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
        builder.new_user("bob");
        builder.revoke_user("bob").customize(|event| {
            event.timestamp = DateTime::from_ymd_hms_us(1999, 1, 1, 0, 0, 0, 0).unwrap()
        });
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, old_bob_revoked_signed) = env.get_revoked_certificate("bob");

    let err = ops
        .add_certificates_batch(
            &store,
            0,
            [alice_signed, old_bob_revoked_signed].into_iter(),
        )
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
async fn non_existing_author(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.revoke_user("bob");
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, bob_revoked_signed) = env.get_revoked_certificate("bob");

    let err = ops
        .add_certificates_batch(&store, 0, [bob_revoked_signed].into_iter())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        AddCertificateError::InvalidCertificate(InvalidCertificateError::NonExistingAuthor { .. })
    )
}

#[parsec_test(testbed = "minimal")]
async fn revoked(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.new_user("mallory");
        builder.revoke_user("bob");
        builder
            .revoke_user("mallory")
            .with_author("bob@dev1".try_into().unwrap());
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, alice_dev1_signed) = env.get_device_certificate("alice@dev1");
    let (_, bob_signed) = env.get_user_certificate("bob");
    let (_, bob_dev1_signed) = env.get_device_certificate("bob@dev1");
    let (_, bob_revoked_signed) = env.get_revoked_certificate("bob");
    let (_, mallory_revoked_signed) = env.get_revoked_certificate("mallory");

    let err = ops
        .add_certificates_batch(
            &store,
            0,
            [
                alice_signed,
                alice_dev1_signed,
                bob_signed,
                bob_dev1_signed,
                bob_revoked_signed,
                mallory_revoked_signed,
            ]
            .into_iter(),
        )
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
            .revoke_user("mallory")
            .with_author("bob@dev1".try_into().unwrap());
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, alice_dev1_signed) = env.get_device_certificate("alice@dev1");
    let (_, bob_signed) = env.get_user_certificate("bob");
    let (_, bob_dev1_signed) = env.get_device_certificate("bob@dev1");
    let (_, mallory_signed) = env.get_user_certificate("mallory");
    let (_, mallory_revoked_signed) = env.get_revoked_certificate("mallory");

    let err = ops
        .add_certificates_batch(
            &store,
            0,
            [
                alice_signed,
                alice_dev1_signed,
                bob_signed,
                bob_dev1_signed,
                mallory_signed,
                mallory_revoked_signed,
            ]
            .into_iter(),
        )
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
            .new_user("bob")
            .with_initial_profile(UserProfile::Admin);
        builder
            .revoke_user("bob")
            .with_author("bob@dev1".try_into().unwrap());
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, alice_dev1_signed) = env.get_device_certificate("alice@dev1");
    let (_, bob_signed) = env.get_user_certificate("bob");
    let (_, bob_dev1_signed) = env.get_device_certificate("bob@dev1");
    let (_, bob_revoked_signed) = env.get_revoked_certificate("bob");

    let err = ops
        .add_certificates_batch(
            &store,
            0,
            [
                alice_signed,
                alice_dev1_signed,
                bob_signed,
                bob_dev1_signed,
                bob_revoked_signed,
            ]
            .into_iter(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        AddCertificateError::InvalidCertificate(InvalidCertificateError::SelfSigned { .. })
    );
}

// Also test an admin can revoke
#[parsec_test(testbed = "minimal")]
async fn revoke_owner(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder
            .new_user("bob")
            .with_initial_profile(UserProfile::Admin);
        builder
            .revoke_user("alice")
            .with_author("bob@dev1".try_into().unwrap());
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, alice_dev1_signed) = env.get_device_certificate("alice@dev1");
    let (_, bob_signed) = env.get_user_certificate("bob");
    let (_, bob_dev1_signed) = env.get_device_certificate("bob@dev1");
    let (_, alice_revoked_certificate) = env.get_revoked_certificate("alice");

    let switch = ops
        .add_certificates_batch(
            &store,
            0,
            [
                alice_signed,
                alice_dev1_signed,
                bob_signed,
                bob_dev1_signed,
                alice_revoked_certificate,
            ]
            .into_iter(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch,);
}
