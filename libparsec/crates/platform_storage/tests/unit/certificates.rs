// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;

use crate::certificates::CertificatesStorageUpdater;

use super::{
    CertificatesStorage, GetCertificateError, GetCertificateQuery, PerTopicLastTimestamps, UpTo,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

#[cfg(target_arch = "wasm32")]
libparsec_tests_lite::platform::wasm_bindgen_test_configure!(run_in_browser);

enum FetchStrategy {
    Single,
    Multiple,
}

#[parsec_test(testbed = "empty")]
async fn testbed_support(
    #[values(FetchStrategy::Single, FetchStrategy::Multiple)] fetch_strategy: FetchStrategy,
    #[values(false, true)] sequestered: bool,
    env: &TestbedEnv,
) {
    let mut expected_last_timestamps = PerTopicLastTimestamps {
        common: None,
        sequester: None,
        shamir_recovery: None,
        realm: HashMap::default(),
    };

    env.customize(|builder| {
        if sequestered {
            expected_last_timestamps.sequester = builder
                .bootstrap_organization("alice")
                .and_set_sequestered_organization()
                .map(|e| Some(e.timestamp));
        } else {
            builder.bootstrap_organization("alice");
        }

        expected_last_timestamps.common = builder.new_user("bob").map(|e| Some(e.timestamp));
        builder.new_realm("bob").map(|e| {
            expected_last_timestamps
                .realm
                .insert(e.realm_id, e.timestamp);
        });

        builder.certificates_storage_fetch_certificates("alice@dev1");

        if matches!(fetch_strategy, FetchStrategy::Multiple) {
            // Only the last fetch is taken into account, so this should be known
            let alice2_id = builder.new_device("alice").map(|e| e.device_id);
            expected_last_timestamps.common = builder.new_user("mike").map(|e| Some(e.timestamp));
            builder.new_realm("alice").map(|e| {
                expected_last_timestamps
                    .realm
                    .insert(e.realm_id, e.timestamp);
            });
            expected_last_timestamps.shamir_recovery = builder
                .new_shamir_recovery(
                    "alice",
                    1,
                    [("bob".parse().unwrap(), 1.try_into().unwrap())],
                    alice2_id,
                )
                .map(|e| Some(e.timestamp));
            if sequestered {
                expected_last_timestamps.sequester =
                    builder.new_sequester_service().map(|e| Some(e.timestamp));
            }
            builder.certificates_storage_fetch_certificates("alice@dev1");
        }

        // Stuff our storage is not aware of
        builder.new_realm("alice");
        let bob2_id = builder.new_device("bob").map(|e| e.device_id);
        builder.new_user("philip");
        builder.new_shamir_recovery(
            "bob",
            1,
            [("alice".parse().unwrap(), 1.try_into().unwrap())],
            bob2_id,
        );
        if sequestered {
            builder.new_sequester_service();
        }
    })
    .await;

    let alice = env.local_device("alice@dev1");

    let mut storage = CertificatesStorage::start(&env.discriminant_dir, &alice)
        .await
        .unwrap();

    p_assert_eq!(
        storage.get_last_timestamps().await.unwrap(),
        expected_last_timestamps
    );
}

#[parsec_test(testbed = "minimal")]
async fn for_update_preserves_task_id(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let mut storage = CertificatesStorage::start(&env.discriminant_dir, &alice)
        .await
        .unwrap();

    let task = libparsec_platform_async::spawn(async move {
        let task_id_from_task = libparsec_platform_async::try_task_id();
        let task_id_from_for_update = storage
            .for_update::<_, ()>(async |_| Ok(libparsec_platform_async::try_task_id()))
            .await
            .unwrap()
            .unwrap();
        (task_id_from_task, task_id_from_for_update)
    });
    let task_id = task.id();
    let (task_id_from_task, task_id_from_for_update) = task.await.unwrap();

    p_assert_eq!(task_id_from_task.unwrap(), task_id);
    p_assert_eq!(task_id_from_for_update.unwrap(), task_id);
}

#[parsec_test(testbed = "minimal")]
async fn get_last_timestamps(mut timestamps: TimestampGenerator, env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let mut storage = CertificatesStorage::start(&env.discriminant_dir, &alice)
        .await
        .unwrap();

    let mut expected = PerTopicLastTimestamps {
        common: None,
        sequester: None,
        realm: HashMap::new(),
        shamir_recovery: None,
    };

    storage
        .for_update(async |mut updater| {
            // Try with no certificates in the database...

            p_assert_eq!(updater.get_last_timestamps().await.unwrap(), expected);

            // Now with certificates

            let device_id = alice.device_id;
            let user_id = alice.user_id;
            let realm_id = VlobID::from_hex("fe15ca8ad55140d08b4951f26f7073d5").unwrap();
            let service_id =
                SequesterServiceID::from_hex("a065170f3b6649f997ec14dbe36c5c13").unwrap();

            let t1 = timestamps.next();
            updater
                .add_certificate(
                    &UserCertificate {
                        timestamp: t1,
                        user_id,
                        // Not meaningful for the test
                        author: CertificateSigner::Root,
                        human_handle: MaybeRedacted::Redacted(HumanHandle::new_redacted(user_id)),
                        public_key: alice.public_key(),
                        algorithm: PrivateKeyAlgorithm::X25519XSalsa20Poly1305,
                        profile: UserProfile::Admin,
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            expected.common = Some(t1);

            p_assert_eq!(updater.get_last_timestamps().await.unwrap(), expected);

            let t2 = timestamps.next();
            updater
                .add_certificate(
                    &RevokedUserCertificate {
                        timestamp: t2,
                        user_id,
                        // Not meaningful for the test
                        author: DeviceID::default(),
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            expected.common = Some(t2);

            p_assert_eq!(updater.get_last_timestamps().await.unwrap(), expected);

            let t3 = timestamps.next();
            updater
                .add_certificate(
                    &UserUpdateCertificate {
                        timestamp: t3,
                        user_id,
                        // Not meaningful for the test
                        author: DeviceID::default(),
                        new_profile: UserProfile::Admin,
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            expected.common = Some(t3);

            p_assert_eq!(updater.get_last_timestamps().await.unwrap(), expected);

            let t4 = timestamps.next();
            updater
                .add_certificate(
                    &DeviceCertificate {
                        timestamp: t4,
                        user_id,
                        device_id,
                        purpose: DevicePurpose::Standard,
                        // Not meaningful for the test
                        author: CertificateSigner::Root,
                        device_label: MaybeRedacted::Redacted(DeviceLabel::new_redacted(device_id)),
                        verify_key: alice.verify_key(),
                        algorithm: SigningKeyAlgorithm::Ed25519,
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            expected.common = Some(t4);

            p_assert_eq!(updater.get_last_timestamps().await.unwrap(), expected);

            let t5 = timestamps.next();
            updater
                .add_certificate(
                    &RealmRoleCertificate {
                        timestamp: t5,
                        realm_id,
                        user_id,
                        // Not meaningful for the test
                        author: device_id,
                        role: None,
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            expected.realm.insert(realm_id, t5);

            p_assert_eq!(updater.get_last_timestamps().await.unwrap(), expected);

            let t6 = timestamps.next();
            updater
                .add_certificate(
                    &SequesterAuthorityCertificate {
                        timestamp: t6,
                        // Not meaningful for the test
                        verify_key_der: SequesterSigningKeyDer::generate_pair(
                            SequesterKeySize::_1024Bits,
                        )
                        .1,
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            expected.sequester = Some(t6);

            p_assert_eq!(updater.get_last_timestamps().await.unwrap(), expected);

            let t7 = timestamps.next();
            updater
                .add_certificate(
                    &SequesterServiceCertificate {
                        timestamp: t7,
                        service_id,
                        // Not meaningful for the test
                        encryption_key_der: SequesterPrivateKeyDer::generate_pair(
                            SequesterKeySize::_1024Bits,
                        )
                        .1,
                        service_label: "service_label".to_string(),
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            expected.sequester = Some(t7);

            p_assert_eq!(updater.get_last_timestamps().await.unwrap(), expected);

            let t8 = timestamps.next();
            updater
                .add_certificate(
                    &RealmKeyRotationCertificate {
                        timestamp: t8,
                        realm_id,
                        key_index: 0,
                        // Not meaningful for the test
                        author: device_id,
                        encryption_algorithm: SecretKeyAlgorithm::Blake2bXsalsa20Poly1305,
                        hash_algorithm: HashAlgorithm::Sha256,
                        key_canary: b"key_canary".to_vec(),
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            expected.realm.insert(realm_id, t8);

            p_assert_eq!(updater.get_last_timestamps().await.unwrap(), expected);

            let t9 = timestamps.next();
            updater
                .add_certificate(
                    &RealmNameCertificate {
                        timestamp: t9,
                        realm_id,
                        // Not meaningful for the test
                        key_index: 0,
                        author: device_id,
                        encrypted_name: b"encrypted_name".to_vec(),
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            expected.realm.insert(realm_id, t9);

            p_assert_eq!(updater.get_last_timestamps().await.unwrap(), expected);

            let t10 = timestamps.next();
            updater
                .add_certificate(
                    &RealmArchivingCertificate {
                        timestamp: t10,
                        realm_id,
                        // Not meaningful for the test
                        author: device_id,
                        configuration: RealmArchivingConfiguration::Archived,
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            expected.realm.insert(realm_id, t10);

            p_assert_eq!(updater.get_last_timestamps().await.unwrap(), expected);

            let t11 = timestamps.next();
            updater
                .add_certificate(
                    &SequesterRevokedServiceCertificate {
                        timestamp: t11,
                        service_id,
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            expected.sequester = Some(t11);

            p_assert_eq!(updater.get_last_timestamps().await.unwrap(), expected);

            let t12 = timestamps.next();
            updater
                .add_certificate(
                    &ShamirRecoveryBriefCertificate {
                        timestamp: t12,
                        author: device_id,
                        user_id,
                        // Not meaningful for the test
                        per_recipient_shares: HashMap::new(),
                        threshold: 1.try_into().unwrap(),
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            expected.shamir_recovery = Some(t12);

            p_assert_eq!(updater.get_last_timestamps().await.unwrap(), expected);

            let t13 = timestamps.next();
            updater
                .add_certificate(
                    &ShamirRecoveryShareCertificate {
                        timestamp: t13,
                        author: device_id,
                        user_id,
                        recipient: user_id,
                        // Not meaningful for the test
                        ciphered_share: b"ciphered_share".to_vec(),
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            expected.shamir_recovery = Some(t13);

            p_assert_eq!(updater.get_last_timestamps().await.unwrap(), expected);

            Result::<(), ()>::Ok(())
        })
        .await
        .unwrap()
        .unwrap();

    p_assert_eq!(storage.get_last_timestamps().await.unwrap(), expected);
}

// This test is horribly long and complex, but the idea is to test all certificates types
// in a single database to ensure they don't step on each other.
// /!\ Every time we add a new certificate type, we should test it here.
#[parsec_test(testbed = "minimal")]
async fn add_and_get_certificate(
    #[values("commit", "rollback")] kind: &str,
    mut timestamps: TimestampGenerator,
    env: &TestbedEnv,
) {
    // Note the testbed env is only used as a convenient way to build a local device
    let alice = env.local_device("alice@dev1");

    let user_id: UserID = "alice".parse().unwrap();
    let other_user_id: UserID = "bob".parse().unwrap();
    let device_id: DeviceID = "alice@dev1".parse().unwrap();
    let other_device_id: DeviceID = "bob@dev1".parse().unwrap();
    let realm_id = VlobID::from_hex("fe15ca8ad55140d08b4951f26f7073d5").unwrap();
    let other_realm_id = VlobID::from_hex("810d8d137b934985b62627042058aee4").unwrap();
    let service_id1 = SequesterServiceID::from_hex("a065170f3b6649f997ec14dbe36c5c13").unwrap();
    let service_id2 = SequesterServiceID::from_hex("fc93467e47ec48a2b654fe8bf28fe7ea").unwrap();

    // Timestamps generator starts at "2020-01-01T00:00:00Z" and increments by 1ms each time
    let t1 = timestamps.next();
    let t2 = timestamps.next();
    let t3 = timestamps.next();
    let t4 = timestamps.next();
    let t5 = timestamps.next();
    let t6 = timestamps.next();
    let t7 = timestamps.next();
    let t8 = timestamps.next();
    let t9 = timestamps.next();
    let t10 = timestamps.next();
    let t11 = timestamps.next();
    let t12 = timestamps.next();
    let t13 = timestamps.next();
    let t14 = timestamps.next();
    let t15 = timestamps.next();
    let t16 = timestamps.next();
    let t17 = timestamps.next();
    let t18 = timestamps.next();
    let t19 = timestamps.next();
    let t20 = timestamps.next();
    let t21 = timestamps.next();
    let t22 = timestamps.next();
    let t23 = timestamps.next();
    let t24 = timestamps.next();
    let t25 = timestamps.next();

    let mut storage = CertificatesStorage::start(&env.discriminant_dir, &alice)
        .await
        .unwrap();

    /*
     * Assert empty database macro
     */

    macro_rules! assert_not_in_db {
        // `$db` can be either `storage` or `updater`
        ($db: ident, $query_meth:ident $(,$query_args:expr)*) => {
            async {
                let query = GetCertificateQuery::$query_meth($($query_args, )*);

                let outcome = $db.get_certificate_encrypted(query.clone(), UpTo::Current).await;
                p_assert_matches!(outcome, Err(GetCertificateError::NonExisting));

                let items = $db.get_multiple_certificates_encrypted(query, UpTo::Current, None, None).await.unwrap();
                assert!(items.is_empty());
            }
        };
    }

    macro_rules! assert_empty_db {
        // `$db` can be either `storage` or `updater`
        ($db: ident) => {
            async {
                assert_not_in_db!($db, user_certificate, &user_id).await;
                assert_not_in_db!($db, user_certificate_from_device_id, &device_id).await;
                assert_not_in_db!($db, revoked_user_certificate, &user_id).await;
                assert_not_in_db!($db, revoked_user_certificate_from_device_id, &device_id).await;
                assert_not_in_db!($db, user_update_certificates, &user_id).await;
                assert_not_in_db!($db, user_update_certificates_from_device_id, &device_id).await;
                assert_not_in_db!($db, device_certificate, &device_id).await;
                assert_not_in_db!($db, user_devices_certificates, &user_id).await;
                assert_not_in_db!($db, realm_role_certificates, &realm_id).await;
                assert_not_in_db!($db, realm_name_certificates, &realm_id).await;
                assert_not_in_db!($db, realm_key_rotation_certificate, &realm_id, 1).await;
                assert_not_in_db!($db, realm_key_rotation_certificates, &realm_id).await;
                assert_not_in_db!($db, realm_archiving_certificates, &realm_id).await;
                assert_not_in_db!($db, user_realm_role_certificates, &user_id).await;
                assert_not_in_db!($db, shamir_recovery_brief_certificates).await;
                assert_not_in_db!($db, shamir_recovery_share_certificates).await;
                assert_not_in_db!($db, sequester_authority_certificate).await;
                assert_not_in_db!($db, sequester_service_certificates).await;
                assert_not_in_db!($db, sequester_service_certificate, &service_id1).await;
            }
        };
    }

    /*
     * Populate database macro
     */

    macro_rules! populate_db {
        // `$db` can be either `storage` or `updater`
        ($db: ident) => {
            async {
                $db.add_certificate(
                    &UserCertificate {
                        timestamp: t1,
                        user_id,
                        // Not meaningful for the test
                        author: CertificateSigner::Root,
                        human_handle: MaybeRedacted::Redacted(HumanHandle::new_redacted(user_id)),
                        public_key: alice.public_key(),
                        algorithm: PrivateKeyAlgorithm::X25519XSalsa20Poly1305,
                        profile: UserProfile::Admin,
                    },
                    b"user".to_vec(),
                )
                .await
                .unwrap();

                $db.add_certificate(
                    &RevokedUserCertificate {
                        timestamp: t2,
                        user_id,
                        // Not meaningful for the test
                        author: DeviceID::default(),
                    },
                    b"revoked_user".to_vec(),
                )
                .await
                .unwrap();

                $db.add_certificate(
                    &UserUpdateCertificate {
                        timestamp: t3,
                        user_id,
                        // Not meaningful for the test
                        author: DeviceID::default(),
                        new_profile: UserProfile::Admin,
                    },
                    b"user_update1".to_vec(),
                )
                .await
                .unwrap();

                $db.add_certificate(
                    &UserUpdateCertificate {
                        timestamp: t4,
                        user_id,
                        // Not meaningful for the test
                        author: DeviceID::default(),
                        new_profile: UserProfile::Admin,
                    },
                    b"user_update2".to_vec(),
                )
                .await
                .unwrap();

                $db.add_certificate(
                    &UserUpdateCertificate {
                        timestamp: t5,
                        user_id: other_user_id,
                        // Not meaningful for the test
                        author: DeviceID::default(),
                        new_profile: UserProfile::Admin,
                    },
                    b"other_user_update".to_vec(),
                )
                .await
                .unwrap();

                $db.add_certificate(
                    &DeviceCertificate {
                        timestamp: t6,
                        device_id,
                        purpose: DevicePurpose::Standard,
                        // Not meaningful for the test
                        author: CertificateSigner::Root,
                        user_id,
                        device_label: MaybeRedacted::Redacted(DeviceLabel::new_redacted(device_id)),
                        verify_key: alice.verify_key(),
                        algorithm: SigningKeyAlgorithm::Ed25519,
                    },
                    b"device".to_vec(),
                )
                .await
                .unwrap();

                $db.add_certificate(
                    &RealmRoleCertificate {
                        timestamp: t7,
                        realm_id,
                        user_id,
                        // Not meaningful for the test
                        author: device_id,
                        role: None,
                    },
                    b"realm_role1".to_vec(),
                )
                .await
                .unwrap();

                $db.add_certificate(
                    &RealmRoleCertificate {
                        timestamp: t8,
                        realm_id,
                        user_id,
                        // Not meaningful for the test
                        author: device_id,
                        role: None,
                    },
                    b"realm_role2".to_vec(),
                )
                .await
                .unwrap();

                $db.add_certificate(
                    &RealmRoleCertificate {
                        timestamp: t9,
                        realm_id: other_realm_id,
                        user_id,
                        // Not meaningful for the test
                        author: device_id,
                        role: None,
                    },
                    b"other_realm_role".to_vec(),
                )
                .await
                .unwrap();

                $db.add_certificate(
                    &RealmRoleCertificate {
                        timestamp: t10,
                        realm_id,
                        user_id: other_user_id,
                        // Not meaningful for the test
                        author: device_id,
                        role: None,
                    },
                    b"realm_role_other_user".to_vec(),
                )
                .await
                .unwrap();

                $db.add_certificate(
                    &SequesterAuthorityCertificate {
                        timestamp: t11,
                        // Not meaningful for the test
                        verify_key_der: SequesterSigningKeyDer::generate_pair(
                            SequesterKeySize::_1024Bits,
                        )
                        .1,
                    },
                    b"sequester_authority".to_vec(),
                )
                .await
                .unwrap();

                $db.add_certificate(
                    &SequesterServiceCertificate {
                        timestamp: t12,
                        service_id: service_id1,
                        // Not meaningful for the test
                        encryption_key_der: SequesterPrivateKeyDer::generate_pair(
                            SequesterKeySize::_1024Bits,
                        )
                        .1,
                        service_label: "service_label".to_string(),
                    },
                    b"sequester_service1".to_vec(),
                )
                .await
                .unwrap();

                $db.add_certificate(
                    &SequesterServiceCertificate {
                        timestamp: t13,
                        service_id: service_id2,
                        // Not meaningful for the test
                        encryption_key_der: SequesterPrivateKeyDer::generate_pair(
                            SequesterKeySize::_1024Bits,
                        )
                        .1,
                        service_label: "service_label".to_string(),
                    },
                    b"sequester_service2".to_vec(),
                )
                .await
                .unwrap();

                $db.add_certificate(
                    &RealmKeyRotationCertificate {
                        timestamp: t14,
                        realm_id,
                        key_index: 0,
                        // Not meaningful for the test
                        author: device_id,
                        encryption_algorithm: SecretKeyAlgorithm::Blake2bXsalsa20Poly1305,
                        hash_algorithm: HashAlgorithm::Sha256,
                        key_canary: b"key_canary".to_vec(),
                    },
                    b"realm_key_rotation1".to_vec(),
                )
                .await
                .unwrap();

                $db.add_certificate(
                    &RealmKeyRotationCertificate {
                        timestamp: t15,
                        realm_id,
                        key_index: 1,
                        // Not meaningful for the test
                        author: device_id,
                        encryption_algorithm: SecretKeyAlgorithm::Blake2bXsalsa20Poly1305,
                        hash_algorithm: HashAlgorithm::Sha256,
                        key_canary: b"key_canary".to_vec(),
                    },
                    b"realm_key_rotation2".to_vec(),
                )
                .await
                .unwrap();

                $db.add_certificate(
                    &RealmNameCertificate {
                        timestamp: t16,
                        realm_id,
                        // Not meaningful for the test
                        key_index: 0,
                        author: device_id,
                        encrypted_name: b"encrypted_name".to_vec(),
                    },
                    b"realm_name1".to_vec(),
                )
                .await
                .unwrap();

                $db.add_certificate(
                    &RealmNameCertificate {
                        timestamp: t17,
                        realm_id,
                        // Not meaningful for the test
                        key_index: 0,
                        author: device_id,
                        encrypted_name: b"encrypted_name".to_vec(),
                    },
                    b"realm_name2".to_vec(),
                )
                .await
                .unwrap();

                $db.add_certificate(
                    &RealmArchivingCertificate {
                        timestamp: t18,
                        realm_id,
                        // Not meaningful for the test
                        author: device_id,
                        configuration: RealmArchivingConfiguration::Archived,
                    },
                    b"realm_archiving1".to_vec(),
                )
                .await
                .unwrap();

                $db.add_certificate(
                    &RealmArchivingCertificate {
                        timestamp: t19,
                        realm_id,
                        // Not meaningful for the test
                        author: device_id,
                        configuration: RealmArchivingConfiguration::Archived,
                    },
                    b"realm_archiving2".to_vec(),
                )
                .await
                .unwrap();

                $db.add_certificate(
                    &SequesterRevokedServiceCertificate {
                        timestamp: t20,
                        service_id: service_id1,
                    },
                    b"sequester_revoked1".to_vec(),
                )
                .await
                .unwrap();

                $db.add_certificate(
                    &SequesterRevokedServiceCertificate {
                        timestamp: t21,
                        service_id: service_id2,
                    },
                    b"sequester_revoked2".to_vec(),
                )
                .await
                .unwrap();

                $db.add_certificate(
                    &ShamirRecoveryBriefCertificate {
                        timestamp: t22,
                        author: device_id,
                        user_id,
                        // Not meaningful for the test
                        per_recipient_shares: HashMap::new(),
                        threshold: 1.try_into().unwrap(),
                    },
                    b"shamir_recovery_brief1".to_vec(),
                )
                .await
                .unwrap();

                $db.add_certificate(
                    &ShamirRecoveryBriefCertificate {
                        timestamp: t23,
                        author: other_device_id,
                        user_id: other_user_id,
                        // Not meaningful for the test
                        per_recipient_shares: HashMap::new(),
                        threshold: 1.try_into().unwrap(),
                    },
                    b"shamir_recovery_brief2".to_vec(),
                )
                .await
                .unwrap();

                $db.add_certificate(
                    &ShamirRecoveryShareCertificate {
                        timestamp: t24,
                        author: device_id,
                        user_id,
                        recipient: user_id,
                        // Not meaningful for the test
                        ciphered_share: b"ciphered_share".to_vec(),
                    },
                    b"shamir_recovery_share1".to_vec(),
                )
                .await
                .unwrap();

                $db.add_certificate(
                    &ShamirRecoveryShareCertificate {
                        timestamp: t25,
                        author: other_device_id,
                        user_id: other_user_id,
                        recipient: other_user_id,
                        // Not meaningful for the test
                        ciphered_share: b"ciphered_share".to_vec(),
                    },
                    b"shamir_recovery_share2".to_vec(),
                )
                .await
                .unwrap();
            }
        };
    }

    /*
     * Assert database populated macro
     */

    macro_rules! assert_one_in_db {
        // `$db` can be either `storage` or `updater`
        ($db: ident, $query_meth:ident $(,$query_args:expr)* => $expected:expr) => {
            async {
                let query = GetCertificateQuery::$query_meth($($query_args, )*);

                let item = $db.get_certificate_encrypted(query.clone(), UpTo::Current).await.unwrap();
                p_assert_eq!(item, $expected);

                let items = $db.get_multiple_certificates_encrypted(query, UpTo::Current, None, None).await.unwrap();
                p_assert_eq!(items, vec![$expected]);
            }
        };
    }

    macro_rules! assert_multiple_in_db {
        // `$db` can be either `storage` or `updater`
        ($db: ident, $query_meth:ident $(,$query_args:expr)* => $expected:expr) => {
            async {
                let query = GetCertificateQuery::$query_meth($($query_args, )*);

                let items = $db.get_multiple_certificates_encrypted(query.clone(), UpTo::Current, None, None).await.unwrap();
                p_assert_eq!(items, $expected);

                let expected_one = items.last().unwrap().to_owned();

                let last_item = $db.get_certificate_encrypted(query, UpTo::Current).await.unwrap();
                p_assert_eq!(last_item, expected_one);
            }
        };
    }

    macro_rules! assert_database_populated {
        // `$db` can be either `storage` or `updater`
        ($db: ident) => {
            async {
                // 1) Try to access the certificates

                assert_one_in_db!($db, user_certificate, &user_id => (t1, b"user".to_vec())).await;
                assert_one_in_db!($db, user_certificate_from_device_id, &device_id => (t1, b"user".to_vec())).await;
                assert_one_in_db!($db, revoked_user_certificate, &user_id => (t2, b"revoked_user".to_vec())).await;
                assert_one_in_db!($db, revoked_user_certificate_from_device_id, &device_id => (t2, b"revoked_user".to_vec())).await;
                assert_multiple_in_db!($db,
                    user_update_certificates, &user_id =>
                    vec![(t3, b"user_update1".to_vec()), (t4, b"user_update2".to_vec())]
                )
                .await;
                assert_one_in_db!($db, user_update_certificates, &other_user_id => (t5, b"other_user_update".to_vec())).await;
                assert_multiple_in_db!($db,
                    user_update_certificates_from_device_id, &device_id =>
                    vec![(t3, b"user_update1".to_vec()), (t4, b"user_update2".to_vec())]
                )
                .await;
                assert_one_in_db!($db, device_certificate, &device_id => (t6, b"device".to_vec())).await;
                assert_multiple_in_db!($db, user_devices_certificates, &user_id =>
                    vec![
                        (t6, b"device".to_vec())
                    ]
                )
                .await;
                assert_multiple_in_db!($db, realm_role_certificates, &realm_id =>
                    vec![
                        (t7, b"realm_role1".to_vec()),
                        (t8, b"realm_role2".to_vec()),
                        (t10, b"realm_role_other_user".to_vec())
                    ]
                )
                .await;
                assert_one_in_db!($db, realm_role_certificates, &other_realm_id => (t9, b"other_realm_role".to_vec())).await;
                assert_multiple_in_db!($db, user_realm_role_certificates, &user_id =>
                    vec![
                        (t7, b"realm_role1".to_vec()),
                        (t8, b"realm_role2".to_vec()),
                        (t9, b"other_realm_role".to_vec()),
                    ]
                )
                .await;
                assert_one_in_db!($db, user_realm_role_certificates, &other_user_id =>
                    (t10, b"realm_role_other_user".to_vec())
                )
                .await;
                assert_one_in_db!($db, sequester_authority_certificate => (t11, b"sequester_authority".to_vec()))
                    .await;
                assert_multiple_in_db!($db, sequester_service_certificates =>
                    vec![
                        (t12, b"sequester_service1".to_vec()),
                        (t13, b"sequester_service2".to_vec())
                    ]
                )
                .await;
                assert_one_in_db!($db, sequester_service_certificate, &service_id1 => (t12, b"sequester_service1".to_vec())).await;
                assert_one_in_db!($db, realm_key_rotation_certificate, &realm_id, 0 => (t14, b"realm_key_rotation1".to_vec())).await;
                assert_multiple_in_db!($db, realm_key_rotation_certificates, &realm_id =>
                    vec![
                        (t14, b"realm_key_rotation1".to_vec()),
                        (t15, b"realm_key_rotation2".to_vec()),
                    ]
                )
                .await;
                assert_multiple_in_db!($db, realm_name_certificates, &realm_id =>
                    vec![
                        (t16, b"realm_name1".to_vec()),
                        (t17, b"realm_name2".to_vec()),
                    ]
                )
                .await;
                assert_multiple_in_db!($db, realm_archiving_certificates, &realm_id =>
                    vec![
                        (t18, b"realm_archiving1".to_vec()),
                        (t19, b"realm_archiving2".to_vec()),
                    ]
                )
                .await;
                assert_one_in_db!($db, sequester_revoked_service_certificate, &service_id1 => (t20, b"sequester_revoked1".to_vec())).await;
                assert_multiple_in_db!($db, sequester_revoked_service_certificates =>
                    vec![
                        (t20, b"sequester_revoked1".to_vec()),
                        (t21, b"sequester_revoked2".to_vec()),
                    ]
                )
                .await;
                assert_one_in_db!($db, user_shamir_recovery_brief_certificates, &user_id => (t22, b"shamir_recovery_brief1".to_vec())).await;
                assert_multiple_in_db!($db, shamir_recovery_brief_certificates =>
                    vec![
                        (t22, b"shamir_recovery_brief1".to_vec()),
                        (t23, b"shamir_recovery_brief2".to_vec()),
                    ]
                )
                .await;
                assert_one_in_db!($db, user_recipient_shamir_recovery_share_certificates, &user_id, &user_id => (t24, b"shamir_recovery_share1".to_vec())).await;
                assert_multiple_in_db!($db, shamir_recovery_share_certificates =>
                    vec![
                        (t24, b"shamir_recovery_share1".to_vec()),
                        (t25, b"shamir_recovery_share2".to_vec()),
                    ]
                )
                .await;

                // 2) Ensure `up_to` filter param is working
                // Note we don't test every possible types here: this is because we have already
                // tested at step 1) that each type correctly gets turned into the right
                // `GetCertificateData` object (itself fed into the type agnostic get certificate
                // methods)

                let t0 = "2019-01-01T00:00:00Z".parse().unwrap();
                let query = GetCertificateQuery::user_certificate(&user_id);
                let item = $db
                    .get_certificate_encrypted(query.clone(), UpTo::Timestamp(t0))
                    .await;
                p_assert_matches!(
                    item,
                    Err(GetCertificateError::ExistButTooRecent{ certificate_timestamp })
                    if certificate_timestamp == "2020-01-01T00:00:00Z".parse().unwrap()
                );
                let items = $db
                    .get_multiple_certificates_encrypted(query, UpTo::Timestamp(t0), None, None)
                    .await
                    .unwrap();
                assert!(items.is_empty());

                let query = GetCertificateQuery::realm_role_certificates(&realm_id);
                let item = $db
                    .get_certificate_encrypted(query.clone(), UpTo::Timestamp(t8))
                    .await
                    .unwrap();
                p_assert_eq!(item, (t8, b"realm_role2".to_vec()));
                let items = $db
                    .get_multiple_certificates_encrypted(query, UpTo::Timestamp(t9), None, None)
                    .await
                    .unwrap();
                p_assert_eq!(
                    items,
                    vec![(t7, b"realm_role1".to_vec()), (t8, b"realm_role2".to_vec()),]
                );

                // 3) Test offset & limit

                let query = GetCertificateQuery::realm_role_certificates(&realm_id);
                p_assert_eq!(
                    $db
                        .get_multiple_certificates_encrypted(query.clone(), UpTo::Current, Some(3), None)
                        .await
                        .unwrap(),
                    vec![]
                );
                p_assert_eq!(
                    $db
                        .get_multiple_certificates_encrypted(query.clone(), UpTo::Current, Some(2), None)
                        .await
                        .unwrap(),
                    vec![(t10, b"realm_role_other_user".to_vec())]
                );
                p_assert_eq!(
                    $db
                        .get_multiple_certificates_encrypted(query.clone(), UpTo::Current, Some(1), Some(1))
                        .await
                        .unwrap(),
                    vec![(t8, b"realm_role2".to_vec())],
                );
                p_assert_eq!(
                    $db
                        .get_multiple_certificates_encrypted(query, UpTo::Current, None, Some(2))
                        .await
                        .unwrap(),
                    vec![(t7, b"realm_role1".to_vec()), (t8, b"realm_role2".to_vec())],
                );
            }
        };
    }

    let commit = match kind {
        "commit" => true,
        "rollback" => false,
        unknown => panic!("Unknown kind: {unknown}"),
    };

    // 0) The database starts empty

    assert_empty_db!(storage).await;

    let outcome = storage
        .for_update(async move |mut updater| {
            // 1) Try with no certificates in the database...

            assert_empty_db!(updater).await;

            // 2) ...now populate with certificates...
            populate_db!(updater).await;

            // 3) ...and access them !

            assert_database_populated!(updater).await;

            if commit {
                Ok(())
            } else {
                Err(())
            }
        })
        .await
        .unwrap();

    if commit {
        p_assert_matches!(outcome, Ok(()));
        assert_database_populated!(storage).await;
    } else {
        p_assert_matches!(outcome, Err(()));
        assert_empty_db!(storage).await;
    }
}

#[parsec_test(testbed = "minimal")]
async fn forget_all_certificates(mut timestamps: TimestampGenerator, env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let mut storage = CertificatesStorage::start(&env.discriminant_dir, &alice)
        .await
        .unwrap();

    let user_id = alice.user_id;
    let device_id = alice.device_id;
    let realm_id = VlobID::from_hex("fe15ca8ad55140d08b4951f26f7073d5").unwrap();
    let service_id = SequesterServiceID::from_hex("a065170f3b6649f997ec14dbe36c5c13").unwrap();

    storage
        .for_update(async move |mut updater| {
            // Add certificates

            let t1 = timestamps.next();
            updater
                .add_certificate(
                    &UserCertificate {
                        timestamp: t1,
                        user_id,
                        // Not meaningful for the test
                        author: CertificateSigner::Root,
                        human_handle: MaybeRedacted::Redacted(HumanHandle::new_redacted(user_id)),
                        public_key: alice.public_key(),
                        algorithm: PrivateKeyAlgorithm::X25519XSalsa20Poly1305,
                        profile: UserProfile::Admin,
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            let t2 = timestamps.next();
            updater
                .add_certificate(
                    &RevokedUserCertificate {
                        timestamp: t2,
                        user_id,
                        // Not meaningful for the test
                        author: DeviceID::default(),
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            let t3 = timestamps.next();
            updater
                .add_certificate(
                    &UserUpdateCertificate {
                        timestamp: t3,
                        user_id,
                        // Not meaningful for the test
                        author: DeviceID::default(),
                        new_profile: UserProfile::Admin,
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            let t4 = timestamps.next();
            updater
                .add_certificate(
                    &DeviceCertificate {
                        timestamp: t4,
                        device_id,
                        purpose: DevicePurpose::Standard,
                        user_id,
                        // Not meaningful for the test
                        author: CertificateSigner::Root,
                        device_label: MaybeRedacted::Redacted(DeviceLabel::new_redacted(device_id)),
                        verify_key: alice.verify_key(),
                        algorithm: SigningKeyAlgorithm::Ed25519,
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            let t5 = timestamps.next();
            updater
                .add_certificate(
                    &RealmRoleCertificate {
                        timestamp: t5,
                        realm_id,
                        user_id,
                        // Not meaningful for the test
                        author: device_id,
                        role: None,
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            let t6 = timestamps.next();
            updater
                .add_certificate(
                    &SequesterAuthorityCertificate {
                        timestamp: t6,
                        // Not meaningful for the test
                        verify_key_der: SequesterSigningKeyDer::generate_pair(
                            SequesterKeySize::_1024Bits,
                        )
                        .1,
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            let t7 = timestamps.next();
            updater
                .add_certificate(
                    &SequesterServiceCertificate {
                        timestamp: t7,
                        service_id,
                        // Not meaningful for the test
                        encryption_key_der: SequesterPrivateKeyDer::generate_pair(
                            SequesterKeySize::_1024Bits,
                        )
                        .1,
                        service_label: "service_label".to_string(),
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            let t8 = timestamps.next();
            updater
                .add_certificate(
                    &RealmKeyRotationCertificate {
                        timestamp: t8,
                        realm_id,
                        key_index: 0,
                        // Not meaningful for the test
                        author: device_id,
                        encryption_algorithm: SecretKeyAlgorithm::Blake2bXsalsa20Poly1305,
                        hash_algorithm: HashAlgorithm::Sha256,
                        key_canary: b"key_canary".to_vec(),
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            let t9 = timestamps.next();
            updater
                .add_certificate(
                    &RealmNameCertificate {
                        timestamp: t9,
                        realm_id,
                        // Not meaningful for the test
                        key_index: 0,
                        author: device_id,
                        encrypted_name: b"encrypted_name".to_vec(),
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            let t10 = timestamps.next();
            updater
                .add_certificate(
                    &RealmArchivingCertificate {
                        timestamp: t10,
                        realm_id,
                        // Not meaningful for the test
                        author: device_id,
                        configuration: RealmArchivingConfiguration::Archived,
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            let t11 = timestamps.next();
            updater
                .add_certificate(
                    &SequesterRevokedServiceCertificate {
                        timestamp: t11,
                        service_id,
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            let t12 = timestamps.next();
            updater
                .add_certificate(
                    &ShamirRecoveryBriefCertificate {
                        timestamp: t12,
                        author: device_id,
                        user_id,
                        // Not meaningful for the test
                        per_recipient_shares: HashMap::new(),
                        threshold: 1.try_into().unwrap(),
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            let t13 = timestamps.next();
            updater
                .add_certificate(
                    &ShamirRecoveryShareCertificate {
                        timestamp: t13,
                        author: device_id,
                        user_id,
                        recipient: user_id,
                        // Not meaningful for the test
                        ciphered_share: b"ciphered_share".to_vec(),
                    },
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

            // Now drop the certificates...
            updater.forget_all_certificates().await.unwrap();

            // ...and make sure this is the case !

            p_assert_eq!(
                updater.get_last_timestamps().await.unwrap(),
                PerTopicLastTimestamps {
                    common: None,
                    sequester: None,
                    realm: HashMap::new(),
                    shamir_recovery: None,
                }
            );

            async fn assert_not_in_db(
                storage: &mut CertificatesStorageUpdater<'_>,
                query: GetCertificateQuery<'_>,
            ) {
                p_assert_matches!(
                    storage
                        .get_certificate_encrypted(query, UpTo::Current)
                        .await,
                    Err(GetCertificateError::NonExisting)
                );
            }

            assert_not_in_db(
                &mut updater,
                GetCertificateQuery::user_certificate(&user_id),
            )
            .await;
            assert_not_in_db(
                &mut updater,
                GetCertificateQuery::user_certificate_from_device_id(&device_id),
            )
            .await;
            assert_not_in_db(
                &mut updater,
                GetCertificateQuery::revoked_user_certificate(&user_id),
            )
            .await;
            assert_not_in_db(
                &mut updater,
                GetCertificateQuery::revoked_user_certificate_from_device_id(&device_id),
            )
            .await;
            assert_not_in_db(
                &mut updater,
                GetCertificateQuery::user_update_certificates(&user_id),
            )
            .await;
            assert_not_in_db(
                &mut updater,
                GetCertificateQuery::user_update_certificates_from_device_id(&device_id),
            )
            .await;
            assert_not_in_db(
                &mut updater,
                GetCertificateQuery::device_certificate(&device_id),
            )
            .await;
            assert_not_in_db(
                &mut updater,
                GetCertificateQuery::user_devices_certificates(&user_id),
            )
            .await;
            assert_not_in_db(
                &mut updater,
                GetCertificateQuery::realm_role_certificates(&realm_id),
            )
            .await;
            assert_not_in_db(
                &mut updater,
                GetCertificateQuery::user_realm_role_certificates(&user_id),
            )
            .await;
            assert_not_in_db(
                &mut updater,
                GetCertificateQuery::sequester_authority_certificate(),
            )
            .await;
            assert_not_in_db(
                &mut updater,
                GetCertificateQuery::sequester_service_certificates(),
            )
            .await;

            Result::<(), ()>::Ok(())
        })
        .await
        .unwrap()
        .unwrap();
}

#[cfg(not(target_arch = "wasm32"))]
#[parsec_test]
async fn start_with_on_disk_db(tmp_path: TmpPath, alice: &Device) {
    // Start when the db file does not exist
    let storage = CertificatesStorage::start(&tmp_path, &alice.local_device())
        .await
        .unwrap();
    storage.stop().await.unwrap();

    // Check the db files have been created
    assert!(tmp_path
        .join("de10a11cec0010000000000000000000/certificates-v1.sqlite")
        .exists());

    // Start when the db file already exists
    let storage = CertificatesStorage::start(&tmp_path, &alice.local_device())
        .await
        .unwrap();
    storage.stop().await.unwrap();
}
