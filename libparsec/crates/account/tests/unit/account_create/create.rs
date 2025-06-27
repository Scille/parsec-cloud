// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use libparsec_client_connection::{
    protocol::{anonymous_account_cmds, authenticated_account_cmds},
    test_register_sequence_of_send_hooks, AnonymousAccountCmds, ProxyConfig,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{Account, AccountCreateError};

#[parsec_test(testbed = "empty")]
async fn ok(env: &TestbedEnv) {
    let human_handle: HumanHandle = "Zack <zack@example.com>".parse().unwrap();
    let password: Password = "P@ssw0rd.".to_string().into();
    let validation_code: ValidationCode = "AD3FXJ".parse().unwrap();
    let cmds = AnonymousAccountCmds::new(
        &env.discriminant_dir,
        env.server_addr.clone(),
        ProxyConfig::default(),
    )
    .unwrap();

    let retrieved_stuff = Arc::new(Mutex::new(None));

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        let expected_human_handle = human_handle.clone();
        let expected_validation_code = validation_code.clone();
        let retrieved_stuff = retrieved_stuff.clone();
        move |req: anonymous_account_cmds::latest::account_create_proceed::Req| {
            match req.account_create_step {
                anonymous_account_cmds::latest::account_create_proceed::AccountCreateStep::Number1Create {
                    human_handle,
                    validation_code,
                    auth_method_password_algorithm,
                    auth_method_id,
                    auth_method_hmac_key,
                    vault_key_access,
                } => {
                    p_assert_eq!(human_handle, expected_human_handle);
                    p_assert_eq!(validation_code, expected_validation_code);

                    retrieved_stuff.lock().unwrap().replace((
                        auth_method_password_algorithm,
                        auth_method_id,
                        auth_method_hmac_key,
                        vault_key_access,
                    ));
                }
                unknown => unreachable!("{:?}", unknown)
            }
            anonymous_account_cmds::latest::account_create_proceed::Rep::Ok
        }
    });

    p_assert_matches!(
        Account::create_3_proceed(
            &cmds,
            validation_code,
            human_handle.clone(),
            password.clone()
        )
        .await,
        Ok(())
    );

    // Now try the connection

    let (auth_method_password_algorithm, _auth_method_id, _auth_method_hmac_key, vault_key_access) =
        retrieved_stuff.lock().unwrap().take().unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        {
            move |_req: anonymous_account_cmds::latest::auth_method_password_get_algorithm::Req| {
                anonymous_account_cmds::latest::auth_method_password_get_algorithm::Rep::Ok {
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

    let account = Account::login_with_password(
        env.discriminant_dir.clone(),
        ProxyConfig::default(),
        env.server_addr.clone(),
        human_handle.email().to_owned(),
        password,
    )
    .await
    .unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: authenticated_account_cmds::latest::vault_item_list::Req| {
            authenticated_account_cmds::latest::vault_item_list::Rep::Ok {
                items: HashMap::new(),
                key_access: vault_key_access,
            }
        }
    );

    account.fetch_registration_devices().await.unwrap();
}

#[parsec_test(testbed = "empty")]
async fn offline(env: &TestbedEnv) {
    let human_handle: HumanHandle = "Zack <zack@example.com>".parse().unwrap();
    let password = "P@ssw0rd.".to_string().into();
    let validation_code: ValidationCode = "AD3FXJ".parse().unwrap();
    let cmds = AnonymousAccountCmds::new(
        &env.discriminant_dir,
        env.server_addr.clone(),
        ProxyConfig::default(),
    )
    .unwrap();

    p_assert_matches!(
        Account::create_3_proceed(&cmds, validation_code, human_handle, password)
            .await
            .unwrap_err(),
        AccountCreateError::Offline(_)
    );
}

#[parsec_test(testbed = "empty")]
async fn unknown_status(env: &TestbedEnv) {
    let human_handle: HumanHandle = "Zack <zack@example.com>".parse().unwrap();
    let password = "P@ssw0rd.".to_string().into();
    let validation_code: ValidationCode = "AD3FXJ".parse().unwrap();
    let cmds = AnonymousAccountCmds::new(
        &env.discriminant_dir,
        env.server_addr.clone(),
        ProxyConfig::default(),
    )
    .unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: anonymous_account_cmds::latest::account_create_proceed::Req| {
            anonymous_account_cmds::latest::account_create_proceed::Rep::UnknownStatus {
                unknown_status: "unknown".to_string(),
                reason: None,
            }
        }
    );

    p_assert_matches!(
        Account::create_3_proceed(&cmds, validation_code, human_handle, password)
        .await
        .unwrap_err(),
        AccountCreateError::Internal(err)
        if format!("{}", err) == "Unexpected server response: UnknownStatus { unknown_status: \"unknown\", reason: None }"
    );
}

#[parsec_test(testbed = "empty")]
async fn invalid_validation_code(env: &TestbedEnv) {
    let human_handle: HumanHandle = "Zack <zack@example.com>".parse().unwrap();
    let password = "P@ssw0rd.".to_string().into();
    let validation_code: ValidationCode = "AD3FXJ".parse().unwrap();
    let cmds = AnonymousAccountCmds::new(
        &env.discriminant_dir,
        env.server_addr.clone(),
        ProxyConfig::default(),
    )
    .unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: anonymous_account_cmds::latest::account_create_proceed::Req| {
            anonymous_account_cmds::latest::account_create_proceed::Rep::InvalidValidationCode
        }
    );

    p_assert_matches!(
        Account::create_3_proceed(&cmds, validation_code, human_handle, password)
            .await
            .unwrap_err(),
        AccountCreateError::InvalidValidationCode
    );
}

#[parsec_test(testbed = "empty")]
async fn auth_method_id_already_exists(env: &TestbedEnv) {
    let human_handle: HumanHandle = "Zack <zack@example.com>".parse().unwrap();
    let password = "P@ssw0rd.".to_string().into();
    let validation_code: ValidationCode = "AD3FXJ".parse().unwrap();
    let cmds = AnonymousAccountCmds::new(
        &env.discriminant_dir,
        env.server_addr.clone(),
        ProxyConfig::default(),
    )
    .unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: anonymous_account_cmds::latest::account_create_proceed::Req| {
            anonymous_account_cmds::latest::account_create_proceed::Rep::AuthMethodIdAlreadyExists
        }
    );

    p_assert_matches!(
        Account::create_3_proceed(&cmds, validation_code, human_handle, password)
            .await
            .unwrap_err(),
        AccountCreateError::AuthMethodIdAlreadyExists
    );
}
