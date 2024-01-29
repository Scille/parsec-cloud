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
    let raw_expected = [
        (
            // Generated from Rust implementation (Parsec v3.0.0+dev)
            // Content:
            //   timestamp: ext(1, 946774800.0)
            //   blob: hex!("666f6f626172")
            //   cmd: "vlob_create"
            //   key_index: 8
            //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
            //   sequester_blob: None
            //   vlob_id: ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))
            //
            &hex!(
                "87a3636d64ab766c6f625f637265617465a87265616c6d5f6964d8021d3353157d7d4e95ad"
                "2fdea7b3bd19c5a7766c6f625f6964d8022b5f314728134a12863da1ce49c112f6a96b6579"
                "5f696e64657808a974696d657374616d70d70141cc375188000000a4626c6f62c406666f6f"
                "626172ae7365717565737465725f626c6f62c0"
            )[..],
            authenticated_cmds::vlob_create::Req {
                realm_id: VlobID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
                key_index: 8,
                vlob_id: VlobID::from_hex("2b5f314728134a12863da1ce49c112f6").unwrap(),
                timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
                blob: b"foobar".as_ref().into(),
                sequester_blob: None,
            },
        ),
        (
            // Generated from Rust implementation (Parsec v3.0.0+dev)
            // Content:
            //   timestamp: ext(1, 946774800.0)
            //   blob: hex!("666f6f626172")
            //   cmd: "vlob_create"
            //   key_index: 8
            //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
            //   sequester_blob: {
            //     ExtType(code=2, data=b'\xb5\xebVSC\xc4B\xb3\xa2k\xe4Es\x81?\xf0'): hex!("666f6f626172")
            //   }
            //   vlob_id: ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))
            //
            &hex!(
                "87a3636d64ab766c6f625f637265617465a87265616c6d5f6964d8021d3353157d7d4e95ad"
                "2fdea7b3bd19c5a7766c6f625f6964d8022b5f314728134a12863da1ce49c112f6a96b6579"
                "5f696e64657808a974696d657374616d70d70141cc375188000000a4626c6f62c406666f6f"
                "626172ae7365717565737465725f626c6f6281d802b5eb565343c442b3a26be44573813ff0"
                "c406666f6f626172"
            )[..],
            authenticated_cmds::vlob_create::Req {
                realm_id: VlobID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
                key_index: 8,
                vlob_id: VlobID::from_hex("2b5f314728134a12863da1ce49c112f6").unwrap(),
                timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
                blob: b"foobar".as_ref().into(),
                sequester_blob: Some(HashMap::from([(
                    SequesterServiceID::from_hex("b5eb565343c442b3a26be44573813ff0").unwrap(),
                    b"foobar".as_ref().into(),
                )])),
            },
        ),
    ];

    for (raw, expected) in raw_expected {
        let expected = authenticated_cmds::AnyCmdReq::VlobCreate(expected);

        let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();

        assert_eq!(data, expected);

        // Also test serialization round trip
        let authenticated_cmds::AnyCmdReq::VlobCreate(req2) = data else {
            unreachable!()
        };

        let raw2 = req2.dump().unwrap();

        let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

        assert_eq!(data2, expected);
    }
}

// Responses

