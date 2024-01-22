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
        builder.new_user_realm("alice");
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
async fn content_already_exists(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        let realm_id = builder.new_realm("alice").map(|e| e.realm_id);
        builder.with_check_consistency_disabled(|builder| {
            builder.new_realm("alice").customize(|e| {
                e.realm_id = realm_id;
            });
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
        CertifAddCertificatesBatchError::InvalidCertificate(
            InvalidCertificateError::ContentAlreadyExists { .. }
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn older_than_author(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user_realm("alice").customize(|event| {
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
        CertifAddCertificatesBatchError::InvalidCertificate(InvalidCertificateError::OlderThanAuthor {
            author_created_on,
            ..
        })
        if author_created_on == DateTime::from_ymd_hms_us(2000, 1, 2, 0, 0, 0, 0).unwrap()
    );
}

#[parsec_test(testbed = "minimal")]
async fn invalid_timestamp(env: &TestbedEnv) {
    let (env, timestamp) = env.customize_with_map(|builder| {
        let realm_id = builder.new_realm("alice").map(|e| e.realm_id);
        let timestamp = builder.rotate_key_realm(realm_id).map(|e| e.timestamp);
        builder
            .share_realm(realm_id, "bob", RealmRole::Contributor)
            .customize(|event| {
                event.timestamp = timestamp;
            });
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
        CertifAddCertificatesBatchError::InvalidCertificate(InvalidCertificateError::InvalidTimestamp {
            last_certificate_timestamp,
            ..
        })
        if last_certificate_timestamp == timestamp
    );
}

// In practice the server ensures only certificates provided together (e.g. user and
// device certificates) can have the same timestamp.
// However current implementation of `CertificateOps` ignore this, so this test is
// mostly here to ensure we will get notified whenever this behavior change.
#[parsec_test(testbed = "minimal")]
async fn different_realms_can_have_same_timestamp(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        let timestamp = builder.new_realm("alice").map(|e| e.timestamp);
        builder.new_realm("alice").customize(|event| {
            event.timestamp = timestamp;
        });
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
async fn non_existing_author(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_realm("alice");
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let common_certificates = vec![
        env.get_user_certificate("alice").1,
        // Alice's device certificate is missing !
    ];

    let err = ops
        .add_certificates_batch(
            &common_certificates,
            &[],
            &[],
            &env.get_realms_certificates_signed(),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(
            InvalidCertificateError::NonExistingAuthor { .. }
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn revoked(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.revoke_user("bob");
        builder
            // TODO: check consistency can't be skipped
            .new_user_realm("alice")
            .with_author("bob@dev1".try_into().unwrap());
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
        CertifAddCertificatesBatchError::InvalidCertificate(InvalidCertificateError::RelatedUserAlreadyRevoked { user_revoked_on, .. })
        if user_revoked_on == DateTime::from_ymd_hms_us(2000, 1, 4, 0, 0, 0, 0).unwrap()
    );
}

#[parsec_test(testbed = "minimal")]
#[case(UserProfile::Standard)]
#[case(UserProfile::Outsider)]
async fn not_admin(#[case] profile: UserProfile, env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob").with_initial_profile(profile);
        builder.new_user_realm("bob");
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
async fn authored_by_root(env: &TestbedEnv) {
    let (env, realm_id) =
        env.customize_with_map(|builder| builder.new_user_realm("alice").map(|e| e.realm_id));
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let (alice_realm_role_certif, _) = env.get_last_realm_role_certificate("alice", realm_id);

    let mut alice_realm_role_certif_authored_by_root = (*alice_realm_role_certif).clone();
    alice_realm_role_certif_authored_by_root.author = CertificateSignerOwned::Root;
    let alice_realm_role_signed_authored_by_root =
        Bytes::from(alice_realm_role_certif_authored_by_root.dump_and_sign(&alice.signing_key));

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &HashMap::from([(realm_id, vec![alice_realm_role_signed_authored_by_root])]),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(
            InvalidCertificateError::Corrupted { .. }
        )
    )
}

#[parsec_test(testbed = "minimal")]
async fn initial_realm_role_not_signed_by_self(env: &TestbedEnv) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        builder.new_user("bob");
        builder.new_realm("bob").map(|e| e.realm_id)
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let (bob_realm_role_certif, _) = env.get_last_realm_role_certificate("bob", realm_id);

    let mut bob_realm_role_certif_authored_by_alice = (*bob_realm_role_certif).clone();
    bob_realm_role_certif_authored_by_alice.author =
        CertificateSignerOwned::User("alice@dev1".try_into().unwrap());
    let bob_realm_role_signed_authored_by_alice =
        Bytes::from(bob_realm_role_certif_authored_by_alice.dump_and_sign(&alice.signing_key));

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &HashMap::from([(realm_id, vec![bob_realm_role_signed_authored_by_alice])]),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(
            InvalidCertificateError::RealmFirstRoleMustBeSelfSigned { .. }
        )
    )
}

#[parsec_test(testbed = "minimal")]
#[case(Some(RealmRole::Manager))]
#[case(Some(RealmRole::Contributor))]
#[case(Some(RealmRole::Reader))]
#[case(None)]
async fn initial_realm_role_owner(#[case] role: Option<RealmRole>, env: &TestbedEnv) {
    let (env, realm_id) =
        env.customize_with_map(|builder| builder.new_user_realm("alice").map(|e| e.realm_id));
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let (alice_realm_role_certif, _) = env.get_last_realm_role_certificate("alice", realm_id);

    let mut alice_realm_role_certif = (*alice_realm_role_certif).clone();
    alice_realm_role_certif.role = role;
    let alice_realm_role_signed =
        Bytes::from(alice_realm_role_certif.dump_and_sign(&alice.signing_key));

    let err = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &HashMap::from([(realm_id, vec![alice_realm_role_signed])]),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifAddCertificatesBatchError::InvalidCertificate(
            InvalidCertificateError::RealmFirstRoleMustBeOwner { .. }
        )
    )
}

#[parsec_test(testbed = "minimal")]
async fn share_with_no_role(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|e| e.realm);
        builder.share_realm(realm_id, "bob", None);
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
        CertifAddCertificatesBatchError::InvalidCertificate(
            InvalidCertificateError::ContentAlreadyExists { .. }
        )
    )
}

#[parsec_test(testbed = "minimal")]
async fn same_realm_id(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.new_user("mallory");
        let realm_id = builder.new_user_realm("alice").map(|e| e.realm_id);
        builder
            .new_user_realm("bob")
            .customize(|event| event.realm_id = realm_id);
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
        CertifAddCertificatesBatchError::InvalidCertificate(
            InvalidCertificateError::RealmAuthorHasNoRole { .. }
        )
    )
}

#[parsec_test(testbed = "minimal")]
#[case(RealmRole::Contributor)]
#[case(RealmRole::Reader)]
async fn share_realm_with_outsider(#[case] role: RealmRole, env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder
            .new_user("bob")
            .with_initial_profile(UserProfile::Outsider);
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|e| e.realm);
        builder.share_realm(realm_id, "bob", Some(role));
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

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch,)
}

#[parsec_test(testbed = "minimal")]
#[case(RealmRole::Owner)]
#[case(RealmRole::Manager)]
async fn share_realm_privileges_with_outsider(#[case] role: RealmRole, env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder
            .new_user("bob")
            .with_initial_profile(UserProfile::Outsider);
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|e| e.realm);
        builder.share_realm(realm_id, "bob", Some(role));
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
        CertifAddCertificatesBatchError::InvalidCertificate(
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
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.new_user("mallory");
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|e| e.realm);
        builder.share_realm(realm_id, "bob", Some(role));
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

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch)
}

