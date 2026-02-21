// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::{protocol::authenticated_cmds, test_register_send_hook};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::utils::alice_cmds_factory;
use crate::{totp_create_opaque_key, TotpCreateOpaqueKeyError};

#[parsec_test(testbed = "minimal")]
async fn ok(env: &TestbedEnv) {
    let client = alice_cmds_factory(env);

    let expected_opaque_key_id = TOTPOpaqueKeyID::default();
    let expected_opaque_key = SecretKey::generate();

    test_register_send_hook(&env.discriminant_dir, {
        let expected_opaque_key = expected_opaque_key.clone();
        move |_req: authenticated_cmds::latest::totp_create_opaque_key::Req| {
            authenticated_cmds::latest::totp_create_opaque_key::Rep::Ok {
                opaque_key_id: expected_opaque_key_id,
                opaque_key: expected_opaque_key,
            }
        }
    });

    let (opaque_key_id, opaque_key) = totp_create_opaque_key(&client).await.unwrap();
    p_assert_eq!(opaque_key_id, expected_opaque_key_id);
    p_assert_eq!(opaque_key, expected_opaque_key);
}

#[parsec_test(testbed = "minimal")]
async fn offline(env: &TestbedEnv) {
    let client = alice_cmds_factory(env);

    let outcome = totp_create_opaque_key(&client).await;
    p_assert_matches!(outcome, Err(TotpCreateOpaqueKeyError::Offline(_)));
}
