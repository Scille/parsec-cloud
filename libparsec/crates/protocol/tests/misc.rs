// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_protocol::{
    authenticated_cmds::latest as authenticated_cmds, invited_cmds::latest as invited_cmds,
};
use libparsec_tests_fixtures::prelude::*;

#[parsec_test]
#[case::invalid_msg_format(
    // Generated from Python implementation (Parsec v2.9.2+dev)
    // Content:
    //   status: "invalid_msg_format"
    //
    &hex!(
        "81a6737461747573b2696e76616c69645f6d73675f666f726d6174"
    )[..],
    authenticated_cmds::block_read::Rep::UnknownStatus {
        unknown_status: "invalid_msg_format".into(),
        reason: None
    }
)]
#[case::invalid_msg_format_with_reason(
    // Generated from Python implementation (Parsec v2.11.1+dev)
    // Content:
    //   reason: "reason"
    //   status: "invalid_msg_format"
    //
    &hex!(
        "82a6726561736f6ea6726561736f6ea6737461747573b2696e76616c69645f6d73675f666f"
        "726d6174"
    )[..],
    authenticated_cmds::block_read::Rep::UnknownStatus {
        unknown_status: "invalid_msg_format".into(),
        reason: Some("reason".into())
    }
)]
// We don't test serialization round trip, because wrong data should not be serializable
fn serde_block_read_rep(#[case] raw: &[u8], #[case] expected: authenticated_cmds::block_read::Rep) {
    let data = authenticated_cmds::block_read::Rep::load(raw).unwrap();

    assert_eq!(data, expected);
}

#[parsec_test]
#[case::invalid_type_deserialization(
    // Generated from msgpack
    // Content:
    //  status': 'ok'
    //  type: 'USER'
    //  claimer_email: 'a@a.com'
    //  greeter_user_id: 'aa'
    //  greeter_human_handle: 42 // Invalid field
    //
    &hex!(
        "85a6737461747573a26f6ba474797065a455534552ad636c61696d65725f656d61696ca761"
        "40612e636f6daf677265657465725f757365725f6964a26161b4677265657465725f68756d"
        "616e5f68616e646c652a"
    )[..],
    "invalid type: integer `42`, expected a tuple of size 2",
)]
fn serde_invalid_type_deserialization(#[case] raw: &[u8], #[case] expected: &str) {
    let err = invited_cmds::invite_info::Rep::load(raw)
        .unwrap_err()
        .to_string();
    assert_eq!(err, expected)
}

#[parsec_test]
fn authenticated_load_response() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   pong: "pong"
    //   status: "ok"
    let raw = hex!("82a4706f6e67a4706f6e67a6737461747573a26f6b");

    let rep = authenticated_cmds::ping::Req::load_response(&raw).unwrap();

    assert_eq!(
        rep,
        authenticated_cmds::ping::Rep::Ok {
            pong: "pong".to_owned(),
        }
    )
}

#[parsec_test]
fn invited_load_response() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   pong: "pong"
    //   status: "ok"
    let raw = hex!("82a4706f6e67a4706f6e67a6737461747573a26f6b");

    let rep = invited_cmds::ping::Req::load_response(&raw).unwrap();

    assert_eq!(
        rep,
        invited_cmds::ping::Rep::Ok {
            pong: "pong".to_owned(),
        }
    )
}
