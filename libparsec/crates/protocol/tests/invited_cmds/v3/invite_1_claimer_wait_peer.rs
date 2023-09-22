// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::invited_cmds;

// Request

pub fn req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   claimer_public_key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
    //   cmd: "invite_1_claimer_wait_peer"
    let raw = hex!(
        "82b2636c61696d65725f7075626c69635f6b6579c4206507907d33bae6b5980b32fa03f3eb"
        "ac56141b126e44f352ea46c5f22cd5ac57a3636d64ba696e766974655f315f636c61696d65"
        "725f776169745f70656572"
    );

    let req = invited_cmds::invite_1_claimer_wait_peer::Req {
        claimer_public_key: PublicKey::from(hex!(
            "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
        )),
    };

    let expected = invited_cmds::AnyCmdReq::Invite1ClaimerWaitPeer(req);

    let data = invited_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let invited_cmds::AnyCmdReq::Invite1ClaimerWaitPeer(req2) = data else {
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
    //   greeter_public_key: hex!("6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57")
    //   status: "ok"
    let raw = hex!(
        "82b2677265657465725f7075626c69635f6b6579c4206507907d33bae6b5980b32fa03f3eb"
        "ac56141b126e44f352ea46c5f22cd5ac57a6737461747573a26f6b"
    );

    let expected = invited_cmds::invite_1_claimer_wait_peer::Rep::Ok {
        greeter_public_key: PublicKey::from(hex!(
            "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
        )),
    };
}

pub fn rep_already_deleted() {
    // Generated from Rust implementation (Parsec v2.15.0+dev)
    // Content:
    //   status: "already_deleted"
    let raw = hex!("81a6737461747573af616c72656164795f64656c65746564");

    let expected = invited_cmds::invite_1_claimer_wait_peer::Rep::AlreadyDeleted;

    let data = invited_cmds::invite_1_claimer_wait_peer::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = invited_cmds::invite_1_claimer_wait_peer::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_not_found() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    let raw = hex!("81a6737461747573a96e6f745f666f756e64");

    let expected = invited_cmds::invite_1_claimer_wait_peer::Rep::NotFound;

    let data = invited_cmds::invite_1_claimer_wait_peer::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = invited_cmds::invite_1_claimer_wait_peer::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_invalid_state() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "invalid_state"
    let raw = hex!("81a6737461747573ad696e76616c69645f7374617465");

    let expected = invited_cmds::invite_1_claimer_wait_peer::Rep::InvalidState;

    let data = invited_cmds::invite_1_claimer_wait_peer::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = invited_cmds::invite_1_claimer_wait_peer::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
