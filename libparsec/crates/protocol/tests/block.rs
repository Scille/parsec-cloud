// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_protocol::authenticated_cmds::v3 as authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

#[parsec_test]
fn serde_block_read_req() {
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

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = req.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
#[case::ok(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   block: hex!("666f6f626172")
    //   status: "ok"
    &hex!(
        "82a5626c6f636bc406666f6f626172a6737461747573a26f6b"
    )[..],
    authenticated_cmds::block_read::Rep::Ok {
        block: b"foobar".as_ref().into(),
    }
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    &hex!(
        "81a6737461747573a96e6f745f666f756e64"
    )[..],
    authenticated_cmds::block_read::Rep::NotFound
)]
#[case::timeout(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "timeout"
    &hex!(
        "81a6737461747573a774696d656f7574"
    )[..],
    authenticated_cmds::block_read::Rep::Timeout
)]
#[case::not_allowed(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_allowed"
    &hex!(
        "81a6737461747573ab6e6f745f616c6c6f776564"
    )[..],
    authenticated_cmds::block_read::Rep::NotAllowed
)]
#[case::in_maintenance(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "in_maintenance"
    &hex!(
        "81a6737461747573ae696e5f6d61696e74656e616e6365"
    )[..],
    authenticated_cmds::block_read::Rep::InMaintenance
)]
fn serde_block_read_rep(#[case] raw: &[u8], #[case] expected: authenticated_cmds::block_read::Rep) {
    let data = authenticated_cmds::block_read::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::block_read::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}
