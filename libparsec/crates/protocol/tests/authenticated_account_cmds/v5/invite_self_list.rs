// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::prelude::*;

use super::authenticated_account_cmds;

// Request

pub fn req() {
    // Generated from Parsec 3.4.1-a.0+dev
    // Content:
    //   cmd: 'invite_self_list'
    let raw: &[u8] = hex!("81a3636d64b0696e766974655f73656c665f6c697374").as_ref();

    let req = authenticated_account_cmds::invite_self_list::Req {};
    println!("***expected: {:?}", req.dump().unwrap());

    let expected = authenticated_account_cmds::AnyCmdReq::InviteSelfList(req.clone());
    let data = authenticated_account_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = req.dump().unwrap();

    let data2 = authenticated_account_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

// Responses

pub fn rep_ok() {
    let raw_expected = [
        (
            // Generated from Parsec 3.4.1-a.0+dev
            // Content:
            //   status: 'ok'
            //   invitations: [ ]
            hex!("82a6737461747573a26f6bab696e7669746174696f6e7390").as_ref(),
            authenticated_account_cmds::invite_self_list::Rep::Ok {
                invitations: vec![],
            },
        ),
        (
            // Generated from Parsec 3.4.1-a.0+dev
            // Content:
            //   status: 'ok'
            //   invitations: [
            //     [ 'Org1', 0xd864b93ded264aae9ae583fd3d40c45a, 'USER' ],
            //     [ 'Org2', 0xe4eaf751ba3546a899a9088002e51918, 'DEVICE' ],
            //     [ 'Org3', 0x5846639483a54f1494b22ce13a64c67a, 'SHAMIR_RECOVERY' ]
            //   ]
            hex!(
                "82a6737461747573a26f6bab696e7669746174696f6e739393a44f726731c410d864b9"
                "3ded264aae9ae583fd3d40c45aa45553455293a44f726732c410e4eaf751ba3546a899"
                "a9088002e51918a644455649434593a44f726733c4105846639483a54f1494b22ce13a"
                "64c67aaf5348414d49525f5245434f56455259"
            )
            .as_ref(),
            authenticated_account_cmds::invite_self_list::Rep::Ok {
                invitations: vec![
                    (
                        "Org1".parse().unwrap(),
                        AccessToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
                        InvitationType::User,
                    ),
                    (
                        "Org2".parse().unwrap(),
                        AccessToken::from_hex("e4eaf751ba3546a899a9088002e51918").unwrap(),
                        InvitationType::Device,
                    ),
                    (
                        "Org3".parse().unwrap(),
                        AccessToken::from_hex("5846639483a54f1494b22ce13a64c67a").unwrap(),
                        InvitationType::ShamirRecovery,
                    ),
                ],
            },
        ),
    ];

    for (raw, expected) in raw_expected {
        println!("***expected: {:?}", expected.dump().unwrap());
        let data = authenticated_account_cmds::invite_self_list::Rep::load(raw).unwrap();

        p_assert_eq!(data, expected);

        // Also test serialization round trip
        let raw2 = data.dump().unwrap();

        let data2 = authenticated_account_cmds::invite_self_list::Rep::load(&raw2).unwrap();

        p_assert_eq!(data2, expected);
    }
}
