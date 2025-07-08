// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::collections::HashMap;

use libparsec_client_connection::{
    protocol::authenticated_account_cmds, test_register_sequence_of_send_hooks,
    test_send_hook_vault_item_list,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{Account, AccountFetchOpaqueKeyFromVaultError};

#[parsec_test(testbed = "empty")]
async fn ok(env: &TestbedEnv) {
    let vault_key = SecretKey::generate();
    let opaque_key_id = AccountVaultItemOpaqueKeyID::default();
    let opaque_key = SecretKey::generate();
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
        test_send_hook_vault_item_list!(env, &account.auth_method_secret_key, &vault_key, opaque_keys: [
            (opaque_key_id, opaque_key),
            (AccountVaultItemOpaqueKeyID::default(), &SecretKey::generate()),
        ]),
    );

    p_assert_eq!(
        account
            .fetch_opaque_key_from_vault(opaque_key_id)
            .await
            .unwrap(),
        opaque_key
    );
}

#[parsec_test(testbed = "empty")]
async fn bad_vault_key_access(env: &TestbedEnv) {
    let opaque_key_id = AccountVaultItemOpaqueKeyID::default();
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
        account.fetch_opaque_key_from_vault(opaque_key_id).await,
        Err(AccountFetchOpaqueKeyFromVaultError::BadVaultKeyAccess(_))
    );
}

#[parsec_test(testbed = "empty")]
async fn offline(env: &TestbedEnv) {
    let opaque_key_id = AccountVaultItemOpaqueKeyID::default();
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
        account.fetch_opaque_key_from_vault(opaque_key_id).await,
        Err(AccountFetchOpaqueKeyFromVaultError::Offline(_))
    );
}

#[parsec_test(testbed = "empty")]
async fn unknown_server_response(env: &TestbedEnv) {
    let opaque_key_id = AccountVaultItemOpaqueKeyID::default();
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
        account.fetch_opaque_key_from_vault(opaque_key_id).await,
        Err(AccountFetchOpaqueKeyFromVaultError::Internal(err))
        if format!("{}", err) == "Unexpected server response: UnknownStatus { unknown_status: \"unknown\", reason: None }"
    );
}
