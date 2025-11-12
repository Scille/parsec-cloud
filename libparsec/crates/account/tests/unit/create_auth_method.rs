// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use libparsec_client_connection::{
    protocol::{anonymous_server_cmds, authenticated_account_cmds},
    test_register_sequence_of_send_hooks, test_send_hook_vault_item_list, ProxyConfig,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{
    Account, AccountAuthMethodStrategy, AccountCreateAuthMethodError, AccountLoginStrategy,
};

#[parsec_test(testbed = "empty", with_server)]
async fn ok(#[values("master_secret", "password")] kind: &str, env: &TestbedEnv) {
    let (account_human_handle, auth_method_master_secret) =
        test_new_account(&env.server_addr).await.unwrap();
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &auth_method_master_secret,
        account_human_handle.clone(),
    )
    .await;

    let new_password: Password = "P@ssw0rd.".to_string().into();
    // ⚠️ We cannot use a constant for the new master secret here, otherwise
    // the testbed server might reject us if it already contains the auth method ID
    // derived from this master secret!
    let new_master_secret = KeyDerivation::generate();

    let (auth_method_strategy, login_strategy) = match kind {
        "master_secret" => (
            AccountAuthMethodStrategy::MasterSecret(&new_master_secret),
            AccountLoginStrategy::MasterSecret(&new_master_secret),
        ),
        "password" => (
            AccountAuthMethodStrategy::Password(&new_password),
            AccountLoginStrategy::Password {
                email: account.human_handle.email(),
                password: &new_password,
            },
        ),
        _ => panic!("Unknown kind: {kind}"),
    };

    account
        .create_auth_method(auth_method_strategy)
        .await
        .unwrap();

    // Now try the connection

    let account2 = Account::login(
        env.discriminant_dir.clone(),
        ProxyConfig::default(),
        env.server_addr.clone(),
        login_strategy,
    )
    .await
    .unwrap();

    // Also make sure both auth methods agree on the vault key

    p_assert_eq!(
        account.test_get_vault_key().await,
        account2.test_get_vault_key().await,
    );
}

#[parsec_test(testbed = "empty")]
async fn ok_mocked(#[values("master_secret", "password")] kind: &str, env: &TestbedEnv) {
    let human_handle: HumanHandle = "Zack <zack@example.com>".parse().unwrap();
    let account1 = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        human_handle.clone(),
    )
    .await;

    let vault_key = SecretKey::generate();

    let new_password: Password = "P@ssw0rd.".to_string().into();
    let new_master_secret = KeyDerivation::from(hex!(
        "4a2bd7ff1096b61cc520eb237a3ccb53d517d60679f5afd55f2a0a780f26ed85"
    ));

    let (auth_method_strategy, login_strategy) = match kind {
        "master_secret" => (
            AccountAuthMethodStrategy::MasterSecret(&new_master_secret),
            AccountLoginStrategy::MasterSecret(&new_master_secret),
        ),
        "password" => (
            AccountAuthMethodStrategy::Password(&new_password),
            AccountLoginStrategy::Password {
                email: account1.human_handle.email(),
                password: &new_password,
            },
        ),
        _ => panic!("Unknown kind: {kind}"),
    };

    let retrieved_stuff = Arc::new(Mutex::new(None));

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        test_send_hook_vault_item_list!(env, &account1.auth_method_secret_key, &vault_key),
        {
            let retrieved_stuff = retrieved_stuff.clone();
            move |req: authenticated_account_cmds::latest::auth_method_create::Req| {
                retrieved_stuff.lock().unwrap().replace((
                    req.auth_method_password_algorithm,
                    req.auth_method_id,
                    req.auth_method_mac_key,
                    req.vault_key_access,
                ));

                authenticated_account_cmds::latest::auth_method_create::Rep::Ok
            }
        }
    );

    account1
        .create_auth_method(auth_method_strategy)
        .await
        .unwrap();

    // Now try to use our new auth method !

    let (auth_method_password_algorithm, _auth_method_id, _auth_method_mac_key, vault_key_access) =
        retrieved_stuff.lock().unwrap().take().unwrap();

    // Do a login...

    if kind == "password" {
        test_register_sequence_of_send_hooks!(
            &env.discriminant_dir,
            {
                move |_req: anonymous_server_cmds::latest::auth_method_password_get_algorithm::Req| {
                    anonymous_server_cmds::latest::auth_method_password_get_algorithm::Rep::Ok {
                        password_algorithm: auth_method_password_algorithm.unwrap(),
                    }
                }
            },
            {
                let human_handle = human_handle.clone();
                move |_req: authenticated_account_cmds::latest::account_info::Req| {
                    authenticated_account_cmds::latest::account_info::Rep::Ok { human_handle }
                }
            }
        );
    } else {
        test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
            let human_handle = human_handle.clone();
            move |_req: authenticated_account_cmds::latest::account_info::Req| {
                authenticated_account_cmds::latest::account_info::Rep::Ok { human_handle }
            }
        });
    }

    let account2 = Account::login(
        env.discriminant_dir.clone(),
        ProxyConfig::default(),
        env.server_addr.clone(),
        login_strategy,
    )
    .await
    .unwrap();

    // ... and check the vault key access uploaded by the new auth method.

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: authenticated_account_cmds::latest::vault_item_list::Req| {
            authenticated_account_cmds::latest::vault_item_list::Rep::Ok {
                items: HashMap::new(),
                key_access: vault_key_access,
            }
        }
    );

    p_assert_eq!(account2.test_get_vault_key().await, vault_key,);
}

