// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use hex_literal::hex;
use rstest::rstest;
use std::collections::HashMap;

use parsec_api_protocol::*;

#[rstest]
fn serde_realm_create_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "realm_create"
    //   role_certificate: hex!("666f6f626172")
    let data = hex!(
        "82a3636d64ac7265616c6d5f637265617465b0726f6c655f6365727469666963617465c406"
        "666f6f626172"
    );

    let expected = RealmCreateReqSchema {
        cmd: "realm_create".to_owned(),
        role_certificate: b"foobar".to_vec(),
    };

    let schema = RealmCreateReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = RealmCreateReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
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
        RealmCreateRepSchema::Ok
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
        RealmCreateRepSchema::InvalidCertification {
            reason: "foobar".to_owned(),
        }
    )
)]
#[case::invalid_data(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "invalid_data"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573ac696e76616c69645f64617461"
        )[..],
        RealmCreateRepSchema::InvalidData {
            reason: "foobar".to_owned(),
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
        RealmCreateRepSchema::NotFound {
            reason: "foobar".to_owned(),
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
        RealmCreateRepSchema::AlreadyExists
    )
)]
fn serde_realm_create_rep(#[case] data_expected: (&[u8], RealmCreateRepSchema)) {
    let (data, expected) = data_expected;

    let schema = RealmCreateRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = RealmCreateRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_realm_status_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "realm_status"
    //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
    let data = hex!(
        "82a3636d64ac7265616c6d5f737461747573a87265616c6d5f6964d8021d3353157d7d4e95"
        "ad2fdea7b3bd19c5"
    );

    let expected = RealmStatusReqSchema {
        cmd: "realm_status".to_owned(),
        realm_id: "1d3353157d7d4e95ad2fdea7b3bd19c5".parse().unwrap(),
    };

    let schema = RealmStatusReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = RealmStatusReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
#[case::ok(
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
        RealmStatusRepSchema::Ok {
            in_maintenance: true,
            maintenance_type: Some(MaintenanceType::GarbageCollection),
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
        RealmStatusRepSchema::NotAllowed
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
        RealmStatusRepSchema::NotFound {
            reason: "foobar".to_owned()
        }
    )
)]
fn serde_realm_status_rep(#[case] data_expected: (&[u8], RealmStatusRepSchema)) {
    let (data, expected) = data_expected;

    let schema = RealmStatusRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = RealmStatusRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_realm_stats_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "realm_stats"
    //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
    let data = hex!(
        "82a3636d64ab7265616c6d5f7374617473a87265616c6d5f6964d8021d3353157d7d4e95ad"
        "2fdea7b3bd19c5"
    );

    let expected = RealmStatsReqSchema {
        cmd: "realm_stats".to_owned(),
        realm_id: "1d3353157d7d4e95ad2fdea7b3bd19c5".parse().unwrap(),
    };

    let schema = RealmStatsReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = RealmStatsReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
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
        RealmStatsRepSchema::Ok {
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
        RealmStatsRepSchema::NotAllowed
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
        RealmStatsRepSchema::NotFound {
            reason: "foobar".to_owned()
        }
    )
)]
fn serde_realm_stats_rep(#[case] data_expected: (&[u8], RealmStatsRepSchema)) {
    let (data, expected) = data_expected;

    let schema = RealmStatsRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = RealmStatsRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_realm_get_role_certificates_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "realm_get_role_certificates"
    //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
    //   since: ext(1, 946774800.0)
    let data = hex!(
        "83a3636d64bb7265616c6d5f6765745f726f6c655f636572746966696361746573a8726561"
        "6c6d5f6964d8021d3353157d7d4e95ad2fdea7b3bd19c5a573696e6365d70141cc37518800"
        "0000"
    );

    let expected = RealmGetRoleCertificatesReqSchema {
        cmd: "realm_get_role_certificates".to_owned(),
        realm_id: "1d3353157d7d4e95ad2fdea7b3bd19c5".parse().unwrap(),
        since: Some("2000-1-2T01:00:00Z".parse().unwrap()),
    };

    let schema = RealmGetRoleCertificatesReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = RealmGetRoleCertificatesReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
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
        RealmGetRoleCertificatesRepSchema::Ok {
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
        RealmGetRoleCertificatesRepSchema::NotAllowed
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
        RealmGetRoleCertificatesRepSchema::NotFound {
            reason: "foobar".to_owned()
        }
    )
)]
fn serde_realm_get_role_certificates_rep(
    #[case] data_expected: (&[u8], RealmGetRoleCertificatesRepSchema),
) {
    let (data, expected) = data_expected;

    let schema = RealmGetRoleCertificatesRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = RealmGetRoleCertificatesRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_realm_update_roles_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "realm_update_roles"
    //   recipient_message: hex!("666f6f626172")
    //   role_certificate: hex!("666f6f626172")
    let data = hex!(
        "83a3636d64b27265616c6d5f7570646174655f726f6c6573b1726563697069656e745f6d65"
        "7373616765c406666f6f626172b0726f6c655f6365727469666963617465c406666f6f6261"
        "72"
    );

    let expected = RealmUpdateRolesReqSchema {
        cmd: "realm_update_roles".to_owned(),
        role_certificate: b"foobar".to_vec(),
        recipient_message: Some(b"foobar".to_vec()),
    };

    let schema = RealmUpdateRolesReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = RealmUpdateRolesReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
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
        RealmUpdateRolesRepSchema::Ok
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
        RealmUpdateRolesRepSchema::NotAllowed {
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
        RealmUpdateRolesRepSchema::InvalidCertification {
            reason: "foobar".to_owned()
        }
    )
)]
#[case::invalid_data(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "invalid_data"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573ac696e76616c69645f64617461"
        )[..],
        RealmUpdateRolesRepSchema::InvalidData {
            reason: "foobar".to_owned()
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
        RealmUpdateRolesRepSchema::AlreadyGranted
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
        RealmUpdateRolesRepSchema::IncompatibleProfile {
            reason: "foobar".to_owned()
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
        RealmUpdateRolesRepSchema::NotFound {
            reason: "foobar".to_owned()
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
        RealmUpdateRolesRepSchema::InMaintenance
    )
)]
fn serde_realm_update_roles_rep(#[case] data_expected: (&[u8], RealmUpdateRolesRepSchema)) {
    let (data, expected) = data_expected;

    let schema = RealmUpdateRolesRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = RealmUpdateRolesRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
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
    let data = hex!(
        "85a3636d64d9247265616c6d5f73746172745f7265656e6372797074696f6e5f6d61696e74"
        "656e616e6365b3656e6372797074696f6e5f7265766973696f6e08b77065725f7061727469"
        "636970616e745f6d65737361676581d9203130396236386261356364663432386561303031"
        "376663366263633034643461c406666f6f626172a87265616c6d5f6964d8021d3353157d7d"
        "4e95ad2fdea7b3bd19c5a974696d657374616d70d70141cc375188000000"
    );

    let expected = RealmStartReencryptionMaintenanceReqSchema {
        cmd: "realm_start_reencryption_maintenance".to_owned(),
        realm_id: "1d3353157d7d4e95ad2fdea7b3bd19c5".parse().unwrap(),
        encryption_revision: 8,
        timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        per_participant_message: HashMap::from([(
            "109b68ba5cdf428ea0017fc6bcc04d4a".parse().unwrap(),
            b"foobar".to_vec(),
        )]),
    };

    let schema = RealmStartReencryptionMaintenanceReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = RealmStartReencryptionMaintenanceReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
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
        RealmStartReencryptionMaintenanceRepSchema::Ok
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
        RealmStartReencryptionMaintenanceRepSchema::NotAllowed
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
        RealmStartReencryptionMaintenanceRepSchema::NotFound {
            reason: "foobar".to_owned()
        }
    )
)]
#[case::bad_encryption(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "bad_encryption_revision"
        &hex!(
            "81a6737461747573b76261645f656e6372797074696f6e5f7265766973696f6e"
        )[..],
        RealmStartReencryptionMaintenanceRepSchema::BadEncryptionRevision
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
        RealmStartReencryptionMaintenanceRepSchema::ParticipantMismatch {
            reason: "foobar".to_owned()
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
        RealmStartReencryptionMaintenanceRepSchema::MaintenanceError {
            reason: "foobar".to_owned()
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
        RealmStartReencryptionMaintenanceRepSchema::InMaintenance
    )
)]
fn serde_realm_start_reencryption_maintenance_rep(
    #[case] data_expected: (&[u8], RealmStartReencryptionMaintenanceRepSchema),
) {
    let (data, expected) = data_expected;

    let schema = RealmStartReencryptionMaintenanceRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = RealmStartReencryptionMaintenanceRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_realm_finish_reencryption_maintenance_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "realm_finish_reencryption_maintenance"
    //   encryption_revision: 8
    //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
    let data = hex!(
        "83a3636d64d9257265616c6d5f66696e6973685f7265656e6372797074696f6e5f6d61696e"
        "74656e616e6365b3656e6372797074696f6e5f7265766973696f6e08a87265616c6d5f6964"
        "d8021d3353157d7d4e95ad2fdea7b3bd19c5"
    );

    let expected = RealmFinishReencryptionMaintenanceReqSchema {
        cmd: "realm_finish_reencryption_maintenance".to_owned(),
        realm_id: "1d3353157d7d4e95ad2fdea7b3bd19c5".parse().unwrap(),
        encryption_revision: 8,
    };

    let schema = RealmFinishReencryptionMaintenanceReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = RealmFinishReencryptionMaintenanceReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
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
        RealmFinishReencryptionMaintenanceRepSchema::Ok
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
        RealmFinishReencryptionMaintenanceRepSchema::NotAllowed
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
        RealmFinishReencryptionMaintenanceRepSchema::NotFound {
            reason: "foobar".to_owned()
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
        RealmFinishReencryptionMaintenanceRepSchema::BadEncryptionRevision
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
        RealmFinishReencryptionMaintenanceRepSchema::NotInMaintenance {
            reason: "foobar".to_owned()
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
        RealmFinishReencryptionMaintenanceRepSchema::MaintenanceError {
            reason: "foobar".to_owned()
        }
    )
)]
fn serde_realm_finish_reencryption_maintenance_rep(
    #[case] data_expected: (&[u8], RealmFinishReencryptionMaintenanceRepSchema),
) {
    let (data, expected) = data_expected;

    let schema = RealmFinishReencryptionMaintenanceRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = RealmFinishReencryptionMaintenanceRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}
