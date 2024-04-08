// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::{hex, p_assert_eq};
use libparsec_types::{HashDigest, InvitationToken};

use super::authenticated_cmds;

// Request

pub fn req() {
    // Generated from Python implementation (Parsec v3.0.0-b.6+dev 2024-03-29)
    // Content:
    //   cmd: "invite_2a_greeter_get_hashed_nonce"
    //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    let raw = hex!(
        "82a3636d64d922696e766974655f32615f677265657465725f6765745f686173"
        "6865645f6e6f6e6365a5746f6b656ec410d864b93ded264aae9ae583fd3d40c4"
        "5a"
    );

    let req = authenticated_cmds::invite_2a_greeter_get_hashed_nonce::Req {
        token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
    };

    let expected = authenticated_cmds::AnyCmdReq::Invite2aGreeterGetHashedNonce(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::Invite2aGreeterGetHashedNonce(req2) = data else {
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
    //   claimer_hashed_nonce: hex!("e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6")
    //   status: "ok"
    let raw = hex!(
        "82b4636c61696d65725f6861736865645f6e6f6e6365c420e37ce3b00a1f15b3de62029972"
        "345420b76313a885c6ccc6e3b5547857b3ecc6a6737461747573a26f6b"
    );
    let expected = authenticated_cmds::invite_2a_greeter_get_hashed_nonce::Rep::Ok {
        claimer_hashed_nonce: HashDigest::from(hex!(
            "e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6"
        )),
    };

    rep_helper(&raw, expected);
}

pub fn rep_invitation_not_found() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "invitation_not_found"
    let raw = hex!("81a6737461747573b4696e7669746174696f6e5f6e6f745f666f756e64");
    let expected = authenticated_cmds::invite_2a_greeter_get_hashed_nonce::Rep::InvitationNotFound;

    rep_helper(&raw, expected);
}

pub fn rep_invitation_deleted() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "invitation_deleted"
    let raw = hex!("81a6737461747573b2696e7669746174696f6e5f64656c65746564");
    let expected = authenticated_cmds::invite_2a_greeter_get_hashed_nonce::Rep::InvitationDeleted;

    rep_helper(&raw, expected);
}

pub fn rep_enrollment_wrong_state() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "enrollment_wrong_state"
    let raw = hex!("81a6737461747573b6656e726f6c6c6d656e745f77726f6e675f7374617465");
    let expected =
        authenticated_cmds::invite_2a_greeter_get_hashed_nonce::Rep::EnrollmentWrongState;

    rep_helper(&raw, expected);
}

fn rep_helper(raw: &[u8], expected: authenticated_cmds::invite_2a_greeter_get_hashed_nonce::Rep) {
    let data = authenticated_cmds::invite_2a_greeter_get_hashed_nonce::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::invite_2a_greeter_get_hashed_nonce::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
