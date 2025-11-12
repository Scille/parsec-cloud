// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_client_connection::{
    protocol::anonymous_server_cmds, test_register_sequence_of_send_hooks, AnonymousServerCmds,
    ProxyConfig,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{Account, AccountCreateError};

// Note a test with the actual server is done within send_validation_email's tests

#[parsec_test(testbed = "empty")]
async fn ok(env: &TestbedEnv) {
    let email: EmailAddress = "zack@example.com".parse().unwrap();
    let validation_code: ValidationCode = "AD3FXJ".parse().unwrap();
    let cmds = AnonymousServerCmds::new(
        &env.discriminant_dir,
        env.server_addr.clone(),
        ProxyConfig::default(),
    )
    .unwrap();

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        let email = email.clone();
        let validation_code = validation_code.clone();
        move |req: anonymous_server_cmds::latest::account_create_proceed::Req| {
            p_assert_eq!(req.account_create_step, anonymous_server_cmds::latest::account_create_proceed::AccountCreateStep::Number0CheckCode { email, validation_code });
            anonymous_server_cmds::latest::account_create_proceed::Rep::Ok
        }
    });

    p_assert_matches!(
        Account::create_2_check_validation_code(&cmds, validation_code, email).await,
        Ok(())
    );
}

#[parsec_test(testbed = "empty")]
async fn offline(env: &TestbedEnv) {
    let email: EmailAddress = "zack@example.com".parse().unwrap();
    let validation_code: ValidationCode = "AD3FXJ".parse().unwrap();
    let cmds = AnonymousServerCmds::new(
        &env.discriminant_dir,
        env.server_addr.clone(),
        ProxyConfig::default(),
    )
    .unwrap();

    p_assert_matches!(
        Account::create_2_check_validation_code(&cmds, validation_code, email)
            .await
            .unwrap_err(),
        AccountCreateError::Offline(_)
    );
}

#[parsec_test(testbed = "empty")]
async fn unknown_status(env: &TestbedEnv) {
    let email: EmailAddress = "zack@example.com".parse().unwrap();
    let validation_code: ValidationCode = "AD3FXJ".parse().unwrap();
    let cmds = AnonymousServerCmds::new(
        &env.discriminant_dir,
        env.server_addr.clone(),
        ProxyConfig::default(),
    )
    .unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: anonymous_server_cmds::latest::account_create_proceed::Req| {
            anonymous_server_cmds::latest::account_create_proceed::Rep::UnknownStatus {
                unknown_status: "unknown".to_string(),
                reason: None,
            }
        }
    );

    p_assert_matches!(
        Account::create_2_check_validation_code(
            &cmds,
            validation_code,
            email,
        )
        .await
        .unwrap_err(),
        AccountCreateError::Internal(err)
        if format!("{err}") == "Unexpected server response: UnknownStatus { unknown_status: \"unknown\", reason: None }"
    );
}

#[parsec_test(testbed = "empty")]
async fn invalid_validation_code(env: &TestbedEnv) {
    let email: EmailAddress = "zack@example.com".parse().unwrap();
    let validation_code: ValidationCode = "AD3FXJ".parse().unwrap();
    let cmds = AnonymousServerCmds::new(
        &env.discriminant_dir,
        env.server_addr.clone(),
        ProxyConfig::default(),
    )
    .unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: anonymous_server_cmds::latest::account_create_proceed::Req| {
            anonymous_server_cmds::latest::account_create_proceed::Rep::InvalidValidationCode
        }
    );

    p_assert_matches!(
        Account::create_2_check_validation_code(&cmds, validation_code, email)
            .await
            .unwrap_err(),
        AccountCreateError::InvalidValidationCode
    );
}

#[parsec_test(testbed = "empty")]
async fn send_validation_email_required(env: &TestbedEnv) {
    let email: EmailAddress = "zack@example.com".parse().unwrap();
    let validation_code: ValidationCode = "AD3FXJ".parse().unwrap();
    let cmds = AnonymousServerCmds::new(
        &env.discriminant_dir,
        env.server_addr.clone(),
        ProxyConfig::default(),
    )
    .unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: anonymous_server_cmds::latest::account_create_proceed::Req| {
            anonymous_server_cmds::latest::account_create_proceed::Rep::SendValidationEmailRequired
        }
    );

    p_assert_matches!(
        Account::create_2_check_validation_code(&cmds, validation_code, email)
            .await
            .unwrap_err(),
        AccountCreateError::SendValidationEmailRequired
    );
}

#[parsec_test(testbed = "empty")]
async fn auth_method_id_already_exists(env: &TestbedEnv) {
    let email: EmailAddress = "zack@example.com".parse().unwrap();
    let validation_code: ValidationCode = "AD3FXJ".parse().unwrap();
    let cmds = AnonymousServerCmds::new(
        &env.discriminant_dir,
        env.server_addr.clone(),
        ProxyConfig::default(),
    )
    .unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: anonymous_server_cmds::latest::account_create_proceed::Req| {
            anonymous_server_cmds::latest::account_create_proceed::Rep::AuthMethodIdAlreadyExists
        }
    );

    p_assert_matches!(
        Account::create_2_check_validation_code(&cmds, validation_code, email)
            .await
            .unwrap_err(),
        AccountCreateError::Internal(err)
        if format!("{err}") == "Unexpected server response: AuthMethodIdAlreadyExists"
    );
}
