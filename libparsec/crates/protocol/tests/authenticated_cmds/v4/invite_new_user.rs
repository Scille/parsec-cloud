// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use super::authenticated_cmds;

use libparsec_tests_lite::{hex, p_assert_eq};
use libparsec_types::InvitationToken;

// Request

pub fn req() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   claimer_email: "alice@dev1"
    //   cmd: "invite_new_user"
    //   send_email: true
    let raw = hex!(
        "83a3636d64af696e766974655f6e65775f75736572ad636c61696d65725f656d61696caa61"
        "6c6963654064657631aa73656e645f656d61696cc3"
    );

    let expected = authenticated_cmds::AnyCmdReq::InviteNewUser(authenticated_cmds::invite_new_user::Req {
        claimer_email: "alice@dev1".to_owned(),
        send_email: true,
    });


    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::InviteNewUser(data2) = data else {
        unreachable!()
    };
    // Also test serialization round trip
    let raw2 = data2.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Rust implementation (Parsec v2.12.1+dev)
    // Content:
    //   status: "ok"
    //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    //
    // Note that raw data does not contain "email_sent".
    // This was valid behavior in api v2 but is no longer valid from v3 onwards.
    // The corresponding expected values used here are therefore not important
    // since loading raw data should fail.
    //
    let raw = hex!("82a6737461747573a26f6ba5746f6b656ed802d864b93ded264aae9ae583fd3d40c45a");
    let err = authenticated_cmds::invite_new_user::Rep::load(&raw).unwrap_err();
    p_assert_eq!(err.to_string(), "missing field `email_sent`");

    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   email_sent: "SUCCESS"
    //   status: "ok"
    //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
    let raw = hex!(
        "83aa656d61696c5f73656e74a753554343455353a6737461747573a26f6ba5746f6b656ed8"
        "02d864b93ded264aae9ae583fd3d40c45a"
    );
    let expected = authenticated_cmds::invite_new_user::Rep::Ok {
        token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
        email_sent: authenticated_cmds::invite_new_user::InvitationEmailSentStatus::Success,
    };

    rep_helper(&raw, expected);
}

pub fn rep_author_not_allowed() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "author_not_allowed"
    let raw = hex!("81a6737461747573b2617574686f725f6e6f745f616c6c6f776564");
    let expected = authenticated_cmds::invite_new_user::Rep::AuthorNotAllowed;
    rep_helper(&raw, expected);
}

pub fn rep_claimer_email_already_enrolled() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "claimer_email_already_enrolled"
    let raw = hex!(
        "81a6737461747573be636c61696d65725f656d61696c5f616c72656164795f656e726f6c6c"
        "6564"
    );
    let expected = authenticated_cmds::invite_new_user::Rep::ClaimerEmailAlreadyEnrolled;
    rep_helper(&raw, expected);
}

fn rep_helper(raw: &[u8], expected: authenticated_cmds::invite_new_user::Rep) {
    let data = authenticated_cmds::invite_new_user::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::invite_new_user::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
