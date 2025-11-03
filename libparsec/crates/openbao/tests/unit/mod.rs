// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_fixtures::prelude::*;

use crate::{OpenBaoCmds, OpenBaoFetchOpaqueKeyError, OpenBaoUploadOpaqueKeyError};

#[parsec_test(testbed = "empty", with_server)]
async fn ok(env: &TestbedEnv) {
    let client = reqwest::Client::new();
    let cmds = OpenBaoCmds::new(
        client,
        // Testbed server exposes a fake OpenBao server here
        env.server_addr
            .to_http_url(Some("testbed/mock/openbao/"))
            .to_string(),
        "secret/parsec-keys".to_string(),
        "65732d02-bb5f-7ce7-eae4-69067383b61d".to_string(),
        // cspell:disable-next-line
        "s.OyKLFX4EG8aigjz0I61ASvXZ".to_string(),
    );

    p_assert_eq!(
        cmds.openbao_entity_id(),
        "65732d02-bb5f-7ce7-eae4-69067383b61d"
    );

    let (key_path, key) = cmds.upload_opaque_key().await.unwrap();
    let (key_path2, key2) = cmds.upload_opaque_key().await.unwrap();
    assert_ne!(key_path, key_path2);
    assert_ne!(key, key2);

    let re_key = cmds.fetch_opaque_key(&key_path).await.unwrap();
    p_assert_eq!(key, re_key);

    let outcome = cmds.fetch_opaque_key("dummy").await.unwrap_err();
    p_assert_matches!(
        outcome,
        err @ OpenBaoFetchOpaqueKeyError::BadServerResponse(_)
        if err.to_string() == "The OpenBao server returned an unexpected response: Bad status code: 404 Not Found"
    );
}

#[parsec_test()]
async fn cannot_be_a_base_url() {
    let client = reqwest::Client::new();
    let cmds = OpenBaoCmds::new(
        client,
        "mailto:jdoe@example.com".parse().unwrap(),
        "65732d02-bb5f-7ce7-eae4-69067383b61d".to_string(),
        "secret/parsec-keys".to_string(),
        "dummy".to_string(),
    );

    p_assert_matches!(
        cmds.upload_opaque_key().await.unwrap_err(),
        err @ OpenBaoUploadOpaqueKeyError::BadURL(_)
        if err.to_string() == "Invalid OpenBao server URL: URL should not be a cannot-be-a-base"
    );

    p_assert_matches!(
        cmds.fetch_opaque_key("foo").await.unwrap_err(),
        err @ OpenBaoFetchOpaqueKeyError::BadURL(_)
        if err.to_string() == "Invalid OpenBao server URL: URL should not be a cannot-be-a-base"
    );
}
