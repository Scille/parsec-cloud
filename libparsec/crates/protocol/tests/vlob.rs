// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_protocol::authenticated_cmds::v3 as authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;

#[parsec_test]
#[case::ok(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   author: "alice@dev1"
    //   timestamp: ext(1, 946774800.0)
    //   version: 8
    //   author_last_role_granted_on: ext(1, 946774800.0)
    //   blob: hex!("666f6f626172")
    //   status: "ok"
    &hex!(
        "86a6617574686f72aa616c6963654064657631bb617574686f725f6c6173745f726f6c655f"
    "6772616e7465645f6f6ed70141cc375188000000a4626c6f62c406666f6f626172a6737461"
    "747573a26f6ba974696d657374616d70d70141cc375188000000a776657273696f6e08"
    )[..],
    authenticated_cmds::vlob_read::Rep::Ok {
        version: 8,
        blob: b"foobar".as_ref().into(),
        author: "alice@dev1".parse().unwrap(),
        timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        author_last_role_granted_on: "2000-1-2T01:00:00Z".parse().unwrap(),
    }
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "not_found"
    &hex!(
        "82a6726561736f6ea6666f6f626172a6737461747573a96e6f745f666f756e64"
    )[..],
    authenticated_cmds::vlob_read::Rep::NotFound {
        reason: Some("foobar".to_owned()),
    }
)]
#[case::not_allowed(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_allowed"
    &hex!(
        "81a6737461747573ab6e6f745f616c6c6f776564"
    )[..],
    authenticated_cmds::vlob_read::Rep::NotAllowed
)]
#[case::bad_version(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "bad_version"
    &hex!(
        "81a6737461747573ab6261645f76657273696f6e"
    )[..],
    authenticated_cmds::vlob_read::Rep::BadVersion
)]
#[case::bad_encryption_revision(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "bad_encryption_revision"
    &hex!(
        "81a6737461747573b76261645f656e6372797074696f6e5f7265766973696f6e"
    )[..],
    authenticated_cmds::vlob_read::Rep::BadEncryptionRevision
)]
#[case::in_maintenance(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "in_maintenance
    &hex!(
        "81a6737461747573ae696e5f6d61696e74656e616e6365"
    )[..],
    authenticated_cmds::vlob_read::Rep::InMaintenance
)]
fn serde_vlob_read_rep(#[case] raw: &[u8], #[case] expected: authenticated_cmds::vlob_read::Rep) {
    let data = authenticated_cmds::vlob_read::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_read::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}
