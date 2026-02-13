// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{protocol::anonymous_cmds, test_register_send_hook};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::anonymous_cmds_factory;
use crate::{totp_setup_confirm_anonymous, TotpSetupConfirmAnonymousError};

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let cmds = anonymous_cmds_factory(env);

    let user_id = UserID::default();
    let token = AccessToken::default();
    let one_time_password = "123456".to_string();

    test_register_send_hook(&env.discriminant_dir, {
        let one_time_password = one_time_password.clone();
        move |req: anonymous_cmds::latest::totp_setup_confirm::Req| {
            p_assert_eq!(req.user_id, user_id);
            p_assert_eq!(req.token, token);
            p_assert_eq!(req.one_time_password, one_time_password);
            anonymous_cmds::latest::totp_setup_confirm::Rep::Ok
        }
    });

    let result = totp_setup_confirm_anonymous(&cmds, user_id, token, one_time_password).await;
    p_assert_matches!(result, Ok(()));
}

#[parsec_test(testbed = "minimal")]
async fn invalid_one_time_password(env: &TestbedEnv) {
    let cmds = anonymous_cmds_factory(env);

    test_register_send_hook(
        &env.discriminant_dir,
        move |_req: anonymous_cmds::latest::totp_setup_confirm::Req| {
            anonymous_cmds::latest::totp_setup_confirm::Rep::InvalidOneTimePassword
        },
    );

    let outcome = totp_setup_confirm_anonymous(
        &cmds,
        UserID::default(),
        AccessToken::default(),
        "000000".to_string(),
    )
    .await;
    p_assert_matches!(
        outcome,
        Err(TotpSetupConfirmAnonymousError::InvalidOneTimePassword)
    );
}

#[parsec_test(testbed = "minimal")]
async fn bad_token(env: &TestbedEnv) {
    let cmds = anonymous_cmds_factory(env);

    test_register_send_hook(
        &env.discriminant_dir,
        move |_req: anonymous_cmds::latest::totp_setup_confirm::Req| {
            anonymous_cmds::latest::totp_setup_confirm::Rep::BadToken
        },
    );

    let outcome = totp_setup_confirm_anonymous(
        &cmds,
        UserID::default(),
        AccessToken::default(),
        "123456".to_string(),
    )
    .await;
    p_assert_matches!(outcome, Err(TotpSetupConfirmAnonymousError::BadToken));
}

#[parsec_test(testbed = "minimal")]
async fn offline(env: &TestbedEnv) {
    let cmds = anonymous_cmds_factory(env);

    let outcome = totp_setup_confirm_anonymous(
        &cmds,
        UserID::default(),
        AccessToken::default(),
        "123456".to_string(),
    )
    .await;
    p_assert_matches!(outcome, Err(TotpSetupConfirmAnonymousError::Offline(_)));
}
