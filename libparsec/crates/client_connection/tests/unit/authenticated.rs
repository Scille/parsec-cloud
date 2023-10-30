// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use crate::{
    test_register_low_level_send_hook, AuthenticatedCmds, Bytes, ConnectionError, HeaderMap,
    HeaderValue, ProxyConfig, ResponseMock, SSEConnectionError, SSEResponseOrMissedEvents,
    StatusCode,
};
use libparsec_platform_async::stream::StreamExt;
use libparsec_protocol::authenticated_cmds::latest as authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

// TODO: test handling of different errors
// This can be easily done with the testbed's send_hook callback

#[parsec_test(testbed = "minimal")]
async fn rpc_ok_mocked(env: &TestbedEnv) {
    rpc_ok(env, true).await
}

#[parsec_test(testbed = "minimal", with_server)]
async fn rpc_ok_with_server(env: &TestbedEnv) {
    rpc_ok(env, false).await
}

async fn rpc_ok(env: &TestbedEnv, mocked: bool) {
    let alice = env.local_device("alice@dev1");
    let cmds = AuthenticatedCmds::new(&env.discriminant_dir, alice.clone(), ProxyConfig::default())
        .unwrap();

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
            p_assert_eq!(
                headers.get("Authorization"),
                Some(&HeaderValue::from_static("PARSEC-SIGN-ED25519"))
            );
            // Base64 of "alice@dev1"
            p_assert_eq!(
                headers.get("Author"),
                // cspell:disable-next-line
                Some(&HeaderValue::from_static("YWxpY2VAZGV2MQ=="))
            );
            assert!(headers.get("Signature").is_some());

            let body = request.body().unwrap().as_bytes().unwrap();
            let request = authenticated_cmds::AnyCmdReq::load(body).unwrap();
            p_assert_matches!(request, authenticated_cmds::AnyCmdReq::Ping(authenticated_cmds::ping::Req { ping }) if ping == "foo");

            Ok(ResponseMock::Mocked((
                StatusCode::OK,
                HeaderMap::new(),
                authenticated_cmds::ping::Rep::Ok {
                    pong: "foo".to_owned(),
                }
                .dump()
                .unwrap()
                .into(),
            )))
        });
    }

    // Good request

    let rep = cmds
        .send(authenticated_cmds::ping::Req {
            ping: "foo".to_owned(),
        })
        .await;
    p_assert_eq!(
        rep.unwrap(),
        authenticated_cmds::ping::Rep::Ok {
            pong: "foo".to_owned()
        }
    );
}

#[parsec_test(testbed = "minimal")]
async fn rpc_unauthorized_mocked(env: &TestbedEnv) {
    rpc_unauthorized(env, true).await
}

#[parsec_test(testbed = "minimal", with_server)]
async fn rpc_unauthorized_with_server(env: &TestbedEnv) {
    rpc_unauthorized(env, false).await
}

async fn rpc_unauthorized(env: &TestbedEnv, mocked: bool) {
    let alice = env.local_device("alice@dev1");
    let bad_alice = {
        let mut bad_alice = (*alice).clone();
        bad_alice.signing_key = SigningKey::generate();
        Arc::new(bad_alice)
    };
    let cmds =
        AuthenticatedCmds::new(&env.discriminant_dir, bad_alice, ProxyConfig::default()).unwrap();

    // Bad request: invalid signature

    if mocked {
        test_register_low_level_send_hook(&env.discriminant_dir, |_request_builder| async {
            Ok(ResponseMock::Mocked((
                StatusCode::UNAUTHORIZED,
                HeaderMap::new(),
                Bytes::new(),
            )))
        });
    }

    let rep = cmds
        .send(authenticated_cmds::ping::Req {
            ping: "foo".to_owned(),
        })
        .await;
    assert!(
        matches!(
            rep,
            Err(ConnectionError::InvalidResponseStatus(
                reqwest::StatusCode::UNAUTHORIZED
            ))
        ),
        r#"expected `InvalidResponseStatus` with code 401, but got {rep:?}"#
    );
}

