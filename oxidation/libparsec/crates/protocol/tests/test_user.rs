// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;
use rstest::rstest;
use std::num::NonZeroU64;

use libparsec_protocol::*;
use libparsec_types::HumanHandle;

#[rstest]
fn serde_user_get_req() {
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

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::ok(
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
    ),
    authenticated_cmds::user_get::Rep::Ok {
        user_certificate: b"foobar".to_vec(),
        revoked_user_certificate: Some(b"foobar".to_vec()),
        device_certificates: vec![b"foobar".to_vec()],
        trustchain: authenticated_cmds::user_get::Trustchain {
            users: vec![b"foobar".to_vec()],
            devices: vec![b"foobar".to_vec()],
            revoked_users: vec![b"foobar".to_vec()],
        },
    }
)]
#[case::ok_null_revoked_user_cert(
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
    ),
    authenticated_cmds::user_get::Rep::Ok {
        user_certificate: b"foobar".to_vec(),
        revoked_user_certificate: None,
        device_certificates: vec![b"foobar".to_vec()],
        trustchain: authenticated_cmds::user_get::Trustchain {
            users: vec![b"foobar".to_vec()],
            devices: vec![b"foobar".to_vec()],
            revoked_users: vec![b"foobar".to_vec()],
        },
    }
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_found"
    &hex!(
        "81a6737461747573a96e6f745f666f756e64"
    ),
    authenticated_cmds::user_get::Rep::NotFound
)]
fn serde_user_get_rep(
    #[case] raw_bytes: &[u8],
    #[case] expected: authenticated_cmds::user_get::Rep,
) {
    let data = authenticated_cmds::user_get::Rep::load(raw_bytes).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_get::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
fn serde_user_create_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "user_create"
    //   device_certificate: hex!("666f6f626172")
    //   redacted_device_certificate: hex!("666f6f626172")
    //   redacted_user_certificate: hex!("666f6f626172")
    //   user_certificate: hex!("666f6f626172")
    let raw = hex!(
        "85a3636d64ab757365725f637265617465b26465766963655f6365727469666963617465c4"
        "06666f6f626172bb72656461637465645f6465766963655f6365727469666963617465c406"
        "666f6f626172b972656461637465645f757365725f6365727469666963617465c406666f6f"
        "626172b0757365725f6365727469666963617465c406666f6f626172"
    );

    let req = authenticated_cmds::user_create::Req {
        user_certificate: b"foobar".to_vec(),
        device_certificate: b"foobar".to_vec(),
        redacted_user_certificate: b"foobar".to_vec(),
        redacted_device_certificate: b"foobar".to_vec(),
    };

    let expected = authenticated_cmds::AnyCmdReq::UserCreate(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "ok"
        &hex!(
            "81a6737461747573a26f6b"
        )[..],
        authenticated_cmds::user_create::Rep::Ok
    )
)]
#[case::not_allowed(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "not_allowed"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573ab6e6f745f616c6c6f776564"
        )[..],
        authenticated_cmds::user_create::Rep::NotAllowed {
            reason: Some("foobar".to_owned())
        }
    )
)]
#[case::invalid_certification(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "invalid_certification"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573b5696e76616c69645f636572746966"
            "69636174696f6e"
        )[..],
        authenticated_cmds::user_create::Rep::InvalidCertification {
            reason: Some("foobar".to_owned())
        }
    )
)]
#[case::invalid_raw(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "invalid_raw"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573ac696e76616c69645f64617461"
        )[..],
        authenticated_cmds::user_create::Rep::InvalidData {
            reason: Some("foobar".to_owned())
        }
    )
)]
#[case::already_exists(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "already_exists"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573ae616c72656164795f657869737473"
        )[..],
        authenticated_cmds::user_create::Rep::AlreadyExists {
            reason: Some("foobar".to_owned())
        }
    )
)]
#[case::active_users_limit_reached(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "active_users_limit_reached"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573ba6163746976655f75736572735f6c"
            "696d69745f72656163686564"
        )[..],
        authenticated_cmds::user_create::Rep::ActiveUsersLimitReached {
            reason: Some("foobar".to_owned())
        }
    )
)]
fn serde_user_create_rep(#[case] raw_expected: (&[u8], authenticated_cmds::user_create::Rep)) {
    let (raw, expected) = raw_expected;

    let data = authenticated_cmds::user_create::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_create::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::missing_redacted_device_certificate(
    // Generated from Python implementation (Parsec v2.11.1+dev)
    // Content:
    //   cmd: "user_create"
    //   device_certificate: hex!("666f6f626172")
    //   redacted_device_certificate: None
    //   redacted_user_certificate: hex!("666f6f626172")
    //   user_certificate: hex!("666f6f626172")
    &hex!(
        "85a3636d64ab757365725f637265617465b26465766963655f6365727469666963617465c4"
        "06666f6f626172bb72656461637465645f6465766963655f6365727469666963617465c0b9"
        "72656461637465645f757365725f6365727469666963617465c406666f6f626172b0757365"
        "725f6365727469666963617465c406666f6f626172"
    )[..],
)]
#[case::missing_redacted_user_certificate(
    // Generated from Python implementation (Parsec v2.11.1+dev)
    // Content:
    //   cmd: "user_create"
    //   device_certificate: hex!("666f6f626172")
    //   redacted_device_certificate: hex!("666f6f626172")
    //   redacted_user_certificate: None
    //   user_certificate: hex!("666f6f626172")
    &hex!(
        "85a3636d64ab757365725f637265617465b26465766963655f6365727469666963617465c4"
        "06666f6f626172bb72656461637465645f6465766963655f6365727469666963617465c406"
        "666f6f626172b972656461637465645f757365725f6365727469666963617465c0b0757365"
        "725f6365727469666963617465c406666f6f626172"
    )[..],
)]
fn serde_user_create_req_invalid(#[case] raw: &[u8]) {
    assert!(authenticated_cmds::AnyCmdReq::load(raw).is_err())
}

