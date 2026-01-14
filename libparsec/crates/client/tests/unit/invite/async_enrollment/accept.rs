// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use super::super::utils::client_factory;
use libparsec_client_connection::{protocol, test_register_sequence_of_send_hooks};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::MockedAsyncEnrollmentIdentityStrategy;
use crate::{ClientAcceptAsyncEnrollmentError, SubmitAsyncEnrollmentIdentityStrategy};

async fn generate_submit_data(
    identity_strategy: &MockedAsyncEnrollmentIdentityStrategy,
) -> (
    AsyncEnrollmentID,
    DateTime,
    Bytes,
    protocol::authenticated_cmds::latest::async_enrollment_list::SubmitPayloadSignature,
) {
    let enrollment_id = AsyncEnrollmentID::default();
    let submitted_on: DateTime = "2010-01-01T00:00:00Z".parse().unwrap();
    let submit_payload = AsyncEnrollmentSubmitPayload {
        verify_key: SigningKey::generate().verify_key(),
        public_key: PrivateKey::generate().public_key(),
        requested_device_label: "PC1".parse().unwrap(),
        requested_human_handle: "Mike <mike@example.invalid>".parse().unwrap(),
    };

    let submit_payload: Bytes = submit_payload.dump().into();
    let submit_payload_signature = {
        use crate::SubmitAsyncEnrollmentIdentityStrategy;
        match identity_strategy.sign_submit_payload(submit_payload.clone()).await.unwrap() {
            protocol::anonymous_cmds::v5::async_enrollment_submit::SubmitPayloadSignature::OpenBao { signature, submitter_openbao_entity_id } => {
                protocol::authenticated_cmds::latest::async_enrollment_list::SubmitPayloadSignature::OpenBao { signature, submitter_openbao_entity_id }
            }
            _ => unreachable!(),
        }
    };

    (
        enrollment_id,
        submitted_on,
        submit_payload,
        submit_payload_signature,
    )
}

#[parsec_test(testbed = "minimal")]
async fn require_greater_timestamp_then_ok(env: &TestbedEnv) {
    let identity_strategy = Arc::new(MockedAsyncEnrollmentIdentityStrategy::default());
    let (enrollment_id, submitted_on, submit_payload, submit_payload_signature) =
        generate_submit_data(&identity_strategy).await;

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    let strictly_greater_than = DateTime::now() + Duration::seconds(10);

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: protocol::authenticated_cmds::latest::async_enrollment_list::Req| {
            protocol::authenticated_cmds::latest::async_enrollment_list::Rep::Ok {
                enrollments: vec![
                    protocol::authenticated_cmds::latest::async_enrollment_list::Enrollment {
                        enrollment_id,
                        submit_payload,
                        submit_payload_signature,
                        submitted_on,
                    },
                ],
            }
        },
        // Consider initial accept content to be too old
        move |req: protocol::authenticated_cmds::latest::async_enrollment_accept::Req| {
            p_assert_eq!(req.enrollment_id, enrollment_id);
            protocol::authenticated_cmds::latest::async_enrollment_accept::Rep::RequireGreaterTimestamp { strictly_greater_than }
        },
        // Next accept attempt should be new enough
        {
            let identity_strategy = identity_strategy.clone();
            move |req: protocol::authenticated_cmds::latest::async_enrollment_accept::Req| {
                p_assert_eq!(req.enrollment_id, enrollment_id);
                // Only do superficial checks here since the full test already does this
                let accept_payload =
                    AsyncEnrollmentAcceptPayload::load(&req.accept_payload).unwrap();
                p_assert_eq!(accept_payload.profile, UserProfile::Outsider);
                p_assert_eq!(
                    accept_payload.human_handle,
                    *identity_strategy.human_handle()
                );
                p_assert_eq!(accept_payload.root_verify_key, *alice.root_verify_key());
                let user_certif =
                    UserCertificate::unsecure_load(req.submitter_user_certificate).unwrap();
                assert!(*user_certif.timestamp() >= strictly_greater_than);
                protocol::authenticated_cmds::latest::async_enrollment_accept::Rep::Ok
            }
        },
    );

    p_assert_matches!(
        client
            .accept_async_enrollment(
                UserProfile::Outsider,
                enrollment_id,
                identity_strategy.as_ref(),
            )
            .await,
        Ok(())
    );
}

#[parsec_test(testbed = "minimal")]
async fn offline(env: &TestbedEnv) {
    let enrollment_id = AsyncEnrollmentID::default();
    let identity_strategy = MockedAsyncEnrollmentIdentityStrategy::default();

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice).await;

    p_assert_matches!(
        client
            .accept_async_enrollment(UserProfile::Outsider, enrollment_id, &identity_strategy,)
            .await,
        Err(ClientAcceptAsyncEnrollmentError::Offline(_))
    )
}

