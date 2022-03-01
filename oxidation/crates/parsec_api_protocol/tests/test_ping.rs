// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use hex_literal::hex;
use rstest::rstest;

use parsec_api_protocol::*;

#[rstest]
fn serde_ping_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "ping"
    //   ping: "ping"
    let data = hex!("82a3636d64a470696e67a470696e67a470696e67");

    let expected = PingReqSchema {
        cmd: "ping".to_owned(),
        ping: "ping".to_owned(),
    };

    let schema = PingReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    // Note we cannot just compare with `data` due to signature and keys order
    let schema2 = PingReqSchema::load(&data2).unwrap();
    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_ping_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   pong: "pong"
    //   status: "ok"
    let data = hex!("82a4706f6e67a4706f6e67a6737461747573a26f6b");

    let expected = PingRepSchema::Ok {
        pong: "pong".to_owned(),
    };

    let schema = PingRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    // Note we cannot just compare with `data` due to signature and keys order
    let schema2 = PingRepSchema::load(&data2).unwrap();
    assert_eq!(schema2, expected);
}
