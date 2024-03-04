// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    test_register_low_level_send_hook, test_register_send_hook, HeaderMap, ResponseMock, StatusCode,
};
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certif::CertifDecryptCurrentRealmNameError;

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let (env, (realm_id, name, timestamp)) = env.customize_with_map(|builder| {
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);

        let (name, timestamp) = builder
            .rename_realm(realm_id, "wkp1")
            .map(|event| (event.name.clone(), event.timestamp));

        builder.certificates_storage_fetch_certificates("alice@dev1");

        (realm_id, name, timestamp)
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let keys_bundle = env.get_last_realm_keys_bundle(realm_id);
    let keys_bundle_access = env.get_last_realm_keys_bundle_access_for(realm_id, alice.user_id());
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

    let res = ops.decrypt_current_realm_name(realm_id).await.unwrap();

    p_assert_eq!(res, (name, timestamp));
}

#[parsec_test(testbed = "minimal")]
#[case::invalid_keys_bundle(
    |env: &TestbedEnv, realm_id, user_id: &UserID| authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
        keys_bundle: Bytes::from_static(b""),
        keys_bundle_access: env.get_last_realm_keys_bundle_access_for(realm_id, user_id),
    },
    |err| p_assert_matches!(err, CertifDecryptCurrentRealmNameError::InvalidKeysBundle(_))
)]
#[case::invalid_keys_bundle_access(
    |env: &TestbedEnv, realm_id, _: &UserID| authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
        keys_bundle: env.get_last_realm_keys_bundle(realm_id),
        keys_bundle_access: Bytes::from_static(b""),
    },
    |err| p_assert_matches!(err, CertifDecryptCurrentRealmNameError::InvalidKeysBundle(_))
)]
async fn invalid_keys_bundle(
    #[case] rep: impl FnOnce(
        &TestbedEnv,
        VlobID,
        &UserID,
    ) -> authenticated_cmds::latest::realm_get_keys_bundle::Rep,
    #[case] assert: impl FnOnce(CertifDecryptCurrentRealmNameError),
    env: &TestbedEnv,
) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);

        builder.rename_realm(realm_id, "wkp1");

        builder.certificates_storage_fetch_certificates("alice@dev1");

        realm_id
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let rep = rep(&env, realm_id, alice.user_id());
    test_register_send_hook(&env.discriminant_dir, {
        move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
            p_assert_eq!(req.key_index, 1);
            p_assert_eq!(req.realm_id, realm_id);

            rep
        }
    });

    let err = ops.decrypt_current_realm_name(realm_id).await.unwrap_err();

    assert(err);
}

#[parsec_test(testbed = "minimal")]
#[case::access_not_available_for_author(
    authenticated_cmds::latest::realm_get_keys_bundle::Rep::AccessNotAvailableForAuthor,
    |err| p_assert_matches!(err, CertifDecryptCurrentRealmNameError::NotAllowed),
)]
#[case::author_not_allowed(
    authenticated_cmds::latest::realm_get_keys_bundle::Rep::AuthorNotAllowed,
    |err| p_assert_matches!(err, CertifDecryptCurrentRealmNameError::NotAllowed),
)]
#[case::bad_key_index(
    authenticated_cmds::latest::realm_get_keys_bundle::Rep::BadKeyIndex,
    |err| p_assert_matches!(err, CertifDecryptCurrentRealmNameError::Internal(_)),
)]
#[case::unknown_status(
    authenticated_cmds::latest::realm_get_keys_bundle::Rep::UnknownStatus { unknown_status: "".into(), reason: None },
    |err| p_assert_matches!(err, CertifDecryptCurrentRealmNameError::Internal(_)),
)]
async fn server_error(
    #[case] rep: authenticated_cmds::latest::realm_get_keys_bundle::Rep,
    #[case] assert: impl FnOnce(CertifDecryptCurrentRealmNameError),
    env: &TestbedEnv,
) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);

        builder.rename_realm(realm_id, "wkp1");
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

    let err = ops.decrypt_current_realm_name(realm_id).await.unwrap_err();

    assert(err);
}

#[parsec_test(testbed = "minimal")]
async fn offline(env: &TestbedEnv) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);

        builder.rename_realm(realm_id, "wkp1");
        builder.certificates_storage_fetch_certificates("alice@dev1");

        realm_id
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let err = ops.decrypt_current_realm_name(realm_id).await.unwrap_err();

    p_assert_matches!(err, CertifDecryptCurrentRealmNameError::Offline);
}

#[parsec_test(testbed = "minimal")]
async fn no_name_certificate(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .decrypt_current_realm_name(VlobID::default())
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifDecryptCurrentRealmNameError::NoNameCertificate);
}

#[parsec_test(testbed = "minimal")]
async fn invalid_response(env: &TestbedEnv) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);

        builder.rename_realm(realm_id, "wkp1");

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

    let err = ops.decrypt_current_realm_name(realm_id).await.unwrap_err();

    p_assert_matches!(err, CertifDecryptCurrentRealmNameError::Internal(_));
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);

        builder.rename_realm(realm_id, "wkp1");

        builder.certificates_storage_fetch_certificates("alice@dev1");

        realm_id
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    ops.stop().await.unwrap();

    let err = ops.decrypt_current_realm_name(realm_id).await.unwrap_err();

    p_assert_matches!(err, CertifDecryptCurrentRealmNameError::Stopped);
}
