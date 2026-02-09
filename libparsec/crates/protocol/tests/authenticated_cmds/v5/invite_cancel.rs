// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use super::authenticated_cmds;

use libparsec_tests_lite::{hex, p_assert_eq};
use libparsec_types::AccessToken;

// Request

pub fn req() {
    // Generated from Rust implementation (Parsec v3.0.0-b.6+dev 2024-03-29)
    // Content:
    //   cmd: "invite_cancel"
    //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    let raw = hex!(
        "82a3636d64ad696e766974655f63616e63656ca5746f6b656ec410d864b93ded264aae9ae5"
        "83fd3d40c45a"
    );

    let req = authenticated_cmds::invite_cancel::Req {
        token: AccessToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
    };

    let expected = authenticated_cmds::AnyCmdReq::InviteCancel(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::InviteCancel(req2) = data else {
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
    //   status: "ok"
    let raw = hex!("81a6737461747573a26f6b");
    let expected = authenticated_cmds::invite_cancel::Rep::Ok;
    rep_helper(&raw, expected);
}

pub fn rep_invitation_not_found() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "invitation_not_found"
    let raw = hex!("81a6737461747573b4696e7669746174696f6e5f6e6f745f666f756e64");
    let expected = authenticated_cmds::invite_cancel::Rep::InvitationNotFound;
    rep_helper(&raw, expected);
}

pub fn rep_author_not_allowed() {
    // Generated from Parsec 3.2.5-a.0+dev
    // Content:
    //   status: 'author_not_allowed'
    let raw: &[u8] = hex!("81a6737461747573b2617574686f725f6e6f745f616c6c6f776564").as_ref();
    let expected = authenticated_cmds::invite_cancel::Rep::AuthorNotAllowed;
    rep_helper(raw, expected);
}

pub fn rep_invitation_already_cancelled() {
    // Generated from Parsec 3.2.5-a.0+dev
    // Content:
    //   status: 'invitation_already_cancelled'
    let raw: &[u8] = hex!(
    "81a6737461747573bc696e7669746174696f6e5f616c72656164795f63616e63656c6c"
    "6564"
    )
    .as_ref();
    let expected = authenticated_cmds::invite_cancel::Rep::InvitationAlreadyCancelled;
    rep_helper(raw, expected);
}

pub fn rep_invitation_completed() {
    // Generated from Parsec 3.2.5-a.0+dev
    // Content:
    //   status: 'invitation_completed'
    let raw: &[u8] = hex!("81a6737461747573b4696e7669746174696f6e5f636f6d706c65746564").as_ref();
    let expected = authenticated_cmds::invite_cancel::Rep::InvitationCompleted;
    rep_helper(raw, expected);
}

fn rep_helper(raw: &[u8], expected: authenticated_cmds::invite_cancel::Rep) {
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_cmds::invite_cancel::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::invite_cancel::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
