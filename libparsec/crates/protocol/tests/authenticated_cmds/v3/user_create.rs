// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;

use super::authenticated_cmds;

// Request

pub fn req() {
    let raws = [
        (
            // Generated from Python implementation (Parsec v2.11.1+dev)
            // Content:
            //   cmd: "user_create"
            //   device_certificate: hex!("666f6f626172")
            //   redacted_device_certificate: None
            //   redacted_user_certificate: hex!("666f6f626172")
            //   user_certificate: hex!("666f6f626172")
            &hex!(
                "85a3636d64ab757365725f637265617465b26465766963655f6365727469666963617465c4"
                "06666f6f626172bb72656461637465645f6465766963655f6365727469666963617465c0b9"
                "72656461637465645f757365725f6365727469666963617465c406666f6f626172b0757365"
                "725f6365727469666963617465c406666f6f626172"
            )[..]
        ),
        (
            // Generated from Python implementation (Parsec v2.11.1+dev)
            // Content:
            //   cmd: "user_create"
            //   device_certificate: hex!("666f6f626172")
            //   redacted_device_certificate: hex!("666f6f626172")
            //   redacted_user_certificate: None
            //   user_certificate: hex!("666f6f626172")
            &hex!(
                "85a3636d64ab757365725f637265617465b26465766963655f6365727469666963617465c4"
                "06666f6f626172bb72656461637465645f6465766963655f6365727469666963617465c406"
                "666f6f626172b972656461637465645f757365725f6365727469666963617465c0b0757365"
                "725f6365727469666963617465c406666f6f626172"
            )[..]
        ),
    ];

    for raw in raws {
        let err = authenticated_cmds::AnyCmdReq::load(raw).unwrap_err();
        assert!(matches!(err, rmp_serde::decode::Error::Syntax(_)));
    }

    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "user_create"
    //   device_certificate: hex!("666f6f626172")
    //   redacted_device_certificate: hex!("666f6f626172")
    //   redacted_user_certificate: hex!("666f6f626172")
    //   user_certificate: hex!("666f6f626172")
    let raw = hex!(
        "85a3636d64ab757365725f637265617465b26465766963655f6365727469666963617465c4"
        "06666f6f626172bb72656461637465645f6465766963655f6365727469666963617465c406"
        "666f6f626172b972656461637465645f757365725f6365727469666963617465c406666f6f"
        "626172b0757365725f6365727469666963617465c406666f6f626172"
    );

    let req = authenticated_cmds::user_create::Req {
        user_certificate: b"foobar".as_ref().into(),
        device_certificate: b"foobar".as_ref().into(),
        redacted_user_certificate: b"foobar".as_ref().into(),
        redacted_device_certificate: b"foobar".as_ref().into(),
    };

    let expected = authenticated_cmds::AnyCmdReq::UserCreate(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::UserCreate(req2) = data else {
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
    let raw = hex!("81a6737461747573a26f6b");

    let expected = authenticated_cmds::user_create::Rep::Ok;

    let data = authenticated_cmds::user_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_not_allowed() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "not_allowed"
    let raw = hex!("82a6726561736f6ea6666f6f626172a6737461747573ab6e6f745f616c6c6f776564");

    let expected = authenticated_cmds::user_create::Rep::NotAllowed {
        reason: Some("foobar".to_owned()),
    };

    let data = authenticated_cmds::user_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_invalid_certification() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "invalid_certification"
    let raw = hex!(
        "82a6726561736f6ea6666f6f626172a6737461747573b5696e76616c69645f636572746966"
        "69636174696f6e"
    );

    let expected = authenticated_cmds::user_create::Rep::InvalidCertification {
        reason: Some("foobar".to_owned()),
    };

    let data = authenticated_cmds::user_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_invalid_data() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "invalid_data"
    let raw = hex!("82a6726561736f6ea6666f6f626172a6737461747573ac696e76616c69645f64617461");

    let expected = authenticated_cmds::user_create::Rep::InvalidData {
        reason: Some("foobar".to_owned()),
    };

    let data = authenticated_cmds::user_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_already_exists() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "already_exists"
    let raw = hex!("82a6726561736f6ea6666f6f626172a6737461747573ae616c72656164795f657869737473");

    let expected = authenticated_cmds::user_create::Rep::AlreadyExists {
        reason: Some("foobar".to_owned()),
    };

    let data = authenticated_cmds::user_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_active_users_limit_reached() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "active_users_limit_reached"
    let raw = hex!(
        "82a6726561736f6ea6666f6f626172a6737461747573ba6163746976655f75736572735f6c"
        "696d69745f72656163686564"
    );

    let expected = authenticated_cmds::user_create::Rep::ActiveUsersLimitReached {
        reason: Some("foobar".to_owned()),
    };

    let data = authenticated_cmds::user_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_bad_timestamp() {
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

    let expected = authenticated_cmds::user_create::Rep::BadTimestamp {
        ballpark_client_early_offset: 50.,
        ballpark_client_late_offset: 70.,
        backend_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        client_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::user_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_create::Rep::load(&raw2).unwrap();

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

    let expected = authenticated_cmds::user_create::Rep::RequireGreaterTimestamp {
        strictly_greater_than: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::user_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
