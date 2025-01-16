// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::collections::HashMap;

use libparsec_tests_lite::prelude::*;

use super::tos_cmds;

// Request

pub fn req() {
    // Generated from Parsec 3.0.1-a.0
    // Content:
    //   cmd: 'tos_get'
    let raw: &[u8] = hex!("81a3636d64a7746f735f676574").as_ref();

    let expected = tos_cmds::AnyCmdReq::TosGet(tos_cmds::tos_get::Req);

    let data = tos_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // roundtrip check ...
    let tos_cmds::AnyCmdReq::TosGet(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = tos_cmds::AnyCmdReq::load(&raw2).unwrap();
    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Parsec 3.0.2-a.0+dev
    // Content:
    //   status: 'ok'
    //   per_locale_urls: { fr_FR: 'https://parsec.invalid/tos_fr.pdf', en_US: 'https://parsec.invalid/tos_en.pdf', }
    //   updated_on: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z
    let raw: &[u8] = hex!(
        "83a6737461747573a26f6baf7065725f6c6f63616c655f75726c7382a566725f4652d9"
        "2168747470733a2f2f7061727365632e696e76616c69642f746f735f66722e706466a5"
        "656e5f5553d92168747470733a2f2f7061727365632e696e76616c69642f746f735f65"
        "6e2e706466aa757064617465645f6f6ed70100035d013b37e000"
    )
    .as_ref();

    let expected = tos_cmds::tos_get::Rep::Ok {
        per_locale_urls: HashMap::from_iter([
            (
                "fr_FR".to_string(),
                "https://parsec.invalid/tos_fr.pdf".to_string(),
            ),
            (
                "en_US".to_string(),
                "https://parsec.invalid/tos_en.pdf".to_string(),
            ),
        ]),
        updated_on: "2000-01-01T00:00:00Z".parse().unwrap(),
    };

    let data = tos_cmds::tos_get::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = tos_cmds::tos_get::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_no_tos() {
    // Generated from Parsec 3.0.1-a.0
    // Content:
    //   status: 'no_tos'
    let raw: &[u8] = hex!("81a6737461747573a66e6f5f746f73").as_ref();

    let expected = tos_cmds::tos_get::Rep::NoTos;

    let data = tos_cmds::tos_get::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = tos_cmds::tos_get::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
