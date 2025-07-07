// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::collections::{HashMap, HashSet};

use libparsec_client_connection::{
    protocol::authenticated_account_cmds, test_register_sequence_of_send_hooks,
    test_send_hook_vault_item_list,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{Account, AccountListRegistrationDevicesError};

#[parsec_test(testbed = "coolorg")]
async fn ok(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let bob = env.local_device("bob@dev1");
    let vault_key = SecretKey::generate();
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        alice.human_handle.clone(),
    )
    .await;

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        test_send_hook_vault_item_list!(env, &account.auth_method_secret_key, &vault_key, registration_devices: [&alice, &bob]),
    );

    p_assert_eq!(
        account.list_registration_devices().await.unwrap(),
        HashSet::from_iter([
            (alice.organization_id().to_owned(), alice.user_id),
            (bob.organization_id().to_owned(), bob.user_id),
        ])
    );
}

#[parsec_test(testbed = "coolorg")]
async fn bad_vault_key_access(env: &TestbedEnv) {
    let account = Account::test_new(
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
        account.list_registration_devices().await,
        Err(AccountListRegistrationDevicesError::BadVaultKeyAccess(_))
    );
}

#[parsec_test(testbed = "empty")]
async fn offline(env: &TestbedEnv) {
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;

    p_assert_matches!(
        account.list_registration_devices().await,
        Err(AccountListRegistrationDevicesError::Offline(_))
    );
}

#[parsec_test(testbed = "empty")]
async fn unknown_server_response(env: &TestbedEnv) {
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
            authenticated_account_cmds::latest::vault_item_list::Rep::UnknownStatus {
                unknown_status: "unknown".to_string(),
                reason: None,
            }
        }
    );

    p_assert_matches!(
        account.list_registration_devices().await,
        Err(AccountListRegistrationDevicesError::Internal(err))
        if format!("{}", err) == "Unexpected server response: UnknownStatus { unknown_status: \"unknown\", reason: None }"
    );
}
