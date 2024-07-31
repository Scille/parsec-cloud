// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

use libparsec_protocol::authenticated_cmds::v4::shamir_recovery_setup::ShamirRecoverySetup;
use libparsec_tests_lite::prelude::*;
use libparsec_types::InvitationToken;

use super::authenticated_cmds;

fn req_helper(raw: &[u8], expected: authenticated_cmds::shamir_recovery_setup::Req) {
    let expected = authenticated_cmds::AnyCmdReq::ShamirRecoverySetup(expected);
    let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::ShamirRecoverySetup(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

fn rep_helper(raw: &[u8], expected: authenticated_cmds::shamir_recovery_setup::Rep) {
    let data = authenticated_cmds::shamir_recovery_setup::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::shamir_recovery_setup::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

fn req_empty_setup() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   cmd: 'shamir_recovery_setup'
    //   setup: None
    let raw: &[u8] =
        hex!("82a3636d64b57368616d69725f7265636f766572795f7365747570a57365747570c0").as_ref();

    let expected = authenticated_cmds::shamir_recovery_setup::Req { setup: None };

    req_helper(raw, expected);
}

fn req_with_setup() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   cmd: 'shamir_recovery_setup'
    //   setup: {
    //     brief: 0x6272696566,
    //     ciphered_data: 0x63697068657265645f64617461,
    //     reveal_token: 0x0563ff98846c4dbf9e0a1cc2f6fbb149,
    //     shares: [ 0x736861726573 ],
    //   }
    let raw: &[u8] = hex!(
    "82a3636d64b57368616d69725f7265636f766572795f7365747570a5736574757084a5"
    "6272696566c4056272696566ad63697068657265645f64617461c40d63697068657265"
    "645f64617461ac72657665616c5f746f6b656ec4100563ff98846c4dbf9e0a1cc2f6fb"
    "b149a673686172657391c406736861726573"
    )
    .as_ref();

    let expected = authenticated_cmds::shamir_recovery_setup::Req {
        setup: Some(ShamirRecoverySetup {
            brief: "brief".into(),
            ciphered_data: "ciphered_data".into(),
            reveal_token: InvitationToken::from_hex("0563ff98846c4dbf9e0a1cc2f6fbb149").unwrap(),
            shares: vec!["shares".into()],
        }),
    };

    req_helper(raw, expected);
}

pub fn req() {
    req_empty_setup();
    req_with_setup();
}

pub fn rep_shamir_setup_already_exists() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'shamir_setup_already_exists'
    //   last_shamir_certificate_timestamp: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z
    let raw: &[u8] = hex!(
    "82a6737461747573bb7368616d69725f73657475705f616c72656164795f6578697374"
    "73d9216c6173745f7368616d69725f63657274696669636174655f74696d657374616d"
    "70d70100035d013b37e000"
    )
    .as_ref();

    let expected = authenticated_cmds::shamir_recovery_setup::Rep::ShamirSetupAlreadyExists {
        last_shamir_certificate_timestamp: "2000-01-01T00:00:00Z".parse().unwrap(),
    };

    rep_helper(raw, expected);
}

pub fn rep_brief_invalid_data() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'brief_invalid_data'
    let raw: &[u8] = hex!("81a6737461747573b262726965665f696e76616c69645f64617461").as_ref();

    let expected = authenticated_cmds::shamir_recovery_setup::Rep::BriefInvalidData;

    rep_helper(raw, expected);
}

pub fn rep_share_invalid_data() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'share_invalid_data'
    let raw: &[u8] = hex!("81a6737461747573b273686172655f696e76616c69645f64617461").as_ref();

    let expected = authenticated_cmds::shamir_recovery_setup::Rep::ShareInvalidData;

    rep_helper(raw, expected);
}

pub fn rep_invalid_recipient() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'invalid_recipient'
    //   user_id: ext(2, 0xa11cec00100000000000000000000000)
    let raw: &[u8] = hex!(
    "82a6737461747573b1696e76616c69645f726563697069656e74a7757365725f6964d8"
    "02a11cec00100000000000000000000000"
    )
    .as_ref();

    let expected = authenticated_cmds::shamir_recovery_setup::Rep::InvalidRecipient {
        user_id: "alice".parse().unwrap(),
    };

    rep_helper(raw, expected);
}

