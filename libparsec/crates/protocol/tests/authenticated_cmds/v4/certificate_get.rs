// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::authenticated_cmds;

// Request

pub fn req() {
    // Generated from Rust implementation (Parsec v2.16.0-a.0+dev)
    // Content:
    //   cmd: "certificate_get"
    //   offset: 0
    let raw = hex!("82a3636d64af63657274696669636174655f676574a66f666673657400");

    let req = authenticated_cmds::certificate_get::Req { offset: 0 };

    let expected = authenticated_cmds::AnyCmdReq::CertificateGet(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::CertificateGet(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    let raw_expected = [
        (
            // Generated from Rust implementation (Parsec v2.16.0-a.0+dev)
            // Content:
            //   certificates: [hex!("666f6f626172")]
            //   status: "ok"
            &hex!("82a6737461747573a26f6bac63657274696669636174657391c406666f6f626172")[..],
            authenticated_cmds::certificate_get::Rep::Ok {
                certificates: vec![Bytes::from_static(b"foobar")],
            },
        ),
        (
            // Generated from Rust implementation (Parsec v2.16.0-a.0+dev)
            // Content:
            //   certificates: [hex!("666f6f626172")]
            //   status: "ok"
            &hex!("82a6737461747573a26f6bac63657274696669636174657390")[..],
            authenticated_cmds::certificate_get::Rep::Ok {
                certificates: vec![],
            },
        ),
    ];

    for (raw, expected) in raw_expected {
        let data = authenticated_cmds::certificate_get::Rep::load(raw).unwrap();

        p_assert_eq!(data, expected);

        // Also test serialization round trip
        let raw2 = data.dump().unwrap();

        let data2 = authenticated_cmds::certificate_get::Rep::load(&raw2).unwrap();

        p_assert_eq!(data2, expected);
    }
}
