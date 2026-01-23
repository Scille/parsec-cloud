// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_fixtures::prelude::*;

use super::mocked_openbao_new_token_entity_id_and_mail;
use crate::{OpenBaoCmds, OpenBaoListEntityEmailsError};

#[parsec_test(testbed = "empty", with_server)]
async fn ok(env: &TestbedEnv) {
    let client = reqwest::Client::new();

    // Generate new entity ID for each test run is important here:
    // since we create a signing key in the server, the outcome would
    // be different between the first run of the test and the subsequent ones
    // if the testbed is not restarted between them!

    let (philip_openbao_token, philip_openbao_entity_id, philip_email) =
        mocked_openbao_new_token_entity_id_and_mail();

    let philip_cmds = OpenBaoCmds::new(
        client.clone(),
        // Testbed server exposes a fake OpenBao server here
        env.server_addr
            .to_http_url(Some("testbed/mock/openbao/"))
            .to_string(),
        "secret".to_string(),
        "transit".to_string(),
        philip_openbao_entity_id.clone(),
        philip_openbao_token,
    );

    p_assert_eq!(
        philip_cmds.list_self_emails().await.unwrap(),
        vec![philip_email.to_string()]
    );
}

#[parsec_test()]
async fn cannot_be_a_base_url() {
    let client = reqwest::Client::new();
    let cmds = OpenBaoCmds::new(
        client,
        "mailto:jdoe@example.com".parse().unwrap(),
        "65732d02-bb5f-7ce7-eae4-69067383b61d".to_string(),
        "secret".to_string(),
        "transit".to_string(),
        "dummy".to_string(),
    );

    p_assert_matches!(
        cmds.list_self_emails().await,
        Err(err @ OpenBaoListEntityEmailsError::BadURL(_))
        if err.to_string() == "Invalid OpenBao server URL: URL should not be a cannot-be-a-base"
    );
}

#[parsec_test(testbed = "empty")]
async fn offline(env: &TestbedEnv) {
    let client = reqwest::Client::new();
    let cmds = OpenBaoCmds::new(
        client,
        // Testbed server exposes a fake OpenBao server here
        env.server_addr
            .to_http_url(Some("testbed/mock/openbao/"))
            .to_string(),
        "secret".to_string(),
        "transit".to_string(),
        "65732d02-bb5f-7ce7-eae4-69067383b61d".to_string(),
        // cspell:disable-next-line
        "s.OyKLFX4EG8aigjz0I61ASvXZ".to_string(),
    );

    p_assert_matches!(
        cmds.list_self_emails().await,
        Err(OpenBaoListEntityEmailsError::NoServerResponse(_))
    );
}
