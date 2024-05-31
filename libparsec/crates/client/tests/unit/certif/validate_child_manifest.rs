// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::num::NonZeroU64;

use libparsec_client_connection::{
    test_register_low_level_send_hook, test_register_send_hook, HeaderMap, ResponseMock, StatusCode,
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

    let keys_bundle = env.get_last_realm_keys_bundle(realm_id);
    let keys_bundle_access = env.get_last_realm_keys_bundle_access_for(realm_id, alice.user_id);
    test_register_send_hook(
        &env.discriminant_dir,
        move |_: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
            authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                keys_bundle,
                keys_bundle_access,
            }
        },
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

    p_assert_matches!(err, CertifValidateManifestError::Offline);
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

    let keys_bundle = env.get_last_realm_keys_bundle(realm_id);
    let keys_bundle_access = env.get_last_realm_keys_bundle_access_for(realm_id, alice.user_id);
    test_register_send_hook(
        &env.discriminant_dir,
        move |_: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
            authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                keys_bundle,
                keys_bundle_access,
            }
        },
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

    let keys_bundle = env.get_last_realm_keys_bundle(realm_id);
    let keys_bundle_access = env.get_last_realm_keys_bundle_access_for(realm_id, alice.user_id);
    test_register_send_hook(
        &env.discriminant_dir,
        move |_: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
            authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                keys_bundle,
                keys_bundle_access,
            }
        },
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
    #[values("dummy", "parent_pointing_on_itself")] kind: &str,
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

    let encrypted = match kind {
        "dummy" => key.encrypt(b"<dummy data>"),
        "parent_pointing_on_itself" => {
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
            manifest.dump_sign_and_encrypt(&alice.signing_key, &key)
        }
        unknown => panic!("Unknown kind {}", unknown),
    };

    let ops = certificates_ops_factory(env, &alice).await;

    let keys_bundle = env.get_last_realm_keys_bundle(realm_id);
    let keys_bundle_access = env.get_last_realm_keys_bundle_access_for(realm_id, alice.user_id);
    test_register_send_hook(
        &env.discriminant_dir,
        move |_: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
            authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                keys_bundle,
                keys_bundle_access,
            }
        },
    );

    let err = ops
        .validate_child_manifest(
            env.get_last_realm_certificate_timestamp(realm_id),
            env.get_last_common_certificate_timestamp(),
            realm_id,
            realm_key_index,
            child_id,
            alice.device_id,
            0,
            now,
            &encrypted,
        )
        .await
        .unwrap_err();

    match kind {
        "dummy" => p_assert_matches!(
            err,
            CertifValidateManifestError::InvalidManifest(boxed)
            if matches!(&*boxed, InvalidManifestError::CleartextCorrupted { error, .. } if **error == DataError::Decryption)
        ),
        "parent_pointing_on_itself" => p_assert_matches!(
            err,
            CertifValidateManifestError::InvalidManifest(boxed)
            if matches!(&*boxed, InvalidManifestError::CleartextCorrupted { error, .. } if matches!(**error, DataError::BadSerialization { .. }))
        ),
        unknown => panic!("Unknown kind {}", unknown),
    }
}