#[parsec_test(testbed = "minimal")]
async fn bad_submit_payload(env: &TestbedEnv) {
    let identity_strategy = MockedAsyncEnrollmentIdentityStrategy::default();
    let (enrollment_id, submitted_on, _submit_payload, submit_payload_signature) =
        generate_submit_data(&identity_strategy).await;

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: protocol::authenticated_cmds::latest::async_enrollment_list::Req| {
            protocol::authenticated_cmds::latest::async_enrollment_list::Rep::Ok {
                enrollments: vec![
                    protocol::authenticated_cmds::latest::async_enrollment_list::Enrollment {
                        enrollment_id,
                        submit_payload: Bytes::from_static(b"<dummy>"),
                        submit_payload_signature,
                        submitted_on,
                    },
                ],
            }
        },
    );

    p_assert_matches!(
        client
            .accept_async_enrollment(UserProfile::Outsider, enrollment_id, &identity_strategy)
            .await,
        Err(err @ ClientAcceptAsyncEnrollmentError::BadSubmitPayload(_))
        if err.to_string() == "Submitter has provided an invalid request payload: Invalid serialization: format <unknown> step <format detection>"
    );
}

#[parsec_test(testbed = "minimal")]
async fn identity_strategy_mismatch(env: &TestbedEnv) {
    let identity_strategy = MockedAsyncEnrollmentIdentityStrategy::default();
    let (enrollment_id, submitted_on, submit_payload, _submit_payload_signature) =
        generate_submit_data(&identity_strategy).await;

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: protocol::authenticated_cmds::latest::async_enrollment_list::Req| {
            protocol::authenticated_cmds::latest::async_enrollment_list::Rep::Ok {
                enrollments: vec![
                    protocol::authenticated_cmds::latest::async_enrollment_list::Enrollment {
                        enrollment_id,
                        submit_payload,
                        submit_payload_signature: protocol::authenticated_cmds::latest::async_enrollment_list::SubmitPayloadSignature::PKI {
                            algorithm: PkiSignatureAlgorithm::RsassaPssSha256,
                            intermediate_der_x509_certificates: vec![],
                            signature: b"<signature>".as_ref().into(),
                            submitter_der_x509_certificate: b"<submitter_der_x509_certificate>".as_ref().into(),
                        },
                        submitted_on,
                    },
                ],
            }
        },
    );

    p_assert_matches!(
        client
            .accept_async_enrollment(UserProfile::Outsider, enrollment_id, &identity_strategy)
            .await,
        Err(err @ ClientAcceptAsyncEnrollmentError::IdentityStrategyMismatch{ .. })
        if err.to_string() == "Submit payload is signed with a different identity system (PKI) than ours (OpenBao)"
    );
}

async fn test_server_error(
    env: &TestbedEnv,
    server_rep: protocol::authenticated_cmds::latest::async_enrollment_accept::Rep,
) -> ClientAcceptAsyncEnrollmentError {
    let identity_strategy = MockedAsyncEnrollmentIdentityStrategy::default();
    let (enrollment_id, submitted_on, submit_payload, submit_payload_signature) =
        generate_submit_data(&identity_strategy).await;

    let alice = env.local_device("alice@dev1");
    let client = client_factory(&env.discriminant_dir, alice.clone()).await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: protocol::authenticated_cmds::latest::async_enrollment_list::Req| {
            protocol::authenticated_cmds::latest::async_enrollment_list::Rep::Ok {
                enrollments: vec![
                    protocol::authenticated_cmds::latest::async_enrollment_list::Enrollment {
                        enrollment_id,
                        submit_payload,
                        submit_payload_signature,
                        submitted_on,
                    },
                ],
            }
        },
        move |req: protocol::authenticated_cmds::latest::async_enrollment_accept::Req| {
            p_assert_eq!(req.enrollment_id, enrollment_id);
            server_rep
        },
    );

    client
        .accept_async_enrollment(UserProfile::Outsider, enrollment_id, &identity_strategy)
        .await
        .unwrap_err()
}

#[parsec_test(testbed = "minimal")]
async fn author_not_allowed(env: &TestbedEnv) {
    p_assert_matches!(
        test_server_error(
            env,
            protocol::authenticated_cmds::latest::async_enrollment_accept::Rep::AuthorNotAllowed
        )
        .await,
        ClientAcceptAsyncEnrollmentError::AuthorNotAllowed
    );
}

#[parsec_test(testbed = "minimal")]
async fn enrollment_not_found(env: &TestbedEnv) {
    p_assert_matches!(
        test_server_error(
            env,
            protocol::authenticated_cmds::latest::async_enrollment_accept::Rep::EnrollmentNotFound
        )
        .await,
        ClientAcceptAsyncEnrollmentError::EnrollmentNotFound
    );
}

#[parsec_test(testbed = "minimal")]
async fn enrollment_no_longer_available(env: &TestbedEnv) {
    p_assert_matches!(
        test_server_error(env, protocol::authenticated_cmds::latest::async_enrollment_accept::Rep::EnrollmentNoLongerAvailable).await,
        ClientAcceptAsyncEnrollmentError::EnrollmentNoLongerAvailable
    );
}

