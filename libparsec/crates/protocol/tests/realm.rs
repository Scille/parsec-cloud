// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_protocol::authenticated_cmds::v3 as authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

#[parsec_test]
fn serde_realm_finish_reencryption_maintenance_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "realm_finish_reencryption_maintenance"
    //   encryption_revision: 8
    //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
    let raw = hex!(
        "83a3636d64d9257265616c6d5f66696e6973685f7265656e6372797074696f6e5f6d61696e"
        "74656e616e6365b3656e6372797074696f6e5f7265766973696f6e08a87265616c6d5f6964"
        "d8021d3353157d7d4e95ad2fdea7b3bd19c5"
    );

    let req = authenticated_cmds::realm_finish_reencryption_maintenance::Req {
        realm_id: VlobID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
        encryption_revision: 8,
    };

    let expected = authenticated_cmds::AnyCmdReq::RealmFinishReencryptionMaintenance(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = if let authenticated_cmds::AnyCmdReq::RealmFinishReencryptionMaintenance(data) = data
    {
        data.dump().unwrap()
    } else {
        unreachable!()
    };

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
#[case::ok(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    &hex!(
        "81a6737461747573a26f6b"
    )[..],
    authenticated_cmds::realm_finish_reencryption_maintenance::Rep::Ok
)]
#[case::not_allowed(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_allowed"
    &hex!(
        "81a6737461747573ab6e6f745f616c6c6f776564"
    )[..],
    authenticated_cmds::realm_finish_reencryption_maintenance::Rep::NotAllowed
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "not_found"
    &hex!(
        "82a6726561736f6ea6666f6f626172a6737461747573a96e6f745f666f756e64"
    )[..],
    authenticated_cmds::realm_finish_reencryption_maintenance::Rep::NotFound {
        reason: Some("foobar".to_owned())
    }
)]
#[case::bad_encryption_revision(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "bad_encryption_revision"
    &hex!(
        "81a6737461747573b76261645f656e6372797074696f6e5f7265766973696f6e"
    )[..],
    authenticated_cmds::realm_finish_reencryption_maintenance::Rep::BadEncryptionRevision
)]
#[case::not_in_maintenance(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "not_in_maintenance"
    &hex!(
        "82a6726561736f6ea6666f6f626172a6737461747573b26e6f745f696e5f6d61696e74656e"
        "616e6365"
    )[..],
    authenticated_cmds::realm_finish_reencryption_maintenance::Rep::NotInMaintenance {
        reason: Some("foobar".to_owned())
    }
)]
#[case::maintenance_error(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "maintenance_error"
    &hex!(
        "82a6726561736f6ea6666f6f626172a6737461747573b16d61696e74656e616e63655f6572"
        "726f72"
    )[..],
    authenticated_cmds::realm_finish_reencryption_maintenance::Rep::MaintenanceError {
        reason: Some("foobar".to_owned())
    }
)]
fn serde_realm_finish_reencryption_maintenance_rep(
    #[case] raw: &[u8],
    #[case] expected: authenticated_cmds::realm_finish_reencryption_maintenance::Rep,
) {
    let data = authenticated_cmds::realm_finish_reencryption_maintenance::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 =
        authenticated_cmds::realm_finish_reencryption_maintenance::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}
