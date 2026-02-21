// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::path::PathBuf;

use libparsec_client_connection::{protocol, test_register_send_hook};
use libparsec_platform_device_loader::{AvailableDevice, AvailableDeviceType, DeviceSaveStrategy};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::super::utils::make_config;
use super::utils::MockedAsyncEnrollmentIdentityStrategy;
use crate::{submitter_finalize_async_enrollment, SubmitterFinalizeAsyncEnrollmentError};

async fn generate_submit(
    env: &TestbedEnv,
    identity_strategy: &MockedAsyncEnrollmentIdentityStrategy,
) -> (AsyncEnrollmentID, PathBuf) {
    let enrollment_id = AsyncEnrollmentID::default();
    let enrollment_file_path =
        libparsec_platform_device_loader::get_default_pending_async_enrollment_file(
            &env.discriminant_dir,
            enrollment_id,
        );

    let (ciphertext_key, identity_system) = {
        use crate::SubmitAsyncEnrollmentIdentityStrategy;
        let (ciphertext_key, raw_identity_system) =
            identity_strategy.generate_ciphertext_key().await.unwrap();
        let identity_system = match raw_identity_system {
            AsyncEnrollmentLocalPendingIdentitySystem::OpenBao {
                openbao_ciphertext_key_path,
                openbao_entity_id,
                openbao_preferred_auth_id,
            } => AsyncEnrollmentLocalPendingIdentitySystem::OpenBao {
                openbao_ciphertext_key_path,
                openbao_entity_id,
                openbao_preferred_auth_id,
            },
            _ => unreachable!(),
        };
        (ciphertext_key, identity_system)
    };

    let cleartext_content = AsyncEnrollmentLocalPendingCleartextContent {
        server_url: env.server_addr.clone(),
        organization_id: env.organization_id.clone(),
        submitted_on: "2010-01-01T00:00:00Z".parse().unwrap(),
        enrollment_id,
        requested_device_label: "PC1".parse().unwrap(),
        requested_human_handle: "Mike <mike@example.invalid>".parse().unwrap(),
        identity_system,
    };
    libparsec_platform_device_loader::save_pending_async_enrollment(
        &env.discriminant_dir,
        cleartext_content.clone(),
        &SigningKey::generate(),
        &PrivateKey::generate(),
        &ciphertext_key,
        enrollment_file_path.clone(),
    )
    .await
    .unwrap();

    (enrollment_id, enrollment_file_path)
}

#[parsec_test(testbed = "empty")]
async fn ok(env: &TestbedEnv) {
    let config = make_config(env);
    let identity_strategy = MockedAsyncEnrollmentIdentityStrategy::default();

    let (enrollment_id, enrollment_file_path) = generate_submit(env, &identity_strategy).await;

    let accept_payload = AsyncEnrollmentAcceptPayload {
        user_id: UserID::default(),
        device_id: DeviceID::default(),
        device_label: "PC".parse().unwrap(),
        human_handle: "Mike <mike@example.invalid>".parse().unwrap(),
        profile: UserProfile::Outsider,
        root_verify_key: SigningKey::generate().verify_key(),
    };
    let accept_payload_raw: Bytes = accept_payload.dump().into();
    let accept_payload_signature = {
        use crate::AcceptAsyncEnrollmentIdentityStrategy;
        match identity_strategy.sign_accept_payload(accept_payload_raw.clone()).await.unwrap() {
            protocol::authenticated_cmds::v5::async_enrollment_accept::AcceptPayloadSignature::OpenBao { accepter_openbao_entity_id, signature } =>
                protocol::anonymous_cmds::v5::async_enrollment_info::AcceptPayloadSignature::OpenBao { accepter_openbao_entity_id, signature },
            _ => unreachable!(),
        }
    };

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: protocol::anonymous_cmds::latest::async_enrollment_info::Req| {
            p_assert_eq!(req.enrollment_id, enrollment_id);
            protocol::anonymous_cmds::latest::async_enrollment_info::Rep::Ok(
                protocol::anonymous_cmds::latest::async_enrollment_info::InfoStatus::Accepted {
                    submitted_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                    accepted_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                    accept_payload: accept_payload_raw,
                    accept_payload_signature,
                },
            )
        },
    );

    let new_device_save_strategy = DeviceSaveStrategy::new_keyring();

    p_assert_matches!(submitter_finalize_async_enrollment(
            config.clone(),
            &enrollment_file_path,
            &new_device_save_strategy,
            &identity_strategy,
        )
        .await,
        Ok(AvailableDevice {
            key_file_path,
            created_on: _,
            protected_on: _,
            server_addr,
            organization_id,
            user_id,
            device_id,
            human_handle,
            device_label,
            totp_opaque_key_id: None,
            ty: AvailableDeviceType::Keyring,
        })
        if key_file_path == config.config_dir.join("devices").join(format!("{}.keys", accept_payload.device_id.hex()))
            && server_addr == env.server_addr
            && organization_id == env.organization_id
            && user_id == accept_payload.user_id
            && device_id == accept_payload.device_id
            && human_handle == accept_payload.human_handle
            && device_label == accept_payload.device_label
    );
}

