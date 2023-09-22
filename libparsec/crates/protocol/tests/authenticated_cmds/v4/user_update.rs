// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::authenticated_cmds;

// Request

pub fn req() {
    // Generated from Rust implementation (Parsec v2.16.0-a.0+dev)
    // Content:
    //   cmd: "user_update"
    //   user_update_certificate: hex!("666f6f626172")
    let raw = hex!(
        "82a3636d64ab757365725f757064617465b7757365725f7570646174655f63657274696669"
        "63617465c406666f6f626172"
    );

    let req = authenticated_cmds::user_update::Req {
        user_update_certificate: Bytes::from_static(b"foobar"),
    };

    let expected = authenticated_cmds::AnyCmdReq::UserUpdate(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::UserUpdate(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Rust implementation (Parsec v2.16.0-a.0+dev)
    // Content:
    //   status: "ok"
    let raw = hex!("81a6737461747573a26f6b");

    let expected = authenticated_cmds::user_update::Rep::Ok;

    let data = authenticated_cmds::user_update::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_update::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_not_allowed() {
    // Generated from Rust implementation (Parsec v2.16.0-a.0+dev)
    // Content:
    //   status: "not_allowed"
    let raw = hex!("81a6737461747573ab6e6f745f616c6c6f776564");

    let expected = authenticated_cmds::user_update::Rep::NotAllowed;

    let data = authenticated_cmds::user_update::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_update::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_invalid_certification() {
    // Generated from Rust implementation (Parsec v2.16.0-a.0+dev)
    // Content:
    //   status: "invalid_certification"
    let raw = hex!("81a6737461747573b5696e76616c69645f63657274696669636174696f6e");

    let expected = authenticated_cmds::user_update::Rep::InvalidCertification;

    let data = authenticated_cmds::user_update::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_update::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_invalid_data() {
    // Generated from Rust implementation (Parsec v2.16.0-a.0+dev)
    // Content:
    //   status: "invalid_data"
    let raw = hex!("81a6737461747573ac696e76616c69645f64617461");

    let expected = authenticated_cmds::user_update::Rep::InvalidData;

    let data = authenticated_cmds::user_update::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_update::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_already_exists() {
    // Generated from Rust implementation (Parsec v2.16.0-a.0+dev)
    // Content:
    //   status: "already_exists"
    let raw = hex!("81a6737461747573ae616c72656164795f657869737473");

    let expected = authenticated_cmds::user_update::Rep::AlreadyExists;

    let data = authenticated_cmds::user_update::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_update::Rep::load(&raw2).unwrap();

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

    let expected = authenticated_cmds::user_update::Rep::BadTimestamp {
        ballpark_client_early_offset: 50.,
        ballpark_client_late_offset: 70.,
        backend_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        client_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::user_update::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_update::Rep::load(&raw2).unwrap();

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

    let expected = authenticated_cmds::user_update::Rep::RequireGreaterTimestamp {
        strictly_greater_than: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::user_update::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_update::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
