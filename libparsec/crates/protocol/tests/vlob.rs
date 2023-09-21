// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]
use std::collections::HashMap;

use libparsec_protocol::authenticated_cmds::v3 as authenticated_cmds;
use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

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

#[parsec_test]
fn serde_vlob_poll_changes_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "vlob_poll_changes"
    //   last_checkpoint: 8
    //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
    let raw = hex!(
        "83a3636d64b1766c6f625f706f6c6c5f6368616e676573af6c6173745f636865636b706f69"
        "6e7408a87265616c6d5f6964d8021d3353157d7d4e95ad2fdea7b3bd19c5"
    );

    let req = authenticated_cmds::vlob_poll_changes::Req {
        realm_id: VlobID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
        last_checkpoint: 8,
    };

    let expected = authenticated_cmds::AnyCmdReq::VlobPollChanges(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = if let authenticated_cmds::AnyCmdReq::VlobPollChanges(data) = data {
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
    //   changes: {ExtType(code=2, raw=b'+_1G(\x13J\x12\x86=\xa1\xceI\xc1\x12\xf6'):8}
    //   current_checkpoint: 8
    //   status: "ok"
    &hex!(
        "83a76368616e67657381d8022b5f314728134a12863da1ce49c112f608b263757272656e74"
        "5f636865636b706f696e7408a6737461747573a26f6b"
    )[..],
    authenticated_cmds::vlob_poll_changes::Rep::Ok {
        changes: HashMap::from([(VlobID::from_hex("2b5f314728134a12863da1ce49c112f6").unwrap(), 8)]),
        current_checkpoint: 8,
    }
)]
#[case::not_allowed(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_allowed"
    &hex!(
        "81a6737461747573ab6e6f745f616c6c6f776564"
    )[..],
    authenticated_cmds::vlob_poll_changes::Rep::NotAllowed
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "not_found"
    &hex!(
        "82a6726561736f6ea6666f6f626172a6737461747573a96e6f745f666f756e64"
    )[..],
    authenticated_cmds::vlob_poll_changes::Rep::NotFound {
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
    authenticated_cmds::vlob_poll_changes::Rep::InMaintenance
)]
fn serde_vlob_poll_changes_rep(
    #[case] raw: &[u8],
    #[case] expected: authenticated_cmds::vlob_poll_changes::Rep,
) {
    let data = authenticated_cmds::vlob_poll_changes::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_poll_changes::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_vlob_list_versions_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "vlob_list_versions"
    //   vlob_id: ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))
    let raw = hex!(
        "82a3636d64b2766c6f625f6c6973745f76657273696f6e73a7766c6f625f6964d8022b5f31"
        "4728134a12863da1ce49c112f6"
    );

    let req = authenticated_cmds::vlob_list_versions::Req {
        vlob_id: VlobID::from_hex("2b5f314728134a12863da1ce49c112f6").unwrap(),
    };

    let expected = authenticated_cmds::AnyCmdReq::VlobListVersions(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = if let authenticated_cmds::AnyCmdReq::VlobListVersions(data) = data {
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
    //   versions: {8:[ext(1, 946774800.0), "alice@dev1"]}
    &hex!(
        "82a6737461747573a26f6ba876657273696f6e73810892d70141cc375188000000aa616c69"
        "63654064657631"
    )[..],
    authenticated_cmds::vlob_list_versions::Rep::Ok {
        versions: HashMap::from([(
            8,
            (
                "2000-1-2T01:00:00Z".parse().unwrap(),
                "alice@dev1".parse().unwrap(),
            ),
        )]),
    }
)]
#[case::not_allowed(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_allowed"
    &hex!(
        "81a6737461747573ab6e6f745f616c6c6f776564"
    )[..],
    authenticated_cmds::vlob_list_versions::Rep::NotAllowed
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "not_found"
    &hex!(
        "82a6726561736f6ea6666f6f626172a6737461747573a96e6f745f666f756e64"
    )[..],
    authenticated_cmds::vlob_list_versions::Rep::NotFound {
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
    authenticated_cmds::vlob_list_versions::Rep::InMaintenance
)]
fn serde_vlob_list_versions_rep(
    #[case] raw: &[u8],
    #[case] expected: authenticated_cmds::vlob_list_versions::Rep,
) {
    let data = authenticated_cmds::vlob_list_versions::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::vlob_list_versions::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
fn serde_vlob_maintenance_get_reencryption_batch_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "vlob_maintenance_get_reencryption_batch"
    //   encryption_revision: 8
    //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
    //   size: 8
    let raw = hex!(
        "84a3636d64d927766c6f625f6d61696e74656e616e63655f6765745f7265656e6372797074"
        "696f6e5f6261746368b3656e6372797074696f6e5f7265766973696f6e08a87265616c6d5f"
        "6964d8021d3353157d7d4e95ad2fdea7b3bd19c5a473697a6508"
    );

    let req = authenticated_cmds::vlob_maintenance_get_reencryption_batch::Req {
        realm_id: VlobID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
        encryption_revision: 8,
        size: 8,
    };

    let expected = authenticated_cmds::AnyCmdReq::VlobMaintenanceGetReencryptionBatch(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 =
        if let authenticated_cmds::AnyCmdReq::VlobMaintenanceGetReencryptionBatch(data) = data {
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
    authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::Ok {
        batch: vec![ReencryptionBatchEntry {
            vlob_id: VlobID::from_hex("2b5f314728134a12863da1ce49c112f6").unwrap(),
            version: 8,
            blob: b"foobar".as_ref().into(),
        }],
    }
)]
#[case::not_allowed(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_allowed"
    &hex!(
        "81a6737461747573ab6e6f745f616c6c6f776564"
    )[..],
    authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::NotAllowed
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "not_found"
    &hex!(
        "82a6726561736f6ea6666f6f626172a6737461747573a96e6f745f666f756e64"
    )[..],
    authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::NotFound {
        reason: Some("foobar".to_owned())
    }
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
    authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::NotInMaintenance {
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
    authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::BadEncryptionRevision
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
    authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::MaintenanceError {
        reason: Some("foobar".to_owned())
    }
)]
fn serde_vlob_maintenance_get_reencryption_batch_rep(
    #[case] raw: &[u8],
    #[case] expected: authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep,
) {
    let data = authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 =
        authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

#[parsec_test]
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
    let raw = hex!(
        "84a562617463689183a776657273696f6e08a7766c6f625f6964d8022b5f314728134a1286"
        "3da1ce49c112f6a4626c6f62c406666f6f626172a3636d64d928766c6f625f6d61696e7465"
        "6e616e63655f736176655f7265656e6372797074696f6e5f6261746368b3656e6372797074"
        "696f6e5f7265766973696f6e08a87265616c6d5f6964d8021d3353157d7d4e95ad2fdea7b3"
        "bd19c5"
    );

    let req = authenticated_cmds::vlob_maintenance_save_reencryption_batch::Req {
        realm_id: VlobID::from_hex("1d3353157d7d4e95ad2fdea7b3bd19c5").unwrap(),
        encryption_revision: 8,
        batch: vec![ReencryptionBatchEntry {
            vlob_id: VlobID::from_hex("2b5f314728134a12863da1ce49c112f6").unwrap(),
            version: 8,
            blob: b"foobar".as_ref().into(),
        }],
    };

    let expected = authenticated_cmds::AnyCmdReq::VlobMaintenanceSaveReencryptionBatch(req);

    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 =
        if let authenticated_cmds::AnyCmdReq::VlobMaintenanceSaveReencryptionBatch(data) = data {
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
    //   done: 8
    //   status: "ok"
    //   total: 8
    &hex!(
        "83a4646f6e6508a6737461747573a26f6ba5746f74616c08"
    )[..],
    authenticated_cmds::vlob_maintenance_save_reencryption_batch::Rep::Ok { total: 8, done: 8 }
)]
#[case::not_allowed(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_allowed"
    &hex!(
        "81a6737461747573ab6e6f745f616c6c6f776564"
    )[..],
    authenticated_cmds::vlob_maintenance_save_reencryption_batch::Rep::NotAllowed
)]
#[case::not_found(
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "not_found"
    &hex!(
        "82a6726561736f6ea6666f6f626172a6737461747573a96e6f745f666f756e64"
    )[..],
    authenticated_cmds::vlob_maintenance_save_reencryption_batch::Rep::NotFound {
        reason: Some("foobar".to_owned())
    }
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
    authenticated_cmds::vlob_maintenance_save_reencryption_batch::Rep::NotInMaintenance {
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
    authenticated_cmds::vlob_maintenance_save_reencryption_batch::Rep::BadEncryptionRevision
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
    authenticated_cmds::vlob_maintenance_save_reencryption_batch::Rep::MaintenanceError {
        reason: Some("foobar".to_owned())
    }
)]
fn serde_vlob_maintenance_save_reencryption_batch_rep(
    #[case] raw: &[u8],
    #[case] expected: authenticated_cmds::vlob_maintenance_save_reencryption_batch::Rep,
) {
    let data =
        authenticated_cmds::vlob_maintenance_save_reencryption_batch::Rep::load(raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 =
        authenticated_cmds::vlob_maintenance_save_reencryption_batch::Rep::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}
