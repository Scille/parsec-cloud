// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

use super::invited_cmds;
use libparsec_tests_lite::{hex, p_assert_eq};
use libparsec_types::AccessToken;

pub fn rep_bad_invitation_type() {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   status: 'bad_invitation_type'
    let raw: &[u8] = hex!("81a6737461747573b36261645f696e7669746174696f6e5f74797065").as_ref();

    let expected = invited_cmds::invite_shamir_recovery_reveal::Rep::BadInvitationType;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = invited_cmds::invite_shamir_recovery_reveal::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
}

pub fn rep_bad_reveal_token() {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   status: 'bad_reveal_token'
    let raw: &[u8] = hex!("81a6737461747573b06261645f72657665616c5f746f6b656e").as_ref();

    let expected = invited_cmds::invite_shamir_recovery_reveal::Rep::BadRevealToken;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = invited_cmds::invite_shamir_recovery_reveal::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
}

pub fn rep_ok() {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   status: 'ok'
    //   ciphered_data: 0x6369706865726564
    let raw: &[u8] =
        hex!("82a6737461747573a26f6bad63697068657265645f64617461c4086369706865726564").as_ref();

    let expected = invited_cmds::invite_shamir_recovery_reveal::InviteShamirRecoveryRevealRep::Ok {
        ciphered_data: "ciphered".into(),
    };
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = invited_cmds::invite_shamir_recovery_reveal::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
}
pub fn req() {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   cmd: 'invite_shamir_recovery_reveal'
    //   reveal_token: 0xd864b93ded264aae9ae583fd3d40c45a
    let raw: &[u8] = hex!(
    "82a3636d64bd696e766974655f7368616d69725f7265636f766572795f72657665616c"
    "ac72657665616c5f746f6b656ec410d864b93ded264aae9ae583fd3d40c45a"
    )
    .as_ref();

    let req = invited_cmds::invite_shamir_recovery_reveal::InviteShamirRecoveryRevealReq {
        reveal_token: AccessToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
    };
    println!("***expected: {:?}", req.dump().unwrap());

    let expected = invited_cmds::AnyCmdReq::InviteShamirRecoveryReveal(req);

    let data = invited_cmds::AnyCmdReq::load(raw).unwrap();
    p_assert_eq!(data, expected);
}
