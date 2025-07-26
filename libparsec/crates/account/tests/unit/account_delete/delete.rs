// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_client_connection::{
    protocol::authenticated_account_cmds, test_register_sequence_of_send_hooks,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{Account, AccountDeleteProceedError};

// Note a test with the actual server is done within send_validation_email's tests

#[parsec_test(testbed = "empty")]
async fn ok(env: &TestbedEnv) {
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;
    let validation_code: ValidationCode = "AD3FXJ".parse().unwrap();

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        let expected_validation_code = validation_code.clone();
        move |req: authenticated_account_cmds::latest::account_delete_proceed::Req| {
            p_assert_eq!(req.validation_code, expected_validation_code);
            authenticated_account_cmds::latest::account_delete_proceed::Rep::Ok
        }
    });

    p_assert_matches!(account.delete_2_proceed(validation_code).await, Ok(()));
}

#[parsec_test(testbed = "empty")]
async fn offline(env: &TestbedEnv) {
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;
    let validation_code: ValidationCode = "AD3FXJ".parse().unwrap();

    p_assert_matches!(
        account.delete_2_proceed(validation_code).await.unwrap_err(),
        AccountDeleteProceedError::Offline(_)
    );
}

#[parsec_test(testbed = "empty")]
async fn unknown_status(env: &TestbedEnv) {
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;
    let validation_code: ValidationCode = "AD3FXJ".parse().unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: authenticated_account_cmds::latest::account_delete_proceed::Req| {
            authenticated_account_cmds::latest::account_delete_proceed::Rep::UnknownStatus {
                unknown_status: "unknown".to_string(),
                reason: None,
            }
        }
    );

    p_assert_matches!(
        account.delete_2_proceed(validation_code).await.unwrap_err(),
        AccountDeleteProceedError::Internal(err)
        if format!("{}", err) == "Unexpected server response: UnknownStatus { unknown_status: \"unknown\", reason: None }"
    );
}

#[parsec_test(testbed = "empty")]
async fn invalid_validation_code(env: &TestbedEnv) {
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;
    let validation_code: ValidationCode = "AD3FXJ".parse().unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: authenticated_account_cmds::latest::account_delete_proceed::Req| {
            authenticated_account_cmds::latest::account_delete_proceed::Rep::InvalidValidationCode
        }
    );

    p_assert_matches!(
        account.delete_2_proceed(validation_code).await.unwrap_err(),
        AccountDeleteProceedError::InvalidValidationCode
    );
}

#[parsec_test(testbed = "empty")]
async fn send_validation_email_required(env: &TestbedEnv) {
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        "Zack <zack@example.com>".parse().unwrap(),
    )
    .await;
    let validation_code: ValidationCode = "AD3FXJ".parse().unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: authenticated_account_cmds::latest::account_delete_proceed::Req| {
            authenticated_account_cmds::latest::account_delete_proceed::Rep::SendValidationEmailRequired
        }
    );

    p_assert_matches!(
        account.delete_2_proceed(validation_code).await.unwrap_err(),
        AccountDeleteProceedError::SendValidationEmailRequired
    );
}
