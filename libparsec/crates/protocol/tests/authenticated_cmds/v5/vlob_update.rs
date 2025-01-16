// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::authenticated_cmds;

// Request

pub fn req() {
    // Generated from Parsec 3.2.4-a.0+dev
    // Content:
    //   cmd: 'vlob_update'
    //   realm_id: ext(2, 0x1d3353157d7d4e95ad2fdea7b3bd19c5)
    //   vlob_id: ext(2, 0x2b5f314728134a12863da1ce49c112f6)
    //   key_index: 8
    //   timestamp: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
    //   version: 8
    //   blob: 0x666f6f626172
    let raw: &[u8] = hex!(
        "87a3636d64ab766c6f625f757064617465a87265616c6d5f6964d8021d3353157d7d4e"
        "95ad2fdea7b3bd19c5a7766c6f625f6964d8022b5f314728134a12863da1ce49c112f6"
        "a96b65795f696e64657808a974696d657374616d70d70100035d162fa2e400a7766572"
        "73696f6e08a4626c6f62c406666f6f626172"
    )
    .as_ref();

    let expected = authenticated_cmds::vlob_update::Req {
        realm_id: VlobID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
        key_index: 8,
        vlob_id: VlobID::from_hex("2b5f314728134a12863da1ce49c112f6").unwrap(),
        timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        version: 8,
        blob: b"foobar".as_ref().into(),
    };

    let expected = authenticated_cmds::AnyCmdReq::VlobUpdate(expected);

    let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::VlobUpdate(req2) = data else {
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

    let expected = authenticated_cmds::vlob_update::Rep::Ok;

    let data = authenticated_cmds::vlob_update::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_update::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_author_not_allowed() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "author_not_allowed"
    let raw = hex!("81a6737461747573b2617574686f725f6e6f745f616c6c6f776564");

    let expected = authenticated_cmds::vlob_update::Rep::AuthorNotAllowed;

    let data = authenticated_cmds::vlob_update::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_update::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_realm_not_found() {
    // Generated from Parsec 3.2.4-a.0+dev
    // Content:
    //   status: 'realm_not_found'
    let raw = hex!("81a6737461747573af7265616c6d5f6e6f745f666f756e64");

    let expected = authenticated_cmds::vlob_update::Rep::RealmNotFound;

    let data = authenticated_cmds::vlob_update::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_update::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_vlob_not_found() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "vlob_not_found"
    let raw = hex!("81a6737461747573ae766c6f625f6e6f745f666f756e64");

    let expected = authenticated_cmds::vlob_update::Rep::VlobNotFound;

    let data = authenticated_cmds::vlob_update::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_update::Rep::load(&raw2).unwrap();

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

    let expected = authenticated_cmds::vlob_update::Rep::BadKeyIndex {
        last_realm_certificate_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::vlob_update::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_update::Rep::load(&raw2).unwrap();

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

    let expected = authenticated_cmds::vlob_update::Rep::RequireGreaterTimestamp {
        strictly_greater_than: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::vlob_update::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_update::Rep::load(&raw2).unwrap();

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

    let expected = authenticated_cmds::vlob_update::Rep::TimestampOutOfBallpark {
        ballpark_client_early_offset: 300.,
        ballpark_client_late_offset: 320.,
        server_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        client_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::vlob_update::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_update::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_rejected_by_sequester_service() {
    // Generated from Parsec 3.1.1-a.0+dev
    // Content:
    //   status: 'rejected_by_sequester_service'
    //   reason: 'foobar'
    //   service_id: ext(2, 0xb5eb565343c442b3a26be44573813ff0)
    let raw: &[u8] = hex!(
        "83a6737461747573bd72656a65637465645f62795f7365717565737465725f73657276"
        "696365a6726561736f6ea6666f6f626172aa736572766963655f6964d802b5eb565343"
        "c442b3a26be44573813ff0"
    )
    .as_ref();

    let expected = authenticated_cmds::vlob_update::Rep::RejectedBySequesterService {
        reason: Some("foobar".to_string()),
        service_id: SequesterServiceID::from_hex("b5eb565343c442b3a26be44573813ff0").unwrap(),
    };

    let data = authenticated_cmds::vlob_update::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_update::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);

    // Also test with no provided reason

    // Generated from Parsec 3.1.1-a.0+dev
    // Content:
    //   status: 'rejected_by_sequester_service'
    //   reason: None
    //   service_id: ext(2, 0xb5eb565343c442b3a26be44573813ff0)
    let raw: &[u8] = hex!(
        "83a6737461747573bd72656a65637465645f62795f7365717565737465725f73657276"
        "696365a6726561736f6ec0aa736572766963655f6964d802b5eb565343c442b3a26be4"
        "4573813ff0"
    )
    .as_ref();

    let expected = authenticated_cmds::vlob_update::Rep::RejectedBySequesterService {
        reason: None,
        service_id: SequesterServiceID::from_hex("b5eb565343c442b3a26be44573813ff0").unwrap(),
    };

    let data = authenticated_cmds::vlob_update::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_update::Rep::load(&raw2).unwrap();

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

    let expected = authenticated_cmds::vlob_update::Rep::SequesterServiceUnavailable {
        service_id: SequesterServiceID::from_hex("c6da35d4f7cb4870b9f3a16e21ff1caf").unwrap(),
    };

    let data = authenticated_cmds::vlob_update::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_update::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_bad_vlob_version() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "bad_vlob_version"
    //
    let raw = hex!("81a6737461747573b06261645f766c6f625f76657273696f6e");

    let expected = authenticated_cmds::vlob_update::Rep::BadVlobVersion;

    let data = authenticated_cmds::vlob_update::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_update::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
