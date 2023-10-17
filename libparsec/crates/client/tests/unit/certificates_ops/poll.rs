// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{
    test_register_low_level_send_hook, test_register_send_hook, HeaderMap, ResponseMock, StatusCode,
};
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::certificates_ops::PollServerError;

use super::utils::certificates_ops_factory;

#[parsec_test(testbed = "minimal")]
async fn empty(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::certificate_get::Req| {
            p_assert_eq!(req.offset, 0);

            authenticated_cmds::latest::certificate_get::Rep::Ok {
                certificates: vec![],
            }
        },
    );

    let index = ops.poll_server_for_new_certificates(None).await.unwrap();

    p_assert_eq!(index, 0);

    ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn user_certificate(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    let (_, alice_signed) = env.get_user_certificate("alice");

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::certificate_get::Req| {
            p_assert_eq!(req.offset, 0);

            authenticated_cmds::latest::certificate_get::Rep::Ok {
                certificates: vec![alice_signed],
            }
        },
    );

    let index = ops.poll_server_for_new_certificates(None).await.unwrap();

    p_assert_eq!(index, 1);

    ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn certificates(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    let (_, alice_signed) = env.get_user_certificate("alice");
    let (_, alice_dev1_signed) = env.get_device_certificate("alice@dev1");

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::certificate_get::Req| {
            p_assert_eq!(req.offset, 0);

            authenticated_cmds::latest::certificate_get::Rep::Ok {
                certificates: vec![alice_signed, alice_dev1_signed],
            }
        },
    );

    let index = ops.poll_server_for_new_certificates(None).await.unwrap();

    p_assert_eq!(index, 2);

    ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn minimal(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;
    let certificates = env
        .template
        .certificates()
        .map(|e| e.signed)
        .collect::<Vec<_>>();
    let expected_index = certificates.len() as IndexInt;

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::certificate_get::Req| {
            p_assert_eq!(req.offset, 0);

            authenticated_cmds::latest::certificate_get::Rep::Ok { certificates }
        },
    );

    let index = ops.poll_server_for_new_certificates(None).await.unwrap();

    p_assert_eq!(index, expected_index);

    ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn nothing_to_poll(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;
    let certificates = env
        .template
        .certificates()
        .map(|e| e.signed)
        .collect::<Vec<_>>();
    let next_offset = env.get_last_certificate_index();

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::certificate_get::Req| {
            p_assert_eq!(req.offset, next_offset);

            authenticated_cmds::latest::certificate_get::Rep::Ok {
                certificates: vec![],
            }
        },
    );

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::certificate_get::Req| {
            p_assert_eq!(req.offset, 0);

            authenticated_cmds::latest::certificate_get::Rep::Ok { certificates }
        },
    );

    // Poll all
    let last_index = ops.poll_server_for_new_certificates(None).await.unwrap();

    // Nothing to poll
    let index = ops
        .poll_server_for_new_certificates(Some(last_index))
        .await
        .unwrap();

    p_assert_eq!(index, last_index);

    ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn invalid_certif_received(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    test_register_send_hook(
        &env.discriminant_dir,
        move |_: authenticated_cmds::latest::certificate_get::Req| {
            authenticated_cmds::latest::certificate_get::Rep::Ok {
                certificates: vec![Bytes::from_static(b"foo")],
            }
        },
    );

    let err = ops
        .poll_server_for_new_certificates(None)
        .await
        .unwrap_err();

    p_assert_matches!(err, PollServerError::InvalidCertificate(..));

    ops.stop().await;
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

    p_assert_matches!(err, PollServerError::Internal(..));

    ops.stop().await;
}

#[parsec_test(testbed = "minimal")]
async fn offline(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");

    let ops = certificates_ops_factory(env, &alice).await;

    let err = ops
        .poll_server_for_new_certificates(None)
        .await
        .unwrap_err();

    p_assert_matches!(err, PollServerError::Offline);

    ops.stop().await;
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

    p_assert_matches!(err, PollServerError::Internal(..));

    ops.stop().await;
}
