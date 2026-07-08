// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    test_register_low_level_send_hook, test_register_send_hook,
    test_send_hook_realm_get_keys_bundle, ConnectionError, HeaderMap, ResponseMock, StatusCode,
};
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::certificates_ops_factory;
use crate::{
    CertifValidateCryptpadSessionKeysError, CryptpadSessionKeys, InvalidCryptpadSessionKeysError,
};

#[parsec_test(testbed = "minimal")]
async fn ok_view_only(#[values("view_only", "view_and_edit")] kind: &str, env: &TestbedEnv) {
    let document_id = VlobID::default();

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
    let (key_derivation, key_index) = env.get_last_realm_key(realm_id);
    let key = key_derivation.derive_secret_key_from_uuid(*document_id);
    let created_on = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, realm_id),
    );

    let encrypted_view_key = CryptpadSessionKey {
        author: alice.device_id,
        timestamp: created_on,
        document_id,
        can_edit: false,
        key: "view-key".to_string(),
    }
    .dump_sign_and_encrypt(&alice.signing_key, &key);

    let (encrypted_edit_key, expected_edit_key) = match kind {
        "view_only" => (None, None),
        "view_and_edit" => {
            let encrypted_edit_key = CryptpadSessionKey {
                author: alice.device_id,
                timestamp: created_on,
                document_id,
                can_edit: true,
                key: "edit-key".to_string(),
            }
            .dump_sign_and_encrypt(&alice.signing_key, &key);
            let expected_edit_key = Some("edit-key".to_string());
            (Some(encrypted_edit_key), expected_edit_key)
        }
        unknown => panic!("Unknown kind: {unknown}"),
    };

    let keys = ops
        .validate_cryptpad_session_keys(
            env.get_last_realm_certificate_timestamp(realm_id),
            env.get_last_common_certificate_timestamp(),
            realm_id,
            key_index,
            document_id,
            alice.device_id,
            created_on,
            &encrypted_view_key,
            encrypted_edit_key.as_deref(),
        )
        .await
        .unwrap();

    p_assert_eq!(
        keys,
        CryptpadSessionKeys {
            view_key: "view-key".to_string(),
            edit_key: expected_edit_key,
        }
    );
}

#[parsec_test(testbed = "minimal")]
async fn offline(env: &TestbedEnv) {
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .validate_cryptpad_session_keys(
            now,
            now,
            VlobID::default(),
            0,
            VlobID::default(),
            alice.device_id,
            now,
            b"",
            None,
        )
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifValidateCryptpadSessionKeysError::Offline(_));
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;
    ops.stop().await.unwrap();

    let err = ops
        .validate_cryptpad_session_keys(
            now,
            now,
            VlobID::default(),
            0,
            VlobID::default(),
            alice.device_id,
            now,
            b"",
            None,
        )
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifValidateCryptpadSessionKeysError::Stopped);
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
    let document_id = VlobID::default();
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();
    let (key_derivation, key_index) = env.get_last_realm_key(realm_id);
    let key = key_derivation.derive_secret_key_from_uuid(*document_id);

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, realm_id),
    );

    let err = ops
        .validate_cryptpad_session_keys(
            env.get_last_realm_certificate_timestamp(realm_id),
            env.get_last_common_certificate_timestamp(),
            realm_id,
            key_index,
            document_id,
            DeviceID::default(), // Non-existant device
            now,
            // Decryption is done first, only then the author is searched for
            &key.encrypt(b""),
            None,
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateCryptpadSessionKeysError::InvalidCryptpadSessionKeys(boxed)
        if matches!(*boxed, InvalidCryptpadSessionKeysError::NonExistentAuthor { .. })
    );
}

