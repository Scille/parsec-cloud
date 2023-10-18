// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certificates_ops::{AddCertificateError, InvalidCertificateError, MaybeRedactedSwitch};

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let vlob_id = VlobID::from_hex("f0000000-0000-0000-0000-000000000001").unwrap();
    let env = env.customize(|builder| {
        builder
            .new_user_realm("alice")
            .customize(|event| event.realm_id = vlob_id);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, alice_dev1_signed) = env.get_device_certificate("alice@dev1");
    let (_, alice_realm_role_signed) = env.get_last_realm_role_certificate("alice", vlob_id);

    let switch = ops
        .add_certificates_batch(
            &store,
            0,
            [alice_signed, alice_dev1_signed, alice_realm_role_signed].into_iter(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch);
}

#[parsec_test(testbed = "minimal")]
async fn content_already_exists(env: &TestbedEnv) {
    let vlob_id = VlobID::from_hex("f0000000-0000-0000-0000-000000000001").unwrap();
    let env = env.customize(|builder| {
        builder
            .new_user_realm("alice")
            .customize(|event| event.realm_id = vlob_id);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, alice_dev1_signed) = env.get_device_certificate("alice@dev1");
    let (_, alice_realm_role_signed) = env.get_last_realm_role_certificate("alice", vlob_id);

    let err = ops
        .add_certificates_batch(
            &store,
            0,
            [
                alice_signed,
                alice_dev1_signed,
                alice_realm_role_signed.clone(),
                alice_realm_role_signed,
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
    let vlob_id = VlobID::from_hex("f0000000-0000-0000-0000-000000000001").unwrap();
    let env = env.customize(|builder| {
        builder
            .new_user_realm("alice")
            .customize(|event| event.realm_id = vlob_id);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, alice_realm_role_signed) = env.get_last_realm_role_certificate("alice", vlob_id);

    ops.add_certificates_batch(&store, 0, [alice_signed].into_iter())
        .await
        .unwrap();

    let err = ops
        .add_certificates_batch(&store, 0, [alice_realm_role_signed].into_iter())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        AddCertificateError::InvalidCertificate(InvalidCertificateError::IndexAlreadyExists { .. })
    );
}

#[parsec_test(testbed = "minimal")]
async fn invalid_timestamp(env: &TestbedEnv) {
    let vlob_id = VlobID::from_hex("f0000000-0000-0000-0000-000000000001").unwrap();
    let env = env.customize(|builder| {
        builder.new_user_realm("alice").customize(|event| {
            event.realm_id = vlob_id;
            event.timestamp = DateTime::from_ymd_hms_us(1999, 1, 1, 0, 0, 0, 0).unwrap()
        });
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, alice_dev1_signed) = env.get_device_certificate("alice@dev1");
    let (_, old_alice_realm_role_signed) = env.get_last_realm_role_certificate("alice", vlob_id);

    let err = ops
        .add_certificates_batch(
            &store,
            0,
            [alice_signed, alice_dev1_signed, old_alice_realm_role_signed].into_iter(),
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
    let vlob_id = VlobID::from_hex("f0000000-0000-0000-0000-000000000001").unwrap();
    let env = env.customize(|builder| {
        builder
            .new_user_realm("alice")
            .customize(|event| event.realm_id = vlob_id);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_realm_role_signed) = env.get_last_realm_role_certificate("alice", vlob_id);

    let err = ops
        .add_certificates_batch(&store, 0, [alice_realm_role_signed].into_iter())
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        AddCertificateError::InvalidCertificate(InvalidCertificateError::NonExistingAuthor { .. })
    );
}

#[parsec_test(testbed = "minimal")]
async fn revoked(env: &TestbedEnv) {
    let vlob_id = VlobID::from_hex("f0000000-0000-0000-0000-000000000001").unwrap();
    let env = env.customize(|builder| {
        builder
            .new_user("bob")
            .with_initial_profile(UserProfile::Admin);
        builder.revoke_user("bob");
        builder
            // we step over the checker
            .new_user_realm("alice")
            .customize(|event| event.realm_id = vlob_id)
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
    let (_, bob_realm_role_signed) = env.get_last_realm_role_certificate("bob", vlob_id);

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
                bob_realm_role_signed,
            ]
            .into_iter(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        AddCertificateError::InvalidCertificate(InvalidCertificateError::RelatedUserAlreadyRevoked { user_revoked_on, .. })
        if user_revoked_on == DateTime::from_ymd_hms_us(2000, 1, 4, 0, 0, 0, 0).unwrap()
    );
}

#[parsec_test(testbed = "minimal")]
#[case(UserProfile::Standard)]
#[case(UserProfile::Outsider)]
async fn not_admin(#[case] profile: UserProfile, env: &TestbedEnv) {
    let vlob_id = VlobID::from_hex("f0000000-0000-0000-0000-000000000001").unwrap();
    let env = env.customize(|builder| {
        builder.new_user("bob").with_initial_profile(profile);
        builder
            .new_user_realm("bob")
            .customize(|event| event.realm_id = vlob_id);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, alice_dev1_signed) = env.get_device_certificate("alice@dev1");
    let (_, bob_signed) = env.get_user_certificate("bob");
    let (_, bob_dev1_signed) = env.get_device_certificate("bob@dev1");
    let (_, bob_realm_role_signed) = env.get_last_realm_role_certificate("bob", vlob_id);

    let switch = ops
        .add_certificates_batch(
            &store,
            0,
            [
                alice_signed,
                alice_dev1_signed,
                bob_signed,
                bob_dev1_signed,
                bob_realm_role_signed,
            ]
            .into_iter(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch,);
}

#[parsec_test(testbed = "minimal")]
async fn authored_by_root(env: &TestbedEnv) {
    let vlob_id = VlobID::from_hex("f0000000-0000-0000-0000-000000000001").unwrap();
    let env = env.customize(|builder| {
        builder
            .new_user_realm("alice")
            .customize(|event| event.realm_id = vlob_id);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, alice_dev1_signed) = env.get_device_certificate("alice@dev1");
    let (alice_realm_role_certif, _) = env.get_last_realm_role_certificate("alice", vlob_id);

    let mut alice_realm_role_certif_authored_by_root = (*alice_realm_role_certif).clone();
    alice_realm_role_certif_authored_by_root.author = CertificateSignerOwned::Root;
    let alice_realm_role_signed_authored_by_root =
        Bytes::from(alice_realm_role_certif_authored_by_root.dump_and_sign(&alice.signing_key));

    let err = ops
        .add_certificates_batch(
            &store,
            0,
            [
                alice_signed,
                alice_dev1_signed,
                alice_realm_role_signed_authored_by_root,
            ]
            .into_iter(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        AddCertificateError::InvalidCertificate(InvalidCertificateError::Corrupted { .. })
    )
}

#[parsec_test(testbed = "minimal")]
async fn not_signed_by_self(env: &TestbedEnv) {
    let vlob_id = VlobID::from_hex("f0000000-0000-0000-0000-000000000001").unwrap();
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder
            .new_user_realm("bob")
            .customize(|event| event.realm_id = vlob_id);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, alice_dev1_signed) = env.get_device_certificate("alice@dev1");
    let (_, bob_signed) = env.get_user_certificate("bob");
    let (_, bob_dev1_signed) = env.get_device_certificate("bob@dev1");
    let (bob_realm_role_certif, _) = env.get_last_realm_role_certificate("bob", vlob_id);

    let mut bob_realm_role_certif_authored_by_alice = (*bob_realm_role_certif).clone();
    bob_realm_role_certif_authored_by_alice.author =
        CertificateSignerOwned::User("alice@dev1".try_into().unwrap());
    let bob_realm_role_signed_authored_by_alice =
        Bytes::from(bob_realm_role_certif_authored_by_alice.dump_and_sign(&alice.signing_key));

    let err = ops
        .add_certificates_batch(
            &store,
            0,
            [
                alice_signed,
                alice_dev1_signed,
                bob_signed,
                bob_dev1_signed,
                bob_realm_role_signed_authored_by_alice,
            ]
            .into_iter(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        AddCertificateError::InvalidCertificate(
            InvalidCertificateError::RealmFirstRoleMustBeSelfSigned { .. }
        )
    )
}

#[parsec_test(testbed = "minimal")]
#[case(Some(RealmRole::Manager))]
#[case(Some(RealmRole::Contributor))]
#[case(Some(RealmRole::Reader))]
#[case(None)]
async fn not_signed_by_owner(#[case] role: Option<RealmRole>, env: &TestbedEnv) {
    let vlob_id = VlobID::from_hex("f0000000-0000-0000-0000-000000000001").unwrap();
    let env = env.customize(|builder| {
        builder
            .new_user_realm("alice")
            .customize(|event| event.realm_id = vlob_id);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, alice_dev1_signed) = env.get_device_certificate("alice@dev1");
    let (alice_realm_role_certif, _) = env.get_last_realm_role_certificate("alice", vlob_id);

    let mut alice_realm_role_certif = (*alice_realm_role_certif).clone();
    alice_realm_role_certif.role = role;
    let alice_realm_role_signed =
        Bytes::from(alice_realm_role_certif.dump_and_sign(&alice.signing_key));

    let err = ops
        .add_certificates_batch(
            &store,
            0,
            [alice_signed, alice_dev1_signed, alice_realm_role_signed].into_iter(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        AddCertificateError::InvalidCertificate(
            InvalidCertificateError::RealmFirstRoleMustBeOwner { .. }
        )
    )
}

#[parsec_test(testbed = "minimal")]
async fn share_with_no_role(env: &TestbedEnv) {
    let vlob_id = VlobID::from_hex("f0000000-0000-0000-0000-000000000001").unwrap();
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder
            .new_user_realm("alice")
            .customize(|event| event.realm_id = vlob_id)
            .then_share_with("bob", None);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, alice_dev1_signed) = env.get_device_certificate("alice@dev1");
    let (_, alice_realm_role_signed) = env.get_last_realm_role_certificate("alice", vlob_id);
    let (_, bob_realm_role_signed) = env.get_last_realm_role_certificate("bob", vlob_id);

    let err = ops
        .add_certificates_batch(
            &store,
            0,
            [
                alice_signed,
                alice_dev1_signed,
                alice_realm_role_signed,
                bob_realm_role_signed,
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
    )
}

#[parsec_test(testbed = "minimal")]
async fn same_realm_id(env: &TestbedEnv) {
    let vlob_id = VlobID::from_hex("f0000000-0000-0000-0000-000000000001").unwrap();
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.new_user("mallory");
        builder
            .new_user_realm("alice")
            .customize(|event| event.realm_id = vlob_id);
        builder
            .new_user_realm("bob")
            .customize(|event| event.realm_id = vlob_id);
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, alice_dev1_signed) = env.get_device_certificate("alice@dev1");
    let (_, bob_signed) = env.get_user_certificate("bob");
    let (_, bob_dev1_signed) = env.get_device_certificate("bob@dev1");
    let (_, alice_realm_role_signed) = env.get_last_realm_role_certificate("alice", vlob_id);
    let (_, bob_realm_role_signed) = env.get_last_realm_role_certificate("bob", vlob_id);

    let err = ops
        .add_certificates_batch(
            &store,
            0,
            [
                alice_signed,
                alice_dev1_signed,
                bob_signed,
                bob_dev1_signed,
                alice_realm_role_signed,
                bob_realm_role_signed,
            ]
            .into_iter(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        AddCertificateError::InvalidCertificate(
            InvalidCertificateError::RealmAuthorHasNoRole { .. }
        )
    )
}

#[parsec_test(testbed = "minimal")]
#[case(RealmRole::Contributor)]
#[case(RealmRole::Reader)]
async fn share_realm_with_outsider(#[case] role: RealmRole, env: &TestbedEnv) {
    let vlob_id = VlobID::from_hex("f0000000-0000-0000-0000-000000000001").unwrap();
    let env = env.customize(|builder| {
        builder
            .new_user("bob")
            .with_initial_profile(UserProfile::Outsider);
        builder
            .new_user_realm("alice")
            .customize(|event| event.realm_id = vlob_id)
            .then_share_with("bob", Some(role));
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, alice_dev1_signed) = env.get_device_certificate("alice@dev1");
    let (_, bob_signed) = env.get_user_certificate("bob");
    let (_, bob_dev1_signed) = env.get_device_certificate("bob@dev1");
    let (_, alice_realm_role_signed) = env.get_last_realm_role_certificate("alice", vlob_id);
    let (_, bob_realm_role_signed) = env.get_last_realm_role_certificate("bob", vlob_id);

    let switch = ops
        .add_certificates_batch(
            &store,
            0,
            [
                alice_signed,
                alice_dev1_signed,
                bob_signed,
                bob_dev1_signed,
                alice_realm_role_signed,
                bob_realm_role_signed,
            ]
            .into_iter(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch,)
}

#[parsec_test(testbed = "minimal")]
#[case(RealmRole::Owner)]
#[case(RealmRole::Manager)]
async fn share_realm_privileges_with_outsider(#[case] role: RealmRole, env: &TestbedEnv) {
    let vlob_id = VlobID::from_hex("f0000000-0000-0000-0000-000000000001").unwrap();
    let env = env.customize(|builder| {
        builder
            .new_user("bob")
            .with_initial_profile(UserProfile::Outsider);
        builder
            .new_user_realm("alice")
            .customize(|event| event.realm_id = vlob_id)
            .then_share_with("bob", Some(role));
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, alice_dev1_signed) = env.get_device_certificate("alice@dev1");
    let (_, bob_signed) = env.get_user_certificate("bob");
    let (_, bob_dev1_signed) = env.get_device_certificate("bob@dev1");
    let (_, alice_realm_role_signed) = env.get_last_realm_role_certificate("alice", vlob_id);
    let (_, bob_realm_role_signed) = env.get_last_realm_role_certificate("bob", vlob_id);

    let err = ops
        .add_certificates_batch(
            &store,
            0,
            [
                alice_signed,
                alice_dev1_signed,
                bob_signed,
                bob_dev1_signed,
                alice_realm_role_signed,
                bob_realm_role_signed,
            ]
            .into_iter(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        AddCertificateError::InvalidCertificate(
            InvalidCertificateError::RealmOutsiderCannotBeOwnerOrManager { .. }
        )
    )
}

#[parsec_test(testbed = "minimal")]
#[case(RealmRole::Owner)]
#[case(RealmRole::Manager)]
#[case(RealmRole::Contributor)]
#[case(RealmRole::Reader)]
async fn owner_giving_role(#[case] role: RealmRole, env: &TestbedEnv) {
    let vlob_id = VlobID::from_hex("f0000000-0000-0000-0000-000000000001").unwrap();
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.new_user("mallory");
        builder
            .new_user_realm("alice")
            .customize(|event| {
                event.realm_id = vlob_id;
            })
            .then_share_with("bob", Some(role));
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let store = ops.store.for_write().await;
    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, alice_dev1_signed) = env.get_device_certificate("alice@dev1");
    let (_, bob_signed) = env.get_user_certificate("bob");
    let (_, bob_dev1_signed) = env.get_device_certificate("bob@dev1");
    let (_, alice_realm_role_signed) = env.get_last_realm_role_certificate("alice", vlob_id);
    let (_, bob_realm_role_signed) = env.get_last_realm_role_certificate("bob", vlob_id);

    let switch = ops
        .add_certificates_batch(
            &store,
            0,
            [
                alice_signed,
                alice_dev1_signed,
                bob_signed,
                bob_dev1_signed,
                alice_realm_role_signed,
                bob_realm_role_signed,
            ]
            .into_iter(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch)
}

#[parsec_test(testbed = "minimal")]
#[case(RealmRole::Contributor)]
#[case(RealmRole::Reader)]
async fn manager_giving_role(#[case] role: RealmRole, env: &TestbedEnv) {
    let vlob_id = VlobID::from_hex("f0000000-0000-0000-0000-000000000001").unwrap();
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.new_user("mallory");
        builder
            .new_user_realm("alice")
            .customize(|event| event.realm_id = vlob_id);
        builder.share_realm(vlob_id, "bob", Some(RealmRole::Manager));
        builder
            .share_realm(vlob_id, "mallory", Some(role))
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
    let (_, mallory_dev1_signed) = env.get_device_certificate("mallory@dev1");
    let (_, alice_realm_role_signed) = env.get_last_realm_role_certificate("alice", vlob_id);
    let (_, bob_realm_role_signed) = env.get_last_realm_role_certificate("bob", vlob_id);
    let (_, mallory_realm_role_signed) = env.get_last_realm_role_certificate("mallory", vlob_id);

    let switch = ops
        .add_certificates_batch(
            &store,
            0,
            [
                alice_signed,
                alice_dev1_signed,
                bob_signed,
                bob_dev1_signed,
                mallory_signed,
                mallory_dev1_signed,
                alice_realm_role_signed,
                bob_realm_role_signed,
                mallory_realm_role_signed,
            ]
            .into_iter(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch)
}

#[parsec_test(testbed = "minimal")]
#[case(RealmRole::Owner)]
#[case(RealmRole::Manager)]
async fn manager_trying_to_give_admin_role(#[case] role: RealmRole, env: &TestbedEnv) {
    let vlob_id = VlobID::from_hex("f0000000-0000-0000-0000-000000000001").unwrap();
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.new_user("mallory");
        builder
            .new_user_realm("alice")
            .customize(|event| event.realm_id = vlob_id);
        builder.share_realm(vlob_id, "bob", Some(RealmRole::Manager));
        builder
            .share_realm(vlob_id, "mallory", Some(role))
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
    let (_, mallory_dev1_signed) = env.get_device_certificate("mallory@dev1");
    let (_, alice_realm_role_signed) = env.get_last_realm_role_certificate("alice", vlob_id);
    let (_, bob_realm_role_signed) = env.get_last_realm_role_certificate("bob", vlob_id);
    let (_, mallory_realm_role_signed) = env.get_last_realm_role_certificate("mallory", vlob_id);

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
                mallory_dev1_signed,
                alice_realm_role_signed,
                bob_realm_role_signed,
                mallory_realm_role_signed,
            ]
            .into_iter(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        AddCertificateError::InvalidCertificate(
            InvalidCertificateError::RealmAuthorNotOwner { .. }
        )
    )
}

#[parsec_test(testbed = "minimal")]
#[case(RealmRole::Contributor, RealmRole::Contributor)]
#[case(RealmRole::Contributor, RealmRole::Reader)]
#[case(RealmRole::Reader, RealmRole::Contributor)]
#[case(RealmRole::Reader, RealmRole::Reader)]
async fn member_trying_to_give_role(
    #[case] greeter_role: RealmRole,
    #[case] claimer_role: RealmRole,
    env: &TestbedEnv,
) {
    let vlob_id = VlobID::from_hex("f0000000-0000-0000-0000-000000000001").unwrap();
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.new_user("mallory");
        builder
            .new_user_realm("alice")
            .customize(|event| event.realm_id = vlob_id);
        builder.share_realm(vlob_id, "bob", Some(greeter_role));
        builder
            .share_realm(vlob_id, "mallory", Some(claimer_role))
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
    let (_, mallory_dev1_signed) = env.get_device_certificate("mallory@dev1");
    let (_, alice_realm_role_signed) = env.get_last_realm_role_certificate("alice", vlob_id);
    let (_, bob_realm_role_signed) = env.get_last_realm_role_certificate("bob", vlob_id);
    let (_, mallory_realm_role_signed) = env.get_last_realm_role_certificate("mallory", vlob_id);

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
                mallory_dev1_signed,
                alice_realm_role_signed,
                bob_realm_role_signed,
                mallory_realm_role_signed,
            ]
            .into_iter(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        AddCertificateError::InvalidCertificate(
            InvalidCertificateError::RealmAuthorNotOwnerOrManager { .. }
        )
    )
}