pub fn rep_ok() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    let raw = hex!("81a6737461747573a26f6b");

    let expected = authenticated_cmds::vlob_create::Rep::Ok;

    let data = authenticated_cmds::vlob_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_vlob_already_exists() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "vlob_already_exists"
    let raw = hex!("81a6737461747573b3766c6f625f616c72656164795f657869737473");

    let expected = authenticated_cmds::vlob_create::Rep::VlobAlreadyExists;

    let data = authenticated_cmds::vlob_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_author_not_allowed() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "author_not_allowed"
    let raw = hex!("81a6737461747573b2617574686f725f6e6f745f616c6c6f776564");

    let expected = authenticated_cmds::vlob_create::Rep::AuthorNotAllowed;

    let data = authenticated_cmds::vlob_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_bad_key_index() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   last_realm_certificate_timestamp: ext(1, 946774800.0)
    //   status: "bad_key_index"
    let raw = hex!(
        "82a6737461747573ad6261645f6b65795f696e646578d9206c6173745f7265616c6d5f6365"
        "7274696669636174655f74696d657374616d70d70141cc375188000000"
    );

    let expected = authenticated_cmds::vlob_create::Rep::BadKeyIndex {
        last_realm_certificate_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::vlob_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_realm_not_found() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "realm_not_found"
    let raw = hex!("81a6737461747573af7265616c6d5f6e6f745f666f756e64");

    let expected = authenticated_cmds::vlob_create::Rep::RealmNotFound;

    let data = authenticated_cmds::vlob_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_require_greater_timestamp() {
    // Generated from Python implementation (Parsec v2.11.1+dev)
    // Content:
    //   status: "require_greater_timestamp"
    //   strictly_greater_than: ext(1, 946774800.0)
    //
    let raw = hex!(
        "82a6737461747573b9726571756972655f677265617465725f74696d657374616d70b57374"
        "726963746c795f677265617465725f7468616ed70141cc375188000000"
    );

    let expected = authenticated_cmds::vlob_create::Rep::RequireGreaterTimestamp {
        strictly_greater_than: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::vlob_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_timestamp_out_of_ballpark() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   ballpark_client_early_offset: 300.0
    //   ballpark_client_late_offset: 320.0
    //   client_timestamp: ext(1, 946774800.0)
    //   server_timestamp: ext(1, 946774800.0)
    //   status: "timestamp_out_of_ballpark"
    //
    let raw = hex!(
        "85a6737461747573b974696d657374616d705f6f75745f6f665f62616c6c7061726bbc6261"
        "6c6c7061726b5f636c69656e745f6561726c795f6f6666736574cb4072c00000000000bb62"
        "616c6c7061726b5f636c69656e745f6c6174655f6f6666736574cb4074000000000000b063"
        "6c69656e745f74696d657374616d70d70141cc375188000000b07365727665725f74696d65"
        "7374616d70d70141cc375188000000"
    );

    let expected = authenticated_cmds::vlob_create::Rep::TimestampOutOfBallpark {
        ballpark_client_early_offset: 300.,
        ballpark_client_late_offset: 320.,
        server_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        client_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::vlob_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_organization_not_sequestered() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "organization_not_sequestered"
    //
    let raw = hex!("81a6737461747573bc6f7267616e697a6174696f6e5f6e6f745f7365717565737465726564");

    let expected = authenticated_cmds::vlob_create::Rep::OrganizationNotSequestered;

    let data = authenticated_cmds::vlob_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_sequester_inconsistency() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   last_common_certificate_timestamp: ext(1, 946774800.0)
    //   status: "sequester_inconsistency"
    //
    let raw = hex!(
        "82a6737461747573b77365717565737465725f696e636f6e73697374656e6379d9216c6173"
        "745f636f6d6d6f6e5f63657274696669636174655f74696d657374616d70d70141cc375188"
        "000000"
    );

    let expected = authenticated_cmds::vlob_create::Rep::SequesterInconsistency {
        last_common_certificate_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::vlob_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_sequester_service_unavailable() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "sequester_service_unavailable"
    //
    let raw = hex!(
        "81a6737461747573bd7365717565737465725f736572766963655f756e617661696c61626c"
        "65"
    );

    let expected = authenticated_cmds::vlob_create::Rep::SequesterServiceUnavailable;

    let data = authenticated_cmds::vlob_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_rejected_by_sequester_service() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   reason: "foobar"
    //   service_id: ext(2, hex!("b5eb565343c442b3a26be44573813ff0"))
    //   status: "rejected_by_sequester_service"
    //
    let raw = hex!(
        "84a6737461747573bd72656a65637465645f62795f7365717565737465725f736572766963"
        "65a6726561736f6ea6666f6f626172aa736572766963655f6964d802b5eb565343c442b3a2"
        "6be44573813ff0ad736572766963655f6c6162656ca6666f6f626172"
    );

    let expected = authenticated_cmds::vlob_create::Rep::RejectedBySequesterService {
        reason: "foobar".into(),
        service_id: SequesterServiceID::from_hex("b5eb565343c442b3a26be44573813ff0").unwrap(),
    };

    let data = authenticated_cmds::vlob_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
