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
            authenticated_cmds::AnyCmdReq::VlobCreate(authenticated_cmds::vlob_create::Req {
                realm_id: VlobID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
                encryption_revision: 8,
                vlob_id: VlobID::from_hex("2b5f314728134a12863da1ce49c112f6").unwrap(),
                timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
                blob: b"foobar".as_ref().into(),
                sequester_blob: None,
            }),
        ),
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
            authenticated_cmds::AnyCmdReq::VlobCreate(authenticated_cmds::vlob_create::Req {
                realm_id: VlobID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
                encryption_revision: 8,
                vlob_id: VlobID::from_hex("2b5f314728134a12863da1ce49c112f6").unwrap(),
                timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
                blob: b"foobar".as_ref().into(),
                sequester_blob: Some(HashMap::from([(
                    SequesterServiceID::from_hex("b5eb565343c442b3a26be44573813ff0").unwrap(),
                    b"foobar".as_ref().into(),
                )])),
            }),
        ),
    ];

    for (raw, expected) in raw_expected {
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

pub fn rep_already_exists() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "already_exists"
    let raw = hex!("82a6726561736f6ea6666f6f626172a6737461747573ae616c72656164795f657869737473");

    let expected = authenticated_cmds::vlob_create::Rep::AlreadyExists {
        reason: Some("foobar".to_owned()),
    };

    let data = authenticated_cmds::vlob_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_not_allowed() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_allowed"
    let raw = hex!("81a6737461747573ab6e6f745f616c6c6f776564");

    let expected = authenticated_cmds::vlob_create::Rep::NotAllowed;

    let data = authenticated_cmds::vlob_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_bad_encryption_revision() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "bad_encryption_revision"
    let raw = hex!("81a6737461747573b76261645f656e6372797074696f6e5f7265766973696f6e");

    let expected = authenticated_cmds::vlob_create::Rep::BadEncryptionRevision;

    let data = authenticated_cmds::vlob_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_in_maintenance() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "in_maintenance"
    let raw = hex!("81a6737461747573ae696e5f6d61696e74656e616e6365");

    let expected = authenticated_cmds::vlob_create::Rep::InMaintenance;

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

pub fn rep_bad_timestamp() {
    // Generated from Python implementation (Parsec v2.8.1+dev)
    // Content:
    //   status: "bad_timestamp"
    //
    // Note that raw data does not contain:
    //  - ballpark_client_early_offset
    //  - ballpark_client_late_offset
    //  - backend_timestamp
    //  - client_timestamp
    // This was valid behavior in api v2 but is no longer valid from v3 onwards.
    // The corresponding expected values used here are therefore not important
    // since loading raw data should fail.
    //
    let raw = hex!("81a6737461747573ad6261645f74696d657374616d70");

    let err = authenticated_cmds::vlob_create::Rep::load(&raw).unwrap_err();
    let expected_err = rmp_serde::decode::Error::Syntax("missing field `backend_timestamp`".into());

    assert!(matches!(err, expected_err));

    // Generated from Python implementation (Parsec v2.11.1+dev)
    // Content:
    //   backend_timestamp: ext(1, 946774800.0)
    //   ballpark_client_early_offset: 50.0
    //   ballpark_client_late_offset: 70.0
    //   client_timestamp: ext(1, 946774800.0)
    //   status: "bad_timestamp"
    //
    let raw = hex!(
        "85b16261636b656e645f74696d657374616d70d70141cc375188000000bc62616c6c706172"
        "6b5f636c69656e745f6561726c795f6f6666736574cb4049000000000000bb62616c6c7061"
        "726b5f636c69656e745f6c6174655f6f6666736574cb4051800000000000b0636c69656e74"
        "5f74696d657374616d70d70141cc375188000000a6737461747573ad6261645f74696d6573"
        "74616d70"
    );
    let expected = authenticated_cmds::vlob_create::Rep::BadTimestamp {
        reason: None,
        ballpark_client_early_offset: 50.,
        ballpark_client_late_offset: 70.,
        backend_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        client_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::vlob_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_not_a_sequestered_organization() {
    // Generated from Python implementation (Parsec v2.11.1+dev)
    // Content:
    //   status: "not_a_sequestered_organization"
    //
    let raw = hex!(
        "81a6737461747573be6e6f745f615f73657175657374657265645f6f7267616e697a617469"
        "6f6e"
    );

    let expected = authenticated_cmds::vlob_create::Rep::NotASequesteredOrganization;

    let data = authenticated_cmds::vlob_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_sequester_inconsistency() {
    // Generated from Python implementation (Parsec v2.11.1+dev)
    // Content:
    //   sequester_authority_certificate: hex!("666f6f626172")
    //   sequester_services_certificates: [hex!("666f6f"), hex!("626172")]
    //   status: "sequester_inconsistency"
    //
    let raw = hex!(
        "83bf7365717565737465725f617574686f726974795f6365727469666963617465c406666f"
        "6f626172bf7365717565737465725f73657276696365735f63657274696669636174657392"
        "c403666f6fc403626172a6737461747573b77365717565737465725f696e636f6e73697374"
        "656e6379"
    );

    let expected = authenticated_cmds::vlob_create::Rep::SequesterInconsistency {
        sequester_authority_certificate: b"foobar".as_ref().into(),
        sequester_services_certificates: vec![b"foo".as_ref().into(), b"bar".as_ref().into()],
    };

    let data = authenticated_cmds::vlob_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_rejected_by_sequester_service() {
    // Generated from Rust implementation (Parsec v2.15.0+dev)
    // Content:
    //   reason: "foobar"
    //   service_id: ext(2, hex!("b5eb565343c442b3a26be44573813ff0"))
    //   service_label: "foobar"
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
        service_label: "foobar".into(),
    };

    let data = authenticated_cmds::vlob_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_timeout() {
    // Generated from Rust implementation (Parsec v2.15.0+dev)
    // Content:
    //   status: "timeout"
    //
    let raw = hex!("81a6737461747573a774696d656f7574");

    let expected = authenticated_cmds::vlob_create::Rep::Timeout;

    let data = authenticated_cmds::vlob_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
