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

use crate::{Account, AccountRecoverProceedError};

// Note a test with the actual server is done within send_validation_email's tests

#[parsec_test(testbed = "empty")]
async fn ok(env: &TestbedEnv) {
    let human_handle: HumanHandle = "Zack <zack@example.com>".parse().unwrap();
    let email = human_handle.email().to_owned();
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
        let expected_email = email.clone();
        let expected_validation_code = validation_code.clone();
        let retrieved_stuff = retrieved_stuff.clone();
        move |req: anonymous_account_cmds::latest::account_recover_proceed::Req| {
            p_assert_eq!(req.validation_code, expected_validation_code);
            p_assert_eq!(req.email, expected_email);

            retrieved_stuff.lock().unwrap().replace((
                req.new_auth_method_password_algorithm,
                req.new_auth_method_id,
                req.new_auth_method_mac_key,
                req.new_vault_key_access,
            ));

            anonymous_account_cmds::latest::account_recover_proceed::Rep::Ok
        }
    });

    p_assert_matches!(
        Account::recover_2_proceed(&cmds, validation_code, email.clone(), &password,).await,
        Ok(())
    );

    // Now try the connection

    let (auth_method_password_algorithm, _auth_method_id, _auth_method_mac_key, vault_key_access) =
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
        email.clone(),
        &password,
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
    let email: EmailAddress = "zack@example.com".parse().unwrap();
    let password = "P@ssw0rd.".to_string().into();
    let validation_code: ValidationCode = "AD3FXJ".parse().unwrap();
    let cmds = AnonymousAccountCmds::new(
        &env.discriminant_dir,
        env.server_addr.clone(),
        ProxyConfig::default(),
    )
    .unwrap();

    p_assert_matches!(
        Account::recover_2_proceed(&cmds, validation_code, email, &password)
            .await
            .unwrap_err(),
        AccountRecoverProceedError::Offline(_)
    );
}

#[parsec_test(testbed = "empty")]
async fn unknown_status(env: &TestbedEnv) {
    let email: EmailAddress = "zack@example.com".parse().unwrap();
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
        move |_req: anonymous_account_cmds::latest::account_recover_proceed::Req| {
            anonymous_account_cmds::latest::account_recover_proceed::Rep::UnknownStatus {
                unknown_status: "unknown".to_string(),
                reason: None,
            }
        }
    );

    p_assert_matches!(
        Account::recover_2_proceed(&cmds, validation_code, email, &password)
        .await
        .unwrap_err(),
        AccountRecoverProceedError::Internal(err)
        if format!("{}", err) == "Unexpected server response: UnknownStatus { unknown_status: \"unknown\", reason: None }"
    );
}

#[parsec_test(testbed = "empty")]
async fn invalid_validation_code(env: &TestbedEnv) {
    let email: EmailAddress = "zack@example.com".parse().unwrap();
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
        move |_req: anonymous_account_cmds::latest::account_recover_proceed::Req| {
            anonymous_account_cmds::latest::account_recover_proceed::Rep::InvalidValidationCode
        }
    );

    p_assert_matches!(
        Account::recover_2_proceed(&cmds, validation_code, email, &password)
            .await
            .unwrap_err(),
        AccountRecoverProceedError::InvalidValidationCode
    );
}

#[parsec_test(testbed = "empty")]
async fn send_validation_email_required(env: &TestbedEnv) {
    let email: EmailAddress = "zack@example.com".parse().unwrap();
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
        move |_req: anonymous_account_cmds::latest::account_recover_proceed::Req| {
            anonymous_account_cmds::latest::account_recover_proceed::Rep::SendValidationEmailRequired
        }
    );

    p_assert_matches!(
        Account::recover_2_proceed(&cmds, validation_code, email, &password)
            .await
            .unwrap_err(),
        AccountRecoverProceedError::SendValidationEmailRequired
    );
}

#[parsec_test(testbed = "empty")]
async fn auth_method_id_already_exists(env: &TestbedEnv) {
    let email: EmailAddress = "zack@example.com".parse().unwrap();
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
        move |_req: anonymous_account_cmds::latest::account_recover_proceed::Req| {
            anonymous_account_cmds::latest::account_recover_proceed::Rep::AuthMethodIdAlreadyExists
        }
    );

    p_assert_matches!(
        Account::recover_2_proceed(&cmds, validation_code, email, &password)
            .await
            .unwrap_err(),
        AccountRecoverProceedError::AuthMethodIdAlreadyExists
    );
}