pub fn rep_share_recipient_not_in_brief() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'share_recipient_not_in_brief'
    let raw: &[u8] = hex!(
    "81a6737461747573bc73686172655f726563697069656e745f6e6f745f696e5f627269"
    "6566"
    )
    .as_ref();

    let expected = authenticated_cmds::shamir_recovery_setup::Rep::ShareRecipientNotInBrief;

    rep_helper(raw, expected);
}

pub fn rep_duplicate_share_for_recipient() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'duplicate_share_for_recipient'
    let raw: &[u8] = hex!(
    "81a6737461747573bd6475706c69636174655f73686172655f666f725f726563697069"
    "656e74"
    )
    .as_ref();

    let expected = authenticated_cmds::shamir_recovery_setup::Rep::DuplicateShareForRecipient;

    rep_helper(raw, expected);
}

pub fn rep_author_included_as_recipient() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'author_included_as_recipient'
    let raw: &[u8] = hex!(
    "81a6737461747573bc617574686f725f696e636c756465645f61735f72656369706965"
    "6e74"
    )
    .as_ref();

    let expected = authenticated_cmds::shamir_recovery_setup::Rep::AuthorIncludedAsRecipient;

    rep_helper(raw, expected);
}

pub fn rep_missing_share_for_recipient() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'missing_share_for_recipient'
    let raw: &[u8] = hex!(
    "81a6737461747573bb6d697373696e675f73686172655f666f725f726563697069656e"
    "74"
    )
    .as_ref();

    let expected = authenticated_cmds::shamir_recovery_setup::Rep::MissingShareForRecipient;

    rep_helper(raw, expected);
}

pub fn rep_share_inconsistent_timestamp() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'share_inconsistent_timestamp'
    let raw: &[u8] = hex!(
    "81a6737461747573bc73686172655f696e636f6e73697374656e745f74696d65737461"
    "6d70"
    )
    .as_ref();

    let expected = authenticated_cmds::shamir_recovery_setup::Rep::ShareInconsistentTimestamp;

    rep_helper(raw, expected);
}

pub fn rep_timestamp_out_of_ballpark() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'timestamp_out_of_ballpark'
    //   ballpark_client_early_offset: 300.0
    //   ballpark_client_late_offset: 320.0
    //   client_timestamp: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
    //   server_timestamp: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
    let raw: &[u8] = hex!(
    "85a6737461747573b974696d657374616d705f6f75745f6f665f62616c6c7061726bbc"
    "62616c6c7061726b5f636c69656e745f6561726c795f6f6666736574cb4072c0000000"
    "0000bb62616c6c7061726b5f636c69656e745f6c6174655f6f6666736574cb40740000"
    "00000000b0636c69656e745f74696d657374616d70d70100035d162fa2e400b0736572"
    "7665725f74696d657374616d70d70100035d162fa2e400"
    )
    .as_ref();

    let expected = authenticated_cmds::shamir_recovery_setup::Rep::TimestampOutOfBallpark {
        ballpark_client_early_offset: 300.,
        ballpark_client_late_offset: 320.,
        server_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        client_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
    };

    rep_helper(raw, expected);
}

pub fn rep_require_greater_timestamp() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'require_greater_timestamp'
    //   strictly_greater_than: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z
    let raw: &[u8] = hex!(
    "82a6737461747573b9726571756972655f677265617465725f74696d657374616d70b5"
    "7374726963746c795f677265617465725f7468616ed70100035d013b37e000"
    )
    .as_ref();

    let expected = authenticated_cmds::shamir_recovery_setup::Rep::RequireGreaterTimestamp {
        strictly_greater_than: "2000-01-01T00:00:00Z".parse().unwrap(),
    };

    rep_helper(raw, expected);
}

pub fn rep_ok() {
    // Generated from Parsec 3.0.0-b.12+dev
    // Content:
    //   status: 'ok'
    let raw: &[u8] = hex!("81a6737461747573a26f6b").as_ref();

    let expected = authenticated_cmds::shamir_recovery_setup::Rep::Ok;

    rep_helper(raw, expected);
}
