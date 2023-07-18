// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use hex_literal::hex;

use libparsec_protocol::{
    authenticated_cmds::v2 as authenticated_cmds, invited_cmds::v2 as invited_cmds,
};
use libparsec_tests_fixtures::prelude::*;

#[parsec_test]
fn serde_authenticated_ping_req() {
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

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = if let authenticated_cmds::AnyCmdReq::Ping(data) = data {
        data.dump().unwrap()
    } else {
        unreachable!()
    };

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_authenticated_ping_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   pong: "pong"
    //   status: "ok"
    let raw = hex!("82a4706f6e67a4706f6e67a6737461747573a26f6b");

    let expected = authenticated_cmds::ping::Rep::Ok {
        pong: "pong".to_owned(),
    };

    let data = authenticated_cmds::ping::Rep::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::ping::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_invited_ping_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "ping"
    //   ping: "ping"
    let raw = hex!("82a3636d64a470696e67a470696e67a470696e67");

    let req = invited_cmds::ping::Req {
        ping: "ping".to_owned(),
    };

    let expected = invited_cmds::AnyCmdReq::Ping(req);

    let data = invited_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = if let invited_cmds::AnyCmdReq::Ping(data) = data {
        data.dump().unwrap()
    } else {
        unreachable!()
    };

    let data2 = invited_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_invited_ping_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   pong: "pong"
    //   status: "ok"
    let raw = hex!("82a4706f6e67a4706f6e67a6737461747573a26f6b");

    let expected = invited_cmds::ping::Rep::Ok {
        pong: "pong".to_owned(),
    };

    let data = invited_cmds::ping::Rep::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = invited_cmds::ping::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn authenticated_ping_load_response() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   pong: "pong"
    //   status: "ok"
    let raw = hex!("82a4706f6e67a4706f6e67a6737461747573a26f6b");

    let rep = authenticated_cmds::ping::Req::load_response(&raw).unwrap();

    assert_eq!(
        rep,
        authenticated_cmds::ping::Rep::Ok {
            pong: "pong".to_owned(),
        }
    )
}

#[parsec_test]
fn invited_ping_load_response() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   pong: "pong"
    //   status: "ok"
    let raw = hex!("82a4706f6e67a4706f6e67a6737461747573a26f6b");

    let rep = invited_cmds::ping::Req::load_response(&raw).unwrap();

    assert_eq!(
        rep,
        invited_cmds::ping::Rep::Ok {
            pong: "pong".to_owned(),
        }
    )
}
