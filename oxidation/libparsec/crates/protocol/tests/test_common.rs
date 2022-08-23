// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;
use rstest::rstest;

use libparsec_protocol::*;

#[rstest]
#[case::invalid_msg_format(
    (
        // Generated from Python implementation (Parsec v2.9.2+dev)
        // Content:
        //   status: "invalid_msg_format"
        //
        &hex!(
            "81a6737461747573b2696e76616c69645f6d73675f666f726d6174"
        )[..],
        authenticated_cmds::block_read::Rep::UnknownStatus {
            _status: "invalid_msg_format".into(),
            reason: None
        }
    )
)]
#[case::invalid_msg_format_with_reason(
    (
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
            _status: "invalid_msg_format".into(),
            reason: Some("reason".into())
        }
    )
)]
// We don't test serialization round trip, because wrong data should not be serializable
fn serde_block_read_rep(#[case] raw_expected: (&[u8], authenticated_cmds::block_read::Rep)) {
    let (raw, expected) = raw_expected;

    let data = authenticated_cmds::block_read::Rep::load(raw).unwrap();

    assert_eq!(data, expected);
}
