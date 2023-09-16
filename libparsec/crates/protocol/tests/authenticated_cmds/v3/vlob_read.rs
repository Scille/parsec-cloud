// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use super::authenticated_cmds;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

// Request

pub fn req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   timestamp: ext(1, 946774800.0)
    //   version: 8
    //   cmd: "vlob_read"
    //   encryption_revision: 8
    //   vlob_id: ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))
    let raw = hex!(
        "85a3636d64a9766c6f625f72656164b3656e6372797074696f6e5f7265766973696f6e08a9"
        "74696d657374616d70d70141cc375188000000a776657273696f6e08a7766c6f625f6964d8"
        "022b5f314728134a12863da1ce49c112f6"
    );

    let req = authenticated_cmds::vlob_read::Req {
        encryption_revision: 8,
        vlob_id: VlobID::from_hex("2b5f314728134a12863da1ce49c112f6").unwrap(),
        version: Some(8),
        timestamp: Some("2000-1-2T01:00:00Z".parse().unwrap()),
    };

    let expected = authenticated_cmds::AnyCmdReq::VlobRead(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = if let authenticated_cmds::AnyCmdReq::VlobRead(data) = data {
        data.dump().unwrap()
    } else {
        unreachable!()
    };

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // TODO #4545: Implement test
}

pub fn rep_not_found() {
    // TODO #4545: Implement test
}

pub fn rep_not_allowed() {
    // TODO #4545: Implement test
}

pub fn rep_bad_version() {
    // TODO #4545: Implement test
}

pub fn rep_bad_encryption_revision() {
    // TODO #4545: Implement test
}

pub fn rep_in_maintenance() {
    // TODO #4545: Implement test
}
