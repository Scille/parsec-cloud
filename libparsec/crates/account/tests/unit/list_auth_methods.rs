// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_client_connection::{
    protocol::authenticated_account_cmds, test_register_sequence_of_send_hooks,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{Account, AccountListAuthMethodsError, AuthMethodInfo};

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

    let auth_method_1_id =
        AccountAuthMethodID::from_hex("1fe91588816a4df98400a9dc2d0bdfa7").unwrap();
    let auth_method_2_id =
        AccountAuthMethodID::from_hex("1fe91588816a4df98400a9dc2d0bdfa7").unwrap();
    let auth_method_1_created_on = "2025-01-02T00:00:00Z".parse().unwrap();
    let auth_method_2_created_on = "2025-01-02T00:00:00Z".parse().unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        move |_req: authenticated_account_cmds::latest::auth_method_list::Req| {
            authenticated_account_cmds::latest::auth_method_list::Rep::Ok {
                items: vec![
                    authenticated_account_cmds::latest::auth_method_list::AuthMethod {
                        auth_method_id: auth_method_1_id,
                        created_on: auth_method_1_created_on,
                        created_by_ip: "127.0.0.1".to_string(),
                        created_by_user_agent: "Parsec-Client/3.4.1 Windows".to_string(),
                        vault_key_access: b"<auth_method_1_vault_key_access>".to_vec().into(),
                        password_algorithm: Some(UntrustedPasswordAlgorithm::Argon2id {
                            memlimit_kb: 131072,
                            opslimit: 3,
                            parallelism: 1,
                        }),
                    },
                    authenticated_account_cmds::latest::auth_method_list::AuthMethod {
                        auth_method_id: auth_method_2_id,
                        created_on: auth_method_2_created_on,
                        created_by_ip: "127.0.0.2".to_string(),
                        created_by_user_agent: "Parsec-Client/3.4.2 Windows".to_string(),
                        vault_key_access: b"<auth_method_2_vault_key_access>".to_vec().into(),
                        password_algorithm: None,
                    },
                ],
            }
        }
    );

    p_assert_eq!(
        account.list_auth_methods().await.unwrap(),
        vec![
            AuthMethodInfo {
                auth_method_id: auth_method_1_id,
                created_on: auth_method_1_created_on,
                created_by_ip: "127.0.0.1".to_string(),
                created_by_user_agent: "Parsec-Client/3.4.1 Windows".to_string(),
                use_password: true,
            },
            AuthMethodInfo {
                auth_method_id: auth_method_2_id,
                created_on: auth_method_2_created_on,
                created_by_ip: "127.0.0.2".to_string(),
                created_by_user_agent: "Parsec-Client/3.4.2 Windows".to_string(),
                use_password: false,
            },
        ]
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
        account.list_auth_methods().await,
        Err(AccountListAuthMethodsError::Offline(_))
    );
}

#[parsec_test(testbed = "minimal")]
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

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        move |_req: authenticated_account_cmds::latest::auth_method_list::Req| {
            authenticated_account_cmds::latest::auth_method_list::Rep::UnknownStatus {
                unknown_status: "unknown".to_string(),
                reason: None,
            }
        }
    });

    p_assert_matches!(
    account.list_auth_methods().await,
        Err(AccountListAuthMethodsError::Internal(err))
        if format!("{err}") == "Unexpected server response: UnknownStatus { unknown_status: \"unknown\", reason: None }"
    );
}
