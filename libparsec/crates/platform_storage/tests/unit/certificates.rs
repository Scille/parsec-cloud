// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use super::{
    AddCertificateData, CertificatesStorage, GetCertificateError, GetCertificateQuery,
    GetTimestampBoundsError, UpTo,
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
    let mut expected_last_index = None;

    env.customize(|builder| {
        builder.new_user("bob");
        builder.new_realm("bob");

        expected_last_index = Some((
            builder.current_certificate_index(),
            builder.current_timestamp(),
        ));
        builder.certificates_storage_fetch_certificates("alice@dev1");

        if matches!(fetch_strategy, FetchStrategy::Multiple) {
            // Only the last fetch is taken into account, so this should be known
            builder.new_device("alice");
            builder.new_user("bill");
            builder.new_realm("alice");

            expected_last_index = Some((
                builder.current_certificate_index(),
                builder.current_timestamp(),
            ));
            builder.certificates_storage_fetch_certificates("alice@dev1");
        }

        // Stuff the our storage is not aware of
        builder.new_realm("alice");
        builder.new_device("alice");
        builder.new_user("john");

        // Sanity check to ensure additional (and to be ignored) certificates have been added
        p_assert_ne!(
            expected_last_index.unwrap().0,
            builder.current_certificate_index()
        );
        p_assert_ne!(expected_last_index.unwrap().1, builder.current_timestamp());
    });

    let alice = env.local_device("alice@dev1");

    let storage = CertificatesStorage::start(&env.discriminant_dir, &alice)
        .await
        .unwrap();

    p_assert_eq!(storage.get_last_index().await.unwrap(), expected_last_index);
}

#[parsec_test(testbed = "minimal")]
async fn get_last_index(mut timestamps: TimestampGenerator, env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let storage = CertificatesStorage::start(&env.discriminant_dir, &alice)
        .await
        .unwrap();

    // Try with no certificates in the database...

    p_assert_eq!(storage.get_last_index().await.unwrap(), None);

    // Now with certificates

    macro_rules! add_and_check {
        ($meth:ident, $index:expr, $timestamp:expr $(,$args:expr)*) => {
            let data = AddCertificateData::$meth(
                b"<encrypted>".to_vec(),
                $timestamp,
                $($args,)*
            );
            storage.add_next_certificate($index, data).await.unwrap();

            p_assert_eq!(
                storage.get_last_index().await.unwrap(),
                Some(($index, $timestamp))
            );
        };
    }

    let device_id = &alice.device_id;
    let user_id = alice.device_id.user_id();
    let realm_id = VlobID::from_hex("fe15ca8ad55140d08b4951f26f7073d5").unwrap();
    let service_id = SequesterServiceID::from_hex("a065170f3b6649f997ec14dbe36c5c13").unwrap();

    let t1 = timestamps.next();
    add_and_check!(from_user_certificate, 1, t1, user_id.to_owned());

    let t2 = timestamps.next();
    add_and_check!(from_revoked_user_certificate, 2, t2, user_id.to_owned());

    let t3 = timestamps.next();
    add_and_check!(from_user_update_certificate, 3, t3, user_id.to_owned());

    let t4 = timestamps.next();
    add_and_check!(from_device_certificate, 4, t4, device_id.to_owned());

    let t5 = timestamps.next();
    add_and_check!(
        from_realm_role_certificate,
        5,
        t5,
        realm_id,
        user_id.to_owned()
    );

    let t6 = timestamps.next();
    add_and_check!(from_sequester_authority_certificate, 6, t6);

    let t7 = timestamps.next();
    add_and_check!(from_sequester_service_certificate, 7, t7, service_id);
}

