// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;

use super::anonymous_server_cmds;

// Request

pub fn req() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   cmd: 'account_recover_send_validation_email'
    //   email: 'alice@invalid.com'
    let raw: &[u8] = hex!(
        "82a3636d64d9256163636f756e745f7265636f7665725f73656e645f76616c69646174"
        "696f6e5f656d61696ca5656d61696cb1616c69636540696e76616c69642e636f6d"
    )
    .as_ref();

    let req = anonymous_server_cmds::account_recover_send_validation_email::Req {
        email: "alice@invalid.com".parse().unwrap(),
    };

    let expected = anonymous_server_cmds::AnyCmdReq::AccountRecoverSendValidationEmail(req.clone());
    println!("***expected: {:?}", req.dump().unwrap());

    let data = anonymous_server_cmds::AnyCmdReq::load(raw).unwrap();
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let anonymous_server_cmds::AnyCmdReq::AccountRecoverSendValidationEmail(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = anonymous_server_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   status: 'ok'
    let raw: &[u8] = hex!("81a6737461747573a26f6b").as_ref();
    let expected = anonymous_server_cmds::account_recover_send_validation_email::Rep::Ok {};

    let data =
        anonymous_server_cmds::account_recover_send_validation_email::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 =
        anonymous_server_cmds::account_recover_send_validation_email::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_email_server_unavailable() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   status: 'email_server_unavailable'
    let raw: &[u8] =
        hex!("81a6737461747573b8656d61696c5f7365727665725f756e617661696c61626c65").as_ref();
    let expected =
        anonymous_server_cmds::account_recover_send_validation_email::Rep::EmailServerUnavailable {};
    let data =
        anonymous_server_cmds::account_recover_send_validation_email::Rep::load(raw).unwrap();
    println!("***expected: {:?}", expected.dump().unwrap());
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 =
        anonymous_server_cmds::account_recover_send_validation_email::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_email_recipient_refused() {
    // Generated from Parsec 3.4.0-a.7+dev
    // Content:
    //   status: 'email_recipient_refused'
    let raw: &[u8] =
        hex!("81a6737461747573b7656d61696c5f726563697069656e745f72656675736564").as_ref();

    let expected =
        anonymous_server_cmds::account_recover_send_validation_email::Rep::EmailRecipientRefused {};
    let data =
        anonymous_server_cmds::account_recover_send_validation_email::Rep::load(raw).unwrap();
    println!("***expected: {:?}", expected.dump().unwrap());
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 =
        anonymous_server_cmds::account_recover_send_validation_email::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_email_sending_rate_limited() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   status: 'email_sending_rate_limited'
    //   wait_until: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z
    let raw: &[u8] = hex!(
        "82a6737461747573ba656d61696c5f73656e64696e675f726174655f6c696d69746564"
        "aa776169745f756e74696cd70100035d013b37e000"
    )
    .as_ref();

    let expected =
        anonymous_server_cmds::account_recover_send_validation_email::Rep::EmailSendingRateLimited {
            wait_until: "2000-01-01T00:00:00Z".parse().unwrap(),
        };
    let data =
        anonymous_server_cmds::account_recover_send_validation_email::Rep::load(raw).unwrap();
    println!("***expected: {:?}", expected.dump().unwrap());
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 =
        anonymous_server_cmds::account_recover_send_validation_email::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
