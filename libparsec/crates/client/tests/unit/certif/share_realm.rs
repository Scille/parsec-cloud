// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    test_register_low_level_send_hook, test_register_send_hook,
    test_register_sequence_of_send_hooks, HeaderMap, ResponseMock, StatusCode,
};
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certif::{CertifShareRealmError, CertificateBasedActionOutcome};

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        builder.new_user("bob");

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
    let keys_bundle_access = env.get_last_realm_keys_bundle_access_for(realm_id, alice.user_id());
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        {
            move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                p_assert_eq!(req.key_index, 1);
                p_assert_eq!(req.realm_id, realm_id);

                authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                    keys_bundle,
                    keys_bundle_access,
                }
            }
        },
        {
            move |req: authenticated_cmds::latest::realm_share::Req| {
                p_assert_eq!(req.key_index, 1);
                authenticated_cmds::latest::realm_share::Rep::Ok
            }
        },
    );

    let res = ops
        .share_realm(
            realm_id,
            "bob".parse().unwrap(),
            Some(RealmRole::Contributor),
        )
        .await
        .unwrap();

    p_assert_matches!(res, CertificateBasedActionOutcome::Uploaded { .. });
}

#[parsec_test(testbed = "minimal")]
async fn recipient_not_found(env: &TestbedEnv) {
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

    let err = ops
        .share_realm(
            realm_id,
            "bob".parse().unwrap(),
            Some(RealmRole::Contributor),
        )
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifShareRealmError::RecipientNotFound);
}

#[parsec_test(testbed = "minimal")]
#[case::invalid_keys_bundle(
    |env: &TestbedEnv, realm_id, user_id: &UserID| authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
        keys_bundle: Bytes::from_static(b""),
        keys_bundle_access: env.get_last_realm_keys_bundle_access_for(realm_id, user_id),
    },
    |err| p_assert_matches!(err, CertifShareRealmError::InvalidKeysBundle(_))
)]
#[case::invalid_keys_bundle_access(
    |env: &TestbedEnv, realm_id, _: &UserID| authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
        keys_bundle: env.get_last_realm_keys_bundle(realm_id),
        keys_bundle_access: Bytes::from_static(b""),
    },
    |err| p_assert_matches!(err, CertifShareRealmError::InvalidKeysBundle(_))
)]
async fn invalid_keys_bundle(
    #[case] rep: impl FnOnce(
        &TestbedEnv,
        VlobID,
        &UserID,
    ) -> authenticated_cmds::latest::realm_get_keys_bundle::Rep,
    #[case] assert: impl FnOnce(CertifShareRealmError),
    env: &TestbedEnv,
) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        builder.new_user("bob");

        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);

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

    let err = ops
        .share_realm(
            realm_id,
            "bob".parse().unwrap(),
            Some(RealmRole::Contributor),
        )
        .await
        .unwrap_err();

    assert(err);
}

#[parsec_test(testbed = "minimal")]
#[case::recipient_not_found(
    authenticated_cmds::latest::realm_share::Rep::RecipientNotFound,
    |err| p_assert_matches!(err, CertifShareRealmError::RecipientNotFound),
)]
#[case::recipient_revoked(
    authenticated_cmds::latest::realm_share::Rep::RecipientRevoked,
    |err| p_assert_matches!(err, CertifShareRealmError::RecipientRevoked),
)]
#[case::author_not_allowed(
    authenticated_cmds::latest::realm_share::Rep::AuthorNotAllowed,
    |err| p_assert_matches!(err, CertifShareRealmError::AuthorNotAllowed),
)]
#[case::realm_not_found(
    authenticated_cmds::latest::realm_share::Rep::RealmNotFound,
    |err| p_assert_matches!(err, CertifShareRealmError::RealmNotFound),
)]
#[case::role_incompatible_with_outsider(
    authenticated_cmds::latest::realm_share::Rep::RoleIncompatibleWithOutsider,
    |err| p_assert_matches!(err, CertifShareRealmError::RoleIncompatibleWithOutsider),
)]
#[case::timestamp_out_of_ballpark(
    authenticated_cmds::latest::realm_share::Rep::TimestampOutOfBallpark {
        ballpark_client_early_offset: 300.,
        ballpark_client_late_offset: 320.,
        client_timestamp: "2000-01-02T00:00:00Z".parse().unwrap(),
        server_timestamp: "2000-01-02T00:00:00Z".parse().unwrap(),
    },
    |err| p_assert_matches!(
        err,
        CertifShareRealmError::TimestampOutOfBallpark {
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            client_timestamp,
            server_timestamp,
        }
        if ballpark_client_early_offset == 300.
            && ballpark_client_late_offset == 320.
            && client_timestamp == "2000-01-02T00:00:00Z".parse().unwrap()
            && server_timestamp == "2000-01-02T00:00:00Z".parse().unwrap(),
    )
)]
#[case::require_greater_timestamp(
    authenticated_cmds::latest::realm_share::Rep::RequireGreaterTimestamp {
        strictly_greater_than: "2000-01-02T00:00:00Z".parse().unwrap(),
    },
    |err| p_assert_matches!(err, CertifShareRealmError::Offline),
)]
#[case::invalid_certificate(
    authenticated_cmds::latest::realm_share::Rep::InvalidCertificate,
    |err| p_assert_matches!(err, CertifShareRealmError::Internal(_)),
)]
#[case::bad_key_index(
    authenticated_cmds::latest::realm_share::Rep::BadKeyIndex {
        last_realm_certificate_timestamp: "2000-01-02T00:00:00Z".parse().unwrap(),
    },
    |err| p_assert_matches!(err, CertifShareRealmError::Offline),
)]
#[case::unknown_status(
    authenticated_cmds::latest::realm_share::Rep::UnknownStatus { unknown_status: "".into(), reason: None },
    |err| p_assert_matches!(err, CertifShareRealmError::Internal(_)),
)]
async fn server_error(
    #[case] rep: authenticated_cmds::latest::realm_share::Rep,
    #[case] assert: impl FnOnce(CertifShareRealmError),
    env: &TestbedEnv,
) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        builder.new_user("bob");

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
    let keys_bundle_access = env.get_last_realm_keys_bundle_access_for(realm_id, alice.user_id());
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        {
            move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                p_assert_eq!(req.key_index, 1);
                p_assert_eq!(req.realm_id, realm_id);

                authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                    keys_bundle,
                    keys_bundle_access,
                }
            }
        },
        {
            move |req: authenticated_cmds::latest::realm_share::Req| {
                p_assert_eq!(req.key_index, 1);
                rep
            }
        },
    );

    let err = ops
        .share_realm(
            realm_id,
            "bob".parse().unwrap(),
            Some(RealmRole::Contributor),
        )
        .await
        .unwrap_err();

    assert(err);
}

