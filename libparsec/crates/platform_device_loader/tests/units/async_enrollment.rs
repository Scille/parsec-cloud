// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use crate::{
    list_pending_async_enrollments, load_pending_async_enrollment, remove_pending_async_enrollment,
    save_pending_async_enrollment, AvailablePendingAsyncEnrollment,
    AvailablePendingAsyncEnrollmentIdentitySystem, RemovePendingAsyncEnrollmentError,
    SaveAsyncEnrollmentLocalPendingError,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_tests_lite::{p_assert_eq, p_assert_matches};
use libparsec_types::prelude::*;

#[parsec_test]
async fn save_list_load(tmp_path: TmpPath) {
    use crate::tests::utils::{create_device_file, key_present_in_system};

    // 1) Not existing directory

    p_assert_eq!(list_pending_async_enrollments(&tmp_path).await.unwrap(), []);

    let enrollments_dir = crate::get_pending_async_enrollment_dir(&tmp_path);

    // 2) Save file

    let enrollment_id =
        AsyncEnrollmentID::from_hex("577668f2-9924-4116-b808-f71206f4fa11").unwrap();

    let file_path = crate::get_default_pending_async_enrollment_file(&tmp_path, enrollment_id);
    assert!(
        file_path.starts_with(&enrollments_dir),
        "{:?}",
        (file_path, enrollments_dir)
    );

    let openbao_entity_id = "98ad4942-b164-467c-9839-f5be64cdb22c".to_string();
    let openbao_preferred_auth_id = "HEXAGONE".to_string();
    let cleartext_content = AsyncEnrollmentLocalPendingCleartextContent {
        server_url: "parsec3://parsec.example.invalid/".parse().unwrap(),
        organization_id: "CoolOrg".parse().unwrap(),
        submitted_on: "2000-01-01T00:00:00Z".parse().unwrap(),
        enrollment_id,
        requested_device_label: "PC".parse().unwrap(),
        requested_human_handle: HumanHandle::from_raw("alice@example.invalid", "Alice").unwrap(),
        identity_system: AsyncEnrollmentLocalPendingIdentitySystem::OpenBao {
            openbao_ciphertext_key_path:
                "65732d02-bb5f-7ce7-eae4-69067383b61d/e89eb9b36b704ff292db320b553fcd32".to_string(),
            openbao_entity_id: openbao_entity_id.clone(),
            openbao_preferred_auth_id: openbao_preferred_auth_id.clone(),
        },
    };

    let to_become_device_signing_key = SigningKey::generate();
    let to_become_user_private_key = PrivateKey::generate();
    let ciphertext_key = SecretKey::generate();
    save_pending_async_enrollment(
        &tmp_path,
        cleartext_content.clone(),
        &to_become_device_signing_key,
        &to_become_user_private_key,
        &ciphertext_key,
        file_path.clone(),
    )
    .await
    .unwrap();
    assert!(key_present_in_system(&file_path).await);

    // Also store invalid data that should just be ignored

    let invalid_file_path = crate::get_default_pending_async_enrollment_file(
        &tmp_path,
        AsyncEnrollmentID::from_hex("299429f5-be97-40b8-ba8d-08425ab568f1").unwrap(),
    );
    create_device_file(&invalid_file_path, b"<dummy>").await;

    // 3) Save file error (target is already a folder)

    p_assert_matches!(
        save_pending_async_enrollment(
            &tmp_path,
            cleartext_content.clone(),
            &SigningKey::generate(),
            &PrivateKey::generate(),
            &SecretKey::generate(),
            tmp_path.clone(), // The target file is already a folder !
        )
        .await,
        Err(SaveAsyncEnrollmentLocalPendingError::InvalidPath(_))
    );

    // 4) Get back file

    let (got_full_content, got_cleartext_content) =
        load_pending_async_enrollment(&tmp_path, &file_path)
            .await
            .unwrap();
    p_assert_eq!(got_cleartext_content, cleartext_content);
    p_assert_eq!(
        ciphertext_key
            .decrypt(&got_full_content.ciphertext_private_key)
            .unwrap(),
        *to_become_user_private_key.to_bytes()
    );
    p_assert_eq!(
        ciphertext_key
            .decrypt(&got_full_content.ciphertext_signing_key)
            .unwrap(),
        *to_become_device_signing_key.to_bytes()
    );
    p_assert_eq!(
        ciphertext_key
            .decrypt(&got_full_content.ciphertext_cleartext_content_digest)
            .unwrap(),
        HashDigest::from_data(&got_full_content.cleartext_content).as_ref()
    );

    p_assert_eq!(
        list_pending_async_enrollments(&tmp_path).await.unwrap(),
        [AvailablePendingAsyncEnrollment {
            file_path: file_path.clone(),
            submitted_on: cleartext_content.submitted_on,
            server_addr: cleartext_content.server_url,
            organization_id: cleartext_content.organization_id,
            enrollment_id,
            requested_device_label: cleartext_content.requested_device_label,
            requested_human_handle: cleartext_content.requested_human_handle,
            identity_system: AvailablePendingAsyncEnrollmentIdentitySystem::OpenBao {
                openbao_entity_id,
                openbao_preferred_auth_id,
            },
        }]
    );

    // 5) Remove file

    remove_pending_async_enrollment(&tmp_path, &file_path)
        .await
        .unwrap();
    assert!(!key_present_in_system(&file_path).await);

    p_assert_eq!(list_pending_async_enrollments(&tmp_path).await.unwrap(), []);
}