#[rstest]
fn serde_user_revoke_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "user_revoke"
    //   revoked_user_certificate: hex!("666f6f626172")
    let raw = hex!(
        "82a3636d64ab757365725f7265766f6b65b87265766f6b65645f757365725f636572746966"
        "6963617465c406666f6f626172"
    );

    let req = authenticated_cmds::user_revoke::Req {
        revoked_user_certificate: b"foobar".to_vec(),
    };

    let expected = authenticated_cmds::AnyCmdReq::UserRevoke(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "ok"
        &hex!(
            "81a6737461747573a26f6b"
        )[..],
        authenticated_cmds::user_revoke::Rep::Ok
    )
)]
#[case::not_allowed(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "not_allowed"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573ab6e6f745f616c6c6f776564"
        )[..],
        authenticated_cmds::user_revoke::Rep::NotAllowed {
            reason: Some("foobar".to_owned())
        }
    )
)]
#[case::invalid_certification(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "invalid_certification"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573b5696e76616c69645f636572746966"
            "69636174696f6e"
        )[..],
        authenticated_cmds::user_revoke::Rep::InvalidCertification {
            reason: Some("foobar".to_owned())
        }
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_found"
        &hex!(
            "81a6737461747573a96e6f745f666f756e64"
        )[..],
        authenticated_cmds::user_revoke::Rep::NotFound
    )
)]
#[case::already_revoked(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "already_revoked"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573af616c72656164795f7265766f6b65"
            "64"
        )[..],
        authenticated_cmds::user_revoke::Rep::AlreadyRevoked {
            reason: Some("foobar".to_owned())
        }
    )
)]
fn serde_user_revoke_rep(#[case] raw_expected: (&[u8], authenticated_cmds::user_revoke::Rep)) {
    let (raw, expected) = raw_expected;

    let data = authenticated_cmds::user_revoke::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::user_revoke::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
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
        device_certificate: b"foobar".to_vec(),
        redacted_device_certificate: b"foobar".to_vec(),
    };

    let expected = authenticated_cmds::AnyCmdReq::DeviceCreate(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "ok"
        &hex!(
            "81a6737461747573a26f6b"
        )[..],
        authenticated_cmds::device_create::Rep::Ok
    )
)]
#[case::invalid_certification(
    (
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
    )
)]
#[case::bad_user_id(
    (
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
    )
)]
#[case::invalid_raw(
    (
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
    )
)]
#[case::already_exists(
    (
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
    )
)]
fn serde_device_create_rep(#[case] raw_expected: (&[u8], authenticated_cmds::device_create::Rep)) {
    let (raw, expected) = raw_expected;

    let data = authenticated_cmds::device_create::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::device_create::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::full(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   cmd: "human_find"
        //   omit_non_human: false
        //   omit_revoked: false
        //   page: 8
        //   per_page: 8
        //   query: "foobar"
        &hex!(
            "86a3636d64aa68756d616e5f66696e64ae6f6d69745f6e6f6e5f68756d616ec2ac6f6d6974"
            "5f7265766f6b6564c2a47061676508a87065725f7061676508a57175657279a6666f6f6261"
            "72"
        )[..],
        authenticated_cmds::AnyCmdReq::HumanFind(authenticated_cmds::human_find::Req {
            query: Some("foobar".to_owned()),
            omit_revoked: false,
            omit_non_human: false,
            page: NonZeroU64::new(8).unwrap(),
            per_page: IntegerBetween1And100::try_from(8).unwrap(),
        })
    )
)]
#[case::without_query(
    (
        // Generated from Python implementation (Parsec v2.12.1+dev)
        // Content:
        //   cmd: "human_find"
        //   omit_non_human: false
        //   omit_revoked: false
        //   page: 8
        //   per_page: 8
        //   query: None
        //
        &hex!(
            "86a3636d64aa68756d616e5f66696e64ae6f6d69745f6e6f6e5f68756d616ec2ac6f6d6974"
            "5f7265766f6b6564c2a47061676508a87065725f7061676508a57175657279c0"
        )[..],
        authenticated_cmds::AnyCmdReq::HumanFind(authenticated_cmds::human_find::Req {
            query: None,
            omit_revoked: false,
            omit_non_human: false,
            page: NonZeroU64::new(8).unwrap(),
            per_page: IntegerBetween1And100::try_from(8).unwrap(),
        })
    )
)]
fn serde_human_find_req(#[case] raw_expected: (&[u8], authenticated_cmds::AnyCmdReq)) {
    let (raw, expected) = raw_expected;

    let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   page: 8
        //   per_page: 8
        //   results: [
        //     {
        //       human_handle: ["bob@dev1", "bob"]
        //       revoked: false
        //       user_id: "109b68ba5cdf428ea0017fc6bcc04d4a"
        //     }
        //   ]
        //   status: "ok"
        //   total: 8
        &hex!(
            "85a47061676508a87065725f7061676508a7726573756c74739183ac68756d616e5f68616e"
            "646c6592a8626f624064657631a3626f62a7757365725f6964d92031303962363862613563"
            "64663432386561303031376663366263633034643461a77265766f6b6564c2a67374617475"
            "73a26f6ba5746f74616c08"
        )[..],
        authenticated_cmds::human_find::Rep::Ok {
            results: vec![authenticated_cmds::human_find::HumanFindResultItem {
                user_id: "109b68ba5cdf428ea0017fc6bcc04d4a".parse().unwrap(),
                human_handle: Some(HumanHandle::new("bob@dev1", "bob").unwrap()),
                revoked: false,
            }],
            page: NonZeroU64::new(8).unwrap(),
            per_page: IntegerBetween1And100::try_from(8).unwrap(),
            total: 8,
        }
    )
)]
#[case::not_allowed(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "not_allowed"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573ab6e6f745f616c6c6f776564"
        )[..],
        authenticated_cmds::human_find::Rep::NotAllowed {
            reason: Some("foobar".to_owned())
        }
    )
)]
fn serde_human_find_rep(#[case] raw_expected: (&[u8], authenticated_cmds::human_find::Rep)) {
    let (raw, expected) = raw_expected;

    let data = authenticated_cmds::human_find::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::human_find::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::page_0(
    // Generated from Python implementation (Parsec v2.11.1+dev)
    // Content:
    //   cmd: "human_find"
    //   omit_non_human: false
    //   omit_revoked: false
    //   page: 0
    //   per_page: 8
    //   query: "foobar"
    &hex!(
        "86a3636d64aa68756d616e5f66696e64ae6f6d69745f6e6f6e5f68756d616ec2ac6f6d6974"
        "5f7265766f6b6564c2a47061676500a87065725f7061676508a57175657279a6666f6f6261"
        "72"
    )[..],
)]
#[case::negative_page(
    // Generated from Python implementation (Parsec v2.12.1+dev)
    // Content:
    //   cmd: "human_find"
    //   omit_non_human: false
    //   omit_revoked: false
    //   page: -1
    //   per_page: 8
    //   query: "foobar"
    &hex!(
        "86a3636d64aa68756d616e5f66696e64ae6f6d69745f6e6f6e5f68756d616ec2ac6f6d6974"
        "5f7265766f6b6564c2a470616765ffa87065725f7061676508a57175657279a6666f6f6261"
        "72"
    )[..],
)]
#[case::per_page_0(
    // Generated from Python implementation (Parsec v2.11.1+dev)
    // Content:
    //   cmd: "human_find"
    //   omit_non_human: false
    //   omit_revoked: false
    //   page: 8
    //   per_page: 0
    //   query: "foobar"
    &hex!(
        "86a3636d64aa68756d616e5f66696e64ae6f6d69745f6e6f6e5f68756d616ec2ac6f6d6974"
        "5f7265766f6b6564c2a47061676508a87065725f7061676500a57175657279a6666f6f6261"
        "72"
    )[..],
)]
#[case::per_page_101(
    // Generated from Python implementation (Parsec v2.11.1+dev)
    // Content:
    //   cmd: "human_find"
    //   omit_non_human: false
    //   omit_revoked: false
    //   page: 0
    //   per_page: 101
    //   query: "foobar"
    &hex!(
        "86a3636d64aa68756d616e5f66696e64ae6f6d69745f6e6f6e5f68756d616ec2ac6f6d6974"
        "5f7265766f6b6564c2a47061676500a87065725f7061676565a57175657279a6666f6f6261"
        "72"
    )[..],
)]
fn serde_human_find_req_invalid(#[case] raw: &[u8]) {
    assert!(authenticated_cmds::AnyCmdReq::load(raw).is_err())
}
