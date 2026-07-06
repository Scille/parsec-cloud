// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

// wrap this in each test function
// macro that contains a closure
use super::authenticated_cmds;

use libparsec_tests_lite::{hex, p_assert_eq};
use libparsec_types::prelude::*;

// Request

pub fn req() {
    let raw_expected = [
        (
            // Generated from Parsec 3.9.3-a.0+dev
            // Content:
            //   cmd: 'cryptpad_register_session'
            //   realm_id: ext(2, 0x1d3353157d7d4e95ad2fdea7b3bd19c5)
            //   document_id: ext(2, 0x87c6b5fd3b454c94bab51d6af1c6930b)
            //   key_index: 1
            //   timestamp: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
            //   encrypted_candidate_view_key: 0x6d795f656e637279707465645f726f5f6b6579
            //   encrypted_candidate_edit_key: 0x6d795f656e637279707465645f72775f6b6579
            &hex!(
                "87a3636d64b963727970747061645f72656769737465725f73657373696f6ea8726561"
                "6c6d5f6964d8021d3353157d7d4e95ad2fdea7b3bd19c5ab646f63756d656e745f6964"
                "d80287c6b5fd3b454c94bab51d6af1c6930ba96b65795f696e64657801a974696d6573"
                "74616d70d70100035d162fa2e400bc656e637279707465645f63616e6469646174655f"
                "766965775f6b6579c4136d795f656e637279707465645f726f5f6b6579bc656e637279"
                "707465645f63616e6469646174655f656469745f6b6579c4136d795f656e6372797074"
                "65645f72775f6b6579"
            )[..],
            authenticated_cmds::cryptpad_register_session::Req {
                realm_id: VlobID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
                document_id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
                key_index: 1,
                timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
                encrypted_candidate_view_key: Bytes::from_static(b"my_encrypted_ro_key"),
                encrypted_candidate_edit_key: Some(Bytes::from_static(
                    b"my_encrypted_rw_key",
                )),
            },
        ),
        (
            // Generated from Parsec 3.9.3-a.0+dev
            // Content:
            //   cmd: 'cryptpad_register_session'
            //   realm_id: ext(2, 0x1d3353157d7d4e95ad2fdea7b3bd19c5)
            //   document_id: ext(2, 0x87c6b5fd3b454c94bab51d6af1c6930b)
            //   key_index: 2
            //   timestamp: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
            //   encrypted_candidate_view_key: 0x6d795f656e637279707465645f726f5f6b6579
            //   encrypted_candidate_edit_key: None
            &hex!(
                "87a3636d64b963727970747061645f72656769737465725f73657373696f6ea8726561"
                "6c6d5f6964d8021d3353157d7d4e95ad2fdea7b3bd19c5ab646f63756d656e745f6964"
                "d80287c6b5fd3b454c94bab51d6af1c6930ba96b65795f696e64657802a974696d6573"
                "74616d70d70100035d162fa2e400bc656e637279707465645f63616e6469646174655f"
                "766965775f6b6579c4136d795f656e637279707465645f726f5f6b6579bc656e637279"
                "707465645f63616e6469646174655f656469745f6b6579c0"
            )[..],
            authenticated_cmds::cryptpad_register_session::Req {
                realm_id: VlobID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
                document_id: VlobID::from_hex("87c6b5fd3b454c94bab51d6af1c6930b").unwrap(),
                key_index: 2,
                timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
                encrypted_candidate_view_key: Bytes::from_static(b"my_encrypted_ro_key"),
                encrypted_candidate_edit_key: None,
            },
        ),
    ];

    for (raw, req) in raw_expected {
        let expected = authenticated_cmds::AnyCmdReq::CryptpadRegisterSession(req.clone());

        println!("***expected: {:?}", req.dump().unwrap());

        let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();

        p_assert_eq!(data, expected);

        // Also test serialization round trip
        let raw2 = req.dump().unwrap();

        let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

        p_assert_eq!(data2, expected);
    }
}

