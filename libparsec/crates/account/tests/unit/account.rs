// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_client_connection::{
    protocol::anonymous_account_cmds, test_register_sequence_of_send_hooks, ProxyConfig,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use crate::{Account, AccountFromPasswordError};

#[parsec_test(testbed = "empty")]
async fn from_master_secret(env: &TestbedEnv) {
    let addr = env.server_addr.clone();
    Account::new(
        &env.discriminant_dir,
        ProxyConfig::default(),
        addr,
        KeyDerivation::from(hex!(
            "2ff13803789977db4f8ccabfb6b26f3e70eb4453d396dcb2315f7690cbc2e3f1"
        )),
    )
    .unwrap();
}

#[parsec_test(testbed = "empty")]
async fn from_password(env: &TestbedEnv) {
    let email: EmailAddress = "zack@example.com".parse().unwrap();
    let password: Password = "P@ssw0rd.".to_string().into();

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        let email = email.clone();
        move |req: anonymous_account_cmds::latest::auth_method_password_get_algorithm::Req| {
            p_assert_eq!(req.email, email);
            anonymous_account_cmds::latest::auth_method_password_get_algorithm::Rep::Ok {
                password_algorithm: PasswordAlgorithm::Argon2id {
                    salt: b"s4lT-s4lt".to_vec(),
                    opslimit: 1,
                    memlimit_kb: 1024,
                    parallelism: 1,
                },
            }
        }
    });

    let addr = env.server_addr.clone();
    Account::from_password(
        &env.discriminant_dir,
        ProxyConfig::default(),
        addr,
        email,
        password,
    )
    .await
    .unwrap();
}

#[parsec_test(testbed = "empty")]
async fn from_password_server_returns_bad_config(env: &TestbedEnv) {
    let email: EmailAddress = "zack@example.com".parse().unwrap();
    let password: Password = "P@ssw0rd.".to_string().into();

    test_register_sequence_of_send_hooks!(&env.discriminant_dir, {
        let email = email.clone();
        move |req: anonymous_account_cmds::latest::auth_method_password_get_algorithm::Req| {
            p_assert_eq!(req.email, email);
            anonymous_account_cmds::latest::auth_method_password_get_algorithm::Rep::Ok {
                password_algorithm: PasswordAlgorithm::Argon2id {
                    salt: b"s4lT-s4lt".to_vec(),
                    opslimit: 1,
                    memlimit_kb: 1, // Too small !
                    parallelism: 1,
                },
            }
        }
    });

    let addr = env.server_addr.clone();
    p_assert_matches!(
        Account::from_password(
            &env.discriminant_dir,
            ProxyConfig::default(),
            addr,
            email,
            password,
        )
        .await
        .unwrap_err(),
        AccountFromPasswordError::BadPasswordAlgorithm(_)
    );
}

#[parsec_test(testbed = "empty")]
async fn from_password_server_offline(env: &TestbedEnv) {
    let email: EmailAddress = "zack@example.com".parse().unwrap();
    let password: Password = "P@ssw0rd.".to_string().into();

    let addr = env.server_addr.clone();
    p_assert_matches!(
        Account::from_password(
            &env.discriminant_dir,
            ProxyConfig::default(),
            addr,
            email,
            password,
        )
        .await
        .unwrap_err(),
        AccountFromPasswordError::Offline(_)
    );
}
