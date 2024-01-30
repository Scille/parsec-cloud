// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    test_register_low_level_send_hook, test_register_send_hook, HeaderMap, ResponseMock, StatusCode,
};
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certif::CertifEncryptForRealmError;

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);
        builder.certificates_storage_fetch_certificates("alice@dev1");

        realm_id
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let keys_bundle = env.get_last_realm_keys_bundle(realm_id);
    let keys_bundle_access = env.get_last_realm_keys_bundle_access_for(realm_id, &alice.user_id());
    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
            p_assert_eq!(req.key_index, 1);
            p_assert_eq!(req.realm_id, realm_id);

            authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                keys_bundle,
                keys_bundle_access,
            }
        },
    );

    let res = ops.encrypt_for_realm(realm_id, b"data").await.unwrap();

    p_assert_eq!(res.1, 1);
}

#[parsec_test(testbed = "minimal")]
#[case::invalid_keys_bundle(
    |env: &TestbedEnv, realm_id, user_id: &UserID| authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
        keys_bundle: Bytes::from_static(b""),
        keys_bundle_access: env.get_last_realm_keys_bundle_access_for(realm_id, user_id),
    },
    |err| p_assert_matches!(err, CertifEncryptForRealmError::InvalidKeysBundle(_))
)]
#[case::invalid_keys_bundle_access(
    |env: &TestbedEnv, realm_id, _: &UserID| authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
        keys_bundle: env.get_last_realm_keys_bundle(realm_id),
        keys_bundle_access: Bytes::from_static(b""),
    },
    |err| p_assert_matches!(err, CertifEncryptForRealmError::InvalidKeysBundle(_))
)]
async fn invalid_keys_bundle(
    #[case] rep: impl FnOnce(
        &TestbedEnv,
        VlobID,
        &UserID,
    ) -> authenticated_cmds::latest::realm_get_keys_bundle::Rep,
    #[case] assert: impl FnOnce(CertifEncryptForRealmError),
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
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let rep = rep(&env, realm_id, &alice.user_id());
    test_register_send_hook(&env.discriminant_dir, {
        move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
            p_assert_eq!(req.key_index, 1);
            p_assert_eq!(req.realm_id, realm_id);

            rep
        }
    });

    let err = ops.encrypt_for_realm(realm_id, b"data").await.unwrap_err();

    assert(err);
}

#[parsec_test(testbed = "minimal")]
#[case::access_not_available_for_author(
    authenticated_cmds::latest::realm_get_keys_bundle::Rep::AccessNotAvailableForAuthor,
    |err| p_assert_matches!(err, CertifEncryptForRealmError::NotAllowed),
)]
#[case::author_not_allowed(
    authenticated_cmds::latest::realm_get_keys_bundle::Rep::AuthorNotAllowed,
    |err| p_assert_matches!(err, CertifEncryptForRealmError::NotAllowed),
)]
#[case::bad_key_index(
    authenticated_cmds::latest::realm_get_keys_bundle::Rep::BadKeyIndex,
    |err| p_assert_matches!(err, CertifEncryptForRealmError::Internal(_)),
)]
#[case::unknown_status(
    authenticated_cmds::latest::realm_get_keys_bundle::Rep::UnknownStatus { unknown_status: "".into(), reason: None },
    |err| p_assert_matches!(err, CertifEncryptForRealmError::Internal(_)),
)]
async fn server_error(
    #[case] rep: authenticated_cmds::latest::realm_get_keys_bundle::Rep,
    #[case] assert: impl FnOnce(CertifEncryptForRealmError),
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
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
            p_assert_eq!(req.key_index, 1);
            p_assert_eq!(req.realm_id, realm_id);

            rep
        },
    );

    let err = ops.encrypt_for_realm(realm_id, b"data").await.unwrap_err();

    assert(err);
}

#[parsec_test(testbed = "minimal")]
async fn unknown_realm(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .encrypt_for_realm(VlobID::default(), b"data")
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifEncryptForRealmError::NoKey);
}

#[parsec_test(testbed = "minimal")]
async fn invalid_response(env: &TestbedEnv) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);
        builder.certificates_storage_fetch_certificates("alice@dev1");

        realm_id
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    test_register_low_level_send_hook(&env.discriminant_dir, |_request_builder| async {
        Ok(ResponseMock::Mocked((
            StatusCode::IM_A_TEAPOT,
            HeaderMap::new(),
            Bytes::new(),
        )))
    });

    let err = ops.encrypt_for_realm(realm_id, b"data").await.unwrap_err();

    p_assert_matches!(err, CertifEncryptForRealmError::Internal(_));
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);
        builder.certificates_storage_fetch_certificates("alice@dev1");

        realm_id
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    ops.stop().await.unwrap();

    let err = ops.encrypt_for_realm(realm_id, b"data").await.unwrap_err();

    p_assert_matches!(err, CertifEncryptForRealmError::Stopped);
}
