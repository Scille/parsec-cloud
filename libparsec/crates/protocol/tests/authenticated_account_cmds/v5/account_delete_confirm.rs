// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::authenticated_account_cmds;

// Request

pub fn req() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   cmd: 'account_delete_confirm'
    //   deletion_token: 0x77cdb576c1ebb43fa0064f404e5ade22
    let raw: &[u8] = hex!(
        "82a3636d64b66163636f756e745f64656c6574655f636f6e6669726dae64656c657469"
        "6f6e5f746f6b656ec41077cdb576c1ebb43fa0064f404e5ade22"
    )
    .as_ref();

    let req = authenticated_account_cmds::account_delete_confirm::Req {
        deletion_token: AccountDeletionToken::from_hex("77cdb576c1ebb43fa0064f404e5ade22").unwrap(),
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

pub fn rep_invalid_deletion_token() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   status: 'invalid_deletion_token'
    let raw: &[u8] =
        hex!("81a6737461747573b6696e76616c69645f64656c6574696f6e5f746f6b656e").as_ref();

    let expected = authenticated_account_cmds::account_delete_confirm::Rep::InvalidDeletionToken;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_account_cmds::account_delete_confirm::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    let raw2 = data.dump().unwrap();
    let data2 = authenticated_account_cmds::account_delete_confirm::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
