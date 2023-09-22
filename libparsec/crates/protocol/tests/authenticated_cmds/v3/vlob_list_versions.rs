// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::collections::HashMap;

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::authenticated_cmds;

// Request

pub fn req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "vlob_list_versions"
    //   vlob_id: ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))
    let raw = hex!(
        "82a3636d64b2766c6f625f6c6973745f76657273696f6e73a7766c6f625f6964d8022b5f31"
        "4728134a12863da1ce49c112f6"
    );

    let req = authenticated_cmds::vlob_list_versions::Req {
        vlob_id: VlobID::from_hex("2b5f314728134a12863da1ce49c112f6").unwrap(),
    };

    let expected = authenticated_cmds::AnyCmdReq::VlobListVersions(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::VlobListVersions(req2) = data else {
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
    //   status: "ok"
    //   versions: {8:[ext(1, 946774800.0), "alice@dev1"]}
    let raw = hex!(
        "82a6737461747573a26f6ba876657273696f6e73810892d70141cc375188000000aa616c69"
        "63654064657631"
    );

    let expected = authenticated_cmds::vlob_list_versions::Rep::Ok {
        versions: HashMap::from([(
            8,
            (
                "2000-1-2T01:00:00Z".parse().unwrap(),
                "alice@dev1".parse().unwrap(),
            ),
        )]),
    };

    let data = authenticated_cmds::vlob_list_versions::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_list_versions::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_not_allowed() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_allowed"
    let raw = hex!("81a6737461747573ab6e6f745f616c6c6f776564");

    let expected = authenticated_cmds::vlob_list_versions::Rep::NotAllowed;

    let data = authenticated_cmds::vlob_list_versions::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_list_versions::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_not_found() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "not_found"
    let raw = hex!("82a6726561736f6ea6666f6f626172a6737461747573a96e6f745f666f756e64");

    let expected = authenticated_cmds::vlob_list_versions::Rep::NotFound {
        reason: Some("foobar".to_owned()),
    };

    let data = authenticated_cmds::vlob_list_versions::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_list_versions::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_in_maintenance() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "in_maintenance"
    let raw = hex!("81a6737461747573ae696e5f6d61696e74656e616e6365");

    let expected = authenticated_cmds::vlob_list_versions::Rep::InMaintenance;

    let data = authenticated_cmds::vlob_list_versions::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_list_versions::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
