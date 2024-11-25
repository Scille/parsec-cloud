// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

use libparsec_tests_lite::prelude::*;

use super::authenticated_cmds;

fn req_helper(raw: &[u8], expected: authenticated_cmds::shamir_recovery_delete::Req) {
    let expected = authenticated_cmds::AnyCmdReq::ShamirRecoveryDelete(expected);
    let data = authenticated_cmds::AnyCmdReq::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let authenticated_cmds::AnyCmdReq::ShamirRecoveryDelete(req2) = data else {
        unreachable!()
    };

    let raw2 = req2.dump().unwrap();

    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

fn rep_helper(raw: &[u8], expected: authenticated_cmds::shamir_recovery_delete::Rep) {
    let data = authenticated_cmds::shamir_recovery_delete::Rep::load(raw).unwrap();

    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = data.dump().unwrap();

    let data2 = authenticated_cmds::shamir_recovery_delete::Rep::load(&raw2).unwrap();

    p_assert_eq!(data2, expected);
}

pub fn req() {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   cmd: 'shamir_recovery_delete'
    //   shamir_recovery_deletion_certificate: 0x64656c6574696f6e
    let raw: &[u8] = hex!(
        "82a3636d64b67368616d69725f7265636f766572795f64656c657465d9247368616d69"
        "725f7265636f766572795f64656c6574696f6e5f6365727469666963617465c4086465"
        "6c6574696f6e"
    )
    .as_ref();

    let expected = authenticated_cmds::shamir_recovery_delete::Req {
        shamir_recovery_deletion_certificate: "deletion".into(),
    };

    req_helper(raw, expected);
}

pub fn rep_ok() {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   status: 'ok'
    let raw: &[u8] = hex!("81a6737461747573a26f6b").as_ref();

    let expected = authenticated_cmds::shamir_recovery_delete::Rep::Ok;

    rep_helper(raw, expected);
}

pub fn rep_invalid_certificate_corrupted() {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   status: 'invalid_certificate_corrupted'
    let raw: &[u8] = hex!(
        "81a6737461747573bd696e76616c69645f63657274696669636174655f636f72727570"
        "746564"
    )
    .as_ref();

    let expected = authenticated_cmds::shamir_recovery_delete::Rep::InvalidCertificateCorrupted;

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
        authenticated_cmds::shamir_recovery_delete::Rep::InvalidCertificateUserIdMustBeSelf;

    rep_helper(raw, expected);
}

pub fn rep_shamir_recovery_not_found() {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   status: 'shamir_recovery_not_found'
    let raw: &[u8] =
        hex!("81a6737461747573b97368616d69725f7265636f766572795f6e6f745f666f756e64").as_ref();

    let expected = authenticated_cmds::shamir_recovery_delete::Rep::ShamirRecoveryNotFound;

    rep_helper(raw, expected);
}

pub fn rep_recipients_mismatch() {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   status: 'recipients_mismatch'
    let raw: &[u8] = hex!("81a6737461747573b3726563697069656e74735f6d69736d61746368").as_ref();

    let expected = authenticated_cmds::shamir_recovery_delete::Rep::RecipientsMismatch;

    rep_helper(raw, expected);
}

pub fn rep_shamir_recovery_already_deleted() {
    // Generated from Parsec 3.2.1-a.0+dev
    // Content:
    //   status: 'shamir_recovery_already_deleted'
    //   last_shamir_certificate_timestamp: ext(1, 946684800000000) i.e. 2000-01-01T01:00:00Z
    let raw: &[u8] = hex!(
        "82a6737461747573bf7368616d69725f7265636f766572795f616c72656164795f6465"
        "6c65746564d9216c6173745f7368616d69725f63657274696669636174655f74696d65"
        "7374616d70d70100035d013b37e000"
    )
    .as_ref();

    let expected = authenticated_cmds::shamir_recovery_delete::Rep::ShamirRecoveryAlreadyDeleted {
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

    let expected = authenticated_cmds::shamir_recovery_delete::Rep::TimestampOutOfBallpark {
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

    let expected = authenticated_cmds::shamir_recovery_delete::Rep::RequireGreaterTimestamp {
        strictly_greater_than: "2000-01-01T00:00:00Z".parse().unwrap(),
    };

    rep_helper(raw, expected);
}
