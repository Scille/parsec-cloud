// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;

use super::authenticated_cmds;

// Request

pub fn req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "ping"
    //   ping: "ping"
    let raw = hex!("82a3636d64a470696e67a470696e67a470696e67");

    let req = authenticated_cmds::ping::Req {
        ping: "ping".to_owned(),
    };

    let expected = authenticated_cmds::AnyCmdReq::Ping(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::Ping(req2) = data else {
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
    //   pong: "pong"
    //   status: "ok"
    let raw = hex!("82a4706f6e67a4706f6e67a6737461747573a26f6b");

    let expected = authenticated_cmds::ping::Rep::Ok {
        pong: "pong".to_owned(),
    };

    let data = authenticated_cmds::ping::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::ping::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
