// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// TODO: fix those tests !

use std::collections::HashMap;

use crate::certificates::CertificatesStorageUpdater;

use super::{
    CertificatesStorage, GetCertificateError, GetCertificateQuery, PerTopicLastTimestamps, UpTo,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

enum FetchStrategy {
    Single,
    Multiple,
}

#[parsec_test(testbed = "minimal")]
#[case::single_fetch(FetchStrategy::Single)]
#[case::multiple_fetch(FetchStrategy::Multiple)]
async fn testbed_support(#[case] fetch_strategy: FetchStrategy, env: &TestbedEnv) {
    let mut expected_last_timestamps = PerTopicLastTimestamps {
        common: None,
        sequester: None,
        shamir_recovery: None,
        realm: HashMap::default(),
    };

    env.customize(|builder| {
        expected_last_timestamps.common = builder.new_user("bob").map(|e| Some(e.timestamp));
        builder.new_realm("bob").map(|e| {
            expected_last_timestamps
                .realm
                .insert(e.realm_id, e.timestamp);
        });

        builder.certificates_storage_fetch_certificates("alice@dev1");

        if matches!(fetch_strategy, FetchStrategy::Multiple) {
            // Only the last fetch is taken into account, so this should be known
            builder.new_device("alice");
            expected_last_timestamps.common = builder.new_user("bill").map(|e| Some(e.timestamp));
            builder.new_realm("alice").map(|e| {
                expected_last_timestamps
                    .realm
                    .insert(e.realm_id, e.timestamp);
            });
            builder.certificates_storage_fetch_certificates("alice@dev1");
        }

        // Stuff the our storage is not aware of
        builder.new_realm("alice");
        builder.new_device("alice");
        builder.new_user("john");
    });

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
async fn get_last_timestamps(mut timestamps: TimestampGenerator, env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let mut storage = CertificatesStorage::start(&env.discriminant_dir, &alice)
        .await
        .unwrap();
    let mut storage_mut = storage.for_update().await.unwrap();

    // Try with no certificates in the database...
    let mut expected = PerTopicLastTimestamps {
        common: None,
        sequester: None,
        realm: HashMap::new(),
        shamir_recovery: None,
    };

    p_assert_eq!(storage_mut.get_last_timestamps().await.unwrap(), expected);

    // Now with certificates

    let device_id = &alice.device_id;
    let user_id = alice.device_id.user_id();
    let realm_id = VlobID::from_hex("fe15ca8ad55140d08b4951f26f7073d5").unwrap();
    let service_id = SequesterServiceID::from_hex("a065170f3b6649f997ec14dbe36c5c13").unwrap();

    let t1 = timestamps.next();
    storage_mut
        .add_certificate(
            &UserCertificate {
                timestamp: t1,
                user_id: user_id.clone(),
                // Not meaningful for the test
                author: CertificateSignerOwned::Root,
                human_handle: MaybeRedacted::Redacted(HumanHandle::new_redacted(user_id)),
                public_key: alice.public_key(),
                profile: UserProfile::Admin,
            },
            b"<encrypted>".to_vec(),
        )
        .await
        .unwrap();

    expected.common = Some(t1);

    p_assert_eq!(storage_mut.get_last_timestamps().await.unwrap(), expected);

    let t2 = timestamps.next();
    storage_mut
        .add_certificate(
            &RevokedUserCertificate {
                timestamp: t2,
                user_id: user_id.clone(),
                // Not meaningful for the test
                author: DeviceID::default(),
            },
            b"<encrypted>".to_vec(),
        )
        .await
        .unwrap();

    expected.common = Some(t2);

    p_assert_eq!(storage_mut.get_last_timestamps().await.unwrap(), expected);

    let t3 = timestamps.next();
    storage_mut
        .add_certificate(
            &UserUpdateCertificate {
                timestamp: t3,
                user_id: user_id.clone(),
                // Not meaningful for the test
                author: DeviceID::default(),
                new_profile: UserProfile::Admin,
            },
            b"<encrypted>".to_vec(),
        )
        .await
        .unwrap();

    expected.common = Some(t3);

    p_assert_eq!(storage_mut.get_last_timestamps().await.unwrap(), expected);

    let t4 = timestamps.next();
    storage_mut
        .add_certificate(
            &DeviceCertificate {
                timestamp: t4,
                device_id: device_id.clone(),
                // Not meaningful for the test
                author: CertificateSignerOwned::Root,
                device_label: MaybeRedacted::Redacted(DeviceLabel::new_redacted(
                    &DeviceName::default(),
                )),
                verify_key: alice.verify_key(),
            },
            b"<encrypted>".to_vec(),
        )
        .await
        .unwrap();

    expected.common = Some(t4);

    p_assert_eq!(storage_mut.get_last_timestamps().await.unwrap(), expected);

    let t5 = timestamps.next();
    storage_mut
        .add_certificate(
            &RealmRoleCertificate {
                timestamp: t5,
                realm_id,
                user_id: user_id.clone(),
                // Not meaningful for the test
                author: CertificateSignerOwned::Root,
                role: None,
            },
            b"<encrypted>".to_vec(),
        )
        .await
        .unwrap();

    expected.realm.insert(realm_id, t5);

    p_assert_eq!(storage_mut.get_last_timestamps().await.unwrap(), expected);

    let t6 = timestamps.next();
    storage_mut
        .add_certificate(
            &SequesterAuthorityCertificate {
                timestamp: t6,
                // Not meaningful for the test
                verify_key_der: SequesterSigningKeyDer::generate_pair(SequesterKeySize::_1024Bits)
                    .1,
            },
            b"<encrypted>".to_vec(),
        )
        .await
        .unwrap();

    expected.sequester = Some(t6);

    p_assert_eq!(storage_mut.get_last_timestamps().await.unwrap(), expected);

    let t7 = timestamps.next();
    storage_mut
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

    p_assert_eq!(storage_mut.get_last_timestamps().await.unwrap(), expected);
}

#[parsec_test(testbed = "minimal")]
async fn get_certificate_empty_storage(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let mut storage = CertificatesStorage::start(&env.discriminant_dir, &alice)
        .await
        .unwrap();

    let user_id = alice.device_id.user_id();
    let device_id = &alice.device_id;
    let realm_id = VlobID::from_hex("fe15ca8ad55140d08b4951f26f7073d5").unwrap();
    let service_id1 = SequesterServiceID::from_hex("a065170f3b6649f997ec14dbe36c5c13").unwrap();

    macro_rules! assert_not_in_db {
        ($query_meth:ident $(,$query_args:expr)*) => {
            async {
                let query = GetCertificateQuery::$query_meth($($query_args, )*);

                let outcome = storage.get_certificate_encrypted(query.clone(), UpTo::Current).await;
                p_assert_matches!(outcome, Err(GetCertificateError::NonExisting));

                let items = storage.get_multiple_certificates_encrypted(query, UpTo::Current, None, None).await.unwrap();
                assert!(items.is_empty());
            }
        };
    }

    assert_not_in_db!(user_certificate, user_id.to_owned()).await;
    assert_not_in_db!(revoked_user_certificate, user_id.to_owned()).await;
    assert_not_in_db!(user_update_certificates, user_id.to_owned()).await;
    assert_not_in_db!(device_certificate, device_id.to_owned()).await;
    assert_not_in_db!(user_device_certificates, user_id.to_owned()).await;
    assert_not_in_db!(realm_role_certificates, realm_id).await;
    assert_not_in_db!(realm_name_certificates, realm_id).await;
    assert_not_in_db!(realm_key_rotation_certificate, realm_id, 1).await;
    assert_not_in_db!(realm_key_rotation_certificates, realm_id).await;
    assert_not_in_db!(realm_archiving_certificates, realm_id).await;
    assert_not_in_db!(user_realm_role_certificates, user_id.to_owned()).await;
    assert_not_in_db!(shamir_recovery_brief_certificates).await;
    assert_not_in_db!(shamir_recovery_share_certificates).await;
    assert_not_in_db!(sequester_authority_certificate).await;
    assert_not_in_db!(sequester_service_certificates).await;
    assert_not_in_db!(sequester_service_certificate, service_id1).await;
}

#[parsec_test(testbed = "minimal")]
async fn get_certificate(mut timestamps: TimestampGenerator, env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let mut storage = CertificatesStorage::start(&env.discriminant_dir, &alice)
        .await
        .unwrap();

    let mut storage_mut = storage.for_update().await.unwrap();

    let user_id = alice.device_id.user_id();
    let other_user_id = &"bob".parse::<UserID>().unwrap();
    let device_id = &alice.device_id;
    let realm_id = VlobID::from_hex("fe15ca8ad55140d08b4951f26f7073d5").unwrap();
    let other_realm_id = VlobID::from_hex("810d8d137b934985b62627042058aee4").unwrap();
    let service_id1 = SequesterServiceID::from_hex("a065170f3b6649f997ec14dbe36c5c13").unwrap();
    let service_id2 = SequesterServiceID::from_hex("fc93467e47ec48a2b654fe8bf28fe7ea").unwrap();

    // 1) Try with no certificates in the database...

    macro_rules! assert_not_in_db {
        ($query_meth:ident $(,$query_args:expr)*) => {
            async {
                let query = GetCertificateQuery::$query_meth($($query_args, )*);

                let outcome = storage_mut.get_certificate_encrypted(query.clone(), UpTo::Current).await;
                p_assert_matches!(outcome, Err(GetCertificateError::NonExisting));

                let items = storage_mut.get_multiple_certificates_encrypted(query, UpTo::Current, None, None).await.unwrap();
                assert!(items.is_empty());
            }
        };
    }

    assert_not_in_db!(user_certificate, user_id.to_owned()).await;
    assert_not_in_db!(revoked_user_certificate, user_id.to_owned()).await;
    assert_not_in_db!(user_update_certificates, user_id.to_owned()).await;
    assert_not_in_db!(device_certificate, device_id.to_owned()).await;
    assert_not_in_db!(user_device_certificates, user_id.to_owned()).await;
    assert_not_in_db!(realm_role_certificates, realm_id).await;
    assert_not_in_db!(user_realm_role_certificates, user_id.to_owned()).await;
    assert_not_in_db!(sequester_authority_certificate).await;
    assert_not_in_db!(sequester_service_certificates).await;
    assert_not_in_db!(sequester_service_certificate, service_id1).await;

    // 2) ...now populate with certificates...

    let t1 = timestamps.next();
    storage_mut
        .add_certificate(
            &UserCertificate {
                timestamp: t1,
                user_id: user_id.clone(),
                // Not meaningful for the test
                author: CertificateSignerOwned::Root,
                human_handle: MaybeRedacted::Redacted(HumanHandle::new_redacted(user_id)),
                public_key: alice.public_key(),
                profile: UserProfile::Admin,
            },
            b"user".to_vec(),
        )
        .await
        .unwrap();

    let t2 = timestamps.next();
    storage_mut
        .add_certificate(
            &RevokedUserCertificate {
                timestamp: t2,
                user_id: user_id.clone(),
                // Not meaningful for the test
                author: DeviceID::default(),
            },
            b"revoked_user".to_vec(),
        )
        .await
        .unwrap();

    let t3 = timestamps.next();
    storage_mut
        .add_certificate(
            &UserUpdateCertificate {
                timestamp: t3,
                user_id: user_id.clone(),
                // Not meaningful for the test
                author: DeviceID::default(),
                new_profile: UserProfile::Admin,
            },
            b"user_update1".to_vec(),
        )
        .await
        .unwrap();

    let t4 = timestamps.next();
    storage_mut
        .add_certificate(
            &UserUpdateCertificate {
                timestamp: t4,
                user_id: user_id.clone(),
                // Not meaningful for the test
                author: DeviceID::default(),
                new_profile: UserProfile::Admin,
            },
            b"user_update2".to_vec(),
        )
        .await
        .unwrap();

    let t5 = timestamps.next();
    storage_mut
        .add_certificate(
            &UserUpdateCertificate {
                timestamp: t5,
                user_id: other_user_id.clone(),
                // Not meaningful for the test
                author: DeviceID::default(),
                new_profile: UserProfile::Admin,
            },
            b"other_user_update".to_vec(),
        )
        .await
        .unwrap();

    let t6 = timestamps.next();
    storage_mut
        .add_certificate(
            &DeviceCertificate {
                timestamp: t6,
                device_id: device_id.clone(),
                // Not meaningful for the test
                author: CertificateSignerOwned::Root,
                device_label: MaybeRedacted::Redacted(DeviceLabel::new_redacted(
                    &DeviceName::default(),
                )),
                verify_key: alice.verify_key(),
            },
            b"device".to_vec(),
        )
        .await
        .unwrap();

    let t7 = timestamps.next();
    storage_mut
        .add_certificate(
            &RealmRoleCertificate {
                timestamp: t7,
                realm_id,
                user_id: user_id.clone(),
                // Not meaningful for the test
                author: CertificateSignerOwned::Root,
                role: None,
            },
            b"realm_role1".to_vec(),
        )
        .await
        .unwrap();

    let t8 = timestamps.next();
    storage_mut
        .add_certificate(
            &RealmRoleCertificate {
                timestamp: t8,
                realm_id,
                user_id: user_id.clone(),
                // Not meaningful for the test
                author: CertificateSignerOwned::Root,
                role: None,
            },
            b"realm_role2".to_vec(),
        )
        .await
        .unwrap();

    let t9 = timestamps.next();
    storage_mut
        .add_certificate(
            &RealmRoleCertificate {
                timestamp: t9,
                realm_id: other_realm_id,
                user_id: user_id.clone(),
                // Not meaningful for the test
                author: CertificateSignerOwned::Root,
                role: None,
            },
            b"other_realm_role".to_vec(),
        )
        .await
        .unwrap();

    let t10 = timestamps.next();
    storage_mut
        .add_certificate(
            &RealmRoleCertificate {
                timestamp: t10,
                realm_id: realm_id,
                user_id: other_user_id.clone(),
                // Not meaningful for the test
                author: CertificateSignerOwned::Root,
                role: None,
            },
            b"realm_role_other_user".to_vec(),
        )
        .await
        .unwrap();

    let t11 = timestamps.next();
    storage_mut
        .add_certificate(
            &SequesterAuthorityCertificate {
                timestamp: t11,
                // Not meaningful for the test
                verify_key_der: SequesterSigningKeyDer::generate_pair(SequesterKeySize::_1024Bits)
                    .1,
            },
            b"sequester_authority".to_vec(),
        )
        .await
        .unwrap();

    let t12 = timestamps.next();
    storage_mut
        .add_certificate(
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

    let t13 = timestamps.next();
    storage_mut
        .add_certificate(
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

    // 3) ...and access them !

    macro_rules! assert_one_in_db {
        ($query_meth:ident $(,$query_args:expr)* => $expected:expr) => {
            async {
                let query = GetCertificateQuery::$query_meth($($query_args, )*);

                let item = storage_mut.get_certificate_encrypted(query.clone(), UpTo::Current).await.unwrap();
                p_assert_eq!(item, $expected);

                let items = storage_mut.get_multiple_certificates_encrypted(query, UpTo::Current, None, None).await.unwrap();
                p_assert_eq!(items, vec![$expected]);
            }
        };
    }

    macro_rules! assert_multiple_in_db {
        ($query_meth:ident $(,$query_args:expr)* => $expected:expr) => {
            async {
                let query = GetCertificateQuery::$query_meth($($query_args, )*);

                let items = storage_mut.get_multiple_certificates_encrypted(query.clone(), UpTo::Current, None, None).await.unwrap();
                p_assert_eq!(items, $expected);

                let expected_one = items.last().unwrap().to_owned();

                let last_item = storage_mut.get_certificate_encrypted(query, UpTo::Current).await.unwrap();
                p_assert_eq!(last_item, expected_one);
            }
        };
    }

    assert_one_in_db!(user_certificate, user_id.to_owned() => (t1, b"user".to_vec())).await;
    assert_one_in_db!(revoked_user_certificate, user_id.to_owned() => (t2, b"revoked_user".to_vec())).await;
    assert_multiple_in_db!(
        user_update_certificates, user_id.to_owned() =>
        vec![(t3, b"user_update1".to_vec()), (t4, b"user_update2".to_vec())]
    )
    .await;
    assert_one_in_db!(user_update_certificates, other_user_id.to_owned() => (t5, b"other_user_update".to_vec())).await;
    assert_one_in_db!(device_certificate, device_id.to_owned() => (t6, b"device".to_vec())).await;
    assert_multiple_in_db!(user_device_certificates, user_id.to_owned() =>
        vec![
            (t6, b"device".to_vec())
        ]
    )
    .await;
    assert_multiple_in_db!(realm_role_certificates, realm_id.to_owned() =>
        vec![
            (t7, b"realm_role1".to_vec()),
            (t8, b"realm_role2".to_vec()),
            (t10, b"realm_role_other_user".to_vec())
        ]
    )
    .await;
    assert_one_in_db!(realm_role_certificates, other_realm_id.to_owned() => (t9, b"other_realm_role".to_vec())).await;
    assert_multiple_in_db!(user_realm_role_certificates, user_id.to_owned() =>
        vec![
            (t7, b"realm_role1".to_vec()),
            (t8, b"realm_role2".to_vec()),
            (t9, b"other_realm_role".to_vec()),
        ]
    )
    .await;
    assert_one_in_db!(user_realm_role_certificates, other_user_id.to_owned() =>
        (t10, b"realm_role_other_user".to_vec())
    )
    .await;
    assert_one_in_db!(sequester_authority_certificate => (t11, b"sequester_authority".to_vec()))
        .await;
    assert_multiple_in_db!(sequester_service_certificates =>
        vec![
            (t12, b"sequester_service1".to_vec()),
            (t13, b"sequester_service2".to_vec())
        ]
    )
    .await;
    assert_one_in_db!(sequester_service_certificate, service_id1 => (t12, b"sequester_service1".to_vec())).await;

    // 4) Ensure `up_to` filter param is working
    // Note we don't test every possible types here: this is because we have already
    // tested at step 3) that each type correctly gets turned into the right
    // `GetCertificateData` object (itself fed into the type agnostic get certificate
    // methods)

    let t0 = "2019-01-01T00:00:00Z".parse().unwrap();
    let query = GetCertificateQuery::user_certificate(user_id.to_owned());
    let item = storage_mut
        .get_certificate_encrypted(query.clone(), UpTo::Timestamp(t0))
        .await;
    p_assert_matches!(
        item,
        Err(GetCertificateError::ExistButTooRecent{ certificate_timestamp })
        if certificate_timestamp == "2020-01-01T00:00:00Z".parse().unwrap()
    );
    let items = storage_mut
        .get_multiple_certificates_encrypted(query, UpTo::Timestamp(t0), None, None)
        .await
        .unwrap();
    assert!(items.is_empty());

    let query = GetCertificateQuery::realm_role_certificates(realm_id.to_owned());
    let item = storage_mut
        .get_certificate_encrypted(query.clone(), UpTo::Timestamp(t8))
        .await
        .unwrap();
    p_assert_eq!(item, (t8, b"realm_role2".to_vec()));
    let items = storage_mut
        .get_multiple_certificates_encrypted(query, UpTo::Timestamp(t9), None, None)
        .await
        .unwrap();
    p_assert_eq!(
        items,
        vec![(t7, b"realm_role1".to_vec()), (t8, b"realm_role2".to_vec()),]
    );

    // 5) Test offset & limit

    let query = GetCertificateQuery::realm_role_certificates(realm_id.to_owned());
    p_assert_eq!(
        storage_mut
            .get_multiple_certificates_encrypted(query.clone(), UpTo::Current, Some(3), None)
            .await
            .unwrap(),
        vec![]
    );
    p_assert_eq!(
        storage_mut
            .get_multiple_certificates_encrypted(query.clone(), UpTo::Current, Some(2), None)
            .await
            .unwrap(),
        vec![(t10, b"realm_role_other_user".to_vec())]
    );
    p_assert_eq!(
        storage_mut
            .get_multiple_certificates_encrypted(query.clone(), UpTo::Current, Some(1), Some(1))
            .await
            .unwrap(),
        vec![(t8, b"realm_role2".to_vec())],
    );
    p_assert_eq!(
        storage_mut
            .get_multiple_certificates_encrypted(query, UpTo::Current, None, Some(2))
            .await
            .unwrap(),
        vec![(t7, b"realm_role1".to_vec()), (t8, b"realm_role2".to_vec())],
    );
}

#[parsec_test(testbed = "minimal")]
async fn forget_all_certificates(mut timestamps: TimestampGenerator, env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let mut storage = CertificatesStorage::start(&env.discriminant_dir, &alice)
        .await
        .unwrap();

    let mut storage_mut = storage.for_update().await.unwrap();

    let user_id = alice.device_id.user_id();
    let device_id = &alice.device_id;
    let realm_id = VlobID::from_hex("fe15ca8ad55140d08b4951f26f7073d5").unwrap();
    let service_id = SequesterServiceID::from_hex("a065170f3b6649f997ec14dbe36c5c13").unwrap();

    // Add certificates

    let t1 = timestamps.next();
    storage_mut
        .add_certificate(
            &UserCertificate {
                timestamp: t1,
                user_id: user_id.clone(),
                // Not meaningful for the test
                author: CertificateSignerOwned::Root,
                human_handle: MaybeRedacted::Redacted(HumanHandle::new_redacted(user_id)),
                public_key: alice.public_key(),
                profile: UserProfile::Admin,
            },
            b"<encrypted>".to_vec(),
        )
        .await
        .unwrap();

    let t2 = timestamps.next();
    storage_mut
        .add_certificate(
            &RevokedUserCertificate {
                timestamp: t2,
                user_id: user_id.clone(),
                // Not meaningful for the test
                author: DeviceID::default(),
            },
            b"<encrypted>".to_vec(),
        )
        .await
        .unwrap();

    let t3 = timestamps.next();
    storage_mut
        .add_certificate(
            &UserUpdateCertificate {
                timestamp: t3,
                user_id: user_id.clone(),
                // Not meaningful for the test
                author: DeviceID::default(),
                new_profile: UserProfile::Admin,
            },
            b"<encrypted>".to_vec(),
        )
        .await
        .unwrap();

    let t4 = timestamps.next();
    storage_mut
        .add_certificate(
            &DeviceCertificate {
                timestamp: t4,
                device_id: device_id.clone(),
                // Not meaningful for the test
                author: CertificateSignerOwned::Root,
                device_label: MaybeRedacted::Redacted(DeviceLabel::new_redacted(
                    &DeviceName::default(),
                )),
                verify_key: alice.verify_key(),
            },
            b"<encrypted>".to_vec(),
        )
        .await
        .unwrap();

    let t5 = timestamps.next();
    storage_mut
        .add_certificate(
            &RealmRoleCertificate {
                timestamp: t5,
                realm_id,
                user_id: user_id.clone(),
                // Not meaningful for the test
                author: CertificateSignerOwned::Root,
                role: None,
            },
            b"<encrypted>".to_vec(),
        )
        .await
        .unwrap();

    let t6 = timestamps.next();
    storage_mut
        .add_certificate(
            &SequesterAuthorityCertificate {
                timestamp: t6,
                // Not meaningful for the test
                verify_key_der: SequesterSigningKeyDer::generate_pair(SequesterKeySize::_1024Bits)
                    .1,
            },
            b"<encrypted>".to_vec(),
        )
        .await
        .unwrap();

    let t7 = timestamps.next();
    storage_mut
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

    // Now drop the certificates...
    storage_mut.forget_all_certificates().await.unwrap();

    // ...and make sure this is the case !

    p_assert_eq!(
        storage_mut.get_last_timestamps().await.unwrap(),
        PerTopicLastTimestamps {
            common: None,
            sequester: None,
            realm: HashMap::new(),
            shamir_recovery: None,
        }
    );

    async fn assert_not_in_db(
        storage: &mut CertificatesStorageUpdater<'_>,
        query: GetCertificateQuery,
    ) {
        p_assert_matches!(
            storage
                .get_certificate_encrypted(query, UpTo::Current)
                .await,
            Err(GetCertificateError::NonExisting)
        );
    }

    assert_not_in_db(
        &mut storage_mut,
        GetCertificateQuery::user_certificate(user_id.to_owned()),
    )
    .await;
    assert_not_in_db(
        &mut storage_mut,
        GetCertificateQuery::revoked_user_certificate(user_id.to_owned()),
    )
    .await;
    assert_not_in_db(
        &mut storage_mut,
        GetCertificateQuery::user_update_certificates(user_id.to_owned()),
    )
    .await;
    assert_not_in_db(
        &mut storage_mut,
        GetCertificateQuery::device_certificate(device_id.to_owned()),
    )
    .await;
    assert_not_in_db(
        &mut storage_mut,
        GetCertificateQuery::user_device_certificates(user_id.to_owned()),
    )
    .await;
    assert_not_in_db(
        &mut storage_mut,
        GetCertificateQuery::realm_role_certificates(realm_id.to_owned()),
    )
    .await;
    assert_not_in_db(
        &mut storage_mut,
        GetCertificateQuery::user_realm_role_certificates(user_id.to_owned()),
    )
    .await;
    assert_not_in_db(
        &mut storage_mut,
        GetCertificateQuery::sequester_authority_certificate(),
    )
    .await;
    assert_not_in_db(
        &mut storage_mut,
        GetCertificateQuery::sequester_service_certificates(),
    )
    .await;
}