#[parsec_test(testbed = "empty")]
async fn auth_method_id_already_exists(env: &TestbedEnv) {
    let master_secret = KeyDerivation::from(hex!(
        "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
    ));
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &master_secret,
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;

    let vault_key = SecretKey::generate();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        test_send_hook_vault_item_list!(env, &account.auth_method_secret_key, &vault_key),
        move |_req: authenticated_account_cmds::latest::auth_method_create::Req| {
            authenticated_account_cmds::latest::auth_method_create::Rep::AuthMethodIdAlreadyExists
        }
    );

    let result = account
        .create_auth_method(AccountAuthMethodStrategy::MasterSecret(&master_secret))
        .await;

    p_assert_matches!(result, Err(AccountCreateAuthMethodError::Internal(err))
        if format!("{err}") == "Unexpected server response: AuthMethodIdAlreadyExists"
    );
}

#[parsec_test(testbed = "empty")]
async fn offline(env: &TestbedEnv) {
    let master_secret = KeyDerivation::from(hex!(
        "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
    ));
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &master_secret,
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;

    p_assert_matches!(
        account
            .create_auth_method(AccountAuthMethodStrategy::MasterSecret(&master_secret))
            .await,
        Err(AccountCreateAuthMethodError::Offline(_))
    );
}

#[parsec_test(testbed = "empty")]
async fn unknown_server_response(
    #[values("during_vault_item_list", "during_auth_method_create")] kind: &str,
    env: &TestbedEnv,
) {
    let master_secret = KeyDerivation::from(hex!(
        "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
    ));
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &master_secret,
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;

    let vault_key = SecretKey::generate();

    match kind {
        "during_vault_item_list" => {
            test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
                move |_req: authenticated_account_cmds::latest::vault_item_list::Req| {
                    authenticated_account_cmds::latest::vault_item_list::Rep::UnknownStatus {
                        unknown_status: "unknown".to_string(),
                        reason: None,
                    }
                }
            });
        }

        "during_auth_method_create" => {
            test_register_sequence_of_send_hooks!(
                &env.discriminant_dir,
                test_send_hook_vault_item_list!(env, &account.auth_method_secret_key, &vault_key),
                move |_req: authenticated_account_cmds::latest::auth_method_create::Req| {
                    authenticated_account_cmds::latest::auth_method_create::Rep::UnknownStatus {
                        unknown_status: "unknown".to_string(),
                        reason: None,
                    }
                }
            );
        }

        unknown => panic!("Unknown kind: {unknown}"),
    }

    p_assert_matches!(
        account.create_auth_method(AccountAuthMethodStrategy::MasterSecret(&master_secret)).await,
        Err(AccountCreateAuthMethodError::Internal(err))
        if format!("{err}") == "Unexpected server response: UnknownStatus { unknown_status: \"unknown\", reason: None }"
    );
}