// Responses

pub fn rep_ok() {
    let raw_expected = [
        (
            // Generated from Parsec 3.9.3-a.0+dev
            // Content:
            //   status: 'ok'
            //   author: ext(2, 0xde10a11cec0010000000000000000000)
            //   encrypted_edit_key: 0x6d795f656e637279707465645f72775f6b6579
            //   encrypted_view_key: 0x6d795f656e637279707465645f726f5f6b6579
            //   key_index: 1
            //   needed_common_certificate_timestamp: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
            //   needed_realm_certificate_timestamp: ext(1, 946861200000000) i.e. 2000-01-03T02:00:00Z
            //   timestamp: ext(1, 946688400000000) i.e. 2000-01-01T02:00:00Z
            &hex!(
                "88a6737461747573a26f6ba6617574686f72d802de10a11cec00100000000000000000"
                "00b2656e637279707465645f656469745f6b6579c4136d795f656e637279707465645f"
                "72775f6b6579b2656e637279707465645f766965775f6b6579c4136d795f656e637279"
                "707465645f726f5f6b6579a96b65795f696e64657801d9236e65656465645f636f6d6d"
                "6f6e5f63657274696669636174655f74696d657374616d70d70100035d162fa2e400d9"
                "226e65656465645f7265616c6d5f63657274696669636174655f74696d657374616d70"
                "d70100035d2a4d7a4400a974696d657374616d70d70100035d0211cb8400"
            )[..],
            authenticated_cmds::cryptpad_register_session::Rep::Ok {
                author: "alice@dev1".parse().unwrap(),
                timestamp: "2000-1-1T01:00:00Z".parse().unwrap(),
                key_index: 1,
                encrypted_edit_key: Some(Bytes::from_static(b"my_encrypted_rw_key")),
                encrypted_view_key: Bytes::from_static(b"my_encrypted_ro_key"),
                needed_common_certificate_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
                needed_realm_certificate_timestamp: "2000-1-3T01:00:00Z".parse().unwrap(),
            },
        ),
        (
            // Generated from Parsec 3.9.3-a.0+dev
            // Content:
            //   status: 'ok'
            //   author: ext(2, 0xde10a11cec0010000000000000000000)
            //   encrypted_edit_key: None
            //   encrypted_view_key: 0x6d795f656e637279707465645f726f5f6b6579
            //   key_index: 2
            //   needed_common_certificate_timestamp: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
            //   needed_realm_certificate_timestamp: ext(1, 946861200000000) i.e. 2000-01-03T02:00:00Z
            //   timestamp: ext(1, 946688400000000) i.e. 2000-01-01T02:00:00Z
            &hex!(
                "88a6737461747573a26f6ba6617574686f72d802de10a11cec00100000000000000000"
                "00b2656e637279707465645f656469745f6b6579c0b2656e637279707465645f766965"
                "775f6b6579c4136d795f656e637279707465645f726f5f6b6579a96b65795f696e6465"
                "7802d9236e65656465645f636f6d6d6f6e5f63657274696669636174655f74696d6573"
                "74616d70d70100035d162fa2e400d9226e65656465645f7265616c6d5f636572746966"
                "69636174655f74696d657374616d70d70100035d2a4d7a4400a974696d657374616d70"
                "d70100035d0211cb8400"
            )[..],
            authenticated_cmds::cryptpad_register_session::Rep::Ok {
                author: "alice@dev1".parse().unwrap(),
                timestamp: "2000-1-1T01:00:00Z".parse().unwrap(),
                key_index: 2,
                encrypted_edit_key: None,
                encrypted_view_key: Bytes::from_static(b"my_encrypted_ro_key"),
                needed_common_certificate_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
                needed_realm_certificate_timestamp: "2000-1-3T01:00:00Z".parse().unwrap(),
            },
        ),
    ];

    for (raw, expected) in raw_expected {
        rep_helper(raw, expected);
    }
}

