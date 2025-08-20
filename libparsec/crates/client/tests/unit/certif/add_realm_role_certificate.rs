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
    env.customize(|builder| {
        builder.new_user_realm("alice");
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let switch = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &env.get_realms_certificates_signed(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch { .. });
}

#[parsec_test(testbed = "minimal")]
async fn ok_with_existing_certificates(
    #[values("none", "reader")] last_known_role: &str,
    #[values("manager", "owner")] new_role: &str,
    env: &TestbedEnv,
) {
    let wksp1_id = env
        .customize(|builder| {
            builder.new_user("bob");
            // Bob creates a realm...
            let wksp1_id = builder.new_realm("bob").map(|e| e.realm_id);
            builder.rotate_key_realm(wksp1_id);
            builder.rename_realm(wksp1_id, "wksp1");
            // ...then shares it with Alice...
            builder.share_realm(wksp1_id, "alice", RealmRole::Manager);
            // ...then change Alice's role one more time
            match last_known_role {
                "none" => {
                    builder.share_realm(wksp1_id, "alice", None);
                }
                "reader" => {
                    builder.share_realm(wksp1_id, "alice", RealmRole::Reader);
                }
                unknown => panic!("Unknown kind: {unknown}"),
            }

            // Finally Bob changes one last time Alice's role, which is the certificate
            // we are testing.
            builder.certificates_storage_fetch_certificates("alice@dev1");
            match new_role {
                "manager" => {
                    builder.share_realm(wksp1_id, "alice", RealmRole::Manager);
                }
                "owner" => {
                    builder.share_realm(wksp1_id, "alice", RealmRole::Owner);
                }
                unknown => panic!("Unknown kind: {unknown}"),
            }

            wksp1_id
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let (_, certif) = env.get_last_realm_role_certificate("alice", wksp1_id);
    let switch = ops
        .add_certificates_batch(
            &[],
            &[],
            &[],
            &[(wksp1_id, vec![certif])].into_iter().collect(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch { .. });
}

#[parsec_test(testbed = "minimal")]
async fn content_already_exists(env: &TestbedEnv) {
    env.customize(|builder| {
        let realm_id = builder.new_realm("alice").map(|e| e.realm_id);
        builder.with_check_consistency_disabled(|builder| {
            builder.new_realm("alice").customize(|e| {
                e.realm_id = realm_id;
            });
        });
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

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
            InvalidCertificateError::ContentAlreadyExists { .. }
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn older_than_author(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.new_user_realm("alice").customize(|event| {
            event.timestamp = DateTime::from_ymd_hms_us(1999, 1, 1, 0, 0, 0, 0).unwrap()
        });
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

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
    let timestamp = env
        .customize(|builder| {
            let realm_id = builder.new_realm("alice").map(|e| e.realm_id);
            let timestamp = builder.rotate_key_realm(realm_id).map(|e| e.timestamp);
            builder
                .share_realm(realm_id, "bob", RealmRole::Contributor)
                .customize(|event| {
                    event.timestamp = timestamp;
                });
            timestamp
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

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

// In practice the server ensures only certificates provided together (e.g. user and
// device certificates) can have the same timestamp.
// However current implementation of `CertificateOps` ignore this, so this test is
// mostly here to ensure we will get notified whenever this behavior change.
#[parsec_test(testbed = "minimal")]
async fn different_realms_can_have_same_timestamp(env: &TestbedEnv) {
    env.customize(|builder| {
        let timestamp = builder.new_realm("alice").map(|e| e.timestamp);
        builder.new_realm("alice").customize(|event| {
            event.timestamp = timestamp;
        });
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let switch = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &env.get_realms_certificates_signed(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch { .. });
}

#[parsec_test(testbed = "minimal")]
async fn non_existing_author(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.new_realm("alice");
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

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
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(
            *boxed,
            InvalidCertificateError::NonExistingAuthor { .. }
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn cannot_share_with_revoked(
    #[values("new_share", "update_role")] kind: &str,
    env: &TestbedEnv,
) {
    env.customize(|builder| {
        let wksp_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation_and_naming("wksp1")
            .map(|e| e.realm);
        builder.new_user("bob");
        match kind {
            "new_share" => (),
            "update_role" => {
                builder.share_realm(wksp_id, "bob", RealmRole::Reader);
            }
            unknown => panic!("Unknown kind: {unknown}"),
        }
        builder.revoke_user("bob");
        builder.with_check_consistency_disabled(|builder| {
            builder.share_realm(wksp_id, "bob", RealmRole::Contributor);
        });
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

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
            InvalidCertificateError::RelatedUserAlreadyRevoked { .. }
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn can_unshare_with_revoked(env: &TestbedEnv) {
    env.customize(|builder| {
        let wksp_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation_and_naming("wksp1")
            .map(|e| e.realm);
        builder.new_user("bob");
        builder.share_realm(wksp_id, "bob", RealmRole::Contributor);
        builder.revoke_user("bob");
        builder.share_realm(wksp_id, "bob", None);
        wksp_id
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let switch = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &env.get_realms_certificates_signed(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch { .. });
}

#[parsec_test(testbed = "minimal")]
#[case(UserProfile::Admin)]
#[case(UserProfile::Standard)]
#[case(UserProfile::Outsider)]
async fn create_with_profile(#[case] profile: UserProfile, env: &TestbedEnv) {
    env.customize(|builder| {
        builder.new_user("bob").with_initial_profile(profile);
        builder.with_check_consistency_disabled(|builder| {
            builder.new_realm("bob");
        })
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let outcome = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &env.get_realms_certificates_signed(),
        )
        .await;

    match profile {
        UserProfile::Admin | UserProfile::Standard => {
            p_assert_matches!(outcome, Ok(MaybeRedactedSwitch::NoSwitch { .. }));
        }
        UserProfile::Outsider => {
            p_assert_matches!(
                outcome,
                Err(CertifAddCertificatesBatchError::InvalidCertificate(boxed))
                if matches!(
                    *boxed,
                    InvalidCertificateError::RealmOutsiderCannotBeOwnerOrManager { .. }
                )
            )
        }
    }
}

#[parsec_test(testbed = "minimal")]
async fn initial_realm_role_not_signed_by_self(env: &TestbedEnv) {
    let realm_id = env
        .customize(|builder| {
            builder.new_user("bob");
            builder.new_realm("bob").map(|e| e.realm_id)
        })
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let (bob_realm_role_certif, _) = env.get_last_realm_role_certificate("bob", realm_id);

    let mut bob_realm_role_certif_authored_by_alice = (*bob_realm_role_certif).clone();
    bob_realm_role_certif_authored_by_alice.author = "alice@dev1".try_into().unwrap();
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
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(
            *boxed,
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
    let realm_id = env
        .customize(|builder| builder.new_user_realm("alice").map(|e| e.realm_id))
        .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

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
        CertifAddCertificatesBatchError::InvalidCertificate(boxed)
        if matches!(
            *boxed,
            InvalidCertificateError::RealmFirstRoleMustBeOwner { .. }
        )
    )
}

#[parsec_test(testbed = "minimal")]
async fn share_with_no_role(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.new_user("bob");
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|e| e.realm);
        builder.share_realm(realm_id, "bob", None);
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

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
            InvalidCertificateError::ContentAlreadyExists { .. }
        )
    )
}

#[parsec_test(testbed = "minimal")]
async fn same_realm_id(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.new_user("bob");
        builder.new_user("mallory");
        let realm_id = builder.new_user_realm("alice").map(|e| e.realm_id);
        builder
            .new_user_realm("bob")
            .customize(|event| event.realm_id = realm_id);
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

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
            InvalidCertificateError::RealmAuthorHasNoRole { .. }
        )
    )
}

#[parsec_test(testbed = "minimal")]
#[case(RealmRole::Contributor)]
#[case(RealmRole::Reader)]
async fn share_realm_with_outsider(#[case] role: RealmRole, env: &TestbedEnv) {
    env.customize(|builder| {
        builder
            .new_user("bob")
            .with_initial_profile(UserProfile::Outsider);
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|e| e.realm);
        builder.share_realm(realm_id, "bob", Some(role));
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let switch = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &env.get_realms_certificates_signed(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch { .. });
}

#[parsec_test(testbed = "minimal")]
#[case(RealmRole::Owner)]
#[case(RealmRole::Manager)]
async fn share_realm_privileges_with_outsider(#[case] role: RealmRole, env: &TestbedEnv) {
    env.customize(|builder| {
        builder
            .new_user("bob")
            .with_initial_profile(UserProfile::Outsider);
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|e| e.realm);
        builder.with_check_consistency_disabled(|builder| {
            builder.share_realm(realm_id, "bob", Some(role));
        })
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

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
            InvalidCertificateError::RealmOutsiderCannotBeOwnerOrManager { .. }
        )
    )
}

#[parsec_test(testbed = "minimal")]
async fn user_becoming_outsider_can_still_share(env: &TestbedEnv) {
    env.customize(|builder| {
        builder
            .new_user("bob")
            .with_initial_profile(UserProfile::Admin);
        builder.new_user("mallory");
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|e| e.realm);
        builder.share_realm(realm_id, "bob", RealmRole::Owner);
        // Alice becomes an Outsider, but is still Owner of the realm !
        builder.update_user_profile("alice", UserProfile::Outsider);

        // So she can still share...
        builder.share_realm(realm_id, "mallory", RealmRole::Owner);
        // ...and unshare
        builder.share_realm(realm_id, "bob", None);
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let switch = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &env.get_realms_certificates_signed(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch { .. });
}

#[parsec_test(testbed = "minimal")]
async fn previously_owner_giving_role(env: &TestbedEnv) {
    env.customize(|builder| {
        builder.new_user("bob");
        builder.new_user("mallory");
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|e| e.realm);
        builder.share_realm(realm_id, "bob", RealmRole::Owner);
        builder.share_realm(realm_id, "mallory", RealmRole::Owner);

        // Bob is no longer owner... but still tries to give a role !
        builder.share_realm(realm_id, "bob", RealmRole::Contributor);
        builder
            .share_realm(realm_id, "alice", RealmRole::Reader)
            .customize(|e| {
                e.author = "bob@dev1".try_into().unwrap();
            });
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

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
    )
}

#[parsec_test(testbed = "minimal")]
#[case(RealmRole::Owner)]
#[case(RealmRole::Manager)]
#[case(RealmRole::Contributor)]
#[case(RealmRole::Reader)]
async fn owner_giving_role(#[case] role: RealmRole, env: &TestbedEnv) {
    env.customize(|builder| {
        builder.new_user("bob");
        builder.new_user("mallory");
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|e| e.realm);
        builder.share_realm(realm_id, "bob", Some(role));
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let switch = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &env.get_realms_certificates_signed(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch { .. });
}

#[parsec_test(testbed = "minimal")]
#[case(RealmRole::Contributor)]
#[case(RealmRole::Reader)]
async fn manager_giving_role(#[case] role: RealmRole, env: &TestbedEnv) {
    env.customize(|builder| {
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
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let switch = ops
        .add_certificates_batch(
            &env.get_common_certificates_signed(),
            &[],
            &[],
            &env.get_realms_certificates_signed(),
        )
        .await
        .unwrap();

    p_assert_matches!(switch, MaybeRedactedSwitch::NoSwitch { .. });
}

#[parsec_test(testbed = "minimal")]
#[case(RealmRole::Owner)]
#[case(RealmRole::Manager)]
async fn manager_trying_to_give_admin_role(#[case] role: RealmRole, env: &TestbedEnv) {
    env.customize(|builder| {
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
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

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
    env.customize(|builder| {
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
    })
    .await;
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

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
            InvalidCertificateError::RealmAuthorNotOwnerOrManager { .. }
        )
    )
}
