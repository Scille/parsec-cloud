// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

use super::invited_cmds;
use libparsec_tests_lite::{hex, p_assert_eq};
use libparsec_types::{CancelledGreetingAttemptReason, GreeterOrClaimer, GreetingAttemptID};

pub fn rep_greeter_not_allowed() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'greeter_not_allowed'
    let raw: &[u8] = hex!("81a6737461747573b3677265657465725f6e6f745f616c6c6f776564").as_ref();

    let expected = invited_cmds::invite_claimer_cancel_greeting_attempt::InviteClaimerCancelGreetingAttemptRep::GreeterNotAllowed;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = invited_cmds::invite_claimer_cancel_greeting_attempt::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
}

pub fn rep_greeter_revoked() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'greeter_revoked'
    let raw: &[u8] = hex!("81a6737461747573af677265657465725f7265766f6b6564").as_ref();

    let expected = invited_cmds::invite_claimer_cancel_greeting_attempt::InviteClaimerCancelGreetingAttemptRep::GreeterRevoked;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = invited_cmds::invite_claimer_cancel_greeting_attempt::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
}

pub fn rep_greeting_attempt_not_found() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'greeting_attempt_not_found'
    let raw: &[u8] =
        hex!("81a6737461747573ba6772656574696e675f617474656d70745f6e6f745f666f756e64").as_ref();
    let expected = invited_cmds::invite_claimer_cancel_greeting_attempt::InviteClaimerCancelGreetingAttemptRep::GreetingAttemptNotFound;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = invited_cmds::invite_claimer_cancel_greeting_attempt::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
}

pub fn rep_greeting_attempt_not_joined() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'greeting_attempt_not_joined'
    let raw: &[u8] = hex!(
    "81a6737461747573bb6772656574696e675f617474656d70745f6e6f745f6a6f696e65"
    "64"
    )
    .as_ref();
    let expected = invited_cmds::invite_claimer_cancel_greeting_attempt::InviteClaimerCancelGreetingAttemptRep::GreetingAttemptNotJoined;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = invited_cmds::invite_claimer_cancel_greeting_attempt::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
}

pub fn rep_greeting_attempt_already_cancelled() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'greeting_attempt_already_cancelled'
    //   origin: 'GREETER'
    //   reason: 'MANUALLY_CANCELLED'
    //   timestamp: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
    let raw: &[u8] = hex!(
    "84a6737461747573d9226772656574696e675f617474656d70745f616c72656164795f"
    "63616e63656c6c6564a66f726967696ea747524545544552a6726561736f6eb24d414e"
    "55414c4c595f43414e43454c4c4544a974696d657374616d70d70100035d162fa2e400"
    )
    .as_ref();
    let expected = invited_cmds::invite_claimer_cancel_greeting_attempt::InviteClaimerCancelGreetingAttemptRep::GreetingAttemptAlreadyCancelled{
        origin: GreeterOrClaimer::Greeter,
        reason: CancelledGreetingAttemptReason::ManuallyCancelled,
        timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
    };
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = invited_cmds::invite_claimer_cancel_greeting_attempt::Rep::load(raw).unwrap();
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

    let expected = invited_cmds::invite_claimer_cancel_greeting_attempt::InviteClaimerCancelGreetingAttemptRep::Ok;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = invited_cmds::invite_claimer_cancel_greeting_attempt::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
}

pub fn req() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   cmd: 'invite_claimer_cancel_greeting_attempt'
    //   greeting_attempt: ext(2, 0xd864b93ded264aae9ae583fd3d40c45a)
    //   reason: 'MANUALLY_CANCELLED'
    let raw: &[u8] = hex!(
    "83a3636d64d926696e766974655f636c61696d65725f63616e63656c5f677265657469"
    "6e675f617474656d7074b06772656574696e675f617474656d7074d802d864b93ded26"
    "4aae9ae583fd3d40c45aa6726561736f6eb24d414e55414c4c595f43414e43454c4c45"
    "44"
    )
    .as_ref();
    let req =
        invited_cmds::invite_claimer_cancel_greeting_attempt::InviteClaimerCancelGreetingAttemptReq {
            greeting_attempt: GreetingAttemptID::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
            reason: CancelledGreetingAttemptReason::ManuallyCancelled,
        };
    println!("***expected: {:?}", req.dump().unwrap());

    let expected = invited_cmds::AnyCmdReq::InviteClaimerCancelGreetingAttempt(req);

    let data = invited_cmds::AnyCmdReq::load(raw).unwrap();
    p_assert_eq!(data, expected);
}
