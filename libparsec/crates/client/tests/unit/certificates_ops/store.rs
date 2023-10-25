// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_platform_storage::certificates::{GetCertificateError, GetTimestampBoundsError};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::certificates_store_factory;
use crate::certificates_ops::store::CertificatesStoreReadExt;

#[parsec_test(testbed = "minimal")]
async fn add_user_certificate(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let store = certificates_store_factory(env, &alice).await;

    let write = store.for_write().await;
    let (alice_certif, alice_signed) = env.get_user_certificate("alice");
    let alice_certif = AnyArcCertificate::User(alice_certif);

    // Check that certificate is absent
    let err = write.get_any_certificate(1).await.unwrap_err();
    p_assert_matches!(err, GetCertificateError::NonExisting);

    // Add new certificate
    write
        .add_next_certificate(1, &alice_certif, &alice_signed)
        .await
        .unwrap();

    // Check that certificate is present
    let got = write.get_any_certificate(1).await.unwrap();

    p_assert_eq!(got, alice_certif);

    store.stop().await;

    // Ensure we don't rely on a cache but on data in persistent database
    let store = certificates_store_factory(env, &alice).await;
    let write = store.for_write().await;
    let got = write.get_any_certificate(1).await.unwrap();

    p_assert_eq!(got, alice_certif);
}

#[parsec_test(testbed = "minimal")]
async fn add_device_certificate(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let store = certificates_store_factory(env, &alice).await;

    let write = store.for_write().await;
    let (alice_dev1_certif, alice_dev1_signed) = env.get_device_certificate("alice@dev1");
    let alice_dev1_certif = AnyArcCertificate::Device(alice_dev1_certif);

    // Check that certificate is absent
    let err = write.get_any_certificate(1).await.unwrap_err();
    p_assert_matches!(err, GetCertificateError::NonExisting);

    // Add new certificate
    write
        .add_next_certificate(1, &alice_dev1_certif, &alice_dev1_signed)
        .await
        .unwrap();

    // Check that certificate is present
    let got = write.get_any_certificate(1).await.unwrap();

    p_assert_eq!(got, alice_dev1_certif);

    store.stop().await;

    // Ensure we don't rely on a cache but on data in persistent database
    let store = certificates_store_factory(env, &alice).await;
    let write = store.for_write().await;
    let got = write.get_any_certificate(1).await.unwrap();

    p_assert_eq!(got, alice_dev1_certif);
}

#[parsec_test(testbed = "minimal")]
async fn add_revoked_user_certificate(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.revoke_user("bob");
    });
    let alice = env.local_device("alice@dev1");
    let store = certificates_store_factory(&env, &alice).await;

    let write = store.for_write().await;
    let (bob_revoked_certif, bob_revoked_signed) = env.get_revoked_certificate("bob");
    let bob_revoked_certif = AnyArcCertificate::RevokedUser(bob_revoked_certif);

    // Check that certificate is absent
    let err = write.get_any_certificate(1).await.unwrap_err();
    p_assert_matches!(err, GetCertificateError::NonExisting);

    // Add new certificate
    write
        .add_next_certificate(1, &bob_revoked_certif, &bob_revoked_signed)
        .await
        .unwrap();

    // Check that certificate is present
    let got = write.get_any_certificate(1).await.unwrap();

    p_assert_eq!(got, bob_revoked_certif);

    store.stop().await;

    // Ensure we don't rely on a cache but on data in persistent database
    let store = certificates_store_factory(&env, &alice).await;
    let write = store.for_write().await;
    let got = write.get_any_certificate(1).await.unwrap();

    p_assert_eq!(got, bob_revoked_certif);
}

