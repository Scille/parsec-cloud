// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use crate::{
    test_register_low_level_send_hook, AuthenticatedCmds, Bytes, ConnectionError, HeaderMap,
    HeaderName, HeaderValue, ProxyConfig, ResponseMock, SSEResponseOrMissedEvents, StatusCode,
};
use libparsec_platform_async::stream::StreamExt;
use libparsec_protocol::authenticated_cmds::latest as authenticated_cmds;
use libparsec_protocol::tos_cmds::latest as tos_cmds;
use libparsec_protocol::API_LATEST_VERSION;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
mod disconnect_proxy;

#[parsec_test(testbed = "minimal")]
async fn rpc_ok_mocked(env: &TestbedEnv) {
    rpc_ok(env, true).await;
    rpc_ok_tos_family(env, true).await;
}

#[parsec_test(testbed = "minimal", with_server)]
async fn rpc_ok_with_server(env: &TestbedEnv) {
    rpc_ok(env, false).await;
    rpc_ok_tos_family(env, false).await;
}

async fn rpc_ok(env: &TestbedEnv, mocked: bool) {
    let alice = env.local_device("alice@dev1");
    let cmds = AuthenticatedCmds::new(&env.discriminant_dir, alice.clone(), ProxyConfig::default())
        .unwrap();

    if mocked {
        test_register_low_level_send_hook(&env.discriminant_dir, |request_builder| async {
            let request = request_builder.build().unwrap();
            p_assert_eq!(request.url().path(), "/authenticated/OfflineOrg");
            let headers = request.headers();
            println!("headers: {headers:?}");
            p_assert_eq!(
                headers.get("Content-Type"),
                Some(&HeaderValue::from_static("application/msgpack"))
            );
            // Cannot check `User-Agent` here given reqwest adds it in a later step
            // assert!(headers.get("User-Agent").unwrap().to_str().unwrap().starts_with("Parsec-Client/"));
            assert!(headers
                .get("Authorization")
                .unwrap()
                .as_bytes()
                .starts_with(b"Bearer PARSEC-SIGN-ED25519."));

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

async fn rpc_ok_tos_family(env: &TestbedEnv, mocked: bool) {
    let alice = env.local_device("alice@dev1");
    let cmds = AuthenticatedCmds::new(&env.discriminant_dir, alice.clone(), ProxyConfig::default())
        .unwrap();

    if mocked {
        test_register_low_level_send_hook(&env.discriminant_dir, |request_builder| async {
            let request = request_builder.build().unwrap();
            p_assert_eq!(request.url().path(), "/authenticated/OfflineOrg/tos");
            let headers = request.headers();
            println!("headers: {headers:?}");
            p_assert_eq!(
                headers.get("Content-Type"),
                Some(&HeaderValue::from_static("application/msgpack"))
            );
            // Cannot check `User-Agent` here given reqwest adds it in a later step
            // assert!(headers.get("User-Agent").unwrap().to_str().unwrap().starts_with("Parsec-Client/"));
            assert!(headers
                .get("Authorization")
                .unwrap()
                .as_bytes()
                .starts_with(b"Bearer PARSEC-SIGN-ED25519."));

            let body = request.body().unwrap().as_bytes().unwrap();
            let request = tos_cmds::AnyCmdReq::load(body).unwrap();
            p_assert_matches!(request, tos_cmds::AnyCmdReq::TosGet(tos_cmds::tos_get::Req));

            Ok(ResponseMock::Mocked((
                StatusCode::OK,
                HeaderMap::new(),
                tos_cmds::tos_get::Rep::NoTos.dump().unwrap().into(),
            )))
        });
    }

    // Good request

    let rep = cmds.send(tos_cmds::tos_get::Req).await;
    p_assert_matches!(rep.unwrap(), tos_cmds::tos_get::Rep::NoTos);
}

#[parsec_test(testbed = "minimal")]
async fn rpc_unauthorized_mocked(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let cmds =
        AuthenticatedCmds::new(&env.discriminant_dir, alice, ProxyConfig::default()).unwrap();

    // The request is valid, but the server complains nevertheless

    test_register_low_level_send_hook(&env.discriminant_dir, |_request_builder| async {
        Ok(ResponseMock::Mocked((
            StatusCode::UNAUTHORIZED,
            HeaderMap::new(),
            Bytes::new(),
        )))
    });

    let rep = cmds
        .send(authenticated_cmds::ping::Req {
            ping: "foo".to_owned(),
        })
        .await;
    p_assert_matches!(rep, Err(ConnectionError::MissingAuthenticationInfo));
}

#[parsec_test(testbed = "minimal")]
async fn rpc_forbidden_mocked(env: &TestbedEnv) {
    rpc_forbidden(env, true).await
}

#[parsec_test(testbed = "minimal", with_server)]
async fn rpc_forbidden_with_server(env: &TestbedEnv) {
    rpc_forbidden(env, false).await
}

async fn rpc_forbidden(env: &TestbedEnv, mocked: bool) {
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
                StatusCode::FORBIDDEN,
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
    p_assert_matches!(rep, Err(ConnectionError::BadAuthenticationInfo));
}

#[parsec_test(testbed = "minimal")]
async fn sse_ok_mocked(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let cmds = AuthenticatedCmds::new(&env.discriminant_dir, alice.clone(), ProxyConfig::default())
        .unwrap();

    test_register_low_level_send_hook(&env.discriminant_dir, |_request_builder| async {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            HeaderValue::from_static("text/event-stream"),
        );
        Ok(ResponseMock::Mocked((
            StatusCode::OK,
            headers,
            "\
            :keepalive\n\n\
            data:haZzdGF0dXOib2ulZXZlbnSzT1JHQU5JWkFUSU9OX0NPTkZJR7JhY3RpdmVfdXNlcnNfbGltaXTAtXNzZV9rZWVwYWxpdmVfc2Vjb25kcx69dXNlcl9wcm9maWxlX291dHNpZGVyX2FsbG93ZWTD\nid:832ea0c75e0d4ca8aedf123a89b3fcc7\n\n\
            event:missed_events\ndata:\n\n\
            data:g6ZzdGF0dXOib2ulZXZlbnSmUElOR0VEpHBpbmemZ29vZCAx\nid:4fe5b6ddf29f4c159e6002da2132d80f\n\n\
            :keepalive\n\n\
            ".into(),
        )))
    });

    let mut sse = cmds
        .start_sse::<authenticated_cmds::events_listen::Req>(None)
        .await
        .unwrap();

    p_assert_eq!(
        sse.next().await.unwrap().unwrap().message,
        SSEResponseOrMissedEvents::Response(authenticated_cmds::events_listen::Rep::Ok(
            authenticated_cmds::events_listen::APIEvent::OrganizationConfig {
                active_users_limit: ActiveUsersLimit::NoLimit,
                user_profile_outsider_allowed: true,
                sse_keepalive_seconds: Some(30.try_into().unwrap()),
            }
        ))
    );

    p_assert_eq!(
        sse.next().await.unwrap().unwrap().message,
        SSEResponseOrMissedEvents::MissedEvents
    );

    p_assert_eq!(
        sse.next().await.unwrap().unwrap().message,
        SSEResponseOrMissedEvents::Response(authenticated_cmds::events_listen::Rep::Ok(
            authenticated_cmds::events_listen::APIEvent::Pinged {
                ping: "good 1".to_owned()
            }
        ))
    );

    // Finally the connection is closed
    p_assert_matches!(sse.next().await, None);
}

#[parsec_test(testbed = "coolorg", with_server)]
async fn sse_ok_with_server(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let bob = env.local_device("bob@dev1");
    let cmds_alice =
        AuthenticatedCmds::new(&env.discriminant_dir, alice.clone(), ProxyConfig::default())
            .unwrap();
    let cmds_bob =
        AuthenticatedCmds::new(&env.discriminant_dir, bob.clone(), ProxyConfig::default()).unwrap();

    let bob_send_ping = |msg: &str| {
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
    bob_send_ping("too soon").await;

    // Start the sse...
    let mut sse = cmds_alice
        .start_sse::<authenticated_cmds::events_listen::Req>(None)
        .await
        .unwrap();

    // Now event are dispatched to the SSE client...
    bob_send_ping("good 1").await;

    // ...but no matter what, the first event the client receive is always organization config
    p_assert_eq!(
        sse.next().await.unwrap().unwrap().message,
        SSEResponseOrMissedEvents::Response(authenticated_cmds::events_listen::Rep::Ok(
            authenticated_cmds::events_listen::APIEvent::OrganizationConfig {
                active_users_limit: ActiveUsersLimit::NoLimit,
                user_profile_outsider_allowed: true,
                sse_keepalive_seconds: Some(30.try_into().unwrap()),
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
    bob_send_ping("good 2").await;
    bob_send_ping("good 3").await;
    bob_send_ping("good 4").await;
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

#[parsec_test(testbed = "minimal")]
async fn sse_forbidden_mocked(env: &TestbedEnv) {
    sse_forbidden(env, true).await
}

#[parsec_test(testbed = "minimal", with_server)]
async fn sse_forbidden_with_server(env: &TestbedEnv) {
    sse_forbidden(env, false).await
}

async fn sse_forbidden(env: &TestbedEnv, mocked: bool) {
    let alice = env.local_device("alice@dev1");
    let bad_alice = {
        let mut bad_alice = (*alice).clone();
        bad_alice.signing_key = SigningKey::generate();
        Arc::new(bad_alice)
    };
    let cmds_bad_alice =
        AuthenticatedCmds::new(&env.discriminant_dir, bad_alice, ProxyConfig::default()).unwrap();

    if mocked {
        test_register_low_level_send_hook(&env.discriminant_dir, |_request_builder| async {
            Ok(ResponseMock::Mocked((
                StatusCode::FORBIDDEN,
                HeaderMap::new(),
                Bytes::new(),
            )))
        });
    }

    // Bad request: invalid signature

    let rep = cmds_bad_alice
        .start_sse::<authenticated_cmds::events_listen::Req>(None)
        .await;
    p_assert_matches!(rep, Err(ConnectionError::BadAuthenticationInfo));
}

#[parsec_test(testbed = "minimal")]
async fn sse_unauthorized_mocked(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let cmds =
        AuthenticatedCmds::new(&env.discriminant_dir, alice, ProxyConfig::default()).unwrap();

    // The request is valid, but the server complains nevertheless

    test_register_low_level_send_hook(&env.discriminant_dir, |_request_builder| async {
        Ok(ResponseMock::Mocked((
            StatusCode::UNAUTHORIZED,
            HeaderMap::new(),
            Bytes::new(),
        )))
    });

    let rep = cmds
        .start_sse::<authenticated_cmds::events_listen::Req>(None)
        .await;
    p_assert_matches!(rep, Err(ConnectionError::MissingAuthenticationInfo));
}

#[parsec_test(testbed = "minimal")]
async fn sse_event_id_mocked(
    #[values("good", "missing", "empty", "invalid_for_http_header")] kind: &'static str,
    env: &TestbedEnv,
) {
    let alice = env.local_device("alice@dev1");
    let cmds = AuthenticatedCmds::new(&env.discriminant_dir, alice.clone(), ProxyConfig::default())
        .unwrap();

    macro_rules! register_send_hook {
        ($body:literal) => {
            test_register_low_level_send_hook(&env.discriminant_dir, |_request_builder| async {
                let mut headers = HeaderMap::new();
                headers.insert(
                    "Content-Type",
                    HeaderValue::from_static("text/event-stream"),
                );
                Ok(ResponseMock::Mocked((
                    StatusCode::OK,
                    headers,
                    $body.into(),
                )))
            })
        };
    }
    match kind {
        "good" => register_send_hook!("data:g6ZzdGF0dXOib2ulZXZlbnSmUElOR0VEpHBpbmemZ29vZCAx\nid:4fe5b6ddf29f4c159e6002da2132d80f\n\n"),
        "missing" => register_send_hook!("data:g6ZzdGF0dXOib2ulZXZlbnSmUElOR0VEpHBpbmemZ29vZCAx\n\n"),
        "empty" => register_send_hook!("data:g6ZzdGF0dXOib2ulZXZlbnSmUElOR0VEpHBpbmemZ29vZCAx\n\nid:\n"),
        // HTTP header only supports printable ASCII
        "invalid_for_http_header" => register_send_hook!("data:g6ZzdGF0dXOib2ulZXZlbnSmUElOR0VEpHBpbmemZ29vZCAx\nid:你好\n\n"),
        unknown => unreachable!("{}", unknown),
    }

    // 1) Alice starts SSE connection...

    let mut sse = cmds
        .start_sse::<authenticated_cmds::events_listen::Req>(None)
        .await
        .unwrap();

    // 2) ...then receives the first event

    let event = sse.next().await.unwrap().unwrap();
    let expected_event_id = match kind {
        "good" => Some("4fe5b6ddf29f4c159e6002da2132d80f"),
        "missing" => None,
        "empty" => None,
        "invalid_for_http_header" => Some("你好"),
        unknown => unreachable!("{}", unknown),
    };
    p_assert_eq!(event.id.as_deref(), expected_event_id);

    // 3) Alice reconnect and try to use the last event id

    macro_rules! register_send_hook {
        ($last_event_id:expr) => {
            test_register_low_level_send_hook(&env.discriminant_dir, |request_builder| async {
                // Check `last-event-id` is passed to the server
                let request = request_builder.build().unwrap();
                let headers = request.headers();
                println!("headers: {:?}", headers);
                let expected_last_event_id = $last_event_id.map(
                    // Don't use `HeaderValue::from_static` as it is broken with
                    // utf8 (see https://github.com/hyperium/http/issues/519)
                    |raw| HeaderValue::from_str(raw).unwrap()
                );
                p_assert_eq!(
                    headers.get("Last-Event-Id"),
                    expected_last_event_id.as_ref()
                );

                let mut headers = HeaderMap::new();
                headers.insert(
                    "Content-Type",
                    HeaderValue::from_static("text/event-stream"),
                );
                Ok(ResponseMock::Mocked((
                    StatusCode::OK,
                    headers,
                    "data:g6ZzdGF0dXOib2ulZXZlbnSmUElOR0VEpHBpbmemZ29vZCAy\nid:779210f80be24178a0402d7d07657f6f\n\n".into(),
                )))
            })
        };
    }
    match kind {
        "good" => register_send_hook!(Some("4fe5b6ddf29f4c159e6002da2132d80f")),
        "missing" => register_send_hook!(None),
        "empty" => register_send_hook!(None),
        "invalid_for_http_header" => register_send_hook!(Some("你好")),
        unknown => unreachable!("{}", unknown),
    }

    let mut sse = cmds
        .start_sse::<authenticated_cmds::events_listen::Req>(event.id)
        .await
        .unwrap();

    let event = sse.next().await.unwrap().unwrap();
    p_assert_eq!(
        event.id.as_deref(),
        Some("779210f80be24178a0402d7d07657f6f")
    );
}

#[parsec_test(testbed = "minimal")]
async fn sse_retry_mocked(
    #[values(
        "good",
        "good_with_data",
        "empty",
        "not_a_number",
        "bad_deserialization"
    )]
    kind: &'static str,
    env: &TestbedEnv,
) {
    let alice = env.local_device("alice@dev1");
    let cmds = AuthenticatedCmds::new(&env.discriminant_dir, alice.clone(), ProxyConfig::default())
        .unwrap();

    macro_rules! register_send_hook {
        ($body:literal) => {
            test_register_low_level_send_hook(&env.discriminant_dir, |_request_builder| async {
                let mut headers = HeaderMap::new();
                headers.insert(
                    "Content-Type",
                    HeaderValue::from_static("text/event-stream"),
                );
                Ok(ResponseMock::Mocked((
                    StatusCode::OK,
                    headers,
                    $body.into(),
                )))
            })
        };
    }
    // Note a `data` field must always be provided, otherwise the event should simply
    // be ignored according to the SSE spec.
    // (cf. https://html.spec.whatwg.org/multipage/server-sent-events.html#dispatchMessage)
    match kind {
        "good" => register_send_hook!("retry: 1000\ndata:\n\n"),
        "good_with_data" => register_send_hook!(
            "data:g6ZzdGF0dXOib2ulZXZlbnSmUElOR0VEpHBpbmemZ29vZCAx\nretry: 1000\n\n"
        ),
        "empty" => register_send_hook!("retry: \ndata:\n\n"),
        "not_a_number" => register_send_hook!("retry: <dummy>\ndata:\n\n"),
        // "bad_deserialization" in base46
        "bad_deserialization" => {
            // cspell:disable-next-line
            register_send_hook!("data: YmFkX2Rlc2VyaWFsaXphdGlvbg==\nretry: 1000\n\n")
        }
        unknown => unreachable!("{}", unknown),
    }

    // 1) Alice starts SSE connection...

    let mut sse = cmds
        .start_sse::<authenticated_cmds::events_listen::Req>(None)
        .await
        .unwrap();

    // 2) ...then receives the first event

    let event = sse.next().await.unwrap().unwrap();
    match kind {
        "good" => {
            p_assert_matches!(event.message, SSEResponseOrMissedEvents::Empty);
            p_assert_eq!(event.retry, Some(std::time::Duration::from_millis(1000)));
        }
        "good_with_data" => {
            p_assert_matches!(event.message, SSEResponseOrMissedEvents::Response(_));
            p_assert_eq!(event.retry, Some(std::time::Duration::from_millis(1000)));
        }
        "empty" => {
            p_assert_matches!(event.message, SSEResponseOrMissedEvents::Empty);
            p_assert_eq!(event.retry, None);
        }
        "not_a_number" => {
            p_assert_matches!(event.message, SSEResponseOrMissedEvents::Empty);
            p_assert_eq!(event.retry, None);
        }
        "bad_deserialization" => {
            p_assert_matches!(event.message, SSEResponseOrMissedEvents::Empty);
            p_assert_eq!(event.retry, Some(std::time::Duration::from_millis(1000)));
        }
        unknown => unreachable!("{}", unknown),
    }
}

// Uses disconnect proxy server which is not available in web
#[cfg(not(target_arch = "wasm32"))]
#[parsec_test(testbed = "coolorg", with_server)]
async fn sse_last_event_id_with_server(env: &TestbedEnv) {
    let alice_disconnect_proxy = disconnect_proxy::spawn(env.server_addr.clone())
        .await
        .unwrap();
    let alice = env.local_device("alice@dev1");
    let alice_through_disconnect_proxy = {
        let mut alice = alice.clone();
        Arc::make_mut(&mut alice).organization_addr = ParsecOrganizationAddr::new(
            alice_disconnect_proxy.to_parsec_addr(),
            alice.organization_id().clone(),
            alice.root_verify_key().clone(),
        );
        alice
    };
    let bob = env.local_device("bob@dev1");

    let cmds_alice = AuthenticatedCmds::new(
        &env.discriminant_dir,
        alice_through_disconnect_proxy.clone(),
        ProxyConfig::default(),
    )
    .unwrap();
    let cmds_bob =
        AuthenticatedCmds::new(&env.discriminant_dir, bob.clone(), ProxyConfig::default()).unwrap();

    let bob_send_ping = |msg: &'static str| async {
        let rep = cmds_bob
            .send(authenticated_cmds::ping::Req {
                ping: msg.to_string(),
            })
            .await
            .expect("Failed to send ping");
        p_assert_matches!(rep, authenticated_cmds::ping::Rep::Ok { .. });
    };

    // 1) Alice starts SSE connection...

    let mut sse_alice = cmds_alice
        .start_sse::<authenticated_cmds::events_listen::Req>(None)
        .await
        .unwrap();

    // 2) Alice gets its first SSE event

    let first_event = sse_alice.next().await.unwrap().unwrap();
    // First event has no ID given it is a fake event always send on connection
    p_assert_eq!(
        first_event.message,
        SSEResponseOrMissedEvents::Response(authenticated_cmds::events_listen::Rep::Ok(
            authenticated_cmds::events_listen::APIEvent::OrganizationConfig {
                active_users_limit: ActiveUsersLimit::NoLimit,
                user_profile_outsider_allowed: true,
                sse_keepalive_seconds: Some(30.try_into().unwrap()),
            }
        ))
    );
    p_assert_eq!(first_event.id, None);

    // 2) ...then the second

    bob_send_ping("got 1").await;

    let second_event = sse_alice.next().await.unwrap().unwrap();
    p_assert_eq!(
        second_event.message,
        SSEResponseOrMissedEvents::Response(authenticated_cmds::events_listen::Rep::Ok(
            authenticated_cmds::events_listen::APIEvent::Pinged {
                ping: "got 1".to_owned()
            }
        ))
    );
    let last_alice_event_id = second_event.id;
    p_assert_matches!(last_alice_event_id, Some(_));

    // 3) Alice gets disconnected, Bob sends some events

    alice_disconnect_proxy.disconnect().await;
    // Can be `ConnectionError::NoResponse` if the disconnection occurred while
    // transmitting data.
    p_assert_matches!(
        sse_alice.next().await,
        None | Some(Err(ConnectionError::NoResponse(_)))
    );

    bob_send_ping("missed 2").await;
    bob_send_ping("missed 3").await;

    // 4) Alice reconnects and should retrieve the missed events

    let cmds_alice = AuthenticatedCmds::new(
        &env.discriminant_dir,
        alice_through_disconnect_proxy.clone(),
        ProxyConfig::default(),
    )
    .unwrap();
    let mut sse_alice = cmds_alice
        .start_sse::<authenticated_cmds::events_listen::Req>(last_alice_event_id)
        .await
        .unwrap();

    p_assert_eq!(
        sse_alice.next().await.unwrap().unwrap().message,
        SSEResponseOrMissedEvents::Response(authenticated_cmds::events_listen::Rep::Ok(
            authenticated_cmds::events_listen::APIEvent::OrganizationConfig {
                active_users_limit: ActiveUsersLimit::NoLimit,
                user_profile_outsider_allowed: true,
                sse_keepalive_seconds: Some(30.try_into().unwrap()),
            }
        ))
    );

    p_assert_eq!(
        sse_alice.next().await.unwrap().unwrap().message,
        SSEResponseOrMissedEvents::Response(authenticated_cmds::events_listen::Rep::Ok(
            authenticated_cmds::events_listen::APIEvent::Pinged {
                ping: "missed 2".to_owned()
            }
        ))
    );

    p_assert_eq!(
        sse_alice.next().await.unwrap().unwrap().message,
        SSEResponseOrMissedEvents::Response(authenticated_cmds::events_listen::Rep::Ok(
            authenticated_cmds::events_listen::APIEvent::Pinged {
                ping: "missed 3".to_owned()
            }
        ))
    );

    alice_disconnect_proxy.close().await.unwrap();

    // 5) Alice reconnects, this time providing a last event ID unknown to the server

    let unknown_or_too_old_event_id = "062bcb4c74b64431a3967087f9875536".to_string();

    let cmds_alice =
        AuthenticatedCmds::new(&env.discriminant_dir, alice, ProxyConfig::default()).unwrap();
    let mut sse_alice = cmds_alice
        .start_sse::<authenticated_cmds::events_listen::Req>(Some(unknown_or_too_old_event_id))
        .await
        .unwrap();

    p_assert_eq!(
        sse_alice.next().await.unwrap().unwrap().message,
        SSEResponseOrMissedEvents::Response(authenticated_cmds::events_listen::Rep::Ok(
            authenticated_cmds::events_listen::APIEvent::OrganizationConfig {
                active_users_limit: ActiveUsersLimit::NoLimit,
                user_profile_outsider_allowed: true,
                sse_keepalive_seconds: Some(30.try_into().unwrap()),
            }
        ))
    );

    p_assert_eq!(
        sse_alice.next().await.unwrap().unwrap().message,
        SSEResponseOrMissedEvents::MissedEvents
    );
}

// Must use a macro to generate the parametrized tests here given `test_register_low_level_send_hook`
// only accept a static closure (i.e. `fn`, not `FnMut`).
macro_rules! register_rpc_http_hook {
    ($test_name: ident, $response_status_code: expr, $assert_err_cb: expr $(, $header_key:literal : $header_value:expr)* $(,)?) => {
        #[parsec_test(testbed = "minimal")]
        async fn $test_name(env: &TestbedEnv) {
            let alice = env.local_device("alice@dev1");
            let cmds = AuthenticatedCmds::new(&env.discriminant_dir, alice, ProxyConfig::default())
                .unwrap();

            test_register_low_level_send_hook(
                &env.discriminant_dir,
                move |_request_builder| async {
                    let header_map = HeaderMap::from_iter(
                        [
                            $({
                                let header_key: HeaderName = $header_key.try_into().unwrap();
                                let header_value: HeaderValue = $header_value.try_into().unwrap();
                                (header_key, header_value)
                            },)*
                        ]
                    );
                    Ok(ResponseMock::Mocked((
                        $response_status_code,
                        header_map,
                        "".into(),
                    )))
                },
            );

            let err = cmds
                .send(authenticated_cmds::ping::Req {
                    ping: "foo".to_owned(),
                })
                .await
                .unwrap_err();
            ($assert_err_cb)(err)
        }
    };
}
register_rpc_http_hook!(
    rpc_missing_authentication_info_http_codes_401,
    StatusCode::from_u16(401).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::MissingAuthenticationInfo);
    }
);
register_rpc_http_hook!(
    rpc_bad_authentication_info_http_codes_403,
    StatusCode::from_u16(403).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::BadAuthenticationInfo);
    }
);
register_rpc_http_hook!(
    rpc_organization_not_found_http_codes_404,
    StatusCode::from_u16(404).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::OrganizationNotFound);
    }
);
register_rpc_http_hook!(
    rpc_bad_accept_type_http_codes_406,
    StatusCode::from_u16(406).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::BadAcceptType);
    }
);
register_rpc_http_hook!(
    rpc_bad_content_type_http_codes_415,
    StatusCode::from_u16(415).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::BadContent);
    }
);
register_rpc_http_hook!(
    rpc_unsupported_api_version_http_codes_422,
    StatusCode::from_u16(422).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::MissingSupportedApiVersions { api_version } if api_version == *API_LATEST_VERSION );
    }
);
register_rpc_http_hook!(
    rpc_unsupported_api_version_http_codes_422_with_supported_api_versions,
    StatusCode::from_u16(422).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::UnsupportedApiVersion { api_version, supported_api_versions } if api_version == *API_LATEST_VERSION && supported_api_versions == vec![(8,1).into(), (9,0).into()]);
    },
    "Supported-Api-Versions": "8.1;9.0"
);
register_rpc_http_hook!(
    rpc_expired_organization_http_codes_460,
    StatusCode::from_u16(460).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::ExpiredOrganization);
    }
);
register_rpc_http_hook!(
    rpc_revoked_user_http_codes_461,
    StatusCode::from_u16(461).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::RevokedUser);
    }
);
register_rpc_http_hook!(
    rpc_frozen_user_http_codes_462,
    StatusCode::from_u16(462).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::FrozenUser);
    }
);
register_rpc_http_hook!(
    rpc_must_accept_tos_http_codes_463,
    StatusCode::from_u16(463).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::UserMustAcceptTos);
    }
);
register_rpc_http_hook!(
    rpc_must_accept_tos_http_codes_464,
    StatusCode::from_u16(464).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::WebClientNotAllowedByOrganization);
    }
);
register_rpc_http_hook!(
    rpc_authentication_token_expired_http_codes_498,
    StatusCode::from_u16(498).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::AuthenticationTokenExpired);
    }
);
register_rpc_http_hook!(
    rpc_invalid_http_status_499,
    StatusCode::from_u16(499).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::InvalidResponseStatus(status) if status == 499);
    }
);
register_rpc_http_hook!(
    rpc_no_response_http_codes_502,
    StatusCode::BAD_GATEWAY,
    |err| {
        p_assert_matches!(err, ConnectionError::NoResponse(_));
    }
);
register_rpc_http_hook!(
    rpc_no_response_http_codes_503,
    StatusCode::SERVICE_UNAVAILABLE,
    |err| {
        p_assert_matches!(err, ConnectionError::NoResponse(_));
    }
);
register_rpc_http_hook!(
    rpc_no_response_http_codes_504,
    StatusCode::GATEWAY_TIMEOUT,
    |err| {
        p_assert_matches!(err, ConnectionError::NoResponse(_));
    }
);

