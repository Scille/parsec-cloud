// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::collections::HashMap;

use libparsec_tests_lite::prelude::*;
use libparsec_types::{Bytes, SequesterServiceID};

use super::authenticated_cmds;

// Request

fn req_sequestered() {
    // Generated from Parsec 3.1.1-a.0+dev
    // Content:
    //   cmd: 'realm_rotate_key'
    //   realm_key_rotation_certificate: 0x3c6365727469663e
    //   per_participant_keys_bundle_access: {
    //     ext(2, 0xa11cec00100000000000000000000000): 0x3c616c696365206163636573733e,
    //   }
    //   per_sequester_service_keys_bundle_access: {
    //     ext(2, 0xb5eb565343c442b3a26be44573813ff0): 0x3c736571756573746572206163636573733e,
    //   }
    //   keys_bundle: 0x3c6b6579735f62756e646c653e
    let raw: &[u8] = hex!(
        "85a3636d64b07265616c6d5f726f746174655f6b6579be7265616c6d5f6b65795f726f"
        "746174696f6e5f6365727469666963617465c4083c6365727469663ed9227065725f70"
        "61727469636970616e745f6b6579735f62756e646c655f61636365737381d802a11cec"
        "00100000000000000000000000c40e3c616c696365206163636573733ed9287065725f"
        "7365717565737465725f736572766963655f6b6579735f62756e646c655f6163636573"
        "7381d802b5eb565343c442b3a26be44573813ff0c4123c736571756573746572206163"
        "636573733eab6b6579735f62756e646c65c40d3c6b6579735f62756e646c653e"
    )
    .as_ref();

    let req = authenticated_cmds::realm_rotate_key::Req {
        keys_bundle: Bytes::from_static(b"<keys_bundle>"),
        per_participant_keys_bundle_access: HashMap::from([(
            "alice".parse().unwrap(),
            Bytes::from_static(b"<alice access>"),
        )]),
        realm_key_rotation_certificate: Bytes::from_static(b"<certif>"),
        per_sequester_service_keys_bundle_access: Some(HashMap::from_iter([(
            SequesterServiceID::from_hex("b5eb565343c442b3a26be44573813ff0").unwrap(),
            Bytes::from_static(b"<sequester access>"),
        )])),
    };

    let expected = authenticated_cmds::AnyCmdReq::RealmRotateKey(req);

    let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::RealmRotateKey(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

fn req_not_sequestered() {
    // Generated from Parsec 3.1.1-a.0+dev
    // Content:
    //   cmd: 'realm_rotate_key'
    //   realm_key_rotation_certificate: 0x3c6365727469663e
    //   per_participant_keys_bundle_access: {
    //     ext(2, 0xa11cec00100000000000000000000000): 0x3c616c696365206163636573733e,
    //   }
    //   per_sequester_service_keys_bundle_access: None
    //   keys_bundle: 0x3c6b6579735f62756e646c653e
    let raw: &[u8] = hex!(
        "85a3636d64b07265616c6d5f726f746174655f6b6579be7265616c6d5f6b65795f726f"
        "746174696f6e5f6365727469666963617465c4083c6365727469663ed9227065725f70"
        "61727469636970616e745f6b6579735f62756e646c655f61636365737381d802a11cec"
        "00100000000000000000000000c40e3c616c696365206163636573733ed9287065725f"
        "7365717565737465725f736572766963655f6b6579735f62756e646c655f6163636573"
        "73c0ab6b6579735f62756e646c65c40d3c6b6579735f62756e646c653e"
    )
    .as_ref();

    let req = authenticated_cmds::realm_rotate_key::Req {
        keys_bundle: Bytes::from_static(b"<keys_bundle>"),
        per_participant_keys_bundle_access: HashMap::from([(
            "alice".parse().unwrap(),
            Bytes::from_static(b"<alice access>"),
        )]),
        realm_key_rotation_certificate: Bytes::from_static(b"<certif>"),
        per_sequester_service_keys_bundle_access: None,
    };

    let expected = authenticated_cmds::AnyCmdReq::RealmRotateKey(req);

    let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::RealmRotateKey(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn req() {
    req_sequestered();
    req_not_sequestered();
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
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   ballpark_client_early_offset: 300.0
    //   ballpark_client_late_offset: 320.0
    //   client_timestamp: ext(1, 946774800.0)
    //   server_timestamp: ext(1, 946774800.0)
    //   status: "timestamp_out_of_ballpark"
    //
    let raw = hex!(
        "85a6737461747573b974696d657374616d705f6f75745f6f665f62616c6c7061726bbc"
        "62616c6c7061726b5f636c69656e745f6561726c795f6f6666736574cb4072c0000000"
        "0000bb62616c6c7061726b5f636c69656e745f6c6174655f6f6666736574cb40740000"
        "00000000b0636c69656e745f74696d657374616d70d70100035d162fa2e400b0736572"
        "7665725f74696d657374616d70d70100035d162fa2e400"
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
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   status: "require_greater_timestamp"
    //   strictly_greater_than: ext(1, 946774800.0)
    //
    let raw = hex!(
        "82a6737461747573b9726571756972655f677265617465725f74696d657374616d70b5"
        "7374726963746c795f677265617465725f7468616ed70100035d162fa2e400"
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
    // Generated from Parsec v3.0.0-b.11+dev
    // Content:
    //   last_realm_certificate_timestamp: ext(1, 946774800.0)
    //   status: "bad_key_index"
    let raw = hex!(
        "82a6737461747573ad6261645f6b65795f696e646578d9206c6173745f7265616c6d5f"
        "63657274696669636174655f74696d657374616d70d70100035d162fa2e400"
    );

    let expected = authenticated_cmds::realm_rotate_key::Rep::BadKeyIndex {
        last_realm_certificate_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::realm_rotate_key::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_rotate_key::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_participant_mismatch() {
    // Generated from Parsec 3.1.1-a.0+dev
    // Content:
    //   status: 'participant_mismatch'
    //   last_realm_certificate_timestamp: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
    let raw: &[u8] = hex!(
        "82a6737461747573b47061727469636970616e745f6d69736d61746368d9206c617374"
        "5f7265616c6d5f63657274696669636174655f74696d657374616d70d70100035d162f"
        "a2e400"
    )
    .as_ref();

    let expected = authenticated_cmds::realm_rotate_key::Rep::ParticipantMismatch {
        last_realm_certificate_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::realm_rotate_key::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = expected.dump().unwrap();

    let data2 = authenticated_cmds::realm_rotate_key::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_sequester_service_mismatch() {
    // Generated from Parsec 3.1.1-a.0+dev
    // Content:
    //   status: 'sequester_service_mismatch'
    //   last_sequester_certificate_timestamp: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
    let raw: &[u8] = hex!(
        "82a6737461747573ba7365717565737465725f736572766963655f6d69736d61746368"
        "d9246c6173745f7365717565737465725f63657274696669636174655f74696d657374"
        "616d70d70100035d162fa2e400"
    )
    .as_ref();

    let expected = authenticated_cmds::realm_rotate_key::Rep::SequesterServiceMismatch {
        last_sequester_certificate_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::realm_rotate_key::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = expected.dump().unwrap();

    let data2 = authenticated_cmds::realm_rotate_key::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_rejected_by_sequester_service() {
    // Generated from Parsec 3.1.1-a.0+dev
    // Content:
    //   status: 'rejected_by_sequester_service'
    //   reason: None
    //   service_id: ext(2, 0xc6da35d4f7cb4870b9f3a16e21ff1caf)
    let raw: &[u8] = hex!(
        "83a6737461747573bd72656a65637465645f62795f7365717565737465725f73657276"
        "696365a6726561736f6ec0aa736572766963655f6964d802c6da35d4f7cb4870b9f3a1"
        "6e21ff1caf"
    )
    .as_ref();

    let expected = authenticated_cmds::realm_rotate_key::Rep::RejectedBySequesterService {
        reason: None,
        service_id: SequesterServiceID::from_hex("c6da35d4f7cb4870b9f3a16e21ff1caf").unwrap(),
    };

    let data = authenticated_cmds::realm_rotate_key::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = expected.dump().unwrap();

    let data2 = authenticated_cmds::realm_rotate_key::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);

    // Also test with `reason` field provided

    // Generated from Parsec 3.1.1-a.0+dev
    // Content:
    //   status: 'rejected_by_sequester_service'
    //   reason: 'Rejected'
    //   service_id: ext(2, 0xc6da35d4f7cb4870b9f3a16e21ff1caf)
    let raw: &[u8] = hex!(
        "83a6737461747573bd72656a65637465645f62795f7365717565737465725f73657276"
        "696365a6726561736f6ea852656a6563746564aa736572766963655f6964d802c6da35"
        "d4f7cb4870b9f3a16e21ff1caf"
    )
    .as_ref();

    let expected = authenticated_cmds::realm_rotate_key::Rep::RejectedBySequesterService {
        reason: Some("Rejected".to_string()),
        service_id: SequesterServiceID::from_hex("c6da35d4f7cb4870b9f3a16e21ff1caf").unwrap(),
    };

    let data = authenticated_cmds::realm_rotate_key::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = expected.dump().unwrap();

    let data2 = authenticated_cmds::realm_rotate_key::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_sequester_service_unavailable() {
    // Generated from Parsec 3.1.1-a.0+dev
    // Content:
    //   status: 'sequester_service_unavailable'
    //   service_id: ext(2, 0xc6da35d4f7cb4870b9f3a16e21ff1caf)
    let raw: &[u8] = hex!(
        "82a6737461747573bd7365717565737465725f736572766963655f756e617661696c61"
        "626c65aa736572766963655f6964d802c6da35d4f7cb4870b9f3a16e21ff1caf"
    )
    .as_ref();

    let expected = authenticated_cmds::realm_rotate_key::Rep::SequesterServiceUnavailable {
        service_id: SequesterServiceID::from_hex("c6da35d4f7cb4870b9f3a16e21ff1caf").unwrap(),
    };

    let data = authenticated_cmds::realm_rotate_key::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = expected.dump().unwrap();

    let data2 = authenticated_cmds::realm_rotate_key::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_organization_not_sequestered() {
    // Generated from Parsec 3.1.1-a.0+dev
    // Content:
    //   status: 'organization_not_sequestered'
    let raw: &[u8] = hex!(
        "81a6737461747573bc6f7267616e697a6174696f6e5f6e6f745f736571756573746572"
        "6564"
    )
    .as_ref();

    let expected = authenticated_cmds::realm_rotate_key::Rep::OrganizationNotSequestered;

    let data = authenticated_cmds::realm_rotate_key::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = expected.dump().unwrap();

    let data2 = authenticated_cmds::realm_rotate_key::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