#[parsec_test(testbed = "empty")]
async fn offline(env: &TestbedEnv) {
    let config = make_config(env);
    let identity_strategy = MockedAsyncEnrollmentIdentityStrategy::default();

    let (_, enrollment_file_path) = generate_submit(env, &identity_strategy).await;

    let new_device_save_strategy = DeviceSaveStrategy::new_keyring();

    p_assert_matches!(
        submitter_finalize_async_enrollment(
            config,
            &enrollment_file_path,
            &new_device_save_strategy,
            &identity_strategy,
        )
        .await,
        Err(SubmitterFinalizeAsyncEnrollmentError::Offline(_)),
    );
}

#[parsec_test(testbed = "empty")]
async fn server_info_rep_not_accepted(env: &TestbedEnv) {
    let config = make_config(env);
    let identity_strategy = MockedAsyncEnrollmentIdentityStrategy::default();

    let (enrollment_id, enrollment_file_path) = generate_submit(env, &identity_strategy).await;

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: protocol::anonymous_cmds::latest::async_enrollment_info::Req| {
            p_assert_eq!(req.enrollment_id, enrollment_id);
            protocol::anonymous_cmds::latest::async_enrollment_info::Rep::Ok(
                protocol::anonymous_cmds::latest::async_enrollment_info::InfoStatus::Submitted {
                    submitted_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                },
            )
        },
    );

    let new_device_save_strategy = DeviceSaveStrategy::new_keyring();

    p_assert_matches!(
        submitter_finalize_async_enrollment(
            config,
            &enrollment_file_path,
            &new_device_save_strategy,
            &identity_strategy,
        )
        .await,
        Err(SubmitterFinalizeAsyncEnrollmentError::NotAccepted),
    );
}

#[parsec_test(testbed = "empty")]
async fn bao_accept_payload(env: &TestbedEnv) {
    let config = make_config(env);
    let identity_strategy = MockedAsyncEnrollmentIdentityStrategy::default();

    let (enrollment_id, enrollment_file_path) = generate_submit(env, &identity_strategy).await;

    let accept_payload = AsyncEnrollmentAcceptPayload {
        user_id: UserID::default(),
        device_id: DeviceID::default(),
        device_label: "PC".parse().unwrap(),
        human_handle: "Mike <mike@example.invalid>".parse().unwrap(),
        profile: UserProfile::Outsider,
        root_verify_key: SigningKey::generate().verify_key(),
    }
    .dump()
    .into();

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: protocol::anonymous_cmds::latest::async_enrollment_info::Req| {
            p_assert_eq!(req.enrollment_id, enrollment_id);
            protocol::anonymous_cmds::latest::async_enrollment_info::Rep::Ok(
                protocol::anonymous_cmds::latest::async_enrollment_info::InfoStatus::Accepted {
                    submitted_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                    accepted_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                    accept_payload,
                    accept_payload_signature: protocol::anonymous_cmds::v5::async_enrollment_info::AcceptPayloadSignature::OpenBao {
                        // Invalid signature!
                        signature: "vault:v1:kqrnMiRBFelGqTq7J4bmlhkGun09HshMIfOeGVoA8WZEEHkBlqoWQV+rI/WlBItUjRhBKVVm2PIigshKA7Cb+Q==".to_string(),
                        accepter_openbao_entity_id: "81b533c6-e41a-4533-9d50-3188cb88edd8".to_string(),
                    },
                },
            )
        },
    );

    let new_device_save_strategy = DeviceSaveStrategy::new_keyring();

    p_assert_matches!(
        submitter_finalize_async_enrollment(
            config,
            &enrollment_file_path,
            &new_device_save_strategy,
            &identity_strategy,
        )
        .await,
        Err(err @ SubmitterFinalizeAsyncEnrollmentError::BadAcceptPayload(_))
        if err.to_string() == "Accepter has provided an invalid request payload: Invalid payload signature"
    );
}

