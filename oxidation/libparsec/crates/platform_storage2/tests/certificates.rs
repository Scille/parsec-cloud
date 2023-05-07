// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{ops::Deref, sync::Arc};

use libparsec_tests_fixtures::prelude::*;

use libparsec_platform_storage2::certificates::CertificatesStorage;

#[parsec_test(testbed = "minimal")]
async fn user_certificate(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1".parse().unwrap());
    let alice_data = env.template.user(&"alice".parse().unwrap());

    // Start and populate the storage

    let certif_storage = CertificatesStorage::start(&env.discriminant_dir, alice.clone())
        .await
        .unwrap();
    certif_storage
        .add_new_user_certificate(
            alice_data.certif.clone().into(),
            alice_data.raw_certif.clone(),
        )
        .await
        .unwrap();

    // Simple access

    let fetched = certif_storage
        .get_user_certificate(alice.user_id())
        .await
        .unwrap()
        .unwrap();
    p_assert_eq!(fetched.deref(), &alice_data.certif);

    // Check that cache is working

    let fetched2 = certif_storage
        .get_user_certificate(alice.user_id())
        .await
        .unwrap()
        .unwrap();
    p_assert_eq!(Arc::as_ptr(&fetched), Arc::as_ptr(&fetched2));

    // Restart the storage and retry to get the certificate

    certif_storage.stop().await;

    let certif_storage_restarted = CertificatesStorage::start(&env.discriminant_dir, alice.clone())
        .await
        .unwrap();

    let fetched_restarted = certif_storage_restarted
        .get_user_certificate(alice.user_id())
        .await
        .unwrap()
        .unwrap();
    p_assert_eq!(fetched_restarted, fetched);
    p_assert_ne!(Arc::as_ptr(&fetched_restarted), Arc::as_ptr(&fetched));

    certif_storage_restarted.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn get_last_certificate_timestamp(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1".parse().unwrap());
    let alice_data = env.template.user(&"alice".parse().unwrap());

    let certif_storage = CertificatesStorage::start(&env.discriminant_dir, alice.clone())
        .await
        .unwrap();

    // Try with no certificates in the database...

    p_assert_eq!(
        certif_storage
            .get_last_certificate_timestamp()
            .await
            .unwrap(),
        None
    );

    // Now with a certificate

    certif_storage
        .add_new_user_certificate(
            alice_data.certif.clone().into(),
            alice_data.raw_certif.clone(),
        )
        .await
        .unwrap();

    p_assert_eq!(
        certif_storage
            .get_last_certificate_timestamp()
            .await
            .unwrap(),
        Some(alice_data.certif.timestamp)
    );
}
