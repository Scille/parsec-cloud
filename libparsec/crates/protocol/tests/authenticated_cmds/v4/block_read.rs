// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::{hex, p_assert_eq};
use libparsec_types::BlockID;

use super::authenticated_cmds;

// Request

pub fn req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   block_id: ext(2, hex!("57c629b69d6c4abbaf651cafa46dbc93"))
    //   cmd: "block_read"
    let raw = hex!(
        "82a8626c6f636b5f6964d80257c629b69d6c4abbaf651cafa46dbc93a3636d64aa626c6f63"
        "6b5f72656164"
    );

    let req = authenticated_cmds::block_read::Req {
        block_id: BlockID::from_hex("57c629b69d6c4abbaf651cafa46dbc93").unwrap(),
    };

    let expected = authenticated_cmds::AnyCmdReq::BlockRead(req.clone());

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
    //   block: hex!("666f6f626172")
    //   status: "ok"
    let raw = hex!("82a5626c6f636bc406666f6f626172a6737461747573a26f6b");
    let expected = authenticated_cmds::block_read::Rep::Ok {
        block: bytes::Bytes::from_static(b"foobar"),
    };
    rep_helper(&raw, expected);
}

pub fn rep_block_not_found() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "block_not_found"
    let raw = hex!("81a6737461747573af626c6f636b5f6e6f745f666f756e64");
    let expected = authenticated_cmds::block_read::Rep::BlockNotFound;
    rep_helper(&raw, expected);
}

pub fn rep_author_not_allowed() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "author_not_allowed"
    let raw = hex!("81a6737461747573b2617574686f725f6e6f745f616c6c6f776564");
    let expected = authenticated_cmds::block_read::Rep::AuthorNotAllowed;
    rep_helper(&raw, expected);
}

pub fn rep_store_unavailable() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "store_unavailable"
    let raw = hex!("81a6737461747573b173746f72655f756e617661696c61626c65");
    let expected = authenticated_cmds::block_read::Rep::StoreUnavailable;
    rep_helper(&raw, expected);
}

fn rep_helper(raw: &[u8], expected: authenticated_cmds::block_read::Rep) {
    let data = authenticated_cmds::block_read::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::block_read::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
