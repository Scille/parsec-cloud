// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use libparsec_client_connection::{
    test_register_low_level_send_hook, test_register_low_level_send_hook_default,
    test_register_low_level_send_hook_multicall, test_register_send_hook, AnonymousCmds,
    AuthenticatedCmds, Bytes, ConnectionError, HeaderMap, InvitedCmds, ProxyConfig, ResponseMock,
    SSEConnectionError, SSEResponseOrMissedEvents, StatusCode,
};
use libparsec_platform_async::stream::StreamExt;
use libparsec_protocol::{
    anonymous_cmds::latest as anonymous_cmds, authenticated_cmds::latest as authenticated_cmds,
    invited_cmds::latest as invited_cmds,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

// TODO: test handling of different errors
// This can be easily done with the testbed's send_hook callback

#[parsec_test(testbed = "minimal", with_server)]
async fn authenticated(env: &TestbedEnv) {
    let alice = env.local_device("alice@dev1");
    let cmds = AuthenticatedCmds::new(&env.discriminant_dir, alice.clone(), ProxyConfig::default())
        .unwrap();

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

    // Bad request: invalid signature

    let bad_alice = {
        let mut bad_alice = (*alice).clone();
        bad_alice.signing_key = SigningKey::generate();
        Arc::new(bad_alice)
    };
    let cmds =
        AuthenticatedCmds::new(&env.discriminant_dir, bad_alice, ProxyConfig::default()).unwrap();
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

#[cfg(not(target_arch = "wasm32"))]
#[parsec_test(testbed = "coolorg", with_server)]
async fn authenticated_sse(env: &TestbedEnv) {
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
            authenticated_cmds::events_listen::APIEvent::ServerConfig { active_users_limit: ActiveUsersLimit::NoLimit, user_profile_outsider_allowed: true }
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

    drop(sse);

    // Bad request: invalid signature

    let bad_alice = {
        let mut bad_alice = (*alice).clone();
        bad_alice.signing_key = SigningKey::generate();
        Arc::new(bad_alice)
    };
    let cmds_bad_alice =
        AuthenticatedCmds::new(&env.discriminant_dir, bad_alice, ProxyConfig::default()).unwrap();
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

#[parsec_test(testbed = "minimal", with_server)]
async fn invited(env: &TestbedEnv) {
    // Create an invitation

    let alice = env.local_device("alice@dev1");
    let cmds = AuthenticatedCmds::new(&env.discriminant_dir, alice.clone(), ProxyConfig::default())
        .unwrap();
    let rep = cmds
        .send(authenticated_cmds::invite_new_device::Req{send_email: false})
        .await;
    let invitation_token = match rep.unwrap() {
        authenticated_cmds::invite_new_device::Rep::Ok { token, .. } => token,
        _ => panic!("fooo"),
    };

    // Good request

    let addr = BackendInvitationAddr::new(
        env.server_addr.clone(),
        env.organization_id.to_owned(),
        InvitationType::User,
        invitation_token,
    );
    let cmds = InvitedCmds::new(&env.discriminant_dir, addr, ProxyConfig::default()).unwrap();

    let rep = cmds
        .send(invited_cmds::ping::Req {
            ping: "foo".to_owned(),
        })
        .await;
    p_assert_eq!(
        rep.unwrap(),
        invited_cmds::ping::Rep::Ok {
            pong: "foo".to_owned()
        }
    );

    // Bad request: invalid invitation token

    let bad_addr = BackendInvitationAddr::new(
        env.server_addr.clone(),
        env.organization_id.to_owned(),
        InvitationType::User,
        InvitationToken::default(),
    );
    let cmds = InvitedCmds::new(&env.discriminant_dir, bad_addr, ProxyConfig::default()).unwrap();

    let rep = cmds
        .send(invited_cmds::ping::Req {
            ping: "foo".to_owned(),
        })
        .await;
    assert!(
        matches!(rep, Err(ConnectionError::InvitationNotFound)),
        r#"expected `InvitationNotFound`, but got {rep:?}"#
    );
}

#[parsec_test(testbed = "minimal", with_server)]
async fn anonymous(env: &TestbedEnv) {
    let addr = BackendAnonymousAddr::BackendPkiEnrollmentAddr(BackendPkiEnrollmentAddr::new(
        env.server_addr.clone(),
        env.organization_id.to_owned(),
    ));
    let cmds = AnonymousCmds::new(&env.discriminant_dir, addr, ProxyConfig::default()).unwrap();

    // Good request

    let rep = cmds
        .send(anonymous_cmds::pki_enrollment_info::Req {
            enrollment_id: EnrollmentID::default(),
        })
        .await;
    p_assert_eq!(
        rep.unwrap(),
        anonymous_cmds::pki_enrollment_info::Rep::EnrollmentNotFound
    );
}

#[parsec_test(testbed = "empty")]
async fn low_level_send_hook(env: &TestbedEnv) {
    test_register_low_level_send_hook(&env.discriminant_dir, |_request_builder| async {
        Ok(ResponseMock::Mocked((
            StatusCode::IM_A_TEAPOT,
            HeaderMap::new(),
            Bytes::new(),
        )))
    });

    let addr = BackendAnonymousAddr::BackendPkiEnrollmentAddr(BackendPkiEnrollmentAddr::new(
        env.server_addr.clone(),
        env.organization_id.to_owned(),
    ));
    let cmds = AnonymousCmds::new(&env.discriminant_dir, addr, ProxyConfig::default()).unwrap();

    let rep = cmds
        .send(anonymous_cmds::pki_enrollment_info::Req {
            enrollment_id: EnrollmentID::default(),
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
        .send(anonymous_cmds::pki_enrollment_info::Req {
            enrollment_id: EnrollmentID::default(),
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
            .send(anonymous_cmds::pki_enrollment_info::Req {
                enrollment_id: EnrollmentID::default(),
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
        .send(anonymous_cmds::pki_enrollment_info::Req {
            enrollment_id: EnrollmentID::default(),
        })
        .await;
    assert!(
        matches!(rep, Err(ConnectionError::NoResponse(None))),
        r#"expected a `NoResponse`, but got {rep:?}"#
    );
}

#[parsec_test(testbed = "empty")]
async fn high_level_send_hook(env: &TestbedEnv) {
    test_register_send_hook(
        &env.discriminant_dir,
        |_req: anonymous_cmds::pki_enrollment_info::Req| {
            anonymous_cmds::pki_enrollment_info::Rep::EnrollmentNotFound
        },
    );

    let addr = BackendAnonymousAddr::BackendPkiEnrollmentAddr(BackendPkiEnrollmentAddr::new(
        env.server_addr.clone(),
        env.organization_id.to_owned(),
    ));
    let cmds = AnonymousCmds::new(&env.discriminant_dir, addr, ProxyConfig::default()).unwrap();

    let rep = cmds
        .send(anonymous_cmds::pki_enrollment_info::Req {
            enrollment_id: EnrollmentID::default(),
        })
        .await;
    assert!(
        matches!(
            rep,
            Ok(anonymous_cmds::pki_enrollment_info::Rep::EnrollmentNotFound)
        ),
        r#"expected a `NotFound`, but got {rep:?}"#
    );

    // Hook should have been reset to the default now

    let rep = cmds
        .send(anonymous_cmds::pki_enrollment_info::Req {
            enrollment_id: EnrollmentID::default(),
        })
        .await;
    assert!(
        matches!(rep, Err(ConnectionError::NoResponse(None))),
        r#"expected a `NoResponse`, but got {rep:?}"#
    );
}
