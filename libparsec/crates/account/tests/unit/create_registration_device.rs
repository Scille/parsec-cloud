// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::collections::{HashMap, HashSet};

use libparsec_client_connection::{
    test_register_sequence_of_send_hooks, test_send_hook_vault_item_list, ProxyConfig,
};
use libparsec_protocol::{authenticated_account_cmds, authenticated_cmds};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{Account, AccountCreateRegistrationDeviceError};

#[parsec_test(testbed = "minimal", with_server)]
async fn ok_with_server(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let (_, auth_method_master_secret) =
        libparsec_tests_fixtures::test_new_account(&env.server_addr)
            .await
            .unwrap();

    let account = Account::login_with_master_secret(
        env.discriminant_dir.clone(),
        ProxyConfig::default(),
        env.server_addr.clone(),
        auth_method_master_secret,
    )
    .await
    .unwrap();

    account
        .create_registration_device(alice.clone())
        .await
        .unwrap();

    // List the new registration device from the server

    assert_eq!(
        account.list_registration_devices().await.unwrap(),
        HashSet::from_iter([(alice.organization_id().to_owned(), alice.user_id)])
    );

    // Finally try to use the new registration device

    account
        .register_new_device(
            alice.organization_id().to_owned(),
            alice.user_id,
            "New PC".parse().unwrap(),
            DeviceSaveStrategy::Password {
                password: "P@ssw0rd.".to_string().into(),
            },
        )
        .await
        .unwrap();
}

#[parsec_test(testbed = "minimal")]
async fn ok_mocked(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;

    let vault_key = SecretKey::generate();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        test_send_hook_vault_item_list!(env, &account.auth_method_secret_key, &vault_key),
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
                p_assert_matches!(certif.purpose, DevicePurpose::Registration);

                authenticated_cmds::latest::device_create::Rep::Ok
            }
        },
        {
            let alice = alice.clone();
            let vault_key = vault_key.clone();

            move |req: authenticated_account_cmds::latest::vault_item_upload::Req| {
                let item = AccountVaultItem::load(&req.item).unwrap();
                p_assert_eq!(req.item_fingerprint, item.fingerprint());
                match item {
                    AccountVaultItem::RegistrationDevice(item) => {
                        p_assert_eq!(item.organization_id, *alice.organization_id());
                        p_assert_eq!(item.user_id, alice.user_id);
                        let local_device =
                            LocalDevice::decrypt_and_load(&item.encrypted_data, &vault_key)
                                .unwrap();
                        p_assert_eq!(local_device.organization_id(), alice.organization_id());
                        p_assert_eq!(local_device.user_id, alice.user_id);
                        // The very important part: we should have actually uploaded
                        // a new device!
                        p_assert_ne!(local_device.device_id, alice.device_id);
                    }
                    AccountVaultItem::WebLocalDeviceKey(unexpected) => {
                        unreachable!("{:?}", unexpected)
                    }
                }

                authenticated_account_cmds::latest::vault_item_upload::Rep::Ok
            }
        },
    );

    account
        .create_registration_device(alice.clone())
        .await
        .unwrap();
}

#[parsec_test(testbed = "minimal")]
async fn offline(
    #[values(
        "during_vault_item_list",
        "during_device_create",
        "during_vault_item_upload"
    )]
    kind: &str,
    env: &TestbedEnv,
) {
    let alice = env.local_device("alice@dev1");
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;

    match kind {
        "during_vault_item_list" => {
            // No send hook, so initial `vault_item_list` will fail
        }

        "during_device_create" => {
            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                test_send_hook_vault_item_list!(
                    env,
                    &account.auth_method_secret_key,
                    &SecretKey::generate()
                ),
                // `device_create` is missing !
            );
        }

        "during_vault_item_upload" => {
            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                test_send_hook_vault_item_list!(
                    env,
                    &account.auth_method_secret_key,
                    &SecretKey::generate()
                ),
                move |_req: authenticated_cmds::latest::device_create::Req| {
                    authenticated_cmds::latest::device_create::Rep::Ok
                } // `vault_item_upload` is missing !
            );
        }

        unknown => panic!("Unknown kind: {}", unknown),
    }

    p_assert_matches!(
        account.create_registration_device(alice.clone()).await,
        Err(AccountCreateRegistrationDeviceError::Offline(_))
    );
}

