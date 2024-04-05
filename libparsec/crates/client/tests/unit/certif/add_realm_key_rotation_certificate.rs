// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certif::{
    CertifAddCertificatesBatchError, InvalidCertificateError, MaybeRedactedSwitch,
};

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_realm("alice").then_do_initial_key_rotation();
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let switch = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &env.get_realms_certificates_signed(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch);
}

#[parsec_test(testbed = "minimal")]
async fn multiple(env: &TestbedEnv) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);
        builder.certificates_storage_fetch_certificates("alice@dev1");
        builder.rotate_key_realm(realm_id);
        realm_id
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;
    let realm_certificates = &env.get_realms_certificates_signed()[&realm_id];

    let switch = ops
        .add_certificates_batch(
            &[],
            &[],
            &[],
            &HashMap::from([(
                realm_id,
                realm_certificates[realm_certificates.len() - 1..].to_vec(),
            )]),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch);
}

#[parsec_test(testbed = "minimal")]
async fn older_than_author(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .customize(|event| {
                event.timestamp = DateTime::from_ymd_hms_us(1999, 1, 1, 0, 0, 0, 0).unwrap()
            });
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &env.get_realms_certificates_signed(),
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
    let (env, timestamp) = env.customize_with_map(|builder| {
        let (realm_id, timestamp) = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| (event.realm, event.timestamp));

        builder
            .rotate_key_realm(realm_id)
            .customize(|event| event.timestamp = timestamp);

        timestamp
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &env.get_realms_certificates_signed(),
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
async fn not_member(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        let bob = builder.new_user("bob").map(|event| event.device_id.clone());
        builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .customize(|event| event.author = bob);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &env.get_realms_certificates_signed(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(*boxed, InvalidCertificateError::RealmAuthorHasNoRole { .. })
    );
}

#[parsec_test(testbed = "minimal")]
async fn author_not_owner(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        let bob = builder.new_user("bob").map(|event| event.device_id.clone());
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);

        builder.share_realm(realm_id, "bob", Some(RealmRole::Contributor));

        builder
            .rotate_key_realm(realm_id)
            .customize(|event| event.author = bob);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &env.get_realms_certificates_signed(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(*boxed, InvalidCertificateError::RealmAuthorNotOwner { .. })
    );
}

#[parsec_test(testbed = "minimal")]
async fn revoked(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        let bob = builder.new_user("bob").map(|event| event.device_id.clone());
        let realm_id = builder
            .new_realm("bob")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);

        builder
            .share_realm(realm_id, "alice", Some(RealmRole::Owner))
            .customize(|event| event.author = bob.clone());

        builder.revoke_user("bob");

        builder
            .rotate_key_realm(realm_id)
            .customize(|event| event.author = bob);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &env.get_realms_certificates_signed(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(*boxed, InvalidCertificateError::RevokedAuthor { .. })
    );
}
