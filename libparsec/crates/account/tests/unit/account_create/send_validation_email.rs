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

use crate::{
    Account, AccountAuthMethodStrategy, AccountCreateSendValidationEmailError, AccountLoginStrategy,
};

#[parsec_test(testbed = "empty", with_server)]
async fn ok_with_server_then_check_code_and_create(env: &TestbedEnv) {
    // Randomize email since the testbed server will conserve it between runs
    let email: EmailAddress = format!("{}@example.com", uuid::Uuid::new_v4())
        .parse()
        .unwrap();
    let cmds = AnonymousServerCmds::new(
        &env.discriminant_dir,
        env.server_addr.clone(),
        ProxyConfig::default(),
    )
    .unwrap();

    let mails = libparsec_tests_fixtures::test_check_mailbox(&env.server_addr, &email)
        .await
        .unwrap();
    p_assert_eq!(mails.len(), 0);

    p_assert_matches!(
        Account::create_1_send_validation_email(&cmds, email.clone()).await,
        Ok(())
    );

    let mails = libparsec_tests_fixtures::test_check_mailbox(&env.server_addr, &email)
        .await
        .unwrap();
    p_assert_eq!(mails.len(), 1);

    let validation_code: ValidationCode = {
        let mail_body: &str = mails[0].2.as_ref();
        const NEEDLE_START: &str = "<pre id=\"code\">";
        const NEEDLE_END: &str = "</pre>";
        let start = mail_body.find(NEEDLE_START).unwrap() + NEEDLE_START.len();
        let offset = mail_body[start..].find(NEEDLE_END).unwrap();
        mail_body[start..start + offset].parse().unwrap()
    };

    // All ready, now try to check the code...

    // ⚠️ We cannot use a constant for the new master secret here, otherwise
    // the testbed server might reject us if it already contains the auth method ID
    // derived from this master secret!
    let new_auth_method_master_secret = KeyDerivation::generate();

    let human_handle = HumanHandle::new(email, "Zack").unwrap();

    let cmds = AnonymousServerCmds::new(
        &env.discriminant_dir,
        env.server_addr.clone(),
        ProxyConfig::default(),
    )
    .unwrap();

    p_assert_matches!(
        Account::create_2_check_validation_code(
            &cmds,
            validation_code.clone(),
            human_handle.email().to_owned(),
        )
        .await,
        Ok(())
    );

    // ...and do the actual account creation...

    p_assert_matches!(
        Account::create_3_proceed(
            &cmds,
            validation_code,
            human_handle.clone(),
            AccountAuthMethodStrategy::MasterSecret(&new_auth_method_master_secret)
        )
        .await,
        Ok(())
    );

    // ...and finally we can connect with the new auth method!

    Account::login(
        env.discriminant_dir.clone(),
        ProxyConfig::default(),
        env.server_addr.clone(),
        AccountLoginStrategy::MasterSecret(&new_auth_method_master_secret),
    )
    .await
    .unwrap();
}

#[parsec_test(testbed = "empty")]
async fn ok_mocked(env: &TestbedEnv) {
    let email: EmailAddress = "zack@example.com".parse().unwrap();
    let cmds = AnonymousServerCmds::new(
        &env.discriminant_dir,
        env.server_addr.clone(),
        ProxyConfig::default(),
    )
    .unwrap();

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        let email = email.clone();
        move |req: anonymous_server_cmds::latest::account_create_send_validation_email::Req| {
            p_assert_eq!(req.email, email);
            anonymous_server_cmds::latest::account_create_send_validation_email::Rep::Ok
        }
    });

    p_assert_matches!(
        Account::create_1_send_validation_email(&cmds, email.clone()).await,
        Ok(())
    );
}

#[parsec_test(testbed = "empty")]
async fn offline(env: &TestbedEnv) {
    let email: EmailAddress = "zack@example.com".parse().unwrap();
    let cmds = AnonymousServerCmds::new(
        &env.discriminant_dir,
        env.server_addr.clone(),
        ProxyConfig::default(),
    )
    .unwrap();

    p_assert_matches!(
        Account::create_1_send_validation_email(&cmds, email)
            .await
            .unwrap_err(),
        AccountCreateSendValidationEmailError::Offline(_)
    );
}

