// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::authenticated_cmds;

// Request

pub fn req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "realm_stats"
    //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
    let raw = hex!(
        "82a3636d64ab7265616c6d5f7374617473a87265616c6d5f6964d8021d3353157d7d4e95ad"
        "2fdea7b3bd19c5"
    );

    let req = authenticated_cmds::realm_stats::Req {
        realm_id: VlobID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
    };

    let expected = authenticated_cmds::AnyCmdReq::RealmStats(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::RealmStats(req2) = data else {
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
    //   blocks_size: 8
    //   status: "ok"
    //   vlobs_size: 8
    let raw = hex!("83ab626c6f636b735f73697a6508a6737461747573a26f6baa766c6f62735f73697a6508");

    let expected = authenticated_cmds::realm_stats::Rep::Ok {
        blocks_size: 8,
        vlobs_size: 8,
    };

    let data = authenticated_cmds::realm_stats::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_stats::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_not_allowed() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_allowed"
    let raw = hex!("81a6737461747573ab6e6f745f616c6c6f776564");

    let expected = authenticated_cmds::realm_stats::Rep::NotAllowed;

    let data = authenticated_cmds::realm_stats::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_stats::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_not_found() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "not_found"
    let raw = hex!("82a6726561736f6ea6666f6f626172a6737461747573a96e6f745f666f756e64");

    let expected = authenticated_cmds::realm_stats::Rep::NotFound {
        reason: Some("foobar".to_owned()),
    };

    let data = authenticated_cmds::realm_stats::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_stats::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
