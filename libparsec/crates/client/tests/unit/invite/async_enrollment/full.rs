// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use super::super::utils::{client_factory, make_config};
use libparsec_platform_device_loader::DeviceSaveStrategy;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{
    submit_async_enrollment, submitter_finalize_async_enrollment,
    submitter_get_async_enrollment_info, submitter_list_local_async_enrollments,
    AsyncEnrollmentIdentitySystem, AsyncEnrollmentUntrusted, ClientAcceptTosError,
    PendingAsyncEnrollmentInfo,
};

use super::utils::MockedAsyncEnrollmentIdentityStrategy;

#[parsec_test(testbed = "minimal", with_server)]
async fn with_server(env: &TestbedEnv) {
    let config = make_config(env);
    let async_enrollment_addr =
        ParsecAsyncEnrollmentAddr::new(env.server_addr.clone(), env.organization_id.clone());

    let expected_human_handle: HumanHandle = "Mike <mike@example.com>".parse().unwrap();
    let expected_device_label: DeviceLabel = "PC".parse().unwrap();
    let mock_identity_strategy =
        MockedAsyncEnrollmentIdentityStrategy::new(expected_human_handle.clone());

    // 1) Submit enrollment

    let available_enrollment = submit_async_enrollment(
        config.clone(),
        async_enrollment_addr.clone(),
        false,
        expected_device_label.clone(),
        &mock_identity_strategy,
    )
    .await
    .unwrap();
    let enrollment_id = available_enrollment.enrollment_id;
    let submitted_on = available_enrollment.submitted_on;
    let enrollment_file = available_enrollment.file_path.clone();

    // 2) Submitter enrollment is present in local and pending on the server

    p_assert_eq!(
        submitter_list_local_async_enrollments(&config.config_dir)
            .await
            .unwrap(),
        [available_enrollment]
    );

    p_assert_matches!(
        submitter_get_async_enrollment_info(config.clone(), async_enrollment_addr.clone(), enrollment_id).await.unwrap(),
        PendingAsyncEnrollmentInfo::Submitted { submitted_on: candidate_submitted_on }
        if candidate_submitted_on == submitted_on
    );

    // 3) Admin list pending enrollments

    let alice = env.local_device("alice@dev1");
    let alice_client = client_factory(&env.discriminant_dir, alice.clone()).await;

    p_assert_eq!(
        alice_client.list_async_enrollments().await.unwrap(),
        [AsyncEnrollmentUntrusted {
            enrollment_id,
            submitted_on,
            untrusted_requested_device_label: expected_device_label.clone(),
            untrusted_requested_human_handle: expected_human_handle.clone(),
            identity_system: AsyncEnrollmentIdentitySystem::OpenBao,
        }]
    );

    // 4) Admin accept enrollment

    alice_client
        .accept_async_enrollment(
            UserProfile::Outsider,
            enrollment_id,
            &mock_identity_strategy,
        )
        .await
        .unwrap();

    // 5) Submitter polls server is see the enrollment is now accepted

    p_assert_matches!(
        submitter_get_async_enrollment_info(config.clone(), async_enrollment_addr.clone(), enrollment_id).await.unwrap(),
        PendingAsyncEnrollmentInfo::Accepted { submitted_on: candidate_submitted_on, accepted_on: _ }
        if candidate_submitted_on == submitted_on
    );

    // 6) Submitter finalize the enrollment

    let mike_save_strategy = DeviceSaveStrategy::Password {
        password: "P@ssw0rd.".to_owned().into(),
    };
    let available_device = submitter_finalize_async_enrollment(
        config.clone(),
        &enrollment_file,
        &mike_save_strategy,
        &mock_identity_strategy,
    )
    .await
    .unwrap();
    p_assert_eq!(
        available_device.key_file_path,
        config
            .config_dir
            .join("devices")
            .join(format!("{}.keys", available_device.device_id.hex()))
    );
    p_assert_eq!(available_device.organization_id, env.organization_id);

    // 7) Ensure the obtained device can be used

    let mike_access_strategy =
        mike_save_strategy.into_access(available_device.key_file_path.clone());
    let mike =
        libparsec_platform_device_loader::load_device(&config.config_dir, &mike_access_strategy)
            .await
            .unwrap();
    p_assert_eq!(mike.human_handle, expected_human_handle);
    p_assert_eq!(mike.device_label, expected_device_label);
    p_assert_ne!(mike.user_id, alice.user_id);
    p_assert_ne!(mike.device_id, alice.device_id);

    let mike_client = client_factory(&env.discriminant_dir, mike).await;

    // Run a dummy command that request the server to ensure we can authenticate fine
    p_assert_matches!(
        mike_client
            .accept_tos("2000-01-01T00:00:00Z".parse().unwrap())
            .await,
        Err(ClientAcceptTosError::NoTos),
    );

    // 8) Finally make sure the pending async enrollment file has been destroyed

    p_assert_eq!(
        submitter_list_local_async_enrollments(&config.config_dir)
            .await
            .unwrap(),
        []
    );
}
