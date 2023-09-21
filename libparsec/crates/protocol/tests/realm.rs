// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]
use std::collections::HashMap;

use libparsec_protocol::authenticated_cmds::v3 as authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

#[parsec_test]
fn serde_realm_start_reencryption_maintenance_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   timestamp: ext(1, 946774800.0)
    //   cmd: "realm_start_reencryption_maintenance"
    //   encryption_revision: 8
    //   per_participant_message: {109b68ba5cdf428ea0017fc6bcc04d4a:hex!("666f6f626172")}
    //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
    let raw = hex!(
        "85a3636d64d9247265616c6d5f73746172745f7265656e6372797074696f6e5f6d61696e74"
        "656e616e6365b3656e6372797074696f6e5f7265766973696f6e08b77065725f7061727469"
        "636970616e745f6d65737361676581d9203130396236386261356364663432386561303031"
        "376663366263633034643461c406666f6f626172a87265616c6d5f6964d8021d3353157d7d"
        "4e95ad2fdea7b3bd19c5a974696d657374616d70d70141cc375188000000"
    );

    let req = authenticated_cmds::realm_start_reencryption_maintenance::Req {
        realm_id: VlobID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
        encryption_revision: 8,
        timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        per_participant_message: HashMap::from([(
            "109b68ba5cdf428ea0017fc6bcc04d4a".parse().unwrap(),
            b"foobar".as_ref().into(),
        )]),
    };

    let expected = authenticated_cmds::AnyCmdReq::RealmStartReencryptionMaintenance(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = if let authenticated_cmds::AnyCmdReq::RealmStartReencryptionMaintenance(data) = data
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
    authenticated_cmds::realm_start_reencryption_maintenance::Rep::Ok
)]
#[case::not_allowed(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_allowed"
    &hex!(
        "81a6737461747573ab6e6f745f616c6c6f776564"
    )[..],
    authenticated_cmds::realm_start_reencryption_maintenance::Rep::NotAllowed
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "not_found"
    &hex!(
        "82a6726561736f6ea6666f6f626172a6737461747573a96e6f745f666f756e64"
    )[..],
    authenticated_cmds::realm_start_reencryption_maintenance::Rep::NotFound {
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
    authenticated_cmds::realm_start_reencryption_maintenance::Rep::BadEncryptionRevision
)]
#[case::participant_mismatch(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "participant_mismatch"
    &hex!(
        "82a6726561736f6ea6666f6f626172a6737461747573b47061727469636970616e745f6d69"
        "736d61746368"
    )[..],
    authenticated_cmds::realm_start_reencryption_maintenance::Rep::ParticipantMismatch {
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
    authenticated_cmds::realm_start_reencryption_maintenance::Rep::MaintenanceError {
        reason: Some("foobar".to_owned())
    }
)]
#[case::in_maintenance(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "in_maintenance"
    &hex!(
        "81a6737461747573ae696e5f6d61696e74656e616e6365"
    )[..],
    authenticated_cmds::realm_start_reencryption_maintenance::Rep::InMaintenance
)]
#[should_panic(expected = "missing field `backend_timestamp`")]
#[case::bad_timestamp_legacy(
    // Generated from Python implementation (Parsec v2.8.1+dev)
    // Content:
    //   status: "bad_timestamp"
    //
    // Note that raw data does not contain:
    //  - ballpark_client_early_offset
    //  - ballpark_client_late_offset
    //  - backend_timestamp
    //  - client_timestamp
    // This was valid behavior in api v2 but is no longer valid from v3 onwards.
    // The corresponding expected values used here are therefore not important
    // since loading raw data should fail.
    //
    &hex!("81a6737461747573ad6261645f74696d657374616d70")[..],
    authenticated_cmds::realm_start_reencryption_maintenance::Rep::BadTimestamp {
        reason: None,
        ballpark_client_early_offset: 0.,
        ballpark_client_late_offset: 0.,
        backend_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        client_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
    }
)]
#[case::bad_timestamp(
    // Generated from Python implementation (Parsec v2.11.1+dev)
    // Content:
    //   backend_timestamp: ext(1, 946774800.0)
    //   ballpark_client_early_offset: 50.0
    //   ballpark_client_late_offset: 70.0
    //   client_timestamp: ext(1, 946774800.0)
    //   status: "bad_timestamp"
    //
    &hex!(
        "85b16261636b656e645f74696d657374616d70d70141cc375188000000bc62616c6c706172"
        "6b5f636c69656e745f6561726c795f6f6666736574cb4049000000000000bb62616c6c7061"
        "726b5f636c69656e745f6c6174655f6f6666736574cb4051800000000000b0636c69656e74"
        "5f74696d657374616d70d70141cc375188000000a6737461747573ad6261645f74696d6573"
        "74616d70"
    )[..],
    authenticated_cmds::realm_start_reencryption_maintenance::Rep::BadTimestamp {
        reason: None,
        ballpark_client_early_offset: 50.,
        ballpark_client_late_offset: 70.,
        backend_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        client_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
    }
)]
fn serde_realm_start_reencryption_maintenance_rep(
    #[case] raw: &[u8],
    #[case] expected: authenticated_cmds::realm_start_reencryption_maintenance::Rep,
) {
    let data = authenticated_cmds::realm_start_reencryption_maintenance::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_start_reencryption_maintenance::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

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