#[parsec_test(testbed = "minimal")]
async fn add_realm_role_certificate(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user_realm("alice");
    });
    let alice = env.local_device("alice@dev1");
    let store = certificates_store_factory(&env, &alice).await;

    let write = store.for_write().await;
    let (alice_realm_role_certif, alice_realm_role_signed) = env.get_last_realm_role_certificate(
        "alice",
        VlobID::from_hex("f0000000-0000-0000-0000-000000000001").unwrap(),
    );
    let alice_realm_role_certif = AnyArcCertificate::RealmRole(alice_realm_role_certif);

    // Check that certificate is absent
    let err = write.get_any_certificate(1).await.unwrap_err();
    p_assert_matches!(err, GetCertificateError::NonExisting);

    // Add new certificate
    write
        .add_next_certificate(1, &alice_realm_role_certif, &alice_realm_role_signed)
        .await
        .unwrap();

    // Check that certificate is present
    let got = write.get_any_certificate(1).await.unwrap();

    p_assert_eq!(got, alice_realm_role_certif);

    store.stop().await;

    // Ensure we don't rely on a cache but on data in persistent database
    let store = certificates_store_factory(&env, &alice).await;
    let write = store.for_write().await;
    let got = write.get_any_certificate(1).await.unwrap();

    p_assert_eq!(got, alice_realm_role_certif);
}

#[parsec_test(testbed = "minimal")]
async fn add_user_update_certificate(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.update_user_profile("bob", UserProfile::Admin);
    });
    let alice = env.local_device("alice@dev1");
    let store = certificates_store_factory(&env, &alice).await;

    let write = store.for_write().await;
    let (alice_update_certif, alice_update_signed) = env.get_last_user_update_certificate("bob");
    let alice_update_certif = AnyArcCertificate::UserUpdate(alice_update_certif);

    // Check that certificate is absent
    let err = write.get_any_certificate(1).await.unwrap_err();
    p_assert_matches!(err, GetCertificateError::NonExisting);

    // Add new certificate
    write
        .add_next_certificate(1, &alice_update_certif, &alice_update_signed)
        .await
        .unwrap();

    // Check that certificate is present
    let got = write.get_any_certificate(1).await.unwrap();

    p_assert_eq!(got, alice_update_certif);

    store.stop().await;

    // Ensure we don't rely on a cache but on data in persistent database
    let store = certificates_store_factory(&env, &alice).await;
    let write = store.for_write().await;
    let got = write.get_any_certificate(1).await.unwrap();

    p_assert_eq!(got, alice_update_certif);
}

#[parsec_test(testbed = "empty")]
async fn add_sequester_authority_certificate(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder
            .bootstrap_organization("alice")
            .and_set_sequestered_organization();
    });
    let alice = env.local_device("alice@dev1");
    let store = certificates_store_factory(&env, &alice).await;

    let write = store.for_write().await;

    let (authority_certif, authority_signed) = env.get_sequester_authority_certificate();
    let authority_certif = AnyArcCertificate::SequesterAuthority(authority_certif);

    // Check that certificate is absent
    let err = write.get_any_certificate(1).await.unwrap_err();
    p_assert_matches!(err, GetCertificateError::NonExisting);

    // Add new certificate
    write
        .add_next_certificate(1, &authority_certif, &authority_signed)
        .await
        .unwrap();

    // Check that certificate is present
    let got = write.get_any_certificate(1).await.unwrap();

    p_assert_eq!(got, authority_certif);

    store.stop().await;

    // Ensure we don't rely on a cache but on data in persistent database
    let store = certificates_store_factory(&env, &alice).await;
    let write = store.for_write().await;
    let got = write.get_any_certificate(1).await.unwrap();

    p_assert_eq!(got, authority_certif);
}

#[parsec_test(testbed = "empty")]
async fn add_sequester_service_certificate(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder
            .bootstrap_organization("alice")
            .and_set_sequestered_organization();
        builder.new_sequester_service();
    });
    let alice = env.local_device("alice@dev1");
    let store = certificates_store_factory(&env, &alice).await;

    let write = store.for_write().await;

    let (service_certif, service_signed) = env.get_sequester_service_certificate(
        SequesterServiceID::from_hex("00000000-0000-0000-0000-000000000001").unwrap(),
    );
    let service_certif = AnyArcCertificate::SequesterService(service_certif);

    // Check that certificate is absent
    let err = write.get_any_certificate(1).await.unwrap_err();
    p_assert_matches!(err, GetCertificateError::NonExisting);

    // Add new certificate
    write
        .add_next_certificate(1, &service_certif, &service_signed)
        .await
        .unwrap();

    // Check that certificate is present
    let got = write.get_any_certificate(1).await.unwrap();

    p_assert_eq!(got, service_certif);

    store.stop().await;

    // Ensure we don't rely on a cache but on data in persistent database
    let store = certificates_store_factory(&env, &alice).await;
    let write = store.for_write().await;
    let got = write.get_any_certificate(1).await.unwrap();

    p_assert_eq!(got, service_certif);
}