#[parsec_test(testbed = "minimal")]
async fn get_timestamp_bounds(mut timestamps: TimestampGenerator, env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let storage = CertificatesStorage::start(&env.discriminant_dir, &alice)
        .await
        .unwrap();

    // Try with no certificates in the database...

    p_assert_matches!(
        storage.get_timestamp_bounds(1).await,
        Err(GetTimestampBoundsError::NonExisting)
    );
    p_assert_matches!(
        storage.get_timestamp_bounds(10).await,
        Err(GetTimestampBoundsError::NonExisting)
    );

    // Now with certificates

    macro_rules! add {
        ($meth:ident, $index:expr, $timestamp:expr $(,$args:expr)*) => {
            let data = AddCertificateData::$meth(
                b"<encrypted>".to_vec(),
                $timestamp,
                $($args,)*
            );
            storage.add_next_certificate($index, data).await.unwrap();
        };
    }

    let user_id = alice.device_id.user_id();
    let device_id = &alice.device_id;
    let realm_id = VlobID::from_hex("fe15ca8ad55140d08b4951f26f7073d5").unwrap();
    let service_id = SequesterServiceID::from_hex("a065170f3b6649f997ec14dbe36c5c13").unwrap();

    let t1 = timestamps.next();
    add!(from_user_certificate, 1, t1, user_id.to_owned());
    p_assert_eq!(storage.get_timestamp_bounds(1).await.unwrap(), (t1, None));

    let t2 = timestamps.next();
    add!(from_revoked_user_certificate, 2, t2, user_id.to_owned());
    p_assert_eq!(
        storage.get_timestamp_bounds(1).await.unwrap(),
        (t1, Some(t2))
    );
    p_assert_eq!(storage.get_timestamp_bounds(2).await.unwrap(), (t2, None));

    let t3 = timestamps.next();
    add!(from_user_update_certificate, 3, t3, user_id.to_owned());
    p_assert_eq!(
        storage.get_timestamp_bounds(1).await.unwrap(),
        (t1, Some(t2))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(2).await.unwrap(),
        (t2, Some(t3))
    );
    p_assert_eq!(storage.get_timestamp_bounds(3).await.unwrap(), (t3, None));

    let t4 = timestamps.next();
    add!(from_device_certificate, 4, t4, device_id.to_owned());
    p_assert_eq!(
        storage.get_timestamp_bounds(1).await.unwrap(),
        (t1, Some(t2))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(2).await.unwrap(),
        (t2, Some(t3))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(3).await.unwrap(),
        (t3, Some(t4))
    );
    p_assert_eq!(storage.get_timestamp_bounds(4).await.unwrap(), (t4, None));

    let t5 = timestamps.next();
    add!(
        from_realm_role_certificate,
        5,
        t5,
        realm_id,
        user_id.to_owned()
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(1).await.unwrap(),
        (t1, Some(t2))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(2).await.unwrap(),
        (t2, Some(t3))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(3).await.unwrap(),
        (t3, Some(t4))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(4).await.unwrap(),
        (t4, Some(t5))
    );
    p_assert_eq!(storage.get_timestamp_bounds(5).await.unwrap(), (t5, None));

    let t6 = timestamps.next();
    add!(from_sequester_authority_certificate, 6, t6);
    p_assert_eq!(
        storage.get_timestamp_bounds(1).await.unwrap(),
        (t1, Some(t2))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(2).await.unwrap(),
        (t2, Some(t3))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(3).await.unwrap(),
        (t3, Some(t4))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(4).await.unwrap(),
        (t4, Some(t5))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(5).await.unwrap(),
        (t5, Some(t6))
    );
    p_assert_eq!(storage.get_timestamp_bounds(6).await.unwrap(), (t6, None));

    let t7 = timestamps.next();
    add!(from_sequester_service_certificate, 7, t7, service_id);
    p_assert_eq!(
        storage.get_timestamp_bounds(1).await.unwrap(),
        (t1, Some(t2))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(2).await.unwrap(),
        (t2, Some(t3))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(3).await.unwrap(),
        (t3, Some(t4))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(4).await.unwrap(),
        (t4, Some(t5))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(5).await.unwrap(),
        (t5, Some(t6))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(6).await.unwrap(),
        (t6, Some(t7))
    );
    p_assert_eq!(storage.get_timestamp_bounds(7).await.unwrap(), (t7, None));

    // Finally test against index too high (this time with certificates present)
    p_assert_matches!(
        storage.get_timestamp_bounds(10).await,
        Err(GetTimestampBoundsError::NonExisting)
    );
}

