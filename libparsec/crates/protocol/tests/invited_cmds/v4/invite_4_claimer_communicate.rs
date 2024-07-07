// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;

use super::invited_cmds;

// Request

pub fn req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_4_claimer_communicate"
    //   payload: hex!("666f6f626172")
    let raw = hex!(
        "82a3636d64bc696e766974655f345f636c61696d65725f636f6d6d756e6963617465a7"
        "7061796c6f6164c406666f6f626172"
    );

    let req = invited_cmds::invite_4_claimer_communicate::Req {
        payload: b"foobar".as_ref().into(),
    };

    let expected = invited_cmds::AnyCmdReq::Invite4ClaimerCommunicate(req);

    let data = invited_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let invited_cmds::AnyCmdReq::Invite4ClaimerCommunicate(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = invited_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   last: false,
    //   payload: hex!("666f6f626172")
    //   status: "ok"
    let raw = hex!("83a6737461747573a26f6ba46c617374c2a77061796c6f6164c406666f6f626172");

    let expected = invited_cmds::invite_4_claimer_communicate::Rep::Ok {
        payload: b"foobar".as_ref().into(),
        last: false,
    };

    let data = invited_cmds::invite_4_claimer_communicate::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = invited_cmds::invite_4_claimer_communicate::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_enrollment_wrong_state() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "enrollment_wrong_state"
    let raw = hex!("81a6737461747573b6656e726f6c6c6d656e745f77726f6e675f7374617465");

    let expected = invited_cmds::invite_4_claimer_communicate::Rep::EnrollmentWrongState;

    let data = invited_cmds::invite_4_claimer_communicate::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = invited_cmds::invite_4_claimer_communicate::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
