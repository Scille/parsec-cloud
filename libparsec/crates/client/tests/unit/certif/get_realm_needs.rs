// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certif::{CertifGetRealmNeedsError, RealmNeeds};

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "coolorg")]
async fn nothing(
    #[values("never_had_needs", "with_needs_already_fulfilled")] kind: &str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    match kind {
        "never_had_needs" => (),
        "with_needs_already_fulfilled" => {
            env.customize(|builder| {
                // New needs for the realm...
                builder.revoke_user("bob");
                builder.share_realm(wksp1_id, "mallory", RealmRole::Contributor);
                builder.share_realm(wksp1_id, "mallory", None);
                // ...but then they get fulfilled (and our certificates ops knows about it)
                builder.share_realm(wksp1_id, "bob", None);
                builder.rotate_key_realm(wksp1_id);
                builder.certificates_storage_fetch_certificates("alice@dev1");
            })
            .await;
        }
        unknown => panic!("Unknown kind: {}", unknown),
    }

    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    p_assert_eq!(
        ops.get_realm_needs(wksp1_id).await.unwrap(),
        RealmNeeds::Nothing
    );
}

#[parsec_test(testbed = "sequestered")]
async fn sequestered_nothing(env: &TestbedEnv) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let sequester_service_2_id: SequesterServiceID =
        *env.template.get_stuff("sequester_service_2_id");
    env.customize(|builder| {
        // New needs for the realm...
        builder.new_sequester_service();
        builder.revoke_sequester_service(sequester_service_2_id);
        // ...but then they get fulfilled (and our certificates ops knows about it)
        builder.rotate_key_realm(wksp1_id);
        builder.certificates_storage_fetch_certificates("alice@dev1");
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    p_assert_eq!(
        ops.get_realm_needs(wksp1_id).await.unwrap(),
        RealmNeeds::Nothing
    );
}

#[parsec_test(testbed = "coolorg")]
async fn key_rotation_only(
    #[values(
        "unshared",
        "unshared_then_shared_again",
        "same_user_revoked_and_unshared"
    )]
    kind: &str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    env.customize(|builder| {
        match kind {
            "unshared" => {
                builder.share_realm(wksp1_id, "bob", None);
            }
            "unshared_then_shared_again" => {
                // It could be tempting to considered the is no need for key rotation in this
                // case, however this would make the realm's needs non-determinist as they
                // may change depending on when they are checked.
                builder.share_realm(wksp1_id, "bob", None);
                builder.share_realm(wksp1_id, "bob", RealmRole::Contributor);
            }
            "same_user_revoked_and_unshared" => {
                builder.revoke_user("bob");
                builder.share_realm(wksp1_id, "bob", None);
                // Now that Bob is no longer part of the realm, we don't care that it has been revoked.
            }
            unknown => panic!("Unknown kind: {}", unknown),
        }
        builder.certificates_storage_fetch_certificates("alice@dev1");
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    p_assert_eq!(
        ops.get_realm_needs(wksp1_id).await.unwrap(),
        RealmNeeds::KeyRotationOnly {
            current_key_index: Some(1)
        }
    );
}

#[parsec_test(testbed = "sequestered")]
async fn sequestered_key_rotation_only(
    #[values(
        "new_sequester_service",
        "revoked_sequester_service",
        "same_sequester_service_created_then_revoked"
    )]
    kind: &str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let sequester_service_2_id: SequesterServiceID =
        *env.template.get_stuff("sequester_service_2_id");
    env.customize(|builder| {
        match kind {
            "new_sequester_service" => {
                builder.new_sequester_service().map(|e| e.id);
            }
            "revoked_sequester_service" => {
                builder.revoke_sequester_service(sequester_service_2_id);
            }
            "same_sequester_service_created_then_revoked" => {
                let sequester_service3_id = builder.new_sequester_service().map(|e| e.id);
                builder.revoke_sequester_service(sequester_service3_id);
            }
            unknown => panic!("Unknown kind: {}", unknown),
        }
        builder.certificates_storage_fetch_certificates("alice@dev1");
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    p_assert_eq!(
        ops.get_realm_needs(wksp1_id).await.unwrap(),
        RealmNeeds::KeyRotationOnly {
            current_key_index: Some(3)
        }
    );
}

#[parsec_test(testbed = "coolorg")]
async fn unshare_then_key_rotation(
    #[values(
        "only_user_revoked",
        "users_revoked_and_unshared",
        "multiple_users_revoked"
    )]
    kind: &str,
    env: &TestbedEnv,
) {
    let wksp1_id: VlobID = *env.template.get_stuff("wksp1_id");
    let mut expected_revoked_users = vec!["bob".parse().unwrap()];
    env.customize(|builder| {
        match kind {
            "only_user_revoked" => {
                builder.revoke_user("bob");
            }
            "users_revoked_and_unshared" => {
                builder.share_realm(wksp1_id, "mallory", RealmRole::Contributor);
                builder.share_realm(wksp1_id, "mallory", None);
                builder.revoke_user("bob");
            }
            "multiple_users_revoked" => {
                builder.share_realm(wksp1_id, "mallory", RealmRole::Contributor);
                builder.revoke_user("bob");
                builder.revoke_user("mallory");
                expected_revoked_users = vec!["mallory".parse().unwrap(), "bob".parse().unwrap()];
            }
            unknown => panic!("Unknown kind: {}", unknown),
        }
        builder.certificates_storage_fetch_certificates("alice@dev1");
    })
    .await;

    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    p_assert_eq!(
        ops.get_realm_needs(wksp1_id).await.unwrap(),
        RealmNeeds::UnshareThenKeyRotation {
            revoked_users: expected_revoked_users,
            current_key_index: Some(1),
        }
    );
}

#[parsec_test(testbed = "minimal")]
async fn unknown_realm(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    p_assert_eq!(
        ops.get_realm_needs(VlobID::default()).await.unwrap(),
        RealmNeeds::Nothing
    );
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    ops.stop().await.unwrap();

    let err = ops.get_realm_needs(VlobID::default()).await.unwrap_err();

    p_assert_matches!(err, CertifGetRealmNeedsError::Stopped);
}
