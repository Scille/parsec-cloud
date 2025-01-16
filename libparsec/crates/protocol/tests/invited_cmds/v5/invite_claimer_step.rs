// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

use super::invited_cmds;
use libparsec_tests_lite::{hex, p_assert_eq};
use libparsec_types::{
    CancelledGreetingAttemptReason, GreeterOrClaimer, GreetingAttemptID, PublicKey,
};

pub fn rep_greeter_not_allowed() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'greeter_not_allowed'
    let raw: &[u8] = hex!("81a6737461747573b3677265657465725f6e6f745f616c6c6f776564").as_ref();
    let expected = invited_cmds::invite_claimer_step::InviteClaimerStepRep::GreeterNotAllowed;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = invited_cmds::invite_claimer_step::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
}

pub fn rep_greeter_revoked() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'greeter_revoked'
    let raw: &[u8] = hex!("81a6737461747573af677265657465725f7265766f6b6564").as_ref();
    let expected = invited_cmds::invite_claimer_step::InviteClaimerStepRep::GreeterRevoked;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = invited_cmds::invite_claimer_step::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
}

pub fn rep_greeting_attempt_not_found() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'greeting_attempt_not_found'
    let raw: &[u8] =
        hex!("81a6737461747573ba6772656574696e675f617474656d70745f6e6f745f666f756e64").as_ref();
    let expected = invited_cmds::invite_claimer_step::InviteClaimerStepRep::GreetingAttemptNotFound;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = invited_cmds::invite_claimer_step::Rep::load(raw).unwrap();
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
    let expected =
        invited_cmds::invite_claimer_step::InviteClaimerStepRep::GreetingAttemptNotJoined;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = invited_cmds::invite_claimer_step::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
}

pub fn rep_greeting_attempt_cancelled() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'greeting_attempt_cancelled'
    //   origin: 'GREETER'
    //   reason: 'MANUALLY_CANCELLED'
    //   timestamp: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
    let raw: &[u8] = hex!(
    "84a6737461747573ba6772656574696e675f617474656d70745f63616e63656c6c6564"
    "a66f726967696ea747524545544552a6726561736f6eb24d414e55414c4c595f43414e"
    "43454c4c4544a974696d657374616d70d70100035d162fa2e400"
    )
    .as_ref();
    let expected =
        invited_cmds::invite_claimer_step::InviteClaimerStepRep::GreetingAttemptCancelled {
            origin: GreeterOrClaimer::Greeter,
            reason: CancelledGreetingAttemptReason::ManuallyCancelled,
            timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        };
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = invited_cmds::invite_claimer_step::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
}

pub fn rep_step_too_advanced() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'step_too_advanced'
    let raw: &[u8] = hex!("81a6737461747573b1737465705f746f6f5f616476616e636564").as_ref();
    let expected = invited_cmds::invite_claimer_step::InviteClaimerStepRep::StepTooAdvanced;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = invited_cmds::invite_claimer_step::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
}

pub fn rep_step_mismatch() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'step_mismatch'
    let raw: &[u8] = hex!("81a6737461747573ad737465705f6d69736d61746368").as_ref();
    let expected = invited_cmds::invite_claimer_step::InviteClaimerStepRep::StepMismatch;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = invited_cmds::invite_claimer_step::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
}

pub fn rep_not_ready() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'not_ready'
    let raw: &[u8] = hex!("81a6737461747573a96e6f745f7265616479").as_ref();
    let expected = invited_cmds::invite_claimer_step::InviteClaimerStepRep::NotReady;
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = invited_cmds::invite_claimer_step::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
}

pub fn rep_ok() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'ok'
    //   greeter_step: { step: 'NUMBER_0_WAIT_PEER', public_key: 0x6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57, }
    let raw: &[u8] = hex!(
    "82a6737461747573a26f6bac677265657465725f7374657082a473746570b24e554d42"
    "45525f305f574149545f50454552aa7075626c69635f6b6579c4206507907d33bae6b5"
    "980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
    )
    .as_ref();
    let expected = invited_cmds::invite_claimer_step::InviteClaimerStepRep::Ok {
        greeter_step: invited_cmds::invite_claimer_step::GreeterStep::Number0WaitPeer {
            public_key: PublicKey::from(hex!(
                "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
            )),
        },
    };
    println!("***expected: {:?}", expected.dump().unwrap());
    let data = invited_cmds::invite_claimer_step::Rep::load(raw).unwrap();
    p_assert_eq!(data, expected);
}
pub fn req() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   cmd: 'invite_claimer_step'
    //   greeting_attempt: ext(2, 0xd864b93ded264aae9ae583fd3d40c45a)
    //   claimer_step: { step: 'NUMBER_0_WAIT_PEER', public_key: 0x6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57, }
    let raw: &[u8] = hex!(
    "83a3636d64b3696e766974655f636c61696d65725f73746570b06772656574696e675f"
    "617474656d7074d802d864b93ded264aae9ae583fd3d40c45aac636c61696d65725f73"
    "74657082a473746570b24e554d4245525f305f574149545f50454552aa7075626c6963"
    "5f6b6579c4206507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22c"
    "d5ac57"
    )
    .as_ref();
    let req = invited_cmds::invite_claimer_step::InviteClaimerStepReq {
        greeting_attempt: GreetingAttemptID::from_hex("d864b93ded264aae9ae583fd3d40c45a").unwrap(),
        claimer_step: invited_cmds::invite_claimer_step::ClaimerStep::Number0WaitPeer {
            public_key: PublicKey::from(hex!(
                "6507907d33bae6b5980b32fa03f3ebac56141b126e44f352ea46c5f22cd5ac57"
            )),
        },
    };
    println!("***expected: {:?}", req.dump().unwrap());

    let expected = invited_cmds::AnyCmdReq::InviteClaimerStep(req);

    let data = invited_cmds::AnyCmdReq::load(raw).unwrap();
    p_assert_eq!(data, expected);
}
