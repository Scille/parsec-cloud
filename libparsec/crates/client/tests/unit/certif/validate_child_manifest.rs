// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    test_register_low_level_send_hook, test_register_send_hook,
    test_send_hook_realm_get_keys_bundle, ConnectionError, HeaderMap, ResponseMock, StatusCode,
};
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::certificates_ops_factory;
use crate::certif::{CertifValidateManifestError, InvalidManifestError};

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let file_id = VlobID::from_hex("aa00000000000000000000000000f11e").unwrap();

    let (realm_id, expected_manifest, vlob, vlob_timestamp) = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.certificates_storage_fetch_certificates("alice@dev1");
            builder
                .create_or_update_file_manifest_vlob("alice@dev1", realm_id, file_id, realm_id)
                .map(|e| {
                    (
                        realm_id,
                        ChildManifest::File((*e.manifest).clone()),
                        e.encrypted(&env.template),
                        e.manifest.timestamp,
                    )
                })
        })
        .await;
    let (_, realm_key_index) = env.get_last_realm_key(realm_id);

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, realm_id),
    );

    let manifest = ops
        .validate_child_manifest(
            env.get_last_realm_certificate_timestamp(realm_id),
            env.get_last_common_certificate_timestamp(),
            realm_id,
            realm_key_index,
            file_id,
            alice.device_id,
            1,
            vlob_timestamp,
            &vlob,
        )
        .await
        .unwrap();

    p_assert_eq!(manifest, expected_manifest);
}

#[parsec_test(testbed = "minimal")]
async fn offline(env: &TestbedEnv) {
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .validate_child_manifest(
            now,
            now,
            VlobID::default(),
            0,
            VlobID::default(),
            alice.device_id,
            0,
            now,
            b"",
        )
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifValidateManifestError::Offline(_));
}

#[parsec_test(testbed = "minimal")]
async fn non_existent_author(env: &TestbedEnv) {
    let realm_id = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.certificates_storage_fetch_certificates("alice@dev1");

            realm_id
        })
        .await;
    let vlob_id = VlobID::default();
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();
    let (key_derivation, realm_key_index) = env.get_last_realm_key(realm_id);
    let key = key_derivation.derive_secret_key_from_uuid(*vlob_id);

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, realm_id),
    );

    let err = ops
        .validate_child_manifest(
            env.get_last_realm_certificate_timestamp(realm_id),
            env.get_last_common_certificate_timestamp(),
            realm_id,
            realm_key_index,
            vlob_id,
            "alice@dev2".parse().unwrap(),
            0,
            now,
            // Decryption is done first, only then the user is searched for
            &key.encrypt(b""),
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
async fn cannot_decrypt(env: &TestbedEnv) {
    let realm_id = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.certificates_storage_fetch_certificates("alice@dev1");
            realm_id
        })
        .await;
    let (_, realm_key_index) = env.get_last_realm_key(realm_id);

    let child_id = VlobID::default();
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, realm_id),
    );

    let err = ops
        .validate_child_manifest(
            env.get_last_realm_certificate_timestamp(realm_id),
            env.get_last_common_certificate_timestamp(),
            realm_id,
            realm_key_index,
            child_id,
            alice.device_id,
            1,
            now,
            b"<dummy data>",
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateManifestError::InvalidManifest(boxed)
        if matches!(*boxed, InvalidManifestError::CannotDecrypt { .. })
    );
}

