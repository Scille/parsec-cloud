// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{submitter_forget_async_enrollment, SubmitterForgetAsyncEnrollmentError};

#[parsec_test(testbed = "empty")]
async fn not_found(env: &TestbedEnv) {
    p_assert_matches!(
        submitter_forget_async_enrollment(&env.discriminant_dir, AsyncEnrollmentID::default())
            .await,
        Err(SubmitterForgetAsyncEnrollmentError::NotFound)
    );
}

#[parsec_test(testbed = "empty")]
async fn ok(env: &TestbedEnv) {
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
        file_path,
    )
    .await
    .unwrap();

    p_assert_matches!(
        submitter_forget_async_enrollment(&env.discriminant_dir, enrollment_id).await,
        Ok(())
    );
    p_assert_matches!(
        submitter_forget_async_enrollment(&env.discriminant_dir, enrollment_id).await,
        Err(SubmitterForgetAsyncEnrollmentError::NotFound)
    );
}