#[parsec_test(testbed = "minimal")]
async fn content_corrupted(
    #[values(
        "blocks_not_sorted",
        "blocks_overlapping",
        "exceed_file_size",
        "same_block_span",
        "in_between_block_span"
    )]
    kind: &str,
    env: &TestbedEnv,
) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);
        builder.certificates_storage_fetch_certificates("alice@dev1");
        realm_id
    });
    let (realm_last_key, realm_key_index) = env.get_last_realm_key(realm_id);

    let child_id = VlobID::default();
    let parent_id = VlobID::default();

    let alice = env.local_device("alice@dev1");
    let blocksize = Blocksize::try_from(1024).unwrap();

    let default_block_id = BlockID::from_hex("00000000000000000000000000000001").unwrap();
    let default_digest = HashDigest::from_data(&[]);
    let default_block_size = NonZeroU64::new(1024).unwrap();

    let blocks = match kind {
        "blocks_not_sorted" => {
            vec![
                BlockAccess {
                    id: default_block_id,
                    size: default_block_size,
                    offset: 1024,
                    digest: default_digest.clone(),
                },
                BlockAccess {
                    id: default_block_id,
                    size: default_block_size,
                    offset: 0,
                    digest: default_digest.clone(),
                },
            ]
        }
        "blocks_overlapping" => {
            vec![
                BlockAccess {
                    id: default_block_id,
                    size: NonZeroU64::new(10).unwrap(),
                    offset: 0,
                    digest: default_digest.clone(),
                },
                BlockAccess {
                    id: default_block_id,
                    size: NonZeroU64::new(10).unwrap(),
                    offset: 5,
                    digest: default_digest.clone(),
                },
            ]
        }
        "exceed_file_size" => {
            vec![BlockAccess {
                id: default_block_id,
                size: default_block_size,
                offset: 2048,
                digest: default_digest.clone(),
            }]
        }
        "same_block_span" => {
            vec![
                BlockAccess {
                    id: default_block_id,
                    size: NonZeroU64::new(10).unwrap(),
                    offset: 0,
                    digest: default_digest.clone(),
                },
                BlockAccess {
                    id: default_block_id,
                    size: NonZeroU64::new(10).unwrap(),
                    offset: 10,
                    digest: default_digest.clone(),
                },
            ]
        }
        "in_between_block_span" => {
            vec![BlockAccess {
                id: default_block_id,
                size: default_block_size,
                offset: 512,
                digest: default_digest.clone(),
            }]
        }
        unknown => panic!("Unknown kind {}", unknown),
    };

    let now = "2020-01-01T00:00:00.000000Z".parse().unwrap();
    let manifest = FileManifest {
        author: alice.device_id.clone(),
        timestamp: now,
        id: child_id,
        parent: parent_id,
        version: 1,
        created: now,
        updated: now,
        blocks,
        blocksize,
        size: 2 * 1024,
    };
    let encrypted = manifest.dump_sign_and_encrypt(&alice.signing_key, realm_last_key);

    let ops = certificates_ops_factory(&env, &alice).await;

    let keys_bundle = env.get_last_realm_keys_bundle(realm_id);
    let keys_bundle_access = env.get_last_realm_keys_bundle_access_for(realm_id, alice.user_id());
    test_register_send_hook(
        &env.discriminant_dir,
        move |_: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
            authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                keys_bundle,
                keys_bundle_access,
            }
        },
    );

    let err = ops
        .validate_child_manifest(
            env.get_last_realm_certificate_timestamp(realm_id),
            env.get_last_common_certificate_timestamp(),
            realm_id,
            realm_key_index,
            child_id,
            &alice.device_id,
            1,
            now,
            &encrypted,
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateManifestError::InvalidManifest(boxed)
        if matches!(&*boxed, InvalidManifestError::CleartextCorrupted { error, .. } if **error == DataError::InvalidFileContent)
    );
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
        e => panic!("Unexpected event {:?}", e),
    };

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    let keys_bundle = env.get_last_realm_keys_bundle(realm_id);
    let keys_bundle_access = env.get_last_realm_keys_bundle_access_for(realm_id, alice.user_id);
    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
            p_assert_eq!(req.realm_id, realm_id);
            authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                keys_bundle,
                keys_bundle_access,
            }
        },
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
        e => panic!("Unexpected event {:?}", e),
    };

    let keys_bundle = env.get_last_realm_keys_bundle(realm_id);
    let keys_bundle_access = env.get_last_realm_keys_bundle_access_for(realm_id, bob.user_id);
    test_register_send_hook(
        &env.discriminant_dir,
        move |_: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
            authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                keys_bundle,
                keys_bundle_access,
            }
        },
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

    let alice = env.local_device("alice@dev1");
    let bob = env.local_device("bob@dev1");

    let ops = certificates_ops_factory(env, &bob).await;

    let (vlob, vlob_timestamp) = match env.template.events.iter().last().unwrap() {
        TestbedEvent::CreateOrUpdateFileManifestVlob(e) => {
            (e.encrypted(&env.template), e.manifest.timestamp)
        }
        e => panic!("Unexpected event {:?}", e),
    };

    let keys_bundle = env.get_last_realm_keys_bundle(shared_realm_id);
    let keys_bundle_access = {
        let alice_keys_bundle_access =
            env.get_last_realm_keys_bundle_access_for(shared_realm_id, alice.user_id);
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
                keys_bundle,
                keys_bundle_access,
            }
        },
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

    p_assert_matches!(err, CertifValidateManifestError::Internal(_));
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
