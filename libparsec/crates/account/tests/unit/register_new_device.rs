// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_client::ProxyConfig;
use libparsec_client_connection::{test_register_sequence_of_send_hooks, AuthenticatedCmds};
use libparsec_protocol::authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{Account, AccountRegisterNewDeviceError};

#[parsec_test(testbed = "minimal", with_server)]
async fn ok(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        alice.human_handle.clone(),
    )
    .await;
    account.test_set_registration_devices_cache([alice.clone()]);

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
            ty: DeviceFileType::Password,
        }
    );

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
async fn unknown_registration_device(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        alice.human_handle.clone(),
    )
    .await;
    account.test_set_registration_devices_cache([alice.clone()]);

    // Bad organization ID

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
async fn offline(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        alice.human_handle.clone(),
    )
    .await;
    account.test_set_registration_devices_cache([alice.clone()]);

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
async fn unknown_server_response(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        alice.human_handle.clone(),
    )
    .await;
    account.test_set_registration_devices_cache([alice.clone()]);

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: authenticated_cmds::latest::device_create::Req| {
            authenticated_cmds::latest::device_create::Rep::UnknownStatus {
                unknown_status: "unknown".to_string(),
                reason: None,
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
        Err(AccountRegisterNewDeviceError::Internal(err))
        if format!("{}", err) == "Unexpected server response: UnknownStatus { unknown_status: \"unknown\", reason: None }"
    );
}

#[parsec_test(testbed = "minimal")]
async fn timestamp_out_of_ballpark(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        alice.human_handle.clone(),
    )
    .await;
    account.test_set_registration_devices_cache([alice.clone()]);

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
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
