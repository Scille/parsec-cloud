// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;
use libparsec_types::AccessToken;

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

pub fn req() {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   cmd: 'shamir_recovery_setup'
    //   ciphered_data: 0x63697068657265645f64617461
    //   reveal_token: 0x0563ff98846c4dbf9e0a1cc2f6fbb149
    //   shamir_recovery_brief_certificate: 0x6272696566
    //   shamir_recovery_share_certificates: [ 0x736861726573, ]
    let raw: &[u8] = hex!(
        "85a3636d64b57368616d69725f7265636f766572795f7365747570ad63697068657265"
        "645f64617461c40d63697068657265645f64617461ac72657665616c5f746f6b656ec4"
        "100563ff98846c4dbf9e0a1cc2f6fbb149d9217368616d69725f7265636f766572795f"
        "62726965665f6365727469666963617465c4056272696566d9227368616d69725f7265"
        "636f766572795f73686172655f63657274696669636174657391c406736861726573"
    )
    .as_ref();

    let expected = authenticated_cmds::shamir_recovery_setup::Req {
        shamir_recovery_brief_certificate: "brief".into(),
        shamir_recovery_share_certificates: vec!["shares".into()],
        ciphered_data: "ciphered_data".into(),
        reveal_token: AccessToken::from_hex("0563ff98846c4dbf9e0a1cc2f6fbb149").unwrap(),
    };

    req_helper(raw, expected);
}

pub fn rep_ok() {
    let raw: &[u8] = hex!("81a6737461747573a26f6b").as_ref();

    let expected = authenticated_cmds::shamir_recovery_setup::Rep::Ok;

    rep_helper(raw, expected);
}

pub fn rep_invalid_certificate_brief_corrupted() {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   status: 'invalid_certificate_brief_corrupted'
    let raw: &[u8] = hex!(
        "81a6737461747573d923696e76616c69645f63657274696669636174655f6272696566"
        "5f636f72727570746564"
    )
    .as_ref();

    let expected = authenticated_cmds::shamir_recovery_setup::Rep::InvalidCertificateBriefCorrupted;

    rep_helper(raw, expected);
}

pub fn rep_invalid_certificate_share_corrupted() {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   status: 'invalid_certificate_share_corrupted'
    let raw: &[u8] = hex!(
        "81a6737461747573d923696e76616c69645f63657274696669636174655f7368617265"
        "5f636f72727570746564"
    )
    .as_ref();

    let expected = authenticated_cmds::shamir_recovery_setup::Rep::InvalidCertificateShareCorrupted;

    rep_helper(raw, expected);
}

pub fn rep_invalid_certificate_share_recipient_not_in_brief() {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   status: 'invalid_certificate_share_recipient_not_in_brief'
    let raw: &[u8] = hex!(
        "81a6737461747573d930696e76616c69645f63657274696669636174655f7368617265"
        "5f726563697069656e745f6e6f745f696e5f6272696566"
    )
    .as_ref();

    let expected =
        authenticated_cmds::shamir_recovery_setup::Rep::InvalidCertificateShareRecipientNotInBrief;

    rep_helper(raw, expected);
}

pub fn rep_invalid_certificate_duplicate_share_for_recipient() {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   status: 'invalid_certificate_duplicate_share_for_recipient'
    let raw: &[u8] = hex!(
        "81a6737461747573d931696e76616c69645f63657274696669636174655f6475706c69"
        "636174655f73686172655f666f725f726563697069656e74"
    )
    .as_ref();

    let expected = authenticated_cmds::shamir_recovery_setup::Rep::InvalidCertificateDuplicateShareForRecipient;

    rep_helper(raw, expected);
}

pub fn rep_invalid_certificate_author_included_as_recipient() {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   status: 'invalid_certificate_author_included_as_recipient'
    let raw: &[u8] = hex!(
        "81a6737461747573d930696e76616c69645f63657274696669636174655f617574686f"
        "725f696e636c756465645f61735f726563697069656e74"
    )
    .as_ref();

    let expected =
        authenticated_cmds::shamir_recovery_setup::Rep::InvalidCertificateAuthorIncludedAsRecipient;

    rep_helper(raw, expected);
}

pub fn rep_invalid_certificate_missing_share_for_recipient() {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   status: 'invalid_certificate_missing_share_for_recipient'
    let raw: &[u8] = hex!(
        "81a6737461747573d92f696e76616c69645f63657274696669636174655f6d69737369"
        "6e675f73686172655f666f725f726563697069656e74"
    )
    .as_ref();

    let expected =
        authenticated_cmds::shamir_recovery_setup::Rep::InvalidCertificateMissingShareForRecipient;

    rep_helper(raw, expected);
}

