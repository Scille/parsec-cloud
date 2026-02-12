// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use crate::{
    test_register_low_level_send_hook, test_register_low_level_send_hook_default,
    test_register_low_level_send_hook_multicall, test_register_send_hook, AnonymousCmds, Bytes,
    ConnectionError, HeaderMap, ProxyConfig, ResponseMock, StatusCode,
};
use libparsec_protocol::anonymous_cmds::latest as anonymous_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

#[parsec_test(testbed = "empty")]
async fn low_level_send_hook(env: &TestbedEnv) {
    test_register_low_level_send_hook(&env.discriminant_dir, |_request_builder| async {
        Ok(ResponseMock::Mocked((
            StatusCode::IM_A_TEAPOT,
            HeaderMap::new(),
            Bytes::new(),
        )))
    });

    let addr =
        ParsecPkiEnrollmentAddr::new(env.server_addr.clone(), env.organization_id.to_owned());
    let cmds =
        AnonymousCmds::new(&env.discriminant_dir, addr.into(), ProxyConfig::default()).unwrap();

    let rep = cmds
        .send(anonymous_cmds::ping::Req {
            ping: "ping".to_owned(),
        })
        .await;
    assert!(
        matches!(
            rep,
            Err(ConnectionError::InvalidResponseStatus(
                StatusCode::IM_A_TEAPOT
            )),
        ),
        "expected a teapot, but got {rep:?}"
    );

    // Hook should have been reset to the default now

    let rep = cmds
        .send(anonymous_cmds::ping::Req {
            ping: "ping".to_owned(),
        })
        .await;
    assert!(
        matches!(rep, Err(ConnectionError::NoResponse(None))),
        r#"expected a `NoResponse`, but got {rep:?}"#
    );

    // Test the multicall hook

    test_register_low_level_send_hook_multicall(&env.discriminant_dir, |_request_builder| async {
        Ok(ResponseMock::Mocked((
            StatusCode::IM_A_TEAPOT,
            HeaderMap::new(),
            Bytes::new(),
        )))
    });
    for _ in 0..3 {
        let rep = cmds
            .send(anonymous_cmds::ping::Req {
                ping: "ping".to_owned(),
            })
            .await;
        assert!(
            matches!(
                rep,
                Err(ConnectionError::InvalidResponseStatus(
                    StatusCode::IM_A_TEAPOT
                )),
            ),
            "expected a teapot, but got {rep:?}"
        );
    }

    // Finally switch back to default

    test_register_low_level_send_hook_default(&env.discriminant_dir);

    let rep = cmds
        .send(anonymous_cmds::ping::Req {
            ping: "ping".to_owned(),
        })
        .await;
    assert!(
        matches!(rep, Err(ConnectionError::NoResponse(None))),
        r#"expected a `NoResponse`, but got {rep:?}"#
    );
}

#[parsec_test(testbed = "empty")]
async fn high_level_send_hook(env: &TestbedEnv) {
    let pong = "pong".to_owned();
    test_register_send_hook(&env.discriminant_dir, |_req: anonymous_cmds::ping::Req| {
        anonymous_cmds::ping::Rep::Ok { pong }
    });

    let addr =
        ParsecPkiEnrollmentAddr::new(env.server_addr.clone(), env.organization_id.to_owned());
    let cmds =
        AnonymousCmds::new(&env.discriminant_dir, addr.into(), ProxyConfig::default()).unwrap();

    let rep = cmds
        .send(anonymous_cmds::ping::Req {
            ping: "ping".to_owned(),
        })
        .await;
    assert!(
        matches!(rep, Ok(anonymous_cmds::ping::Rep::Ok { .. })),
        r#"expected a `Ok`, but got {rep:?}"#
    );

    // Hook should have been reset to the default now

    let rep = cmds
        .send(anonymous_cmds::ping::Req {
            ping: "ping".to_owned(),
        })
        .await;
    assert!(
        matches!(rep, Err(ConnectionError::NoResponse(None))),
        r#"expected a `NoResponse`, but got {rep:?}"#
    );
}
