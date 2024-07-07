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
            // Generated from Parsec v3.0.0-b.11+dev
            // Content:
            //   cmd: "certificate_get"
            //   common_after: None
            //   realm_after: {}
            //   sequester_after: None
            //   shamir_recovery_after: None
            &hex!(
                "85a3636d64af63657274696669636174655f676574ac636f6d6d6f6e5f6166746572c0"
                "af7365717565737465725f6166746572c0b57368616d69725f7265636f766572795f61"
                "66746572c0ab7265616c6d5f616674657280"
            )[..],
            authenticated_cmds::certificate_get::Req {
                common_after: None,
                realm_after: HashMap::new(),
                sequester_after: None,
                shamir_recovery_after: None,
            },
        ),
        (
            // Generated from Parsec v3.0.0-b.11+dev
            // Content:
            //   cmd: "certificate_get"
            //   common_after: ext(1, 946774800.0)
            //   realm_after: {ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5")):ext(1, 946774800.0)}
            //   sequester_after: ext(1, 946774800.0)
            //   shamir_recovery_after: ext(1, 946774800.0)
            &hex!(
                "85a3636d64af63657274696669636174655f676574ac636f6d6d6f6e5f6166746572d7"
                "0100035d162fa2e400af7365717565737465725f6166746572d70100035d162fa2e400"
                "b57368616d69725f7265636f766572795f6166746572d70100035d162fa2e400ab7265"
                "616c6d5f616674657281d8021d3353157d7d4e95ad2fdea7b3bd19c5d70100035d162f"
                "a2e400"
            )[..],
            authenticated_cmds::certificate_get::Req {
                common_after: Some("2000-1-2T01:00:00Z".parse().unwrap()),
                realm_after: HashMap::from([(
                    VlobID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
                    "2000-1-2T01:00:00Z".parse().unwrap(),
                )]),
                sequester_after: Some("2000-1-2T01:00:00Z".parse().unwrap()),
                shamir_recovery_after: Some("2000-1-2T01:00:00Z".parse().unwrap()),
            },
        ),
    ];

    for (raw, expected) in raw_expected {
        let expected = authenticated_cmds::AnyCmdReq::CertificateGet(expected);

        let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();
        p_assert_eq!(data, expected);

        // Also test serialization round trip
        let authenticated_cmds::AnyCmdReq::CertificateGet(req2) = data else {
            unreachable!()
        };

        let raw2 = req2.dump().unwrap();

        let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

        p_assert_eq!(data2, expected);
    }
}

// // Responses

pub fn rep_ok() {
    let raw_expected = [
        (
            // Generated from Rust implementation (Parsec v3.0.0+dev)
            // Content:
            //   common_certificates: [hex!("666f6f626172")]
            //   realm_certificates: {ext(2, hex!("2b5f314728134a12863da1ce49c112f6")):[hex!("666f6f626172")]}
            //   sequester_certificates: [hex!("666f6f626172")]
            //   shamir_recovery_certificates: [hex!("666f6f626172")]
            //   status: "ok"
            &hex!(
                "85a6737461747573a26f6bb3636f6d6d6f6e5f63657274696669636174657391c40666"
                "6f6f626172b27265616c6d5f63657274696669636174657381d8022b5f314728134a12"
                "863da1ce49c112f691c406666f6f626172b67365717565737465725f63657274696669"
                "636174657391c406666f6f626172bc7368616d69725f7265636f766572795f63657274"
                "696669636174657391c406666f6f626172"
            )[..],
            authenticated_cmds::certificate_get::Rep::Ok {
                common_certificates: vec![Bytes::from_static(b"foobar")],
                realm_certificates: HashMap::from([(
                    VlobID::from_hex("2b5f314728134a12863da1ce49c112f6").unwrap(),
                    vec![Bytes::from_static(b"foobar")],
                )]),
                sequester_certificates: vec![Bytes::from_static(b"foobar")],
                shamir_recovery_certificates: vec![Bytes::from_static(b"foobar")],
            },
        ),
        (
            // Generated from Rust implementation (Parsec v3.0.0+dev)
            // Content:
            //   common_certificates: []
            //   realm_certificates: {}
            //   sequester_certificates: []
            //   shamir_recovery_certificates: []
            //   status: "ok"
            &hex!(
                "85a6737461747573a26f6bb3636f6d6d6f6e5f63657274696669636174657390b27265"
                "616c6d5f63657274696669636174657380b67365717565737465725f63657274696669"
                "636174657390bc7368616d69725f7265636f766572795f636572746966696361746573"
                "90"
            )[..],
            authenticated_cmds::certificate_get::Rep::Ok {
                common_certificates: vec![],
                realm_certificates: HashMap::new(),
                sequester_certificates: vec![],
                shamir_recovery_certificates: vec![],
            },
        ),
    ];

    for (raw, expected) in raw_expected {
        let data = authenticated_cmds::certificate_get::Rep::load(raw).unwrap();

        p_assert_eq!(data, expected);

        // Also test serialization round trip
        let raw2 = data.dump().unwrap();

        let data2 = authenticated_cmds::certificate_get::Rep::load(&raw2).unwrap();

        p_assert_eq!(data2, expected);
    }
}
