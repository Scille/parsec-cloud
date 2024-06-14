// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(clippy::unwrap_used)]

use libparsec_protocol::authenticated_cmds::v4::shamir_recovery_setup::ShamirRecoverySetup;
use libparsec_tests_lite::prelude::*;
use libparsec_types::{DateTime, ShamirRevealToken};

use super::authenticated_cmds;

macro_rules! test_roundtrip_serialization {
    ($($s:literal)*, $expected:path, $load:path) => {
        let raw = hex!($($s)*);
        let data = $load(&raw).unwrap();
        p_assert_eq!(
            data,
            $expected,
            "Expected hex is:\n\"{}\"",
            encode_and_format_with_70_width($expected.dump().unwrap())
        );

        // Also test serialization round trip
        let raw2 = data.dump().unwrap();
        let data2 = $load(&raw2).unwrap();
        p_assert_eq!(data2, $expected);
    };
}

fn encode_and_format_with_70_width<T: AsRef<[u8]>>(data: T) -> String {
    hex::encode(data)
        .as_bytes()
        .chunks(70)
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join("\"\n\"")
}

pub fn rep_shamir_setup_already_exists() {
    // Generated from Rust implementation (Parsec v3.0.0-b.6+dev 2024-06-06)
    // Content:
    //   status: "shamir_setup_already_exist"
    test_roundtrip_serialization!(
        "81a6737461747573bb7368616d69725f73657475705f616c72656164795f6578697374"
        "73",
        authenticated_cmds::shamir_recovery_setup::Rep::ShamirSetupAlreadyExists,
        authenticated_cmds::shamir_recovery_setup::Rep::load
    );
}

pub fn rep_brief_invalid_data() {
    // Generated from Rust implementation (Parsec v3.0.0-b.6+dev 2024-06-06)
    // Content:
    //   status: "brief_invalid_data"
    test_roundtrip_serialization!(
        "81a6737461747573b262726965665f696e76616c69645f64617461",
        authenticated_cmds::shamir_recovery_setup::Rep::BriefInvalidData,
        authenticated_cmds::shamir_recovery_setup::Rep::load
    );
}
pub fn rep_share_invalid_data() {
    // Generated from Rust implementation (Parsec v3.0.0-b.6+dev 2024-06-06)
    // Content:
    //   status: "share_invalid_data"
    test_roundtrip_serialization!(
        "81a6737461747573b273686172655f696e76616c69645f64617461",
        authenticated_cmds::shamir_recovery_setup::Rep::ShareInvalidData,
        authenticated_cmds::shamir_recovery_setup::Rep::load
    );
}
pub fn rep_invalid_recipient() {
    // Generated from Rust implementation (Parsec v3.0.0-b.6+dev 2024-06-06)
    // Content:
    //   status: "invalid_recipient"
    test_roundtrip_serialization!(
        "81a6737461747573b1696e76616c69645f726563697069656e74",
        authenticated_cmds::shamir_recovery_setup::Rep::InvalidRecipient,
        authenticated_cmds::shamir_recovery_setup::Rep::load
    );
}
pub fn rep_share_recipient_not_in_brief() {
    // Generated from Rust implementation (Parsec v3.0.0-b.6+dev 2024-06-06)
    // Content:
    //   status: "share_recipient_not_in_brief"
    test_roundtrip_serialization!(
        "81a6737461747573bc73686172655f726563697069656e745f6e6f745f696e5f627269"
        "6566",
        authenticated_cmds::shamir_recovery_setup::Rep::ShareRecipientNotInBrief,
        authenticated_cmds::shamir_recovery_setup::Rep::load
    );
}
pub fn rep_duplicate_share_for_recipient() {
    // Generated from Rust implementation (Parsec v3.0.0-b.6+dev 2024-06-06)
    // Content:
    //   status: "duplicate_share_for_recipient"
    test_roundtrip_serialization!(
        "81a6737461747573bd6475706c69636174655f73686172655f666f725f726563697069"
        "656e74",
        authenticated_cmds::shamir_recovery_setup::Rep::DuplicateShareForRecipient,
        authenticated_cmds::shamir_recovery_setup::Rep::load
    );
}
pub fn rep_author_included_as_recipient() {
    // Generated from Rust implementation (Parsec v3.0.0-b.6+dev 2024-06-06)
    // Content:
    //   status: "author_included_as_recipient"
    test_roundtrip_serialization!(
        "81a6737461747573bc617574686f725f696e636c756465645f61735f72656369706965"
        "6e74",
        authenticated_cmds::shamir_recovery_setup::Rep::AuthorIncludedAsRecipient,
        authenticated_cmds::shamir_recovery_setup::Rep::load
    );
}
pub fn rep_missing_share_for_recipient() {
    // Generated from Rust implementation (Parsec v3.0.0-b.6+dev 2024-06-06)
    // Content:
    //   status: "missing_share_for_recipient"
    test_roundtrip_serialization!(
        "81a6737461747573bb6d697373696e675f73686172655f666f725f726563697069656e"
        "74",
        authenticated_cmds::shamir_recovery_setup::Rep::MissingShareForRecipient,
        authenticated_cmds::shamir_recovery_setup::Rep::load
    );
}
pub fn rep_share_incoherent_timestamp() {
    // Generated from Rust implementation (Parsec v3.0.0-b.6+dev 2024-06-06)
    // Content:
    //   status: "share_incoherent_timestamp"
    test_roundtrip_serialization!(
        "81a6737461747573ba73686172655f696e636f686572656e745f74696d657374616d70",
        authenticated_cmds::shamir_recovery_setup::Rep::ShareIncoherentTimestamp,
        authenticated_cmds::shamir_recovery_setup::Rep::load
    );
}
pub fn rep_timestamp_out_of_ballpark() {
    // Generated from Rust implementation (Parsec v3.0.0-b.6+dev 2024-06-06)
    // Content:
    //   status: "timestamp_out_of_ballpark"
    //   ballpark_client_early_offset: 32.0
    //   ballpark_client_late_offset: 32.0
    //   client_timestamp: ext(1, timestamp(2009-02-13T23:31:30.000000Z))
    //   server_timestamp: ext(1, timestamp(2009-02-13T23:31:30.000000Z))

    let dt = DateTime::from_timestamp_seconds(1234567890).unwrap();
    let req = authenticated_cmds::shamir_recovery_setup::Rep::TimestampOutOfBallpark {
        ballpark_client_early_offset: 32.,
        ballpark_client_late_offset: 32.,
        client_timestamp: dt,
        server_timestamp: dt,
    };
    test_roundtrip_serialization!(
        "85a6737461747573b974696d657374616d705f6f75745f6f665f62616c6c7061726bbc"
        "62616c6c7061726b5f636c69656e745f6561726c795f6f6666736574cb404000000000"
        "0000bb62616c6c7061726b5f636c69656e745f6c6174655f6f6666736574cb40400000"
        "00000000b0636c69656e745f74696d657374616d70d701000462d53c88d880b0736572"
        "7665725f74696d657374616d70d701000462d53c88d880",
        req,
        authenticated_cmds::shamir_recovery_setup::Rep::load
    );
}
pub fn rep_require_greater_timestamp() {
    // Generated from Rust implementation (Parsec v3.0.0-b.6+dev 2024-06-06)
    // Content:
    //   status: "require_greater_timestamp"
    //   strictly_greater_than: ext(1, timestamp(2009-02-13T23:31:30.000000Z))
    //
    let req = authenticated_cmds::shamir_recovery_setup::Rep::RequireGreaterTimestamp {
        strictly_greater_than: DateTime::from_timestamp_seconds(1234567890).unwrap(),
    };
    test_roundtrip_serialization!(
        "82a6737461747573b9726571756972655f677265617465725f74696d657374616d70b5"
        "7374726963746c795f677265617465725f7468616ed701000462d53c88d880",
        req,
        authenticated_cmds::shamir_recovery_setup::Rep::load
    );
}