#[parsec_test(testbed = "empty")]
async fn identity_strategy_mismatch(env: &TestbedEnv) {
    let config = make_config(env);
    let identity_strategy = MockedAsyncEnrollmentIdentityStrategy::default();

    let (enrollment_id, enrollment_file_path) = generate_submit(env, &identity_strategy).await;

    let accept_payload = AsyncEnrollmentAcceptPayload {
        user_id: UserID::default(),
        device_id: DeviceID::default(),
        device_label: "PC".parse().unwrap(),
        human_handle: "Mike <mike@example.invalid>".parse().unwrap(),
        profile: UserProfile::Outsider,
        root_verify_key: SigningKey::generate().verify_key(),
    }
    .dump()
    .into();

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: protocol::anonymous_cmds::latest::async_enrollment_info::Req| {
            p_assert_eq!(req.enrollment_id, enrollment_id);
            protocol::anonymous_cmds::latest::async_enrollment_info::Rep::Ok(
                protocol::anonymous_cmds::latest::async_enrollment_info::InfoStatus::Accepted {
                    submitted_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                    accepted_on: "2000-01-01T00:00:00Z".parse().unwrap(),
                    accept_payload,
                    accept_payload_signature: protocol::anonymous_cmds::v5::async_enrollment_info::AcceptPayloadSignature::PKI {
                        algorithm: PkiSignatureAlgorithm::RsassaPssSha256,
                        intermediate_der_x509_certificates: vec![],
                        signature: b"<signature>".as_ref().into(),
                        accepter_der_x509_certificate: b"<accepter_der_x509_certificate>".as_ref().into(),
                    },
                },
            )
        },
    );

    let new_device_save_strategy = DeviceSaveStrategy::new_keyring();

    p_assert_matches!(
        submitter_finalize_async_enrollment(
            config,
            &enrollment_file_path,
            &new_device_save_strategy,
            &identity_strategy,
        )
        .await,
        Err(err @ SubmitterFinalizeAsyncEnrollmentError::IdentityStrategyMismatch{ .. })
        if err.to_string() == "Accept payload is signed with a different identity system (PKI) than ours (OpenBao)"
    );
}

#[parsec_test(testbed = "empty")]
async fn enrollment_file_invalid_path(env: &TestbedEnv) {
    let config = make_config(env);
    let identity_strategy = MockedAsyncEnrollmentIdentityStrategy::default();

    let new_device_save_strategy = DeviceSaveStrategy::new_keyring();

    p_assert_matches!(
        submitter_finalize_async_enrollment(
            config,
            &env.discriminant_dir.join("dummy.file"),
            &new_device_save_strategy,
            &identity_strategy,
        )
        .await,
        Err(SubmitterFinalizeAsyncEnrollmentError::EnrollmentFileInvalidPath(_))
    );
}

#[parsec_test(testbed = "empty")]
async fn enrollment_file_cannot_retrieve_ciphertext_key(env: &TestbedEnv) {
    let config = make_config(env);
    let identity_strategy = MockedAsyncEnrollmentIdentityStrategy::default();

    let enrollment_id = AsyncEnrollmentID::default();
    let enrollment_file_path =
        libparsec_platform_device_loader::get_default_pending_async_enrollment_file(
            &env.discriminant_dir,
            enrollment_id,
        );

    let cleartext_content = AsyncEnrollmentLocalPendingCleartextContent {
        server_url: env.server_addr.clone(),
        organization_id: env.organization_id.clone(),
        submitted_on: "2010-01-01T00:00:00Z".parse().unwrap(),
        enrollment_id: AsyncEnrollmentID::default(),
        requested_device_label: "PC1".parse().unwrap(),
        requested_human_handle: "Mike <mike@example.invalid>".parse().unwrap(),
        identity_system: AsyncEnrollmentLocalPendingIdentitySystem::OpenBao {
            // `MockedAsyncEnrollmentIdentityStrategy` encodes the ciphertext key
            // as base64 in the key path, so here we set a dummy value so that
            // the identity system won't be able to retrieve the ciphertext key.
            openbao_ciphertext_key_path: "<dummy>".to_string(),
            openbao_entity_id: "65732d02-bb5f-7ce7-eae4-69067383b61d".to_string(),
            openbao_preferred_auth_id: "auth/my_sso".to_string(),
        },
    };
    libparsec_platform_device_loader::save_pending_async_enrollment(
        &env.discriminant_dir,
        cleartext_content.clone(),
        &SigningKey::generate(),
        &PrivateKey::generate(),
        // Wrong ciphertext key, the decryption will fail
        &SecretKey::generate(),
        enrollment_file_path.clone(),
    )
    .await
    .unwrap();

    let new_device_save_strategy = DeviceSaveStrategy::new_keyring();

    p_assert_matches!(
        submitter_finalize_async_enrollment(
            config,
            &enrollment_file_path,
            &new_device_save_strategy,
            &identity_strategy,
        )
        .await,
        Err(SubmitterFinalizeAsyncEnrollmentError::EnrollmentFileCannotRetrieveCiphertextKey(_))
    );
}

