// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::invited_cmds;

// Request

pub fn req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "invite_info"
    let raw = hex!("81a3636d64ab696e766974655f696e666f");

    let req = invited_cmds::invite_info::Req;

    let expected = invited_cmds::AnyCmdReq::InviteInfo(req);

    let data = invited_cmds::AnyCmdReq::load(&raw).unwrap();

    assert_eq!(data, expected);

    // Also test serialization round trip
    let invited_cmds::AnyCmdReq::InviteInfo(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = invited_cmds::AnyCmdReq::load(&raw2).unwrap();

    assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    let raw_expected = [
        (
            // Generated from Python implementation (Parsec v2.6.0+dev)
            // Content:
            //   type: "USER"
            //   claimer_email: "alice@dev1"
            //   greeter_human_handle: ["bob@dev1", "bob"]
            //   greeter_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a"
            //   status: "ok"
            &hex!(
                "85ad636c61696d65725f656d61696caa616c6963654064657631b4677265657465725f6875"
                "6d616e5f68616e646c6592a8626f624064657631a3626f62af677265657465725f75736572"
                "5f6964d9203130396236386261356364663432386561303031376663366263633034643461"
                "a6737461747573a26f6ba474797065a455534552"
            )[..],
            invited_cmds::invite_info::Rep::Ok(invited_cmds::invite_info::UserOrDevice::User {
                claimer_email: "alice@dev1".to_owned(),
                greeter_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a".parse().unwrap(),
                greeter_human_handle: Some(HumanHandle::new("bob@dev1", "bob").unwrap()),
            }),
        ),
        (
            // Generated from Python implementation (Parsec v2.6.0+dev)
            // Content:
            //   type: "DEVICE"
            //   greeter_human_handle: ["bob@dev1", "bob"]
            //   greeter_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a"
            //   status: "ok"
            &hex!(
                "84b4677265657465725f68756d616e5f68616e646c6592a8626f624064657631a3626f62af"
                "677265657465725f757365725f6964d9203130396236386261356364663432386561303031"
                "376663366263633034643461a6737461747573a26f6ba474797065a6444556494345"
            )[..],
            invited_cmds::invite_info::Rep::Ok(invited_cmds::invite_info::UserOrDevice::Device {
                greeter_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a".parse().unwrap(),
                greeter_human_handle: Some(HumanHandle::new("bob@dev1", "bob").unwrap()),
            }),
        ),
        (
            // Generated from Rust implementation (Parsec v2.13.0-rc1+dev)
            // Content:
            //   type: "USER"
            //   claimer_email: "alice@dev1"
            //   greeter_human_handle: None
            //   greeter_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a"
            //   status: "ok"
            //
            &hex!(
                "85a6737461747573a26f6ba474797065a455534552ad636c61696d65725f656d61696caa61"
                "6c6963654064657631af677265657465725f757365725f6964d92031303962363862613563"
                "64663432386561303031376663366263633034643461b4677265657465725f68756d616e5f"
                "68616e646c65c0"
            ),
            invited_cmds::invite_info::Rep::Ok(invited_cmds::invite_info::UserOrDevice::User {
                greeter_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a".parse().unwrap(),
                claimer_email: "alice@dev1".to_string(),
                greeter_human_handle: None,
            }),
        ),
        (
            // Generated from Rust implementation (Parsec v2.13.0-rc1+dev)
            // Content:
            //   type: "DEVICE"
            //   greeter_human_handle: None
            //   greeter_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a"
            //   status: "ok"
            //
            &hex!(
                "84a6737461747573a26f6ba474797065a6444556494345af677265657465725f757365725f"
                "6964d9203130396236386261356364663432386561303031376663366263633034643461b4"
                "677265657465725f68756d616e5f68616e646c65c0"
            ),
            invited_cmds::invite_info::Rep::Ok(invited_cmds::invite_info::UserOrDevice::Device {
                greeter_user_id: "109b68ba5cdf428ea0017fc6bcc04d4a".parse().unwrap(),
                greeter_human_handle: None,
            }),
        ),
    ];

    for (raw, expected) in raw_expected {
        let data = invited_cmds::invite_info::Rep::load(raw).unwrap();

        assert_eq!(data, expected);

        // Also test serialization round trip
        let raw2 = data.dump().unwrap();

        let data2 = invited_cmds::invite_info::Rep::load(&raw2).unwrap();

        assert_eq!(data2, expected);
    }
}