#[parsec_test(testbed = "minimal")]
async fn revoked_author(env: &TestbedEnv) {
    let realm_id = env
        .customize(|builder| {
            builder.new_user("bob");
            let realm_id = builder
                .new_realm("bob")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.revoke_user("bob");
            builder.certificates_storage_fetch_certificates("bob@dev1");
            realm_id
        })
        .await;
    let document_id = VlobID::default();
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();
    let (key_derivation, key_index) = env.get_last_realm_key(realm_id);
    let key = key_derivation.derive_secret_key_from_uuid(*document_id);

    let bob = env.local_device("bob@dev1");

    let ops = certificates_ops_factory(env, &bob).await;

    test_register_send_hook(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, bob.user_id, realm_id),
    );

    let err = ops
        .validate_cryptpad_session_keys(
            env.get_last_realm_certificate_timestamp(realm_id),
            env.get_last_common_certificate_timestamp(),
            realm_id,
            key_index,
            document_id,
            bob.device_id,
            now,
            // Decryption is done first, only then the author's revocation is checked
            &key.encrypt(b""),
            None,
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateCryptpadSessionKeysError::InvalidCryptpadSessionKeys(boxed)
        if matches!(*boxed, InvalidCryptpadSessionKeysError::RevokedAuthor { .. })
    );
}

#[parsec_test(testbed = "minimal")]
async fn author_no_access_to_realm(env: &TestbedEnv) {
    let document_id = VlobID::default();

    let realm_id = env
        .customize(|builder| {
            builder.new_user("bob");
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|e| e.realm);
            builder.certificates_storage_fetch_certificates("alice@dev1");
            realm_id
        })
        .await;
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();
    let (key_derivation, key_index) = env.get_last_realm_key(realm_id);
    let key = key_derivation.derive_secret_key_from_uuid(*document_id);

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, realm_id),
    );

    let err = ops
        .validate_cryptpad_session_keys(
            env.get_last_realm_certificate_timestamp(realm_id),
            env.get_last_common_certificate_timestamp(),
            realm_id,
            key_index,
            document_id,
            // Bob is not part of this realm
            "bob@dev1".parse().unwrap(),
            now,
            // Decryption is done first, only then realm access is checked
            &key.encrypt(b""),
            None,
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateCryptpadSessionKeysError::InvalidCryptpadSessionKeys(boxed)
        if matches!(*boxed, InvalidCryptpadSessionKeysError::AuthorNoAccessToRealm { .. })
    );
}

#[parsec_test(testbed = "minimal")]
async fn author_cannot_write(env: &TestbedEnv) {
    let document_id = VlobID::default();

    let realm_id = env
        .customize(|builder| {
            builder.new_user("bob");
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|e| e.realm);
            builder.share_realm(realm_id, "bob", Some(RealmRole::Reader));
            builder.certificates_storage_fetch_certificates("bob@dev1");
            realm_id
        })
        .await;
    let created_on = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();
    let (key_derivation, key_index) = env.get_last_realm_key(realm_id);
    let key = key_derivation.derive_secret_key_from_uuid(*document_id);

    let bob = env.local_device("bob@dev1");

    let ops = certificates_ops_factory(env, &bob).await;

    test_register_send_hook(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, bob.user_id, realm_id),
    );

    let err = ops
        .validate_cryptpad_session_keys(
            env.get_last_realm_certificate_timestamp(realm_id),
            env.get_last_common_certificate_timestamp(),
            realm_id,
            key_index,
            document_id,
            bob.device_id,
            created_on,
            // Decryption is done first, only then realm access is checked
            &key.encrypt(b"<view key>"),
            Some(&key.encrypt(b"<edit key>")),
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateCryptpadSessionKeysError::InvalidCryptpadSessionKeys(boxed)
        if matches!(&*boxed, InvalidCryptpadSessionKeysError::AuthorRealmRoleCannotWrite {
            author_role: RealmRole::Reader,
            ..
        })
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
    let document_id = VlobID::default();
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();
    let (_, key_index) = env.get_last_realm_key(realm_id);

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, realm_id),
    );

    let err = ops
        .validate_cryptpad_session_keys(
            env.get_last_realm_certificate_timestamp(realm_id),
            env.get_last_common_certificate_timestamp(),
            realm_id,
            key_index,
            document_id,
            alice.device_id,
            now,
            b"<dummy data>",
            None,
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateCryptpadSessionKeysError::InvalidCryptpadSessionKeys(boxed)
        if matches!(*boxed, InvalidCryptpadSessionKeysError::CannotDecrypt { .. })
    );
}

