// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_platform_storage::certificates::CertificatesStorage;
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
            storage
                .$meth(
                    $index,
                    $timestamp,
                    $($args,)*
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();

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
    add_and_check!(add_user_certificate, 1, t1, user_id.to_owned());

    let t2 = timestamps.next();
    add_and_check!(add_revoked_user_certificate, 2, t2, user_id.to_owned());

    let t3 = timestamps.next();
    add_and_check!(add_user_update_certificate, 3, t3, user_id.to_owned());

    let t4 = timestamps.next();
    add_and_check!(add_device_certificate, 4, t4, device_id.to_owned());

    let t5 = timestamps.next();
    add_and_check!(
        add_realm_role_certificate,
        5,
        t5,
        realm_id,
        user_id.to_owned()
    );

    let t6 = timestamps.next();
    add_and_check!(add_sequester_authority_certificate, 6, t6);

    let t7 = timestamps.next();
    add_and_check!(add_sequester_service_certificate, 7, t7, service_id);
}

#[parsec_test(testbed = "minimal")]
async fn get_timestamp_bounds(mut timestamps: TimestampGenerator, env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let storage = CertificatesStorage::start(&env.discriminant_dir, &alice)
        .await
        .unwrap();

    // Try with no certificates in the database...

    p_assert_eq!(storage.get_timestamp_bounds(1).await.unwrap(), (None, None));
    p_assert_eq!(
        storage.get_timestamp_bounds(10).await.unwrap(),
        (None, None)
    );

    // Now with certificates

    macro_rules! add {
        ($meth:ident, $index:expr, $timestamp:expr $(,$args:expr)*) => {
            storage
                .$meth(
                    $index,
                    $timestamp,
                    $($args,)*
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();
        };
    }

    let user_id = alice.device_id.user_id();
    let device_id = &alice.device_id;
    let realm_id = VlobID::from_hex("fe15ca8ad55140d08b4951f26f7073d5").unwrap();
    let service_id = SequesterServiceID::from_hex("a065170f3b6649f997ec14dbe36c5c13").unwrap();

    let t1 = timestamps.next();
    add!(add_user_certificate, 1, t1, user_id.to_owned());
    p_assert_eq!(
        storage.get_timestamp_bounds(1).await.unwrap(),
        (Some(t1), None)
    );

    let t2 = timestamps.next();
    add!(add_revoked_user_certificate, 2, t2, user_id.to_owned());
    p_assert_eq!(
        storage.get_timestamp_bounds(1).await.unwrap(),
        (Some(t1), Some(t2))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(2).await.unwrap(),
        (Some(t2), None)
    );

    let t3 = timestamps.next();
    add!(add_user_update_certificate, 3, t3, user_id.to_owned());
    p_assert_eq!(
        storage.get_timestamp_bounds(1).await.unwrap(),
        (Some(t1), Some(t2))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(2).await.unwrap(),
        (Some(t2), Some(t3))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(3).await.unwrap(),
        (Some(t3), None)
    );

    let t4 = timestamps.next();
    add!(add_device_certificate, 4, t4, device_id.to_owned());
    p_assert_eq!(
        storage.get_timestamp_bounds(1).await.unwrap(),
        (Some(t1), Some(t2))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(2).await.unwrap(),
        (Some(t2), Some(t3))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(3).await.unwrap(),
        (Some(t3), Some(t4))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(4).await.unwrap(),
        (Some(t4), None)
    );

    let t5 = timestamps.next();
    add!(
        add_realm_role_certificate,
        5,
        t5,
        realm_id,
        user_id.to_owned()
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(1).await.unwrap(),
        (Some(t1), Some(t2))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(2).await.unwrap(),
        (Some(t2), Some(t3))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(3).await.unwrap(),
        (Some(t3), Some(t4))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(4).await.unwrap(),
        (Some(t4), Some(t5))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(5).await.unwrap(),
        (Some(t5), None)
    );

    let t6 = timestamps.next();
    add!(add_sequester_authority_certificate, 6, t6);
    p_assert_eq!(
        storage.get_timestamp_bounds(1).await.unwrap(),
        (Some(t1), Some(t2))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(2).await.unwrap(),
        (Some(t2), Some(t3))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(3).await.unwrap(),
        (Some(t3), Some(t4))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(4).await.unwrap(),
        (Some(t4), Some(t5))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(5).await.unwrap(),
        (Some(t5), Some(t6))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(6).await.unwrap(),
        (Some(t6), None)
    );

    let t7 = timestamps.next();
    add!(add_sequester_service_certificate, 7, t7, service_id);
    p_assert_eq!(
        storage.get_timestamp_bounds(1).await.unwrap(),
        (Some(t1), Some(t2))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(2).await.unwrap(),
        (Some(t2), Some(t3))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(3).await.unwrap(),
        (Some(t3), Some(t4))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(4).await.unwrap(),
        (Some(t4), Some(t5))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(5).await.unwrap(),
        (Some(t5), Some(t6))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(6).await.unwrap(),
        (Some(t6), Some(t7))
    );
    p_assert_eq!(
        storage.get_timestamp_bounds(7).await.unwrap(),
        (Some(t7), None)
    );

    // Finally test against index too high (this time with certificates present)
    p_assert_eq!(
        storage.get_timestamp_bounds(10).await.unwrap(),
        (None, None)
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

    // Try with no certificates in the database...

    p_assert_eq!(storage.get_certificate(1).await.unwrap(), None);
    p_assert_eq!(
        storage
            .get_user_certificate(user_id.to_owned())
            .await
            .unwrap(),
        None
    );
    p_assert_eq!(
        storage
            .get_revoked_user_certificate(user_id.to_owned())
            .await
            .unwrap(),
        None
    );
    p_assert_eq!(
        storage
            .get_user_update_certificates(user_id.to_owned())
            .await
            .unwrap(),
        vec![]
    );
    p_assert_eq!(
        storage
            .get_device_certificate(device_id.to_owned())
            .await
            .unwrap(),
        None
    );
    p_assert_eq!(
        storage
            .get_realm_certificates(realm_id.to_owned())
            .await
            .unwrap(),
        vec![]
    );
    p_assert_eq!(
        storage
            .get_user_realms_certificates(user_id.to_owned())
            .await
            .unwrap(),
        vec![]
    );
    p_assert_eq!(
        storage.get_sequester_authority_certificate().await.unwrap(),
        None
    );
    p_assert_eq!(
        storage.get_sequester_service_certificates().await.unwrap(),
        vec![]
    );

    // ...now populate with certificates...

    let t1 = timestamps.next();
    storage
        .add_user_certificate(1, t1, user_id.to_owned(), b"user".to_vec())
        .await
        .unwrap();

    let t2 = timestamps.next();
    storage
        .add_revoked_user_certificate(2, t2, user_id.to_owned(), b"revoked_user".to_vec())
        .await
        .unwrap();

    let t3 = timestamps.next();
    storage
        .add_user_update_certificate(3, t3, user_id.to_owned(), b"user_update1".to_vec())
        .await
        .unwrap();

    let t4 = timestamps.next();
    storage
        .add_user_update_certificate(4, t4, user_id.to_owned(), b"user_update2".to_vec())
        .await
        .unwrap();

    let t5 = timestamps.next();
    storage
        .add_user_update_certificate(
            5,
            t5,
            other_user_id.to_owned(),
            b"other_user_update".to_vec(),
        )
        .await
        .unwrap();

    let t6 = timestamps.next();
    storage
        .add_device_certificate(6, t6, device_id.to_owned(), b"device".to_vec())
        .await
        .unwrap();

    let t7 = timestamps.next();
    storage
        .add_realm_role_certificate(7, t7, realm_id, user_id.to_owned(), b"realm_role1".to_vec())
        .await
        .unwrap();

    let t8 = timestamps.next();
    storage
        .add_realm_role_certificate(8, t8, realm_id, user_id.to_owned(), b"realm_role2".to_vec())
        .await
        .unwrap();

    let t9 = timestamps.next();
    storage
        .add_realm_role_certificate(
            9,
            t9,
            other_realm_id,
            user_id.to_owned(),
            b"other_realm_role".to_vec(),
        )
        .await
        .unwrap();

    let t10 = timestamps.next();
    storage
        .add_realm_role_certificate(
            10,
            t10,
            realm_id,
            other_user_id.to_owned(),
            b"realm_role_other_user".to_vec(),
        )
        .await
        .unwrap();

    let t11 = timestamps.next();
    storage
        .add_sequester_authority_certificate(11, t11, b"sequester_authority".to_vec())
        .await
        .unwrap();

    let t12 = timestamps.next();
    storage
        .add_sequester_service_certificate(12, t12, service_id1, b"sequester_service1".to_vec())
        .await
        .unwrap();

    let t13 = timestamps.next();
    storage
        .add_sequester_service_certificate(13, t13, service_id2, b"sequester_service2".to_vec())
        .await
        .unwrap();

    // ...and access them !

    p_assert_eq!(
        storage.get_certificate(1).await.unwrap(),
        Some(b"user".to_vec())
    );
    p_assert_eq!(storage.get_certificate(42).await.unwrap(), None);
    p_assert_eq!(
        storage
            .get_user_certificate(user_id.to_owned())
            .await
            .unwrap(),
        Some((1, b"user".to_vec()))
    );
    p_assert_eq!(
        storage
            .get_revoked_user_certificate(user_id.to_owned())
            .await
            .unwrap(),
        Some((2, b"revoked_user".to_vec()))
    );
    p_assert_eq!(
        storage
            .get_user_update_certificates(user_id.to_owned())
            .await
            .unwrap(),
        vec![(3, b"user_update1".to_vec()), (4, b"user_update2".to_vec())]
    );
    p_assert_eq!(
        storage
            .get_user_update_certificates(other_user_id.to_owned())
            .await
            .unwrap(),
        vec![(5, b"other_user_update".to_vec()),]
    );
    p_assert_eq!(
        storage
            .get_device_certificate(device_id.to_owned())
            .await
            .unwrap(),
        Some((6, b"device".to_vec()))
    );
    p_assert_eq!(
        storage
            .get_realm_certificates(realm_id.to_owned())
            .await
            .unwrap(),
        vec![
            (7, b"realm_role1".to_vec()),
            (8, b"realm_role2".to_vec()),
            (10, b"realm_role_other_user".to_vec())
        ]
    );
    p_assert_eq!(
        storage
            .get_realm_certificates(other_realm_id.to_owned())
            .await
            .unwrap(),
        vec![(9, b"other_realm_role".to_vec()),]
    );
    p_assert_eq!(
        storage
            .get_user_realms_certificates(user_id.to_owned())
            .await
            .unwrap(),
        vec![
            (7, b"realm_role1".to_vec()),
            (8, b"realm_role2".to_vec()),
            (9, b"other_realm_role".to_vec()),
        ]
    );
    p_assert_eq!(
        storage
            .get_user_realms_certificates(other_user_id.to_owned())
            .await
            .unwrap(),
        vec![(10, b"realm_role_other_user".to_vec())]
    );
    p_assert_eq!(
        storage.get_sequester_authority_certificate().await.unwrap(),
        Some((11, b"sequester_authority".to_vec()))
    );
    p_assert_eq!(
        storage.get_sequester_service_certificates().await.unwrap(),
        vec![
            (12, b"sequester_service1".to_vec()),
            (13, b"sequester_service2".to_vec())
        ]
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
    storage
        .add_user_certificate(1, t1, user_id.to_owned(), b"user".to_vec())
        .await
        .unwrap();

    // ...and now all subsequent certificates wants to take it index !

    macro_rules! check_bad_outcome {
        ($outcome:expr) => {
            if !matches!($outcome, Err(anyhow::Error { .. })) {
                println!("outcome: {:?}", $outcome);
                assert!(false);
            }
        };
    }

    let outcome = storage
        .add_user_certificate(1, t2, user_id.to_owned(), b"user".to_vec())
        .await;
    check_bad_outcome!(outcome);
    let outcome = storage
        .add_revoked_user_certificate(1, t2, user_id.to_owned(), b"revoked_user".to_vec())
        .await;
    check_bad_outcome!(outcome);
    let outcome = storage
        .add_user_update_certificate(1, t2, user_id.to_owned(), b"user_update".to_vec())
        .await;
    check_bad_outcome!(outcome);
    let outcome = storage
        .add_device_certificate(1, t2, device_id.to_owned(), b"device".to_vec())
        .await;
    check_bad_outcome!(outcome);
    let outcome = storage
        .add_realm_role_certificate(1, t2, realm_id, user_id.to_owned(), b"realm_role1".to_vec())
        .await;
    check_bad_outcome!(outcome);
    let outcome = storage
        .add_sequester_authority_certificate(1, t2, b"sequester_authority".to_vec())
        .await;
    check_bad_outcome!(outcome);
    let outcome = storage
        .add_sequester_service_certificate(1, t2, service_id, b"sequester_service".to_vec())
        .await;
    check_bad_outcome!(outcome);
}

#[parsec_test(testbed = "minimal")]
async fn forget_all_certificates(mut timestamps: TimestampGenerator, env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let storage = CertificatesStorage::start(&env.discriminant_dir, &alice)
        .await
        .unwrap();

    macro_rules! add {
        ($meth:ident, $index:expr, $timestamp:expr $(,$args:expr)*) => {
            storage
                .$meth(
                    $index,
                    $timestamp,
                    $($args,)*
                    b"<encrypted>".to_vec(),
                )
                .await
                .unwrap();
        };
    }

    let user_id = alice.device_id.user_id();
    let device_id = &alice.device_id;
    let realm_id = VlobID::from_hex("fe15ca8ad55140d08b4951f26f7073d5").unwrap();
    let service_id = SequesterServiceID::from_hex("a065170f3b6649f997ec14dbe36c5c13").unwrap();

    // Add certificates

    let t1 = timestamps.next();
    add!(add_user_certificate, 1, t1, user_id.to_owned());
    let t2 = timestamps.next();
    add!(add_revoked_user_certificate, 2, t2, user_id.to_owned());
    let t3 = timestamps.next();
    add!(add_user_update_certificate, 3, t3, user_id.to_owned());
    let t4 = timestamps.next();
    add!(add_device_certificate, 4, t4, device_id.to_owned());
    let t5 = timestamps.next();
    add!(
        add_realm_role_certificate,
        5,
        t5,
        realm_id,
        user_id.to_owned()
    );
    let t6 = timestamps.next();
    add!(add_sequester_authority_certificate, 6, t6);
    let t7 = timestamps.next();
    add!(add_sequester_service_certificate, 7, t7, service_id);

    let (last_index, _) = storage.get_last_index().await.unwrap().unwrap();

    // Now drop the certificates...
    storage.forget_all_certificates().await.unwrap();

    // ...and make sure this is the case !

    p_assert_eq!(storage.get_last_index().await.unwrap(), None);

    for index in 1..=last_index {
        p_assert_eq!(
            storage.get_timestamp_bounds(index).await.unwrap(),
            (None, None)
        );
        p_assert_eq!(storage.get_certificate(index).await.unwrap(), None);
    }
    p_assert_eq!(
        storage
            .get_user_certificate(user_id.to_owned())
            .await
            .unwrap(),
        None
    );
    p_assert_eq!(
        storage
            .get_revoked_user_certificate(user_id.to_owned())
            .await
            .unwrap(),
        None
    );
    p_assert_eq!(
        storage
            .get_user_update_certificates(user_id.to_owned())
            .await
            .unwrap(),
        vec![]
    );
    p_assert_eq!(
        storage
            .get_device_certificate(device_id.to_owned())
            .await
            .unwrap(),
        None
    );
    p_assert_eq!(
        storage
            .get_realm_certificates(realm_id.to_owned())
            .await
            .unwrap(),
        vec![]
    );
    p_assert_eq!(
        storage
            .get_user_realms_certificates(user_id.to_owned())
            .await
            .unwrap(),
        vec![]
    );
    p_assert_eq!(
        storage.get_sequester_authority_certificate().await.unwrap(),
        None
    );
    p_assert_eq!(
        storage.get_sequester_service_certificates().await.unwrap(),
        vec![]
    );
}
