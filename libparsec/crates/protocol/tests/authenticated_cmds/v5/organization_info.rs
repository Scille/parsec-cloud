// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use super::authenticated_cmds;

use libparsec_tests_lite::{hex, p_assert_eq};

// Request

pub fn req() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   cmd: 'organization_info'
    let raw: &[u8] = hex!("81a3636d64b16f7267616e697a6174696f6e5f696e666f").as_ref();
    let req = authenticated_cmds::organization_info::Req;

    println!("***expected: {:?}", req.dump().unwrap());
    let expected = authenticated_cmds::AnyCmdReq::OrganizationInfo(req);

    let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::OrganizationInfo(req2) = data else {
        unreachable!()
    };
    let raw2 = req2.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   status: 'ok'
    //   total_block_bytes: 180
    //   total_metadata_bytes: 5
    let raw: &[u8] = hex!(
        "83a6737461747573a26f6bb1746f74616c5f626c6f636b5f6279746573ccb4b4746f74"
        "616c5f6d657461646174615f627974657305"
    )
    .as_ref();

    let expected = authenticated_cmds::organization_info::Rep::Ok {
        total_block_bytes: 180,
        total_metadata_bytes: 5,
    };

    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_cmds::organization_info::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::organization_info::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