#[parsec_test(testbed = "minimal")]
async fn add_same_index_fail(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let store = certificates_store_factory(env, &alice).await;

    let write = store.for_write().await;

    let (alice_certif, alice_signed) = env.get_user_certificate("alice");
    let alice_certif = AnyArcCertificate::User(alice_certif);

    write
        .add_next_certificate(1, &alice_certif, &alice_signed)
        .await
        .unwrap();

    write
        .add_next_certificate(1, &alice_certif, &alice_signed)
        .await
        .unwrap_err();

    store.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn add_skip_index_fail(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let store = certificates_store_factory(env, &alice).await;

    let write = store.for_write().await;

    let (alice_certif, alice_signed) = env.get_user_certificate("alice");
    let alice_certif = AnyArcCertificate::User(alice_certif);

    // Add new certificate
    write
        .add_next_certificate(42, &alice_certif, &alice_signed)
        .await
        .unwrap_err();

    store.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn forget_certificates(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let store = certificates_store_factory(env, &alice).await;

    let write = store.for_write().await;

    let (alice_certif, alice_signed) = env.get_user_certificate("alice");
    let alice_certif = AnyArcCertificate::User(alice_certif);

    let (alice_dev1_certif, alice_dev1_signed) = env.get_device_certificate("alice@dev1");
    let alice_dev1_certif = AnyArcCertificate::Device(alice_dev1_certif);

    write
        .add_next_certificate(1, &alice_certif, &alice_signed)
        .await
        .unwrap();

    write
        .add_next_certificate(2, &alice_dev1_certif, &alice_dev1_signed)
        .await
        .unwrap();

    // Check that certificates are now present
    let got = write.get_any_certificate(1).await.unwrap();

    p_assert_eq!(got, alice_certif);

    let got = write.get_any_certificate(2).await.unwrap();

    p_assert_eq!(got, alice_dev1_certif);

    write.forget_all_certificates().await.unwrap();

    // Check that certificates are now absent
    let err = write.get_any_certificate(1).await.unwrap_err();
    p_assert_matches!(err, GetCertificateError::NonExisting);

    let err = write.get_any_certificate(2).await.unwrap_err();
    p_assert_matches!(err, GetCertificateError::NonExisting);

    store.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn get_last_certificate_index(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let store = certificates_store_factory(env, &alice).await;

    let write = store.for_write().await;

    let (alice_certif, alice_signed) = env.get_user_certificate("alice");
    let alice_certif = AnyArcCertificate::User(alice_certif);

    // Check that certificates are now present
    let index = write.get_last_certificate_index().await.unwrap();

    p_assert_eq!(index, 0);

    write
        .add_next_certificate(1, &alice_certif, &alice_signed)
        .await
        .unwrap();

    write
        .add_next_certificate(2, &alice_certif, &alice_signed)
        .await
        .unwrap();

    let index = write.get_last_certificate_index().await.unwrap();

    p_assert_eq!(index, 2);

    store.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn get_timestamp_bounds(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let store = certificates_store_factory(env, &alice).await;

    let write = store.for_write().await;

    let (alice_certif, alice_signed) = env.get_user_certificate("alice");
    let timestamp = alice_certif.timestamp;
    let alice_certif = AnyArcCertificate::User(alice_certif);

    write
        .add_next_certificate(1, &alice_certif, &alice_signed)
        .await
        .unwrap();

    let (start, end) = write.get_timestamp_bounds(1).await.unwrap();

    p_assert_eq!(start, timestamp);
    // Certificate index is the last one, hence it has no upper bound
    p_assert_eq!(end, None);

    write
        .add_next_certificate(2, &alice_certif, &alice_signed)
        .await
        .unwrap();

    let (start, end) = write.get_timestamp_bounds(1).await.unwrap();

    p_assert_eq!(start, timestamp);
    p_assert_eq!(end, Some(timestamp));

    let err = write.get_timestamp_bounds(42).await.unwrap_err();

    p_assert_matches!(err, GetTimestampBoundsError::NonExisting);

    store.stop().await;
}