#[parsec_test(testbed = "minimal")]
async fn active_users_limit_reached(env: &TestbedEnv) {
    p_assert_matches!(
        test_server_error(env, protocol::authenticated_cmds::latest::async_enrollment_accept::Rep::ActiveUsersLimitReached).await,
        ClientAcceptAsyncEnrollmentError::ActiveUsersLimitReached
    );
}

#[parsec_test(testbed = "minimal")]
async fn human_handle_already_taken(env: &TestbedEnv) {
    p_assert_matches!(
        test_server_error(env, protocol::authenticated_cmds::latest::async_enrollment_accept::Rep::HumanHandleAlreadyTaken).await,
        ClientAcceptAsyncEnrollmentError::HumanHandleAlreadyTaken
    );
}

#[parsec_test(testbed = "minimal")]
async fn timestamp_out_of_ballpark(env: &TestbedEnv) {
    p_assert_matches!(
        test_server_error(env, protocol::authenticated_cmds::latest::async_enrollment_accept::Rep::TimestampOutOfBallpark {
            ballpark_client_early_offset: 300.,
            ballpark_client_late_offset: 320.,
            client_timestamp: "2000-01-02T00:00:00Z".parse().unwrap(),
            server_timestamp: "2000-01-02T00:00:00Z".parse().unwrap(),
        }).await,
        ClientAcceptAsyncEnrollmentError::TimestampOutOfBallpark { server_timestamp, client_timestamp, ballpark_client_early_offset, ballpark_client_late_offset }
        if ballpark_client_early_offset == 300.
            && ballpark_client_late_offset == 320.
            && client_timestamp == "2000-01-02T00:00:00Z".parse().unwrap()
            && server_timestamp == "2000-01-02T00:00:00Z".parse().unwrap()
    );
}

#[parsec_test(testbed = "minimal")]
async fn invalid_x509_trustchain(env: &TestbedEnv) {
    p_assert_matches!(
        test_server_error(env, protocol::authenticated_cmds::latest::async_enrollment_accept::Rep::InvalidX509Trustchain).await,
        ClientAcceptAsyncEnrollmentError::PKIServerInvalidX509Trustchain
    );
}

#[parsec_test(testbed = "minimal")]
async fn user_already_exists(env: &TestbedEnv) {
    p_assert_matches!(
        test_server_error(env, protocol::authenticated_cmds::latest::async_enrollment_accept::Rep::UserAlreadyExists).await,
        ClientAcceptAsyncEnrollmentError::Internal(err)
        if err.to_string() == "Unexpected server response: UserAlreadyExists"
    );
}

#[parsec_test(testbed = "minimal")]
async fn invalid_certificate(env: &TestbedEnv) {
    p_assert_matches!(
        test_server_error(env, protocol::authenticated_cmds::latest::async_enrollment_accept::Rep::InvalidCertificate).await,
        ClientAcceptAsyncEnrollmentError::Internal(err)
        if err.to_string() == "Unexpected server response: InvalidCertificate"
    );
}

#[parsec_test(testbed = "minimal")]
async fn invalid_accept_payload(env: &TestbedEnv) {
    p_assert_matches!(
        test_server_error(env, protocol::authenticated_cmds::latest::async_enrollment_accept::Rep::InvalidAcceptPayload).await,
        ClientAcceptAsyncEnrollmentError::Internal(err)
        if err.to_string() == "Unexpected server response: InvalidAcceptPayload"
    );
}

#[parsec_test(testbed = "minimal")]
async fn invalid_accept_payload_signature(env: &TestbedEnv) {
    p_assert_matches!(
        test_server_error(env, protocol::authenticated_cmds::latest::async_enrollment_accept::Rep::InvalidAcceptPayloadSignature).await,
        ClientAcceptAsyncEnrollmentError::Internal(err)
        if err.to_string() == "Unexpected server response: InvalidAcceptPayloadSignature"
    );
}

#[parsec_test(testbed = "minimal")]
async fn submit_and_accept_identity_systems_mismatch(env: &TestbedEnv) {
    p_assert_matches!(
        test_server_error(env, protocol::authenticated_cmds::latest::async_enrollment_accept::Rep::SubmitAndAcceptIdentitySystemsMismatch).await,
        ClientAcceptAsyncEnrollmentError::Internal(err)
        if err.to_string() == "Unexpected server response: SubmitAndAcceptIdentitySystemsMismatch"
    );
}

#[parsec_test(testbed = "minimal")]
async fn invalid_der_x509_certificate(env: &TestbedEnv) {
    p_assert_matches!(
        test_server_error(env, protocol::authenticated_cmds::latest::async_enrollment_accept::Rep::InvalidDerX509Certificate).await,
        ClientAcceptAsyncEnrollmentError::Internal(err)
        if err.to_string() == "Unexpected server response: InvalidDerX509Certificate"
    );
}