// Must use a macro to generate the parametrized tests here given `test_register_low_level_send_hook`
// only accept a static closure (i.e. `fn`, not `FnMut`).
macro_rules! register_sse_http_hook {
    ($test_name: ident, $response_status_code: expr, $assert_err_cb: expr $(, $header_key:literal : $header_value:expr)* $(,)?) => {
        #[parsec_test(testbed = "minimal")]
        async fn $test_name(env: &TestbedEnv) {
            let alice = env.local_device("alice@dev1");
            let cmds = AuthenticatedCmds::new(&env.discriminant_dir, alice, ProxyConfig::default())
                .unwrap();

            test_register_low_level_send_hook(
                &env.discriminant_dir,
                move |_request_builder| async {
                    let header_map = HeaderMap::from_iter(
                        [
                            $({
                                let header_key: HeaderName = $header_key.try_into().unwrap();
                                let header_value: HeaderValue = $header_value.try_into().unwrap();
                                (header_key, header_value)
                            },)*
                        ]
                    );
                    Ok(ResponseMock::Mocked((
                        $response_status_code,
                        header_map,
                        "".into(),
                    )))
                },
            );

            let err = cmds
                .start_sse::<authenticated_cmds::events_listen::Req>(None)
                .await
                .unwrap_err();
            ($assert_err_cb)(err)
        }
    };
}
register_sse_http_hook!(
    sse_missing_authentication_info_http_codes_401,
    StatusCode::from_u16(401).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::MissingAuthenticationInfo);
    }
);
register_sse_http_hook!(
    sse_bad_authentication_info_http_codes_403,
    StatusCode::from_u16(403).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::BadAuthenticationInfo);
    }
);
register_sse_http_hook!(
    sse_organization_not_found_http_codes_404,
    StatusCode::from_u16(404).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::OrganizationNotFound);
    }
);
register_sse_http_hook!(
    sse_bad_accept_type_http_codes_406,
    StatusCode::from_u16(406).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::BadAcceptType);
    }
);
register_sse_http_hook!(
    sse_bad_content_type_http_codes_415,
    StatusCode::from_u16(415).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::BadContent);
    }
);
register_sse_http_hook!(
    sse_unsupported_api_version_http_codes_422,
    StatusCode::from_u16(422).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::MissingSupportedApiVersions { api_version } if api_version == *API_LATEST_VERSION );
    }
);
register_rpc_http_hook!(
    sse_unsupported_api_version_http_codes_422_with_supported_api_versions,
    StatusCode::from_u16(422).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::UnsupportedApiVersion { api_version, supported_api_versions } if api_version == *API_LATEST_VERSION && supported_api_versions == vec![(8,1).into(), (9,0).into()]);
    },
    "Supported-Api-Versions": "8.1;9.0"
);
register_sse_http_hook!(
    sse_expired_organization_http_codes_460,
    StatusCode::from_u16(460).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::ExpiredOrganization);
    }
);
register_sse_http_hook!(
    sse_revoked_user_http_codes_461,
    StatusCode::from_u16(461).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::RevokedUser);
    }
);
register_sse_http_hook!(
    sse_frozen_user_http_codes_462,
    StatusCode::from_u16(462).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::FrozenUser);
    }
);
register_sse_http_hook!(
    sse_must_accept_tos_http_codes_463,
    StatusCode::from_u16(463).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::UserMustAcceptTos);
    }
);
register_sse_http_hook!(
    sse_authentication_token_expired_http_codes_498,
    StatusCode::from_u16(498).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::AuthenticationTokenExpired);
    }
);
register_sse_http_hook!(
    sse_invalid_http_status_499,
    StatusCode::from_u16(499).unwrap(),
    |err| {
        p_assert_matches!(err, ConnectionError::InvalidResponseStatus(status) if status == 499);
    }
);
register_sse_http_hook!(
    sse_no_response_http_codes_502,
    StatusCode::BAD_GATEWAY,
    |err| {
        p_assert_matches!(err, ConnectionError::NoResponse(_));
    }
);
register_sse_http_hook!(
    sse_no_response_http_codes_503,
    StatusCode::SERVICE_UNAVAILABLE,
    |err| {
        p_assert_matches!(err, ConnectionError::NoResponse(_));
    }
);
register_sse_http_hook!(
    sse_no_response_http_codes_504,
    StatusCode::GATEWAY_TIMEOUT,
    |err| {
        p_assert_matches!(err, ConnectionError::NoResponse(_));
    }
);
