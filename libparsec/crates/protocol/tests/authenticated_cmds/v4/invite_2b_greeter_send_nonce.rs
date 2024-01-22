// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::{hex, p_assert_eq};
use libparsec_types::InvitationToken;

use super::authenticated_cmds;

// Request

pub fn req() {
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

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::Invite2bGreeterSendNonce(req2) = data else {
        unreachable!()
    };
    let raw2 = req2.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   claimer_nonce: hex!("666f6f626172")
    //   status: "ok"
    let raw = hex!("82ad636c61696d65725f6e6f6e6365c406666f6f626172a6737461747573a26f6b");
    let expected = authenticated_cmds::invite_2b_greeter_send_nonce::Rep::Ok {
        claimer_nonce: b"foobar".as_ref().into(),
    };

    rep_helper(&raw, expected);
}

pub fn rep_invitation_not_found() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "invitation_not_found"
    let raw = hex!("81a6737461747573b4696e7669746174696f6e5f6e6f745f666f756e64");
    let expected = authenticated_cmds::invite_2b_greeter_send_nonce::Rep::InvitationNotFound;

    rep_helper(&raw, expected);
}

pub fn rep_invitation_deleted() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "invitation_deleted"
    let raw = hex!("81a6737461747573b2696e7669746174696f6e5f64656c65746564");
    let expected = authenticated_cmds::invite_2b_greeter_send_nonce::Rep::InvitationDeleted;

    rep_helper(&raw, expected);
}

pub fn rep_enrollment_wrong_state() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "enrollment_wrong_state"
    let raw = hex!("81a6737461747573b6656e726f6c6c6d656e745f77726f6e675f7374617465");
    let expected = authenticated_cmds::invite_2b_greeter_send_nonce::Rep::EnrollmentWrongState;

    rep_helper(&raw, expected);
}

fn rep_helper(raw: &[u8], expected: authenticated_cmds::invite_2b_greeter_send_nonce::Rep) {
    let data = authenticated_cmds::invite_2b_greeter_send_nonce::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::invite_2b_greeter_send_nonce::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
