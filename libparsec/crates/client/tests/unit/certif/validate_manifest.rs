// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::test_register_send_hook;
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::certificates_ops_factory;
use crate::certif::{CertifValidateManifestError, InvalidManifestError};

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
            env.get_last_realm_certificate_timestamp(alice.user_realm_id),
            env.get_last_common_certificate_timestamp(),
            &alice.device_id,
            1,
            vlob_timestamp,
            &vlob,
        )
        .await
        .unwrap();

    p_assert_eq!(manifest, *expected_manifest);
}

#[parsec_test(testbed = "minimal")]
async fn offline(env: &TestbedEnv) {
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .validate_user_manifest(now, now, &alice.device_id, 0, now, b"")
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifValidateManifestError::Offline);
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;
    ops.stop().await.unwrap();

    let err = ops
        .validate_user_manifest(now, now, &alice.device_id, 0, now, b"")
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifValidateManifestError::Stopped);
}

#[parsec_test(testbed = "minimal")]
async fn non_existent_author(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user_realm("alice");
        builder.certificates_storage_fetch_certificates("alice@dev1");
    });

    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(&env, &alice).await;

    let err = ops
        .validate_user_manifest(
            env.get_last_realm_certificate_timestamp(alice.user_realm_id),
            env.get_last_common_certificate_timestamp(),
            &"alice@dev2".parse().unwrap(),
            0,
            now,
            b"",
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateManifestError::InvalidManifest(boxed)
        if matches!(*boxed, InvalidManifestError::NonExistentAuthor { .. })
    );
}

#[parsec_test(testbed = "minimal")]
async fn corrupted(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user_realm("alice");
        builder.certificates_storage_fetch_certificates("alice@dev1");
    });

    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(&env, &alice).await;

    let err = ops
        .validate_user_manifest(
            env.get_last_realm_certificate_timestamp(alice.user_realm_id),
            env.get_last_common_certificate_timestamp(),
            &alice.device_id,
            0,
            now,
            b"",
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateManifestError::InvalidManifest(boxed)
        if matches!(*boxed, InvalidManifestError::Corrupted { .. })
    );
}

#[parsec_test(testbed = "minimal")]
async fn user_manifest_corrupted_by_bad_realm_id(env: &TestbedEnv) {
    let (env, (other_realm_id, timestamp)) = env.customize_with_map(|builder| {
        builder.new_user_realm("alice");
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
        workspaces_legacy_initial_info: vec![],
    };

    let err = ops
        .validate_user_manifest(
            env.get_last_realm_certificate_timestamp(alice.user_realm_id),
            env.get_last_common_certificate_timestamp(),
            &alice.device_id,
            0,
            timestamp,
            &alice_manifest.dump_sign_and_encrypt(&alice.signing_key, &ops.device.local_symkey),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateManifestError::InvalidManifest(boxed)
        if matches!(*boxed, InvalidManifestError::Corrupted { .. })
    );
}

#[parsec_test(testbed = "minimal")]
async fn author_no_access_to_realm(env: &TestbedEnv) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        builder.new_user("bob");
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|e| e.realm);
        builder.certificates_storage_fetch_certificates("alice@dev1");
        builder.with_check_consistency_disabled(|builder| {
            builder.create_or_update_workspace_manifest_vlob("bob@dev1", realm_id);
        });
        realm_id
    });

    let (vlob, vlob_timestamp) = match env.template.events.iter().last().unwrap() {
        TestbedEvent::CreateOrUpdateWorkspaceManifestVlob(e) => {
            (e.encrypted(&env.template), e.manifest.timestamp)
        }
        e => panic!("Unexpected event {:?}", e),
    };

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(&env, &alice).await;

    let keys_bundle = env.get_last_realm_keys_bundle(realm_id);
    let keys_bundle_access = env.get_last_realm_keys_bundle_access_for(realm_id, &alice.user_id());
    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
            p_assert_eq!(req.realm_id, realm_id);
            authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                key_index: 1,
                keys_bundle,
                keys_bundle_access,
            }
        },
    );

    let err = ops
        .validate_workspace_manifest(
            env.get_last_realm_certificate_timestamp(realm_id),
            env.get_last_common_certificate_timestamp(),
            realm_id,
            1,
            &"bob@dev1".parse().unwrap(),
            1,
            vlob_timestamp,
            &vlob,
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateManifestError::InvalidManifest(boxed) if matches!(*boxed, InvalidManifestError::AuthorNoAccessToRealm { .. })
    );
}

