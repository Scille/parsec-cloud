// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::{Arc, Mutex};

use super::super::utils::make_config;
use libparsec_client_connection::{protocol, test_register_send_hook};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{
    submit_async_enrollment, submitter_list_local_async_enrollments, SubmitAsyncEnrollmentError,
    SubmitAsyncEnrollmentIdentityStrategy,
};

use super::utils::MockedAsyncEnrollmentIdentityStrategy;

#[parsec_test(testbed = "empty")]
async fn offline(env: &TestbedEnv) {
    let config = make_config(env);
    let async_enrollment_addr =
        ParsecAsyncEnrollmentAddr::new(env.server_addr.clone(), env.organization_id.clone());

    p_assert_matches!(
        submit_async_enrollment(
            config,
            async_enrollment_addr,
            false,
            "PC".parse().unwrap(),
            &MockedAsyncEnrollmentIdentityStrategy::default(),
        )
        .await,
        Err(SubmitAsyncEnrollmentError::Offline(_))
    );
}

#[parsec_test(testbed = "empty")]
async fn ok(env: &TestbedEnv) {
    let config = make_config(env);
    let async_enrollment_addr =
        ParsecAsyncEnrollmentAddr::new(env.server_addr.clone(), env.organization_id.clone());

    let identity_strategy = MockedAsyncEnrollmentIdentityStrategy::default();
    let expected_human_handle = identity_strategy.human_handle().clone();
    let expected_submitted_on: DateTime = "2010-01-01T00:00:00Z".parse().unwrap();
    let expected_device_label: DeviceLabel = "PC".parse().unwrap();

    // 1) Submit enrollment

    let got_enrollment_id = Arc::new(Mutex::new(None));
    test_register_send_hook(&env.discriminant_dir, {
        let got_enrollment_id = got_enrollment_id.clone();
        let expected_device_label = expected_device_label.clone();
        move |req: protocol::anonymous_cmds::latest::async_enrollment_submit::Req| {
            *got_enrollment_id.lock().expect("Mutex is poisoned") = Some(req.enrollment_id);
            p_assert_eq!(req.force, false);
            let payload = AsyncEnrollmentSubmitPayload::load(&req.submit_payload).unwrap();
            p_assert_eq!(payload.requested_device_label, expected_device_label);
            p_assert_eq!(payload.requested_human_handle, expected_human_handle);
            protocol::anonymous_cmds::latest::async_enrollment_submit::Rep::Ok {
                submitted_on: expected_submitted_on,
            }
        }
    });

    let available = submit_async_enrollment(
        config.clone(),
        async_enrollment_addr,
        false,
        expected_device_label,
        &identity_strategy,
    )
    .await
    .unwrap();

    p_assert_eq!(
        available.enrollment_id,
        got_enrollment_id
            .lock()
            .expect("Mutex is poisoned")
            .unwrap()
    );

    // 2) Submitter enrollment is present in local

    p_assert_eq!(
        submitter_list_local_async_enrollments(&config.config_dir)
            .await
            .unwrap(),
        [available]
    );
}

async fn test_server_error(
    env: &TestbedEnv,
    server_rep: protocol::anonymous_cmds::latest::async_enrollment_submit::Rep,
) -> SubmitAsyncEnrollmentError {
    let config = make_config(env);
    let async_enrollment_addr =
        ParsecAsyncEnrollmentAddr::new(env.server_addr.clone(), env.organization_id.clone());

    let identity_strategy = MockedAsyncEnrollmentIdentityStrategy::default();
    let expected_device_label: DeviceLabel = "PC".parse().unwrap();

    test_register_send_hook(
        &env.discriminant_dir,
        move |_req: protocol::anonymous_cmds::latest::async_enrollment_submit::Req| server_rep,
    );

    submit_async_enrollment(
        config.clone(),
        async_enrollment_addr,
        false,
        expected_device_label,
        &identity_strategy,
    )
    .await
    .unwrap_err()
}

#[parsec_test(testbed = "minimal")]
async fn email_already_submitted(env: &TestbedEnv) {
    let expected_submitted_on = "2010-01-01T00:00:00Z".parse().unwrap();
    p_assert_matches!(
        test_server_error(env, protocol::anonymous_cmds::latest::async_enrollment_submit::Rep::EmailAlreadySubmitted { submitted_on: expected_submitted_on }).await,
        SubmitAsyncEnrollmentError::EmailAlreadySubmitted { submitted_on }
        if submitted_on == expected_submitted_on
    );
}

#[parsec_test(testbed = "minimal")]
async fn email_already_enrolled(env: &TestbedEnv) {
    p_assert_matches!(
        test_server_error(
            env,
            protocol::anonymous_cmds::latest::async_enrollment_submit::Rep::EmailAlreadyEnrolled
        )
        .await,
        SubmitAsyncEnrollmentError::EmailAlreadyEnrolled
    );
}

#[parsec_test(testbed = "minimal")]
async fn invalid_x509_trustchain(env: &TestbedEnv) {
    p_assert_matches!(
        test_server_error(
            env,
            protocol::anonymous_cmds::latest::async_enrollment_submit::Rep::InvalidX509Trustchain
        )
        .await,
        SubmitAsyncEnrollmentError::InvalidX509Trustchain
    );
}

#[parsec_test(testbed = "minimal")]
async fn id_already_used(env: &TestbedEnv) {
    p_assert_matches!(
       test_server_error(env, protocol::anonymous_cmds::latest::async_enrollment_submit::Rep::IdAlreadyUsed).await,
       SubmitAsyncEnrollmentError::Internal(err)
       if err.to_string() == "Unexpected server response: IdAlreadyUsed"
    );
}

#[parsec_test(testbed = "minimal")]
async fn invalid_submit_payload(env: &TestbedEnv) {
    p_assert_matches!(
        test_server_error(env, protocol::anonymous_cmds::latest::async_enrollment_submit::Rep::InvalidSubmitPayload).await,
        SubmitAsyncEnrollmentError::Internal(err)
        if err.to_string() == "Unexpected server response: InvalidSubmitPayload"
    );
}

#[parsec_test(testbed = "minimal")]
async fn invalid_submit_payload_signature(env: &TestbedEnv) {
    p_assert_matches!(
        test_server_error(env, protocol::anonymous_cmds::latest::async_enrollment_submit::Rep::InvalidSubmitPayloadSignature).await,
        SubmitAsyncEnrollmentError::Internal(err)
        if err.to_string() == "Unexpected server response: InvalidSubmitPayloadSignature"
    );
}

#[parsec_test(testbed = "minimal")]
async fn invalid_der_x509_certificate(env: &TestbedEnv) {
    p_assert_matches!(
        test_server_error(env, protocol::anonymous_cmds::latest::async_enrollment_submit::Rep::InvalidDerX509Certificate).await,
        SubmitAsyncEnrollmentError::Internal(err)
        if err.to_string() == "Unexpected server response: InvalidDerX509Certificate"
    );
}
