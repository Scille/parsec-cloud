// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use hex_literal::hex;
use rstest::rstest;
use std::collections::HashMap;

use libparsec_protocol::authenticated_cmds::v2 as authenticated_cmds;
use libparsec_types::{Maybe, RealmID};

#[rstest]
fn serde_realm_create_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "realm_create"
    //   role_certificate: hex!("666f6f626172")
    let raw = hex!(
        "82a3636d64ac7265616c6d5f637265617465b0726f6c655f6365727469666963617465c406"
        "666f6f626172"
    );

    let req = authenticated_cmds::realm_create::Req {
        role_certificate: b"foobar".to_vec(),
    };

    let expected = authenticated_cmds::AnyCmdReq::RealmCreate(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "ok"
        &hex!(
            "81a6737461747573a26f6b"
        )[..],
        authenticated_cmds::realm_create::Rep::Ok
    )
)]
#[case::invalid_certification(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "invalid_certification"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573b5696e76616c69645f636572746966"
            "69636174696f6e"
        )[..],
        authenticated_cmds::realm_create::Rep::InvalidCertification {
            reason: Some("foobar".to_owned()),
        }
    )
)]
#[case::invalid_raw(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "invalid_raw"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573ac696e76616c69645f64617461"
        )[..],
        authenticated_cmds::realm_create::Rep::InvalidData {
            reason: Some("foobar".to_owned()),
        }
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "not_found"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573a96e6f745f666f756e64"
        )[..],
        authenticated_cmds::realm_create::Rep::NotFound {
            reason: Some("foobar".to_owned()),
        }
    )
)]
#[case::already_exists(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "already_exists"
        &hex!(
            "81a6737461747573ae616c72656164795f657869737473"
        )[..],
        authenticated_cmds::realm_create::Rep::AlreadyExists
    )
)]
#[case::bad_timestamp_legacy(
    (
        // Generated from Python implementation (Parsec v2.11.1+dev)
        // Content:
        //   status: "bad_timestamp"
        //
        &hex!("81a6737461747573ad6261645f74696d657374616d70")[..],
        authenticated_cmds::realm_create::Rep::BadTimestamp {
            reason: None,
            ballpark_client_early_offset: Maybe::Absent,
            ballpark_client_late_offset: Maybe::Absent,
            backend_timestamp: Maybe::Absent,
            client_timestamp: Maybe::Absent,
        }
    )
)]
#[case::bad_timestamp(
    (
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
        authenticated_cmds::realm_create::Rep::BadTimestamp {
            reason: None,
            ballpark_client_early_offset: Maybe::Present(50.),
            ballpark_client_late_offset: Maybe::Present(70.),
            backend_timestamp: Maybe::Present("2000-1-2T01:00:00Z".parse().unwrap()),
            client_timestamp: Maybe::Present("2000-1-2T01:00:00Z".parse().unwrap()),
        }
    )
)]
fn serde_realm_create_rep(#[case] raw_expected: (&[u8], authenticated_cmds::realm_create::Rep)) {
    let (raw, expected) = raw_expected;

    let data = authenticated_cmds::realm_create::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_create::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
fn serde_realm_status_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "realm_status"
    //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
    let raw = hex!(
        "82a3636d64ac7265616c6d5f737461747573a87265616c6d5f6964d8021d3353157d7d4e95"
        "ad2fdea7b3bd19c5"
    );

    let req = authenticated_cmds::realm_status::Req {
        realm_id: RealmID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
    };

    let expected = authenticated_cmds::AnyCmdReq::RealmStatus(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::ok_without(
    (
        // Generated from Rust implementation (Parsec v2.12.1+dev)
        // Content:
        //   encryption_revision: 8
        //   in_maintenance: true
        //   maintenance_started_by: None
        //   maintenance_started_on: None
        //   maintenance_type: "REENCRYPTION"
        //   status: "ok"
        //
        &hex!(
            "86a6737461747573a26f6bae696e5f6d61696e74656e616e6365c3b06d61696e74656e616e"
            "63655f74797065ac5245454e4352595054494f4eb66d61696e74656e616e63655f73746172"
            "7465645f6f6ec0b66d61696e74656e616e63655f737461727465645f6279c0b3656e637279"
            "7074696f6e5f7265766973696f6e08"
        )[..],
        authenticated_cmds::realm_status::Rep::Ok {
            in_maintenance: true,
            maintenance_type: Some(authenticated_cmds::realm_status::MaintenanceType::Reencryption),
            maintenance_started_on: None,
            maintenance_started_by: None,
            encryption_revision: 8,
        }
    )
)]
#[case::ok_full(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   encryption_revision: 8
        //   in_maintenance: true
        //   maintenance_started_by: "alice@dev1"
        //   maintenance_started_on: ext(1, 946774800.0)
        //   maintenance_type: "GARBAGE_COLLECTION"
        //   status: "ok"
        &hex!(
            "86b3656e6372797074696f6e5f7265766973696f6e08ae696e5f6d61696e74656e616e6365"
            "c3b66d61696e74656e616e63655f737461727465645f6279aa616c6963654064657631b66d"
            "61696e74656e616e63655f737461727465645f6f6ed70141cc375188000000b06d61696e74"
            "656e616e63655f74797065b2474152424147455f434f4c4c454354494f4ea6737461747573"
            "a26f6b"
        )[..],
        authenticated_cmds::realm_status::Rep::Ok {
            in_maintenance: true,
            maintenance_type: Some(authenticated_cmds::realm_status::MaintenanceType::GarbageCollection),
            maintenance_started_on: Some("2000-1-2T01:00:00Z".parse().unwrap()),
            maintenance_started_by: Some("alice@dev1".parse().unwrap()),
            encryption_revision: 8,
        }
    )
)]
#[case::not_allowed(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_allowed"
        &hex!(
            "81a6737461747573ab6e6f745f616c6c6f776564"
        )[..],
        authenticated_cmds::realm_status::Rep::NotAllowed
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "not_found"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573a96e6f745f666f756e64"
        )[..],
        authenticated_cmds::realm_status::Rep::NotFound {
            reason: Some("foobar".to_owned())
        }
    )
)]
fn serde_realm_status_rep(#[case] raw_expected: (&[u8], authenticated_cmds::realm_status::Rep)) {
    let (raw, expected) = raw_expected;

    let data = authenticated_cmds::realm_status::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_status::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
fn serde_realm_stats_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "realm_stats"
    //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
    let raw = hex!(
        "82a3636d64ab7265616c6d5f7374617473a87265616c6d5f6964d8021d3353157d7d4e95ad"
        "2fdea7b3bd19c5"
    );

    let req = authenticated_cmds::realm_stats::Req {
        realm_id: RealmID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
    };

    let expected = authenticated_cmds::AnyCmdReq::RealmStats(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   blocks_size: 8
        //   status: "ok"
        //   vlobs_size: 8
        &hex!(
            "83ab626c6f636b735f73697a6508a6737461747573a26f6baa766c6f62735f73697a6508"
        )[..],
        authenticated_cmds::realm_stats::Rep::Ok {
            blocks_size: 8,
            vlobs_size: 8,
        }
    )
)]
#[case::not_allowed(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_allowed"
        &hex!(
            "81a6737461747573ab6e6f745f616c6c6f776564"
        )[..],
        authenticated_cmds::realm_stats::Rep::NotAllowed
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "not_found"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573a96e6f745f666f756e64"
        )[..],
        authenticated_cmds::realm_stats::Rep::NotFound {
            reason: Some("foobar".to_owned())
        }
    )
)]
fn serde_realm_stats_rep(#[case] raw_expected: (&[u8], authenticated_cmds::realm_stats::Rep)) {
    let (raw, expected) = raw_expected;

    let data = authenticated_cmds::realm_stats::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_stats::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
fn serde_realm_get_role_certificates_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "realm_get_role_certificates"
    //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
    let raw = hex!(
        "82a3636d64bb7265616c6d5f6765745f726f6c655f636572746966696361746573a8726561"
        "6c6d5f6964d8021d3353157d7d4e95ad2fdea7b3bd19c5"
    );

    let req = authenticated_cmds::realm_get_role_certificates::Req {
        realm_id: RealmID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
    };

    let expected = authenticated_cmds::AnyCmdReq::RealmGetRoleCertificates(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   certificates: [hex!("666f6f626172")]
        //   status: "ok"
        &hex!(
            "82ac63657274696669636174657391c406666f6f626172a6737461747573a26f6b"
        )[..],
        authenticated_cmds::realm_get_role_certificates::Rep::Ok {
            certificates: vec![b"foobar".to_vec()],
        }
    )
)]
#[case::not_allowed(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_allowed"
        &hex!(
            "81a6737461747573ab6e6f745f616c6c6f776564"
        )[..],
        authenticated_cmds::realm_get_role_certificates::Rep::NotAllowed
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "not_found"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573a96e6f745f666f756e64"
        )[..],
        authenticated_cmds::realm_get_role_certificates::Rep::NotFound {
            reason: Some("foobar".to_owned())
        }
    )
)]
fn serde_realm_get_role_certificates_rep(
    #[case] raw_expected: (&[u8], authenticated_cmds::realm_get_role_certificates::Rep),
) {
    let (raw, expected) = raw_expected;

    let data = authenticated_cmds::realm_get_role_certificates::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_get_role_certificates::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::without(
    (
        // Generated from Rust implementation (Parsec v2.12.1+dev)
        // Content:
        //   cmd: "realm_update_roles"
        //   recipient_message: None
        //   role_certificate: hex!("666f6f626172")
        //
        &hex!(
            "83a3636d64b27265616c6d5f7570646174655f726f6c6573b0726f6c655f63657274696669"
            "63617465c406666f6f626172b1726563697069656e745f6d657373616765c0"
        )[..],
        authenticated_cmds::AnyCmdReq::RealmUpdateRoles(authenticated_cmds::realm_update_roles::Req {
            role_certificate: b"foobar".to_vec(),
            recipient_message: None,
        })
    )
)]
#[case::full(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   cmd: "realm_update_roles"
        //   recipient_message: hex!("666f6f626172")
        //   role_certificate: hex!("666f6f626172")
        &hex!(
            "83a3636d64b27265616c6d5f7570646174655f726f6c6573b1726563697069656e745f6d65"
            "7373616765c406666f6f626172b0726f6c655f6365727469666963617465c406666f6f6261"
            "72"
        )[..],
        authenticated_cmds::AnyCmdReq::RealmUpdateRoles(authenticated_cmds::realm_update_roles::Req {
            role_certificate: b"foobar".to_vec(),
            recipient_message: Some(b"foobar".to_vec()),
        })
    )
)]
fn serde_realm_update_roles_req(#[case] raw_expected: (&[u8], authenticated_cmds::AnyCmdReq)) {
    let (raw, expected) = raw_expected;

    let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "ok"
        &hex!(
            "81a6737461747573a26f6b"
        )[..],
        authenticated_cmds::realm_update_roles::Rep::Ok
    )
)]
#[case::not_allowed(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "not_allowed"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573ab6e6f745f616c6c6f776564"
        )[..],
        authenticated_cmds::realm_update_roles::Rep::NotAllowed {
            reason: Some("foobar".to_owned())
        }
    )
)]
#[case::invalid_certification(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "invalid_certification"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573b5696e76616c69645f636572746966"
            "69636174696f6e"
        )[..],
        authenticated_cmds::realm_update_roles::Rep::InvalidCertification {
            reason: Some("foobar".to_owned())
        }
    )
)]
#[case::invalid_raw(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "invalid_raw"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573ac696e76616c69645f64617461"
        )[..],
        authenticated_cmds::realm_update_roles::Rep::InvalidData {
            reason: Some("foobar".to_owned())
        }
    )
)]
#[case::already_granted(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "already_granted"
        &hex!(
            "81a6737461747573af616c72656164795f6772616e746564"
        )[..],
        authenticated_cmds::realm_update_roles::Rep::AlreadyGranted
    )
)]
#[case::incompatible_profile(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "incompatible_profile"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573b4696e636f6d70617469626c655f70"
            "726f66696c65"
        )[..],
        authenticated_cmds::realm_update_roles::Rep::IncompatibleProfile {
            reason: Some("foobar".to_owned())
        }
    )
)]
#[case::not_found(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "not_found"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573a96e6f745f666f756e64"
        )[..],
        authenticated_cmds::realm_update_roles::Rep::NotFound {
            reason: Some("foobar".to_owned())
        }
    )
)]
#[case::in_maintenance(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "in_maintenance"
        &hex!(
            "81a6737461747573ae696e5f6d61696e74656e616e6365"
        )[..],
        authenticated_cmds::realm_update_roles::Rep::InMaintenance
    )
)]
// TODO: Test me on API-v3
// #[case::user_revoked(
//     (
//         // Generated from Python implementation (Parsec v2.11.1+dev)
//         // Content:
//         //   status: "user_revoked"
//         //
//         &hex!("81a6737461747573ac757365725f7265766f6b6564")[..],
//         authenticated_cmds::realm_update_roles::Rep::UserRevoked
//     )
// )]
#[case::require_greater_timestamp(
    (
        // Generated from Python implementation (Parsec v2.11.1+dev)
        // Content:
        //   status: "require_greater_timestamp"
        //   strictly_greater_than: ext(1, 946774800.0)
        //
        &hex!(
            "82a6737461747573b9726571756972655f677265617465725f74696d657374616d70b57374"
            "726963746c795f677265617465725f7468616ed70141cc375188000000"
        )[..],
        authenticated_cmds::realm_update_roles::Rep::RequireGreaterTimestamp {
            strictly_greater_than: "2000-1-2T01:00:00Z".parse().unwrap(),
        }
    )
)]
#[case::bad_timestamp_legacy(
    (
        // Generated from Python implementation (Parsec v2.8.1+dev)
        // Content:
        //   status: "bad_timestamp"
        //
        &hex!("81a6737461747573ad6261645f74696d657374616d70")[..],
        authenticated_cmds::realm_update_roles::Rep::BadTimestamp {
            reason: None,
            ballpark_client_early_offset: Maybe::Absent,
            ballpark_client_late_offset: Maybe::Absent,
            backend_timestamp: Maybe::Absent,
            client_timestamp: Maybe::Absent,
        }
    )
)]
#[case::bad_timestamp(
    (
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
        authenticated_cmds::realm_update_roles::Rep::BadTimestamp {
            reason: None,
            ballpark_client_early_offset: Maybe::Present(50.),
            ballpark_client_late_offset: Maybe::Present(70.),
            backend_timestamp: Maybe::Present("2000-1-2T01:00:00Z".parse().unwrap()),
            client_timestamp: Maybe::Present("2000-1-2T01:00:00Z".parse().unwrap()),
        }
    )
)]
fn serde_realm_update_roles_rep(
    #[case] raw_expected: (&[u8], authenticated_cmds::realm_update_roles::Rep),
) {
    let (raw, expected) = raw_expected;

    let data = authenticated_cmds::realm_update_roles::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_update_roles::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
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
        realm_id: RealmID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
        encryption_revision: 8,
        timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        per_participant_message: HashMap::from([(
            "109b68ba5cdf428ea0017fc6bcc04d4a".parse().unwrap(),
            b"foobar".to_vec(),
        )]),
    };

    let expected = authenticated_cmds::AnyCmdReq::RealmStartReencryptionMaintenance(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "ok"
        &hex!(
            "81a6737461747573a26f6b"
        )[..],
        authenticated_cmds::realm_start_reencryption_maintenance::Rep::Ok
    )
)]
#[case::not_allowed(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_allowed"
        &hex!(
            "81a6737461747573ab6e6f745f616c6c6f776564"
        )[..],
        authenticated_cmds::realm_start_reencryption_maintenance::Rep::NotAllowed
    )
)]
#[case::not_found(
    (
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
    )
)]
#[case::bad_encryption_revision(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "bad_encryption_revision"
        &hex!(
            "81a6737461747573b76261645f656e6372797074696f6e5f7265766973696f6e"
        )[..],
        authenticated_cmds::realm_start_reencryption_maintenance::Rep::BadEncryptionRevision
    )
)]
#[case::participant_mismatch(
    (
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
    )
)]
#[case::maintenance_error(
    (
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
    )
)]
#[case::in_maintenance(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "in_maintenance"
        &hex!(
            "81a6737461747573ae696e5f6d61696e74656e616e6365"
        )[..],
        authenticated_cmds::realm_start_reencryption_maintenance::Rep::InMaintenance
    )
)]
#[case::bad_timestamp_legacy(
    (
        // Generated from Python implementation (Parsec v2.8.1+dev)
        // Content:
        //   status: "bad_timestamp"
        //
        &hex!("81a6737461747573ad6261645f74696d657374616d70")[..],
        authenticated_cmds::realm_start_reencryption_maintenance::Rep::BadTimestamp {
            reason: None,
            ballpark_client_early_offset: Maybe::Absent,
            ballpark_client_late_offset: Maybe::Absent,
            backend_timestamp: Maybe::Absent,
            client_timestamp: Maybe::Absent,
        }
    )
)]
#[case::bad_timestamp(
    (
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
            ballpark_client_early_offset: Maybe::Present(50.),
            ballpark_client_late_offset: Maybe::Present(70.),
            backend_timestamp: Maybe::Present("2000-1-2T01:00:00Z".parse().unwrap()),
            client_timestamp: Maybe::Present("2000-1-2T01:00:00Z".parse().unwrap()),
        }
    )
)]
fn serde_realm_start_reencryption_maintenance_rep(
    #[case] raw_expected: (
        &[u8],
        authenticated_cmds::realm_start_reencryption_maintenance::Rep,
    ),
) {
    let (raw, expected) = raw_expected;

    let data = authenticated_cmds::realm_start_reencryption_maintenance::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::realm_start_reencryption_maintenance::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
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
        realm_id: RealmID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
        encryption_revision: 8,
    };

    let expected = authenticated_cmds::AnyCmdReq::RealmFinishReencryptionMaintenance(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "ok"
        &hex!(
            "81a6737461747573a26f6b"
        )[..],
        authenticated_cmds::realm_finish_reencryption_maintenance::Rep::Ok
    )
)]
#[case::not_allowed(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "not_allowed"
        &hex!(
            "81a6737461747573ab6e6f745f616c6c6f776564"
        )[..],
        authenticated_cmds::realm_finish_reencryption_maintenance::Rep::NotAllowed
    )
)]
#[case::not_found(
    (
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
    )
)]
#[case::bad_encryption_revision(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "bad_encryption_revision"
        &hex!(
            "81a6737461747573b76261645f656e6372797074696f6e5f7265766973696f6e"
        )[..],
        authenticated_cmds::realm_finish_reencryption_maintenance::Rep::BadEncryptionRevision
    )
)]
#[case::not_in_maintenance(
    (
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
    )
)]
#[case::maintenance_error(
    (
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
    )
)]
fn serde_realm_finish_reencryption_maintenance_rep(
    #[case] raw_expected: (
        &[u8],
        authenticated_cmds::realm_finish_reencryption_maintenance::Rep,
    ),
) {
    let (raw, expected) = raw_expected;

    let data = authenticated_cmds::realm_finish_reencryption_maintenance::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 =
        authenticated_cmds::realm_finish_reencryption_maintenance::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}
