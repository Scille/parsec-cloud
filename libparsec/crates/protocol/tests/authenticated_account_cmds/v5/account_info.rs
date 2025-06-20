// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;

use super::authenticated_account_cmds;

// Request
pub fn req() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   cmd: 'account_info'
    let raw: &[u8] = hex!("81a3636d64ac6163636f756e745f696e666f").as_ref();

    let req = authenticated_account_cmds::account_info::Req {};
    let expected = authenticated_account_cmds::AnyCmdReq::AccountInfo(req.clone());
    println!("***expected: {:?}", req.dump().unwrap());
    let data = authenticated_account_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = req.dump().unwrap();
    let data2 = authenticated_account_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected)
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   status: 'ok'
    //   human_handle: [ 'zack@example.com', 'Zack', ]
    let raw: &[u8] = hex!(
        "82a6737461747573a26f6bac68756d616e5f68616e646c6592b07a61636b406578616d"
        "706c652e636f6da45a61636b"
    )
    .as_ref();

    let expected = authenticated_account_cmds::account_info::Rep::Ok {
        human_handle: "Zack <zack@example.com>".parse().unwrap(),
    };
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_account_cmds::account_info::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();
    let data2 = authenticated_account_cmds::account_info::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
