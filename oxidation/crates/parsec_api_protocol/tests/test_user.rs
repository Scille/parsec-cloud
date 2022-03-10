// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use std::num::NonZeroU64;

use hex_literal::hex;
use parsec_api_types::HumanHandle;
use rstest::rstest;

use parsec_api_protocol::*;

#[rstest]
fn serde_user_get_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "user_get"
    //   user_id: "109b68ba5cdf428ea0017fc6bcc04d4a"
    let data = hex!(
        "82a3636d64a8757365725f676574a7757365725f6964d92031303962363862613563646634"
        "32386561303031376663366263633034643461"
    );

    let expected = UserGetReqSchema {
        cmd: "user_get".to_owned(),
        user_id: "109b68ba5cdf428ea0017fc6bcc04d4a".parse().unwrap(),
    };

    let schema = UserGetReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = UserGetReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_user_get_rep() {
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
    let data = hex!(
        "85b36465766963655f63657274696669636174657391c406666f6f626172b87265766f6b65"
        "645f757365725f6365727469666963617465c406666f6f626172a6737461747573a26f6baa"
        "7472757374636861696e83a76465766963657391c406666f6f626172ad7265766f6b65645f"
        "757365727391c406666f6f626172a5757365727391c406666f6f626172b0757365725f6365"
        "727469666963617465c406666f6f626172"
    );

    let expected = UserGetRepSchema {
        status: Status::Ok,
        user_certificate: b"foobar".to_vec(),
        revoked_user_certificate: b"foobar".to_vec(),
        device_certificates: vec![b"foobar".to_vec()],
        trustchain: TrustchainSchema {
            users: vec![b"foobar".to_vec()],
            devices: vec![b"foobar".to_vec()],
            revoked_users: vec![b"foobar".to_vec()],
        },
    };

    let schema = UserGetRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = UserGetRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
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
    let data = hex!(
        "85a3636d64ab757365725f637265617465b26465766963655f6365727469666963617465c4"
        "06666f6f626172bb72656461637465645f6465766963655f6365727469666963617465c406"
        "666f6f626172b972656461637465645f757365725f6365727469666963617465c406666f6f"
        "626172b0757365725f6365727469666963617465c406666f6f626172"
    );

    let expected = UserCreateReqSchema {
        cmd: "user_create".to_owned(),
        user_certificate: b"foobar".to_vec(),
        device_certificate: b"foobar".to_vec(),
        redacted_user_certificate: b"foobar".to_vec(),
        redacted_device_certificate: b"foobar".to_vec(),
    };

    let schema = UserCreateReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = UserCreateReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_user_create_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    let data = hex!("81a6737461747573a26f6b");

    let expected = UserCreateRepSchema { status: Status::Ok };

    let schema = UserCreateRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = UserCreateRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_user_revoke_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "user_revoke"
    //   revoked_user_certificate: hex!("666f6f626172")
    let data = hex!(
        "82a3636d64ab757365725f7265766f6b65b87265766f6b65645f757365725f636572746966"
        "6963617465c406666f6f626172"
    );

    let expected = UserRevokeReqSchema {
        cmd: "user_revoke".to_owned(),
        revoked_user_certificate: b"foobar".to_vec(),
    };

    let schema = UserRevokeReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = UserRevokeReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_user_revoke_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    let data = hex!("81a6737461747573a26f6b");

    let expected = UserRevokeRepSchema { status: Status::Ok };

    let schema = UserRevokeRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = UserRevokeRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_device_create_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "device_create"
    //   device_certificate: hex!("666f6f626172")
    //   redacted_device_certificate: hex!("666f6f626172")
    let data = hex!(
        "83a3636d64ad6465766963655f637265617465b26465766963655f63657274696669636174"
        "65c406666f6f626172bb72656461637465645f6465766963655f6365727469666963617465"
        "c406666f6f626172"
    );

    let expected = DeviceCreateReqSchema {
        cmd: "device_create".to_owned(),
        device_certificate: b"foobar".to_vec(),
        redacted_device_certificate: b"foobar".to_vec(),
    };

    let schema = DeviceCreateReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = DeviceCreateReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_device_create_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    let data = hex!("81a6737461747573a26f6b");

    let expected = DeviceCreateRepSchema { status: Status::Ok };

    let schema = DeviceCreateRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = DeviceCreateRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_human_find_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "human_find"
    //   omit_non_human: false
    //   omit_revoked: false
    //   page: 8
    //   per_page: 8
    //   query: "foobar"
    let data = hex!(
        "86a3636d64aa68756d616e5f66696e64ae6f6d69745f6e6f6e5f68756d616ec2ac6f6d6974"
        "5f7265766f6b6564c2a47061676508a87065725f7061676508a57175657279a6666f6f6261"
        "72"
    );

    let expected = HumanFindReqSchema {
        cmd: "human_find".to_owned(),
        query: Some("foobar".to_owned()),
        omit_revoked: false,
        omit_non_human: false,
        page: NonZeroU64::new(8).unwrap(),
        per_page: NonZeroU64::new(8).unwrap(),
    };

    let schema = HumanFindReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = HumanFindReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_human_find_rep() {
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
    let data = hex!(
        "85a47061676508a87065725f7061676508a7726573756c74739183ac68756d616e5f68616e"
        "646c6592a8626f624064657631a3626f62a7757365725f6964d92031303962363862613563"
        "64663432386561303031376663366263633034643461a77265766f6b6564c2a67374617475"
        "73a26f6ba5746f74616c08"
    );

    let expected = HumanFindRepSchema {
        status: Status::Ok,
        results: vec![HumanFindResultItemSchema {
            user_id: "109b68ba5cdf428ea0017fc6bcc04d4a".parse().unwrap(),
            human_handle: Some(HumanHandle::new("bob@dev1", "bob").unwrap()),
            revoked: false,
        }],
        page: NonZeroU64::new(8).unwrap(),
        per_page: NonZeroU64::new(8).unwrap(),
        total: 8,
    };

    let schema = HumanFindRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = HumanFindRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}
