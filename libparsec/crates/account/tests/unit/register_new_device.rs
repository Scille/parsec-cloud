// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_client::ProxyConfig;
use libparsec_client_connection::{
    test_register_sequence_of_send_hooks, test_send_hook_vault_item_list, AuthenticatedCmds,
};
use libparsec_platform_device_loader::{
    AvailableDevice, AvailableDeviceType, DeviceAccessStrategy, DeviceSaveStrategy,
};
use libparsec_protocol::{authenticated_account_cmds, authenticated_cmds};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{Account, AccountRegisterNewDeviceError};

#[parsec_test(testbed = "minimal", with_server)]
async fn ok_with_server(env: &TestbedEnv) {
    let org_id = env.organization_id.clone();
    let org_user_id = UserID::default();
    let (human_handle, auth_method_master_secret) =
        libparsec_tests_fixtures::test_new_account(&env.server_addr)
            .await
            .unwrap();

    // Register a new user in the organization with the account's email
    let org_device1_id = env
        .customize(|builder| {
            builder
                .new_user(org_user_id)
                .customize(|e| {
                    e.human_handle = human_handle.clone();
                })
                .map(|e| e.first_device_id)
        })
        .await;

    let device1 = env.local_device(org_device1_id);

    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &auth_method_master_secret,
        human_handle.clone(),
    )
    .await;

    // Must start by uploading the a registration device in the vault...

    let registration_device_id = account
        .create_registration_device(device1.clone())
        .await
        .unwrap();
    p_assert_ne!(registration_device_id, device1.device_id); // Sanity check

    // ...then we can use the registration device to create a new device

    let new_device_label: DeviceLabel = "NewLabel".parse().unwrap();
    let available_device = account
        .register_new_device(
            org_id.clone(),
            org_user_id,
            new_device_label.clone(),
            DeviceSaveStrategy::Password {
                password: "P@ssw0rd.".to_owned().into(),
            },
        )
        .await
        .unwrap();

    p_assert_eq!(
        available_device,
        AvailableDevice {
            key_file_path: available_device.key_file_path.clone(),
            created_on: available_device.created_on,
            protected_on: available_device.created_on, // Protected and created should be the same
            server_url: env.server_addr.to_http_url(None).to_string(),
            organization_id: org_id.clone(),
            user_id: org_user_id,
            device_id: available_device.device_id,
            human_handle: human_handle.clone(),
            device_label: new_device_label,
            ty: AvailableDeviceType::Password,
        }
    );
    // Make sure that an *actual* new device has been created
    p_assert_ne!(available_device.device_id, registration_device_id);
    p_assert_ne!(available_device.device_id, device1.device_id);

    // Try to use the new device

    let new_device = libparsec_platform_device_loader::load_device(
        &env.discriminant_dir,
        &DeviceAccessStrategy::Password {
            key_file: available_device.key_file_path,
            password: "P@ssw0rd.".to_owned().into(),
        },
    )
    .await
    .unwrap();

    let cmds =
        AuthenticatedCmds::new(&env.discriminant_dir, new_device, ProxyConfig::default()).unwrap();
    p_assert_eq!(
        cmds.send(authenticated_cmds::latest::ping::Req {
            ping: "ping".to_string()
        })
        .await
        .unwrap(),
        authenticated_cmds::latest::ping::Rep::Ok {
            pong: "ping".to_string()
        }
    );
}

#[parsec_test(testbed = "minimal")]
async fn ok_mocked(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let vault_key = SecretKey::generate();
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        alice.human_handle.clone(),
    )
    .await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        test_send_hook_vault_item_list!(env, &account.auth_method_secret_key, &vault_key, registration_devices: [&alice]),
        {
            let alice = alice.clone();

            move |req: authenticated_cmds::latest::device_create::Req| {
                // Minimal tests on the content of the request, the heavy lifting
                // will be taken care of by the server in `ok_with_server` test.
                let certif = DeviceCertificate::verify_and_load(
                    &req.device_certificate,
                    &alice.verify_key(),
                    CertificateSigner::User(alice.device_id),
                    None,
                )
                .unwrap();
                let redacted_certif = DeviceCertificate::verify_and_load(
                    &req.redacted_device_certificate,
                    &alice.verify_key(),
                    CertificateSigner::User(alice.device_id),
                    None,
                )
                .unwrap();
                p_assert_eq!(certif.device_id, redacted_certif.device_id);
                p_assert_ne!(certif.device_id, alice.device_id);
                p_assert_matches!(certif.purpose, DevicePurpose::Standard);

                authenticated_cmds::latest::device_create::Rep::Ok
            }
        },
    );

    let new_device_label: DeviceLabel = "NewLabel".parse().unwrap();
    let available_device = account
        .register_new_device(
            alice.organization_id().to_owned(),
            alice.user_id,
            new_device_label.clone(),
            DeviceSaveStrategy::Password {
                password: "P@ssw0rd.".to_owned().into(),
            },
        )
        .await
        .unwrap();

    p_assert_eq!(
        available_device,
        AvailableDevice {
            key_file_path: available_device.key_file_path.clone(),
            created_on: available_device.created_on,
            protected_on: available_device.created_on, // Protected and created should be the same
            server_url: env.server_addr.to_http_url(None).to_string(),
            organization_id: alice.organization_id().to_owned(),
            user_id: alice.user_id,
            device_id: available_device.device_id,
            human_handle: alice.human_handle.clone(),
            device_label: new_device_label,
            ty: AvailableDeviceType::Password,
        }
    );
}

