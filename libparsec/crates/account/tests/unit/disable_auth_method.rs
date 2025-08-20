// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_client_connection::{
    protocol::authenticated_account_cmds, test_register_sequence_of_send_hooks,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{Account, AccountDisableAuthMethodError};

#[parsec_test(testbed = "empty")]
async fn ok(env: &TestbedEnv) {
    let human_handle: HumanHandle = "Zack <zack@example.com>".parse().unwrap();
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        human_handle.clone(),
    )
    .await;

    let auth_method_id = AccountAuthMethodID::from_hex("9aae259f748045cc9fe7146eab0b132e").unwrap();

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        move |req: authenticated_account_cmds::latest::auth_method_disable::Req| {
            assert_eq!(req.auth_method_id, auth_method_id);
            authenticated_account_cmds::latest::auth_method_disable::Rep::Ok
        }
    });

    account.disable_auth_method(auth_method_id).await.unwrap();
}

#[parsec_test(testbed = "empty")]
async fn auth_method_not_found(env: &TestbedEnv) {
    let human_handle: HumanHandle = "Zack <zack@example.com>".parse().unwrap();
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        human_handle.clone(),
    )
    .await;

    let auth_method_id = AccountAuthMethodID::from_hex("9aae259f748045cc9fe7146eab0b132e").unwrap();

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        move |_req: authenticated_account_cmds::latest::auth_method_disable::Req| {
            authenticated_account_cmds::latest::auth_method_disable::Rep::AuthMethodNotFound
        }
    });

    let result = account.disable_auth_method(auth_method_id).await;

    p_assert_matches!(
        result,
        Err(AccountDisableAuthMethodError::AuthMethodNotFound)
    );
}

#[parsec_test(testbed = "empty")]
async fn auth_method_already_disabled(env: &TestbedEnv) {
    let human_handle: HumanHandle = "Zack <zack@example.com>".parse().unwrap();
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        human_handle.clone(),
    )
    .await;

    let auth_method_id = AccountAuthMethodID::from_hex("9aae259f748045cc9fe7146eab0b132e").unwrap();

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        move |_req: authenticated_account_cmds::latest::auth_method_disable::Req| {
            authenticated_account_cmds::latest::auth_method_disable::Rep::AuthMethodAlreadyDisabled
        }
    });

    let result = account.disable_auth_method(auth_method_id).await;

    p_assert_matches!(
        result,
        Err(AccountDisableAuthMethodError::AuthMethodAlreadyDisabled)
    );
}

#[parsec_test(testbed = "empty")]
async fn self_disable_not_allowed(env: &TestbedEnv) {
    let human_handle: HumanHandle = "Zack <zack@example.com>".parse().unwrap();
    let account = Account::test_new(
        env.discriminant_dir.clone(),
        env.server_addr.clone(),
        &KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
        human_handle.clone(),
    )
    .await;

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        move |_req: authenticated_account_cmds::latest::auth_method_disable::Req| {
            authenticated_account_cmds::latest::auth_method_disable::Rep::SelfDisableNotAllowed
        }
    });

    let result = account.disable_auth_method(account.auth_method_id()).await;

    p_assert_matches!(
        result,
        Err(AccountDisableAuthMethodError::SelfDisableNotAllowed)
    );
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

    let auth_method_id = AccountAuthMethodID::from_hex("9aae259f748045cc9fe7146eab0b132e").unwrap();

    p_assert_matches!(
        account.disable_auth_method(auth_method_id).await,
        Err(AccountDisableAuthMethodError::Offline(_))
    );
}

#[parsec_test(testbed = "empty")]
async fn unknown_server_response(env: &TestbedEnv) {
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

    let auth_method_id = AccountAuthMethodID::from_hex("9aae259f748045cc9fe7146eab0b132e").unwrap();

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        move |_req: authenticated_account_cmds::latest::auth_method_disable::Req| {
            authenticated_account_cmds::latest::auth_method_disable::Rep::UnknownStatus {
                unknown_status: "unknown".to_string(),
                reason: None,
            }
        }
    });

    p_assert_matches!(
    account.disable_auth_method(auth_method_id).await,
        Err(AccountDisableAuthMethodError::Internal(err))
        if format!("{err}") == "Unexpected server response: UnknownStatus { unknown_status: \"unknown\", reason: None }"
    );
}
