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
    //   cmd: "events_subscribe"
    let raw = hex!("81a3636d64b06576656e74735f737562736372696265");

    let req = authenticated_cmds::events_subscribe::Req;

    let expected = authenticated_cmds::AnyCmdReq::EventsSubscribe(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    let authenticated_cmds::AnyCmdReq::EventsSubscribe(req2) = data else {
        unreachable!()
    };

    // Also test serialization round trip
    let raw2 = req2.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    let raw = hex!("81a6737461747573a26f6b");

    let expected = authenticated_cmds::events_subscribe::Rep::Ok;

    let data = authenticated_cmds::events_subscribe::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::events_subscribe::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
