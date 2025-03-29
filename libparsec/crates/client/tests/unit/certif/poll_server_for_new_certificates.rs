// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashMap, sync::Arc};

use libparsec_client_connection::{
    test_register_low_level_send_hook, test_register_send_hook, ConnectionError, HeaderMap,
    ResponseMock, StatusCode,
};
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certif::CertifPollServerError;

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn empty(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::certificate_get::Req| {
            p_assert_eq!(req.common_after, None);
            assert!(req.realm_after.is_empty());
            p_assert_eq!(req.sequester_after, None);
            p_assert_eq!(req.shamir_recovery_after, None);

            authenticated_cmds::latest::certificate_get::Rep::Ok {
                common_certificates: vec![],
                sequester_certificates: vec![],
                shamir_recovery_certificates: vec![],
                realm_certificates: HashMap::default(),
            }
        },
    );

    p_assert_matches!(
        ops.poll_server_for_new_certificates(None).await,
        Ok(new_certificates) if new_certificates == 0
    );
}

async fn poll_testbed_certificates(
    env: &TestbedEnv,
    device: Arc<LocalDevice>,
    expected_new_certificates: usize,
) {
    let ops = certificates_ops_factory(env, &device).await;

    ops.forget_all_certificates().await.unwrap();

    test_register_send_hook(&env.discriminant_dir, {
        let rep = authenticated_cmds::latest::certificate_get::Rep::Ok {
            common_certificates: env.get_common_certificates_signed(),
            sequester_certificates: env.get_sequester_certificates_signed(),
            shamir_recovery_certificates: env
                .get_shamir_recovery_certificates_signed_for_user_topic(device.user_id),
            realm_certificates: env.get_realms_certificates_signed(),
        };
        move |req: authenticated_cmds::latest::certificate_get::Req| {
            p_assert_eq!(req.common_after, None);
            assert!(req.realm_after.is_empty());
            p_assert_eq!(req.sequester_after, None);
            p_assert_eq!(req.shamir_recovery_after, None);
            rep
        }
    });

    p_assert_matches!(
        ops.poll_server_for_new_certificates(None).await,
        Ok(new_certificates) if new_certificates == expected_new_certificates
    );

    // Re-poll, but nothing more to get

    let expected_common_after = Some(env.get_last_common_certificate_timestamp());
    let expected_realm_after = env.get_last_realm_certificate_timestamp_for_all_realms();
    let expected_sequester_after = env.get_last_sequester_certificate_timestamp();
    let expected_shamir_recovery_after =
        env.get_last_shamir_recovery_certificate_timestamp(device.user_id);

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::certificate_get::Req| {
            p_assert_eq!(req.common_after, expected_common_after);
            p_assert_eq!(req.realm_after, expected_realm_after);
            p_assert_eq!(req.sequester_after, expected_sequester_after);
            p_assert_eq!(req.shamir_recovery_after, expected_shamir_recovery_after);

            authenticated_cmds::latest::certificate_get::Rep::Ok {
                common_certificates: vec![],
                sequester_certificates: vec![],
                shamir_recovery_certificates: vec![],
                realm_certificates: HashMap::default(),
            }
        },
    );

    p_assert_matches!(
        ops.poll_server_for_new_certificates(None).await,
        Ok(new_certificates) if new_certificates == 0
    );
}

#[parsec_test(testbed = "minimal")]
async fn minimal(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    poll_testbed_certificates(env, alice, 2).await;
}

#[parsec_test(testbed = "coolorg")]
async fn coolorg(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    poll_testbed_certificates(env, alice, 14).await;
}

#[parsec_test(testbed = "sequestered")]
async fn sequestered(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    poll_testbed_certificates(env, alice, 14).await;
}

#[parsec_test(testbed = "shamir")]
async fn shamir(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    poll_testbed_certificates(env, alice, 15).await;
}

#[parsec_test(testbed = "minimal")]
async fn invalid_certif_received(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        move |_: authenticated_cmds::latest::certificate_get::Req| {
            authenticated_cmds::latest::certificate_get::Rep::Ok {
                common_certificates: vec![Bytes::from_static(b"foo")],
                sequester_certificates: vec![],
                shamir_recovery_certificates: vec![],
                realm_certificates: HashMap::default(),
            }
        },
    );

    let err = ops
        .poll_server_for_new_certificates(None)
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifPollServerError::InvalidCertificate(..));
}

#[parsec_test(testbed = "minimal")]
async fn unknown_status(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        move |_: authenticated_cmds::latest::certificate_get::Req| {
            authenticated_cmds::latest::certificate_get::Rep::UnknownStatus {
                unknown_status: "".into(),
                reason: None,
            }
        },
    );

    let err = ops
        .poll_server_for_new_certificates(None)
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifPollServerError::Internal(..));
}

#[parsec_test(testbed = "minimal")]
async fn offline(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .poll_server_for_new_certificates(None)
        .await
        .unwrap_err();

    p_assert_matches!(err, CertifPollServerError::Offline(_));
}

#[parsec_test(testbed = "minimal")]
async fn invalid_response(env: &TestbedEnv) {
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
        .poll_server_for_new_certificates(None)
        .await
        .unwrap_err();

    p_assert_matches!(
        err,
        CertifPollServerError::Offline(ConnectionError::InvalidResponseStatus(
            StatusCode::IM_A_TEAPOT
        ))
    );
}
