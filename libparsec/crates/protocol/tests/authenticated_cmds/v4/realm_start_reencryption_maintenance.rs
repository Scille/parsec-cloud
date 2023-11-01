// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::collections::HashMap;

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::authenticated_cmds;

// Request

pub fn req() {
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
            Bytes::from_static(b"foobar"),
        )]),
    };

    let expected = authenticated_cmds::AnyCmdReq::RealmStartReencryptionMaintenance(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::RealmStartReencryptionMaintenance(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    let raw = hex!("81a6737461747573a26f6b");

    let expected = authenticated_cmds::realm_start_reencryption_maintenance::Rep::Ok;

    let data = authenticated_cmds::realm_start_reencryption_maintenance::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_start_reencryption_maintenance::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_not_allowed() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_allowed"
    let raw = hex!("81a6737461747573ab6e6f745f616c6c6f776564");

    let expected = authenticated_cmds::realm_start_reencryption_maintenance::Rep::NotAllowed;

    let data = authenticated_cmds::realm_start_reencryption_maintenance::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_start_reencryption_maintenance::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_not_found() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "not_found"
    let raw = hex!("82a6726561736f6ea6666f6f626172a6737461747573a96e6f745f666f756e64");

    let expected = authenticated_cmds::realm_start_reencryption_maintenance::Rep::NotFound {
        reason: Some("foobar".to_owned()),
    };

    let data = authenticated_cmds::realm_start_reencryption_maintenance::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_start_reencryption_maintenance::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_bad_encryption_revision() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "bad_encryption_revision"
    let raw = hex!("81a6737461747573b76261645f656e6372797074696f6e5f7265766973696f6e");

    let expected =
        authenticated_cmds::realm_start_reencryption_maintenance::Rep::BadEncryptionRevision;

    let data = authenticated_cmds::realm_start_reencryption_maintenance::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_start_reencryption_maintenance::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_participant_mismatch() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "participant_mismatch"
    let raw = hex!(
        "82a6726561736f6ea6666f6f626172a6737461747573b47061727469636970616e745f6d69"
        "736d61746368"
    );

    let expected =
        authenticated_cmds::realm_start_reencryption_maintenance::Rep::ParticipantMismatch {
            reason: Some("foobar".to_owned()),
        };

    let data = authenticated_cmds::realm_start_reencryption_maintenance::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_start_reencryption_maintenance::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_maintenance_error() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "maintenance_error"
    let raw = hex!(
        "82a6726561736f6ea6666f6f626172a6737461747573b16d61696e74656e616e63655f6572"
        "726f72"
    );

    let expected =
        authenticated_cmds::realm_start_reencryption_maintenance::Rep::MaintenanceError {
            reason: Some("foobar".to_owned()),
        };

    let data = authenticated_cmds::realm_start_reencryption_maintenance::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_start_reencryption_maintenance::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_in_maintenance() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "in_maintenance"
    let raw = hex!("81a6737461747573ae696e5f6d61696e74656e616e6365");

    let expected = authenticated_cmds::realm_start_reencryption_maintenance::Rep::InMaintenance;

    let data = authenticated_cmds::realm_start_reencryption_maintenance::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_start_reencryption_maintenance::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_bad_timestamp() {
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
    let raw = hex!("81a6737461747573ad6261645f74696d657374616d70");

    let err =
        authenticated_cmds::realm_start_reencryption_maintenance::Rep::load(&raw).unwrap_err();
    let expected_err = rmp_serde::decode::Error::Syntax("missing field `backend_timestamp`".into());

    assert!(matches!(err, expected_err));

    // Generated from Python implementation (Parsec v2.11.1+dev)
    // Content:
    //   backend_timestamp: ext(1, 946774800.0)
    //   ballpark_client_early_offset: 50.0
    //   ballpark_client_late_offset: 70.0
    //   client_timestamp: ext(1, 946774800.0)
    //   status: "bad_timestamp"
    //
    let raw = hex!(
        "85b16261636b656e645f74696d657374616d70d70141cc375188000000bc62616c6c706172"
        "6b5f636c69656e745f6561726c795f6f6666736574cb4049000000000000bb62616c6c7061"
        "726b5f636c69656e745f6c6174655f6f6666736574cb4051800000000000b0636c69656e74"
        "5f74696d657374616d70d70141cc375188000000a6737461747573ad6261645f74696d6573"
        "74616d70"
    );

    let expected = authenticated_cmds::realm_start_reencryption_maintenance::Rep::BadTimestamp {
        reason: None,
        ballpark_client_early_offset: 50.,
        ballpark_client_late_offset: 70.,
        backend_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        client_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    let data = authenticated_cmds::realm_start_reencryption_maintenance::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_start_reencryption_maintenance::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
