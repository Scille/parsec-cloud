// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_client_connection::{
    protocol::authenticated_account_cmds, test_register_sequence_of_send_hooks, ConnectionError,
    ProxyConfig,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{Account, AccountDeleteSendValidationEmailError, AccountLoginWithMasterSecretError};

#[parsec_test(testbed = "empty", with_server)]
async fn ok_with_server_then_proceed(env: &TestbedEnv) {
    // Ask the testbed server to create a new account
    let (account_human_handle, auth_method_master_secret) =
        libparsec_tests_fixtures::test_new_account(&env.server_addr)
            .await
            .unwrap();

    let account = Account::login_with_master_secret(
        env.discriminant_dir.clone(),
        ProxyConfig::default(),
        env.server_addr.clone(),
        auth_method_master_secret.clone(),
    )
    .await
    .unwrap();

    // Sanity check
    let mails = libparsec_tests_fixtures::test_check_mailbox(
        &env.server_addr,
        account_human_handle.email(),
    )
    .await
    .unwrap();
    p_assert_eq!(mails.len(), 0);

    // Do the actual operation!

    p_assert_matches!(account.delete_1_send_validation_email().await, Ok(()));

    // Ding! You got a new mail!
    let mails = libparsec_tests_fixtures::test_check_mailbox(
        &env.server_addr,
        account_human_handle.email(),
    )
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

    // All ready, now do the final deletion!

    p_assert_matches!(account.delete_2_proceed(validation_code).await, Ok(()));

    // Finally we can no longer use this account

    p_assert_matches!(
        Account::login_with_master_secret(
            env.discriminant_dir.clone(),
            ProxyConfig::default(),
            env.server_addr.clone(),
            auth_method_master_secret
        )
        .await,
        Err(AccountLoginWithMasterSecretError::Offline(
            ConnectionError::BadAuthenticationInfo
        ))
    );
}

#[parsec_test(testbed = "empty")]
async fn ok_mocked(env: &TestbedEnv) {
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
        move |_req: authenticated_account_cmds::latest::account_delete_send_validation_email::Req| {
            authenticated_account_cmds::latest::account_delete_send_validation_email::Rep::Ok
        }
    });

    p_assert_matches!(account.delete_1_send_validation_email().await, Ok(()));
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
        account.delete_1_send_validation_email().await.unwrap_err(),
        AccountDeleteSendValidationEmailError::Offline(_)
    );
}

#[parsec_test(testbed = "empty")]
async fn email_recipient_refused(env: &TestbedEnv) {
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
        move |_req: authenticated_account_cmds::latest::account_delete_send_validation_email::Req| {
            authenticated_account_cmds::latest::account_delete_send_validation_email::Rep::EmailRecipientRefused
        }
    });

    p_assert_matches!(
        account.delete_1_send_validation_email().await.unwrap_err(),
        AccountDeleteSendValidationEmailError::EmailRecipientRefused
    );
}

#[parsec_test(testbed = "empty")]
async fn email_server_unavailable(env: &TestbedEnv) {
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
        move |_req: authenticated_account_cmds::latest::account_delete_send_validation_email::Req| {
            authenticated_account_cmds::latest::account_delete_send_validation_email::Rep::EmailServerUnavailable
        }
    });

    p_assert_matches!(
        account.delete_1_send_validation_email().await.unwrap_err(),
        AccountDeleteSendValidationEmailError::EmailServerUnavailable
    );
}

#[parsec_test(testbed = "empty")]
async fn email_sending_rate_limited(env: &TestbedEnv) {
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;
    let expected_wait_until = "2000-01-01T00:00:00Z".parse().unwrap();

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        move |_req: authenticated_account_cmds::latest::account_delete_send_validation_email::Req| {
            authenticated_account_cmds::latest::account_delete_send_validation_email::Rep::EmailSendingRateLimited {
                wait_until: expected_wait_until
            }
        }
    });

    p_assert_matches!(
        account.delete_1_send_validation_email().await.unwrap_err(),
        AccountDeleteSendValidationEmailError::EmailSendingRateLimited { wait_until } if wait_until == expected_wait_until
    );
}

#[parsec_test(testbed = "empty")]
async fn unknown_status(env: &TestbedEnv) {
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
        move |_req: authenticated_account_cmds::latest::account_delete_send_validation_email::Req| {
            authenticated_account_cmds::latest::account_delete_send_validation_email::Rep::UnknownStatus { unknown_status: "unknown".to_string(), reason: None }
        }
    });

    p_assert_matches!(
        account.delete_1_send_validation_email()
        .await
        .unwrap_err(),
        AccountDeleteSendValidationEmailError::Internal(err)
        if format!("{}", err) == "Unexpected server response: UnknownStatus { unknown_status: \"unknown\", reason: None }"
    );
}
