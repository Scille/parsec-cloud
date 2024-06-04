// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

use bytes::Bytes;
use libparsec_protocol::authenticated_cmds::v4::shamir_recovery_self_info;
use libparsec_tests_lite::prelude::*;
use libparsec_types::ShamirRevealToken;

use super::authenticated_cmds;

pub fn rep_ok() {
    // Generated from Rust implementation (Parsec v3.0.0-b.6+dev 2024-05-29)
    // Content:
    //   "status": ok
    //   "self_info": "None"
    let raw = hex!("82a6737461747573a26f6ba973656c665f696e666fc0");
    let expected = authenticated_cmds::shamir_recovery_self_info::Rep::Ok { self_info: None };
    let data = authenticated_cmds::shamir_recovery_self_info::Rep::load(&raw).unwrap();
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();
    let data2 = authenticated_cmds::shamir_recovery_self_info::Rep::load(&raw2).unwrap();
    p_assert_eq!(data2, expected);

    // Generated from Rust implementation (Parsec v3.0.0-b.6+dev 2024-05-29)
    // Content:
    //   "self_info": b"shamirsetup"
    let raw = hex!("82a6737461747573a26f6ba973656c665f696e666fc40b7368616d69727365747570");
    let expected = authenticated_cmds::shamir_recovery_self_info::Rep::Ok {
        self_info: Some(Bytes::from_static(b"shamirsetup")),
    };
    let data = authenticated_cmds::shamir_recovery_self_info::Rep::load(&raw).unwrap();
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();
    let data2 = authenticated_cmds::shamir_recovery_self_info::Rep::load(&raw2).unwrap();
    p_assert_eq!(data2, expected);
}

pub fn req() {
    let empty_req = authenticated_cmds::shamir_recovery_self_info::Req;
    // Generated from Rust implementation (Parsec v3.0.0-b.6+dev 2024-05-29)
    // Content:
    //   cmd: "shamir_recovery_self_info",
    let raw = hex!("81a3636d64b97368616d69725f7265636f766572795f73656c665f696e666f");
    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();
    let expected = authenticated_cmds::AnyCmdReq::ShamirRecoverySelfInfo(empty_req.clone());
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = empty_req.dump().unwrap();
    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();
    p_assert_eq!(data2, expected);
}