#[parsec_test(testbed = "minimal")]
async fn get_certificate(mut timestamps: TimestampGenerator, env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let storage = CertificatesStorage::start(&env.discriminant_dir, &alice)
        .await
        .unwrap();

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
    assert_not_in_db!(user_realm_role_certificates, user_id.to_owned()).await;
    assert_not_in_db!(sequester_authority_certificate).await;
    assert_not_in_db!(sequester_service_certificates).await;
    assert_not_in_db!(sequester_service_certificate, service_id1).await;

    // 2) ...now populate with certificates...

    macro_rules! add {
        ($meth:ident, $index:expr, $timestamp:expr, $encrypted:expr $(,$args:expr)*) => {
            async {
                let data = AddCertificateData::$meth(
                    $encrypted.to_vec(),
                    $timestamp,
                    $($args,)*
                );
                storage.add_next_certificate($index, data).await.unwrap();
            }
        };
    }

    let t1 = timestamps.next();
    add!(from_user_certificate, 1, t1, b"user", user_id.to_owned()).await;

    let t2 = timestamps.next();
    add!(
        from_revoked_user_certificate,
        2,
        t2,
        b"revoked_user",
        user_id.to_owned()
    )
    .await;

    let t3 = timestamps.next();
    add!(
        from_user_update_certificate,
        3,
        t3,
        b"user_update1",
        user_id.to_owned()
    )
    .await;

    let t4 = timestamps.next();
    add!(
        from_user_update_certificate,
        4,
        t4,
        b"user_update2",
        user_id.to_owned()
    )
    .await;

    let t5 = timestamps.next();
    add!(
        from_user_update_certificate,
        5,
        t5,
        b"other_user_update",
        other_user_id.to_owned()
    )
    .await;

    let t6 = timestamps.next();
    add!(
        from_device_certificate,
        6,
        t6,
        b"device",
        device_id.to_owned()
    )
    .await;

    let t7 = timestamps.next();
    add!(
        from_realm_role_certificate,
        7,
        t7,
        b"realm_role1",
        realm_id,
        user_id.to_owned()
    )
    .await;

    let t8 = timestamps.next();
    add!(
        from_realm_role_certificate,
        8,
        t8,
        b"realm_role2",
        realm_id,
        user_id.to_owned()
    )
    .await;

    let t9 = timestamps.next();
    add!(
        from_realm_role_certificate,
        9,
        t9,
        b"other_realm_role",
        other_realm_id,
        user_id.to_owned()
    )
    .await;

    let t10 = timestamps.next();
    add!(
        from_realm_role_certificate,
        10,
        t10,
        b"realm_role_other_user",
        realm_id,
        other_user_id.to_owned()
    )
    .await;

    let t11 = timestamps.next();
    add!(
        from_sequester_authority_certificate,
        11,
        t11,
        b"sequester_authority"
    )
    .await;

    let t12 = timestamps.next();
    add!(
        from_sequester_service_certificate,
        12,
        t12,
        b"sequester_service1",
        service_id1
    )
    .await;

    let t13 = timestamps.next();
    add!(
        from_sequester_service_certificate,
        13,
        t13,
        b"sequester_service2",
        service_id2
    )
    .await;

    // 3) ...and access them !

    macro_rules! assert_one_in_db {
        ($query_meth:ident $(,$query_args:expr)* => $expected:expr) => {
            async {
                let query = GetCertificateQuery::$query_meth($($query_args, )*);

                let item = storage.get_certificate_encrypted(query.clone(), UpTo::Current).await.unwrap();
                p_assert_eq!(item, $expected);

                let items = storage.get_multiple_certificates_encrypted(query, UpTo::Current, None, None).await.unwrap();
                p_assert_eq!(items, vec![$expected]);
            }
        };
    }

    macro_rules! assert_multiple_in_db {
        ($query_meth:ident $(,$query_args:expr)* => $expected:expr) => {
            async {
                let query = GetCertificateQuery::$query_meth($($query_args, )*);

                let items = storage.get_multiple_certificates_encrypted(query.clone(), UpTo::Current, None, None).await.unwrap();
                p_assert_eq!(items, $expected);

                let expected_one = items.last().unwrap().to_owned();

                let last_item = storage.get_certificate_encrypted(query, UpTo::Current).await.unwrap();
                p_assert_eq!(last_item, expected_one);
            }
        };
    }

    assert_one_in_db!(user_certificate, user_id.to_owned() => (1, b"user".to_vec())).await;
    assert_one_in_db!(revoked_user_certificate, user_id.to_owned() => (2, b"revoked_user".to_vec())).await;
    assert_multiple_in_db!(
        user_update_certificates, user_id.to_owned() =>
        vec![(3, b"user_update1".to_vec()), (4, b"user_update2".to_vec())]
    )
    .await;
    assert_one_in_db!(user_update_certificates, other_user_id.to_owned() => (5, b"other_user_update".to_vec())).await;
    assert_one_in_db!(device_certificate, device_id.to_owned() => (6, b"device".to_vec())).await;
    assert_multiple_in_db!(user_device_certificates, user_id.to_owned() =>
        vec![
            (6, b"device".to_vec())
        ]
    )
    .await;
    assert_multiple_in_db!(realm_role_certificates, realm_id.to_owned() =>
        vec![
            (7, b"realm_role1".to_vec()),
            (8, b"realm_role2".to_vec()),
            (10, b"realm_role_other_user".to_vec())
        ]
    )
    .await;
    assert_one_in_db!(realm_role_certificates, other_realm_id.to_owned() => (9, b"other_realm_role".to_vec())).await;
    assert_multiple_in_db!(user_realm_role_certificates, user_id.to_owned() =>
        vec![
            (7, b"realm_role1".to_vec()),
            (8, b"realm_role2".to_vec()),
            (9, b"other_realm_role".to_vec()),
        ]
    )
    .await;
    assert_one_in_db!(user_realm_role_certificates, other_user_id.to_owned() =>
        (10, b"realm_role_other_user".to_vec())
    )
    .await;
    assert_one_in_db!(sequester_authority_certificate => (11, b"sequester_authority".to_vec()))
        .await;
    assert_multiple_in_db!(sequester_service_certificates =>
        vec![
            (12, b"sequester_service1".to_vec()),
            (13, b"sequester_service2".to_vec())
        ]
    )
    .await;
    assert_one_in_db!(sequester_service_certificate, service_id1 => (12, b"sequester_service1".to_vec())).await;

    // 4) Ensure `up_to` filter param is working
    // Note we don't test every possible types here: this is because we have already
    // tested at step 3) that each type correctly gets turned into the right
    // `GetCertificateData` object (itself fed into the type agnostic get certificate
    // methods)

    let query = GetCertificateQuery::user_certificate(user_id.to_owned());
    let item = storage
        .get_certificate_encrypted(query.clone(), UpTo::Index(0))
        .await;
    p_assert_matches!(
        item,
        Err(GetCertificateError::ExistButTooRecent{certificate_index, certificate_timestamp,})
        if certificate_index == 1 && certificate_timestamp == "2020-01-01T00:00:00Z".parse().unwrap()
    );
    let items = storage
        .get_multiple_certificates_encrypted(query, UpTo::Index(0), None, None)
        .await
        .unwrap();
    assert!(items.is_empty());

    let query = GetCertificateQuery::realm_role_certificates(realm_id.to_owned());
    let item = storage
        .get_certificate_encrypted(query.clone(), UpTo::Index(8))
        .await
        .unwrap();
    p_assert_eq!(item, (8, b"realm_role2".to_vec()));
    let items = storage
        .get_multiple_certificates_encrypted(query, UpTo::Index(9), None, None)
        .await
        .unwrap();
    p_assert_eq!(
        items,
        vec![(7, b"realm_role1".to_vec()), (8, b"realm_role2".to_vec()),]
    );

    // 5) Test offset & limit

    let query = GetCertificateQuery::realm_role_certificates(realm_id.to_owned());
    p_assert_eq!(
        storage
            .get_multiple_certificates_encrypted(query.clone(), UpTo::Current, Some(3), None)
            .await
            .unwrap(),
        vec![]
    );
    p_assert_eq!(
        storage
            .get_multiple_certificates_encrypted(query.clone(), UpTo::Current, Some(2), None)
            .await
            .unwrap(),
        vec![(10, b"realm_role_other_user".to_vec())]
    );
    p_assert_eq!(
        storage
            .get_multiple_certificates_encrypted(query.clone(), UpTo::Current, Some(1), Some(1))
            .await
            .unwrap(),
        vec![(8, b"realm_role2".to_vec())],
    );
    p_assert_eq!(
        storage
            .get_multiple_certificates_encrypted(query, UpTo::Current, None, Some(2))
            .await
            .unwrap(),
        vec![(7, b"realm_role1".to_vec()), (8, b"realm_role2".to_vec())],
    );
}

