// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::collections::{HashMap, HashSet};

use libparsec_client_connection::{
    protocol::authenticated_account_cmds, test_register_sequence_of_send_hooks,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{Account, AccountFetchRegistrationDevicesError};

#[parsec_test(testbed = "coolorg")]
async fn list_and_fetch_ok(env: &TestbedEnv) {
    let mut account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;

    // 1. Account starts with an empty cache

    p_assert_eq!(account.list_registration_devices(), HashSet::new(),);

    let alice1 = env.local_device("alice@dev1");
    let alice2 = env.local_device("alice@dev2");
    let bob1 = env.local_device("bob@dev1");

    // 2. Now refresh the cache

    let vault_key1 = SecretKey::generate();

    let vault_item_web_local_device = AccountVaultItemWebLocalDeviceKey {
        organization_id: alice1.organization_id().to_owned(),
        device_id: alice1.device_id,
        encrypted_data: alice1.dump_and_encrypt(&vault_key1).into(),
    };

    let alice2_vault_item_registration_device = AccountVaultItemRegistrationDevice {
        organization_id: alice2.organization_id().to_owned(),
        user_id: alice2.user_id,
        encrypted_data: alice2.dump_and_encrypt(&vault_key1).into(),
    };

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        let key_access = AccountVaultKeyAccess {
            vault_key: vault_key1.clone(),
        }
        .dump_and_encrypt(&account.auth_method_secret_key)
        .into();

        move |_req: authenticated_account_cmds::latest::vault_item_list::Req| {
            authenticated_account_cmds::latest::vault_item_list::Rep::Ok {
                key_access,
                items: HashMap::from_iter([
                    // Invalid data is ignored
                    (
                        HashDigest::from_data(b"ignored_invalid_item"),
                        Bytes::from_static(b"<dummy>"),
                    ),
                    // `AccountVaultItemWebLocalDeviceKey` are also ignored
                    (
                        HashDigest::from_data(b"ignored_web_local_device_item"),
                        vault_item_web_local_device.dump().into(),
                    ),
                    (
                        HashDigest::from_data(b"alice2_vault_item_registration_device"),
                        alice2_vault_item_registration_device.dump().into(),
                    ),
                ]),
            }
        }
    });

    account.fetch_registration_devices().await.unwrap();

    p_assert_eq!(
        account.list_registration_devices(),
        HashSet::from_iter([(alice2.organization_id().to_owned(), alice2.user_id),])
    );

    // 3. Another refresh entirely overwrite the previous cache

    // While not supposed to occur in practice, we change the vault key to ensure
    // the previous vault key hasn't been kept in an internal cache.
    let vault_key2 = SecretKey::generate();

    let alice1_vault_item_registration_device = AccountVaultItemRegistrationDevice {
        organization_id: alice1.organization_id().to_owned(),
        user_id: alice1.user_id,
        encrypted_data: alice1.dump_and_encrypt(&vault_key2).into(),
    };
    let bob1_vault_item_registration_device = AccountVaultItemRegistrationDevice {
        organization_id: bob1.organization_id().to_owned(),
        user_id: bob1.user_id,
        encrypted_data: bob1.dump_and_encrypt(&vault_key2).into(),
    };

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        let key_access = AccountVaultKeyAccess {
            vault_key: vault_key2.clone(),
        }
        .dump_and_encrypt(&account.auth_method_secret_key)
        .into();

        move |_req: authenticated_account_cmds::latest::vault_item_list::Req| {
            authenticated_account_cmds::latest::vault_item_list::Rep::Ok {
                key_access,
                items: HashMap::from_iter([
                    (
                        HashDigest::from_data(b"alice1_vault_item_registration_device"),
                        alice1_vault_item_registration_device.dump().into(),
                    ),
                    (
                        HashDigest::from_data(b"bob1_vault_item_registration_device"),
                        bob1_vault_item_registration_device.dump().into(),
                    ),
                ]),
            }
        }
    });

    account.fetch_registration_devices().await.unwrap();

    p_assert_eq!(
        account.list_registration_devices(),
        HashSet::from_iter([
            (alice1.organization_id().to_owned(), alice1.user_id),
            (bob1.organization_id().to_owned(), bob1.user_id),
        ])
    );
}

#[parsec_test(testbed = "coolorg")]
async fn fetch_invalid_vault_key(env: &TestbedEnv) {
    let mut account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        move |_req: authenticated_account_cmds::latest::vault_item_list::Req| {
            authenticated_account_cmds::latest::vault_item_list::Rep::Ok {
                key_access: Bytes::from_static(b"<dummy>"),
                items: HashMap::new(),
            }
        }
    });

    p_assert_matches!(
        account.fetch_registration_devices().await,
        Err(AccountFetchRegistrationDevicesError::BadVaultKeyAccess(_))
    );
}

#[parsec_test(testbed = "empty")]
async fn fetch_offline(env: &TestbedEnv) {
    let mut account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;

    p_assert_matches!(
        account.fetch_registration_devices().await,
        Err(AccountFetchRegistrationDevicesError::Offline(_))
    );
}

#[parsec_test(testbed = "empty")]
async fn fetch_unknown_server_response(env: &TestbedEnv) {
    let mut account = Account::test_new(
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
            authenticated_account_cmds::latest::vault_item_list::Rep::UnknownStatus {
                unknown_status: "unknown".to_string(),
                reason: None,
            }
        }
    );

    p_assert_matches!(
        account.fetch_registration_devices().await,
        Err(AccountFetchRegistrationDevicesError::Internal(err))
        if format!("{}", err) == "Unexpected server response: UnknownStatus { unknown_status: \"unknown\", reason: None }"
    );
}
