// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::num::NonZeroU64;

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::authenticated_cmds;

// Request

pub fn req() {
    let raws = [
        // Generated from Python implementation (Parsec v2.11.1+dev)
        // Content:
        //   cmd: "human_find"
        //   omit_non_human: false
        //   omit_revoked: false
        //   page: 0
        //   per_page: 8
        //   query: "foobar"
        &hex!(
            "86a3636d64aa68756d616e5f66696e64ae6f6d69745f6e6f6e5f68756d616ec2ac6f6d6974"
            "5f7265766f6b6564c2a47061676500a87065725f7061676508a57175657279a6666f6f6261"
            "72"
        )[..],
        // Generated from Python implementation (Parsec v2.12.1+dev)
        // Content:
        //   cmd: "human_find"
        //   omit_non_human: false
        //   omit_revoked: false
        //   page: -1
        //   per_page: 8
        //   query: "foobar"
        &hex!(
            "86a3636d64aa68756d616e5f66696e64ae6f6d69745f6e6f6e5f68756d616ec2ac6f6d6974"
            "5f7265766f6b6564c2a470616765ffa87065725f7061676508a57175657279a6666f6f6261"
            "72"
        )[..],
        // Generated from Python implementation (Parsec v2.11.1+dev)
        // Content:
        //   cmd: "human_find"
        //   omit_non_human: false
        //   omit_revoked: false
        //   page: 8
        //   per_page: 0
        //   query: "foobar"
        &hex!(
            "86a3636d64aa68756d616e5f66696e64ae6f6d69745f6e6f6e5f68756d616ec2ac6f6d6974"
            "5f7265766f6b6564c2a47061676508a87065725f7061676500a57175657279a6666f6f6261"
            "72"
        )[..],
        // Generated from Python implementation (Parsec v2.11.1+dev)
        // Content:
        //   cmd: "human_find"
        //   omit_non_human: false
        //   omit_revoked: false
        //   page: 0
        //   per_page: 101
        //   query: "foobar"
        &hex!(
            "86a3636d64aa68756d616e5f66696e64ae6f6d69745f6e6f6e5f68756d616ec2ac6f6d6974"
            "5f7265766f6b6564c2a47061676500a87065725f7061676565a57175657279a6666f6f6261"
            "72"
        )[..],
    ];

    for raw in raws {
        let err = authenticated_cmds::AnyCmdReq::load(raw).unwrap_err();
        assert!(matches!(err, rmp_serde::decode::Error::Syntax(_)));
    }

    let raw_expected = [
        (
            // Generated from Python implementation (Parsec v2.6.0+dev)
            // Content:
            //   cmd: "human_find"
            //   omit_non_human: false
            //   omit_revoked: false
            //   page: 8
            //   per_page: 8
            //   query: "foobar"
            &hex!(
                "86a3636d64aa68756d616e5f66696e64ae6f6d69745f6e6f6e5f68756d616ec2ac6f6d6974"
                "5f7265766f6b6564c2a47061676508a87065725f7061676508a57175657279a6666f6f6261"
                "72"
            )[..],
            authenticated_cmds::AnyCmdReq::HumanFind(authenticated_cmds::human_find::Req {
                query: Some("foobar".to_owned()),
                omit_revoked: false,
                omit_non_human: false,
                page: NonZeroU64::new(8).unwrap(),
                per_page: IntegerBetween1And100::try_from(8).unwrap(),
            }),
        ),
        (
            // Generated from Python implementation (Parsec v2.12.1+dev)
            // Content:
            //   cmd: "human_find"
            //   omit_non_human: false
            //   omit_revoked: false
            //   page: 8
            //   per_page: 8
            //   query: None
            //
            &hex!(
                "86a3636d64aa68756d616e5f66696e64ae6f6d69745f6e6f6e5f68756d616ec2ac6f6d6974"
                "5f7265766f6b6564c2a47061676508a87065725f7061676508a57175657279c0"
            )[..],
            authenticated_cmds::AnyCmdReq::HumanFind(authenticated_cmds::human_find::Req {
                query: None,
                omit_revoked: false,
                omit_non_human: false,
                page: NonZeroU64::new(8).unwrap(),
                per_page: IntegerBetween1And100::try_from(8).unwrap(),
            }),
        ),
    ];

    for (raw, expected) in raw_expected {
        let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();

        p_assert_eq!(data, expected);

        // Also test serialization round trip
        let authenticated_cmds::AnyCmdReq::HumanFind(req2) = data else {
            unreachable!()
        };

        let raw2 = req2.dump().unwrap();

        let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

        p_assert_eq!(data2, expected);
    }
}

// Responses

pub fn rep_ok() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   page: 8
    //   per_page: 8
    //   results: [
    //     {
    //       human_handle: ["bob@dev1", "bob"]
    //       revoked: false
    //       user_id: "109b68ba5cdf428ea0017fc6bcc04d4a"
    //     }
    //   ]
    //   status: "ok"
    //   total: 8
    let raw = hex!(
        "85a47061676508a87065725f7061676508a7726573756c74739183ac68756d616e5f68616e"
        "646c6592a8626f624064657631a3626f62a7757365725f6964d92031303962363862613563"
        "64663432386561303031376663366263633034643461a77265766f6b6564c2a67374617475"
        "73a26f6ba5746f74616c08"
    );

    let expected = authenticated_cmds::human_find::Rep::Ok {
        results: vec![authenticated_cmds::human_find::HumanFindResultItem {
            user_id: "109b68ba5cdf428ea0017fc6bcc04d4a".parse().unwrap(),
            human_handle: Some(HumanHandle::new("bob@dev1", "bob").unwrap()),
            revoked: false,
        }],
        page: NonZeroU64::new(8).unwrap(),
        per_page: IntegerBetween1And100::try_from(8).unwrap(),
        total: 8,
    };

    let data = authenticated_cmds::human_find::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::human_find::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn rep_not_allowed() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   reason: "foobar"
    //   status: "not_allowed"
    let raw = hex!("82a6726561736f6ea6666f6f626172a6737461747573ab6e6f745f616c6c6f776564");

    let expected = authenticated_cmds::human_find::Rep::NotAllowed {
        reason: Some("foobar".to_owned()),
    };

    let data = authenticated_cmds::human_find::Rep::load(&raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::human_find::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}
