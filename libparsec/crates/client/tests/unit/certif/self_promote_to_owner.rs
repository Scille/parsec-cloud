// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    test_register_low_level_send_hook, test_register_sequence_of_send_hooks, ConnectionError,
    HeaderMap, ResponseMock, StatusCode,
};
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certif::{CertifSelfPromoteToOwnerError, CertificateBasedActionOutcome};

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let realm_id = env
        .customize(|builder| {
            builder.new_user("bob");
            let realm_id = builder
                .new_realm("bob")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.share_realm(realm_id, "alice", RealmRole::Manager);
            builder.revoke_user("bob");
            builder.certificates_storage_fetch_certificates("alice@dev1");
            realm_id
        })
        .await;

    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        move |_req: authenticated_cmds::latest::realm_self_promote_to_owner::Req| {
            authenticated_cmds::latest::realm_self_promote_to_owner::Rep::Ok
        }
    },);

    let res = ops.self_promote_to_owner(realm_id).await.unwrap();

    p_assert_matches!(res, CertificateBasedActionOutcome::Uploaded { .. });
}

#[parsec_test(testbed = "minimal")]
#[case::active_owner_already_exists(
    authenticated_cmds::latest::realm_self_promote_to_owner::Rep::ActiveOwnerAlreadyExists {
        last_realm_certificate_timestamp: "2000-01-02T00:00:00Z".parse().unwrap(),
    },
    |err| p_assert_matches!(err, CertifSelfPromoteToOwnerError::ActiveOwnerAlreadyExists)
)]
#[case::author_not_allowed(
    authenticated_cmds::latest::realm_self_promote_to_owner::Rep::AuthorNotAllowed,
    |err| p_assert_matches!(err, CertifSelfPromoteToOwnerError::AuthorNotAllowed)
)]
#[case::realm_not_found(
    authenticated_cmds::latest::realm_self_promote_to_owner::Rep::RealmNotFound,
    |err| p_assert_matches!(err, CertifSelfPromoteToOwnerError::RealmNotFound)
)]
#[case::timestamp_out_of_ballpark(
    authenticated_cmds::latest::realm_self_promote_to_owner::Rep::TimestampOutOfBallpark {
        ballpark_client_early_offset: 300.,
        ballpark_client_late_offset: 320.,
        client_timestamp: "2000-01-02T00:00:00Z".parse().unwrap(),
        server_timestamp: "2000-01-02T00:00:00Z".parse().unwrap(),
    },
    |err| p_assert_matches!(
        err,
        CertifSelfPromoteToOwnerError::TimestampOutOfBallpark {
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            client_timestamp,
            server_timestamp,
        }
        if ballpark_client_early_offset == 300.
            && ballpark_client_late_offset == 320.
            && client_timestamp == "2000-01-02T00:00:00Z".parse().unwrap()
            && server_timestamp == "2000-01-02T00:00:00Z".parse().unwrap()
    )
)]
#[case::require_greater_timestamp(
    authenticated_cmds::latest::realm_self_promote_to_owner::Rep::RequireGreaterTimestamp {
        strictly_greater_than: "2000-01-02T00:00:00Z".parse().unwrap(),
    },
    |err| p_assert_matches!(err, CertifSelfPromoteToOwnerError::Offline(_))
)]
#[case::invalid_certificate(
    authenticated_cmds::latest::realm_self_promote_to_owner::Rep::InvalidCertificate,
    |err| p_assert_matches!(err, CertifSelfPromoteToOwnerError::Internal(_))
)]
#[case::unknown_status(
    authenticated_cmds::latest::realm_self_promote_to_owner::Rep::UnknownStatus {
        unknown_status: "".into(),
        reason: None,
    },
    |err| p_assert_matches!(err, CertifSelfPromoteToOwnerError::Internal(_))
)]
async fn server_error(
    #[case] rep: authenticated_cmds::latest::realm_self_promote_to_owner::Rep,
    #[case] assert: impl FnOnce(CertifSelfPromoteToOwnerError),
    env: &TestbedEnv,
) {
    let realm_id = env
        .customize(|builder| {
            builder.new_user("bob");
            let realm_id = builder
                .new_realm("bob")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.share_realm(realm_id, "alice", RealmRole::Manager);
            builder.revoke_user("bob");
            builder.certificates_storage_fetch_certificates("alice@dev1");
            realm_id
        })
        .await;

    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        move |_req: authenticated_cmds::latest::realm_self_promote_to_owner::Req| rep
    },);

    let err = ops.self_promote_to_owner(realm_id).await.unwrap_err();

    assert(err);
}

#[parsec_test(testbed = "minimal")]
async fn offline(env: &TestbedEnv) {
    let realm_id = env
        .customize(|builder| {
            builder.new_user("bob");
            let realm_id = builder
                .new_realm("bob")
                .then_do_initial_key_rotation()
                .map(|event| event.realm);
            builder.share_realm(realm_id, "alice", RealmRole::Manager);
            builder.revoke_user("bob");
            builder.certificates_storage_fetch_certificates("alice@dev1");
            realm_id
        })
        .await;

    let alice = env.local_device("alice@dev1");
    let ops = certificates_ops_factory(env, &alice).await;

    test_register_low_level_send_hook(&env.discriminant_dir, |_request_builder| async {
        Ok(ResponseMock::Mocked((
            StatusCode::IM_A_TEAPOT,
            HeaderMap::new(),
            Bytes::new(),
        )))
    });

    let err = ops.self_promote_to_owner(realm_id).await.unwrap_err();

    p_assert_matches!(
        err,
        CertifSelfPromoteToOwnerError::Offline(ConnectionError::InvalidResponseStatus(
            StatusCode::IM_A_TEAPOT
        ))
    );
}