#[parsec_test(testbed = "minimal")]
async fn add_already_existing_index(mut timestamps: TimestampGenerator, env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let storage = CertificatesStorage::start(&env.discriminant_dir, &alice)
        .await
        .unwrap();

    let user_id = alice.device_id.user_id();
    let device_id = &alice.device_id;
    let realm_id = VlobID::from_hex("fe15ca8ad55140d08b4951f26f7073d5").unwrap();
    let service_id = SequesterServiceID::from_hex("a065170f3b6649f997ec14dbe36c5c13").unwrap();

    let t1 = timestamps.next();
    let t2 = timestamps.next();

    // Initial certificate...

    macro_rules! add_with_always_same_index {
        ($meth:ident, $timestamp:expr $(,$args:expr)*) => {
            async {
                let data = AddCertificateData::$meth(
                    b"<encrypted>".to_vec(),
                    $timestamp,
                    $($args,)*
                );
                let already_used_index = 1;
                storage.add_next_certificate(already_used_index, data).await
            }
        };
    }

    add_with_always_same_index!(from_user_certificate, t1, user_id.to_owned())
        .await
        .unwrap();

    // ...and now all subsequent certificates wants to take it index !

    macro_rules! add_check_bad_outcome {
        ($meth:ident $(,$args:expr)*) => {
            async {
                let outcome = add_with_always_same_index!($meth, t2 $(,$args)*).await;
                if !matches!(outcome, Err(anyhow::Error { .. })) {
                    println!("outcome: {:?}", outcome);
                    assert!(false);
                }
            }
        };
    }

    add_check_bad_outcome!(from_user_certificate, user_id.to_owned()).await;
    add_check_bad_outcome!(from_revoked_user_certificate, user_id.to_owned()).await;
    add_check_bad_outcome!(from_user_update_certificate, user_id.to_owned()).await;
    add_check_bad_outcome!(from_device_certificate, device_id.to_owned()).await;
    add_check_bad_outcome!(from_realm_role_certificate, realm_id, user_id.to_owned()).await;
    add_check_bad_outcome!(from_sequester_authority_certificate).await;
    add_check_bad_outcome!(from_sequester_service_certificate, service_id).await;
}

