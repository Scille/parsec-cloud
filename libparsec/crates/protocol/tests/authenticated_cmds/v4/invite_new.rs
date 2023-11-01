// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use super::authenticated_cmds;

use libparsec_tests_lite::{hex, p_assert_eq};
use libparsec_types::InvitationToken;

// Request

pub fn req() {
    let raw_expected = [
        (
            // Generated from Python implementation (Parsec v2.6.0+dev)
            // Content:
            //   type: "USER"
            //   claimer_email: "alice@dev1"
            //   cmd: "invite_new"
            //   send_email: true
            &hex!(
                "84ad636c61696d65725f656d61696caa616c6963654064657631a3636d64aa696e76697465"
                "5f6e6577aa73656e645f656d61696cc3a474797065a455534552"
            )[..],
            authenticated_cmds::AnyCmdReq::InviteNew(authenticated_cmds::invite_new::Req(
                authenticated_cmds::invite_new::UserOrDevice::User {
                    claimer_email: "alice@dev1".to_owned(),
                    send_email: true,
                },
            )),
        ),
        (
            // Generated from Python implementation (Parsec v2.6.0+dev)
            // Content:
            //   type: "DEVICE"
            //   cmd: "invite_new"
            //   send_email: true
            &hex!(
                "83a3636d64aa696e766974655f6e6577aa73656e645f656d61696cc3a474797065a6444556"
                "494345"
            )[..],
            authenticated_cmds::AnyCmdReq::InviteNew(authenticated_cmds::invite_new::Req(
                authenticated_cmds::invite_new::UserOrDevice::Device { send_email: true },
            )),
        ),
    ];

    for (raw, expected) in raw_expected {
        let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();

        p_assert_eq!(data, expected);

        // Also test serialization round trip
        let authenticated_cmds::AnyCmdReq::InviteNew(data2) = data else {
            unreachable!()
        };
        let raw2 = data2.dump().unwrap();

        let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

        p_assert_eq!(data2, expected);
    }
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
    let err = authenticated_cmds::invite_new::Rep::load(&raw).unwrap_err();
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
    let expected = authenticated_cmds::invite_new::Rep::Ok {
        token: InvitationToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
        email_sent: authenticated_cmds::invite_new::InvitationEmailSentStatus::Success,
    };

    rep_helper(&raw, expected);
}

pub fn rep_not_allowed() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_allowed"
    let raw = hex!("81a6737461747573ab6e6f745f616c6c6f776564");
    let expected = authenticated_cmds::invite_new::Rep::NotAllowed;
    rep_helper(&raw, expected);
}

pub fn rep_already_member() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "already_member"
    let raw = hex!("81a6737461747573ae616c72656164795f6d656d626572");
    let expected = authenticated_cmds::invite_new::Rep::AlreadyMember;
    rep_helper(&raw, expected);
}

pub fn rep_not_available() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_available"
    let raw = hex!("81a6737461747573ad6e6f745f617661696c61626c65");
    let expected = authenticated_cmds::invite_new::Rep::NotAvailable;
    rep_helper(&raw, expected)
}

fn rep_helper(raw: &[u8], expected: authenticated_cmds::invite_new::Rep) {
    let data = authenticated_cmds::invite_new::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::invite_new::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