pub fn rep_realm_not_found() {
    // Generated from Parsec 3.9.1-a.0+dev
    // Content:
    //   status: 'realm_not_found'
    let raw: &[u8] = hex!("81a6737461747573af7265616c6d5f6e6f745f666f756e64").as_ref();
    let expected = authenticated_cmds::cryptpad_register_session::Rep::RealmNotFound;

    rep_helper(raw, expected);
}

pub fn rep_realm_deleted() {
    // Generated from Parsec 3.9.1-a.0+dev
    // Content:
    //   status: 'realm_deleted'
    let raw: &[u8] = hex!("81a6737461747573ad7265616c6d5f64656c65746564").as_ref();
    let expected = authenticated_cmds::cryptpad_register_session::Rep::RealmDeleted;

    rep_helper(raw, expected);
}

pub fn rep_author_not_allowed() {
    // Generated from Parsec 3.9.1-a.0+dev
    // Content:
    //   status: 'author_not_allowed'
    let raw: &[u8] = hex!("81a6737461747573b2617574686f725f6e6f745f616c6c6f776564").as_ref();
    let expected = authenticated_cmds::cryptpad_register_session::Rep::AuthorNotAllowed;

    rep_helper(raw, expected);
}

pub fn rep_bad_key_index() {
    // Generated from Parsec 3.9.1-a.0+dev
    // Content:
    //   status: 'bad_key_index'
    //   last_realm_certificate_timestamp: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z
    let raw: &[u8] = hex!(
        "82a6737461747573ad6261645f6b65795f696e646578d9206c6173745f7265616c6d5f"
        "63657274696669636174655f74696d657374616d70d70100035d013b37e000"
    )
    .as_ref();
    let expected = authenticated_cmds::cryptpad_register_session::Rep::BadKeyIndex {
        last_realm_certificate_timestamp: "2000-01-01T00:00:00Z".parse().unwrap(),
    };

    rep_helper(raw, expected);
}

pub fn rep_timestamp_out_of_ballpark() {
    // Generated from Parsec 3.9.1-a.0+dev
    // Content:
    //   status: 'timestamp_out_of_ballpark'
    //   ballpark_client_early_offset: 300.0
    //   ballpark_client_late_offset: 320.0
    //   client_timestamp: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
    //   server_timestamp: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
    let raw: &[u8] = hex!(
        "85a6737461747573b974696d657374616d705f6f75745f6f665f62616c6c7061726bbc"
        "62616c6c7061726b5f636c69656e745f6561726c795f6f6666736574cb4072c0000000"
        "0000bb62616c6c7061726b5f636c69656e745f6c6174655f6f6666736574cb40740000"
        "00000000b0636c69656e745f74696d657374616d70d70100035d162fa2e400b0736572"
        "7665725f74696d657374616d70d70100035d162fa2e400"
    ).as_ref();
    let expected = authenticated_cmds::cryptpad_register_session::Rep::TimestampOutOfBallpark {
        ballpark_client_early_offset: 300.,
        ballpark_client_late_offset: 320.,
        server_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        client_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    rep_helper(raw, expected);
}

pub fn rep_cryptpad_unavailable() {
    // Generated from Parsec 3.9.1-a.0+dev
    // Content:
    //   status: 'cryptpad_unavailable'
    let raw: &[u8] = hex!("81a6737461747573b463727970747061645f756e617661696c61626c65").as_ref();
    let expected = authenticated_cmds::cryptpad_register_session::Rep::CryptpadUnavailable;

    rep_helper(raw, expected);
}

fn rep_helper(raw: &[u8], expected: authenticated_cmds::cryptpad_register_session::Rep) {
    println!("***expected: {:?}", expected.dump().unwrap());

    let data = authenticated_cmds::cryptpad_register_session::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::cryptpad_register_session::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
