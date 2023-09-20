// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

// wrap this in each test function
// macro qui contient une closure
use super::authenticated_cmds;

use libparsec_tests_lite::{hex, p_assert_eq};
use libparsec_types::prelude::*;

// Request

pub fn req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   block: hex!("666f6f626172")
    //   block_id: ext(2, hex!("57c629b69d6c4abbaf651cafa46dbc93"))
    //   cmd: "block_create"
    //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
    let raw = hex!(
        "84a5626c6f636bc406666f6f626172a8626c6f636b5f6964d80257c629b69d6c4abbaf651c"
        "afa46dbc93a3636d64ac626c6f636b5f637265617465a87265616c6d5f6964d8021d335315"
        "7d7d4e95ad2fdea7b3bd19c5"
    );

    let req = authenticated_cmds::block_create::Req {
        block_id: BlockID::from_hex("57c629b69d6c4abbaf651cafa46dbc93").unwrap(),
        realm_id: VlobID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
        block: b"foobar".as_ref().into(),
    };

    let expected = authenticated_cmds::AnyCmdReq::BlockCreate(req.clone());

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = req.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    let raw = hex!("81a6737461747573a26f6b");
    let expected = authenticated_cmds::block_create::Rep::Ok;

    rep_helper(&raw, expected);
}

pub fn rep_already_exists() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "already_exists"
    let raw = hex!("81a6737461747573ae616c72656164795f657869737473");
    let expected = authenticated_cmds::block_create::Rep::AlreadyExists;

    rep_helper(&raw, expected);
}

pub fn rep_not_found() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    let raw = hex!("81a6737461747573a96e6f745f666f756e64");
    let expected = authenticated_cmds::block_create::Rep::NotFound;

    rep_helper(&raw, expected);
}

pub fn rep_not_allowed() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_allowed"
    let raw = hex!("81a6737461747573ab6e6f745f616c6c6f776564");
    let expected = authenticated_cmds::block_create::Rep::NotAllowed;

    rep_helper(&raw, expected);
}

pub fn rep_in_maintenance() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "in_maintenance"
    let raw = hex!("81a6737461747573ae696e5f6d61696e74656e616e6365");
    let expected = authenticated_cmds::block_create::Rep::InMaintenance;

    rep_helper(&raw, expected);
}

pub fn rep_timeout() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "timeout"
    let raw = hex!("81a6737461747573a774696d656f7574");
    let expected = authenticated_cmds::block_create::Rep::Timeout;

    rep_helper(&raw, expected);
}

fn rep_helper(raw: &[u8], expected: authenticated_cmds::block_create::Rep) {
    let data = authenticated_cmds::block_create::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::block_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
