// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{protocol, test_register_send_hook};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::super::utils::make_config;
use crate::{submitter_cancel_async_enrollment, SubmitterCancelAsyncEnrollmentError};

#[parsec_test(testbed = "empty")]
async fn ok(env: &TestbedEnv) {
    let config = make_config(env);
    let async_enrollment_addr =
        ParsecAsyncEnrollmentAddr::new(env.server_addr.clone(), env.organization_id.clone());

    let enrollment_id = AsyncEnrollmentID::default();
    let file_path = libparsec_platform_device_loader::get_default_pending_async_enrollment_file(
        &env.discriminant_dir,
        enrollment_id,
    );
    libparsec_platform_device_loader::save_pending_async_enrollment(
        &env.discriminant_dir,
        AsyncEnrollmentLocalPendingCleartextContent {
            server_url: env.server_addr.clone(),
            organization_id: env.organization_id.clone(),
            submitted_on: "2010-01-01T00:00:00Z".parse().unwrap(),
            enrollment_id,
            requested_device_label: "PC1".parse().unwrap(),
            requested_human_handle: "Mike <mike@example.invalid>".parse().unwrap(),
            identity_system: AsyncEnrollmentLocalPendingIdentitySystem::OpenBao {
                openbao_ciphertext_key_path:
                    "65732d02-bb5f-7ce7-eae4-69067383b61d/e89eb9b36b704ff292db320b553fcd32"
                        .to_string(),
                openbao_entity_id: "65732d02-bb5f-7ce7-eae4-69067383b61d".to_string(),
                openbao_preferred_auth_id: "auth/my_sso".to_string(),
            },
        },
        &SigningKey::generate(),
        &PrivateKey::generate(),
        &SecretKey::generate(),
        file_path.clone(),
    )
    .await
    .unwrap();

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: protocol::anonymous_cmds::latest::async_enrollment_cancel::Req| {
            p_assert_eq!(req.enrollment_id, enrollment_id);
            protocol::anonymous_cmds::latest::async_enrollment_cancel::Rep::Ok
        },
    );

    p_assert_matches!(
        submitter_cancel_async_enrollment(
            config.clone(),
            async_enrollment_addr.clone(),
            enrollment_id
        )
        .await,
        Ok(())
    );

    // Verify that the local file has been removed
    assert!(!file_path.exists());

    // Trying again should fail with not found (local file is gone)
    test_register_send_hook(
        &env.discriminant_dir,
        move |req: protocol::anonymous_cmds::latest::async_enrollment_cancel::Req| {
            p_assert_eq!(req.enrollment_id, enrollment_id);
            protocol::anonymous_cmds::latest::async_enrollment_cancel::Rep::EnrollmentNotFound
        },
    );
    p_assert_matches!(
        submitter_cancel_async_enrollment(config, async_enrollment_addr, enrollment_id).await,
        Err(SubmitterCancelAsyncEnrollmentError::NotFound)
    );
}

#[parsec_test(testbed = "empty")]
async fn offline(env: &TestbedEnv) {
    let config = make_config(env);
    let async_enrollment_addr =
        ParsecAsyncEnrollmentAddr::new(env.server_addr.clone(), env.organization_id.clone());

    p_assert_matches!(
        submitter_cancel_async_enrollment(
            config,
            async_enrollment_addr,
            AsyncEnrollmentID::default()
        )
        .await,
        Err(SubmitterCancelAsyncEnrollmentError::Offline(_)),
    );
}

#[parsec_test(testbed = "empty")]
async fn enrollment_not_found(#[values("server", "local")] kind: &str, env: &TestbedEnv) {
    let config = make_config(env);
    let async_enrollment_addr =
        ParsecAsyncEnrollmentAddr::new(env.server_addr.clone(), env.organization_id.clone());

    let enrollment_id = AsyncEnrollmentID::default();

    // `submitter_cancel_async_enrollment` first cancels on the server *then* remove locally
    let server_response = match kind {
        "server" => {
            protocol::anonymous_cmds::latest::async_enrollment_cancel::Rep::EnrollmentNotFound
        }

        "local" => protocol::anonymous_cmds::latest::async_enrollment_cancel::Rep::Ok,

        unknown => panic!("Unknown kind: {unknown}"),
    };

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: protocol::anonymous_cmds::latest::async_enrollment_cancel::Req| {
            p_assert_eq!(req.enrollment_id, enrollment_id);
            server_response
        },
    );

    p_assert_matches!(
        submitter_cancel_async_enrollment(config, async_enrollment_addr, enrollment_id).await,
        Err(SubmitterCancelAsyncEnrollmentError::NotFound),
    );
}

#[parsec_test(testbed = "empty")]
async fn enrollment_server_err_no_longer_available(env: &TestbedEnv) {
    let config = make_config(env);
    let async_enrollment_addr =
        ParsecAsyncEnrollmentAddr::new(env.server_addr.clone(), env.organization_id.clone());

    let enrollment_id = AsyncEnrollmentID::default();

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: protocol::anonymous_cmds::latest::async_enrollment_cancel::Req| {
            p_assert_eq!(req.enrollment_id, enrollment_id);
            protocol::anonymous_cmds::latest::async_enrollment_cancel::Rep::EnrollmentNoLongerAvailable
        },
    );

    p_assert_matches!(
        submitter_cancel_async_enrollment(config, async_enrollment_addr, enrollment_id).await,
        Err(SubmitterCancelAsyncEnrollmentError::NotFound),
    );
}

#[parsec_test(testbed = "empty")]
async fn enrollment_server_err_unknown(env: &TestbedEnv) {
    let config = make_config(env);
    let async_enrollment_addr =
        ParsecAsyncEnrollmentAddr::new(env.server_addr.clone(), env.organization_id.clone());

    let enrollment_id = AsyncEnrollmentID::default();

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: protocol::anonymous_cmds::latest::async_enrollment_cancel::Req| {
            p_assert_eq!(req.enrollment_id, enrollment_id);
            protocol::anonymous_cmds::latest::async_enrollment_cancel::Rep::UnknownStatus {
                unknown_status: "D'oh!".to_string(),
                reason: None,
            }
        },
    );

    p_assert_matches!(
        submitter_cancel_async_enrollment(config, async_enrollment_addr, enrollment_id).await,
        Err(SubmitterCancelAsyncEnrollmentError::Internal(err))
        if format!("{}", err) == "Unexpected server response: UnknownStatus { unknown_status: \"D'oh!\", reason: None }"
    );
}
