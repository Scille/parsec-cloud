// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use std::collections::HashMap;

use hex_literal::hex;
use rstest::rstest;

use parsec_api_protocol::*;

#[rstest]
fn serde_vlob_create_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   timestamp: ext(1, 946774800.0)
    //   blob: hex!("666f6f626172")
    //   cmd: "vlob_create"
    //   encryption_revision: 8
    //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
    //   vlob_id: ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))
    let data = hex!(
        "86a4626c6f62c406666f6f626172a3636d64ab766c6f625f637265617465b3656e63727970"
        "74696f6e5f7265766973696f6e08a87265616c6d5f6964d8021d3353157d7d4e95ad2fdea7"
        "b3bd19c5a974696d657374616d70d70141cc375188000000a7766c6f625f6964d8022b5f31"
        "4728134a12863da1ce49c112f6"
    );

    let expected = VlobCreateReqSchema {
        cmd: "vlob_create".to_owned(),
        realm_id: "1d3353157d7d4e95ad2fdea7b3bd19c5".parse().unwrap(),
        encryption_revision: 8,
        vlob_id: "2b5f314728134a12863da1ce49c112f6".parse().unwrap(),
        timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        blob: b"foobar".to_vec(),
    };

    let schema = VlobCreateReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = VlobCreateReqSchema::load(&data2).unwrap();

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
        VlobCreateRepSchema::Ok
    )
)]
#[case::already_exists(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   reason: "foobar"
        //   status: "already_exists"
        &hex!(
            "82a6726561736f6ea6666f6f626172a6737461747573ae616c72656164795f657869737473"
        )[..],
        VlobCreateRepSchema::AlreadyExists {
            reason: "foobar".to_owned()
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
        VlobCreateRepSchema::NotAllowed
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
        VlobCreateRepSchema::BadEncryptionRevision
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
        VlobCreateRepSchema::InMaintenance
    )
)]
fn serde_vlob_create_rep(#[case] data_expected: (&[u8], VlobCreateRepSchema)) {
    let (data, expected) = data_expected;

    let schema = VlobCreateRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = VlobCreateRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_vlob_read_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   timestamp: ext(1, 946774800.0)
    //   version: 8
    //   cmd: "vlob_read"
    //   encryption_revision: 8
    //   vlob_id: ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))
    let data = hex!(
        "85a3636d64a9766c6f625f72656164b3656e6372797074696f6e5f7265766973696f6e08a9"
        "74696d657374616d70d70141cc375188000000a776657273696f6e08a7766c6f625f6964d8"
        "022b5f314728134a12863da1ce49c112f6"
    );

    let expected = VlobReadReqSchema {
        cmd: "vlob_read".to_owned(),
        encryption_revision: 8,
        vlob_id: "2b5f314728134a12863da1ce49c112f6".parse().unwrap(),
        version: Some(8),
        timestamp: Some("2000-1-2T01:00:00Z".parse().unwrap()),
    };

    let schema = VlobReadReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = VlobReadReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
#[case::ok(
    (
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
        VlobReadRepSchema::Ok {
            version: 8,
            blob: b"foobar".to_vec(),
            author: "alice@dev1".parse().unwrap(),
            timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
            author_last_role_granted_on: Some("2000-1-2T01:00:00Z".parse().unwrap()),
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
        VlobReadRepSchema::NotFound {
            reason: "foobar".to_owned(),
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
        VlobReadRepSchema::NotAllowed
    )
)]
#[case::bad_version(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "bad_version"
        &hex!(
            "81a6737461747573ab6261645f76657273696f6e"
        )[..],
        VlobReadRepSchema::BadVersion
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
        VlobReadRepSchema::BadEncryptionRevision
    )
)]
#[case::in_maintenance(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "in_maintenance
        &hex!(
            "81a6737461747573ae696e5f6d61696e74656e616e6365"
        )[..],
        VlobReadRepSchema::InMaintenance
    )
)]
fn serde_vlob_read_rep(#[case] data_expected: (&[u8], VlobReadRepSchema)) {
    let (data, expected) = data_expected;

    let schema = VlobReadRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = VlobReadRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_vlob_update_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   timestamp: ext(1, 946774800.0)
    //   version: 8
    //   blob: hex!("666f6f626172")
    //   cmd: "vlob_update"
    //   encryption_revision: 8
    //   vlob_id: ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))
    let data = hex!(
        "86a4626c6f62c406666f6f626172a3636d64ab766c6f625f757064617465b3656e63727970"
        "74696f6e5f7265766973696f6e08a974696d657374616d70d70141cc375188000000a77665"
        "7273696f6e08a7766c6f625f6964d8022b5f314728134a12863da1ce49c112f6"
    );

    let expected = VlobUpdateReqSchema {
        cmd: "vlob_update".to_owned(),
        encryption_revision: 8,
        vlob_id: "2b5f314728134a12863da1ce49c112f6".parse().unwrap(),
        timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        version: 8,
        blob: b"foobar".to_vec(),
    };

    let schema = VlobUpdateReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = VlobUpdateReqSchema::load(&data2).unwrap();

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
        VlobUpdateRepSchema::Ok
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
        VlobUpdateRepSchema::NotFound {
            reason: "foobar".to_owned(),
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
        VlobUpdateRepSchema::NotAllowed
    )
)]
#[case::bad_version(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "bad_version"
        &hex!(
            "81a6737461747573ab6261645f76657273696f6e"
        )[..],
        VlobUpdateRepSchema::BadVersion
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
        VlobUpdateRepSchema::BadEncryptionRevision
    )
)]
#[case::in_maintenance(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "in_maintenance
        &hex!(
            "81a6737461747573ae696e5f6d61696e74656e616e6365"
        )[..],
        VlobUpdateRepSchema::InMaintenance
    )
)]
fn serde_vlob_update_rep(#[case] data_expected: (&[u8], VlobUpdateRepSchema)) {
    let (data, expected) = data_expected;

    let schema = VlobUpdateRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = VlobUpdateRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_vlob_poll_changes_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "vlob_poll_changes"
    //   last_checkpoint: 8
    //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
    let data = hex!(
        "83a3636d64b1766c6f625f706f6c6c5f6368616e676573af6c6173745f636865636b706f69"
        "6e7408a87265616c6d5f6964d8021d3353157d7d4e95ad2fdea7b3bd19c5"
    );

    let expected = VlobPollChangesReqSchema {
        cmd: "vlob_poll_changes".to_owned(),
        realm_id: "1d3353157d7d4e95ad2fdea7b3bd19c5".parse().unwrap(),
        last_checkpoint: 8,
    };

    let schema = VlobPollChangesReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = VlobPollChangesReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   changes: {ExtType(code=2, data=b'+_1G(\x13J\x12\x86=\xa1\xceI\xc1\x12\xf6'):8}
        //   current_checkpoint: 8
        //   status: "ok"
        &hex!(
            "83a76368616e67657381d8022b5f314728134a12863da1ce49c112f608b263757272656e74"
            "5f636865636b706f696e7408a6737461747573a26f6b"
        )[..],
        VlobPollChangesRepSchema::Ok {
            changes: HashMap::from([("2b5f314728134a12863da1ce49c112f6".parse().unwrap(), 8)]),
            current_checkpoint: 8,
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
        VlobPollChangesRepSchema::NotAllowed
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
        VlobPollChangesRepSchema::NotFound {
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
        VlobPollChangesRepSchema::InMaintenance
    )
)]
fn serde_vlob_poll_changes_rep(#[case] data_expected: (&[u8], VlobPollChangesRepSchema)) {
    let (data, expected) = data_expected;

    let schema = VlobPollChangesRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = VlobPollChangesRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_vlob_list_versions_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "vlob_list_versions"
    //   vlob_id: ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))
    let data = hex!(
        "82a3636d64b2766c6f625f6c6973745f76657273696f6e73a7766c6f625f6964d8022b5f31"
        "4728134a12863da1ce49c112f6"
    );

    let expected = VlobListVersionsReqSchema {
        cmd: "vlob_list_versions".to_owned(),
        vlob_id: "2b5f314728134a12863da1ce49c112f6".parse().unwrap(),
    };

    let schema = VlobListVersionsReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = VlobListVersionsReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   status: "ok"
        //   versions: {8:[ext(1, 946774800.0), "alice@dev1"]}
        &hex!(
            "82a6737461747573a26f6ba876657273696f6e73810892d70141cc375188000000aa616c69"
            "63654064657631"
        )[..],
        VlobListVersionsRepSchema::Ok {
            versions: HashMap::from([(
                8,
                (
                    "2000-1-2T01:00:00Z".parse().unwrap(),
                    "alice@dev1".parse().unwrap(),
                ),
            )]),
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
        VlobListVersionsRepSchema::NotAllowed
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
        VlobListVersionsRepSchema::NotFound {
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
        VlobListVersionsRepSchema::InMaintenance
    )
)]
fn serde_vlob_list_versions_rep(#[case] data_expected: (&[u8], VlobListVersionsRepSchema)) {
    let (data, expected) = data_expected;

    let schema = VlobListVersionsRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = VlobListVersionsRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_vlob_maintenance_get_reencryption_batch_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "vlob_maintenance_get_reencryption_batch"
    //   encryption_revision: 8
    //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
    //   size: 8
    let data = hex!(
        "84a3636d64d927766c6f625f6d61696e74656e616e63655f6765745f7265656e6372797074"
        "696f6e5f6261746368b3656e6372797074696f6e5f7265766973696f6e08a87265616c6d5f"
        "6964d8021d3353157d7d4e95ad2fdea7b3bd19c5a473697a6508"
    );

    let expected = VlobMaintenanceGetReencryptionBatchReqSchema {
        cmd: "vlob_maintenance_get_reencryption_batch".to_owned(),
        realm_id: "1d3353157d7d4e95ad2fdea7b3bd19c5".parse().unwrap(),
        encryption_revision: 8,
        size: 8,
    };

    let schema = VlobMaintenanceGetReencryptionBatchReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = VlobMaintenanceGetReencryptionBatchReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   batch: [
        //     {
        //       version: 8
        //       blob: hex!("666f6f626172")
        //       vlob_id: ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))
        //     }
        //   ]
        //   status: "ok"
        &hex!(
            "82a562617463689183a776657273696f6e08a7766c6f625f6964d8022b5f314728134a1286"
            "3da1ce49c112f6a4626c6f62c406666f6f626172a6737461747573a26f6b"
        )[..],
        VlobMaintenanceGetReencryptionBatchRepSchema::Ok {
            batch: vec![ReencryptionBatchEntrySchema {
                vlob_id: "2b5f314728134a12863da1ce49c112f6".parse().unwrap(),
                version: 8,
                blob: b"foobar".to_vec(),
            }],
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
        VlobMaintenanceGetReencryptionBatchRepSchema::NotAllowed
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
        VlobMaintenanceGetReencryptionBatchRepSchema::NotFound {
            reason: "foobar".to_owned()
        }
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
        VlobMaintenanceGetReencryptionBatchRepSchema::NotInMaintenance {
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
        VlobMaintenanceGetReencryptionBatchRepSchema::BadEncryptionRevision
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
        VlobMaintenanceGetReencryptionBatchRepSchema::MaintenanceError {
            reason: "foobar".to_owned()
        }
    )
)]
fn serde_vlob_maintenance_get_reencryption_batch_rep(
    #[case] data_expected: (&[u8], VlobMaintenanceGetReencryptionBatchRepSchema),
) {
    let (data, expected) = data_expected;

    let schema = VlobMaintenanceGetReencryptionBatchRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = VlobMaintenanceGetReencryptionBatchRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_vlob_maintenance_save_reencryption_batch_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   batch: [
    //     {
    //       version: 8
    //       blob: hex!("666f6f626172")
    //       vlob_id: ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))
    //     }
    //   ]
    //   cmd: "vlob_maintenance_save_reencryption_batch"
    //   encryption_revision: 8
    //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
    let data = hex!(
        "84a562617463689183a776657273696f6e08a7766c6f625f6964d8022b5f314728134a1286"
        "3da1ce49c112f6a4626c6f62c406666f6f626172a3636d64d928766c6f625f6d61696e7465"
        "6e616e63655f736176655f7265656e6372797074696f6e5f6261746368b3656e6372797074"
        "696f6e5f7265766973696f6e08a87265616c6d5f6964d8021d3353157d7d4e95ad2fdea7b3"
        "bd19c5"
    );

    let expected = VlobMaintenanceSaveReencryptionBatchReqSchema {
        cmd: "vlob_maintenance_save_reencryption_batch".to_owned(),
        realm_id: "1d3353157d7d4e95ad2fdea7b3bd19c5".parse().unwrap(),
        encryption_revision: 8,
        batch: vec![ReencryptionBatchEntrySchema {
            vlob_id: "2b5f314728134a12863da1ce49c112f6".parse().unwrap(),
            version: 8,
            blob: b"foobar".to_vec(),
        }],
    };

    let schema = VlobMaintenanceSaveReencryptionBatchReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = VlobMaintenanceSaveReencryptionBatchReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
#[case::ok(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   done: 8
        //   status: "ok"
        //   total: 8
        &hex!(
            "83a4646f6e6508a6737461747573a26f6ba5746f74616c08"
        )[..],
        VlobMaintenanceSaveReencryptionBatchRepSchema::Ok { total: 8, done: 8 }
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
        VlobMaintenanceSaveReencryptionBatchRepSchema::NotAllowed
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
        VlobMaintenanceSaveReencryptionBatchRepSchema::NotFound {
            reason: "foobar".to_owned()
        }
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
        VlobMaintenanceSaveReencryptionBatchRepSchema::NotInMaintenance {
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
        VlobMaintenanceSaveReencryptionBatchRepSchema::BadEncryptionRevision
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
        VlobMaintenanceSaveReencryptionBatchRepSchema::MaintenanceError {
            reason: "foobar".to_owned()
        }
    )
)]
fn serde_vlob_maintenance_save_reencryption_batch_rep(
    #[case] data_expected: (&[u8], VlobMaintenanceSaveReencryptionBatchRepSchema),
) {
    let (data, expected) = data_expected;

    let schema = VlobMaintenanceSaveReencryptionBatchRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = VlobMaintenanceSaveReencryptionBatchRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}
