// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_protocol::{
    authenticated_cmds::v3 as authenticated_cmds, invited_cmds::v3 as invited_cmds,
};
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

#[parsec_test]
fn serde_invite_2b_greeter_send_nonce_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_2b_greeter_send_nonce"
    //   greeter_nonce: hex!("666f6f626172")
    //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    let raw = hex!(
        "83a3636d64bc696e766974655f32625f677265657465725f73656e645f6e6f6e6365ad6772"
        "65657465725f6e6f6e6365c406666f6f626172a5746f6b656ed802d864b93ded264aae9ae5"
        "83fd3d40c45a"
    );

    let req = authenticated_cmds::invite_2b_greeter_send_nonce::Req {
        token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
        greeter_nonce: b"foobar".as_ref().into(),
    };

    let expected = authenticated_cmds::AnyCmdReq::Invite2bGreeterSendNonce(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = if let authenticated_cmds::AnyCmdReq::Invite2bGreeterSendNonce(data) = data {
        data.dump().unwrap()
    } else {
        unreachable!()
    };

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
#[case::ok(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   claimer_nonce: hex!("666f6f626172")
    //   status: "ok"
    &hex!(
        "82ad636c61696d65725f6e6f6e6365c406666f6f626172a6737461747573a26f6b"
    )[..],
    authenticated_cmds::invite_2b_greeter_send_nonce::Rep::Ok {
        claimer_nonce: b"foobar".as_ref().into(),
    }
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    &hex!(
        "81a6737461747573a96e6f745f666f756e64"
    )[..],
    authenticated_cmds::invite_2b_greeter_send_nonce::Rep::NotFound
)]
#[case::already_deleted(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "already_deleted"
    &hex!(
        "81a6737461747573af616c72656164795f64656c65746564"
    )[..],
    authenticated_cmds::invite_2b_greeter_send_nonce::Rep::AlreadyDeleted
)]
#[case::invalid_state(
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "invalid_state"
        &hex!(
            "81a6737461747573ad696e76616c69645f7374617465"
        )[..],
        authenticated_cmds::invite_2b_greeter_send_nonce::Rep::InvalidState
)]
fn serde_invite_2b_greeter_send_nonce_rep(
    #[case] raw: &[u8],
    #[case] expected: authenticated_cmds::invite_2b_greeter_send_nonce::Rep,
) {
    let data = authenticated_cmds::invite_2b_greeter_send_nonce::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::invite_2b_greeter_send_nonce::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_invite_3a_greeter_wait_peer_trust_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_3a_greeter_wait_peer_trust"
    //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    let raw = hex!(
        "82a3636d64d921696e766974655f33615f677265657465725f776169745f706565725f7472"
        "757374a5746f6b656ed802d864b93ded264aae9ae583fd3d40c45a"
    );

    let req = authenticated_cmds::invite_3a_greeter_wait_peer_trust::Req {
        token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
    };

    let expected = authenticated_cmds::AnyCmdReq::Invite3aGreeterWaitPeerTrust(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = if let authenticated_cmds::AnyCmdReq::Invite3aGreeterWaitPeerTrust(data) = data {
        data.dump().unwrap()
    } else {
        unreachable!()
    };

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
#[case::ok(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    &hex!(
        "81a6737461747573a26f6b"
    )[..],
    authenticated_cmds::invite_3a_greeter_wait_peer_trust::Rep::Ok
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    &hex!(
        "81a6737461747573a96e6f745f666f756e64"
    )[..],
    authenticated_cmds::invite_3a_greeter_wait_peer_trust::Rep::NotFound
)]
#[case::already_deleted(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "already_deleted"
    &hex!(
        "81a6737461747573af616c72656164795f64656c65746564"
    )[..],
    authenticated_cmds::invite_3a_greeter_wait_peer_trust::Rep::AlreadyDeleted
)]
#[case::invalid_state(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "invalid_state"
    &hex!(
        "81a6737461747573ad696e76616c69645f7374617465"
    )[..],
    authenticated_cmds::invite_3a_greeter_wait_peer_trust::Rep::InvalidState
)]
fn serde_invite_3a_greeter_wait_peer_trust_rep(
    #[case] raw: &[u8],
    #[case] expected: authenticated_cmds::invite_3a_greeter_wait_peer_trust::Rep,
) {
    let data = authenticated_cmds::invite_3a_greeter_wait_peer_trust::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::invite_3a_greeter_wait_peer_trust::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_invite_3b_claimer_wait_peer_trust_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_3b_claimer_wait_peer_trust"
    let raw = hex!(
        "81a3636d64d921696e766974655f33625f636c61696d65725f776169745f706565725f7472"
        "757374"
    );

    let req = invited_cmds::invite_3b_claimer_wait_peer_trust::Req;

    let expected = invited_cmds::AnyCmdReq::Invite3bClaimerWaitPeerTrust(req);

    let data = invited_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = if let invited_cmds::AnyCmdReq::Invite3bClaimerWaitPeerTrust(data) = data {
        data.dump().unwrap()
    } else {
        unreachable!()
    };

    let data2 = invited_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
#[case::ok(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    &hex!(
        "81a6737461747573a26f6b"
    )[..],
    invited_cmds::invite_3b_claimer_wait_peer_trust::Rep::Ok
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    &hex!(
        "81a6737461747573a96e6f745f666f756e64"
    )[..],
    invited_cmds::invite_3b_claimer_wait_peer_trust::Rep::NotFound
)]
#[case::invalid_state(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "invalid_state"
    &hex!(
        "81a6737461747573ad696e76616c69645f7374617465"
    )[..],
    invited_cmds::invite_3b_claimer_wait_peer_trust::Rep::InvalidState
)]
fn serde_invite_3b_claimer_wait_peer_trust_rep(
    #[case] raw: &[u8],
    #[case] expected: invited_cmds::invite_3b_claimer_wait_peer_trust::Rep,
) {
    let data = invited_cmds::invite_3b_claimer_wait_peer_trust::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = invited_cmds::invite_3b_claimer_wait_peer_trust::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_invite_3b_greeter_signify_trust_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_3b_greeter_signify_trust"
    //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    let raw = hex!(
        "82a3636d64bf696e766974655f33625f677265657465725f7369676e6966795f7472757374"
        "a5746f6b656ed802d864b93ded264aae9ae583fd3d40c45a"
    );

    let req = authenticated_cmds::invite_3b_greeter_signify_trust::Req {
        token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
    };

    let expected = authenticated_cmds::AnyCmdReq::Invite3bGreeterSignifyTrust(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = if let authenticated_cmds::AnyCmdReq::Invite3bGreeterSignifyTrust(data) = data {
        data.dump().unwrap()
    } else {
        unreachable!()
    };

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
#[case::ok(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    &hex!(
        "81a6737461747573a26f6b"
    )[..],
    authenticated_cmds::invite_3b_greeter_signify_trust::Rep::Ok
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    &hex!(
        "81a6737461747573a96e6f745f666f756e64"
    )[..],
    authenticated_cmds::invite_3b_greeter_signify_trust::Rep::NotFound
)]
#[case::already_deleted(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "already_deleted"
    &hex!(
        "81a6737461747573af616c72656164795f64656c65746564"
    )[..],
    authenticated_cmds::invite_3b_greeter_signify_trust::Rep::AlreadyDeleted
)]
#[case::invalid_state(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "invalid_state"
    &hex!(
        "81a6737461747573ad696e76616c69645f7374617465"
    )[..],
    authenticated_cmds::invite_3b_greeter_signify_trust::Rep::InvalidState
)]
fn serde_invite_3b_greeter_signify_trust_rep(
    #[case] raw: &[u8],
    #[case] expected: authenticated_cmds::invite_3b_greeter_signify_trust::Rep,
) {
    let data = authenticated_cmds::invite_3b_greeter_signify_trust::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::invite_3b_greeter_signify_trust::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_invite_4_claimer_communicate_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_4_claimer_communicate"
    //   payload: hex!("666f6f626172")
    let raw = hex!(
        "82a3636d64bc696e766974655f345f636c61696d65725f636f6d6d756e6963617465a77061"
        "796c6f6164c406666f6f626172"
    );

    let req = invited_cmds::invite_4_claimer_communicate::Req {
        payload: b"foobar".as_ref().into(),
    };

    let expected = invited_cmds::AnyCmdReq::Invite4ClaimerCommunicate(req);

    let data = invited_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = if let invited_cmds::AnyCmdReq::Invite4ClaimerCommunicate(data) = data {
        data.dump().unwrap()
    } else {
        unreachable!()
    };

    let data2 = invited_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
#[case::ok(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   payload: hex!("666f6f626172")
    //   status: "ok"
    &hex!(
        "82a77061796c6f6164c406666f6f626172a6737461747573a26f6b"
    )[..],
    invited_cmds::invite_4_claimer_communicate::Rep::Ok {
        payload: b"foobar".as_ref().into(),
    }
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    &hex!(
        "81a6737461747573a96e6f745f666f756e64"
    )[..],
    invited_cmds::invite_4_claimer_communicate::Rep::NotFound
)]
#[case::invalid_state(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "invalid_state"
    &hex!(
        "81a6737461747573ad696e76616c69645f7374617465"
    )[..],
    invited_cmds::invite_4_claimer_communicate::Rep::InvalidState
)]
fn serde_invite_4_claimer_communicate_rep(
    #[case] raw: &[u8],
    #[case] expected: invited_cmds::invite_4_claimer_communicate::Rep,
) {
    let data = invited_cmds::invite_4_claimer_communicate::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = invited_cmds::invite_4_claimer_communicate::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}
