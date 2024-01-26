// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::Bytes;

use super::authenticated_cmds;

// Request

pub fn req() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   cmd: "realm_share"
    //   key_index: 8
    //   realm_role_certificate: hex!("666f6f626172")
    //   recipient_keys_bundle_access: hex!("666f6f626172")
    let raw = hex!(
        "84a3636d64ab7265616c6d5f7368617265b67265616c6d5f726f6c655f6365727469666963"
        "617465c406666f6f626172bc726563697069656e745f6b6579735f62756e646c655f616363"
        "657373c406666f6f626172a96b65795f696e64657808"
    );

    let req = authenticated_cmds::realm_share::Req {
        key_index: 8,
        realm_role_certificate: Bytes::from_static(b"foobar"),
        recipient_keys_bundle_access: Bytes::from_static(b"foobar"),
    };

    let expected = authenticated_cmds::AnyCmdReq::RealmShare(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::RealmShare(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "ok"
    let raw = hex!("81a6737461747573a26f6b");

    let expected = authenticated_cmds::realm_share::Rep::Ok;

    let data = authenticated_cmds::realm_share::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_share::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_author_not_allowed() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "author_not_allowed"
    let raw = hex!("81a6737461747573b2617574686f725f6e6f745f616c6c6f776564");

    let expected = authenticated_cmds::realm_share::Rep::AuthorNotAllowed;

    let data = authenticated_cmds::realm_share::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_share::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_invalid_certificate() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "invalid_certificate"
    //
    let raw = hex!("81a6737461747573b3696e76616c69645f6365727469666963617465");

    let expected = authenticated_cmds::realm_share::Rep::InvalidCertificate;

    let data = authenticated_cmds::realm_share::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_share::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_realm_not_found() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "realm_not_found"
    let raw = hex!("81a6737461747573af7265616c6d5f6e6f745f666f756e64");

    let expected = authenticated_cmds::realm_share::Rep::RealmNotFound;

    let data = authenticated_cmds::realm_share::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_share::Rep::load(&raw2).unwrap();

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

    let expected = authenticated_cmds::realm_share::Rep::TimestampOutOfBallpark {
        ballpark_client_early_offset: 300.,
        ballpark_client_late_offset: 320.,
        server_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        client_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::realm_share::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_share::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_require_greater_timestamp() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "require_greater_timestamp"
    //   strictly_greater_than: ext(1, 946774800.0)
    //
    let raw = hex!(
        "82a6737461747573b9726571756972655f677265617465725f74696d657374616d70b57374"
        "726963746c795f677265617465725f7468616ed70141cc375188000000"
    );

    let expected = authenticated_cmds::realm_share::Rep::RequireGreaterTimestamp {
        strictly_greater_than: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::realm_share::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_share::Rep::load(&raw2).unwrap();

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

    let expected = authenticated_cmds::realm_share::Rep::BadKeyIndex {
        last_realm_certificate_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::realm_share::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_share::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_role_already_granted() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   last_realm_certificate_timestamp: ext(1, 946774800.0)
    //   status: "role_already_granted"
    let raw = hex!(
        "82a6737461747573b4726f6c655f616c72656164795f6772616e746564d9206c6173745f72"
        "65616c6d5f63657274696669636174655f74696d657374616d70d70141cc375188000000"
    );

    let expected = authenticated_cmds::realm_share::Rep::RoleAlreadyGranted {
        last_realm_certificate_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::realm_share::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_share::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_role_incompatible_with_outsider() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "role_incompatible_with_outsider"
    let raw =
        hex!("81a6737461747573bf726f6c655f696e636f6d70617469626c655f776974685f6f75747369646572");

    let expected = authenticated_cmds::realm_share::Rep::RoleIncompatibleWithOutsider;

    let data = authenticated_cmds::realm_share::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_share::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_recipient_revoked() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "recipient_revoked"
    let raw = hex!("81a6737461747573b1726563697069656e745f7265766f6b6564");

    let expected = authenticated_cmds::realm_share::Rep::RecipientRevoked;

    let data = authenticated_cmds::realm_share::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_share::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_recipient_not_found() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "recipient_not_found"
    let raw = hex!("81a6737461747573b3726563697069656e745f6e6f745f666f756e64");

    let expected = authenticated_cmds::realm_share::Rep::RecipientNotFound;

    let data = authenticated_cmds::realm_share::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_share::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
