// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;

use super::anonymous_server_cmds;

// Request

pub fn req() {
    // Generated from Parsec 3.8.1-a.0+dev
    // Content:
    //   cmd: 'scws_service_certificate'
    let raw: &[u8] = hex!("81a3636d64b8736377735f736572766963655f6365727469666963617465").as_ref();
    let req = anonymous_server_cmds::scws_service_certificate::Req;

    println!("***expected: {:?}", req.dump().unwrap());
    let expected = anonymous_server_cmds::AnyCmdReq::ScwsServiceCertificate(req);
    let data = anonymous_server_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let anonymous_server_cmds::AnyCmdReq::ScwsServiceCertificate(req2) = data else {
        unreachable!()
    };
    let raw2 = req2.dump().unwrap();

    let data2 = anonymous_server_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.8.1-a.0+dev
    // Content:
    //   status: 'ok'
    //   certificate: 0x48656c6c6f20776f726c6421
    let raw: &[u8] = hex!(
        "82a6737461747573a26f6bab6365727469666963617465c40c48656c6c6f20776f726c"
        "6421"
    )
    .as_ref();
    let rep = anonymous_server_cmds::scws_service_certificate::Rep::Ok {
        certificate: "Hello world!".to_owned(),
    };

    println!("***expected: {:?}", rep.dump().unwrap());

    let data = anonymous_server_cmds::scws_service_certificate::Rep::load(raw).unwrap();

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_server_cmds::scws_service_certificate::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, rep);
}

pub fn rep_not_available() {
    // Generated from Parsec 3.8.1-a.0+dev
    // Content:
    //   status: 'not_available'
    let raw: &[u8] = hex!("81a6737461747573ad6e6f745f617661696c61626c65").as_ref();
    let rep = anonymous_server_cmds::scws_service_certificate::Rep::NotAvailable;

    println!("***expected: {:?}", rep.dump().unwrap());

    let data = anonymous_server_cmds::scws_service_certificate::Rep::load(raw).unwrap();

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = anonymous_server_cmds::scws_service_certificate::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, rep);
}