#[parsec_test(testbed = "minimal")]
async fn revoked(env: &TestbedEnv) {
    let env = env.customize(|builder| {
        builder.new_user("bob");
        builder.new_user_realm("bob");
        builder.revoke_user("bob");
        builder.certificates_storage_fetch_certificates("bob@dev1");
        builder.with_check_consistency_disabled(|builder| {
            builder.create_or_update_user_manifest_vlob("bob");
        })
    });

    let bob = env.local_device("bob@dev1");

    let ops = certificates_ops_factory(&env, &bob).await;

    let (vlob, vlob_timestamp) = match env.template.events.iter().last().unwrap() {
        TestbedEvent::CreateOrUpdateUserManifestVlob(e) => {
            (e.encrypted(&env.template), e.manifest.timestamp)
        }
        e => panic!("Unexpected event {:?}", e),
    };

    let err = ops
        .validate_user_manifest(
            env.get_last_realm_certificate_timestamp(bob.user_realm_id),
            env.get_last_common_certificate_timestamp(),
            &bob.device_id,
            1,
            vlob_timestamp,
            &vlob,
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateManifestError::InvalidManifest(boxed)
            if matches!(
                &*boxed,
                InvalidManifestError::RevokedAuthor {
                    author,
                    realm,
                    timestamp,
                    version,
                    vlob,
                }
                if {
                    p_assert_eq!(*author, bob.device_id);
                    p_assert_eq!(*realm, bob.user_realm_id);
                    p_assert_eq!(*timestamp, vlob_timestamp);
                    p_assert_eq!(*version, 1);
                    p_assert_eq!(*vlob, bob.user_realm_id);
                    true
                }
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn workspace_manifest_cannot_write(env: &TestbedEnv) {
    let (env, shared_realm_id) = env.customize_with_map(|builder| {
        builder.new_user("bob");
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|e| e.realm);
        builder.share_realm(realm_id, "bob", Some(RealmRole::Reader));
        builder.certificates_storage_fetch_certificates("bob@dev1");
        builder.with_check_consistency_disabled(|builder| {
            builder.create_or_update_workspace_manifest_vlob("bob@dev1", realm_id);
        });
        realm_id
    });
    let (_, shared_realm_key_index) = env.get_last_realm_key(shared_realm_id);

    let alice = env.local_device("alice@dev1");
    let bob = env.local_device("bob@dev1");

    let ops = certificates_ops_factory(&env, &bob).await;

    let (vlob, vlob_timestamp) = match env.template.events.iter().last().unwrap() {
        TestbedEvent::CreateOrUpdateWorkspaceManifestVlob(e) => {
            (e.encrypted(&env.template), e.manifest.timestamp)
        }
        e => panic!("Unexpected event {:?}", e),
    };

    let keys_bundle = env.get_last_realm_keys_bundle(shared_realm_id);
    let keys_bundle_access = {
        let alice_keys_bundle_access =
            env.get_last_realm_keys_bundle_access_for(shared_realm_id, alice.user_id());
        let cleartext_keys_bundle_access = alice
            .private_key
            .decrypt_from_self(&alice_keys_bundle_access)
            .unwrap();
        bob.public_key()
            .encrypt_for_self(&cleartext_keys_bundle_access)
            .into()
    };
    test_register_send_hook(
        &env.discriminant_dir,
        move |_req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
            authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                key_index: 1,
                keys_bundle,
                keys_bundle_access,
            }
        },
    );

    let err = ops
        .validate_workspace_manifest(
            env.get_last_realm_certificate_timestamp(shared_realm_id),
            env.get_last_common_certificate_timestamp(),
            shared_realm_id,
            shared_realm_key_index,
            &bob.device_id,
            1,
            vlob_timestamp,
            &vlob,
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateManifestError::InvalidManifest(boxed) if matches!(&*boxed,
            InvalidManifestError::AuthorRealmRoleCannotWrite {
                author,
                author_role,
                realm,
                timestamp,
                version,
                vlob,
            }
        if {
            p_assert_eq!(*author, bob.device_id);
            p_assert_eq!(*author_role, RealmRole::Reader);
            p_assert_eq!(*realm, shared_realm_id);
            p_assert_eq!(*timestamp, vlob_timestamp);
            p_assert_eq!(*version, 1);
            p_assert_eq!(*vlob, shared_realm_id);
            true
        },
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn user_manifest_cannot_write(env: &TestbedEnv) {
    let (env, (shared_realm_id, vlob, vlob_timestamp)) = env.customize_with_map(|builder| {
        builder.new_user("bob");
        let realm_id = builder
            .new_user_realm("alice")
            // Key rotation is not supposed to be used for user realm, but
            // current test is rather odd anyway (access rights should never
            // change for user realm)
            .then_do_initial_key_rotation()
            .map(|e| e.realm);
        builder.share_realm(realm_id, "bob", Some(RealmRole::Owner));
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
            env.get_last_realm_certificate_timestamp(shared_realm_id),
            env.get_last_common_certificate_timestamp(),
            &alice.device_id,
            1,
            vlob_timestamp,
            &vlob,
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateManifestError::InvalidManifest(boxed) if matches!(&*boxed,
            InvalidManifestError::AuthorRealmRoleCannotWrite {
                author,
                author_role,
                realm,
                timestamp,
                version,
                vlob,
            }
        if {
            p_assert_eq!(*author, alice.device_id);
            p_assert_eq!(*author_role, RealmRole::Reader);
            p_assert_eq!(*realm, shared_realm_id);
            p_assert_eq!(*timestamp, vlob_timestamp);
            p_assert_eq!(*version, 1);
            p_assert_eq!(*vlob, shared_realm_id);
            true
        },
        )
    );
}
