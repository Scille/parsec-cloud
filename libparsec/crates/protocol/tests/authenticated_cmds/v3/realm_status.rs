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
    //   cmd: "realm_status"
    //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
    let raw = hex!(
        "82a3636d64ac7265616c6d5f737461747573a87265616c6d5f6964d8021d3353157d7d4e95"
        "ad2fdea7b3bd19c5"
    );

    let req = authenticated_cmds::realm_status::Req {
        realm_id: VlobID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
    };

    let expected = authenticated_cmds::AnyCmdReq::RealmStatus(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::RealmStatus(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    let raw_expected = [
        (
            // Generated from Rust implementation (Parsec v2.12.1+dev)
            // Content:
            //   encryption_revision: 8
            //   in_maintenance: true
            //   maintenance_started_by: None
            //   maintenance_started_on: None
            //   maintenance_type: "REENCRYPTION"
            //   status: "ok"
            //
            &hex!(
                "86a6737461747573a26f6bae696e5f6d61696e74656e616e6365c3b06d61696e74656e616e"
                "63655f74797065ac5245454e4352595054494f4eb66d61696e74656e616e63655f73746172"
                "7465645f6f6ec0b66d61696e74656e616e63655f737461727465645f6279c0b3656e637279"
                "7074696f6e5f7265766973696f6e08"
            )[..],
            authenticated_cmds::realm_status::Rep::Ok {
                in_maintenance: true,
                maintenance_type: Some(
                    authenticated_cmds::realm_status::MaintenanceType::Reencryption,
                ),
                maintenance_started_on: None,
                maintenance_started_by: None,
                encryption_revision: 8,
            },
        ),
        (
            // Generated from Python implementation (Parsec v2.6.0+dev)
            // Content:
            //   encryption_revision: 8
            //   in_maintenance: true
            //   maintenance_started_by: "alice@dev1"
            //   maintenance_started_on: ext(1, 946774800.0)
            //   maintenance_type: "GARBAGE_COLLECTION"
            //   status: "ok"
            &hex!(
                "86b3656e6372797074696f6e5f7265766973696f6e08ae696e5f6d61696e74656e616e6365"
                "c3b66d61696e74656e616e63655f737461727465645f6279aa616c6963654064657631b66d"
                "61696e74656e616e63655f737461727465645f6f6ed70141cc375188000000b06d61696e74"
                "656e616e63655f74797065b2474152424147455f434f4c4c454354494f4ea6737461747573"
                "a26f6b"
            )[..],
            authenticated_cmds::realm_status::Rep::Ok {
                in_maintenance: true,
                maintenance_type: Some(
                    authenticated_cmds::realm_status::MaintenanceType::GarbageCollection,
                ),
                maintenance_started_on: Some("2000-1-2T01:00:00Z".parse().unwrap()),
                maintenance_started_by: Some("alice@dev1".parse().unwrap()),
                encryption_revision: 8,
            },
        ),
    ];

    for (raw, expected) in raw_expected {
        let data = authenticated_cmds::realm_status::Rep::load(raw).unwrap();

        p_assert_eq!(data, expected);

        // Also test serialization round trip
        let raw2 = data.dump().unwrap();

        let data2 = authenticated_cmds::realm_status::Rep::load(&raw2).unwrap();

        p_assert_eq!(data2, expected);
    }
}

pub fn rep_not_allowed() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_allowed"
    let raw = hex!("81a6737461747573ab6e6f745f616c6c6f776564");

    let expected = authenticated_cmds::realm_status::Rep::NotAllowed;

    let data = authenticated_cmds::realm_status::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_status::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_not_found() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "not_found"
    let raw = hex!("82a6726561736f6ea6666f6f626172a6737461747573a96e6f745f666f756e64");

    let expected = authenticated_cmds::realm_status::Rep::NotFound {
        reason: Some("foobar".to_owned()),
    };

    let data = authenticated_cmds::realm_status::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_status::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
