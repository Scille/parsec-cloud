// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_protocol::authenticated_cmds::v3 as authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;

#[parsec_test]
fn serde_device_create_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "device_create"
    //   device_certificate: hex!("666f6f626172")
    //   redacted_device_certificate: hex!("666f6f626172")
    let raw = hex!(
        "83a3636d64ad6465766963655f637265617465b26465766963655f63657274696669636174"
        "65c406666f6f626172bb72656461637465645f6465766963655f6365727469666963617465"
        "c406666f6f626172"
    );

    let req = authenticated_cmds::device_create::Req {
        device_certificate: b"foobar".as_ref().into(),
        redacted_device_certificate: b"foobar".as_ref().into(),
    };

    let expected = authenticated_cmds::AnyCmdReq::DeviceCreate(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = if let authenticated_cmds::AnyCmdReq::DeviceCreate(data) = data {
        data.dump().unwrap()
    } else {
        unreachable!()
    };

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
#[case::ok(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    &hex!(
        "81a6737461747573a26f6b"
    )[..],
    authenticated_cmds::device_create::Rep::Ok
)]
#[case::invalid_certification(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "invalid_certification"
    &hex!(
        "82a6726561736f6ea6666f6f626172a6737461747573b5696e76616c69645f636572746966"
        "69636174696f6e"
    )[..],
    authenticated_cmds::device_create::Rep::InvalidCertification {
        reason: Some("foobar".to_owned())
    }
)]
#[case::bad_user_id(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "bad_user_id"
    &hex!(
        "82a6726561736f6ea6666f6f626172a6737461747573ab6261645f757365725f6964"
    )[..],
    authenticated_cmds::device_create::Rep::BadUserId {
        reason: Some("foobar".to_owned())
    }
)]
#[case::invalid_raw(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "invalid_raw"
    &hex!(
        "82a6726561736f6ea6666f6f626172a6737461747573ac696e76616c69645f64617461"
    )[..],
    authenticated_cmds::device_create::Rep::InvalidData {
        reason: Some("foobar".to_owned())
    }
)]
#[case::already_exists(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "already_exists"
    &hex!(
        "82a6726561736f6ea6666f6f626172a6737461747573ae616c72656164795f657869737473"
    )[..],
    authenticated_cmds::device_create::Rep::AlreadyExists {
        reason: Some("foobar".to_owned())
    }
)]
fn serde_device_create_rep(
    #[case] raw: &[u8],
    #[case] expected: authenticated_cmds::device_create::Rep,
) {
    let data = authenticated_cmds::device_create::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::device_create::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}
