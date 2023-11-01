// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;

use super::authenticated_cmds;

// Request

pub fn req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "user_get"
    //   user_id: "109b68ba5cdf428ea0017fc6bcc04d4a"
    let raw = hex!(
        "82a3636d64a8757365725f676574a7757365725f6964d92031303962363862613563646634"
        "32386561303031376663366263633034643461"
    );

    let req = authenticated_cmds::user_get::Req {
        user_id: "109b68ba5cdf428ea0017fc6bcc04d4a".parse().unwrap(),
    };

    let expected = authenticated_cmds::AnyCmdReq::UserGet(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::UserGet(req2) = data else {
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
            // Generated from Python implementation (Parsec v2.6.0+dev)
            // Content:
            //   device_certificates: [hex!("666f6f626172")]
            //   revoked_user_certificate: hex!("666f6f626172")
            //   status: "ok"
            //   trustchain: {
            //     devices: [hex!("666f6f626172")]
            //     revoked_users: [hex!("666f6f626172")]
            //     users: [hex!("666f6f626172")]
            //   }
            //   user_certificate: hex!("666f6f626172")
            &hex!(
                "85b36465766963655f63657274696669636174657391c406666f6f626172b87265766f6b65"
                "645f757365725f6365727469666963617465c406666f6f626172a6737461747573a26f6baa"
                "7472757374636861696e83a76465766963657391c406666f6f626172ad7265766f6b65645f"
                "757365727391c406666f6f626172a5757365727391c406666f6f626172b0757365725f6365"
                "727469666963617465c406666f6f626172"
            )[..],
            authenticated_cmds::user_get::Rep::Ok {
                user_certificate: b"foobar".as_ref().into(),
                revoked_user_certificate: Some(b"foobar".as_ref().into()),
                device_certificates: vec![b"foobar".as_ref().into()],
                trustchain: authenticated_cmds::user_get::Trustchain {
                    users: vec![b"foobar".as_ref().into()],
                    devices: vec![b"foobar".as_ref().into()],
                    revoked_users: vec![b"foobar".as_ref().into()],
                },
            },
        ),
        (
            // Generated from Rust implementation (Parsec v2.13.0-rc1+dev)
            // Content:
            //   device_certificates: [hex!("666f6f626172")]
            //   revoked_user_certificate: None
            //   status: "ok"
            //   trustchain: {
            //     devices: [hex!("666f6f626172")]
            //     revoked_users: [hex!("666f6f626172")]
            //     users: [hex!("666f6f626172")]
            //   }
            //   user_certificate: hex!("666f6f626172")
            //
            &hex!(
                "85b36465766963655f63657274696669636174657391c406666f6f626172b87265766f6b65"
                "645f757365725f6365727469666963617465c0a6737461747573a26f6baa74727573746368"
                "61696e83a76465766963657391c406666f6f626172ad7265766f6b65645f757365727391c4"
                "06666f6f626172a5757365727391c406666f6f626172b0757365725f636572746966696361"
                "7465c406666f6f626172"
            )[..],
            authenticated_cmds::user_get::Rep::Ok {
                user_certificate: b"foobar".as_ref().into(),
                revoked_user_certificate: None,
                device_certificates: vec![b"foobar".as_ref().into()],
                trustchain: authenticated_cmds::user_get::Trustchain {
                    users: vec![b"foobar".as_ref().into()],
                    devices: vec![b"foobar".as_ref().into()],
                    revoked_users: vec![b"foobar".as_ref().into()],
                },
            },
        ),
    ];

    for (raw, expected) in raw_expected {
        let data = authenticated_cmds::user_get::Rep::load(raw).unwrap();

        p_assert_eq!(data, expected);

        // Also test serialization round trip
        let raw2 = data.dump().unwrap();

        let data2 = authenticated_cmds::user_get::Rep::load(&raw2).unwrap();

        p_assert_eq!(data2, expected);
    }
}

pub fn rep_not_found() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    let raw = hex!("81a6737461747573a96e6f745f666f756e64");

    let expected = authenticated_cmds::user_get::Rep::NotFound;

    let data = authenticated_cmds::user_get::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_get::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