#[parsec_test(testbed = "minimal")]
async fn forget_all_certificates(mut timestamps: TimestampGenerator, env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let storage = CertificatesStorage::start(&env.discriminant_dir, &alice)
        .await
        .unwrap();

    macro_rules! add {
        ($meth:ident, $index:expr, $timestamp:expr $(,$args:expr)*) => {
            let data = AddCertificateData::$meth(
                b"<encrypted>".to_vec(),
                $timestamp,
                $($args,)*
            );
            storage.add_next_certificate($index, data).await.unwrap();
        };
    }

    let user_id = alice.device_id.user_id();
    let device_id = &alice.device_id;
    let realm_id = VlobID::from_hex("fe15ca8ad55140d08b4951f26f7073d5").unwrap();
    let service_id = SequesterServiceID::from_hex("a065170f3b6649f997ec14dbe36c5c13").unwrap();

    // Add certificates

    let t1 = timestamps.next();
    add!(from_user_certificate, 1, t1, user_id.to_owned());
    let t2 = timestamps.next();
    add!(from_revoked_user_certificate, 2, t2, user_id.to_owned());
    let t3 = timestamps.next();
    add!(from_user_update_certificate, 3, t3, user_id.to_owned());
    let t4 = timestamps.next();
    add!(from_device_certificate, 4, t4, device_id.to_owned());
    let t5 = timestamps.next();
    add!(
        from_realm_role_certificate,
        5,
        t5,
        realm_id,
        user_id.to_owned()
    );
    let t6 = timestamps.next();
    add!(from_sequester_authority_certificate, 6, t6);
    let t7 = timestamps.next();
    add!(from_sequester_service_certificate, 7, t7, service_id);

    let (last_index, _) = storage.get_last_index().await.unwrap().unwrap();

    // Now drop the certificates...
    storage.forget_all_certificates().await.unwrap();

    // ...and make sure this is the case !

    p_assert_eq!(storage.get_last_index().await.unwrap(), None);

    for index in 1..=last_index {
        p_assert_matches!(
            storage.get_timestamp_bounds(index).await,
            Err(GetTimestampBoundsError::NonExisting)
        );
        p_assert_eq!(storage.test_get_raw_certificate(index).await.unwrap(), None);
    }

    async fn assert_not_in_db(storage: &CertificatesStorage, query: GetCertificateQuery) {
        p_assert_matches!(
            storage
                .get_certificate_encrypted(query, UpTo::Current)
                .await,
            Err(GetCertificateError::NonExisting)
        );
    }

    assert_not_in_db(
        &storage,
        GetCertificateQuery::user_certificate(user_id.to_owned()),
    )
    .await;
    assert_not_in_db(
        &storage,
        GetCertificateQuery::revoked_user_certificate(user_id.to_owned()),
    )
    .await;
    assert_not_in_db(
        &storage,
        GetCertificateQuery::user_update_certificates(user_id.to_owned()),
    )
    .await;
    assert_not_in_db(
        &storage,
        GetCertificateQuery::device_certificate(device_id.to_owned()),
    )
    .await;
    assert_not_in_db(
        &storage,
        GetCertificateQuery::user_device_certificates(user_id.to_owned()),
    )
    .await;
    assert_not_in_db(
        &storage,
        GetCertificateQuery::realm_role_certificates(realm_id.to_owned()),
    )
    .await;
    assert_not_in_db(
        &storage,
        GetCertificateQuery::user_realm_role_certificates(user_id.to_owned()),
    )
    .await;
    assert_not_in_db(
        &storage,
        GetCertificateQuery::sequester_authority_certificate(),
    )
    .await;
    assert_not_in_db(
        &storage,
        GetCertificateQuery::sequester_service_certificates(),
    )
    .await;
}

