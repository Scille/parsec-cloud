// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;

use super::authenticated_cmds;

// Request

pub fn req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "user_revoke"
    //   revoked_user_certificate: hex!("666f6f626172")
    let raw = hex!(
        "82a3636d64ab757365725f7265766f6b65b87265766f6b65645f757365725f636572746966"
        "6963617465c406666f6f626172"
    );

    let req = authenticated_cmds::user_revoke::Req {
        revoked_user_certificate: b"foobar".as_ref().into(),
    };

    let expected = authenticated_cmds::AnyCmdReq::UserRevoke(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::UserRevoke(req2) = data else {
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

    let expected = authenticated_cmds::user_revoke::Rep::Ok;

    let data = authenticated_cmds::user_revoke::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_revoke::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_not_allowed() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "not_allowed"
    let raw = hex!("82a6726561736f6ea6666f6f626172a6737461747573ab6e6f745f616c6c6f776564");

    let expected = authenticated_cmds::user_revoke::Rep::NotAllowed {
        reason: Some("foobar".to_owned()),
    };

    let data = authenticated_cmds::user_revoke::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_revoke::Rep::load(&raw2).unwrap();

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

    let expected = authenticated_cmds::user_revoke::Rep::InvalidCertification {
        reason: Some("foobar".to_owned()),
    };

    let data = authenticated_cmds::user_revoke::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_revoke::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_not_found() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    let raw = hex!("81a6737461747573a96e6f745f666f756e64");

    let expected = authenticated_cmds::user_revoke::Rep::NotFound;

    let data = authenticated_cmds::user_revoke::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_revoke::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_already_revoked() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "already_revoked"
    let raw = hex!(
        "82a6726561736f6ea6666f6f626172a6737461747573af616c72656164795f7265766f6b65"
        "64"
    );

    let expected = authenticated_cmds::user_revoke::Rep::AlreadyRevoked {
        reason: Some("foobar".to_owned()),
    };

    let data = authenticated_cmds::user_revoke::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_revoke::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_bad_timestamp() {
    // Generated from Rust implementation (Parsec v2.15.0+dev)
    // Content:
    //   backend_timestamp: ext(1, 946774800.0)
    //   ballpark_client_early_offset: 50.0
    //   ballpark_client_late_offset: 70.0
    //   client_timestamp: ext(1, 946774800.0)
    //   status: "bad_timestamp"
    //
    let raw = hex!(
        "86a6737461747573ad6261645f74696d657374616d70b16261636b656e645f74696d657374"
        "616d70d70141cc375188000000bc62616c6c7061726b5f636c69656e745f6561726c795f6f"
        "6666736574cb4049000000000000bb62616c6c7061726b5f636c69656e745f6c6174655f6f"
        "6666736574cb4051800000000000b0636c69656e745f74696d657374616d70d70141cc3751"
        "88000000a6726561736f6ec0"
    );

    let expected = authenticated_cmds::user_revoke::Rep::BadTimestamp {
        reason: None,
        ballpark_client_early_offset: 50.,
        ballpark_client_late_offset: 70.,
        backend_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        client_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::user_revoke::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_revoke::Rep::load(&raw2).unwrap();

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

    let expected = authenticated_cmds::user_revoke::Rep::RequireGreaterTimestamp {
        strictly_greater_than: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::user_revoke::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_revoke::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
