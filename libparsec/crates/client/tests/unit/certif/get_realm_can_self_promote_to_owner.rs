// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certif::CertifStoreError;

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(
    #[values(
        "as_manager_alone",
        "as_contributor_with_another_contributor",
        // OUTSIDER cannot self-promote, so we should be allowed to
        // self-promote even if we don't have the highest role!
        "as_reader_with_outsider_contributor",
        "with_multiple_higher_roles_revoked",
    )]
    kind: &str,
    env: &TestbedEnv,
) {
    let (alice_role, mallory_profile_and_role) = match kind {
        "as_manager_alone" => (RealmRole::Manager, None),
        "as_contributor_with_another_contributor" => (
            RealmRole::Contributor,
            Some((UserProfile::Standard, RealmRole::Contributor, false)),
        ),
        "as_reader_with_outsider_contributor" => (
            RealmRole::Reader,
            Some((UserProfile::Outsider, RealmRole::Contributor, false)),
        ),
        "with_multiple_higher_roles_revoked" => (
            RealmRole::Reader,
            Some((UserProfile::Standard, RealmRole::Manager, true)),
        ),
        unknown => panic!("Unknown kind: {unknown}"),
    };
    let realm_id = env
        .customize(|builder| {
            builder.new_user("bob");
            let realm_id = builder
                .new_realm("bob")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.share_realm(realm_id, "alice", alice_role);
            if let Some((mallory_profile, mallory_role, mallory_is_revoked)) =
                mallory_profile_and_role
            {
                builder
                    .new_user("mallory")
                    .with_initial_profile(mallory_profile);
                builder.share_realm(realm_id, "mallory", mallory_role);
                if mallory_is_revoked {
                    builder.revoke_user("mallory");
                }
            }
            builder.revoke_user("bob");
            builder.certificates_storage_fetch_certificates("alice@dev1");
            realm_id
        })
        .await;

    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let result = ops
        .get_realm_can_self_promote_to_owner(realm_id)
        .await
        .unwrap();

    p_assert_eq!(result, true);
}

#[parsec_test(testbed = "minimal")]
async fn false_already_owner(env: &TestbedEnv) {
    let realm_id = env
        .customize(|builder| {
            let realm_id = builder.new_realm("alice").map(|event| event.realm_id);
            builder.certificates_storage_fetch_certificates("alice@dev1");
            realm_id
        })
        .await;

    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let result = ops
        .get_realm_can_self_promote_to_owner(realm_id)
        .await
        .unwrap();

    p_assert_eq!(result, false);
}

#[parsec_test(testbed = "minimal")]
async fn false_not_in_realm(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let result = ops
        .get_realm_can_self_promote_to_owner(VlobID::default())
        .await
        .unwrap();

    p_assert_eq!(result, false);
}

#[parsec_test(testbed = "minimal")]
async fn false_outsider(env: &TestbedEnv) {
    // Mallory is an OUTSIDER; OUTSIDERs can never become OWNER, so returns
    // false even if all higher roles are revoked
    let realm_id = env
        .customize(|builder| {
            builder.new_user("bob");
            builder
                .new_user("mallory")
                .with_initial_profile(UserProfile::Outsider);
            let realm_id = builder
                .new_realm("bob")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.share_realm(realm_id, "mallory", RealmRole::Contributor);
            builder.revoke_user("bob");
            builder.certificates_storage_fetch_certificates("mallory@dev1");
            realm_id
        })
        .await;

    let mallory = env.local_device("mallory@dev1");
    let ops = certificates_ops_factory(env, &mallory).await;

    let result = ops
        .get_realm_can_self_promote_to_owner(realm_id)
        .await
        .unwrap();

    p_assert_eq!(result, false);
}

#[parsec_test(testbed = "minimal")]
async fn false_active_owner_exists(env: &TestbedEnv) {
    // Bob (OWNER) is NOT revoked; Alice (MANAGER) cannot self-promote
    let realm_id = env
        .customize(|builder| {
            builder.new_user("bob");
            let realm_id = builder
                .new_realm("bob")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.share_realm(realm_id, "alice", RealmRole::Manager);
            builder.certificates_storage_fetch_certificates("alice@dev1");
            realm_id
        })
        .await;

    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let result = ops
        .get_realm_can_self_promote_to_owner(realm_id)
        .await
        .unwrap();

    p_assert_eq!(result, false);
}

#[parsec_test(testbed = "minimal")]
async fn false_active_higher_role_exists(env: &TestbedEnv) {
    // Bob (OWNER) is revoked, but Mallory (MANAGER) is not; Alice (CONTRIBUTOR) cannot self-promote
    let realm_id = env
        .customize(|builder| {
            builder.new_user("bob");
            builder.new_user("mallory");
            let realm_id = builder
                .new_realm("bob")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.share_realm(realm_id, "alice", RealmRole::Contributor);
            builder.share_realm(realm_id, "mallory", RealmRole::Manager);
            builder.revoke_user("bob");
            builder.certificates_storage_fetch_certificates("alice@dev1");
            realm_id
        })
        .await;

    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let result = ops
        .get_realm_can_self_promote_to_owner(realm_id)
        .await
        .unwrap();

    p_assert_eq!(result, false);
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    ops.stop().await.unwrap();

    let err = ops
        .get_realm_can_self_promote_to_owner(VlobID::default())
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifStoreError::Stopped);
}