#[parsec_test(testbed = "minimal")]
async fn unknown_registration_device(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let auth_method_master_secret = KeyDerivation::generate();
    let vault_key = SecretKey::generate();
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &auth_method_master_secret,
        alice.human_handle.clone(),
    )
    .await;

    // Bad organization ID

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        test_send_hook_vault_item_list!(env, &account.auth_method_secret_key, &vault_key, registration_devices: [&alice]),
    );

    p_assert_matches!(
        account
            .register_new_device(
                "Dummy".parse().unwrap(),
                alice.user_id,
                DeviceLabel::default(),
                DeviceSaveStrategy::Keyring,
            )
            .await,
        Err(AccountRegisterNewDeviceError::UnknownRegistrationDevice)
    );

    // Bad user ID

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        test_send_hook_vault_item_list!(env, &account.auth_method_secret_key, &vault_key, registration_devices: [&alice]),
    );

    p_assert_matches!(
        account
            .register_new_device(
                alice.organization_id().to_owned(),
                UserID::default(),
                DeviceLabel::default(),
                DeviceSaveStrategy::Keyring,
            )
            .await,
        Err(AccountRegisterNewDeviceError::UnknownRegistrationDevice)
    );
}

#[parsec_test(testbed = "minimal")]
async fn offline(
    #[values("during_vault_item_list", "during_device_create")] kind: &str,
    env: &TestbedEnv,
) {
    let alice = env.local_device("alice@dev1");
    let auth_method_master_secret = KeyDerivation::generate();
    let vault_key = SecretKey::generate();
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &auth_method_master_secret,
        alice.human_handle.clone(),
    )
    .await;

    match kind {
        "during_vault_item_list" => {
            // No send hook, so initial `vault_item_list` will fail
        }

        "during_device_create" => {
            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                test_send_hook_vault_item_list!(env, &account.auth_method_secret_key, &vault_key, registration_devices: [&alice]),
                // `device_create` is missing !
            );
        }

        unknown => panic!("Unknown kind: {unknown}"),
    }

    p_assert_matches!(
        account
            .register_new_device(
                alice.organization_id().to_owned(),
                alice.user_id,
                DeviceLabel::default(),
                DeviceSaveStrategy::Keyring,
            )
            .await,
        Err(AccountRegisterNewDeviceError::Offline(_))
    );
}

#[parsec_test(testbed = "minimal")]
async fn unknown_server_response(
    #[values("during_vault_item_list", "during_device_create")] kind: &str,
    env: &TestbedEnv,
) {
    let alice = env.local_device("alice@dev1");
    let auth_method_master_secret = KeyDerivation::generate();
    let vault_key = SecretKey::generate();
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &auth_method_master_secret,
        alice.human_handle.clone(),
    )
    .await;

    match kind {
        "during_vault_item_list" => {
            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                move |_req: authenticated_account_cmds::latest::vault_item_list::Req| {
                    authenticated_account_cmds::latest::vault_item_list::Rep::UnknownStatus {
                        unknown_status: "unknown".to_string(),
                        reason: None,
                    }
                }
            );
        }

        "during_device_create" => {
            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                test_send_hook_vault_item_list!(env, &account.auth_method_secret_key, &vault_key, registration_devices: [&alice]),
                move |_req: authenticated_cmds::latest::device_create::Req| {
                    authenticated_cmds::latest::device_create::Rep::UnknownStatus {
                        unknown_status: "unknown".to_string(),
                        reason: None,
                    }
                }
            );
        }

        unknown => panic!("Unknown kind: {unknown}"),
    }

    p_assert_matches!(
        account.register_new_device(
            alice.organization_id().to_owned(),
            alice.user_id,
            DeviceLabel::default(),
            DeviceSaveStrategy::Keyring,
        ).await,
        Err(AccountRegisterNewDeviceError::Internal(err))
        if format!("{err}") == "Unexpected server response: UnknownStatus { unknown_status: \"unknown\", reason: None }"
    );
}

#[parsec_test(testbed = "minimal")]
async fn timestamp_out_of_ballpark(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let auth_method_master_secret = KeyDerivation::generate();
    let vault_key = SecretKey::generate();
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &auth_method_master_secret,
        alice.human_handle.clone(),
    )
    .await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        test_send_hook_vault_item_list!(env, &account.auth_method_secret_key, &vault_key, registration_devices: [&alice]),
        move |_req: authenticated_cmds::latest::device_create::Req| {
            authenticated_cmds::latest::device_create::Rep::TimestampOutOfBallpark {
                ballpark_client_early_offset: 300.,
                ballpark_client_late_offset: 320.,
                server_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
                client_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
            }
        }
    );

    p_assert_matches!(
        account.register_new_device(
            alice.organization_id().to_owned(),
            alice.user_id,
            DeviceLabel::default(),
            DeviceSaveStrategy::Keyring,
        ).await,
        Err(AccountRegisterNewDeviceError::TimestampOutOfBallpark { server_timestamp, client_timestamp, ballpark_client_early_offset, ballpark_client_late_offset })
        if
            ballpark_client_early_offset == 300. &&
            ballpark_client_late_offset == 320. &&
            server_timestamp == "2000-1-2T01:00:00Z".parse().unwrap() &&
            client_timestamp == "2000-1-2T01:00:00Z".parse().unwrap()
    );
}