#[parsec_test(testbed = "minimal")]
async fn cleartext_corrupted(env: &TestbedEnv) {
    let document_id = VlobID::default();

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
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();
    let (key_derivation, key_index) = env.get_last_realm_key(realm_id);
    let key = key_derivation.derive_secret_key_from_uuid(*document_id);

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, realm_id),
    );

    // Valid encryption, but the cleartext isn't a valid signed `CryptpadSessionKey`
    let encrypted = key.encrypt(b"<dummy data>");

    let err = ops
        .validate_cryptpad_session_keys(
            env.get_last_realm_certificate_timestamp(realm_id),
            env.get_last_common_certificate_timestamp(),
            realm_id,
            key_index,
            document_id,
            alice.device_id,
            now,
            &encrypted,
            None,
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateCryptpadSessionKeysError::InvalidCryptpadSessionKeys(boxed)
        if matches!(&*boxed, InvalidCryptpadSessionKeysError::CleartextCorrupted { error, .. } if **error == DataError::Signature)
    );
}

#[parsec_test(testbed = "minimal")]
async fn unexpected_key_kind(env: &TestbedEnv) {
    let document_id = VlobID::default();

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
    let created_on = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();
    let (key_derivation, key_index) = env.get_last_realm_key(realm_id);
    let key = key_derivation.derive_secret_key_from_uuid(*document_id);

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, realm_id),
    );

    // The bytes provided as the view key actually contain an edit key
    let encrypted_view_key = CryptpadSessionKey {
        author: alice.device_id,
        timestamp: created_on,
        document_id,
        can_edit: true,
        key: "actually-an-edit-key".to_string(),
    }
    .dump_sign_and_encrypt(&alice.signing_key, &key);

    let err = ops
        .validate_cryptpad_session_keys(
            env.get_last_realm_certificate_timestamp(realm_id),
            env.get_last_common_certificate_timestamp(),
            realm_id,
            key_index,
            document_id,
            alice.device_id,
            created_on,
            &encrypted_view_key,
            None,
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateCryptpadSessionKeysError::InvalidCryptpadSessionKeys(boxed)
        if matches!(&*boxed, InvalidCryptpadSessionKeysError::CleartextCorrupted {
            ..
        })
    );
}

#[parsec_test(testbed = "minimal")]
async fn non_existent_key_index(env: &TestbedEnv) {
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
    let document_id = VlobID::default();
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, realm_id),
    );

    let err = ops
        .validate_cryptpad_session_keys(
            env.get_last_realm_certificate_timestamp(realm_id),
            env.get_last_common_certificate_timestamp(),
            realm_id,
            42,
            document_id,
            alice.device_id,
            now,
            b"<dummy encrypted>",
            None,
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateCryptpadSessionKeysError::InvalidCryptpadSessionKeys(boxed)
        if matches!(*boxed, InvalidCryptpadSessionKeysError::NonExistentKeyIndex { .. })
    );
}

#[parsec_test(testbed = "minimal")]
async fn corrupted_key(env: &TestbedEnv) {
    let realm_id = env
        .customize(|builder| {
            let realm_id = builder
                .new_realm("alice")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            // We do another key rotation to be able to provide our own keys bundle
            // containing a corrupted key for key index 1 (the key we will use).
            builder.rotate_key_realm(realm_id).customize(|e| {
                p_assert_eq!(e.keys.len(), 2);
                e.keys[0] = KeyDerivation::generate();
            });
            builder.certificates_storage_fetch_certificates("alice@dev1");
            realm_id
        })
        .await;
    let document_id = VlobID::default();
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        test_send_hook_realm_get_keys_bundle!(env, alice.user_id, realm_id),
    );

    let err = ops
        .validate_cryptpad_session_keys(
            env.get_last_realm_certificate_timestamp(realm_id),
            env.get_last_common_certificate_timestamp(),
            realm_id,
            1,
            document_id,
            alice.device_id,
            now,
            b"<dummy encrypted>",
            None,
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateCryptpadSessionKeysError::InvalidCryptpadSessionKeys(boxed)
        if matches!(*boxed, InvalidCryptpadSessionKeysError::CorruptedKey { .. })
    );
}