#[parsec_test(testbed = "minimal")]
async fn prohibit_skipping_indexes(mut timestamps: TimestampGenerator, env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let storage = CertificatesStorage::start(&env.discriminant_dir, &alice)
        .await
        .unwrap();

    let t1 = timestamps.next();
    let data = AddCertificateData::from_user_certificate(
        b"<encrypted>".to_vec(),
        t1,
        alice.device_id.user_id().clone(),
    );

    // Try with no certificates skipping 1 index
    storage.add_next_certificate(2, data).await.unwrap_err();

    let t2 = timestamps.next();
    let data = AddCertificateData::from_user_certificate(
        b"<encrypted>".to_vec(),
        t2,
        alice.device_id.user_id().clone(),
    );

    // Valid
    storage.add_next_certificate(1, data).await.unwrap();

    let t3 = timestamps.next();
    let data = AddCertificateData::from_user_certificate(
        b"<encrypted>".to_vec(),
        t3,
        alice.device_id.user_id().clone(),
    );

    // Try with certificate skipping 1 index
    storage.add_next_certificate(3, data).await.unwrap_err();

    let t4 = timestamps.next();
    let data = AddCertificateData::from_user_certificate(
        b"<encrypted>".to_vec(),
        t4,
        alice.device_id.user_id().clone(),
    );

    // Try with previous index
    storage.add_next_certificate(1, data).await.unwrap_err();
}
