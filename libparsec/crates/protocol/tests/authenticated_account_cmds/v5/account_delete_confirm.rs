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
    //   cmd: 'account_delete_confirm'
    //   deletion_code: '777777'
    let raw: &[u8] = hex!(
        "82a3636d64b66163636f756e745f64656c6574655f636f6e6669726dad64656c657469"
        "6f6e5f636f6465a6373737373737"
    )
    .as_ref();

    let req = authenticated_account_cmds::account_delete_confirm::Req {
        deletion_code: "777777".parse().unwrap(),
    };
    let expected = authenticated_account_cmds::AnyCmdReq::AccountDeleteConfirm(req.clone());
    println!("***expected: {:?}", req.dump().unwrap());

    let data = authenticated_account_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    let raw2 = req.dump().unwrap();
    let data2 = authenticated_account_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   status: 'ok'
    let raw: &[u8] = hex!("81a6737461747573a26f6b").as_ref();

    let expected = authenticated_account_cmds::account_delete_confirm::Rep::Ok;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_account_cmds::account_delete_confirm::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    let raw2 = data.dump().unwrap();
    let data2 = authenticated_account_cmds::account_delete_confirm::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_invalid_deletion_code() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   status: 'invalid_deletion_code'
    let raw: &[u8] = hex!("81a6737461747573b5696e76616c69645f64656c6574696f6e5f636f6465").as_ref();

    let expected = authenticated_account_cmds::account_delete_confirm::Rep::InvalidDeletionCode;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_account_cmds::account_delete_confirm::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    let raw2 = data.dump().unwrap();
    let data2 = authenticated_account_cmds::account_delete_confirm::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_new_code_needed() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   status: 'new_code_needed'
    let raw: &[u8] = hex!("81a6737461747573af6e65775f636f64655f6e6565646564").as_ref();

    let expected = authenticated_account_cmds::account_delete_confirm::Rep::NewCodeNeeded;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_account_cmds::account_delete_confirm::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    let raw2 = data.dump().unwrap();
    let data2 = authenticated_account_cmds::account_delete_confirm::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