#[parsec_test(testbed = "minimal")]
#[case(RealmRole::Contributor)]
#[case(RealmRole::Reader)]
async fn manager_giving_role(#[case] role: RealmRole, env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.new_user("mallory");
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|e| e.realm);
        builder
            .share_realm(realm_id, "bob", Some(RealmRole::Manager))
            .then_also_share_with("mallory", Some(role))
            .with_author("bob@dev1".try_into().unwrap());
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

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch)
}

#[parsec_test(testbed = "minimal")]
#[case(RealmRole::Owner)]
#[case(RealmRole::Manager)]
async fn manager_trying_to_give_admin_role(#[case] role: RealmRole, env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.new_user("mallory");
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|e| e.realm);
        builder
            .share_realm(realm_id, "bob", Some(RealmRole::Manager))
            .then_also_share_with("mallory", Some(role))
            .with_author("bob@dev1".try_into().unwrap());
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
        CertifAddCertificatesBatchError::InvalidCertificate(
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
    #[case] author_role: RealmRole,
    #[case] recipient_role: RealmRole,
    env: &TestbedEnv,
) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.new_user("mallory");
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|e| e.realm);
        builder
            .share_realm(realm_id, "bob", Some(author_role))
            .then_also_share_with("mallory", Some(recipient_role))
            .with_author("bob@dev1".try_into().unwrap());
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
        CertifAddCertificatesBatchError::InvalidCertificate(
            InvalidCertificateError::RealmAuthorNotOwnerOrManager { .. }
        )
    )
}
