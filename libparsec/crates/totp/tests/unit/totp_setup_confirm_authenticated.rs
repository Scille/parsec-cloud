// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{protocol::authenticated_cmds, test_register_send_hook};
use libparsec_tests_fixtures::prelude::*;

use super::utils::alice_cmds_factory;
use crate::{totp_setup_confirm_authenticated, TotpSetupConfirmAuthenticatedError};

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let client = alice_cmds_factory(env);

    let one_time_password = "123456";

    test_register_send_hook(
        &env.discriminant_dir,
        move |req: authenticated_cmds::latest::totp_setup_confirm::Req| {
            p_assert_eq!(req.one_time_password, one_time_password);
            authenticated_cmds::latest::totp_setup_confirm::Rep::Ok
        },
    );

    let result = totp_setup_confirm_authenticated(&client, one_time_password.to_string()).await;
    p_assert_matches!(result, Ok(()));
}

#[parsec_test(testbed = "minimal")]
async fn invalid_one_time_password(env: &TestbedEnv) {
    let client = alice_cmds_factory(env);

    test_register_send_hook(
        &env.discriminant_dir,
        move |_req: authenticated_cmds::latest::totp_setup_confirm::Req| {
            authenticated_cmds::latest::totp_setup_confirm::Rep::InvalidOneTimePassword
        },
    );

    let outcome = totp_setup_confirm_authenticated(&client, "000000".to_string()).await;
    p_assert_matches!(
        outcome,
        Err(TotpSetupConfirmAuthenticatedError::InvalidOneTimePassword)
    );
}

#[parsec_test(testbed = "minimal")]
async fn already_setup(env: &TestbedEnv) {
    let client = alice_cmds_factory(env);

    test_register_send_hook(
        &env.discriminant_dir,
        move |_req: authenticated_cmds::latest::totp_setup_confirm::Req| {
            authenticated_cmds::latest::totp_setup_confirm::Rep::AlreadySetup
        },
    );

    let outcome = totp_setup_confirm_authenticated(&client, "123456".to_string()).await;
    p_assert_matches!(
        outcome,
        Err(TotpSetupConfirmAuthenticatedError::AlreadySetup)
    );
}

#[parsec_test(testbed = "minimal")]
async fn offline(env: &TestbedEnv) {
    let client = alice_cmds_factory(env);

    let outcome = totp_setup_confirm_authenticated(&client, "123456".to_string()).await;
    p_assert_matches!(outcome, Err(TotpSetupConfirmAuthenticatedError::Offline(_)));
}
