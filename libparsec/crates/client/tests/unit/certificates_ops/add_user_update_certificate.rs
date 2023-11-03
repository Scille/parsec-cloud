// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certificates_ops::{AddCertificateError, InvalidCertificateError, MaybeRedactedSwitch};

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.update_user_profile("bob", UserProfile::Admin);
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

#[parsec_test(testbed = "minimal")]
async fn content_already_exists(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.update_user_profile("bob", UserProfile::Admin);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, alice_dev1_signed) = env.get_device_certificate("alice@dev1");
    let (_, bob_signed) = env.get_user_certificate("bob");
    let (_, bob_user_update_signed) = env.get_last_user_update_certificate("bob");

    let err = ops
        .add_certificates_batch(
            &store,
            0,
            [
                alice_signed,
                alice_dev1_signed,
                bob_signed,
                bob_user_update_signed.clone(),
                bob_user_update_signed,
            ]
            .into_iter(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        AddCertificateError::InvalidCertificate(
            InvalidCertificateError::ContentAlreadyExists { .. }
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn index_already_exists(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.update_user_profile("bob", UserProfile::Admin);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, bob_user_update_signed) = env.get_last_user_update_certificate("bob");

    ops.add_certificates_batch(&store, 0, [alice_signed].into_iter())
        .await
        .unwrap();

    let err = ops
        .add_certificates_batch(&store, 0, [bob_user_update_signed].into_iter())
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
        builder
            .update_user_profile("bob", UserProfile::Admin)
            .customize(|event| {
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
        if last_certificate_timestamp == DateTime::from_ymd_hms_us(2000, 1, 3, 0, 0, 0, 0).unwrap()
    );
}

#[parsec_test(testbed = "minimal")]
async fn non_existing_author(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.update_user_profile("bob", UserProfile::Standard);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, bob_user_update_signed) = env.get_last_user_update_certificate("bob");

    let err = ops
        .add_certificates_batch(&store, 0, [bob_user_update_signed].into_iter())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        AddCertificateError::InvalidCertificate(InvalidCertificateError::NonExistingAuthor { .. })
    );
}

#[parsec_test(testbed = "minimal")]
async fn same_profile(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.update_user_profile("bob", UserProfile::Standard);
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
        AddCertificateError::InvalidCertificate(
            InvalidCertificateError::ContentAlreadyExists { .. }
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn owner_of_realm_not_shared_to_outsider_is_ok(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.new_user_realm("bob");
        builder.update_user_profile("bob", UserProfile::Outsider);
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

#[parsec_test(testbed = "minimal")]
async fn owner_of_realm_shared_to_outsider_error(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder
            .new_user_realm("bob")
            .then_share_with("alice", Some(RealmRole::Contributor));
        builder.update_user_profile("bob", UserProfile::Outsider);
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
        AddCertificateError::InvalidCertificate(
            InvalidCertificateError::CannotDowngradeUserToOutsider { .. }
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn manager_of_realm_to_outsider_error(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder
            .new_user_realm("alice")
            .then_share_with("bob", Some(RealmRole::Manager));
        builder.update_user_profile("bob", UserProfile::Outsider);
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
        AddCertificateError::InvalidCertificate(
            InvalidCertificateError::CannotDowngradeUserToOutsider { .. }
        )
    );
}

#[parsec_test(testbed = "minimal")]
#[case(RealmRole::Contributor)]
#[case(RealmRole::Reader)]
async fn contributor_of_realm_to_outsider_is_ok(#[case] role: RealmRole, env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder
            .new_user_realm("alice")
            .then_share_with("bob", Some(role));
        builder.update_user_profile("bob", UserProfile::Outsider);
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

#[parsec_test(testbed = "minimal")]
async fn revoked(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder
            .new_user("bob")
            .with_initial_profile(UserProfile::Admin);
        builder.new_user("mallory");
        builder.revoke_user("bob");
        builder
            .update_user_profile("mallory", UserProfile::Admin)
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
        AddCertificateError::InvalidCertificate(
            InvalidCertificateError::RevokedAuthor { author_revoked_on, .. }
        )
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
            .update_user_profile("mallory", UserProfile::Admin)
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
        builder.new_user("bob");
        builder
            .update_user_profile("bob", UserProfile::Admin)
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
        AddCertificateError::InvalidCertificate(InvalidCertificateError::SelfSigned { .. })
    );
}
