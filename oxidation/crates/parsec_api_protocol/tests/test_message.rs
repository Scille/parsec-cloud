// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use chrono::{TimeZone, Utc};
use hex_literal::hex;
use rstest::rstest;

use parsec_api_protocol::*;

#[rstest]
fn serde_message_get_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "message_get"
    //   offset: 8
    let data = hex!("82a3636d64ab6d6573736167655f676574a66f666673657408");

    let expected = MessageGetReqSchema {
        cmd: "message_get".to_owned(),
        offset: 8,
    };

    let schema = MessageGetReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = MessageGetReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_message_get_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   messages: [
    //     {
    //       timestamp: ext(1, 946774800.0)
    //       body: hex!("666f6f626172")
    //       count: 1
    //       sender: "alice@dev1"
    //     }
    //   ]
    //   status: "ok"
    let data = hex!(
        "82a86d657373616765739184a673656e646572aa616c6963654064657631a4626f6479c406"
        "666f6f626172a974696d657374616d70d70141cc375188000000a5636f756e7401a6737461"
        "747573a26f6b"
    );

    let expected = MessageGetRepSchema {
        status: Status::Ok,
        messages: vec![MessageSchema {
            count: 1,
            sender: "alice@dev1".parse().unwrap(),
            timestamp: Utc.ymd(2000, 1, 2).and_hms(1, 0, 0),
            body: b"foobar".to_vec(),
        }],
    };

    let schema = MessageGetRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = MessageGetRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}