#[parsec_test(testbed = "empty")]
async fn email_recipient_refused(env: &TestbedEnv) {
    let email: EmailAddress = "zack@example.com".parse().unwrap();
    let cmds = AnonymousServerCmds::new(
        &env.discriminant_dir,
        env.server_addr.clone(),
        ProxyConfig::default(),
    )
    .unwrap();

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        let email = email.clone();
        move |req: anonymous_server_cmds::latest::account_create_send_validation_email::Req| {
            p_assert_eq!(req.email, email);
            anonymous_server_cmds::latest::account_create_send_validation_email::Rep::EmailRecipientRefused
        }
    });

    p_assert_matches!(
        Account::create_1_send_validation_email(&cmds, email)
            .await
            .unwrap_err(),
        AccountCreateSendValidationEmailError::EmailRecipientRefused
    );
}

#[parsec_test(testbed = "empty")]
async fn email_server_unavailable(env: &TestbedEnv) {
    let email: EmailAddress = "zack@example.com".parse().unwrap();
    let cmds = AnonymousServerCmds::new(
        &env.discriminant_dir,
        env.server_addr.clone(),
        ProxyConfig::default(),
    )
    .unwrap();

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        let email = email.clone();
        move |req: anonymous_server_cmds::latest::account_create_send_validation_email::Req| {
            p_assert_eq!(req.email, email);
            anonymous_server_cmds::latest::account_create_send_validation_email::Rep::EmailServerUnavailable
        }
    });

    p_assert_matches!(
        Account::create_1_send_validation_email(&cmds, email)
            .await
            .unwrap_err(),
        AccountCreateSendValidationEmailError::EmailServerUnavailable
    );
}

#[parsec_test(testbed = "empty")]
async fn email_sending_rate_limited(env: &TestbedEnv) {
    let email: EmailAddress = "zack@example.com".parse().unwrap();
    let cmds = AnonymousServerCmds::new(
        &env.discriminant_dir,
        env.server_addr.clone(),
        ProxyConfig::default(),
    )
    .unwrap();
    let expected_wait_until = "2000-01-01T00:00:00Z".parse().unwrap();

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        let email = email.clone();
        move |req: anonymous_server_cmds::latest::account_create_send_validation_email::Req| {
            p_assert_eq!(req.email, email);
            anonymous_server_cmds::latest::account_create_send_validation_email::Rep::EmailSendingRateLimited {
                wait_until: expected_wait_until
            }
        }
    });

    p_assert_matches!(
        Account::create_1_send_validation_email(&cmds, email)
            .await
            .unwrap_err(),
        AccountCreateSendValidationEmailError::EmailSendingRateLimited { wait_until } if wait_until == expected_wait_until
    );
}

#[parsec_test(testbed = "empty")]
async fn unknown_status(env: &TestbedEnv) {
    let email: EmailAddress = "zack@example.com".parse().unwrap();
    let cmds = AnonymousServerCmds::new(
        &env.discriminant_dir,
        env.server_addr.clone(),
        ProxyConfig::default(),
    )
    .unwrap();

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        let email = email.clone();
        move |req: anonymous_server_cmds::latest::account_create_send_validation_email::Req| {
            p_assert_eq!(req.email, email);
            anonymous_server_cmds::latest::account_create_send_validation_email::Rep::UnknownStatus { unknown_status: "unknown".to_string(), reason: None }
        }
    });

    p_assert_matches!(
        Account::create_1_send_validation_email(
            &cmds,
            email
        )
        .await
        .unwrap_err(),
        AccountCreateSendValidationEmailError::Internal(err)
        if format!("{err}") == "Unexpected server response: UnknownStatus { unknown_status: \"unknown\", reason: None }"
    );
}
