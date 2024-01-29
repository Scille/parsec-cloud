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

pub fn rep_author_not_allowed() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "author_not_allowed"
    let raw = hex!("81a6737461747573b2617574686f725f6e6f745f616c6c6f776564");

    let expected = authenticated_cmds::user_create::Rep::AuthorNotAllowed;

    let data = authenticated_cmds::user_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_invalid_certificate() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "invalid_certificate"
    //
    let raw = hex!("81a6737461747573b3696e76616c69645f6365727469666963617465");

    let expected = authenticated_cmds::user_create::Rep::InvalidCertificate;

    let data = authenticated_cmds::user_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_human_handle_already_taken() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "human_handle_already_taken"
    //
    let raw = hex!("81a6737461747573ba68756d616e5f68616e646c655f616c72656164795f74616b656e");

    let expected = authenticated_cmds::user_create::Rep::HumanHandleAlreadyTaken;

    let data = authenticated_cmds::user_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_create::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_user_already_exists() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "user_already_exists"
    //
    let raw = hex!("81a6737461747573b3757365725f616c72656164795f657869737473");

    let expected = authenticated_cmds::user_create::Rep::UserAlreadyExists;

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
    //   status: "active_users_limit_reached"
    let raw = hex!(
        "82a6726561736f6ea6666f6f626172a6737461747573ba6163746976655f75736572735f6c"
        "696d69745f72656163686564"
    );

    let expected = authenticated_cmds::user_create::Rep::ActiveUsersLimitReached;

    let data = authenticated_cmds::user_create::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_create::Rep::load(&raw2).unwrap();

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

    let expected = authenticated_cmds::user_create::Rep::TimestampOutOfBallpark {
        ballpark_client_early_offset: 300.,
        ballpark_client_late_offset: 320.,
        server_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
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
