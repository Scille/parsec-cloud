// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::{hex, p_assert_eq};
use libparsec_types::{InvitationToken, PublicKey};

use super::authenticated_cmds;

// Request

pub fn req() {
    // Generated from Python implementation (Parsec v3.0.0-b.6+dev 2024-03-29)
    // Content:
    //   cmd: "invite_1_greeter_wait_peer"
    //   greeter_public_key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
    //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    let raw = hex!(
        "83a3636d64ba696e766974655f315f677265657465725f776169745f70656572a5746f"
        "6b656ec410d864b93ded264aae9ae583fd3d40c45ab2677265657465725f7075626c69"
        "635f6b6579c4206507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f2"
        "2cd5ac57"
    );

    let req = authenticated_cmds::invite_1_greeter_wait_peer::Req {
        token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
        greeter_public_key: PublicKey::from(hex!(
            "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
        )),
    };

    let expected = authenticated_cmds::AnyCmdReq::Invite1GreeterWaitPeer(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::Invite1GreeterWaitPeer(req2) = data else {
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
    //   claimer_public_key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
    //   status: "ok"
    let raw = hex!(
        "82b2636c61696d65725f7075626c69635f6b6579c4206507907d33bae6b5980b32fa03"
        "f3ebac56141b126e44f352ea46c5f22cd5ac57a6737461747573a26f6b"
    );
    let expected = authenticated_cmds::invite_1_greeter_wait_peer::Rep::Ok {
        claimer_public_key: PublicKey::from(hex!(
            "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
        )),
    };
    rep_helper(&raw, expected);
}

pub fn rep_invitation_not_found() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "invitation_not_found"
    let raw = hex!("81a6737461747573b4696e7669746174696f6e5f6e6f745f666f756e64");
    let expected = authenticated_cmds::invite_1_greeter_wait_peer::Rep::InvitationNotFound;
    rep_helper(&raw, expected);
}

pub fn rep_invitation_deleted() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "invitation_deleted"
    let raw = hex!("81a6737461747573b2696e7669746174696f6e5f64656c65746564");
    let expected = authenticated_cmds::invite_1_greeter_wait_peer::Rep::InvitationDeleted;
    rep_helper(&raw, expected);
}

fn rep_helper(raw: &[u8], expected: authenticated_cmds::invite_1_greeter_wait_peer::Rep) {
    let data = authenticated_cmds::invite_1_greeter_wait_peer::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::invite_1_greeter_wait_peer::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