// TODO: SSE not implemented in web yet
// TODO: testbed send hook doesn't support SSE yet
#[cfg(not(target_arch = "wasm32"))]
#[parsec_test(testbed = "coolorg", with_server)]
async fn sse_ok_with_server(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let bob = env.local_device("bob@dev1");
    let cmds_alice =
        AuthenticatedCmds::new(&env.discriminant_dir, alice.clone(), ProxyConfig::default())
            .unwrap();
    let cmds_bob =
        AuthenticatedCmds::new(&env.discriminant_dir, bob.clone(), ProxyConfig::default()).unwrap();

    let send_ping = |msg: &str| {
        let msg = msg.to_owned();
        async {
            let rep = cmds_bob
                .send(authenticated_cmds::ping::Req { ping: msg })
                .await;
            assert!(matches!(
                rep.unwrap(),
                authenticated_cmds::ping::Rep::Ok { .. }
            ));
        }
    };

    // Request sent before sse started, will be ignored
    send_ping("too soon").await;

    // Start the sse...
    let mut sse = cmds_alice
        .start_sse::<authenticated_cmds::events_listen::Req>(None)
        .await
        .unwrap();

    // Now event are dispatched to the SSE client...
    send_ping("good 1").await;

    // ...but no matter what, the first event the client receive is always organization config
    p_assert_eq!(
        sse.next().await.unwrap().unwrap().message,
        SSEResponseOrMissedEvents::Response(authenticated_cmds::events_listen::Rep::Ok(
            authenticated_cmds::events_listen::APIEvent::ServerConfig {
                active_users_limit: ActiveUsersLimit::NoLimit,
                user_profile_outsider_allowed: true
            }
        ))
    );

    // Now the actual event can be received
    p_assert_eq!(
        sse.next().await.unwrap().unwrap().message,
        SSEResponseOrMissedEvents::Response(authenticated_cmds::events_listen::Rep::Ok(
            authenticated_cmds::events_listen::APIEvent::Pinged {
                ping: "good 1".to_owned()
            }
        ))
    );

    // Also try to enqueue multiple events
    send_ping("good 2").await;
    send_ping("good 3").await;
    send_ping("good 4").await;
    p_assert_eq!(
        sse.next().await.unwrap().unwrap().message,
        SSEResponseOrMissedEvents::Response(authenticated_cmds::events_listen::Rep::Ok(
            authenticated_cmds::events_listen::APIEvent::Pinged {
                ping: "good 2".to_owned()
            }
        ))
    );
    p_assert_eq!(
        sse.next().await.unwrap().unwrap().message,
        SSEResponseOrMissedEvents::Response(authenticated_cmds::events_listen::Rep::Ok(
            authenticated_cmds::events_listen::APIEvent::Pinged {
                ping: "good 3".to_owned()
            }
        ))
    );
    p_assert_eq!(
        sse.next().await.unwrap().unwrap().message,
        SSEResponseOrMissedEvents::Response(authenticated_cmds::events_listen::Rep::Ok(
            authenticated_cmds::events_listen::APIEvent::Pinged {
                ping: "good 4".to_owned()
            }
        ))
    );
}

// TODO: SSE not implemented in web yet
// TODO: testbed send hook doesn't support SSE yet
#[cfg(not(target_arch = "wasm32"))]
#[parsec_test(testbed = "minimal", with_server)]
async fn sse_unauthorized_with_server(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let bad_alice = {
        let mut bad_alice = (*alice).clone();
        bad_alice.signing_key = SigningKey::generate();
        Arc::new(bad_alice)
    };
    let cmds_bad_alice =
        AuthenticatedCmds::new(&env.discriminant_dir, bad_alice, ProxyConfig::default()).unwrap();

    // Bad request: invalid signature

    let res = cmds_bad_alice
        .start_sse::<authenticated_cmds::events_listen::Req>(None)
        .await;
    assert!(
        matches!(
            res,
            Err(SSEConnectionError::InvalidStatusCode(
                reqwest::StatusCode::UNAUTHORIZED
            ))
        ),
        r#"expected `InvalidResponseStatus` with code 401, but got {res:?}"#
    );
}