#[parsec_test(testbed = "minimal")]
async fn fingerprint_already_exists(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        test_send_hook_vault_item_list!(
            env,
            &account.auth_method_secret_key,
            &SecretKey::generate()
        ),
        move |_req: authenticated_cmds::latest::device_create::Req| {
            authenticated_cmds::latest::device_create::Rep::Ok
        },
        move |_req: authenticated_account_cmds::latest::vault_item_upload::Req| {
            authenticated_account_cmds::latest::vault_item_upload::Rep::FingerprintAlreadyExists
        }
    );

    p_assert_matches!(
        account.create_registration_device(alice.clone()).await,
        Err(AccountCreateRegistrationDeviceError::Internal(err))
        if format!("{}", err) == "Unexpected server response: FingerprintAlreadyExists"
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
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        test_send_hook_vault_item_list!(
            env,
            &account.auth_method_secret_key,
            &SecretKey::generate()
        ),
        move |_req: authenticated_cmds::latest::device_create::Req| {
            authenticated_cmds::latest::device_create::Rep::TimestampOutOfBallpark {
                ballpark_client_early_offset: 300.,
                ballpark_client_late_offset: 320.,
                client_timestamp: "2000-01-02T00:00:00Z".parse().unwrap(),
                server_timestamp: "2000-01-02T00:00:00Z".parse().unwrap(),
            }
        },
    );

    p_assert_matches!(
        account.create_registration_device(alice.clone()).await,
        Err(AccountCreateRegistrationDeviceError::TimestampOutOfBallpark {
            ballpark_client_early_offset,
            ballpark_client_late_offset,
            client_timestamp,
            server_timestamp,
        })
        if ballpark_client_early_offset == 300.
            && ballpark_client_late_offset == 320.
            && client_timestamp == "2000-01-02T00:00:00Z".parse().unwrap()
            && server_timestamp == "2000-01-02T00:00:00Z".parse().unwrap()
    );
}

#[parsec_test(testbed = "minimal")]
async fn bad_vault_key_access(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: authenticated_account_cmds::latest::vault_item_list::Req| {
            authenticated_account_cmds::latest::vault_item_list::Rep::Ok {
                key_access: Bytes::from_static(b"<invalid>"),
                items: HashMap::new(),
            }
        }
    );

    p_assert_matches!(
        account.create_registration_device(alice.clone()).await,
        Err(AccountCreateRegistrationDeviceError::BadVaultKeyAccess(_))
    );
}

#[parsec_test(testbed = "minimal")]
async fn unknown_server_response(
    #[values(
        "during_vault_item_list",
        "during_device_create",
        "during_vault_item_upload"
    )]
    kind: &str,
    env: &TestbedEnv,
) {
    let alice = env.local_device("alice@dev1");
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        "Zack <zack@example.com>".parse().unwrap(),
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
                test_send_hook_vault_item_list!(
                    env,
                    &account.auth_method_secret_key,
                    &SecretKey::generate()
                ),
                move |_req: authenticated_cmds::latest::device_create::Req| {
                    authenticated_cmds::latest::device_create::Rep::UnknownStatus {
                        unknown_status: "unknown".to_string(),
                        reason: None,
                    }
                },
            );
        }

        "during_vault_item_upload" => {
            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                test_send_hook_vault_item_list!(
                    env,
                    &account.auth_method_secret_key,
                    &SecretKey::generate()
                ),
                move |_req: authenticated_cmds::latest::device_create::Req| {
                    authenticated_cmds::latest::device_create::Rep::Ok
                },
                move |_req: authenticated_account_cmds::latest::vault_item_upload::Req| {
                    authenticated_account_cmds::latest::vault_item_upload::Rep::UnknownStatus {
                        unknown_status: "unknown".to_string(),
                        reason: None,
                    }
                },
            );
        }

        unknown => panic!("Unknown kind: {}", unknown),
    }

    p_assert_matches!(
        account.create_registration_device(alice.clone()).await,
        Err(AccountCreateRegistrationDeviceError::Internal(err))
        if format!("{}", err) == "Unexpected server response: UnknownStatus { unknown_status: \"unknown\", reason: None }"
    );
}
