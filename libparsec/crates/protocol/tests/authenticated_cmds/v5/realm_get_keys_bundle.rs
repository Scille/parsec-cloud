// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::*;

use super::authenticated_cmds;

// Request

pub fn req() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   cmd: "realm_get_keys_bundle"
    //   key_index: 8
    //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
    let raw = hex!(
        "83a3636d64b57265616c6d5f6765745f6b6579735f62756e646c65a87265616c6d5f6964d8"
        "021d3353157d7d4e95ad2fdea7b3bd19c5a96b65795f696e64657808
    ");

    let req = authenticated_cmds::realm_get_keys_bundle::Req {
        key_index: 8,
        realm_id: VlobID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
    };

    let expected = authenticated_cmds::AnyCmdReq::RealmGetKeysBundle(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::RealmGetKeysBundle(req2) = data else {
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
    //   keys_bundle: hex!("666f6f626172")
    //   keys_bundle_access: hex!("666f6f626172")
    let raw = hex!(
        "83a6737461747573a26f6bab6b6579735f62756e646c65c406666f6f626172b26b6579735f"
        "62756e646c655f616363657373c406666f6f626172"
    );

    let expected = authenticated_cmds::realm_get_keys_bundle::Rep::Ok {
        keys_bundle: Bytes::from_static(b"foobar"),
        keys_bundle_access: Bytes::from_static(b"foobar"),
    };

    let data = authenticated_cmds::realm_get_keys_bundle::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_get_keys_bundle::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_author_not_allowed() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "author_not_allowed"
    let raw = hex!("81a6737461747573b2617574686f725f6e6f745f616c6c6f776564");

    let expected = authenticated_cmds::realm_get_keys_bundle::Rep::AuthorNotAllowed;

    let data = authenticated_cmds::realm_get_keys_bundle::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_get_keys_bundle::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_bad_key_index() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "bad_key_index"
    let raw = hex!("81a6737461747573ad6261645f6b65795f696e646578");

    let expected = authenticated_cmds::realm_get_keys_bundle::Rep::BadKeyIndex;

    let data = authenticated_cmds::realm_get_keys_bundle::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_get_keys_bundle::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_access_not_available_for_author() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "access_not_available_for_author"
    let raw = hex!(
        "81a6737461747573bf6163636573735f6e6f745f617661696c61626c655f666f725f617574"
        "686f72"
    );

    let expected = authenticated_cmds::realm_get_keys_bundle::Rep::AccessNotAvailableForAuthor;

    let data = authenticated_cmds::realm_get_keys_bundle::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_get_keys_bundle::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