#[parsec_test(testbed = "minimal")]
async fn cleartext_corrupted(
    #[values("dummy", "folder_pointing_to_itself")] kind: &str,
    env: &TestbedEnv,
) {
    let realm_id = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.certificates_storage_fetch_certificates("alice@dev1");
            realm_id
        })
        .await;
    let child_id = VlobID::default();
    let (realm_last_key_derivation, realm_key_index) = env.get_last_realm_key(realm_id);
    let key = realm_last_key_derivation.derive_secret_key_from_uuid(*child_id);
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();

    let alice = env.local_device("alice@dev1");

    let (encrypted, data_error) = match kind {
        "dummy" => (key.encrypt(b"<dummy data>"), DataError::Signature),
        // It is valid to have a folder manifest pointing to itself, as long as it is the root manifest
        // Since the dump method is not aware of the context, it will not prevent this case.
        // Still, the validation after the load should catch it.
        "folder_pointing_to_itself" => {
            let now = "2020-01-01T00:00:00.000000Z".parse().unwrap();
            let manifest = FolderManifest {
                author: alice.device_id,
                timestamp: now,
                id: child_id,
                parent: child_id,
                version: 1,
                created: now,
                updated: now,
                children: Default::default(),
            };
            (
                manifest.dump_sign_and_encrypt(&alice.signing_key, &key),
                DataError::DataIntegrity {
                    data_type: "libparsec_types::manifest::FolderManifest",
                    invariant: "id and parent are different for child manifest",
                },
            )
        }
        unknown => panic!("Unknown kind {unknown}"),
    };

    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, realm_id),
    );

    let err = ops
        .validate_child_manifest(
            env.get_last_realm_certificate_timestamp(realm_id),
            env.get_last_common_certificate_timestamp(),
            realm_id,
            realm_key_index,
            child_id,
            alice.device_id,
            1,
            now,
            &encrypted,
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateManifestError::InvalidManifest(boxed)
        if matches!(&*boxed, InvalidManifestError::CleartextCorrupted { error, .. } if **error == data_error)
    )
}

#[parsec_test(testbed = "minimal")]
async fn author_no_access_to_realm(env: &TestbedEnv) {
    let file_id = VlobID::from_hex("aa00000000000000000000000000f11e").unwrap();

    let realm_id = env
        .customize(|builder| {
            builder.new_user("bob");
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|e| e.realm);
            builder.certificates_storage_fetch_certificates("alice@dev1");
            builder.with_check_consistency_disabled(|builder| {
                builder.create_or_update_file_manifest_vlob(
                    "bob@dev1",
                    realm_id,
                    file_id,
                    Some(realm_id),
                );
            });
            realm_id
        })
        .await;

    let (vlob, vlob_timestamp) = match env.template.events.iter().last().unwrap() {
        TestbedEvent::CreateOrUpdateFileManifestVlob(e) => {
            (e.encrypted(&env.template), e.manifest.timestamp)
        }
        e => panic!("Unexpected event {e:?}"),
    };

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, realm_id),
    );

    let err = ops
        .validate_child_manifest(
            env.get_last_realm_certificate_timestamp(realm_id),
            env.get_last_common_certificate_timestamp(),
            realm_id,
            1,
            file_id,
            "bob@dev1".parse().unwrap(),
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
    let file_id = VlobID::from_hex("aa00000000000000000000000000f11e").unwrap();

    let realm_id = env
        .customize(|builder| {
            builder.new_user("bob");
            let realm_id = builder
                .new_realm("bob")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.revoke_user("bob");
            builder.certificates_storage_fetch_certificates("bob@dev1");
            builder.with_check_consistency_disabled(|builder| {
                builder.create_or_update_file_manifest_vlob(
                    "bob@dev1",
                    realm_id,
                    file_id,
                    Some(realm_id),
                );
            });

            realm_id
        })
        .await;

    let (_, realm_key_index) = env.get_last_realm_key(realm_id);

    let bob = env.local_device("bob@dev1");

    let ops = certificates_ops_factory(env, &bob).await;

    let (vlob, vlob_timestamp) = match env.template.events.iter().last().unwrap() {
        TestbedEvent::CreateOrUpdateFileManifestVlob(e) => {
            (e.encrypted(&env.template), e.manifest.timestamp)
        }
        e => panic!("Unexpected event {e:?}"),
    };

    test_register_send_hook(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, bob.user_id, realm_id),
    );

    let err = ops
        .validate_child_manifest(
            env.get_last_realm_certificate_timestamp(realm_id),
            env.get_last_common_certificate_timestamp(),
            realm_id,
            realm_key_index,
            file_id,
            bob.device_id,
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
                    p_assert_eq!(*realm, realm_id);
                    p_assert_eq!(*timestamp, vlob_timestamp);
                    p_assert_eq!(*version, 1);
                    p_assert_eq!(*vlob, file_id);
                    true
                }
        )
    );
}

