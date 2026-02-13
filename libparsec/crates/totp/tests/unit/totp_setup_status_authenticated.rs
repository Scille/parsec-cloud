// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{protocol::authenticated_cmds, test_register_send_hook};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::alice_cmds_factory;
use crate::{totp_setup_status_authenticated, TOTPSetupStatus, TotpSetupStatusAuthenticatedError};

#[parsec_test(testbed = "minimal")]
async fn ok_stalled(env: &TestbedEnv) {
    let client = alice_cmds_factory(env);

    let expected_secret = Bytes::from_static(b"S3cr3t");

    test_register_send_hook(&env.discriminant_dir, {
        let expected_secret = expected_secret.clone();
        move |_req: authenticated_cmds::latest::totp_setup_get_secret::Req| {
            authenticated_cmds::latest::totp_setup_get_secret::Rep::Ok {
                totp_secret: expected_secret,
            }
        }
    });

    let result = totp_setup_status_authenticated(&client).await.unwrap();
    p_assert_eq!(
        result,
        TOTPSetupStatus::Unconfirmed {
            // cspell: disable-next-line
            base32_totp_secret: "KMZWG4RTOQ".to_string(),
        }
    );
}

#[parsec_test(testbed = "minimal")]
async fn ok_already_setup(env: &TestbedEnv) {
    let client = alice_cmds_factory(env);

    test_register_send_hook(
        &env.discriminant_dir,
        move |_req: authenticated_cmds::latest::totp_setup_get_secret::Req| {
            authenticated_cmds::latest::totp_setup_get_secret::Rep::AlreadySetup
        },
    );

    let result = totp_setup_status_authenticated(&client).await.unwrap();
    p_assert_eq!(result, TOTPSetupStatus::Confirmed);
}

#[parsec_test(testbed = "minimal")]
async fn offline(env: &TestbedEnv) {
    let client = alice_cmds_factory(env);

    let outcome = totp_setup_status_authenticated(&client).await;
    p_assert_matches!(outcome, Err(TotpSetupStatusAuthenticatedError::Offline(_)));
}
