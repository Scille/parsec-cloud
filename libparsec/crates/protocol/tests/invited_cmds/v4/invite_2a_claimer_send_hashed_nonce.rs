// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::invited_cmds;

// Request

pub fn req() {
    // Generated from Python implementation (Parsec v2.10.0+dev)
    // Content:
    //   claimer_hashed_nonce: hex!("e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6")
    //   cmd: "invite_2a_claimer_send_hashed_nonce"
    let raw = hex!(
        "82b4636c61696d65725f6861736865645f6e6f6e6365c420e37ce3b00a1f15b3de62029972"
        "345420b76313a885c6ccc6e3b5547857b3ecc6a3636d64d923696e766974655f32615f636c"
        "61696d65725f73656e645f6861736865645f6e6f6e6365"
    );

    let req = invited_cmds::invite_2a_claimer_send_hashed_nonce::Req {
        claimer_hashed_nonce: HashDigest::from(hex!(
            "e37ce3b00a1f15b3de62029972345420b76313a885c6ccc6e3b5547857b3ecc6"
        )),
    };

    let expected = invited_cmds::AnyCmdReq::Invite2aClaimerSendHashedNonce(req);

    let data = invited_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let invited_cmds::AnyCmdReq::Invite2aClaimerSendHashedNonce(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = invited_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   greeter_nonce: hex!("666f6f626172")
    //   status: "ok"
    let raw = hex!("82ad677265657465725f6e6f6e6365c406666f6f626172a6737461747573a26f6b");

    let expected = invited_cmds::invite_2a_claimer_send_hashed_nonce::Rep::Ok {
        greeter_nonce: Bytes::from_static(b"foobar"),
    };

    let data = invited_cmds::invite_2a_claimer_send_hashed_nonce::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = invited_cmds::invite_2a_claimer_send_hashed_nonce::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_enrollment_wrong_state() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "enrollment_wrong_state"
    let raw = hex!("81a6737461747573b6656e726f6c6c6d656e745f77726f6e675f7374617465");

    let expected = invited_cmds::invite_2a_claimer_send_hashed_nonce::Rep::EnrollmentWrongState;

    let data = invited_cmds::invite_2a_claimer_send_hashed_nonce::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = invited_cmds::invite_2a_claimer_send_hashed_nonce::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
