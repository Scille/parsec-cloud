// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::certificates_ops_factory;
use crate::certificates_ops::{InvalidManifestError, ValidateManifestError};

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let (env, (expected_manifest, vlob, vlob_timestamp)) = env.customize_with_map(|builder| {
        builder.new_user_realm("alice");
        builder.certificates_storage_fetch_certificates("alice@dev1");
        builder
            .create_or_update_user_manifest_vlob("alice")
            .map(|e| {
                (
                    e.manifest.clone(),
                    e.encrypted(&env.template),
                    e.manifest.timestamp,
                )
            })
    });

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(&env, &alice).await;

    let manifest = ops
        .validate_user_manifest(
            env.get_last_certificate_index(),
            &alice.device_id,
            1,
            vlob_timestamp,
            &vlob,
        )
        .await
        .unwrap();

    p_assert_eq!(manifest, *expected_manifest);

    ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn offline(env: &TestbedEnv) {
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .validate_user_manifest(1, &alice.device_id, 0, now, b"")
        .await
        .unwrap_err();

    p_assert_matches!(err, ValidateManifestError::Offline,);

    ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn non_existent_author(env: &TestbedEnv) {
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .validate_user_manifest(0, &alice.device_id, 0, now, b"")
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        ValidateManifestError::InvalidManifest(InvalidManifestError::NonExistentAuthor { .. })
    );

    ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn corrupted(env: &TestbedEnv) {
    let (env, timestamp) = env.customize_with_map(|builder| {
        let timestamp = builder.new_realm("alice").map(|e| e.timestamp);
        builder.certificates_storage_fetch_certificates("alice@dev1");
        timestamp
    });

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(&env, &alice).await;

    let err = ops
        .validate_user_manifest(
            env.get_last_certificate_index(),
            &alice.device_id,
            0,
            timestamp,
            b"",
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        ValidateManifestError::InvalidManifest(InvalidManifestError::Corrupted { .. })
    );

    ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn user_manifest_corrupted_by_bad_realm_id(env: &TestbedEnv) {
    let (env, (other_realm_id, timestamp)) = env.customize_with_map(|builder| {
        let (realm_id, timestamp) = builder
            .new_realm("alice")
            .map(|e| (e.realm_id, e.timestamp));
        builder.certificates_storage_fetch_certificates("alice@dev1");
        (realm_id, timestamp)
    });

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(&env, &alice).await;

    let alice_manifest = UserManifest {
        author: alice.device_id.clone(),
        timestamp,
        // Bad realm !
        id: other_realm_id,
        version: 0,
        created: timestamp,
        updated: timestamp,
        last_processed_message: 0,
        workspaces: vec![],
    };

    let err = ops
        .validate_user_manifest(
            env.get_last_certificate_index(),
            &alice.device_id,
            0,
            timestamp,
            &alice_manifest.dump_sign_and_encrypt(&alice.signing_key, &ops.device.local_symkey),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        ValidateManifestError::InvalidManifest(InvalidManifestError::Corrupted { .. })
    );

    ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn author_no_access_to_realm(env: &TestbedEnv) {
    let (env, (vlob, vlob_timestamp)) = env.customize_with_map(|builder| {
        builder.certificates_storage_fetch_certificates("alice@dev1");
        builder.with_check_consistency_disabled(|builder| {
            builder
                .create_or_update_user_manifest_vlob("alice")
                .map(|e| (e.encrypted(&env.template), e.manifest.timestamp))
        })
    });

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(&env, &alice).await;

    let err = ops
        .validate_user_manifest(
            env.get_last_certificate_index(),
            &alice.device_id,
            1,
            vlob_timestamp,
            &vlob,
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        ValidateManifestError::InvalidManifest(InvalidManifestError::AuthorNoAccessToRealm { .. })
    );

    ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn revoked(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.revoke_user("bob");
        builder.certificates_storage_fetch_certificates("bob@dev1");
        builder.with_check_consistency_disabled(|builder| {
            builder.create_or_update_user_manifest_vlob("bob");
        })
    });

    let bob = env.local_device("bob@dev1");
    let vlob_id = bob.user_realm_id;

    let ops = certificates_ops_factory(&env, &bob).await;

    let (vlob, vlob_timestamp) = match env.template.events.iter().last().unwrap() {
        TestbedEvent::CreateOrUpdateUserManifestVlob(e) => {
            (e.encrypted(&env.template), e.manifest.timestamp)
        }
        e => panic!("Unexpected event {:?}", e),
    };

    let err = ops
        .validate_user_manifest(
            env.get_last_certificate_index(),
            &bob.device_id,
            1,
            vlob_timestamp,
            &vlob,
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        ValidateManifestError::InvalidManifest(
            InvalidManifestError::RevokedAuthor {
                author,
                realm,
                timestamp,
                version,
                vlob,
            }
        )
        if {
            p_assert_eq!(author, bob.device_id);
            p_assert_eq!(realm, vlob_id);
            p_assert_eq!(timestamp, vlob_timestamp);
            p_assert_eq!(version, 1);
            p_assert_eq!(vlob, vlob_id);
            true
        }
    );

    ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn workspace_manifest_cannot_write(env: &TestbedEnv) {
    let (env, (shared_realm_id, shared_realm_key)) = env.customize_with_map(|builder| {
        builder.new_user("bob");
        let (realm_id, realm_key) = builder
            .new_user_realm("alice")
            .then_share_with("bob", Some(RealmRole::Reader))
            .map(|e| (e.realm, e.realm_key.clone()));
        builder.certificates_storage_fetch_certificates("bob@dev1");
        builder.with_check_consistency_disabled(|builder| {
            builder.create_or_update_workspace_manifest_vlob("bob@dev1", realm_id);
        });
        (realm_id, realm_key)
    });

    let bob = env.local_device("bob@dev1");

    let ops = certificates_ops_factory(&env, &bob).await;

    let (vlob, vlob_timestamp) = match env.template.events.iter().last().unwrap() {
        TestbedEvent::CreateOrUpdateWorkspaceManifestVlob(e) => {
            (e.encrypted(&env.template), e.manifest.timestamp)
        }
        e => panic!("Unexpected event {:?}", e),
    };

    let err = ops
        .validate_workspace_manifest(
            shared_realm_id,
            &shared_realm_key,
            env.get_last_certificate_index(),
            &bob.device_id,
            1,
            vlob_timestamp,
            &vlob,
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        ValidateManifestError::InvalidManifest(
            InvalidManifestError::AuthorRealmRoleCannotWrite {
                author,
                author_role,
                realm,
                timestamp,
                version,
                vlob,
            }
        )
        if {
            p_assert_eq!(author, bob.device_id);
            p_assert_eq!(author_role, RealmRole::Reader);
            p_assert_eq!(realm, shared_realm_id);
            p_assert_eq!(timestamp, vlob_timestamp);
            p_assert_eq!(version, 1);
            p_assert_eq!(vlob, shared_realm_id);
            true
        },
    );

    ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn user_manifest_cannot_write(env: &TestbedEnv) {
    let (env, (shared_realm_id, vlob, vlob_timestamp)) = env.customize_with_map(|builder| {
        builder.new_user("bob");
        let realm_id = builder
            .new_user_realm("alice")
            .then_share_with("bob", Some(RealmRole::Owner))
            .map(|e| e.realm);
        builder.share_realm(realm_id, "alice", Some(RealmRole::Reader));
        builder.certificates_storage_fetch_certificates("alice@dev1");
        let (vlob, vlob_timestamp) = builder.with_check_consistency_disabled(|builder| {
            builder
                .create_or_update_user_manifest_vlob("alice")
                .map(|e| (e.encrypted(&env.template), e.manifest.timestamp))
        });
        (realm_id, vlob, vlob_timestamp)
    });

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(&env, &alice).await;

    let err = ops
        .validate_user_manifest(
            env.get_last_certificate_index(),
            &alice.device_id,
            1,
            vlob_timestamp,
            &vlob,
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        ValidateManifestError::InvalidManifest(
            InvalidManifestError::AuthorRealmRoleCannotWrite {
                author,
                author_role,
                realm,
                timestamp,
                version,
                vlob,
            }
        )
        if {
            p_assert_eq!(author, alice.device_id);
            p_assert_eq!(author_role, RealmRole::Reader);
            p_assert_eq!(realm, shared_realm_id);
            p_assert_eq!(timestamp, vlob_timestamp);
            p_assert_eq!(version, 1);
            p_assert_eq!(vlob, shared_realm_id);
            true
        },
    );

    ops.stop().await;
}
