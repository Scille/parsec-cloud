// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use super::authenticated_cmds;

use libparsec_tests_lite::{hex, p_assert_eq};
use libparsec_types::AccessToken;
use libparsec_types::UserID;

// Request

pub fn req() {
    // Generated from Parsec 3.1.1-a.0+dev
    // Content:
    //   cmd: 'invite_new_shamir_recovery'
    //   claimer_user_id: ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4a)
    //   send_email: True
    let raw: &[u8] = hex!(
    "83a3636d64ba696e766974655f6e65775f7368616d69725f7265636f76657279af636c"
    "61696d65725f757365725f6964d802109b68ba5cdf428ea0017fc6bcc04d4aaa73656e"
    "645f656d61696cc3"
    )
    .as_ref();

    let req = authenticated_cmds::invite_new_shamir_recovery::Req {
        send_email: true,
        claimer_user_id: UserID::from_hex("109b68ba5cdf428ea0017fc6bcc04d4a").unwrap(),
    };
    println!("***expected: {:?}", req.dump().unwrap());

    let expected = authenticated_cmds::AnyCmdReq::InviteNewShamirRecovery(req);
    let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::InviteNewShamirRecovery(data2) = data else {
        unreachable!()
    };
    let raw2 = data2.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Rust implementation (Parsec v3.0.0-b.6+dev 2024-03-29)
    // Content:
    //   status: "ok"
    //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    //
    // Note that raw data does not contain "email_sent".
    // This was valid behavior in api v2 but is no longer valid from v3 onwards.
    // The corresponding expected values used here are therefore not important
    // since loading raw data should fail.
    //
    let raw = hex!("82a6737461747573a26f6ba5746f6b656ec410d864b93ded264aae9ae583fd3d40c45a");
    let err = authenticated_cmds::invite_new_shamir_recovery::Rep::load(&raw).unwrap_err();
    p_assert_eq!(err.to_string(), "missing field `email_sent`");

    // Generated from Python implementation (Parsec v3.0.0-b.6+dev 2024-03-29)
    // Content:
    //   email_sent: "SUCCESS"
    //   status: "ok"
    //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    let raw = hex!(
        "83aa656d61696c5f73656e74a753554343455353a6737461747573a26f6ba5746f6b656ec4"
        "10d864b93ded264aae9ae583fd3d40c45a"
    );
    let expected = authenticated_cmds::invite_new_shamir_recovery::Rep::Ok {
        token: AccessToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
        email_sent:
            authenticated_cmds::invite_new_shamir_recovery::InvitationEmailSentStatus::Success,
    };

    rep_helper(&raw, expected);
}

pub fn rep_author_not_allowed() {
    // Generated from Parsec 3.1.1-a.0+dev
    // Content:
    //   status: 'author_not_allowed'
    let raw: &[u8] = hex!("81a6737461747573b2617574686f725f6e6f745f616c6c6f776564").as_ref();

    let expected = authenticated_cmds::invite_new_shamir_recovery::Rep::AuthorNotAllowed;
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

pub fn rep_user_not_found() {
    // Generated from Parsec 3.1.1-a.0+dev
    // Content:
    //   status: 'user_not_found'
    let raw: &[u8] = hex!("81a6737461747573ae757365725f6e6f745f666f756e64").as_ref();

    let expected = authenticated_cmds::invite_new_shamir_recovery::Rep::UserNotFound;
    println!("***expected: {:?}", expected.dump().unwrap());

    rep_helper(raw, expected);
}

fn rep_helper(raw: &[u8], expected: authenticated_cmds::invite_new_shamir_recovery::Rep) {
    let data = authenticated_cmds::invite_new_shamir_recovery::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::invite_new_shamir_recovery::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