#[parsec_test(testbed = "empty")]
async fn enrollment_file_invalid_data(env: &TestbedEnv) {
    let config = make_config(env);
    let identity_strategy = MockedAsyncEnrollmentIdentityStrategy::default();

    let enrollment_id = AsyncEnrollmentID::default();
    let enrollment_file_path =
        libparsec_platform_device_loader::get_default_pending_async_enrollment_file(
            &env.discriminant_dir,
            enrollment_id,
        );

    // Wrong ciphertext key, decryption will fail!
    let dummy_ciphertext_key = SecretKey::generate();
    let identity_system = {
        use crate::SubmitAsyncEnrollmentIdentityStrategy;
        let (_, raw_identity_system) = identity_strategy.generate_ciphertext_key().await.unwrap();
        match raw_identity_system {
            AsyncEnrollmentLocalPendingIdentitySystem::OpenBao {
                openbao_ciphertext_key_path,
                openbao_entity_id,
                openbao_preferred_auth_id,
            } => AsyncEnrollmentLocalPendingIdentitySystem::OpenBao {
                openbao_ciphertext_key_path,
                openbao_entity_id,
                openbao_preferred_auth_id,
            },
            _ => unreachable!(),
        }
    };

    let cleartext_content = AsyncEnrollmentLocalPendingCleartextContent {
        server_url: env.server_addr.clone(),
        organization_id: env.organization_id.clone(),
        submitted_on: "2010-01-01T00:00:00Z".parse().unwrap(),
        enrollment_id,
        requested_device_label: "PC1".parse().unwrap(),
        requested_human_handle: "Mike <mike@example.invalid>".parse().unwrap(),
        identity_system,
    };
    libparsec_platform_device_loader::save_pending_async_enrollment(
        &env.discriminant_dir,
        cleartext_content.clone(),
        &SigningKey::generate(),
        &PrivateKey::generate(),
        &dummy_ciphertext_key,
        enrollment_file_path.clone(),
    )
    .await
    .unwrap();

    let new_device_save_strategy = DeviceSaveStrategy::new_keyring();

    p_assert_matches!(
        submitter_finalize_async_enrollment(
            config,
            &enrollment_file_path,
            &new_device_save_strategy,
            &identity_strategy,
        )
        .await,
        Err(SubmitterFinalizeAsyncEnrollmentError::EnrollmentFileInvalidData)
    );
}

#[parsec_test(testbed = "empty")]
async fn server_info_rep_enrollment_not_found(env: &TestbedEnv) {
    let config = make_config(env);
    let identity_strategy = MockedAsyncEnrollmentIdentityStrategy::default();

    let (enrollment_id, enrollment_file_path) = generate_submit(env, &identity_strategy).await;

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: protocol::anonymous_cmds::latest::async_enrollment_info::Req| {
            p_assert_eq!(req.enrollment_id, enrollment_id);
            protocol::anonymous_cmds::latest::async_enrollment_info::Rep::EnrollmentNotFound
        },
    );

    let new_device_save_strategy = DeviceSaveStrategy::new_keyring();

    p_assert_matches!(
        submitter_finalize_async_enrollment(
            config,
            &enrollment_file_path,
            &new_device_save_strategy,
            &identity_strategy,
        )
        .await,
        Err(SubmitterFinalizeAsyncEnrollmentError::EnrollmentNotFoundOnServer),
    );
}
