// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::authenticated_cmds;

// Request

pub fn req() {
    let raw_expected = [
        (
            // Generated from Rust implementation (Parsec v3.0.0+dev)
            // Content:
            //   cmd: "vlob_read_batch"
            //   realm_id: ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))
            //   vlobs: []
            &hex!(
                "84a3636d64af766c6f625f726561645f6261746368a87265616c6d5f6964d8022b5f314728"
                "134a12863da1ce49c112f6a5766c6f627390a26174c0"
            )[..],
            authenticated_cmds::vlob_read_batch::Req {
                at: None,
                realm_id: VlobID::from_hex("2b5f314728134a12863da1ce49c112f6").unwrap(),
                vlobs: vec![],
            },
        ),
        (
            // Generated from Rust implementation (Parsec v3.0.0+dev)
            // Content:
            //   cmd: "vlob_read_batch"
            //   at: ext(1, 946774800.0)
            //   realm_id: ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))
            //   vlobs: [ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))]
            &hex!(
                "84a3636d64af766c6f625f726561645f6261746368a87265616c6d5f6964d8022b5f314728"
                "134a12863da1ce49c112f6a5766c6f627391d8022b5f314728134a12863da1ce49c112f6a2"
                "6174d70141cc375188000000"
            )[..],
            authenticated_cmds::vlob_read_batch::Req {
                at: Some("2000-1-2T01:00:00Z".parse().unwrap()),
                realm_id: VlobID::from_hex("2b5f314728134a12863da1ce49c112f6").unwrap(),
                vlobs: vec![VlobID::from_hex("2b5f314728134a12863da1ce49c112f6").unwrap()],
            },
        ),
    ];

    for (raw, expected) in raw_expected {
        let expected = authenticated_cmds::AnyCmdReq::VlobReadBatch(expected);

        let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();

        p_assert_eq!(data, expected);

        // Also test serialization round trip
        let authenticated_cmds::AnyCmdReq::VlobReadBatch(req2) = data else {
            unreachable!()
        };

        let raw2 = req2.dump().unwrap();

        let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

        p_assert_eq!(data2, expected);
    }
}

// Responses

pub fn rep_ok() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "ok"
    //   items: [(
    //      ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5")),
    //      8,
    //      "alice@dev1",
    //      1,
    //      ext(1, 946774800.0),
    //      hex!("666f6f626172"),
    //   )]
    //   needed_common_certificate_timestamp: ext(1, 946774800.0)
    //   needed_realm_certificate_timestamp: ext(1, 946774800.0)
    //
    let raw = hex!(
        "84a6737461747573a26f6ba56974656d739196d8022b5f314728134a12863da1ce49c112f6"
        "08aa616c696365406465763101d70141cc375188000000c406666f6f626172d9236e656564"
        "65645f636f6d6d6f6e5f63657274696669636174655f74696d657374616d70d70141cc3751"
        "88000000d9226e65656465645f7265616c6d5f63657274696669636174655f74696d657374"
        "616d70d70141cc375188000000"
    );

    let expected = authenticated_cmds::vlob_read_batch::Rep::Ok {
        items: vec![(
            VlobID::from_hex("2b5f314728134a12863da1ce49c112f6").unwrap(),
            8,
            "alice@dev1".parse().unwrap(),
            1,
            "2000-1-2T01:00:00Z".parse().unwrap(),
            Bytes::from_static(b"foobar"),
        )],
        needed_common_certificate_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        needed_realm_certificate_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::vlob_read_batch::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_read_batch::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_realm_not_found() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "realm_not_found"
    let raw = hex!("81a6737461747573af7265616c6d5f6e6f745f666f756e64");

    let expected = authenticated_cmds::vlob_read_batch::Rep::RealmNotFound;

    let data = authenticated_cmds::vlob_read_batch::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_read_batch::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_author_not_allowed() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "author_not_allowed"
    let raw = hex!("81a6737461747573b2617574686f725f6e6f745f616c6c6f776564");

    let expected = authenticated_cmds::vlob_read_batch::Rep::AuthorNotAllowed;

    let data = authenticated_cmds::vlob_read_batch::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_read_batch::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_too_many_elements() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "too_many_elements"
    let raw = hex!("81a6737461747573b1746f6f5f6d616e795f656c656d656e7473");

    let expected = authenticated_cmds::vlob_read_batch::Rep::TooManyElements;

    let data = authenticated_cmds::vlob_read_batch::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_read_batch::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
