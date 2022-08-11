// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;
use rstest::rstest;

use libparsec_protocol::*;

#[rstest]
fn serde_message_get_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "message_get"
    //   offset: 8
    let raw = hex!("82a3636d64ab6d6573736167655f676574a66f666673657408");

    let req = authenticated_cmds::message_get::Req { offset: 8 };

    let expected = authenticated_cmds::AnyCmdReq::MessageGet(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
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
    let raw = hex!(
        "82a86d657373616765739184a673656e646572aa616c6963654064657631a4626f6479c406"
        "666f6f626172a974696d657374616d70d70141cc375188000000a5636f756e7401a6737461"
        "747573a26f6b"
    );

    let expected = authenticated_cmds::message_get::Rep::Ok {
        messages: vec![authenticated_cmds::message_get::Message {
            count: 1,
            sender: "alice@dev1".parse().unwrap(),
            timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
            body: b"foobar".to_vec(),
        }],
    };

    let data = authenticated_cmds::message_get::Rep::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::message_get::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}
