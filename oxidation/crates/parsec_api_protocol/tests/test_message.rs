// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use hex_literal::hex;
use rstest::rstest;

use parsec_api_protocol::*;

#[rstest]
fn serde_message_get_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "message_get"
    //   offset: 8
    let raw = hex!("82a3636d64ab6d6573736167655f676574a66f666673657408");

    let req = authenticated_cmds::message_get::Req { offset: 8 };

    let expected = authenticated_cmds::AnyCmdReq::MessageGet(req.clone());

    let data = authenticated_cmds::AnyCmdReq::loads(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dumps().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::loads(&raw2).unwrap();

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
        messages: vec![Message {
            count: 1,
            sender: "alice@dev1".parse().unwrap(),
            timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
            body: b"foobar".to_vec(),
        }],
    };

    let data = authenticated_cmds::message_get::Rep::loads(&raw);

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dumps().unwrap();

    let data2 = authenticated_cmds::message_get::Rep::loads(&raw2);

    assert_eq!(data2, expected);
}
