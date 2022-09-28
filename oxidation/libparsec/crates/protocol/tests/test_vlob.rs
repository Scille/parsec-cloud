// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::collections::HashMap;

use hex_literal::hex;
use rstest::rstest;

use libparsec_protocol::*;
use libparsec_types::{Maybe, ReencryptionBatchEntry};

#[rstest]
#[case::legacy(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   timestamp: ext(1, 946774800.0)
        //   blob: hex!("666f6f626172")
        //   cmd: "vlob_create"
        //   encryption_revision: 8
        //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
        //   vlob_id: ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))
        &hex!(
            "86a4626c6f62c406666f6f626172a3636d64ab766c6f625f637265617465b3656e63727970"
            "74696f6e5f7265766973696f6e08a87265616c6d5f6964d8021d3353157d7d4e95ad2fdea7"
            "b3bd19c5a974696d657374616d70d70141cc375188000000a7766c6f625f6964d8022b5f31"
            "4728134a12863da1ce49c112f6"
        )[..],
        authenticated_cmds::AnyCmdReq::VlobCreate(
            authenticated_cmds::vlob_create::Req {
                realm_id: "1d3353157d7d4e95ad2fdea7b3bd19c5".parse().unwrap(),
                encryption_revision: 8,
                vlob_id: "2b5f314728134a12863da1ce49c112f6".parse().unwrap(),
                timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
                blob: b"foobar".to_vec(),
                sequester_blob: Maybe::Absent,
            }
        )
    )
)]
#[case::without(
    (
        // Generated from Rust implementation (Parsec v2.12.1+dev)
        // Content:
        //   timestamp: ext(1, 946774800.0)
        //   blob: hex!("666f6f626172")
        //   cmd: "vlob_create"
        //   encryption_revision: 8
        //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
        //   sequester_blob: None
        //   vlob_id: ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))
        //
        &hex!(
            "87a3636d64ab766c6f625f637265617465a87265616c6d5f6964d8021d3353157d7d4e95ad"
            "2fdea7b3bd19c5b3656e6372797074696f6e5f7265766973696f6e08a7766c6f625f6964d8"
            "022b5f314728134a12863da1ce49c112f6a974696d657374616d70d70141cc375188000000"
            "a4626c6f62c406666f6f626172ae7365717565737465725f626c6f62c0"
        )[..],
        authenticated_cmds::AnyCmdReq::VlobCreate(
            authenticated_cmds::vlob_create::Req {
                realm_id: "1d3353157d7d4e95ad2fdea7b3bd19c5".parse().unwrap(),
                encryption_revision: 8,
                vlob_id: "2b5f314728134a12863da1ce49c112f6".parse().unwrap(),
                timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
                blob: b"foobar".to_vec(),
                sequester_blob: Maybe::Present(None),
            }
        )
    )
)]
#[case::full(
    (
        // Generated from Python implementation (Parsec v2.11.1+dev)
        // Content:
        //   timestamp: ext(1, 946774800.0)
        //   blob: hex!("666f6f626172")
        //   cmd: "vlob_create"
        //   encryption_revision: 8
        //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
        //   sequester_blob: {
        //     ExtType(code=2, data=b'\xb5\xebVSC\xc4B\xb3\xa2k\xe4Es\x81?\xf0'): hex!("666f6f626172")
        //   }
        //   vlob_id: ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))
        //
        &hex!(
            "87a4626c6f62c406666f6f626172a3636d64ab766c6f625f637265617465b3656e63727970"
            "74696f6e5f7265766973696f6e08a87265616c6d5f6964d8021d3353157d7d4e95ad2fdea7"
            "b3bd19c5ae7365717565737465725f626c6f6281d802b5eb565343c442b3a26be44573813f"
            "f0c406666f6f626172a974696d657374616d70d70141cc375188000000a7766c6f625f6964"
            "d8022b5f314728134a12863da1ce49c112f6"
        )[..],
        authenticated_cmds::AnyCmdReq::VlobCreate(
            authenticated_cmds::vlob_create::Req {
                realm_id: "1d3353157d7d4e95ad2fdea7b3bd19c5".parse().unwrap(),
                encryption_revision: 8,
                vlob_id: "2b5f314728134a12863da1ce49c112f6".parse().unwrap(),
                timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
                blob: b"foobar".to_vec(),
                sequester_blob: Maybe::Present(Some(
                    HashMap::from([("b5eb565343c442b3a26be44573813ff0".parse().unwrap(), b"foobar".to_vec())])
                )),
            }
        )
    )
)]
fn serde_vlob_create_req(#[case] raw_expected: (&[u8], authenticated_cmds::AnyCmdReq)) {
    let (raw, expected) = raw_expected;

    let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "ok"
        &hex!(
            "81a6737461747573a26f6b"
        )[..],
        authenticated_cmds::vlob_create::Rep::Ok
    )
)]
#[case::already_exists(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "already_exists"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573ae616c72656164795f657869737473"
        )[..],
        authenticated_cmds::vlob_create::Rep::AlreadyExists {
            reason: Some("foobar".to_owned())
        }
    )
)]
#[case::not_allowed(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_allowed"
        &hex!(
            "81a6737461747573ab6e6f745f616c6c6f776564"
        )[..],
        authenticated_cmds::vlob_create::Rep::NotAllowed
    )
)]
#[case::bad_encryption_revision(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "bad_encryption_revision"
        &hex!(
            "81a6737461747573b76261645f656e6372797074696f6e5f7265766973696f6e"
        )[..],
        authenticated_cmds::vlob_create::Rep::BadEncryptionRevision
    )
)]
#[case::in_maintenance(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "in_maintenance"
        &hex!(
            "81a6737461747573ae696e5f6d61696e74656e616e6365"
        )[..],
        authenticated_cmds::vlob_create::Rep::InMaintenance
    )
)]
#[case::require_greater_timestamp(
    (
        // Generated from Python implementation (Parsec v2.11.1+dev)
        // Content:
        //   status: "require_greater_timestamp"
        //   strictly_greater_than: ext(1, 946774800.0)
        //
        &hex!(
            "82a6737461747573b9726571756972655f677265617465725f74696d657374616d70b57374"
            "726963746c795f677265617465725f7468616ed70141cc375188000000"
        )[..],
        authenticated_cmds::vlob_create::Rep::RequireGreaterTimestamp {
            strictly_greater_than: "2000-1-2T01:00:00Z".parse().unwrap(),
        }
    )
)]
#[case::bad_timestamp(
    (
        // Generated from Python implementation (Parsec v2.11.1+dev)
        // Content:
        //   backend_timestamp: ext(1, 946774800.0)
        //   ballpark_client_early_offset: 50.0
        //   ballpark_client_late_offset: 70.0
        //   client_timestamp: ext(1, 946774800.0)
        //   status: "bad_timestamp"
        //
        &hex!(
            "85b16261636b656e645f74696d657374616d70d70141cc375188000000bc62616c6c706172"
            "6b5f636c69656e745f6561726c795f6f6666736574cb4049000000000000bb62616c6c7061"
            "726b5f636c69656e745f6c6174655f6f6666736574cb4051800000000000b0636c69656e74"
            "5f74696d657374616d70d70141cc375188000000a6737461747573ad6261645f74696d6573"
            "74616d70"
        )[..],
        authenticated_cmds::vlob_create::Rep::BadTimestamp {
            reason: None,
            ballpark_client_early_offset: 50.,
            ballpark_client_late_offset: 70.,
            backend_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
            client_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        }
    )
)]
#[case::not_a_sequestered_organization(
    (
        // Generated from Python implementation (Parsec v2.11.1+dev)
        // Content:
        //   status: "not_a_sequestered_organization"
        //
        &hex!(
            "81a6737461747573be6e6f745f615f73657175657374657265645f6f7267616e697a617469"
            "6f6e"
        )[..],
        authenticated_cmds::vlob_create::Rep::NotASequesteredOrganization,
    )
)]
#[case::sequester_inconsistency(
    (
        // Generated from Python implementation (Parsec v2.11.1+dev)
        // Content:
        //   sequester_authority_certificate: hex!("666f6f626172")
        //   sequester_services_certificates: [hex!("666f6f"), hex!("626172")]
        //   status: "sequester_inconsistency"
        //
        &hex!(
            "83bf7365717565737465725f617574686f726974795f6365727469666963617465c406666f"
            "6f626172bf7365717565737465725f73657276696365735f63657274696669636174657392"
            "c403666f6fc403626172a6737461747573b77365717565737465725f696e636f6e73697374"
            "656e6379"
        )[..],
        authenticated_cmds::vlob_create::Rep::SequesterInconsistency {
            sequester_authority_certificate: b"foobar".to_vec(),
            sequester_services_certificates: vec![b"foo".to_vec(), b"bar".to_vec()],
        },
    )
)]
fn serde_vlob_create_rep(#[case] raw_expected: (&[u8], authenticated_cmds::vlob_create::Rep)) {
    let (raw, expected) = raw_expected;

    let data = authenticated_cmds::vlob_create::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_create::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
fn serde_vlob_read_req() {
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
        vlob_id: "2b5f314728134a12863da1ce49c112f6".parse().unwrap(),
        version: Some(8),
        timestamp: Some("2000-1-2T01:00:00Z".parse().unwrap()),
    };

    let expected = authenticated_cmds::AnyCmdReq::VlobRead(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   author: "alice@dev1"
        //   timestamp: ext(1, 946774800.0)
        //   version: 8
        //   author_last_role_granted_on: ext(1, 946774800.0)
        //   blob: hex!("666f6f626172")
        //   status: "ok"
        &hex!(
            "86a6617574686f72aa616c6963654064657631bb617574686f725f6c6173745f726f6c655f"
        "6772616e7465645f6f6ed70141cc375188000000a4626c6f62c406666f6f626172a6737461"
        "747573a26f6ba974696d657374616d70d70141cc375188000000a776657273696f6e08"
        )[..],
        authenticated_cmds::vlob_read::Rep::Ok {
            version: 8,
            blob: b"foobar".to_vec(),
            author: "alice@dev1".parse().unwrap(),
            timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
            author_last_role_granted_on: Maybe::Present(Some("2000-1-2T01:00:00Z".parse().unwrap())),
        }
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "not_found"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573a96e6f745f666f756e64"
        )[..],
        authenticated_cmds::vlob_read::Rep::NotFound {
            reason: Some("foobar".to_owned()),
        }
    )
)]
#[case::not_allowed(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_allowed"
        &hex!(
            "81a6737461747573ab6e6f745f616c6c6f776564"
        )[..],
        authenticated_cmds::vlob_read::Rep::NotAllowed
    )
)]
#[case::bad_version(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "bad_version"
        &hex!(
            "81a6737461747573ab6261645f76657273696f6e"
        )[..],
        authenticated_cmds::vlob_read::Rep::BadVersion
    )
)]
#[case::bad_encryption_revision(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "bad_encryption_revision"
        &hex!(
            "81a6737461747573b76261645f656e6372797074696f6e5f7265766973696f6e"
        )[..],
        authenticated_cmds::vlob_read::Rep::BadEncryptionRevision
    )
)]
#[case::in_maintenance(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "in_maintenance
        &hex!(
            "81a6737461747573ae696e5f6d61696e74656e616e6365"
        )[..],
        authenticated_cmds::vlob_read::Rep::InMaintenance
    )
)]
fn serde_vlob_read_rep(#[case] raw_expected: (&[u8], authenticated_cmds::vlob_read::Rep)) {
    let (raw, expected) = raw_expected;

    let data = authenticated_cmds::vlob_read::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_read::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[rstest]
#[case::legacy(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   timestamp: ext(1, 946774800.0)
        //   version: 8
        //   blob: hex!("666f6f626172")
        //   cmd: "vlob_update"
        //   encryption_revision: 8
        //   vlob_id: ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))
        &hex!(
            "86a4626c6f62c406666f6f626172a3636d64ab766c6f625f757064617465b3656e63727970"
            "74696f6e5f7265766973696f6e08a974696d657374616d70d70141cc375188000000a77665"
            "7273696f6e08a7766c6f625f6964d8022b5f314728134a12863da1ce49c112f6"
        )[..],
        authenticated_cmds::AnyCmdReq::VlobUpdate(
            authenticated_cmds::vlob_update::Req {
                encryption_revision: 8,
                vlob_id: "2b5f314728134a12863da1ce49c112f6".parse().unwrap(),
                timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
                version: 8,
                blob: b"foobar".to_vec(),
                sequester_blob: Maybe::Absent,
            }
        )
    )
)]
#[case::without(
    (
        // Generated from Rust implementation (Parsec v2.12.1+dev)
        // Content:
        //   timestamp: ext(1, 946774800.0)
        //   version: 8
        //   blob: hex!("666f6f626172")
        //   cmd: "vlob_update"
        //   encryption_revision: 8
        //   sequester_blob: None
        //   vlob_id: ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))
        //
        &hex!(
            "87a3636d64ab766c6f625f757064617465b3656e6372797074696f6e5f7265766973696f6e"
            "08a7766c6f625f6964d8022b5f314728134a12863da1ce49c112f6a974696d657374616d70"
            "d70141cc375188000000a776657273696f6e08a4626c6f62c406666f6f626172ae73657175"
            "65737465725f626c6f62c0"
        )[..],
        authenticated_cmds::AnyCmdReq::VlobUpdate(
            authenticated_cmds::vlob_update::Req {
                encryption_revision: 8,
                vlob_id: "2b5f314728134a12863da1ce49c112f6".parse().unwrap(),
                timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
                version: 8,
                blob: b"foobar".to_vec(),
                sequester_blob: Maybe::Present(None),
            }
        )
    )
)]
#[case::full(
    (
        // Generated from Python implementation (Parsec v2.11.1+dev)
        // Content:
        //   timestamp: ext(1, 946774800.0)
        //   version: 8
        //   blob: hex!("666f6f626172")
        //   cmd: "vlob_update"
        //   encryption_revision: 8
        //   sequester_blob: {
        //     ExtType(code=2, data=b'\xb5\xebVSC\xc4B\xb3\xa2k\xe4Es\x81?\xf0'): hex!("666f6f626172")
        //   }
        //   vlob_id: ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))
        //
        &hex!(
            "87a4626c6f62c406666f6f626172a3636d64ab766c6f625f757064617465b3656e63727970"
            "74696f6e5f7265766973696f6e08ae7365717565737465725f626c6f6281d802b5eb565343"
            "c442b3a26be44573813ff0c406666f6f626172a974696d657374616d70d70141cc37518800"
            "0000a776657273696f6e08a7766c6f625f6964d8022b5f314728134a12863da1ce49c112f6"
        )[..],
        authenticated_cmds::AnyCmdReq::VlobUpdate(
            authenticated_cmds::vlob_update::Req {
                encryption_revision: 8,
                vlob_id: "2b5f314728134a12863da1ce49c112f6".parse().unwrap(),
                timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
                version: 8,
                blob: b"foobar".to_vec(),
                sequester_blob: Maybe::Present(Some(
                    HashMap::from([("b5eb565343c442b3a26be44573813ff0".parse().unwrap(), b"foobar".to_vec())])
                )),
            }
        )
    )
)]
fn serde_vlob_update_req(#[case] raw_expected: (&[u8], authenticated_cmds::AnyCmdReq)) {
    let (raw, expected) = raw_expected;

    let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "ok"
        &hex!(
            "81a6737461747573a26f6b"
        )[..],
        authenticated_cmds::vlob_update::Rep::Ok
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "not_found"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573a96e6f745f666f756e64"
        )[..],
        authenticated_cmds::vlob_update::Rep::NotFound {
            reason: Some("foobar".to_owned()),
        }
    )
)]
#[case::not_allowed(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_allowed"
        &hex!(
            "81a6737461747573ab6e6f745f616c6c6f776564"
        )[..],
        authenticated_cmds::vlob_update::Rep::NotAllowed
    )
)]
#[case::bad_version(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "bad_version"
        &hex!(
            "81a6737461747573ab6261645f76657273696f6e"
        )[..],
        authenticated_cmds::vlob_update::Rep::BadVersion
    )
)]
#[case::bad_encryption_revision(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "bad_encryption_revision"
        &hex!(
            "81a6737461747573b76261645f656e6372797074696f6e5f7265766973696f6e"
        )[..],
        authenticated_cmds::vlob_update::Rep::BadEncryptionRevision
    )
)]
#[case::in_maintenance(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "in_maintenance"
        &hex!(
            "81a6737461747573ae696e5f6d61696e74656e616e6365"
        )[..],
        authenticated_cmds::vlob_update::Rep::InMaintenance
    )
)]
#[case::require_greater_timestamp(
    (
        // Generated from Python implementation (Parsec v2.11.1+dev)
        // Content:
        //   status: "require_greater_timestamp"
        //   strictly_greater_than: ext(1, 946774800.0)
        //
        &hex!(
            "82a6737461747573b9726571756972655f677265617465725f74696d657374616d70b57374"
            "726963746c795f677265617465725f7468616ed70141cc375188000000"
        )[..],
        authenticated_cmds::vlob_update::Rep::RequireGreaterTimestamp {
            strictly_greater_than: "2000-1-2T01:00:00Z".parse().unwrap(),
        }
    )
)]
#[case::bad_timestamp(
    (
        // Generated from Python implementation (Parsec v2.11.1+dev)
        // Content:
        //   backend_timestamp: ext(1, 946774800.0)
        //   ballpark_client_early_offset: 50.0
        //   ballpark_client_late_offset: 70.0
        //   client_timestamp: ext(1, 946774800.0)
        //   status: "bad_timestamp"
        //
        &hex!(
            "85b16261636b656e645f74696d657374616d70d70141cc375188000000bc62616c6c706172"
            "6b5f636c69656e745f6561726c795f6f6666736574cb4049000000000000bb62616c6c7061"
            "726b5f636c69656e745f6c6174655f6f6666736574cb4051800000000000b0636c69656e74"
            "5f74696d657374616d70d70141cc375188000000a6737461747573ad6261645f74696d6573"
            "74616d70"
        )[..],
        authenticated_cmds::vlob_update::Rep::BadTimestamp {
            reason: None,
            ballpark_client_early_offset: 50.,
            ballpark_client_late_offset: 70.,
            backend_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
            client_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        }
    )
)]
#[case::not_a_sequestered_organization(
    (
        // Generated from Python implementation (Parsec v2.11.1+dev)
        // Content:
        //   status: "not_a_sequestered_organization"
        //
        &hex!(
            "81a6737461747573be6e6f745f615f73657175657374657265645f6f7267616e697a617469"
            "6f6e"
        )[..],
        authenticated_cmds::vlob_update::Rep::NotASequesteredOrganization,
    )
)]
#[case::sequester_inconsistency(
    (
        // Generated from Python implementation (Parsec v2.11.1+dev)
        // Content:
        //   sequester_authority_certificate: hex!("666f6f626172")
        //   sequester_services_certificates: [hex!("666f6f"), hex!("626172")]
        //   status: "sequester_inconsistency"
        //
        &hex!(
            "83bf7365717565737465725f617574686f726974795f6365727469666963617465c406666f"
            "6f626172bf7365717565737465725f73657276696365735f63657274696669636174657392"
            "c403666f6fc403626172a6737461747573b77365717565737465725f696e636f6e73697374"
            "656e6379"
        )[..],
        authenticated_cmds::vlob_update::Rep::SequesterInconsistency {
            sequester_authority_certificate: b"foobar".to_vec(),
            sequester_services_certificates: vec![b"foo".to_vec(), b"bar".to_vec()],
        },
    )
)]
fn serde_vlob_update_rep(#[case] raw_expected: (&[u8], authenticated_cmds::vlob_update::Rep)) {
    let (raw, expected) = raw_expected;

    let data = authenticated_cmds::vlob_update::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_update::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
fn serde_vlob_poll_changes_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "vlob_poll_changes"
    //   last_checkpoint: 8
    //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
    let raw = hex!(
        "83a3636d64b1766c6f625f706f6c6c5f6368616e676573af6c6173745f636865636b706f69"
        "6e7408a87265616c6d5f6964d8021d3353157d7d4e95ad2fdea7b3bd19c5"
    );

    let req = authenticated_cmds::vlob_poll_changes::Req {
        realm_id: "1d3353157d7d4e95ad2fdea7b3bd19c5".parse().unwrap(),
        last_checkpoint: 8,
    };

    let expected = authenticated_cmds::AnyCmdReq::VlobPollChanges(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   changes: {ExtType(code=2, raw=b'+_1G(\x13J\x12\x86=\xa1\xceI\xc1\x12\xf6'):8}
        //   current_checkpoint: 8
        //   status: "ok"
        &hex!(
            "83a76368616e67657381d8022b5f314728134a12863da1ce49c112f608b263757272656e74"
            "5f636865636b706f696e7408a6737461747573a26f6b"
        )[..],
        authenticated_cmds::vlob_poll_changes::Rep::Ok {
            changes: HashMap::from([("2b5f314728134a12863da1ce49c112f6".parse().unwrap(), 8)]),
            current_checkpoint: 8,
        }
    )
)]
#[case::not_allowed(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_allowed"
        &hex!(
            "81a6737461747573ab6e6f745f616c6c6f776564"
        )[..],
        authenticated_cmds::vlob_poll_changes::Rep::NotAllowed
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "not_found"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573a96e6f745f666f756e64"
        )[..],
        authenticated_cmds::vlob_poll_changes::Rep::NotFound {
            reason: Some("foobar".to_owned())
        }
    )
)]
#[case::in_maintenance(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "in_maintenance"
        &hex!(
            "81a6737461747573ae696e5f6d61696e74656e616e6365"
        )[..],
        authenticated_cmds::vlob_poll_changes::Rep::InMaintenance
    )
)]
fn serde_vlob_poll_changes_rep(
    #[case] raw_expected: (&[u8], authenticated_cmds::vlob_poll_changes::Rep),
) {
    let (raw, expected) = raw_expected;

    let data = authenticated_cmds::vlob_poll_changes::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_poll_changes::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
fn serde_vlob_list_versions_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "vlob_list_versions"
    //   vlob_id: ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))
    let raw = hex!(
        "82a3636d64b2766c6f625f6c6973745f76657273696f6e73a7766c6f625f6964d8022b5f31"
        "4728134a12863da1ce49c112f6"
    );

    let req = authenticated_cmds::vlob_list_versions::Req {
        vlob_id: "2b5f314728134a12863da1ce49c112f6".parse().unwrap(),
    };

    let expected = authenticated_cmds::AnyCmdReq::VlobListVersions(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "ok"
        //   versions: {8:[ext(1, 946774800.0), "alice@dev1"]}
        &hex!(
            "82a6737461747573a26f6ba876657273696f6e73810892d70141cc375188000000aa616c69"
            "63654064657631"
        )[..],
        authenticated_cmds::vlob_list_versions::Rep::Ok {
            versions: HashMap::from([(
                8,
                (
                    "2000-1-2T01:00:00Z".parse().unwrap(),
                    "alice@dev1".parse().unwrap(),
                ),
            )]),
        }
    )
)]
#[case::not_allowed(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_allowed"
        &hex!(
            "81a6737461747573ab6e6f745f616c6c6f776564"
        )[..],
        authenticated_cmds::vlob_list_versions::Rep::NotAllowed
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "not_found"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573a96e6f745f666f756e64"
        )[..],
        authenticated_cmds::vlob_list_versions::Rep::NotFound {
            reason: Some("foobar".to_owned())
        }
    )
)]
#[case::in_maintenance(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "in_maintenance"
        &hex!(
            "81a6737461747573ae696e5f6d61696e74656e616e6365"
        )[..],
        authenticated_cmds::vlob_list_versions::Rep::InMaintenance
    )
)]
fn serde_vlob_list_versions_rep(
    #[case] raw_expected: (&[u8], authenticated_cmds::vlob_list_versions::Rep),
) {
    let (raw, expected) = raw_expected;

    let data = authenticated_cmds::vlob_list_versions::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_list_versions::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
fn serde_vlob_maintenance_get_reencryption_batch_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "vlob_maintenance_get_reencryption_batch"
    //   encryption_revision: 8
    //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
    //   size: 8
    let raw = hex!(
        "84a3636d64d927766c6f625f6d61696e74656e616e63655f6765745f7265656e6372797074"
        "696f6e5f6261746368b3656e6372797074696f6e5f7265766973696f6e08a87265616c6d5f"
        "6964d8021d3353157d7d4e95ad2fdea7b3bd19c5a473697a6508"
    );

    let req = authenticated_cmds::vlob_maintenance_get_reencryption_batch::Req {
        realm_id: "1d3353157d7d4e95ad2fdea7b3bd19c5".parse().unwrap(),
        encryption_revision: 8,
        size: 8,
    };

    let expected = authenticated_cmds::AnyCmdReq::VlobMaintenanceGetReencryptionBatch(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   batch: [
        //     {
        //       version: 8
        //       blob: hex!("666f6f626172")
        //       vlob_id: ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))
        //     }
        //   ]
        //   status: "ok"
        &hex!(
            "82a562617463689183a776657273696f6e08a7766c6f625f6964d8022b5f314728134a1286"
            "3da1ce49c112f6a4626c6f62c406666f6f626172a6737461747573a26f6b"
        )[..],
        authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::Ok {
            batch: vec![ReencryptionBatchEntry {
                vlob_id: "2b5f314728134a12863da1ce49c112f6".parse().unwrap(),
                version: 8,
                blob: b"foobar".to_vec(),
            }],
        }
    )
)]
#[case::not_allowed(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_allowed"
        &hex!(
            "81a6737461747573ab6e6f745f616c6c6f776564"
        )[..],
        authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::NotAllowed
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "not_found"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573a96e6f745f666f756e64"
        )[..],
        authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::NotFound {
            reason: Some("foobar".to_owned())
        }
    )
)]
#[case::not_in_maintenance(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "not_in_maintenance"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573b26e6f745f696e5f6d61696e74656e"
            "616e6365"
        )[..],
        authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::NotInMaintenance {
            reason: Some("foobar".to_owned())
        }
    )
)]
#[case::bad_encryption_revision(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "bad_encryption_revision"
        &hex!(
            "81a6737461747573b76261645f656e6372797074696f6e5f7265766973696f6e"
        )[..],
        authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::BadEncryptionRevision
    )
)]
#[case::maintenance_error(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "maintenance_error"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573b16d61696e74656e616e63655f6572"
            "726f72"
        )[..],
        authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::MaintenanceError {
            reason: Some("foobar".to_owned())
        }
    )
)]
fn serde_vlob_maintenance_get_reencryption_batch_rep(
    #[case] raw_expected: (
        &[u8],
        authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep,
    ),
) {
    let (raw, expected) = raw_expected;

    let data = authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 =
        authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
fn serde_vlob_maintenance_save_reencryption_batch_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   batch: [
    //     {
    //       version: 8
    //       blob: hex!("666f6f626172")
    //       vlob_id: ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))
    //     }
    //   ]
    //   cmd: "vlob_maintenance_save_reencryption_batch"
    //   encryption_revision: 8
    //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
    let raw = hex!(
        "84a562617463689183a776657273696f6e08a7766c6f625f6964d8022b5f314728134a1286"
        "3da1ce49c112f6a4626c6f62c406666f6f626172a3636d64d928766c6f625f6d61696e7465"
        "6e616e63655f736176655f7265656e6372797074696f6e5f6261746368b3656e6372797074"
        "696f6e5f7265766973696f6e08a87265616c6d5f6964d8021d3353157d7d4e95ad2fdea7b3"
        "bd19c5"
    );

    let req = authenticated_cmds::vlob_maintenance_save_reencryption_batch::Req {
        realm_id: "1d3353157d7d4e95ad2fdea7b3bd19c5".parse().unwrap(),
        encryption_revision: 8,
        batch: vec![ReencryptionBatchEntry {
            vlob_id: "2b5f314728134a12863da1ce49c112f6".parse().unwrap(),
            version: 8,
            blob: b"foobar".to_vec(),
        }],
    };

    let expected = authenticated_cmds::AnyCmdReq::VlobMaintenanceSaveReencryptionBatch(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   done: 8
        //   status: "ok"
        //   total: 8
        &hex!(
            "83a4646f6e6508a6737461747573a26f6ba5746f74616c08"
        )[..],
        authenticated_cmds::vlob_maintenance_save_reencryption_batch::Rep::Ok { total: 8, done: 8 }
    )
)]
#[case::not_allowed(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_allowed"
        &hex!(
            "81a6737461747573ab6e6f745f616c6c6f776564"
        )[..],
        authenticated_cmds::vlob_maintenance_save_reencryption_batch::Rep::NotAllowed
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "not_found"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573a96e6f745f666f756e64"
        )[..],
        authenticated_cmds::vlob_maintenance_save_reencryption_batch::Rep::NotFound {
            reason: Some("foobar".to_owned())
        }
    )
)]
#[case::not_in_maintenance(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "not_in_maintenance"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573b26e6f745f696e5f6d61696e74656e"
            "616e6365"
        )[..],
        authenticated_cmds::vlob_maintenance_save_reencryption_batch::Rep::NotInMaintenance {
            reason: Some("foobar".to_owned())
        }
    )
)]
#[case::bad_encryption_revision(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "bad_encryption_revision"
        &hex!(
            "81a6737461747573b76261645f656e6372797074696f6e5f7265766973696f6e"
        )[..],
        authenticated_cmds::vlob_maintenance_save_reencryption_batch::Rep::BadEncryptionRevision
    )
)]
#[case::maintenance_error(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "maintenance_error"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573b16d61696e74656e616e63655f6572"
            "726f72"
        )[..],
        authenticated_cmds::vlob_maintenance_save_reencryption_batch::Rep::MaintenanceError {
            reason: Some("foobar".to_owned())
        }
    )
)]
fn serde_vlob_maintenance_save_reencryption_batch_rep(
    #[case] raw_expected: (
        &[u8],
        authenticated_cmds::vlob_maintenance_save_reencryption_batch::Rep,
    ),
) {
    let (raw, expected) = raw_expected;

    let data =
        authenticated_cmds::vlob_maintenance_save_reencryption_batch::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 =
        authenticated_cmds::vlob_maintenance_save_reencryption_batch::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}
