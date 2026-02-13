// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{protocol::anonymous_cmds, test_register_send_hook};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::anonymous_cmds_factory;
use crate::{totp_setup_status_anonymous, TOTPSetupStatus, TotpSetupStatusAnonymousError};

#[parsec_test(testbed = "minimal")]
async fn ok_stalled(env: &TestbedEnv) {
    let cmds = anonymous_cmds_factory(env);
    let user_id = UserID::default();
    let token = AccessToken::default();

    let expected_secret = Bytes::from_static(b"S3cr3t");

    test_register_send_hook(&env.discriminant_dir, {
        let expected_secret = expected_secret.clone();
        move |req: anonymous_cmds::latest::totp_setup_get_secret::Req| {
            p_assert_eq!(req.user_id, user_id);
            p_assert_eq!(req.token, token);
            anonymous_cmds::latest::totp_setup_get_secret::Rep::Ok {
                totp_secret: expected_secret,
            }
        }
    });

    let result = totp_setup_status_anonymous(&cmds, user_id, token)
        .await
        .unwrap();
    p_assert_eq!(
        result,
        TOTPSetupStatus::Unconfirmed {
            base32_totp_secret: data_encoding::BASE32_NOPAD.encode(&expected_secret),
        }
    );
}

#[parsec_test(testbed = "minimal")]
async fn bad_token(env: &TestbedEnv) {
    let cmds = anonymous_cmds_factory(env);

    test_register_send_hook(
        &env.discriminant_dir,
        move |_req: anonymous_cmds::latest::totp_setup_get_secret::Req| {
            anonymous_cmds::latest::totp_setup_get_secret::Rep::BadToken
        },
    );

    let outcome =
        totp_setup_status_anonymous(&cmds, UserID::default(), AccessToken::default()).await;
    p_assert_matches!(outcome, Err(TotpSetupStatusAnonymousError::BadToken));
}

#[parsec_test(testbed = "minimal")]
async fn offline(env: &TestbedEnv) {
    let cmds = anonymous_cmds_factory(env);

    let outcome =
        totp_setup_status_anonymous(&cmds, UserID::default(), AccessToken::default()).await;
    p_assert_matches!(outcome, Err(TotpSetupStatusAnonymousError::Offline(_)));
}
