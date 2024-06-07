// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use crate::{
    test_register_low_level_send_hook, AnonymousCmds, ConnectionError, HeaderMap, HeaderValue,
    ProxyConfig, ResponseMock, StatusCode,
};
use libparsec_protocol::anonymous_cmds::latest as anonymous_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

// TODO: test handling of different errors
// This can be easily done with the testbed's send_hook callback

#[parsec_test(testbed = "minimal")]
async fn ok_mocked(env: &TestbedEnv) {
    ok(env, true).await
}

#[parsec_test(testbed = "minimal", with_server)]
async fn ok_with_server(env: &TestbedEnv) {
    ok(env, false).await
}

async fn ok(env: &TestbedEnv, mocked: bool) {
    let addr = ParsecAnonymousAddr::ParsecPkiEnrollmentAddr(ParsecPkiEnrollmentAddr::new(
        env.server_addr.clone(),
        env.organization_id.to_owned(),
    ));
    let cmds = AnonymousCmds::new(&env.discriminant_dir, addr, ProxyConfig::default()).unwrap();

    // Good request

    if mocked {
        test_register_low_level_send_hook(&env.discriminant_dir, |request_builder| async {
            let request = request_builder.build().unwrap();
            let headers = request.headers();
            println!("headers: {:?}", headers);
            p_assert_eq!(
                headers.get("Content-Type"),
                Some(&HeaderValue::from_static("application/msgpack"))
            );
            // Cannot check `User-Agent` here given reqwest adds it in a later step
            // assert!(headers.get("User-Agent").unwrap().to_str().unwrap().starts_with("Parsec-Client/"));
            assert!(headers.get("Authorization").is_none());

            let body = request.body().unwrap().as_bytes().unwrap();
            let request = anonymous_cmds::AnyCmdReq::load(body).unwrap();
            p_assert_matches!(request, anonymous_cmds::AnyCmdReq::Ping(anonymous_cmds::ping::Req { ping }) if ping == "foo");

            Ok(ResponseMock::Mocked((
                StatusCode::OK,
                HeaderMap::new(),
                anonymous_cmds::ping::Rep::Ok {
                    pong: "foo".to_owned(),
                }
                .dump()
                .unwrap()
                .into(),
            )))
        });
    }

    let rep = cmds
        .send(anonymous_cmds::ping::Req {
            ping: "foo".to_owned(),
        })
        .await;
    p_assert_eq!(
        rep.unwrap(),
        anonymous_cmds::ping::Rep::Ok {
            pong: "foo".to_owned()
        }
    );
}

// Must use a macro to generate the parametrized tests here given `test_register_low_level_send_hook`
// only accept a static closure (i.e. `fn`, not `FnMut`).
macro_rules! register_http_hook {
    ($test_name: ident, $response_status_code: expr) => {
        #[parsec_test(testbed = "minimal")]
        async fn $test_name(env: &TestbedEnv) {
            let addr = ParsecAnonymousAddr::ParsecPkiEnrollmentAddr(ParsecPkiEnrollmentAddr::new(
                env.server_addr.clone(),
                env.organization_id.to_owned(),
            ));
            let cmds =
                AnonymousCmds::new(&env.discriminant_dir, addr, ProxyConfig::default()).unwrap();

            test_register_low_level_send_hook(
                &env.discriminant_dir,
                move |_request_builder| async {
                    Ok(ResponseMock::Mocked((
                        $response_status_code,
                        HeaderMap::new(),
                        "".into(),
                    )))
                },
            );

            let rep = cmds
                .send(anonymous_cmds::ping::Req {
                    ping: "foo".to_owned(),
                })
                .await;
            p_assert_eq!(rep.unwrap_err(), ConnectionError::NoResponse(None));
        }
    };
}

register_http_hook!(no_response_http_codes_502, StatusCode::BAD_GATEWAY);
register_http_hook!(no_response_http_codes_503, StatusCode::SERVICE_UNAVAILABLE);
register_http_hook!(no_response_http_codes_504, StatusCode::GATEWAY_TIMEOUT);
