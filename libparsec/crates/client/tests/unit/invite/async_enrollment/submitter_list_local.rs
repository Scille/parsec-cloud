// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{
    submitter_list_local_async_enrollments, AvailablePendingAsyncEnrollment,
    AvailablePendingAsyncEnrollmentIdentitySystem,
};

#[parsec_test(testbed = "empty")]
async fn ok(env: &TestbedEnv) {
    p_assert_eq!(
        submitter_list_local_async_enrollments(&env.discriminant_dir)
            .await
            .unwrap(),
        vec![]
    );

    let enrollment_id = AsyncEnrollmentID::default();
    let openbao_ciphertext_key_path =
        "65732d02-bb5f-7ce7-eae4-69067383b61d/e89eb9b36b704ff292db320b553fcd32".to_string();
    let openbao_entity_id = "65732d02-bb5f-7ce7-eae4-69067383b61d".to_string();
    let openbao_preferred_auth_id = "auth/my_sso".to_string();
    let cleartext_content = AsyncEnrollmentLocalPendingCleartextContent {
        server_url: env.server_addr.clone(),
        organization_id: env.organization_id.clone(),
        submitted_on: "2010-01-01T00:00:00Z".parse().unwrap(),
        enrollment_id,
        requested_device_label: "PC1".parse().unwrap(),
        requested_human_handle: "Mike <mike@example.invalid>".parse().unwrap(),
        identity_system: AsyncEnrollmentLocalPendingIdentitySystem::OpenBao {
            openbao_ciphertext_key_path: openbao_ciphertext_key_path.clone(),
            openbao_entity_id: openbao_entity_id.clone(),
            openbao_preferred_auth_id: openbao_preferred_auth_id.clone(),
        },
    };
    let file_path = env.discriminant_dir.join("my.key");
    libparsec_platform_device_loader::save_pending_async_enrollment(
        &env.discriminant_dir,
        cleartext_content.clone(),
        &SigningKey::generate(),
        &PrivateKey::generate(),
        &SecretKey::generate(),
        file_path.clone(),
    )
    .await
    .unwrap();

    p_assert_eq!(
        submitter_list_local_async_enrollments(&env.discriminant_dir)
            .await
            .unwrap(),
        vec![AvailablePendingAsyncEnrollment {
            file_path,
            submitted_on: cleartext_content.submitted_on,
            addr: ParsecAsyncEnrollmentAddr::new(
                cleartext_content.server_url,
                cleartext_content.organization_id
            ),
            enrollment_id: cleartext_content.enrollment_id,
            requested_device_label: cleartext_content.requested_device_label,
            requested_human_handle: cleartext_content.requested_human_handle,
            identity_system: AvailablePendingAsyncEnrollmentIdentitySystem::OpenBao {
                openbao_entity_id,
                openbao_preferred_auth_id,
            },
        }]
    );
}
