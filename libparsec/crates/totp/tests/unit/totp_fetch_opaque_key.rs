// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{protocol::anonymous_cmds, test_register_send_hook};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::anonymous_cmds_factory;
use crate::{totp_fetch_opaque_key, TotpFetchOpaqueKeyError};

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let cmds = anonymous_cmds_factory(env);

    let expected_opaque_key = SecretKey::generate();
    let user_id = UserID::default();
    let opaque_key_id = TOTPOpaqueKeyID::default();
    let one_time_password = "123456";

    test_register_send_hook(&env.discriminant_dir, {
        let expected_opaque_key = expected_opaque_key.clone();
        move |req: anonymous_cmds::latest::totp_fetch_opaque_key::Req| {
            p_assert_eq!(req.user_id, user_id);
            p_assert_eq!(req.opaque_key_id, opaque_key_id);
            p_assert_eq!(req.one_time_password, one_time_password);
            anonymous_cmds::latest::totp_fetch_opaque_key::Rep::Ok {
                opaque_key: expected_opaque_key,
            }
        }
    });

    let result =
        totp_fetch_opaque_key(&cmds, user_id, opaque_key_id, one_time_password.to_string())
            .await
            .unwrap();
    p_assert_eq!(result, expected_opaque_key);
}

#[parsec_test(testbed = "minimal")]
async fn invalid_one_time_password(env: &TestbedEnv) {
    let cmds = anonymous_cmds_factory(env);

    test_register_send_hook(
        &env.discriminant_dir,
        move |_req: anonymous_cmds::latest::totp_fetch_opaque_key::Req| {
            anonymous_cmds::latest::totp_fetch_opaque_key::Rep::InvalidOneTimePassword
        },
    );

    let outcome = totp_fetch_opaque_key(
        &cmds,
        UserID::default(),
        TOTPOpaqueKeyID::default(),
        "000000".to_string(),
    )
    .await;
    p_assert_matches!(
        outcome,
        Err(TotpFetchOpaqueKeyError::InvalidOneTimePassword)
    );
}

#[parsec_test(testbed = "minimal")]
async fn throttled(env: &TestbedEnv) {
    let cmds = anonymous_cmds_factory(env);

    let wait_until: DateTime = "2030-01-01T00:00:00Z".parse().unwrap();

    test_register_send_hook(&env.discriminant_dir, {
        move |_req: anonymous_cmds::latest::totp_fetch_opaque_key::Req| {
            anonymous_cmds::latest::totp_fetch_opaque_key::Rep::Throttled { wait_until }
        }
    });

    let outcome = totp_fetch_opaque_key(
        &cmds,
        UserID::default(),
        TOTPOpaqueKeyID::default(),
        "000000".to_string(),
    )
    .await;
    p_assert_matches!(
        outcome,
        Err(TotpFetchOpaqueKeyError::Throttled { wait_until: wu }) if wu == wait_until
    );
}

#[parsec_test(testbed = "minimal")]
async fn offline(env: &TestbedEnv) {
    let cmds = anonymous_cmds_factory(env);

    let outcome = totp_fetch_opaque_key(
        &cmds,
        UserID::default(),
        TOTPOpaqueKeyID::default(),
        "123456".to_string(),
    )
    .await;
    p_assert_matches!(outcome, Err(TotpFetchOpaqueKeyError::Offline(_)));
}
