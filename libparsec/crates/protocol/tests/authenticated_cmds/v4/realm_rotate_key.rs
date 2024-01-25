// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::collections::HashMap;

use libparsec_tests_lite::prelude::*;
use libparsec_types::Bytes;

use super::authenticated_cmds;

// Request

pub fn req() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   cmd: "realm_rotate_key"
    //   keys_bundle: hex!("666f6f626172")
    //   never_legacy_reencrypted_or_fail: false
    //   per_participant_keys_bundle_access: {"alice":hex!("666f6f626172")}
    //   realm_key_rotation_certificate: hex!("666f6f626172")
    //
    let raw = hex!(
        "85a3636d64b07265616c6d5f726f746174655f6b6579be7265616c6d5f6b65795f726f7461"
        "74696f6e5f6365727469666963617465c406666f6f626172d9227065725f70617274696369"
        "70616e745f6b6579735f62756e646c655f61636365737381a5616c696365c406666f6f6261"
        "72ab6b6579735f62756e646c65c406666f6f626172d9206e657665725f6c65676163795f72"
        "65656e637279707465645f6f725f6661696cc2"
    );

    let req = authenticated_cmds::realm_rotate_key::Req {
        keys_bundle: Bytes::from_static(b"foobar"),
        never_legacy_reencrypted_or_fail: false,
        per_participant_keys_bundle_access: HashMap::from([(
            "alice".parse().unwrap(),
            Bytes::from_static(b"foobar"),
        )]),
        realm_key_rotation_certificate: Bytes::from_static(b"foobar"),
    };

    let expected = authenticated_cmds::AnyCmdReq::RealmRotateKey(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::RealmRotateKey(req2) = data else {
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

    let expected = authenticated_cmds::realm_rotate_key::Rep::Ok;

    let data = authenticated_cmds::realm_rotate_key::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_rotate_key::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_author_not_allowed() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "author_not_allowed"
    let raw = hex!("81a6737461747573b2617574686f725f6e6f745f616c6c6f776564");

    let expected = authenticated_cmds::realm_rotate_key::Rep::AuthorNotAllowed;

    let data = authenticated_cmds::realm_rotate_key::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_rotate_key::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_invalid_certificate() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "invalid_certificate"
    //
    let raw = hex!("81a6737461747573b3696e76616c69645f6365727469666963617465");

    let expected = authenticated_cmds::realm_rotate_key::Rep::InvalidCertificate;

    let data = authenticated_cmds::realm_rotate_key::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_rotate_key::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_realm_not_found() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "realm_not_found"
    let raw = hex!("81a6737461747573af7265616c6d5f6e6f745f666f756e64");

    let expected = authenticated_cmds::realm_rotate_key::Rep::RealmNotFound;

    let data = authenticated_cmds::realm_rotate_key::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_rotate_key::Rep::load(&raw2).unwrap();

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

    let expected = authenticated_cmds::realm_rotate_key::Rep::TimestampOutOfBallpark {
        ballpark_client_early_offset: 300.,
        ballpark_client_late_offset: 320.,
        server_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        client_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::realm_rotate_key::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_rotate_key::Rep::load(&raw2).unwrap();

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

    let expected = authenticated_cmds::realm_rotate_key::Rep::RequireGreaterTimestamp {
        strictly_greater_than: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::realm_rotate_key::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_rotate_key::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_bad_key_index() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "bad_key_index"
    let raw = hex!("81a6737461747573ad6261645f6b65795f696e646578");

    let expected = authenticated_cmds::realm_rotate_key::Rep::BadKeyIndex;

    let data = authenticated_cmds::realm_rotate_key::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_rotate_key::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_participant_mismatch() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "participant_mismatch"
    let raw = hex!("81a6737461747573b47061727469636970616e745f6d69736d61746368");

    let expected = authenticated_cmds::realm_rotate_key::Rep::ParticipantMismatch;

    let data = authenticated_cmds::realm_rotate_key::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = expected.dump().unwrap();

    let data2 = authenticated_cmds::realm_rotate_key::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_legacy_reencrypted_realm() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "legacy_reencrypted_realm"
    //   encryption_revision: 8
    let raw = hex!(
        "82a6737461747573b86c65676163795f7265656e637279707465645f7265616c6db3656e63"
        "72797074696f6e5f7265766973696f6e08"
    );

    let expected = authenticated_cmds::realm_rotate_key::Rep::LegacyReencryptedRealm {
        encryption_revision: 8,
    };

    let data = authenticated_cmds::realm_rotate_key::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_rotate_key::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
