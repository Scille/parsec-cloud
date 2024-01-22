// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::collections::HashMap;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::authenticated_cmds;

// Request

pub fn req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "vlob_poll_changes"
    //   last_checkpoint: 8
    //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
    let raw = hex!(
        "83a3636d64b1766c6f625f706f6c6c5f6368616e676573af6c6173745f636865636b706f69"
        "6e7408a87265616c6d5f6964d8021d3353157d7d4e95ad2fdea7b3bd19c5"
    );

    let req = authenticated_cmds::vlob_poll_changes::Req {
        realm_id: VlobID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
        last_checkpoint: 8,
    };

    let expected = authenticated_cmds::AnyCmdReq::VlobPollChanges(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::VlobPollChanges(req2) = data else {
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
    //   changes: [(ExtType(code=2, raw=b'+_1G(\x13J\x12\x86=\xa1\xceI\xc1\x12\xf6'), 8)]
    //   current_checkpoint: 8
    //   status: "ok"
    let raw = hex!(
        "83a6737461747573a26f6ba76368616e6765739192d8022b5f314728134a12863da1ce49c1"
        "12f608b263757272656e745f636865636b706f696e7408"
    );

    let expected = authenticated_cmds::vlob_poll_changes::Rep::Ok {
        changes: vec![(
            VlobID::from_hex("2b5f314728134a12863da1ce49c112f6").unwrap(),
            8,
        )],
        current_checkpoint: 8,
    };

    let data = authenticated_cmds::vlob_poll_changes::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_poll_changes::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_author_not_allowed() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "author_not_allowed"
    let raw = hex!("81a6737461747573b2617574686f725f6e6f745f616c6c6f776564");

    let expected = authenticated_cmds::vlob_poll_changes::Rep::AuthorNotAllowed;

    let data = authenticated_cmds::vlob_poll_changes::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_poll_changes::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_realm_not_found() {
    // Generated from Rust implementation (Parsec v3.0.0+dev)
    // Content:
    //   status: "realm_not_found"
    let raw = hex!("81a6737461747573af7265616c6d5f6e6f745f666f756e64");

    let expected = authenticated_cmds::vlob_poll_changes::Rep::RealmNotFound;

    let data = authenticated_cmds::vlob_poll_changes::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_poll_changes::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
