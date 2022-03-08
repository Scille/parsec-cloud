// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use hex_literal::hex;
use rstest::rstest;

use parsec_api_protocol::*;

#[rstest]
fn serde_events_listen_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "events_listen"
    //   wait: false
    let data = hex!("82a3636d64ad6576656e74735f6c697374656ea477616974c2");

    let expected = EventsListenReqSchema {
        cmd: "events_listen".to_owned(),
        wait: false,
    };

    let schema = EventsListenReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = EventsListenReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_events_listen_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   event: "pinged"
    //   ping: "foobar"
    //   status: "ok"
    let data = hex!("83a56576656e74a670696e676564a470696e67a6666f6f626172a6737461747573a26f6b");

    let expected = EventsListenRepSchema(APIEvent::Pinged(EventsPingedRepSchema {
        status: Status::Ok,
        ping: "foobar".to_owned(),
    }));

    let schema = EventsListenRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = EventsListenRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_events_subscribe_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "events_subscribe"
    let data = hex!("81a3636d64b06576656e74735f737562736372696265");

    let expected = EventsSubscribeReqSchema {
        cmd: "events_subscribe".to_owned(),
    };

    let schema = EventsSubscribeReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = EventsSubscribeReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_events_subscribe_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    let data = hex!("81a6737461747573a26f6b");

    let expected = EventsSubscribeRepSchema { status: Status::Ok };

    let schema = EventsSubscribeRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = EventsSubscribeRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}