#[parsec_test(testbed = "minimal")]
async fn cannot_write(env: &TestbedEnv) {
    let file_id = VlobID::from_hex("aa00000000000000000000000000f11e").unwrap();

    let shared_realm_id = env
        .customize(|builder| {
            builder.new_user("bob");
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|e| e.realm);
            builder.share_realm(realm_id, "bob", Some(RealmRole::Reader));
            builder.certificates_storage_fetch_certificates("bob@dev1");
            builder.with_check_consistency_disabled(|builder| {
                builder.create_or_update_file_manifest_vlob(
                    "bob@dev1",
                    realm_id,
                    file_id,
                    Some(realm_id),
                );
            });
            realm_id
        })
        .await;
    let (_, shared_realm_key_index) = env.get_last_realm_key(shared_realm_id);

    let bob = env.local_device("bob@dev1");

    let ops = certificates_ops_factory(env, &bob).await;

    let (vlob, vlob_timestamp) = match env.template.events.iter().last().unwrap() {
        TestbedEvent::CreateOrUpdateFileManifestVlob(e) => {
            (e.encrypted(&env.template), e.manifest.timestamp)
        }
        e => panic!("Unexpected event {e:?}"),
    };

    test_register_send_hook(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, bob.user_id, shared_realm_id),
    );

    let err = ops
        .validate_child_manifest(
            env.get_last_realm_certificate_timestamp(shared_realm_id),
            env.get_last_common_certificate_timestamp(),
            shared_realm_id,
            shared_realm_key_index,
            file_id,
            bob.device_id,
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
            p_assert_eq!(*vlob, file_id);
            true
        },
        )
    );
}

#[parsec_test(testbed = "minimal")]
#[case::access_not_available_for_author(
    authenticated_cmds::latest::realm_get_keys_bundle::Rep::AccessNotAvailableForAuthor,
    |err| p_assert_matches!(err, CertifValidateManifestError::NotAllowed),
)]
#[case::author_not_allowed(
    authenticated_cmds::latest::realm_get_keys_bundle::Rep::AuthorNotAllowed,
    |err| p_assert_matches!(err, CertifValidateManifestError::NotAllowed),
)]
#[case::bad_key_index(
    authenticated_cmds::latest::realm_get_keys_bundle::Rep::BadKeyIndex,
    |err| p_assert_matches!(err, CertifValidateManifestError::Internal(_)),
)]
#[case::unknown_status(
    authenticated_cmds::latest::realm_get_keys_bundle::Rep::UnknownStatus { unknown_status: "".into(), reason: None },
    |err| p_assert_matches!(err, CertifValidateManifestError::Internal(_)),
)]
async fn server_error(
    #[case] rep: authenticated_cmds::latest::realm_get_keys_bundle::Rep,
    #[case] assert: impl FnOnce(CertifValidateManifestError),
    env: &TestbedEnv,
) {
    let file_id = VlobID::from_hex("aa00000000000000000000000000f11e").unwrap();

    let (realm_id, vlob, vlob_timestamp) = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.certificates_storage_fetch_certificates("alice@dev1");
            builder
                .create_or_update_file_manifest_vlob(
                    "alice@dev1",
                    realm_id,
                    file_id,
                    Some(realm_id),
                )
                .map(|e| (realm_id, e.encrypted(&env.template), e.manifest.timestamp))
        })
        .await;
    let (_, realm_key_index) = env.get_last_realm_key(realm_id);

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        move |_: authenticated_cmds::latest::realm_get_keys_bundle::Req| rep,
    );

    let err = ops
        .validate_child_manifest(
            env.get_last_realm_certificate_timestamp(realm_id),
            env.get_last_common_certificate_timestamp(),
            realm_id,
            realm_key_index,
            file_id,
            alice.device_id,
            1,
            vlob_timestamp,
            &vlob,
        )
        .await
        .unwrap_err();

    assert(err);
}

