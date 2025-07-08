// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use libparsec_client_connection::{
    test_register_sequence_of_send_hooks, test_send_hook_vault_item_list, ProxyConfig,
};
use libparsec_protocol::authenticated_account_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{Account, AccountEncryptDeviceFileAccountVaultAndUploadAccessError};

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

    let ciphertext = account
        .save_device_file_account_vault_and_upload_access(&alice)
        .await
        .unwrap();

    // Finally test the round-trip

    p_assert_eq!(
        account
            .fetch_access_and_load_device_file_account_vault(&alice)
            .await
            .unwrap(),
        alice
    );
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
    let device_file_key: Arc<Mutex<Option<SecretKey>>> = Default::default();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        test_send_hook_vault_item_list!(env, &account.auth_method_secret_key, &vault_key),
        {
            let alice = alice.clone();
            let vault_key = vault_key.clone();
            let device_file_key = device_file_key.clone();

            move |req: authenticated_account_cmds::latest::vault_item_upload::Req| {
                let item = AccountVaultItem::load(&req.item).unwrap();
                p_assert_eq!(req.item_fingerprint, item.fingerprint());
                match item {
                    AccountVaultItem::OpaqueKey(item) => {
                        p_assert_eq!(item.organization_id, *alice.organization_id());
                        p_assert_eq!(item.device_id, alice.device_id);
                        let key_access = AccountVaultItemOpaqueKeyEncryptedData::decrypt_and_load(
                            &item.encrypted_data,
                            &vault_key,
                        )
                        .unwrap();
                        *device_file_key.lock().unwrap() = Some(key_access.device_file_key);
                    }
                    AccountVaultItem::RegistrationDevice(unexpected) => {
                        unreachable!("{:?}", unexpected)
                    }
                }

                authenticated_account_cmds::latest::vault_item_upload::Rep::Ok
            }
        },
    );

    let ciphertext = account
        .save_device_file_account_vault_and_upload_access(&alice)
        .await
        .unwrap();

    let device_file_key = device_file_key.lock().unwrap().as_ref().unwrap().to_owned();
    p_assert_eq!(
        LocalDevice::decrypt_and_load(&ciphertext, &device_file_key).unwrap(),
        *alice
    );
}

#[parsec_test(testbed = "minimal")]
async fn offline(
    #[values("during_vault_item_list", "during_vault_item_upload")] kind: &str,
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

        "during_vault_item_upload" => {
            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                test_send_hook_vault_item_list!(
                    env,
                    &account.auth_method_secret_key,
                    &SecretKey::generate()
                ),
                // `vault_item_upload` is missing !
            );
        }

        unknown => panic!("Unknown kind: {}", unknown),
    }

    p_assert_matches!(
        account
            .save_device_file_account_vault_and_upload_access(&alice)
            .await,
        Err(AccountEncryptDeviceFileAccountVaultAndUploadAccessError::Offline(_))
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
        move |_req: authenticated_account_cmds::latest::vault_item_upload::Req| {
            authenticated_account_cmds::latest::vault_item_upload::Rep::FingerprintAlreadyExists
        }
    );

    p_assert_matches!(
        account.save_device_file_account_vault_and_upload_access(&alice).await,
        Err(AccountEncryptDeviceFileAccountVaultAndUploadAccessError::Internal(err))
        if format!("{}", err) == "Unexpected server response: FingerprintAlreadyExists"
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
        account
            .save_device_file_account_vault_and_upload_access(&alice)
            .await,
        Err(AccountEncryptDeviceFileAccountVaultAndUploadAccessError::BadVaultKeyAccess(_))
    );
}

#[parsec_test(testbed = "minimal")]
async fn unknown_server_response(
    #[values("during_vault_item_list", "during_vault_item_upload")] kind: &str,
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

        "during_vault_item_upload" => {
            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                test_send_hook_vault_item_list!(
                    env,
                    &account.auth_method_secret_key,
                    &SecretKey::generate()
                ),
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
        account.save_device_file_account_vault_and_upload_access(&alice).await,
        Err(AccountEncryptDeviceFileAccountVaultAndUploadAccessError::Internal(err))
        if format!("{}", err) == "Unexpected server response: UnknownStatus { unknown_status: \"unknown\", reason: None }"
    );
}
