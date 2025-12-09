// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::{Arc, Mutex};

use super::super::utils::make_config;
use libparsec_client_connection::{protocol, test_register_send_hook};
use libparsec_platform_async::PinBoxFutureResult;
use libparsec_platform_device_loader::AvailablePendingAsyncEnrollment;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{
    submit_async_enrollment, submitter_list_local_async_enrollments, SubmitAsyncEnrollmentError,
    SubmitAsyncEnrollmentIdentityStrategy,
};

struct MockedSubmitAsyncEnrollmentIdentityStrategy {
    human_handle: HumanHandle,
    sign_outcome: Mutex<
        Option<protocol::anonymous_cmds::latest::async_enrollment_submit::SubmitPayloadSignature>,
    >,
    generate_ciphertext_key_outcome: Mutex<
        Option<
            Result<
                (SecretKey, AsyncEnrollmentLocalPendingIdentitySystem),
                SubmitAsyncEnrollmentError,
            >,
        >,
    >,
}

impl Default for MockedSubmitAsyncEnrollmentIdentityStrategy {
    fn default() -> Self {
        Self {
            human_handle: "John Doe <john.doe@example.com>".parse().unwrap(),
            sign_outcome: Mutex::new(Some(protocol::anonymous_cmds::latest::async_enrollment_submit::SubmitPayloadSignature::OpenBao {
                signature: "vault:v1:C4jRZx+ymLou26tN8Q2KDy46dA47W7x/MH6nuEZVqdz+H0RvoaFbQUAHceBKhB+Qow2qXAXiRFAFWKGPZU93CQ==".to_string(),
                submitter_openbao_entity_alias_id: "e7d07e2a-b05d-ec59-dcf2-4403efdbf1b7".to_string(),
            })),
            generate_ciphertext_key_outcome: Mutex::new(Some(Ok((
                SecretKey::generate(),
                AsyncEnrollmentLocalPendingIdentitySystem::OpenBao {
                    openbao_ciphertext_key_path: "65732d02-bb5f-7ce7-eae4-69067383b61d/e89eb9b36b704ff292db320b553fcd32".to_string(),
                    openbao_entity_id: "98ad4942-b164-467c-9839-f5be64cdb22c".to_string(),
                    openbao_preferred_auth_id: "HEXAGONE".to_string(),
                }
            )))),
        }
    }
}

impl SubmitAsyncEnrollmentIdentityStrategy for MockedSubmitAsyncEnrollmentIdentityStrategy {
    fn sign(
        &self,
        _payload: &[u8],
    ) -> protocol::anonymous_cmds::latest::async_enrollment_submit::SubmitPayloadSignature {
        self.sign_outcome
            .lock()
            .expect("Mutex is poisoned")
            .take()
            .expect("Already called")
    }

    fn human_handle(&self) -> &HumanHandle {
        &self.human_handle
    }

    fn generate_ciphertext_key(
        &self,
    ) -> PinBoxFutureResult<
        (SecretKey, AsyncEnrollmentLocalPendingIdentitySystem),
        SubmitAsyncEnrollmentError,
    > {
        let outcome = self
            .generate_ciphertext_key_outcome
            .lock()
            .expect("Mutex is poisoned")
            .take()
            .expect("Already called");
        Box::pin(async move { outcome })
    }
}

#[parsec_test(testbed = "empty")]
async fn offline(env: &TestbedEnv) {
    let config = make_config(env);
    let async_enrollment_addr =
        ParsecAsyncEnrollmentAddr::new(env.server_addr.clone(), env.organization_id.clone());

    p_assert_matches!(
        submit_async_enrollment(
            config,
            async_enrollment_addr.into(),
            false,
            "PC".parse().unwrap(),
            &MockedSubmitAsyncEnrollmentIdentityStrategy::default(),
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

    let identity_strategy = MockedSubmitAsyncEnrollmentIdentityStrategy::default();
    let expected_human_handle = identity_strategy.human_handle.clone();
    let expected_submitted_on: DateTime = "2010-01-01T00:00:00Z".parse().unwrap();
    let expected_device_label: DeviceLabel = "PC".parse().unwrap();

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
        async_enrollment_addr.into(),
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

    p_assert_eq!(
        submitter_list_local_async_enrollments(&config.config_dir)
            .await
            .unwrap(),
        [available,]
    );
}
