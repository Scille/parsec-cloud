// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use libparsec_client_connection::{
    test_register_sequence_of_send_hooks, test_send_hook_vault_item_list,
};
use libparsec_protocol::authenticated_account_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{Account, AccountUploadOpaqueKeyInVaultError};

#[parsec_test(testbed = "empty")]
async fn ok_mocked(env: &TestbedEnv) {
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;

    let vault_key = SecretKey::generate();
    let expected_stuff: Arc<Mutex<Option<(AccountVaultItemOpaqueKeyID, SecretKey)>>> =
        Default::default();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        test_send_hook_vault_item_list!(env, &account.auth_method_secret_key, &vault_key),
        {
            let vault_key = vault_key.clone();
            let expected_stuff = expected_stuff.clone();

            move |req: authenticated_account_cmds::latest::vault_item_upload::Req| {
                let item = AccountVaultItem::load(&req.item).unwrap();
                p_assert_eq!(req.item_fingerprint, item.fingerprint());
                match item {
                    AccountVaultItem::OpaqueKey(item) => {
                        let key_access = AccountVaultItemOpaqueKeyEncryptedData::decrypt_and_load(
                            &item.encrypted_data,
                            &vault_key,
                        )
                        .unwrap();
                        p_assert_eq!(item.key_id, key_access.key_id);
                        *expected_stuff.lock().unwrap() = Some((item.key_id, key_access.key));
                    }
                    AccountVaultItem::RegistrationDevice(unexpected) => {
                        unreachable!("{:?}", unexpected)
                    }
                }

                authenticated_account_cmds::latest::vault_item_upload::Rep::Ok
            }
        },
    );

    let (ciphertext_key_id, ciphertext_key) = account.upload_opaque_key_in_vault().await.unwrap();

    let (expected_ciphertext_key_id, expected_ciphertext_key) =
        expected_stuff.lock().unwrap().clone().unwrap();
    p_assert_eq!(ciphertext_key_id, expected_ciphertext_key_id);
    p_assert_eq!(ciphertext_key, expected_ciphertext_key);
}

#[parsec_test(testbed = "empty")]
async fn offline(
    #[values(
        "during_vault_item_list",
        "during_organization_self_list",
        "during_vault_item_upload"
    )]
    kind: &str,
    env: &TestbedEnv,
) {
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;

    match kind {
        "during_vault_item_list" => {
            // No send hook, so initial `vault_item_list` will fail
        }

        "during_organization_self_list" => {
            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                test_send_hook_vault_item_list!(
                    env,
                    &account.auth_method_secret_key,
                    &SecretKey::generate()
                ),
                // `organization_self_list` is missing !
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
                // `vault_item_upload` is missing !
            );
        }

        unknown => panic!("Unknown kind: {unknown}"),
    }

    p_assert_matches!(
        account.upload_opaque_key_in_vault().await,
        Err(AccountUploadOpaqueKeyInVaultError::Offline(_))
    );
}

#[parsec_test(testbed = "empty")]
async fn fingerprint_already_exists(env: &TestbedEnv) {
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &KeyDerivation::from(hex!(
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
        account.upload_opaque_key_in_vault().await.unwrap_err(),
        err @ AccountUploadOpaqueKeyInVaultError::BadServerResponse(_)
        if err.to_string() == "The Parsec account server returned an unexpected response: FingerprintAlreadyExists"
    );
}

#[parsec_test(testbed = "empty")]
async fn bad_vault_key_access(env: &TestbedEnv) {
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &KeyDerivation::from(hex!(
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
        account.upload_opaque_key_in_vault().await,
        Err(AccountUploadOpaqueKeyInVaultError::BadVaultKeyAccess(_))
    );
}

#[parsec_test(testbed = "empty")]
async fn unknown_server_response(
    #[values("during_vault_item_list", "during_vault_item_upload")] kind: &str,
    env: &TestbedEnv,
) {
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &KeyDerivation::from(hex!(
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

        unknown => panic!("Unknown kind: {unknown}"),
    }

    p_assert_matches!(
        account.upload_opaque_key_in_vault().await.unwrap_err(),
        err @ AccountUploadOpaqueKeyInVaultError::BadServerResponse(_)
        if err.to_string() == "The Parsec account server returned an unexpected response: UnknownStatus { unknown_status: \"unknown\", reason: None }"
    );
}
