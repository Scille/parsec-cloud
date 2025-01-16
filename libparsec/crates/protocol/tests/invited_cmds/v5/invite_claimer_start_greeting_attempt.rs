// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

use super::invited_cmds;
use libparsec_tests_lite::{hex, p_assert_eq};
use libparsec_types::GreetingAttemptID;
use libparsec_types::UserID;

pub fn rep_greeter_not_allowed() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'greeter_not_allowed'
    let raw: &[u8] = hex!("81a6737461747573b3677265657465725f6e6f745f616c6c6f776564").as_ref();

    let expected = invited_cmds::invite_claimer_start_greeting_attempt::InviteClaimerStartGreetingAttemptRep::GreeterNotAllowed;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = invited_cmds::invite_claimer_start_greeting_attempt::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
}

pub fn rep_greeter_not_found() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'greeter_not_found'
    let raw: &[u8] = hex!("81a6737461747573b1677265657465725f6e6f745f666f756e64").as_ref();
    let expected = invited_cmds::invite_claimer_start_greeting_attempt::InviteClaimerStartGreetingAttemptRep::GreeterNotFound;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = invited_cmds::invite_claimer_start_greeting_attempt::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
}

pub fn rep_greeter_revoked() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'greeter_revoked'
    let raw: &[u8] = hex!("81a6737461747573af677265657465725f7265766f6b6564").as_ref();

    let expected = invited_cmds::invite_claimer_start_greeting_attempt::InviteClaimerStartGreetingAttemptRep::GreeterRevoked;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = invited_cmds::invite_claimer_start_greeting_attempt::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
}

pub fn rep_ok() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'ok'
    //   greeting_attempt: ext(2, 0xd864b93ded264aae9ae583fd3d40c45a)
    let raw: &[u8] = hex!(
    "82a6737461747573a26f6bb06772656574696e675f617474656d7074d802d864b93ded"
    "264aae9ae583fd3d40c45a"
    )
    .as_ref();

    let expected = invited_cmds::invite_claimer_start_greeting_attempt::InviteClaimerStartGreetingAttemptRep::Ok{greeting_attempt:GreetingAttemptID::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap()};
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = invited_cmds::invite_claimer_start_greeting_attempt::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
}
pub fn req() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   cmd: 'invite_claimer_start_greeting_attempt'
    //   token: 0xd864b93ded264aae9ae583fd3d40c45a
    //   greeter: ext(2, 0x109b68ba5cdf428ea0017fc6bcc04d4a)
    let raw: &[u8] = hex!(
    "83a3636d64d925696e766974655f636c61696d65725f73746172745f6772656574696e"
    "675f617474656d7074a5746f6b656ec410d864b93ded264aae9ae583fd3d40c45aa767"
    "726565746572d802109b68ba5cdf428ea0017fc6bcc04d4a"
    )
    .as_ref();
    let req =
        invited_cmds::invite_claimer_start_greeting_attempt::InviteClaimerStartGreetingAttemptReq {
            greeter: UserID::from_hex("109b68ba5cdf428ea0017fc6bcc04d4a").unwrap(),
        };
    println!("***expected: {:?}", req.dump().unwrap());

    let expected = invited_cmds::AnyCmdReq::InviteClaimerStartGreetingAttempt(req);

    let data = invited_cmds::AnyCmdReq::load(raw).unwrap();
    p_assert_eq!(data, expected);
}
