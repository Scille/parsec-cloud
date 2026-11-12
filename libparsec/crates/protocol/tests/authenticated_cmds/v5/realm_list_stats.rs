// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use super::authenticated_cmds;

use libparsec_tests_lite::{hex, p_assert_eq};
use libparsec_types::VlobID;

// Request

pub fn req() {
    // Generated from Parsec 3.11.1-a.0+dev
    // Content:
    //   cmd: 'realm_list_stats'
    let raw: &[u8] = hex!("81a3636d64b07265616c6d5f6c6973745f7374617473").as_ref();
    let req = authenticated_cmds::realm_list_stats::Req;

    println!("***expected: {:?}", req.dump().unwrap());
    let expected = authenticated_cmds::AnyCmdReq::RealmListStats(req);

    let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::RealmListStats(req2) = data else {
        unreachable!()
    };
    let raw2 = req2.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.11.1-a.0+dev
    // Content:
    //   status: 'ok'
    //   realms: [
    //     {
    //       realm_id: ext(2, 0xa11cec00100000000000000000000000),
    //       metadata_size: 663,
    //       data_size: 4096,
    //       owners: 1,
    //       managers: 2,
    //       contributors: 3,
    //       readers: 4,
    //     },
    //   ]
    let raw: &[u8] = hex!(
        "82a6737461747573a26f6ba67265616c6d739187ac636f6e7472696275746f727303a9"
        "646174615f73697a65cd1000a86d616e616765727302ad6d657461646174615f73697a"
        "65cd0297a66f776e65727301a77265616465727304a87265616c6d5f6964d802a11cec"
        "00100000000000000000000000"
    )
    .as_ref();

    let expected = authenticated_cmds::realm_list_stats::Rep::Ok {
        realms: vec![authenticated_cmds::realm_list_stats::RealmStats {
            realm_id: VlobID::from_hex("a11cec00100000000000000000000000").unwrap(),
            metadata_size: 663,
            data_size: 4096,
            owners: 1,
            managers: 2,
            contributors: 3,
            readers: 4,
        }],
    };

    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_cmds::realm_list_stats::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_list_stats::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_author_not_allowed() {
    // Generated from Parsec 3.11.1-a.0+dev
    // Content:
    //   status: 'author_not_allowed'
    let raw: &[u8] = hex!("81a6737461747573b2617574686f725f6e6f745f616c6c6f776564").as_ref();

    let expected = authenticated_cmds::realm_list_stats::Rep::AuthorNotAllowed;

    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_cmds::realm_list_stats::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_list_stats::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
