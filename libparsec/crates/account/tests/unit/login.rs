// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_client_connection::{
    protocol::{anonymous_server_cmds, authenticated_account_cmds},
    test_register_sequence_of_send_hooks, ProxyConfig,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{Account, AccountLoginError, AccountLoginStrategy};

#[parsec_test(testbed = "empty", with_server)]
async fn testbed(env: &TestbedEnv) {
    let (human_handle, auth_method_master_secret) =
        libparsec_tests_fixtures::test_new_account(&env.server_addr)
            .await
            .unwrap();

    let account = Account::login(
        env.discriminant_dir.clone(),
        ProxyConfig::default(),
        env.server_addr.clone(),
        AccountLoginStrategy::MasterSecret(&auth_method_master_secret),
    )
    .await
    .unwrap();
    p_assert_eq!(*account.human_handle(), human_handle);
}

#[parsec_test(testbed = "empty")]
async fn login_with_master_secret(env: &TestbedEnv) {
    let addr = env.server_addr.clone();
    let human_handle: HumanHandle = "Zack <zack@example.com>".parse().unwrap();

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        let human_handle = human_handle.clone();
        move |_req: authenticated_account_cmds::latest::account_info::Req| {
            authenticated_account_cmds::latest::account_info::Rep::Ok { human_handle }
        }
    });

    let account = Account::login(
        env.discriminant_dir.clone(),
        ProxyConfig::default(),
        addr,
        AccountLoginStrategy::MasterSecret(&KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        ))),
    )
    .await
    .unwrap();

    p_assert_eq!(*account.human_handle(), human_handle,);
}

#[parsec_test(testbed = "empty")]
async fn login_with_password(env: &TestbedEnv) {
    let email: EmailAddress = "zack@example.com".parse().unwrap();
    let password: Password = "P@ssw0rd.".to_string().into();
    let human_handle: HumanHandle = "Zack <zack@example.com>".parse().unwrap();

    test_register_sequence_of_send_hooks!(
        &env.discriminant_dir,
        {
            let email = email.clone();
            move |req: anonymous_server_cmds::latest::auth_method_password_get_algorithm::Req| {
                p_assert_eq!(req.email, email);
                anonymous_server_cmds::latest::auth_method_password_get_algorithm::Rep::Ok {
                    password_algorithm: UntrustedPasswordAlgorithm::Argon2id {
                        opslimit: 3,
                        memlimit_kb: 128 * 1024,
                        parallelism: 1,
                    },
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

    let addr = env.server_addr.clone();
    let account = Account::login(
        env.discriminant_dir.clone(),
        ProxyConfig::default(),
        addr,
        AccountLoginStrategy::Password {
            password: &password,
            email: &email,
        },
    )
    .await
    .unwrap();

    p_assert_eq!(*account.human_handle(), human_handle,);
}

#[parsec_test(testbed = "empty")]
async fn login_with_password_server_returns_bad_config(env: &TestbedEnv) {
    let email: EmailAddress = "zack@example.com".parse().unwrap();
    let password: Password = "P@ssw0rd.".to_string().into();

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        let email = email.clone();
        move |req: anonymous_server_cmds::latest::auth_method_password_get_algorithm::Req| {
            p_assert_eq!(req.email, email);
            anonymous_server_cmds::latest::auth_method_password_get_algorithm::Rep::Ok {
                password_algorithm: UntrustedPasswordAlgorithm::Argon2id {
                    opslimit: 3,
                    memlimit_kb: 1, // Too small !
                    parallelism: 1,
                },
            }
        }
    });

    let addr = env.server_addr.clone();
    p_assert_matches!(
        Account::login(
            env.discriminant_dir.clone(),
            ProxyConfig::default(),
            addr,
            AccountLoginStrategy::Password {
                password: &password,
                email: &email
            },
        )
        .await
        .unwrap_err(),
        AccountLoginError::BadPasswordAlgorithm(_)
    );
}

#[parsec_test(testbed = "empty")]
async fn login_with_password_server_offline(env: &TestbedEnv) {
    let email: EmailAddress = "zack@example.com".parse().unwrap();
    let password: Password = "P@ssw0rd.".to_string().into();

    let addr = env.server_addr.clone();
    p_assert_matches!(
        Account::login(
            env.discriminant_dir.clone(),
            ProxyConfig::default(),
            addr,
            AccountLoginStrategy::Password {
                password: &password,
                email: &email
            },
        )
        .await
        .unwrap_err(),
        AccountLoginError::Offline(_)
    );
}

#[parsec_test(testbed = "empty")]
async fn login_with_master_secret_server_offline(env: &TestbedEnv) {
    let addr = env.server_addr.clone();
    p_assert_matches!(
        Account::login(
            env.discriminant_dir.clone(),
            ProxyConfig::default(),
            addr,
            AccountLoginStrategy::MasterSecret(&KeyDerivation::from(hex!(
                "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
            ))),
        )
        .await
        .unwrap_err(),
        AccountLoginError::Offline(_)
    );
}
