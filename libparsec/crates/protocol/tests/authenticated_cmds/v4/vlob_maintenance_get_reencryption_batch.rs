// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::authenticated_cmds;

// Request

pub fn req() {
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

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::VlobMaintenanceGetReencryptionBatch(req2) = data else {
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
    //   batch: [
    //     {
    //       version: 8
    //       blob: hex!("666f6f626172")
    //       vlob_id: ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))
    //     }
    //   ]
    //   status: "ok"
    let raw = hex!(
        "82a562617463689183a776657273696f6e08a7766c6f625f6964d8022b5f314728134a1286"
        "3da1ce49c112f6a4626c6f62c406666f6f626172a6737461747573a26f6b"
    );

    let expected = authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::Ok {
        batch: vec![ReencryptionBatchEntry {
            vlob_id: VlobID::from_hex("2b5f314728134a12863da1ce49c112f6").unwrap(),
            version: 8,
            blob: b"foobar".as_ref().into(),
        }],
    };

    let data =
        authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 =
        authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_not_allowed() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "not_allowed"
    let raw = hex!("81a6737461747573ab6e6f745f616c6c6f776564");

    let expected = authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::NotAllowed;

    let data =
        authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 =
        authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_not_found() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "not_found"
    let raw = hex!("82a6726561736f6ea6666f6f626172a6737461747573a96e6f745f666f756e64");

    let expected = authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::NotFound {
        reason: Some("foobar".to_owned()),
    };

    let data =
        authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 =
        authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_not_in_maintenance() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "not_in_maintenance"
    let raw = hex!(
        "82a6726561736f6ea6666f6f626172a6737461747573b26e6f745f696e5f6d61696e74656e"
        "616e6365"
    );

    let expected =
        authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::NotInMaintenance {
            reason: Some("foobar".to_owned()),
        };

    let data =
        authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 =
        authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_bad_encryption_revision() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "bad_encryption_revision"
    let raw = hex!("81a6737461747573b76261645f656e6372797074696f6e5f7265766973696f6e");

    let expected =
        authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::BadEncryptionRevision;

    let data =
        authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 =
        authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::load(&raw2).unwrap();

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
        authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::MaintenanceError {
            reason: Some("foobar".to_owned()),
        };

    let data =
        authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 =
        authenticated_cmds::vlob_maintenance_get_reencryption_batch::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