pub fn rep_ok() {
    // Generated from Rust implementation (Parsec v3.0.0-b.6+dev 2024-05-29)
    // Content:
    //   status: "ok"
    test_roundtrip_serialization!(
        "81a6737461747573a26f6b",
        authenticated_cmds::shamir_recovery_setup::Rep::Ok,
        authenticated_cmds::shamir_recovery_setup::Rep::load
    );
}

pub fn req() {
    let empty_req = authenticated_cmds::shamir_recovery_setup::Req { setup: None };
    // Generated from Rust implementation (Parsec v3.0.0-b.6+dev 2024-05-29)
    // Content:
    //   cmd: "shamir_recovery_setup",
    //   setup: None
    let raw = hex!("82a3636d64b57368616d69725f7265636f766572795f7365747570a57365747570c0");
    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();
    let expected = authenticated_cmds::AnyCmdReq::ShamirRecoverySetup(empty_req.clone());
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = empty_req.dump().unwrap();
    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();
    p_assert_eq!(data2, expected);

    let req = authenticated_cmds::shamir_recovery_setup::Req {
        setup: Some(ShamirRecoverySetup {
            brief: "brief".into(),
            ciphered_data: "ciphered_data".into(),
            reveal_token: ShamirRevealToken::from_hex("0563ff98846c4dbf9e0a1cc2f6fbb149").unwrap(),
            shares: vec!["shares".into()],
        }),
    };

    // Generated from Rust implementation (Parsec v3.0.0-b.6+dev 2024-05-29)
    // Content:
    //   cmd: "shamir_recovery_setup",
    //   setup:
    //      ciphered_data: "ciphered_data"
    //      reveal_token: hex!("0563ff98846c4dbf9e0a1cc2f6fbb149") // random bytes
    //      brief: "brief"
    //      shares: ["shares"]
    let raw = hex!(
        "82a3636d64b57368616d69725f7265636f766572795f7365747570a5736574757084a56272"
        "696566c4056272696566ad63697068657265645f64617461c40d63697068657265645f6461"
        "7461ac72657665616c5f746f6b656ec4100563ff98846c4dbf9e0a1cc2f6fbb149a6736861"
        "72657391c406736861726573"
    );
    let data = authenticated_cmds::AnyCmdReq::load(&raw).unwrap();
    let expected = authenticated_cmds::AnyCmdReq::ShamirRecoverySetup(req.clone());
    p_assert_eq!(data, expected);

    // Also test serialization round trip
    let raw2 = req.dump().unwrap();
    let data2 = authenticated_cmds::AnyCmdReq::load(&raw2).unwrap();
    p_assert_eq!(data2, expected);
}
