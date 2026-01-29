// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_fixtures::prelude::*;

use super::mocked_openbao_new_token_entity_id_and_mail;
use crate::{OpenBaoCmds, OpenBaoSignError, OpenBaoVerifyError};

#[parsec_test(testbed = "empty", with_server)]
async fn ok(env: &TestbedEnv) {
    let client = reqwest::Client::new();

    // Generate new entity ID for each test run is important here:
    // since we create a signing key in the server, the outcome would
    // be different between the first run of the test and the subsequent ones
    // if the testbed is not restarted between them!

    let (philip_openbao_token, philip_openbao_entity_id, philip_email) =
        mocked_openbao_new_token_entity_id_and_mail();
    let (mike_openbao_token, mike_openbao_entity_id, mike_email) =
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
    let mike_cmds = OpenBaoCmds::new(
        client,
        // Testbed server exposes a fake OpenBao server here
        env.server_addr
            .to_http_url(Some("testbed/mock/openbao/"))
            .to_string(),
        "secret".to_string(),
        "transit".to_string(),
        mike_openbao_entity_id.clone(),
        mike_openbao_token,
    );

    // cspell: disable-next-line
    let p1 = b"<Philip Je sais ou tu t'cache!>";
    // cspell: disable-next-line
    let p2 = b"<Salaaaaud!>";
    // First sign: signing key has to be created first!
    let s1 = mike_cmds.sign(p1).await.unwrap();
    // Subsequent sign: existing signing key is reused.
    let s1bis = mike_cmds.sign(p1).await.unwrap();

    p_assert_eq!(s1, s1bis);

    p_assert_matches!(
        philip_cmds
            .verify(&mike_openbao_entity_id, &s1, p1, Some(&mike_email))
            .await,
        Ok(())
    );

    p_assert_matches!(
        philip_cmds
            .verify(&mike_openbao_entity_id, &s1, p1, None)
            .await,
        Ok(())
    );

    p_assert_matches!(
        philip_cmds
            .verify(
                &mike_openbao_entity_id,
                &s1,
                p2, // Bad payload
                Some(&mike_email),
            )
            .await,
        Err(OpenBaoVerifyError::BadSignature)
    );

    p_assert_matches!(
        philip_cmds
            .verify(
                &philip_openbao_entity_id, // Bad entity ID
                &s1,
                p1,
                Some(&mike_email),
            )
            .await,
        Err(OpenBaoVerifyError::BadSignature)
    );

    p_assert_matches!(
        philip_cmds
            .verify(
                &mike_openbao_entity_id,
                &s1,
                p1,
                Some(&philip_email), // Bad email
            )
            .await,
        Err(OpenBaoVerifyError::UnexpectedAuthor {
            expected,
            got,
        })
        if expected == philip_email.to_string() && got == mike_email.to_string()
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
        cmds.sign(b"<payload>").await,
        Err(err @ OpenBaoSignError::BadURL(_))
        if err.to_string() == "Invalid OpenBao server URL: URL should not be a cannot-be-a-base"
    );

    p_assert_matches!(
        cmds.verify("792d2291-8d9b-4da0-b14a-6003d0ace897", "<signature>", b"<payload>", Some(&"mike@example.invalid".parse().unwrap())).await,
        Err(err @ OpenBaoVerifyError::BadURL(_))
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
        cmds.sign(b"<payload>").await,
        Err(OpenBaoSignError::NoServerResponse(_))
    );

    p_assert_matches!(
        cmds.verify(
            "792d2291-8d9b-4da0-b14a-6003d0ace897",
            "<signature>",
            b"<payload>",
            Some(&"mike@example.invalid".parse().unwrap())
        )
        .await,
        Err(OpenBaoVerifyError::NoServerResponse(_))
    );
}