#[parsec_test(testbed = "minimal")]
#[case::invalid_keys_bundle(
    |env: &TestbedEnv, realm_id, user_id: UserID| authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
        keys_bundle: Bytes::from_static(b""),
        keys_bundle_access: env.get_last_realm_keys_bundle_access_for(realm_id, user_id),
    },
    |err| p_assert_matches!(err, CertifValidateManifestError::InvalidKeysBundle(_))
)]
#[case::invalid_keys_bundle_access(
    |env: &TestbedEnv, realm_id, _: UserID| authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
        keys_bundle: env.get_last_realm_keys_bundle(realm_id),
        keys_bundle_access: Bytes::from_static(b""),
    },
    |err| p_assert_matches!(err, CertifValidateManifestError::InvalidKeysBundle(_))
)]
async fn invalid_keys_bundle(
    #[case] rep: impl FnOnce(
        &TestbedEnv,
        VlobID,
        UserID,
    ) -> authenticated_cmds::latest::realm_get_keys_bundle::Rep,
    #[case] assert: impl FnOnce(CertifValidateManifestError),
    env: &TestbedEnv,
) {
    let file_id = VlobID::from_hex("aa00000000000000000000000000f11e").unwrap();

    let (realm_id, vlob, vlob_timestamp) = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.certificates_storage_fetch_certificates("alice@dev1");
            builder
                .create_or_update_file_manifest_vlob(
                    "alice@dev1",
                    realm_id,
                    file_id,
                    Some(realm_id),
                )
                .map(|e| (realm_id, e.encrypted(&env.template), e.manifest.timestamp))
        })
        .await;
    let (_, realm_key_index) = env.get_last_realm_key(realm_id);

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    let rep = rep(env, realm_id, alice.user_id);
    test_register_send_hook(
        &env.discriminant_dir,
        move |_: authenticated_cmds::latest::realm_get_keys_bundle::Req| rep,
    );

    let err = ops
        .validate_child_manifest(
            env.get_last_realm_certificate_timestamp(realm_id),
            env.get_last_common_certificate_timestamp(),
            realm_id,
            realm_key_index,
            file_id,
            alice.device_id,
            1,
            vlob_timestamp,
            &vlob,
        )
        .await
        .unwrap_err();

    assert(err);
}

#[parsec_test(testbed = "minimal")]
async fn invalid_response(env: &TestbedEnv) {
    let file_id = VlobID::from_hex("aa00000000000000000000000000f11e").unwrap();

    let (realm_id, vlob, vlob_timestamp) = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.certificates_storage_fetch_certificates("alice@dev1");
            builder
                .create_or_update_file_manifest_vlob(
                    "alice@dev1",
                    realm_id,
                    file_id,
                    Some(realm_id),
                )
                .map(|e| (realm_id, e.encrypted(&env.template), e.manifest.timestamp))
        })
        .await;
    let (_, realm_key_index) = env.get_last_realm_key(realm_id);

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    test_register_low_level_send_hook(&env.discriminant_dir, |_request_builder| async {
        Ok(ResponseMock::Mocked((
            StatusCode::IM_A_TEAPOT,
            HeaderMap::new(),
            Bytes::new(),
        )))
    });

    let err = ops
        .validate_child_manifest(
            env.get_last_realm_certificate_timestamp(realm_id),
            env.get_last_common_certificate_timestamp(),
            realm_id,
            realm_key_index,
            file_id,
            alice.device_id,
            1,
            vlob_timestamp,
            &vlob,
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateManifestError::Offline(ConnectionError::InvalidResponseStatus(
            StatusCode::IM_A_TEAPOT
        ))
    );
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;
    ops.stop().await.unwrap();

    let err = ops
        .validate_child_manifest(
            now,
            now,
            VlobID::default(),
            0,
            VlobID::default(),
            alice.device_id,
            0,
            now,
            b"",
        )
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifValidateManifestError::Stopped);
}
