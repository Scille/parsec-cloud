// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_client_connection::{
    protocol::anonymous_account_cmds, test_register_sequence_of_send_hooks, AnonymousAccountCmds,
    ProxyConfig,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{Account, AccountCreateSendValidationEmailError};

#[parsec_test(testbed = "empty", with_server)]
async fn ok_with_server_and_create(env: &TestbedEnv) {
    // Randomize email since the testbed server will conserve it between runs
    let email: EmailAddress = format!("{}@example.com", uuid::Uuid::new_v4())
        .parse()
        .unwrap();
    let cmds = AnonymousAccountCmds::new(
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

    // TODO: Get validation code from the email and use to to actually create the account
    //       once migration out of invitation addr is done.

    // let invitation_token = {
    //     let mail_body: &str = mails[0].2.as_ref();
    //     let needle = "<a href=\"";
    //     let addr_start = mail_body.find(needle).unwrap() + needle.len();
    //     let addr_offset = mail_body[addr_start..].find("\"").unwrap();
    //     // Note we cannot use the address to configure `AnonymousAccountCmds` since its
    //     // contains an invalid domain (i.e. `saas.parsec.invalid/`)
    //     let invitation_addr = ParsecAccountEmailValidationAddr::from_http_redirection(&mail_body[addr_start..addr_start + addr_offset]).unwrap();
    //     invitation_addr.token()
    // };

    // let human_handle: HumanHandle = "Zack <zack@example.com>".parse().unwrap();
    // let password: Password = "P@ssw0rd.".to_string().into();
    // let validation_code: ValidationCode = "AD3FXJ".parse().unwrap();

    // let cmds = AnonymousAccountCmds::new(
    //     &env.discriminant_dir,
    //     env.server_addr.clone(),
    //     ProxyConfig::default(),
    // )
    // .unwrap();

    // p_assert_matches!(
    //     Account::create_3_proceed(
    //         &cmds,
    //         validation_code,
    //         human_handle.clone(),
    //         password.clone()
    //     )
    //     .await,
    //     Ok(())
    // );
}

#[parsec_test(testbed = "empty")]
async fn ok_mocked(env: &TestbedEnv) {
    let email: EmailAddress = "zack@example.com".parse().unwrap();
    let cmds = AnonymousAccountCmds::new(
        &env.discriminant_dir,
        env.server_addr.clone(),
        ProxyConfig::default(),
    )
    .unwrap();

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        let email = email.clone();
        move |req: anonymous_account_cmds::latest::account_create_send_validation_email::Req| {
            p_assert_eq!(req.email, email);
            anonymous_account_cmds::latest::account_create_send_validation_email::Rep::Ok
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
    let cmds = AnonymousAccountCmds::new(
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
    let cmds = AnonymousAccountCmds::new(
        &env.discriminant_dir,
        env.server_addr.clone(),
        ProxyConfig::default(),
    )
    .unwrap();

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        let email = email.clone();
        move |req: anonymous_account_cmds::latest::account_create_send_validation_email::Req| {
            p_assert_eq!(req.email, email);
            anonymous_account_cmds::latest::account_create_send_validation_email::Rep::EmailRecipientRefused
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
    let cmds = AnonymousAccountCmds::new(
        &env.discriminant_dir,
        env.server_addr.clone(),
        ProxyConfig::default(),
    )
    .unwrap();

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        let email = email.clone();
        move |req: anonymous_account_cmds::latest::account_create_send_validation_email::Req| {
            p_assert_eq!(req.email, email);
            anonymous_account_cmds::latest::account_create_send_validation_email::Rep::EmailServerUnavailable
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
async fn unknown_status(env: &TestbedEnv) {
    let email: EmailAddress = "zack@example.com".parse().unwrap();
    let cmds = AnonymousAccountCmds::new(
        &env.discriminant_dir,
        env.server_addr.clone(),
        ProxyConfig::default(),
    )
    .unwrap();

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        let email = email.clone();
        move |req: anonymous_account_cmds::latest::account_create_send_validation_email::Req| {
            p_assert_eq!(req.email, email);
            anonymous_account_cmds::latest::account_create_send_validation_email::Rep::UnknownStatus { unknown_status: "unknown".to_string(), reason: None }
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
        if format!("{}", err) == "Unexpected server response: UnknownStatus { unknown_status: \"unknown\", reason: None }"
    );
}