pub fn rep_invalid_certificate_share_inconsistent_timestamp() {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   status: 'invalid_certificate_share_inconsistent_timestamp'
    let raw: &[u8] = hex!(
        "81a6737461747573d930696e76616c69645f63657274696669636174655f7368617265"
        "5f696e636f6e73697374656e745f74696d657374616d70"
    )
    .as_ref();

    let expected = authenticated_cmds::shamir_recovery_setup::Rep::InvalidCertificateShareInconsistentTimestamp;

    rep_helper(raw, expected);
}

pub fn rep_invalid_certificate_user_id_must_be_self() {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   status: 'invalid_certificate_user_id_must_be_self'
    let raw: &[u8] = hex!(
        "81a6737461747573d928696e76616c69645f63657274696669636174655f757365725f"
        "69645f6d7573745f62655f73656c66"
    )
    .as_ref();

    let expected =
        authenticated_cmds::shamir_recovery_setup::Rep::InvalidCertificateUserIdMustBeSelf;

    rep_helper(raw, expected);
}

pub fn rep_recipient_not_found() {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   status: 'recipient_not_found'
    let raw: &[u8] = hex!("81a6737461747573b3726563697069656e745f6e6f745f666f756e64").as_ref();

    let expected = authenticated_cmds::shamir_recovery_setup::Rep::RecipientNotFound;

    rep_helper(raw, expected);
}

pub fn rep_revoked_recipient() {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   status: 'revoked_recipient'
    //   last_common_certificate_timestamp: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z
    let raw: &[u8] = hex!(
        "82a6737461747573b17265766f6b65645f726563697069656e74d9216c6173745f636f"
        "6d6d6f6e5f63657274696669636174655f74696d657374616d70d70100035d013b37e0"
        "00"
    )
    .as_ref();

    let expected = authenticated_cmds::shamir_recovery_setup::Rep::RevokedRecipient {
        last_common_certificate_timestamp: "2000-01-01T00:00:00Z".parse().unwrap(),
    };

    rep_helper(raw, expected);
}

pub fn rep_shamir_recovery_already_exists() {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   status: 'shamir_recovery_already_exists'
    //   last_shamir_certificate_timestamp: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z
    let raw: &[u8] = hex!(
        "82a6737461747573be7368616d69725f7265636f766572795f616c72656164795f6578"
        "69737473d9216c6173745f7368616d69725f63657274696669636174655f74696d6573"
        "74616d70d70100035d013b37e000"
    )
    .as_ref();

    let expected = authenticated_cmds::shamir_recovery_setup::Rep::ShamirRecoveryAlreadyExists {
        last_shamir_certificate_timestamp: "2000-01-01T00:00:00Z".parse().unwrap(),
    };

    rep_helper(raw, expected);
}

pub fn rep_timestamp_out_of_ballpark() {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   status: 'timestamp_out_of_ballpark'
    //   ballpark_client_early_offset: 300.0
    //   ballpark_client_late_offset: 320.0
    //   client_timestamp: ext(1, 946778400000000) i.e. 2000-01-02T03:00:00Z
    //   server_timestamp: ext(1, 946774800000000) i.e. 2000-01-02T02:00:00Z
    let raw: &[u8] = hex!(
        "85a6737461747573b974696d657374616d705f6f75745f6f665f62616c6c7061726bbc"
        "62616c6c7061726b5f636c69656e745f6561726c795f6f6666736574cb4072c0000000"
        "0000bb62616c6c7061726b5f636c69656e745f6c6174655f6f6666736574cb40740000"
        "00000000b0636c69656e745f74696d657374616d70d70100035d1706368800b0736572"
        "7665725f74696d657374616d70d70100035d162fa2e400"
    )
    .as_ref();

    let expected = authenticated_cmds::shamir_recovery_setup::Rep::TimestampOutOfBallpark {
        ballpark_client_early_offset: 300.,
        ballpark_client_late_offset: 320.,
        server_timestamp: "2000-1-2T01:00:00Z".parse().unwrap(),
        client_timestamp: "2000-1-2T02:00:00Z".parse().unwrap(),
    };

    rep_helper(raw, expected);
}

pub fn rep_require_greater_timestamp() {
    // Generated from Parsec 3.2.1-a.0+dev
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