#[parsec_test(testbed = "minimal")]
async fn server_role_already_granted(env: &TestbedEnv) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        builder.new_user("bob");

        let realm_id = builder
            .new_realm("alice")
            .then_do_initial_key_rotation()
            .map(|event| event.realm);

        builder.certificates_storage_fetch_certificates("alice@dev1");

        realm_id
    });
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(&env, &alice).await;

    let timestamp = alice.time_provider.now();
    let keys_bundle = env.get_last_realm_keys_bundle(realm_id);
    let keys_bundle_access = env.get_last_realm_keys_bundle_access_for(realm_id, alice.user_id());
    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        {
            move |req: authenticated_cmds::latest::realm_get_keys_bundle::Req| {
                p_assert_eq!(req.key_index, 1);
                p_assert_eq!(req.realm_id, realm_id);

                authenticated_cmds::latest::realm_get_keys_bundle::Rep::Ok {
                    keys_bundle,
                    keys_bundle_access,
                }
            }
        },
        {
            move |req: authenticated_cmds::latest::realm_share::Req| {
                p_assert_eq!(req.key_index, 1);
                authenticated_cmds::latest::realm_share::Rep::RoleAlreadyGranted {
                    last_realm_certificate_timestamp: timestamp,
                }
            }
        },
    );

    let res = ops
        .share_realm(
            realm_id,
            "bob".parse().unwrap(),
            Some(RealmRole::Contributor),
        )
        .await
        .unwrap();

    p_assert_matches!(
        res,
        CertificateBasedActionOutcome::RemoteIdempotent { certificate_timestamp }
        if certificate_timestamp == timestamp
    );
}

#[parsec_test(testbed = "minimal")]
async fn recipient_is_self(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .share_realm(
            VlobID::default(),
            "alice".parse().unwrap(),
            Some(RealmRole::Contributor),
        )
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifShareRealmError::RecipientIsSelf);
}

#[parsec_test(testbed = "minimal")]
async fn unknown_realm(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .share_realm(
            VlobID::default(),
            "bob".parse().unwrap(),
            Some(RealmRole::Contributor),
        )
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifShareRealmError::NoKey);
}

#[parsec_test(testbed = "minimal")]
async fn offline(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .share_realm(VlobID::default(), "bob".parse().unwrap(), None)
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifShareRealmError::Offline);
}

#[parsec_test(testbed = "minimal")]
async fn invalid_response(env: &TestbedEnv) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        builder.new_user("bob");

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

    let err = ops
        .share_realm(
            realm_id,
            "bob".parse().unwrap(),
            Some(RealmRole::Contributor),
        )
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifShareRealmError::Internal(_));
}

#[parsec_test(testbed = "minimal")]
async fn stopped(env: &TestbedEnv) {
    let (env, realm_id) = env.customize_with_map(|builder| {
        builder.new_user("bob");

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

    let err = ops
        .share_realm(
            realm_id,
            "bob".parse().unwrap(),
            Some(RealmRole::Contributor),
        )
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifShareRealmError::Stopped);
}
