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
    //   cmd: "invite_new_device"
    //   send_email: true
    let raw = hex!(
        "82a3636d64b1696e766974655f6e65775f646576696365aa73656e645f656d61696cc3"
    );

    let expected = authenticated_cmds::AnyCmdReq::InviteNewDevice(authenticated_cmds::invite_new_device::Req {
        send_email: true,
    });

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::InviteNewDevice(data2) = data else {
        unreachable!()
    };
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
    let err = authenticated_cmds::invite_new_device::Rep::load(&raw).unwrap_err();
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
    let expected = authenticated_cmds::invite_new_device::Rep::Ok {
        token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
        email_sent: authenticated_cmds::invite_new_device::InvitationEmailSentStatus::Success,
    };

    rep_helper(&raw, expected);
}

fn rep_helper(raw: &[u8], expected: authenticated_cmds::invite_new_device::Rep) {
    let data = authenticated_cmds::invite_new_device::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::invite_new_device::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
