// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

use super::authenticated_cmds;
use libparsec_tests_lite::{hex, p_assert_eq};
use libparsec_types::AccessToken;
use libparsec_types::GreetingAttemptID;

pub fn rep_author_not_allowed() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'author_not_allowed'
    let raw: &[u8] = hex!("81a6737461747573b2617574686f725f6e6f745f616c6c6f776564").as_ref();
    let expected = authenticated_cmds::invite_greeter_start_greeting_attempt::InviteGreeterStartGreetingAttemptRep::AuthorNotAllowed;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_cmds::invite_greeter_start_greeting_attempt::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
}

pub fn rep_invitation_cancelled() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'invitation_cancelled'
    let raw: &[u8] = hex!("81a6737461747573b4696e7669746174696f6e5f63616e63656c6c6564").as_ref();
    let expected = authenticated_cmds::invite_greeter_start_greeting_attempt::InviteGreeterStartGreetingAttemptRep::InvitationCancelled;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_cmds::invite_greeter_start_greeting_attempt::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
}

pub fn rep_invitation_completed() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'invitation_completed'
    let raw: &[u8] = hex!("81a6737461747573b4696e7669746174696f6e5f636f6d706c65746564").as_ref();
    let expected = authenticated_cmds::invite_greeter_start_greeting_attempt::InviteGreeterStartGreetingAttemptRep::InvitationCompleted;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_cmds::invite_greeter_start_greeting_attempt::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
}

pub fn rep_invitation_not_found() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'invitation_not_found'
    let raw: &[u8] = hex!("81a6737461747573b4696e7669746174696f6e5f6e6f745f666f756e64").as_ref();
    let expected = authenticated_cmds::invite_greeter_start_greeting_attempt::InviteGreeterStartGreetingAttemptRep::InvitationNotFound;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_cmds::invite_greeter_start_greeting_attempt::Rep::load(raw).unwrap();
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
    let expected = authenticated_cmds::invite_greeter_start_greeting_attempt::InviteGreeterStartGreetingAttemptRep::Ok{greeting_attempt:GreetingAttemptID::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap()};
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = authenticated_cmds::invite_greeter_start_greeting_attempt::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
}
pub fn req() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   cmd: 'invite_greeter_start_greeting_attempt'
    //   token: 0xd864b93ded264aae9ae583fd3d40c45a
    let raw: &[u8] = hex!(
    "82a3636d64d925696e766974655f677265657465725f73746172745f6772656574696e"
    "675f617474656d7074a5746f6b656ec410d864b93ded264aae9ae583fd3d40c45a"
    )
    .as_ref();
    let req = authenticated_cmds::invite_greeter_start_greeting_attempt::InviteGreeterStartGreetingAttemptReq {token:AccessToken::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap()};
    println!("***expected: {:?}", req.dump().unwrap());

    let expected = authenticated_cmds::AnyCmdReq::InviteGreeterStartGreetingAttempt(req);

    let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();
    p_assert_eq!(data, expected);
}
