// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::Bytes;

use super::authenticated_cmds;

// Request

pub fn req() {
    // Generated from Parsec 3.8.1-a.0+dev
    // Content:
    //   cmd: 'realm_update_archiving'
    //   archiving_certificate: 0x666f6f626172
    let raw = hex!(
        "82a3636d64b67265616c6d5f7570646174655f617263686976696e67b5617263686976"
        "696e675f6365727469666963617465c406666f6f626172"
    );

    let req = authenticated_cmds::realm_update_archiving::Req {
        archiving_certificate: Bytes::from_static(b"foobar"),
    };
    println!("***expected: {:?}", req.dump().unwrap());

    let expected = authenticated_cmds::AnyCmdReq::RealmUpdateArchiving(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::RealmUpdateArchiving(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.8.1-a.0+dev
    // Content:
    //   status: 'ok'
    let raw = hex!("81a6737461747573a26f6b");

    let expected = authenticated_cmds::realm_update_archiving::Rep::Ok;
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = authenticated_cmds::realm_update_archiving::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_update_archiving::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_author_not_allowed() {
    // Generated from Parsec 3.8.1-a.0+dev
    // Content:
    //   status: 'author_not_allowed'
    let raw = hex!("81a6737461747573b2617574686f725f6e6f745f616c6c6f776564");

    let expected = authenticated_cmds::realm_update_archiving::Rep::AuthorNotAllowed;
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = authenticated_cmds::realm_update_archiving::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_update_archiving::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_invalid_certificate() {
    // Generated from Parsec 3.8.1-a.0+dev
    // Content:
    //   status: 'invalid_certificate'
    let raw = hex!("81a6737461747573b3696e76616c69645f6365727469666963617465");

    let expected = authenticated_cmds::realm_update_archiving::Rep::InvalidCertificate;
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = authenticated_cmds::realm_update_archiving::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_update_archiving::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_realm_not_found() {
    // Generated from Parsec 3.8.1-a.0+dev
    // Content:
    //   status: 'realm_not_found'
    let raw = hex!("81a6737461747573af7265616c6d5f6e6f745f666f756e64");

    let expected = authenticated_cmds::realm_update_archiving::Rep::RealmNotFound;
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = authenticated_cmds::realm_update_archiving::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_update_archiving::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_realm_deleted() {
    // Generated from Parsec 3.8.1-a.0+dev
    // Content:
    //   status: 'realm_deleted'
    let raw = hex!("81a6737461747573ad7265616c6d5f64656c65746564");

    let expected = authenticated_cmds::realm_update_archiving::Rep::RealmDeleted;
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = authenticated_cmds::realm_update_archiving::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_update_archiving::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_archiving_period_too_short() {
    // Generated from Parsec 3.8.1-a.0+dev
    // Content:
    //   status: 'archiving_period_too_short'
    let raw = hex!("81a6737461747573ba617263686976696e675f706572696f645f746f6f5f73686f7274");

    let expected = authenticated_cmds::realm_update_archiving::Rep::ArchivingPeriodTooShort;
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = authenticated_cmds::realm_update_archiving::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_update_archiving::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_require_greater_timestamp() {
    // Generated from Parsec 3.8.1-a.0+dev
    // Content:
    //   status: 'require_greater_timestamp'
    //   strictly_greater_than: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
    let raw = hex!(
        "82a6737461747573b9726571756972655f677265617465725f74696d657374616d70b5"
        "7374726963746c795f677265617465725f7468616ed70100035d162fa2e400"
    );

    let expected = authenticated_cmds::realm_update_archiving::Rep::RequireGreaterTimestamp {
        strictly_greater_than: "2000-1-2T01:00:00Z".parse().unwrap(),
    };
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = authenticated_cmds::realm_update_archiving::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_update_archiving::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_timestamp_out_of_ballpark() {
    // Generated from Parsec 3.8.1-a.0+dev
    // Content:
    //   status: 'timestamp_out_of_ballpark'
    //   ballpark_client_early_offset: 300.0
    //   ballpark_client_late_offset: 320.0
    //   client_timestamp: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
    //   server_timestamp: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
    let raw = hex!(
        "85a6737461747573b974696d657374616d705f6f75745f6f665f62616c6c7061726bbc"
        "62616c6c7061726b5f636c69656e745f6561726c795f6f6666736574cb4072c0000000"
        "0000bb62616c6c7061726b5f636c69656e745f6c6174655f6f6666736574cb40740000"
        "00000000b0636c69656e745f74696d657374616d70d70100035d162fa2e400b0736572"
        "7665725f74696d657374616d70d70100035d162fa2e400"
    );

    let expected = authenticated_cmds::realm_update_archiving::Rep::TimestampOutOfBallpark {
        ballpark_client_early_offset: 300.,
        ballpark_client_late_offset: 320.,
        server_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        client_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
    };
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = authenticated_cmds::realm_update_archiving::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_update_archiving::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
