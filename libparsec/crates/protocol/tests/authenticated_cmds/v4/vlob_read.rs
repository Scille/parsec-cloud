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

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::VlobRead(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Rust implementation (Parsec v2.15.0+dev)
    // Content:
    //   author: "alice@dev1"
    //   certificate_index: 0
    //   timestamp: ext(1, 946774800.0)
    //   version: 8
    //   blob: hex!("666f6f626172")
    //   status: "ok"
    let raw = hex!(
        "86a6737461747573a26f6ba6617574686f72aa616c6963654064657631a4626c6f62c40666"
        "6f6f626172b163657274696669636174655f696e64657800a974696d657374616d70d70141"
        "cc375188000000a776657273696f6e08"
    );

    let expected = authenticated_cmds::vlob_read::Rep::Ok {
        version: 8,
        blob: b"foobar".as_ref().into(),
        author: "alice@dev1".parse().unwrap(),
        timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        certificate_index: 0,
    };

    let data = authenticated_cmds::vlob_read::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_read::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_not_found() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "not_found"
    let raw = hex!("82a6726561736f6ea6666f6f626172a6737461747573a96e6f745f666f756e64");

    let expected = authenticated_cmds::vlob_read::Rep::NotFound {
        reason: Some("foobar".to_owned()),
    };

    let data = authenticated_cmds::vlob_read::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_read::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_not_allowed() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_allowed"
    let raw = hex!("81a6737461747573ab6e6f745f616c6c6f776564");

    let expected = authenticated_cmds::vlob_read::Rep::NotAllowed;

    let data = authenticated_cmds::vlob_read::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_read::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_bad_version() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "bad_version"
    let raw = hex!("81a6737461747573ab6261645f76657273696f6e");

    let expected = authenticated_cmds::vlob_read::Rep::BadVersion;

    let data = authenticated_cmds::vlob_read::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_read::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_bad_encryption_revision() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "bad_encryption_revision"
    let raw = hex!("81a6737461747573b76261645f656e6372797074696f6e5f7265766973696f6e");

    let expected = authenticated_cmds::vlob_read::Rep::BadEncryptionRevision;

    let data = authenticated_cmds::vlob_read::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_read::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_in_maintenance() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "in_maintenance
    let raw = hex!("81a6737461747573ae696e5f6d61696e74656e616e6365");

    let expected = authenticated_cmds::vlob_read::Rep::InMaintenance;

    let data = authenticated_cmds::vlob_read::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_read::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
