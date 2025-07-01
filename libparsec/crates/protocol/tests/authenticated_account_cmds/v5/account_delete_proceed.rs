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
    //   cmd: 'account_delete_proceed'
    //   validation_code: '777777'
    let raw: &[u8] = hex!(
        "82a3636d64b66163636f756e745f64656c6574655f70726f63656564af76616c696461"
        "74696f6e5f636f6465a6373737373737"
    )
    .as_ref();

    let req = authenticated_account_cmds::account_delete_proceed::Req {
        validation_code: "777777".parse().unwrap(),
    };
    let expected = authenticated_account_cmds::AnyCmdReq::AccountDeleteProceed(req.clone());
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

    let expected = authenticated_account_cmds::account_delete_proceed::Rep::Ok;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_account_cmds::account_delete_proceed::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    let raw2 = data.dump().unwrap();
    let data2 = authenticated_account_cmds::account_delete_proceed::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_invalid_validation_code() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   status: 'invalid_validation_code'
    let raw: &[u8] =
        hex!("81a6737461747573b7696e76616c69645f76616c69646174696f6e5f636f6465").as_ref();

    let expected = authenticated_account_cmds::account_delete_proceed::Rep::InvalidValidationCode;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_account_cmds::account_delete_proceed::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    let raw2 = data.dump().unwrap();
    let data2 = authenticated_account_cmds::account_delete_proceed::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_send_validation_code_required() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   status: 'send_validation_code_required'
    let raw: &[u8] = hex!(
        "81a6737461747573bd73656e645f76616c69646174696f6e5f636f64655f7265717569"
        "726564"
    )
    .as_ref();

    let expected =
        authenticated_account_cmds::account_delete_proceed::Rep::SendValidationCodeRequired;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_account_cmds::account_delete_proceed::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    let raw2 = data.dump().unwrap();
    let data2 = authenticated_account_cmds::account_delete_proceed::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
