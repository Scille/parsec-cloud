// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;

use super::anonymous_server_cmds;

// Request

pub fn req() {
    // Generated from Parsec 3.3.3-a.0+dev
    // Content:
    //   cmd: 'ping'
    //   ping: 'ping'
    let raw: &[u8] = hex!("82a3636d64a470696e67a470696e67a470696e67").as_ref();
    let req = anonymous_server_cmds::ping::Req {
        ping: "ping".to_owned(),
    };

    let expected = anonymous_server_cmds::AnyCmdReq::Ping(req);
    let data = anonymous_server_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let anonymous_server_cmds::AnyCmdReq::Ping(req2) = data else {
        unreachable!()
    };
    let raw2 = req2.dump().unwrap();

    let data2 = anonymous_server_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.3.3-a.0+dev
    // Content:
    //   status: 'ok'
    //   pong: 'pong'
    let raw: &[u8] = hex!("82a6737461747573a26f6ba4706f6e67a4706f6e67").as_ref();

    let expected = anonymous_server_cmds::ping::Rep::Ok {
        pong: "pong".to_owned(),
    };
    let data = anonymous_server_cmds::ping::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_server_cmds::ping::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