#[parsec_test(testbed = "empty")]
async fn testbed(env: &TestbedEnv) {
    // 1) Not existing directory

    p_assert_eq!(
        list_pending_async_enrollments(&env.discriminant_dir)
            .await
            .unwrap(),
        []
    );

    // 2) Save file

    let enrollment_id =
        AsyncEnrollmentID::from_hex("577668f2-9924-4116-b808-f71206f4fa11").unwrap();

    let file_path =
        crate::get_default_pending_async_enrollment_file(&env.discriminant_dir, enrollment_id);
    let openbao_entity_id = "98ad4942-b164-467c-9839-f5be64cdb22c".to_string();
    let openbao_preferred_auth_id = "HEXAGONE".to_string();
    let cleartext_content = AsyncEnrollmentLocalPendingCleartextContent {
        server_url: "parsec3://parsec.example.invalid/".parse().unwrap(),
        organization_id: "CoolOrg".parse().unwrap(),
        submitted_on: "2000-01-01T00:00:00Z".parse().unwrap(),
        enrollment_id,
        requested_device_label: "PC".parse().unwrap(),
        requested_human_handle: HumanHandle::from_raw("alice@example.invalid", "Alice").unwrap(),
        identity_system: AsyncEnrollmentLocalPendingIdentitySystem::OpenBao {
            openbao_ciphertext_key_path:
                "65732d02-bb5f-7ce7-eae4-69067383b61d/e89eb9b36b704ff292db320b553fcd32".to_string(),
            openbao_entity_id: openbao_entity_id.clone(),
            openbao_preferred_auth_id: openbao_preferred_auth_id.clone(),
        },
    };

    let to_become_device_signing_key = SigningKey::generate();
    let to_become_user_private_key = PrivateKey::generate();
    let ciphertext_key = SecretKey::generate();
    save_pending_async_enrollment(
        &env.discriminant_dir,
        cleartext_content.clone(),
        &to_become_device_signing_key,
        &to_become_user_private_key,
        &ciphertext_key,
        file_path.clone(),
    )
    .await
    .unwrap();

    // 4) Get back file

    let (got_full_content, got_cleartext_content) =
        load_pending_async_enrollment(&env.discriminant_dir, &file_path)
            .await
            .unwrap();
    p_assert_eq!(got_cleartext_content, cleartext_content);
    p_assert_eq!(
        ciphertext_key
            .decrypt(&got_full_content.ciphertext_private_key)
            .unwrap(),
        *to_become_user_private_key.to_bytes()
    );
    p_assert_eq!(
        ciphertext_key
            .decrypt(&got_full_content.ciphertext_signing_key)
            .unwrap(),
        *to_become_device_signing_key.to_bytes()
    );
    p_assert_eq!(
        ciphertext_key
            .decrypt(&got_full_content.ciphertext_cleartext_content_digest)
            .unwrap(),
        HashDigest::from_data(&got_full_content.cleartext_content).as_ref()
    );

    p_assert_eq!(
        list_pending_async_enrollments(&env.discriminant_dir)
            .await
            .unwrap(),
        [AvailablePendingAsyncEnrollment {
            file_path: file_path.clone(),
            submitted_on: cleartext_content.submitted_on,
            server_addr: cleartext_content.server_url,
            organization_id: cleartext_content.organization_id,
            enrollment_id,
            requested_device_label: cleartext_content.requested_device_label,
            requested_human_handle: cleartext_content.requested_human_handle,
            identity_system: AvailablePendingAsyncEnrollmentIdentitySystem::OpenBao {
                openbao_entity_id,
                openbao_preferred_auth_id,
            },
        }]
    );

    // 3) Remove file

    remove_pending_async_enrollment(&env.discriminant_dir, &file_path)
        .await
        .unwrap();

    p_assert_matches!(
        remove_pending_async_enrollment(&env.discriminant_dir, &file_path).await,
        Err(RemovePendingAsyncEnrollmentError::NotFound)
    );

    p_assert_eq!(
        list_pending_async_enrollments(&env.discriminant_dir)
            .await
            .unwrap(),
        []
    );
}
