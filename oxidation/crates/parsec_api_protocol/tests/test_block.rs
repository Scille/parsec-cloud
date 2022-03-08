// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use hex_literal::hex;
use rstest::rstest;

use parsec_api_protocol::*;

#[rstest]
fn serde_block_create_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   block: hex!("666f6f626172")
    //   block_id: ext(2, hex!("57c629b69d6c4abbaf651cafa46dbc93"))
    //   cmd: "block_create"
    //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
    let data = hex!(
        "84a5626c6f636bc406666f6f626172a8626c6f636b5f6964d80257c629b69d6c4abbaf651c"
        "afa46dbc93a3636d64ac626c6f636b5f637265617465a87265616c6d5f6964d8021d335315"
        "7d7d4e95ad2fdea7b3bd19c5"
    );

    let expected = BlockCreateReqSchema {
        cmd: "block_create".to_owned(),
        block_id: "57c629b69d6c4abbaf651cafa46dbc93".parse().unwrap(),
        realm_id: "1d3353157d7d4e95ad2fdea7b3bd19c5".parse().unwrap(),
        block: b"foobar".to_vec(),
    };

    let schema = BlockCreateReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = BlockCreateReqSchema::load(&data2).unwrap();
    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_block_create_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    let data = hex!("81a6737461747573a26f6b");

    let expected = BlockCreateRepSchema { status: Status::Ok };

    let schema = BlockCreateRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = BlockCreateRepSchema::load(&data2).unwrap();
    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_block_read_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   block_id: ext(2, hex!("57c629b69d6c4abbaf651cafa46dbc93"))
    //   cmd: "block_read"
    let data = hex!(
        "82a8626c6f636b5f6964d80257c629b69d6c4abbaf651cafa46dbc93a3636d64aa626c6f63"
        "6b5f72656164"
    );

    let expected = BlockReadReqSchema {
        cmd: "block_read".to_owned(),
        block_id: "57c629b69d6c4abbaf651cafa46dbc93".parse().unwrap(),
    };

    let schema = BlockReadReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = BlockReadReqSchema::load(&data2).unwrap();
    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_block_read_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   block: hex!("666f6f626172")
    //   status: "ok"
    let data = hex!("82a5626c6f636bc406666f6f626172a6737461747573a26f6b");

    let expected = BlockReadRepSchema {
        status: Status::Ok,
        block: b"foobar".to_vec(),
    };

    let schema = BlockReadRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = BlockReadRepSchema::load(&data2).unwrap();
    assert_eq!(schema2, expected);
}
