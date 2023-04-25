// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client_connection::{
    test_register_low_level_send_hook, test_register_low_level_send_hook_default,
    test_register_low_level_send_hook_multicall, test_register_send_hook, AnonymousCmds,
    AuthenticatedCmds, Bytes, CommandError, HeaderMap, InvitedCmds, ProxyConfig, ResponseMock,
    StatusCode,
};
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
    let alice = env.local_device("alice@dev1".parse().unwrap());
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
            Err(CommandError::InvalidResponseStatus(
                reqwest::StatusCode::UNAUTHORIZED
            ))
        ),
        r#"expected `InvalidResponseStatus` with code 401, but got {rep:?}"#
    );
}

#[parsec_test(testbed = "minimal", with_server)]
async fn invited(env: &TestbedEnv) {
    // Create an invitation

    let alice = env.local_device("alice@dev1".parse().unwrap());
    let cmds = AuthenticatedCmds::new(&env.discriminant_dir, alice.clone(), ProxyConfig::default())
        .unwrap();
    let rep = cmds
        .send(authenticated_cmds::invite_new::Req(
            authenticated_cmds::invite_new::UserOrDevice::Device { send_email: false },
        ))
        .await;
    let invitation_token = match rep.unwrap() {
        authenticated_cmds::invite_new::Rep::Ok { token, .. } => token,
        _ => panic!("fooo"),
    };

    // Good request

    let addr = BackendInvitationAddr::new(
        env.organization_addr.clone(),
        env.organization_addr.organization_id().to_owned(),
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
        env.organization_addr.clone(),
        env.organization_addr.organization_id().to_owned(),
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
        matches!(rep, Err(CommandError::InvitationNotFound)),
        r#"expected `InvitationNotFound`, but got {rep:?}"#
    );
}

#[parsec_test(testbed = "minimal", with_server)]
async fn anonymous(env: &TestbedEnv) {
    let addr = BackendAnonymousAddr::BackendPkiEnrollmentAddr(BackendPkiEnrollmentAddr::new(
        env.organization_addr.clone(),
        env.organization_addr.organization_id().to_owned(),
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
        anonymous_cmds::pki_enrollment_info::Rep::NotFound {
            reason: Some("".to_owned())
        }
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
        env.organization_addr.clone(),
        env.organization_addr.organization_id().to_owned(),
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
            Err(CommandError::InvalidResponseStatus(StatusCode::IM_A_TEAPOT)),
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
        matches!(rep, Err(CommandError::NoResponse(None))),
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
                Err(CommandError::InvalidResponseStatus(StatusCode::IM_A_TEAPOT)),
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
        matches!(rep, Err(CommandError::NoResponse(None))),
        r#"expected a `NoResponse`, but got {rep:?}"#
    );
}

#[parsec_test(testbed = "empty")]
async fn high_level_send_hook(env: &TestbedEnv) {
    test_register_send_hook(
        &env.discriminant_dir,
        |_req: anonymous_cmds::pki_enrollment_info::Req| async {
            anonymous_cmds::pki_enrollment_info::Rep::NotFound { reason: None }
        },
    );

    let addr = BackendAnonymousAddr::BackendPkiEnrollmentAddr(BackendPkiEnrollmentAddr::new(
        env.organization_addr.clone(),
        env.organization_addr.organization_id().to_owned(),
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
            Ok(anonymous_cmds::pki_enrollment_info::Rep::NotFound { reason: None })
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
        matches!(rep, Err(CommandError::NoResponse(None))),
        r#"expected a `NoResponse`, but got {rep:?}"#
    );
}