#[parsec_test(testbed = "minimal")]
#[case::invalid_keys_bundle(
    |env: &TestbedEnv, realm_id, user_id: UserID| authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
        keys_bundle: Bytes::from_static(b""),
        keys_bundle_access: env.get_last_realm_keys_bundle_access_for(realm_id, user_id),
    },
    |err| p_assert_matches!(err, CertifValidateCryptpadSessionKeysError::InvalidKeysBundle(_))
)]
#[case::invalid_keys_bundle_access(
    |env: &TestbedEnv, realm_id, _: UserID| authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
        keys_bundle: env.get_last_realm_keys_bundle(realm_id),
        keys_bundle_access: Bytes::from_static(b""),
    },
    |err| p_assert_matches!(err, CertifValidateCryptpadSessionKeysError::InvalidKeysBundle(_))
)]
async fn invalid_keys_bundle(
    #[case] rep: impl FnOnce(
        &TestbedEnv,
        VlobID,
        UserID,
    ) -> authenticated_cmds::latest::realm_get_keys_bundle::Rep,
    #[case] assert: impl FnOnce(CertifValidateCryptpadSessionKeysError),
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
    let document_id = VlobID::default();
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();
    let (_, key_index) = env.get_last_realm_key(realm_id);

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    let rep = rep(env, realm_id, alice.user_id);
    test_register_send_hook(
        &env.discriminant_dir,
        move |_: authenticated_cmds::latest::realm_get_keys_bundle::Req| rep,
    );

    let err = ops
        .validate_cryptpad_session_keys(
            env.get_last_realm_certificate_timestamp(realm_id),
            env.get_last_common_certificate_timestamp(),
            realm_id,
            key_index,
            document_id,
            alice.device_id,
            now,
            b"<dummy encrypted>",
            None,
        )
        .await
        .unwrap_err();

    assert(err);
}

#[parsec_test(testbed = "minimal")]
#[case::access_not_available_for_author(
    authenticated_cmds::latest::realm_get_keys_bundle::Rep::AccessNotAvailableForAuthor,
    |err| p_assert_matches!(err, CertifValidateCryptpadSessionKeysError::NotAllowed),
)]
#[case::author_not_allowed(
    authenticated_cmds::latest::realm_get_keys_bundle::Rep::AuthorNotAllowed,
    |err| p_assert_matches!(err, CertifValidateCryptpadSessionKeysError::NotAllowed),
)]
#[case::bad_key_index(
    authenticated_cmds::latest::realm_get_keys_bundle::Rep::BadKeyIndex,
    |err| p_assert_matches!(err, CertifValidateCryptpadSessionKeysError::Internal(_)),
)]
#[case::unknown_status(
    authenticated_cmds::latest::realm_get_keys_bundle::Rep::UnknownStatus { unknown_status: "".into(), reason: None },
    |err| p_assert_matches!(err, CertifValidateCryptpadSessionKeysError::Internal(_)),
)]
async fn server_error(
    #[case] rep: authenticated_cmds::latest::realm_get_keys_bundle::Rep,
    #[case] assert: impl FnOnce(CertifValidateCryptpadSessionKeysError),
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
    let document_id = VlobID::default();
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();
    let (_, key_index) = env.get_last_realm_key(realm_id);

    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        move |_: authenticated_cmds::latest::realm_get_keys_bundle::Req| rep,
    );

    let err = ops
        .validate_cryptpad_session_keys(
            env.get_last_realm_certificate_timestamp(realm_id),
            env.get_last_common_certificate_timestamp(),
            realm_id,
            key_index,
            document_id,
            alice.device_id,
            now,
            b"<dummy encrypted>",
            None,
        )
        .await
        .unwrap_err();

    assert(err);
}

#[parsec_test(testbed = "minimal")]
async fn invalid_response(env: &TestbedEnv) {
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
    let document_id = VlobID::default();
    let now = DateTime::from_ymd_hms_us(2020, 1, 1, 0, 0, 0, 0).unwrap();
    let (_, key_index) = env.get_last_realm_key(realm_id);

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
        .validate_cryptpad_session_keys(
            env.get_last_realm_certificate_timestamp(realm_id),
            env.get_last_common_certificate_timestamp(),
            realm_id,
            key_index,
            document_id,
            alice.device_id,
            now,
            b"<dummy encrypted>",
            None,
        )
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifValidateCryptpadSessionKeysError::Offline(ConnectionError::InvalidResponseStatus(
            StatusCode::IM_A_TEAPOT
        ))
    );
}
