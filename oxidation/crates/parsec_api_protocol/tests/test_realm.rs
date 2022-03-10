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
fn serde_realm_create_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    let data = hex!("81a6737461747573a26f6b");

    let expected = RealmCreateRepSchema { status: Status::Ok };

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
fn serde_realm_status_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   encryption_revision: 8
    //   in_maintenance: true
    //   maintenance_started_by: "alice@dev1"
    //   maintenance_started_on: ext(1, 946774800.0)
    //   maintenance_type: "GARBAGE_COLLECTION"
    //   status: "ok"
    let data = hex!(
        "86b3656e6372797074696f6e5f7265766973696f6e08ae696e5f6d61696e74656e616e6365"
        "c3b66d61696e74656e616e63655f737461727465645f6279aa616c6963654064657631b66d"
        "61696e74656e616e63655f737461727465645f6f6ed70141cc375188000000b06d61696e74"
        "656e616e63655f74797065b2474152424147455f434f4c4c454354494f4ea6737461747573"
        "a26f6b"
    );

    let expected = RealmStatusRepSchema {
        status: Status::Ok,
        in_maintenance: true,
        maintenance_type: Some(MaintenanceType::GarbageCollection),
        maintenance_started_on: Some("2000-1-2T01:00:00Z".parse().unwrap()),
        maintenance_started_by: Some("alice@dev1".parse().unwrap()),
        encryption_revision: 8,
    };

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
fn serde_realm_stats_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   blocks_size: 8
    //   status: "ok"
    //   vlobs_size: 8
    let data = hex!("83ab626c6f636b735f73697a6508a6737461747573a26f6baa766c6f62735f73697a6508");

    let expected = RealmStatsRepSchema {
        status: Status::Ok,
        blocks_size: 8,
        vlobs_size: 8,
    };

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
fn serde_realm_get_role_certificates_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   certificates: [hex!("666f6f626172")]
    //   status: "ok"
    let data = hex!("82ac63657274696669636174657391c406666f6f626172a6737461747573a26f6b");

    let expected = RealmGetRoleCertificatesRepSchema {
        status: Status::Ok,
        certificates: vec![b"foobar".to_vec()],
    };

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
fn serde_realm_update_roles_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    let data = hex!("81a6737461747573a26f6b");

    let expected = RealmUpdateRolesRepSchema { status: Status::Ok };

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
fn serde_realm_start_reencryption_maintenance_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    let data = hex!("81a6737461747573a26f6b");

    let expected = RealmStartReencryptionMaintenanceRepSchema { status: Status::Ok };

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
fn serde_realm_finish_reencryption_maintenance_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    let data = hex!("81a6737461747573a26f6b");

    let expected = RealmFinishReencryptionMaintenanceRepSchema { status: Status::Ok };

    let schema = RealmFinishReencryptionMaintenanceRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = RealmFinishReencryptionMaintenanceRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}
